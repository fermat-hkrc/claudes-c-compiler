// Oracle: differential — llvm-mc i686 assembler (POP r16 / POP Sreg / POP m16)
// Evidence: system.rs:324 "Encode popw (16-bit pop)";
//   mod.rs:175 "popw" => encode_pop16;
//   sibling encode_pop (gp_integer.rs:402) handles popl + memory 8F /0 + Sreg forms;
//   Intel SDM Vol.2 POP — r16 short 58+rw, Sreg one-byte/0F, r/m16 8F /0;
//   operand-size override 0x66 required for 16-bit forms in 32-bit code.
// Stronger considered:
//   - State machine: rejected — pure encoding, no lifecycle
//   - Algebraic round-trip: rejected — no in-tree i686 decoder for popw
// Differential: candidate=encode_pop16 (via InstructionEncoder::encode popw),
//   reference=llvm-mc -triple=i686 -show-encoding (LLVM 15),
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=AT&T operand bytes <-> llvm-mc encoding bytes.
// Also: algebraic.invariant (r16 = 66 58+n), algebraic.metamorphic
//   (popw = 66 ‖ popl), negative_error (arity / cs / r32 / r8).

use super::InstructionEncoder;
use crate::backend::x86::assembler::parser::{
    Displacement, Instruction, MemoryOperand, Operand, Register,
};
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;

fn cfg() -> ProptestConfig {
    ProptestConfig::with_cases(CASES)
}

const R16: &[&str] = &["ax", "cx", "dx", "bx", "sp", "bp", "si", "di"];
const R32: &[&str] = &["eax", "ecx", "edx", "ebx", "esp", "ebp", "esi", "edi"];
const R8: &[&str] = &["al", "cl", "dl", "bl", "ah", "ch", "dh", "bh"];
const SREG_POP: &[&str] = &["es", "ss", "ds", "fs", "gs"]; // cs is invalid for POP
const GP32: &[&str] = &["eax", "ecx", "edx", "ebx", "esp", "ebp", "esi", "edi"];
const SCALES: &[u8] = &[1, 2, 4, 8];

const R16_R32: &[(&str, &str)] = &[
    ("ax", "eax"),
    ("cx", "ecx"),
    ("dx", "edx"),
    ("bx", "ebx"),
    ("sp", "esp"),
    ("bp", "ebp"),
    ("si", "esi"),
    ("di", "edi"),
];

fn reg_num(name: &str) -> u8 {
    match name {
        "ax" | "eax" | "al" => 0,
        "cx" | "ecx" | "cl" => 1,
        "dx" | "edx" | "dl" => 2,
        "bx" | "ebx" | "bl" => 3,
        "sp" | "esp" | "ah" => 4,
        "bp" | "ebp" | "ch" => 5,
        "si" | "esi" | "dh" => 6,
        "di" | "edi" | "bh" => 7,
        _ => panic!("bad reg {name}"),
    }
}

fn mem_base(base: &str, disp: Displacement, segment: Option<&str>) -> MemoryOperand {
    MemoryOperand {
        segment: segment.map(|s| s.to_string()),
        displacement: disp,
        base: Some(Register::new(base)),
        index: None,
        scale: None,
    }
}

fn mem_base_index(
    base: Option<&str>,
    index: &str,
    scale: u8,
    disp: Displacement,
    segment: Option<&str>,
) -> MemoryOperand {
    MemoryOperand {
        segment: segment.map(|s| s.to_string()),
        displacement: disp,
        base: base.map(Register::new),
        index: Some(Register::new(index)),
        scale: Some(scale),
    }
}

fn mem_abs(disp: i64, segment: Option<&str>) -> MemoryOperand {
    MemoryOperand {
        segment: segment.map(|s| s.to_string()),
        displacement: Displacement::Integer(disp),
        base: None,
        index: None,
        scale: None,
    }
}

fn att_disp(d: &Displacement) -> String {
    match d {
        Displacement::None => String::new(),
        Displacement::Integer(0) => String::new(),
        Displacement::Integer(v) => format!("{v}"),
        Displacement::Symbol(s) => s.clone(),
        Displacement::SymbolAddend(s, a) => {
            if *a >= 0 {
                format!("{s}+{a}")
            } else {
                format!("{s}{a}")
            }
        }
        Displacement::SymbolPlusOffset(s, a) => {
            if *a >= 0 {
                format!("{s}+{a}")
            } else {
                format!("{s}{a}")
            }
        }
        Displacement::SymbolMod(s, m) => format!("{s}@{m}"),
    }
}

fn att_mem(mem: &MemoryOperand) -> String {
    if mem.base.is_none() && mem.index.is_none() {
        let mut abs = String::new();
        if let Some(ref seg) = mem.segment {
            abs.push('%');
            abs.push_str(seg);
            abs.push(':');
        }
        match &mem.displacement {
            Displacement::None => abs.push_str("0"),
            Displacement::Integer(v) => abs.push_str(&format!("{v}")),
            other => abs.push_str(&att_disp(other)),
        }
        return abs;
    }
    let mut s = String::new();
    if let Some(ref seg) = mem.segment {
        s.push('%');
        s.push_str(seg);
        s.push(':');
    }
    s.push_str(&att_disp(&mem.displacement));
    s.push('(');
    match (&mem.base, &mem.index, mem.scale) {
        (Some(b), None, _) => {
            s.push('%');
            s.push_str(&b.name);
        }
        (Some(b), Some(i), sc) => {
            s.push('%');
            s.push_str(&b.name);
            s.push(',');
            s.push('%');
            s.push_str(&i.name);
            s.push(',');
            s.push_str(&format!("{}", sc.unwrap_or(1)));
        }
        (None, Some(i), sc) => {
            s.push(',');
            s.push('%');
            s.push_str(&i.name);
            s.push(',');
            s.push_str(&format!("{}", sc.unwrap_or(1)));
        }
        (None, None, _) => {}
    }
    s.push(')');
    s
}

fn sut_encode(mnemonic: &str, ops: Vec<Operand>) -> Result<Vec<u8>, String> {
    let mut enc = InstructionEncoder::new();
    enc.encode(&Instruction {
        prefix: None,
        mnemonic: mnemonic.to_string(),
        operands: ops,
    })?;
    Ok(enc.bytes)
}

fn parse_llvm_bytes(stdout: &str) -> Result<Vec<u8>, String> {
    let marker = "encoding: [";
    let start = stdout
        .find(marker)
        .ok_or_else(|| format!("no encoding in llvm-mc output: {stdout}"))?
        + marker.len();
    let end = stdout[start..]
        .find(']')
        .ok_or_else(|| format!("no closing bracket in: {stdout}"))?
        + start;
    let body = &stdout[start..end];
    let mut bytes = Vec::new();
    for part in body.split(',') {
        let t = part.trim();
        if t.is_empty() {
            continue;
        }
        let v = u8::from_str_radix(t.trim_start_matches("0x"), 16)
            .map_err(|e| format!("bad byte {t}: {e}"))?;
        bytes.push(v);
    }
    if bytes.is_empty() {
        return Err(format!("empty encoding: {stdout}"));
    }
    Ok(bytes)
}

fn llvm_mc_bytes(asm: &str) -> Result<Vec<u8>, String> {
    let mut child = Command::new(LLVM_MC)
        .args(["-triple=i686", "-show-encoding"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("spawn llvm-mc: {e}"))?;
    {
        let mut stdin = child.stdin.take().ok_or("llvm-mc stdin")?;
        writeln!(stdin, "{asm}").map_err(|e| format!("write llvm-mc: {e}"))?;
    }
    let out = child
        .wait_with_output()
        .map_err(|e| format!("wait llvm-mc: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "llvm-mc error: {}",
            String::from_utf8_lossy(&out.stderr)
        ));
    }
    parse_llvm_bytes(&String::from_utf8_lossy(&out.stdout))
}

// --- KAT gate (reference oracle prerequisite) ---

#[test]
fn encode_pop16_kat_llvm_mc_ax() {
    let mc = llvm_mc_bytes("popw %ax").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x66, 0x58], "llvm-mc KAT mapping broken");
    let sut = sut_encode("popw", vec![Operand::Register(Register::new("ax"))])
        .expect("SUT KAT");
    assert_eq!(sut, mc, "SUT KAT popw %ax");
}

#[test]
fn encode_pop16_kat_llvm_mc_es() {
    // popw %es must include operand-size override 0x66
    let mc = llvm_mc_bytes("popw %es").expect("llvm-mc KAT es");
    assert_eq!(mc, vec![0x66, 0x07], "llvm-mc KAT popw %es");
    let sut = sut_encode("popw", vec![Operand::Register(Register::new("es"))]);
    match sut {
        Ok(bytes) => assert_eq!(bytes, mc, "SUT KAT popw %es must include 0x66"),
        Err(e) => panic!("SUT rejected popw %es: {e}"),
    }
}

#[test]
fn encode_pop16_kat_llvm_mc_fs() {
    let mc = llvm_mc_bytes("popw %fs").expect("llvm-mc KAT fs");
    assert_eq!(mc, vec![0x66, 0x0f, 0xa1]);
    let sut = sut_encode("popw", vec![Operand::Register(Register::new("fs"))]);
    match sut {
        Ok(bytes) => assert_eq!(bytes, mc, "SUT KAT popw %fs must include 0x66"),
        Err(e) => panic!("SUT rejected popw %fs: {e}"),
    }
}

#[test]
fn encode_pop16_kat_llvm_mc_mem() {
    let mc = llvm_mc_bytes("popw (%eax)").expect("llvm-mc KAT mem");
    assert_eq!(mc, vec![0x66, 0x8f, 0x00]);
    let sut = sut_encode(
        "popw",
        vec![Operand::Memory(mem_base("eax", Displacement::None, None))],
    );
    match sut {
        Ok(bytes) => assert_eq!(bytes, mc, "SUT KAT popw (%eax)"),
        Err(e) => panic!("SUT rejected popw (%eax): {e}"),
    }
}

#[test]
fn encode_pop16_kat_llvm_mc_di() {
    let mc = llvm_mc_bytes("popw %di").expect("llvm-mc KAT di");
    assert_eq!(mc, vec![0x66, 0x5f]);
    let sut = sut_encode("popw", vec![Operand::Register(Register::new("di"))])
        .expect("SUT");
    assert_eq!(sut, mc);
}

proptest! {
    #![proptest_config(cfg())]

    // Oracle: differential — popw r16 short form
    #[test]
    fn encode_pop16_diff_r16(r16 in prop::sample::select(R16)) {
        let asm = format!("popw %{r16}");
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let sut = sut_encode("popw", vec![Operand::Register(Register::new(r16))])
            .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(sut, mc, "r16 diff vs llvm-mc for {}", asm);
    }

    // Oracle: differential — popw Sreg (must emit 0x66)
    #[test]
    fn encode_pop16_diff_sreg(sreg in prop::sample::select(SREG_POP)) {
        let asm = format!("popw %{sreg}");
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let sut = sut_encode("popw", vec![Operand::Register(Register::new(sreg))])
            .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(
            sut, mc,
            "Sreg popw must match llvm-mc (incl. 0x66) for {}",
            asm
        );
    }

    // Oracle: differential — popw memory (no segment override)
    #[test]
    fn encode_pop16_diff_mem(
        kind in 0u8..6,
        base in prop::sample::select(GP32),
        index in prop::sample::select(GP32),
        scale in prop::sample::select(SCALES),
        disp in prop_oneof![
            Just(0i64),
            Just(4i64),
            Just(-1i64),
            Just(0x1234i64),
            any::<i8>().prop_map(|v| v as i64),
        ],
    ) {
        prop_assume!(index != "esp" || kind < 3);

        let mem = match kind {
            0 => mem_base(base, Displacement::None, None),
            1 => mem_base(base, Displacement::Integer(disp), None),
            2 => mem_base("esp", Displacement::None, None),
            3 => mem_base("ebp", Displacement::Integer(disp.max(1).min(127)), None),
            4 => mem_base_index(Some(base), index, scale, Displacement::Integer(disp), None),
            _ => mem_abs(disp, None),
        };

        let att = att_mem(&mem);
        let asm = format!("popw {att}");
        let mc = match llvm_mc_bytes(&asm) {
            Ok(b) => b,
            Err(_e) => {
                prop_assume!(false);
                return Ok(());
            }
        };
        let sut = sut_encode("popw", vec![Operand::Memory(mem)])
            .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(sut, mc, "mem diff vs llvm-mc for {}", asm);
    }

    // Oracle: differential — memory with segment override
    #[test]
    fn encode_pop16_diff_mem_segment(
        mseg in prop::sample::select(SREG_POP),
        base in prop::sample::select(GP32),
        disp in prop_oneof![Just(0i64), Just(4i64), Just(8i64)],
    ) {
        let mem = mem_base(base, Displacement::Integer(disp), Some(mseg));
        let att = att_mem(&mem);
        let asm = format!("popw {att}");
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let sut = sut_encode("popw", vec![Operand::Memory(mem)])
            .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(
            sut, mc,
            "segmented mem must include override + 0x66 for {}",
            asm
        );
    }

    // Oracle: algebraic.invariant — r16 = [0x66, 0x58+n]
    #[test]
    fn encode_pop16_invariant_r16(r16 in prop::sample::select(R16)) {
        let bytes = sut_encode("popw", vec![Operand::Register(Register::new(r16))])
            .expect("valid popw r16 must encode");
        prop_assert_eq!(bytes.len(), 2, "expected 2 bytes, got {:02x?}", bytes);
        prop_assert_eq!(bytes[0], 0x66u8, "operand-size prefix");
        prop_assert_eq!(bytes[1], 0x58u8 + reg_num(r16), "short-form opcode");
    }

    // Oracle: algebraic.metamorphic — popw r16 = 0x66 ‖ popl r32
    #[test]
    fn encode_pop16_metamorphic_popw_vs_popl_gp(
        pair in prop::sample::select(R16_R32),
    ) {
        let (r16, r32) = pair;
        let w = sut_encode("popw", vec![Operand::Register(Register::new(r16))])
            .expect("popw");
        let l = sut_encode("popl", vec![Operand::Register(Register::new(r32))])
            .expect("popl");
        let mut expect = vec![0x66u8];
        expect.extend_from_slice(&l);
        prop_assert_eq!(
            &w, &expect,
            "popw %{} must be 0x66 || popl %{} (w={:02x?} l={:02x?})",
            r16, r32, w, l
        );
    }

    // Oracle: algebraic.metamorphic — popw Sreg = 0x66 ‖ popl Sreg
    #[test]
    fn encode_pop16_metamorphic_sreg_vs_popl(
        sreg in prop::sample::select(SREG_POP),
    ) {
        let w = sut_encode("popw", vec![Operand::Register(Register::new(sreg))])
            .expect("popw sreg");
        let l = sut_encode("popl", vec![Operand::Register(Register::new(sreg))])
            .expect("popl sreg");
        let mut expect = vec![0x66u8];
        expect.extend_from_slice(&l);
        prop_assert_eq!(
            &w, &expect,
            "popw %{} must be 0x66 || popl %{} (w={:02x?} l={:02x?})",
            sreg, sreg, w, l
        );
    }

    // Oracle: negative_error — wrong arity
    #[test]
    fn encode_pop16_neg_arity(n in 0usize..4) {
        prop_assume!(n != 1);
        let ops: Vec<Operand> = (0..n)
            .map(|_| Operand::Register(Register::new("ax")))
            .collect();
        let r = sut_encode("popw", ops);
        prop_assert!(r.is_err(), "arity {n} must be Err, got {r:?}");
    }

    // Oracle: negative_error — cannot pop CS
    #[test]
    fn encode_pop16_neg_cs(_u in 0u8..1) {
        let r = sut_encode("popw", vec![Operand::Register(Register::new("cs"))]);
        prop_assert!(r.is_err(), "popw %cs must be Err, got {r:?}");
    }

    // Oracle: negative_error — r32 invalid for popw (llvm-mc rejects)
    #[test]
    fn encode_pop16_neg_r32(r32 in prop::sample::select(R32)) {
        let asm = format!("popw %{r32}");
        // Confirm reference rejects
        prop_assert!(
            llvm_mc_bytes(&asm).is_err(),
            "llvm-mc unexpectedly accepted `{asm}`"
        );
        let r = sut_encode("popw", vec![Operand::Register(Register::new(r32))]);
        prop_assert!(
            r.is_err(),
            "popw %{} must be Err like llvm-mc, got Ok({:02x?})",
            r32,
            r.as_ref().unwrap_or(&vec![])
        );
    }

    // Oracle: negative_error — r8 invalid for popw
    #[test]
    fn encode_pop16_neg_r8(r8 in prop::sample::select(R8)) {
        let asm = format!("popw %{r8}");
        prop_assert!(
            llvm_mc_bytes(&asm).is_err(),
            "llvm-mc unexpectedly accepted `{asm}`"
        );
        let r = sut_encode("popw", vec![Operand::Register(Register::new(r8))]);
        prop_assert!(
            r.is_err(),
            "popw %{} must be Err like llvm-mc, got Ok({:02x?})",
            r8,
            r.as_ref().unwrap_or(&vec![])
        );
    }
}

// Deterministic regression witnesses (filled after triage)

#[test]
fn test_encode_pop16_regression_sreg_missing_66() {
    // Witness: popw %es → SUT [07], llvm-mc [66, 07]
    let mc = llvm_mc_bytes("popw %es").expect("llvm-mc");
    assert_eq!(mc, vec![0x66, 0x07]);
    let sut = sut_encode("popw", vec![Operand::Register(Register::new("es"))])
        .expect("SUT should encode popw %es");
    assert_eq!(
        sut, mc,
        "regression: popw %es must include 0x66 (got {:02x?})",
        sut
    );
}

#[test]
fn test_encode_pop16_regression_mem_unsupported() {
    // Witness: popw (%eax) → SUT Err, llvm-mc [66, 8f, 00]
    let mc = llvm_mc_bytes("popw (%eax)").expect("llvm-mc");
    assert_eq!(mc, vec![0x66, 0x8f, 0x00]);
    let sut = sut_encode(
        "popw",
        vec![Operand::Memory(mem_base("eax", Displacement::None, None))],
    );
    match sut {
        Ok(bytes) => assert_eq!(bytes, mc, "regression popw (%eax)"),
        Err(e) => panic!("regression: popw (%eax) rejected: {e}"),
    }
}

#[test]
fn test_encode_pop16_regression_r32_accepted() {
    // Witness: popw %eax should be Err (llvm-mc rejects); SUT emits 66 58
    assert!(llvm_mc_bytes("popw %eax").is_err());
    let sut = sut_encode("popw", vec![Operand::Register(Register::new("eax"))]);
    assert!(
        sut.is_err(),
        "regression: popw %eax must be Err, got Ok({:02x?})",
        sut.unwrap_or_default()
    );
}
