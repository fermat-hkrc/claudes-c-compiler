// Oracle: differential — llvm-mc i686 assembler (PUSHW imm / r16 / Sreg / m16)
// Evidence: gp_integer.rs:382 encode_push16; mod.rs:196 "pushw" => encode_push16;
//   Intel SDM Vol.2 PUSH — imm8 6A ib, imm16 68 iw, r16 short 50+rw, r/m16 FF /6,
//   Sreg one-byte/0F forms; operand-size override 0x66 required in 32-bit code;
//   sibling encode_push (gp_integer.rs:346) handles 32-bit forms; encode_pop16
//   handles 16-bit POP with 0x66; core.rs:31-42 emit_segment_prefix.
// Stronger considered:
//   - State machine: rejected — pure encoding, no lifecycle
//   - Algebraic round-trip: rejected — no in-tree i686 decoder for pushw
// Differential: candidate=encode_push16 (via InstructionEncoder::encode pushw),
//   reference=llvm-mc -triple=i686 -show-encoding (LLVM 15),
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=AT&T operand bytes <-> llvm-mc encoding bytes.
// Also: algebraic.invariant (imm form 66 6A/68), algebraic.metamorphic
//   (pushw imm8 = 66 ‖ pushl imm8), negative_error (arity / r32 / r8).

use super::InstructionEncoder;
use crate::backend::x86::assembler::parser::{
    Displacement, ImmediateValue, Instruction, MemoryOperand, Operand, Register,
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
const SREGS: &[&str] = &["es", "cs", "ss", "ds", "fs", "gs"];
const GP32: &[&str] = &["eax", "ecx", "edx", "ebx", "esp", "ebp", "esi", "edi"];
const SCALES: &[u8] = &[1, 2, 4, 8];

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
        if t.chars().all(|c| c == 'A') {
            for _ in 0..t.len() {
                bytes.push(0);
            }
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
fn encode_push16_kat_llvm_mc_imm0() {
    let mc = llvm_mc_bytes("pushw $0").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x66, 0x6a, 0x00], "llvm-mc KAT mapping broken");
    let sut = sut_encode(
        "pushw",
        vec![Operand::Immediate(ImmediateValue::Integer(0))],
    )
    .expect("SUT KAT");
    assert_eq!(sut, mc, "SUT KAT pushw $0");
}

#[test]
fn encode_push16_kat_llvm_mc_imm16() {
    let mc = llvm_mc_bytes("pushw $128").expect("llvm-mc KAT imm128");
    assert_eq!(mc, vec![0x66, 0x68, 0x80, 0x00]);
    let sut = sut_encode(
        "pushw",
        vec![Operand::Immediate(ImmediateValue::Integer(128))],
    )
    .expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_push16_kat_llvm_mc_ax() {
    let mc = llvm_mc_bytes("pushw %ax").expect("llvm-mc KAT ax");
    assert_eq!(mc, vec![0x66, 0x50], "llvm-mc KAT pushw %ax");
    let sut = sut_encode("pushw", vec![Operand::Register(Register::new("ax"))]);
    match sut {
        Ok(bytes) => assert_eq!(bytes, mc, "SUT must encode pushw %ax as 66 50"),
        Err(e) => panic!("SUT erred on valid pushw %ax: {e}"),
    }
}

#[test]
fn encode_push16_kat_llvm_mc_es() {
    let mc = llvm_mc_bytes("pushw %es").expect("llvm-mc KAT es");
    assert_eq!(mc, vec![0x66, 0x06], "llvm-mc KAT pushw %es");
    let sut = sut_encode("pushw", vec![Operand::Register(Register::new("es"))]);
    match sut {
        Ok(bytes) => assert_eq!(bytes, mc, "SUT must encode pushw %es as 66 06"),
        Err(e) => panic!("SUT erred on valid pushw %es: {e}"),
    }
}

#[test]
fn encode_push16_kat_llvm_mc_mem() {
    let mc = llvm_mc_bytes("pushw (%eax)").expect("llvm-mc KAT mem");
    assert_eq!(mc, vec![0x66, 0xff, 0x30]);
    let sut = sut_encode(
        "pushw",
        vec![Operand::Memory(mem_base("eax", Displacement::None, None))],
    );
    match sut {
        Ok(bytes) => assert_eq!(bytes, mc, "SUT must encode pushw (%eax)"),
        Err(e) => panic!("SUT erred on valid pushw mem: {e}"),
    }
}

#[test]
fn encode_push16_kat_llvm_mc_fs_segment() {
    let mc = llvm_mc_bytes("pushw %fs:(%eax)").expect("llvm-mc KAT fs");
    assert_eq!(mc, vec![0x64, 0x66, 0xff, 0x30]);
    let sut = sut_encode(
        "pushw",
        vec![Operand::Memory(mem_base(
            "eax",
            Displacement::None,
            Some("fs"),
        ))],
    );
    match sut {
        Ok(bytes) => assert_eq!(bytes, mc, "SUT must emit FS override before pushw mem"),
        Err(e) => panic!("SUT erred on valid FS pushw: {e}"),
    }
}

/// Deterministic regression: r16 register form rejected (B1).
#[test]
fn test_encode_push16_regression_r16_unsupported() {
    let mc = llvm_mc_bytes("pushw %ax").expect("llvm-mc");
    assert_eq!(mc, vec![0x66, 0x50]);
    let sut = sut_encode("pushw", vec![Operand::Register(Register::new("ax"))]);
    match sut {
        Ok(bytes) => assert_eq!(bytes, mc, "pushw %ax must be 66 50"),
        Err(e) => panic!("regression: encode_push16 must accept r16, got Err({e})"),
    }
}

/// Deterministic regression: Sreg pushw rejected (B2).
#[test]
fn test_encode_push16_regression_sreg_unsupported() {
    let mc = llvm_mc_bytes("pushw %es").expect("llvm-mc");
    assert_eq!(mc, vec![0x66, 0x06]);
    let sut = sut_encode("pushw", vec![Operand::Register(Register::new("es"))]);
    match sut {
        Ok(bytes) => assert_eq!(bytes, mc, "pushw %es must be 66 06"),
        Err(e) => panic!("regression: encode_push16 must accept Sreg, got Err({e})"),
    }
}

/// Deterministic regression: memory form rejected (B3).
#[test]
fn test_encode_push16_regression_mem_unsupported() {
    let mc = llvm_mc_bytes("pushw (%ebx)").expect("llvm-mc");
    assert_eq!(mc, vec![0x66, 0xff, 0x33]);
    let sut = sut_encode(
        "pushw",
        vec![Operand::Memory(mem_base("ebx", Displacement::None, None))],
    );
    match sut {
        Ok(bytes) => assert_eq!(bytes, mc, "pushw (%ebx) must be 66 ff 33"),
        Err(e) => panic!("regression: encode_push16 must accept memory, got Err({e})"),
    }
}

/// Deterministic regression: segmented memory missing (B3 companion).
#[test]
fn test_encode_push16_regression_mem_segment_unsupported() {
    let mc = llvm_mc_bytes("pushw %es:(%eax)").expect("llvm-mc");
    assert_eq!(mc, vec![0x26, 0x66, 0xff, 0x30]);
    let sut = sut_encode(
        "pushw",
        vec![Operand::Memory(mem_base(
            "eax",
            Displacement::None,
            Some("es"),
        ))],
    );
    match sut {
        Ok(bytes) => assert_eq!(bytes, mc, "pushw %es:(%eax) must include 0x26"),
        Err(e) => panic!("regression: encode_push16 must accept segmented mem, got Err({e})"),
    }
}

proptest! {
    #![proptest_config(cfg())]

    // P1: differential — immediate (skewed to i8 / i16 boundaries)
    #[test]
    fn encode_push16_diff_imm(
        v in prop_oneof![
            Just(0i64),
            Just(-1i64),
            Just(1i64),
            Just(127i64),
            Just(-128i64),
            Just(128i64),
            Just(-129i64),
            Just(255i64),
            Just(-256i64),
            Just(0x7fffi64),
            Just(-0x8000i64),
            Just(0x8000i64),
            Just(-0x8001i64),
            Just(40000i64),
            Just(0x7fff_ffffi64),
            Just(-0x8000_0000i64),
            (-0x8000_0000i64..=0x7fff_ffffi64),
            (-200i64..=200i64),
            (-40000i64..=40000i64),
        ],
    ) {
        let asm = format!("pushw ${v}");
        let mc = llvm_mc_bytes(&asm)
            .map_err(|e| TestCaseError::fail(format!("llvm-mc rejected `{asm}`: {e}")))?;
        let sut = sut_encode(
            "pushw",
            vec![Operand::Immediate(ImmediateValue::Integer(v))],
        )
        .map_err(|e| TestCaseError::fail(format!("SUT rejected `{asm}`: {e}")))?;
        prop_assert_eq!(&sut, &mc, "imm diff `{}`: sut={:02x?} mc={:02x?}", asm, &sut, &mc);
    }

    // P2: differential — r16 short form 66 50+n
    #[test]
    fn encode_push16_diff_r16(r16 in prop::sample::select(R16)) {
        let asm = format!("pushw %{r16}");
        let mc = llvm_mc_bytes(&asm)
            .map_err(|e| TestCaseError::fail(format!("llvm-mc rejected `{asm}`: {e}")))?;
        let sut = sut_encode("pushw", vec![Operand::Register(Register::new(r16))])
            .map_err(|e| TestCaseError::fail(format!(
                "SUT rejected valid r16 form `{asm}`: {e}; \
                 Intel PUSH r16 / AT&T pushw %r16 must encode as 0x66 0x50+rw \
                 (sibling encode_push handles r32; encode_pop16 handles r16)."
            )))?;
        prop_assert_eq!(&sut, &mc, "r16 diff `{}`: sut={:02x?} mc={:02x?}", asm, &sut, &mc);
    }

    // P3: differential — Sreg pushw (66-prefixed)
    #[test]
    fn encode_push16_diff_sreg(sreg in prop::sample::select(SREGS)) {
        let asm = format!("pushw %{sreg}");
        let mc = llvm_mc_bytes(&asm)
            .map_err(|e| TestCaseError::fail(format!("llvm-mc rejected `{asm}`: {e}")))?;
        let sut = sut_encode("pushw", vec![Operand::Register(Register::new(sreg))])
            .map_err(|e| TestCaseError::fail(format!(
                "SUT rejected valid Sreg form `{asm}`: {e}; \
                 Intel PUSH Sreg with operand-size override must encode under pushw."
            )))?;
        prop_assert_eq!(&sut, &mc, "sreg diff `{}`: sut={:02x?} mc={:02x?}", asm, &sut, &mc);
    }

    // P4: differential — memory, no segment
    #[test]
    fn encode_push16_diff_mem(
        kind in 0u8..12,
        base in prop::sample::select(GP32),
        index in prop::sample::select(
            GP32.iter().copied().filter(|r| *r != "esp").collect::<Vec<_>>()
        ),
        scale in prop::sample::select(SCALES),
        disp in prop_oneof![
            Just(0i64),
            Just(-1i64),
            Just(1i64),
            Just(127i64),
            Just(-128i64),
            Just(128i64),
            Just(-129i64),
            Just(0x1234i64),
            (-512i64..=512i64),
        ],
    ) {
        let mem = match kind {
            0 => mem_base(base, Displacement::None, None),
            1 => mem_base(base, Displacement::Integer(disp), None),
            2 => mem_base("esp", Displacement::None, None),
            3 => mem_base("ebp", Displacement::None, None),
            4 => mem_base("esp", Displacement::Integer(disp), None),
            5 => mem_base("ebp", Displacement::Integer(if disp == 0 { 1 } else { disp }), None),
            6 => mem_base_index(Some(base), index, scale, Displacement::None, None),
            7 => mem_base_index(Some(base), index, scale, Displacement::Integer(disp), None),
            8 => mem_base_index(Some("esp"), index, scale, Displacement::Integer(disp), None),
            9 => mem_abs(disp, None),
            10 => mem_abs(0x1234_5678, None),
            _ => mem_abs(0, None),
        };
        let asm = format!("pushw {}", att_mem(&mem));
        let mc = match llvm_mc_bytes(&asm) {
            Ok(b) => b,
            Err(_) => return Ok(()), // skip unencodable edge
        };
        let sut = sut_encode("pushw", vec![Operand::Memory(mem)])
            .map_err(|e| TestCaseError::fail(format!(
                "SUT rejected valid mem form `{asm}`: {e}; \
                 Intel PUSH r/m16 = 0x66 + FF /6 + ModR/M (sibling encode_push uses FF /6)."
            )))?;
        prop_assert_eq!(&sut, &mc, "mem diff `{}`: sut={:02x?} mc={:02x?}", asm, &sut, &mc);
    }

    // P5: differential — all six segment overrides on memory form
    #[test]
    fn encode_push16_diff_mem_segment(
        seg in prop::sample::select(SREGS),
        base in prop::sample::select(GP32),
        disp in prop_oneof![Just(0i64), Just(8i64), Just(-4i64), Just(127i64), Just(-128i64)],
    ) {
        let mem = mem_base(base, Displacement::Integer(disp), Some(seg));
        let asm = format!("pushw {}", att_mem(&mem));
        let mc = llvm_mc_bytes(&asm)
            .map_err(|e| TestCaseError::fail(format!("llvm-mc rejected `{asm}`: {e}")))?;
        let sut = sut_encode("pushw", vec![Operand::Memory(mem)])
            .map_err(|e| TestCaseError::fail(format!(
                "SUT rejected valid segment form `{asm}`: {e}; \
                 i686 PUSHW must emit segment override (core.rs emit_segment_prefix)."
            )))?;
        prop_assert_eq!(
            &sut, &mc,
            "segment diff `{}`: sut={:02x?} mc={:02x?}",
            asm, &sut, &mc
        );
    }

    // P6: algebraic.invariant — imm form choice at ±128 boundary + 0x66
    #[test]
    fn encode_push16_invariant_imm_form(
        v in prop_oneof![
            Just(-128i64), Just(-129i64), Just(127i64), Just(128i64),
            Just(0i64), Just(-1i64), Just(1i64),
            Just(0x7fffi64), Just(-0x8000i64),
            Just(0x7fff_ffffi64), Just(-0x8000_0000i64),
            (-0x8000_0000i64..=0x7fff_ffffi64),
        ],
    ) {
        let bytes = sut_encode(
            "pushw",
            vec![Operand::Immediate(ImmediateValue::Integer(v))],
        )
        .map_err(|e| TestCaseError::fail(e))?;
        prop_assert_eq!(bytes[0], 0x66u8, "pushw must start with 0x66, got {:02x?}", bytes);
        if v >= -128 && v <= 127 {
            let expect = vec![0x66u8, 0x6Au8, v as u8];
            prop_assert_eq!(&bytes, &expect, "imm8 form for v={}: got {:02x?}", v, bytes);
        } else {
            let mut expect = vec![0x66u8, 0x68u8];
            expect.extend_from_slice(&(v as i16).to_le_bytes());
            prop_assert_eq!(&bytes, &expect, "imm16 form for v={}: got {:02x?}", v, bytes);
        }
    }

    // P7: algebraic.metamorphic — pushw imm8 = 0x66 ‖ pushl imm8
    #[test]
    fn encode_push16_metamorphic_imm8_vs_pushl(
        v in prop_oneof![
            Just(0i64), Just(-1i64), Just(1i64), Just(127i64), Just(-128i64),
            (-128i64..=127i64),
        ],
    ) {
        let pushw = sut_encode(
            "pushw",
            vec![Operand::Immediate(ImmediateValue::Integer(v))],
        )
        .map_err(|e| TestCaseError::fail(format!("pushw: {e}")))?;
        let pushl = sut_encode(
            "pushl",
            vec![Operand::Immediate(ImmediateValue::Integer(v))],
        )
        .map_err(|e| TestCaseError::fail(format!("pushl: {e}")))?;
        let mut expect = vec![0x66u8];
        expect.extend_from_slice(&pushl);
        prop_assert_eq!(
            &pushw, &expect,
            "metamorphic imm8 v={}: pushw={:02x?} 66‖pushl={:02x?}",
            v, &pushw, &expect
        );
    }

    // P8: negative — arity ≠ 1
    #[test]
    fn encode_push16_neg_arity(n in 0usize..4) {
        prop_assume!(n != 1);
        let ops: Vec<Operand> = (0..n)
            .map(|_| Operand::Immediate(ImmediateValue::Integer(0)))
            .collect();
        let sut = sut_encode("pushw", ops);
        prop_assert!(
            sut.is_err(),
            "arity {n} must be Err, got {sut:?}"
        );
    }

    // P9a: negative — r32 rejected (llvm-mc rejects; guard against reg_num alias)
    #[test]
    fn encode_push16_neg_r32(r32 in prop::sample::select(R32)) {
        let asm = format!("pushw %{r32}");
        prop_assert!(
            llvm_mc_bytes(&asm).is_err(),
            "llvm-mc must reject `{asm}`"
        );
        let sut = sut_encode("pushw", vec![Operand::Register(Register::new(r32))]);
        prop_assert!(
            sut.is_err(),
            "pushw %{} must be Err (not r16), got {sut:?}",
            r32
        );
    }

    // P9b: negative — r8 rejected
    #[test]
    fn encode_push16_neg_r8(r8 in prop::sample::select(R8)) {
        let asm = format!("pushw %{r8}");
        prop_assert!(
            llvm_mc_bytes(&asm).is_err(),
            "llvm-mc must reject `{asm}`"
        );
        let sut = sut_encode("pushw", vec![Operand::Register(Register::new(r8))]);
        prop_assert!(
            sut.is_err(),
            "pushw %{} must be Err (not r16), got {sut:?}",
            r8
        );
    }
}
