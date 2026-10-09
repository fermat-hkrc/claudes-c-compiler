// Oracle: differential — llvm-mc i686 assembler (PUSH r32 / imm / m32 / Sreg)
// Evidence: gp_integer.rs:346 encode_push; mod.rs:191 "pushl|push" => encode_push;
//   Intel SDM Vol.2 PUSH — r32 short 50+rd, imm8 6A ib, imm32 68 id, r/m32 FF /6,
//   Sreg one-byte/0F forms; core.rs:31-42 emit_segment_prefix for fs/gs/es/cs/ss/ds;
//   x86-64 sibling gp_integer.rs:369 calls emit_segment_prefix before FF /6.
// Stronger considered:
//   - State machine: rejected — pure encoding, no lifecycle
//   - Algebraic round-trip: rejected — no in-tree i686 decoder for push
// Differential: candidate=encode_push (via InstructionEncoder::encode pushl/push),
//   reference=llvm-mc -triple=i686 -show-encoding (LLVM 15),
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=AT&T operand bytes <-> llvm-mc encoding bytes.
// Also: algebraic.invariant (r32=50+n; imm form), algebraic.metamorphic
//   (segmented mem = seg_prefix ‖ bare mem), negative_error (arity / xmm / empty).

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

const GP32: &[&str] = &["eax", "ecx", "edx", "ebx", "esp", "ebp", "esi", "edi"];
const R16: &[&str] = &["ax", "cx", "dx", "bx", "sp", "bp", "si", "di"];
const R8: &[&str] = &["al", "cl", "dl", "bl", "ah", "ch", "dh", "bh"];
const SREGS: &[&str] = &["es", "cs", "ss", "ds", "fs", "gs"];
const XMM: &[&str] = &["xmm0", "xmm1", "xmm2", "xmm3", "xmm4", "xmm5", "xmm6", "xmm7"];
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

fn seg_prefix_byte(seg: &str) -> u8 {
    match seg {
        "es" => 0x26,
        "cs" => 0x2E,
        "ss" => 0x36,
        "ds" => 0x3E,
        "fs" => 0x64,
        "gs" => 0x65,
        _ => panic!("bad seg {seg}"),
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
        // llvm-mc may emit bare 'A' fixup placeholders for relocatable immediates
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

/// Strip leading segment-override / operand-size prefixes for body checks.
fn strip_seg_prefixes(bytes: &[u8]) -> &[u8] {
    let mut i = 0;
    while i < bytes.len() && matches!(bytes[i], 0x26 | 0x2E | 0x36 | 0x3E | 0x64 | 0x65 | 0x66) {
        i += 1;
    }
    &bytes[i..]
}

// --- KAT gate (reference oracle prerequisite) ---

#[test]
fn encode_push_kat_llvm_mc_eax() {
    let mc = llvm_mc_bytes("pushl %eax").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x50], "llvm-mc KAT mapping broken");
    let sut = sut_encode("pushl", vec![Operand::Register(Register::new("eax"))])
        .expect("SUT KAT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_push_kat_llvm_mc_imm8() {
    let mc = llvm_mc_bytes("pushl $0").expect("llvm-mc KAT imm0");
    assert_eq!(mc, vec![0x6a, 0x00]);
    let sut = sut_encode(
        "pushl",
        vec![Operand::Immediate(ImmediateValue::Integer(0))],
    )
    .expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_push_kat_llvm_mc_imm32() {
    let mc = llvm_mc_bytes("pushl $128").expect("llvm-mc KAT imm128");
    assert_eq!(mc, vec![0x68, 0x80, 0x00, 0x00, 0x00]);
    let sut = sut_encode(
        "pushl",
        vec![Operand::Immediate(ImmediateValue::Integer(128))],
    )
    .expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_push_kat_llvm_mc_mem() {
    let mc = llvm_mc_bytes("pushl (%ebx)").expect("llvm-mc KAT mem");
    assert_eq!(mc, vec![0xff, 0x33]);
    let sut = sut_encode(
        "pushl",
        vec![Operand::Memory(mem_base("ebx", Displacement::None, None))],
    )
    .expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_push_kat_llvm_mc_fs_segment() {
    // Contract: all six segment overrides are valid on i686 (core.rs emit_segment_prefix).
    let mc = llvm_mc_bytes("pushl %fs:(%eax)").expect("llvm-mc KAT fs");
    assert_eq!(mc, vec![0x64, 0xff, 0x30]);
    let sut = sut_encode(
        "pushl",
        vec![Operand::Memory(mem_base(
            "eax",
            Displacement::None,
            Some("fs"),
        ))],
    );
    match sut {
        Ok(bytes) => assert_eq!(bytes, mc, "SUT must emit FS override 0x64 before FF /6"),
        Err(e) => panic!("SUT erred on valid FS push: {e}"),
    }
}

#[test]
fn encode_push_kat_llvm_mc_es_segment() {
    let mc = llvm_mc_bytes("pushl %es:(%ebx)").expect("llvm-mc KAT es");
    assert_eq!(mc, vec![0x26, 0xff, 0x33]);
    let sut = sut_encode(
        "pushl",
        vec![Operand::Memory(mem_base(
            "ebx",
            Displacement::None,
            Some("es"),
        ))],
    );
    match sut {
        Ok(bytes) => assert_eq!(bytes, mc, "SUT must emit ES override 0x26 before FF /6"),
        Err(e) => panic!("SUT erred on valid ES push: {e}"),
    }
}

/// Deterministic regression: missing segment override (B1).
/// Witness: pushl %fs:(%eax) → SUT omits 0x64.
#[test]
fn encode_push_regression_missing_fs_prefix() {
    let mc = llvm_mc_bytes("pushl %fs:(%eax)").expect("llvm-mc");
    assert_eq!(mc, vec![0x64, 0xff, 0x30]);
    let sut = sut_encode(
        "pushl",
        vec![Operand::Memory(mem_base(
            "eax",
            Displacement::None,
            Some("fs"),
        ))],
    )
    .expect("valid FS push must encode");
    assert_eq!(
        sut, mc,
        "regression: encode_push must emit segment override before 0xFF /6"
    );
}

/// Deterministic regression: non-GP xmm accepted via reg_num alias (B2).
/// Witness: pushl %xmm0 → SUT emits [0x50].
#[test]
fn encode_push_regression_non_gp_xmm0() {
    let asm = "pushl %xmm0";
    assert!(
        llvm_mc_bytes(asm).is_err(),
        "llvm-mc must reject non-GP PUSH operand"
    );
    let sut = sut_encode("pushl", vec![Operand::Register(Register::new("xmm0"))]);
    assert!(
        sut.is_err(),
        "regression: encode_push must reject non-GP xmm0, got {sut:?}"
    );
}

/// Deterministic regression: Sreg push forms missing (B3).
/// Witness: pushl %es → llvm-mc [0x06], SUT "bad register".
#[test]
fn encode_push_regression_sreg_es() {
    let mc = llvm_mc_bytes("pushl %es").expect("llvm-mc es");
    assert_eq!(mc, vec![0x06]);
    let sut = sut_encode("pushl", vec![Operand::Register(Register::new("es"))]);
    match sut {
        Ok(bytes) => assert_eq!(bytes, mc, "SUT must encode push %es as 0x06"),
        Err(e) => panic!("regression: encode_push must accept Sreg push %es, got Err({e})"),
    }
}

proptest! {
    #![proptest_config(cfg())]

    // P1: differential — r32 short form
    #[test]
    fn encode_push_diff_r32(r in prop::sample::select(GP32)) {
        let asm = format!("pushl %{r}");
        let mc = llvm_mc_bytes(&asm)
            .map_err(|e| TestCaseError::fail(format!("llvm-mc rejected `{asm}`: {e}")))?;
        let sut = sut_encode("pushl", vec![Operand::Register(Register::new(r))])
            .map_err(|e| TestCaseError::fail(format!("SUT rejected `{asm}`: {e}")))?;
        prop_assert_eq!(&sut, &mc, "r32 diff `{}`: sut={:02x?} mc={:02x?}", asm, &sut, &mc);
    }

    // P2: differential — immediate (skewed to i8 boundaries)
    #[test]
    fn encode_push_diff_imm(
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
            Just(0x7fff_ffffi64),
            Just(-0x8000_0000i64),
            (-0x8000_0000i64..=0x7fff_ffffi64),
            (-200i64..=200i64),
        ],
    ) {
        let asm = format!("pushl ${v}");
        let mc = llvm_mc_bytes(&asm)
            .map_err(|e| TestCaseError::fail(format!("llvm-mc rejected `{asm}`: {e}")))?;
        let sut = sut_encode(
            "pushl",
            vec![Operand::Immediate(ImmediateValue::Integer(v))],
        )
        .map_err(|e| TestCaseError::fail(format!("SUT rejected `{asm}`: {e}")))?;
        prop_assert_eq!(&sut, &mc, "imm diff `{}`: sut={:02x?} mc={:02x?}", asm, &sut, &mc);
    }

    // P3: differential — memory, no segment
    #[test]
    fn encode_push_diff_mem(
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
        let asm = format!("pushl {}", att_mem(&mem));
        let mc = match llvm_mc_bytes(&asm) {
            Ok(b) => b,
            Err(_) => return Ok(()), // skip unencodable edge
        };
        let sut = sut_encode("pushl", vec![Operand::Memory(mem)])
            .map_err(|e| TestCaseError::fail(format!("SUT rejected `{asm}`: {e}")))?;
        prop_assert_eq!(&sut, &mc, "mem diff `{}`: sut={:02x?} mc={:02x?}", asm, &sut, &mc);
    }

    // P4: differential — all six segment overrides on memory form
    #[test]
    fn encode_push_diff_mem_segment(
        seg in prop::sample::select(SREGS),
        base in prop::sample::select(GP32),
        disp in prop_oneof![Just(0i64), Just(8i64), Just(-4i64), Just(127i64), Just(-128i64)],
    ) {
        let mem = mem_base(base, Displacement::Integer(disp), Some(seg));
        let asm = format!("pushl {}", att_mem(&mem));
        let mc = llvm_mc_bytes(&asm)
            .map_err(|e| TestCaseError::fail(format!("llvm-mc rejected `{asm}`: {e}")))?;
        let sut = sut_encode("pushl", vec![Operand::Memory(mem)])
            .map_err(|e| TestCaseError::fail(format!(
                "SUT rejected valid segment form `{asm}`: {e}; \
                 i686 PUSH must emit segment override (core.rs emit_segment_prefix; \
                 x86-64 sibling does; Intel SDM 2.1.1)."
            )))?;
        prop_assert_eq!(
            &sut, &mc,
            "segment diff `{}`: sut={:02x?} mc={:02x?}",
            asm, &sut, &mc
        );
    }

    // P5: algebraic.invariant — r32 short form = [0x50+n]
    #[test]
    fn encode_push_invariant_r32_opcode(r in prop::sample::select(GP32)) {
        let bytes = sut_encode("pushl", vec![Operand::Register(Register::new(r))])
            .map_err(|e| TestCaseError::fail(e))?;
        prop_assert_eq!(bytes.len(), 1, "r32 push is 1 byte, got {:02x?}", bytes);
        prop_assert_eq!(bytes[0], 0x50 + reg_num(r), "opcode 0x50+n for {}", r);
    }

    // P6: algebraic.invariant — imm form choice at ±128 boundary
    #[test]
    fn encode_push_invariant_imm_form(
        v in prop_oneof![
            Just(-128i64), Just(-129i64), Just(127i64), Just(128i64),
            Just(0i64), Just(-1i64), Just(1i64),
            Just(0x7fff_ffffi64), Just(-0x8000_0000i64),
            (-0x8000_0000i64..=0x7fff_ffffi64),
        ],
    ) {
        let bytes = sut_encode(
            "pushl",
            vec![Operand::Immediate(ImmediateValue::Integer(v))],
        )
        .map_err(|e| TestCaseError::fail(e))?;
        if v >= -128 && v <= 127 {
            let expect = vec![0x6Au8, v as u8];
            prop_assert_eq!(&bytes, &expect, "imm8 form for v={}: got {:02x?}", v, bytes);
        } else {
            let mut expect = vec![0x68u8];
            expect.extend_from_slice(&(v as i32).to_le_bytes());
            prop_assert_eq!(&bytes, &expect, "imm32 form for v={}: got {:02x?}", v, bytes);
        }
    }

    // P7: algebraic.metamorphic — segmented = seg_prefix ‖ bare; strip recovers bare
    #[test]
    fn encode_push_meta_segment_stripped_eq_bare(
        seg in prop::sample::select(SREGS),
        base in prop::sample::select(GP32),
        disp in prop_oneof![Just(0i64), Just(4i64), Just(-1i64), Just(127i64)],
    ) {
        let bare_mem = mem_base(base, Displacement::Integer(disp), None);
        let seg_mem = mem_base(base, Displacement::Integer(disp), Some(seg));
        let bare = sut_encode("pushl", vec![Operand::Memory(bare_mem)])
            .map_err(|e| TestCaseError::fail(format!("bare: {e}")))?;
        let with_seg = sut_encode("pushl", vec![Operand::Memory(seg_mem)])
            .map_err(|e| TestCaseError::fail(format!(
                "segmented push must encode (emit_segment_prefix contract): {e}"
            )))?;
        let expect_prefix = seg_prefix_byte(seg);
        prop_assert!(
            !with_seg.is_empty() && with_seg[0] == expect_prefix,
            "segmented push must start with 0x{expect_prefix:02x} for %{seg}:, got {:02x?}",
            with_seg
        );
        let stripped = strip_seg_prefixes(&with_seg);
        prop_assert_eq!(
            stripped, &bare[..],
            "strip_seg(seg-push) must equal bare-push; seg={:02x?} bare={:02x?}",
            with_seg, bare
        );
    }

    // P8: negative_error — wrong arity
    #[test]
    fn encode_push_neg_arity(n in 0usize..4) {
        prop_assume!(n != 1);
        let ops: Vec<Operand> = (0..n)
            .map(|_| Operand::Register(Register::new("eax")))
            .collect();
        let r = sut_encode("pushl", ops);
        prop_assert!(r.is_err(), "arity {n} must be Err, got {r:?}");
    }

    // P9: negative_error — non-GP xmm must be rejected (llvm-mc rejects)
    #[test]
    fn encode_push_neg_xmm(x in prop::sample::select(XMM)) {
        let asm = format!("pushl %{x}");
        prop_assert!(
            llvm_mc_bytes(&asm).is_err(),
            "llvm-mc must reject `{asm}`"
        );
        let r = sut_encode("pushl", vec![Operand::Register(Register::new(x))]);
        prop_assert!(
            r.is_err(),
            "encode_push must reject non-GP `{x}`, got {r:?}"
        );
    }

    // P10: differential — Sreg push forms (Intel SDM one-byte / 0F)
    #[test]
    fn encode_push_diff_sreg(sreg in prop::sample::select(SREGS)) {
        let asm = format!("pushl %{sreg}");
        let mc = llvm_mc_bytes(&asm)
            .map_err(|e| TestCaseError::fail(format!("llvm-mc rejected `{asm}`: {e}")))?;
        let sut = sut_encode("pushl", vec![Operand::Register(Register::new(sreg))])
            .map_err(|e| TestCaseError::fail(format!(
                "SUT rejected Sreg push `{asm}`: {e}; Intel SDM PUSH Sreg is valid"
            )))?;
        prop_assert_eq!(&sut, &mc, "Sreg diff `{}`: sut={:02x?} mc={:02x?}", asm, &sut, &mc);
    }

    // P11: negative-ish / width — r16 via pushl should not silently alias r32
    // (llvm-mc accepts `push %ax` as pushw with 0x66; bare pushl %ax is rejected or 16-bit)
    #[test]
    fn encode_push_diff_r16_via_push(
        r16 in prop::sample::select(R16),
    ) {
        // Via mnemonic "push" (same encode_push path as pushl)
        let asm = format!("push %{r16}");
        let mc = match llvm_mc_bytes(&asm) {
            Ok(b) => b,
            Err(_) => return Ok(()),
        };
        let sut = match sut_encode("push", vec![Operand::Register(Register::new(r16))]) {
            Ok(b) => b,
            Err(_) => {
                // SUT rejecting r16 where llvm accepts is a finding
                return Err(TestCaseError::fail(format!(
                    "SUT rejected `{asm}` which llvm-mc encodes as {:02x?}",
                    mc
                )));
            }
        };
        prop_assert_eq!(
            &sut, &mc,
            "r16 push diff `{}`: sut={:02x?} mc={:02x?} \
             (must include 0x66 operand-size override, not bare 0x50+n)",
            asm, &sut, &mc
        );
    }

    // P12: negative_error — r8 is invalid for push
    #[test]
    fn encode_push_neg_r8(r8 in prop::sample::select(R8)) {
        let asm = format!("pushl %{r8}");
        // llvm-mc may or may not reject; if it rejects, SUT must too
        if llvm_mc_bytes(&asm).is_err() {
            let r = sut_encode("pushl", vec![Operand::Register(Register::new(r8))]);
            prop_assert!(
                r.is_err(),
                "encode_push must reject r8 `{r8}` when llvm-mc does, got {r:?}"
            );
        }
    }

    // P13 (sweep): algebraic.invariant — Symbol immediate → 0x68 + 4 zero reloc bytes
    #[test]
    fn encode_push_invariant_symbol_imm(
        name in prop::sample::select(&["foo", "bar", "sym", "_start", "label"]),
        addend in prop_oneof![Just(0i64), Just(4i64), Just(-8i64), Just(16i64)],
        use_offset in any::<bool>(),
    ) {
        let op = if use_offset && addend != 0 {
            Operand::Immediate(ImmediateValue::SymbolPlusOffset(name.to_string(), addend))
        } else {
            Operand::Immediate(ImmediateValue::Symbol(name.to_string()))
        };
        let bytes = sut_encode("pushl", vec![op])
            .map_err(|e| TestCaseError::fail(e))?;
        prop_assert_eq!(bytes.len(), 5, "symbol imm push is 5 bytes, got {:02x?}", bytes);
        prop_assert_eq!(bytes[0], 0x68u8, "opcode 0x68 for symbol imm");
        prop_assert_eq!(&bytes[1..], &[0, 0, 0, 0], "reloc placeholder zeros");
    }

    // P14 (sweep): negative_error — empty / two-imm / bare label-less unsupported shapes
    // (arity already covered; this hits the `_ => Err("unsupported push operand")` arm via
    // a Register that is not matched... actually Register always matches. Use no operands
    // already covered. Hit unsupported by using a second path: only Operand variants that
    // fall through are non-Reg/Imm/Mem. If parser has Label-like forms as other enums,
    // skip; otherwise arity-0 already exercises early return. Keep as crash-free smoke
    // that extra GP + Imm is rejected.)
    #[test]
    fn encode_push_neg_extra_mixed(
        r in prop::sample::select(GP32),
        v in -128i64..=127i64,
    ) {
        let ops = vec![
            Operand::Register(Register::new(r)),
            Operand::Immediate(ImmediateValue::Integer(v)),
        ];
        let res = sut_encode("pushl", ops);
        prop_assert!(res.is_err(), "two operands must Err, got {res:?}");
    }
}

/// Sweep KAT: symbol immediate form (gp_integer.rs:366-372).
#[test]
fn encode_push_kat_symbol_imm() {
    let sut = sut_encode(
        "pushl",
        vec![Operand::Immediate(ImmediateValue::Symbol("foo".into()))],
    )
    .expect("symbol imm must encode");
    assert_eq!(sut, vec![0x68, 0, 0, 0, 0]);
}

/// Sweep KAT: SymbolPlusOffset.
#[test]
fn encode_push_kat_symbol_plus_offset() {
    let sut = sut_encode(
        "pushl",
        vec![Operand::Immediate(ImmediateValue::SymbolPlusOffset(
            "foo".into(),
            4,
        ))],
    )
    .expect("symbol+off must encode");
    assert_eq!(sut, vec![0x68, 0, 0, 0, 0]);
}
