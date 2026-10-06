// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md:239 System table lists hint;
//   encoder/mod.rs:3 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:967 "hint" => encode_hint(operands);
//   ARM ARM HINT: 1101 0101 0000 0011 0010 CRm op2 11111 = 0xD503201F | (CRm << 8) | (op2 << 5);
//   CRm = imm[6:3], op2 = imm[2:0]; imm ∈ 0..=127;
//   llvm-mc -triple=aarch64 -show-encoding (LLVM 15.0.6);
//   gas: "immediate value out of range 0 to 127", "missing immediate expression",
//        "unexpected characters following instruction", "immediate operand required".
// Stronger considered:
//   - State machine: rejected — encode_hint is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree HINT decoder
//   - Differential vs NOP/YIELD/WFE/WFI/SEV/SEVL/BTI: rejected — same-job gate
//     (those mnemonics are HINT aliases without a free imm)
// Weaker available: algebraic.metamorphic (imm field isolation bits[11:5]),
//   algebraic.invariant (ARM HINT layout),
//   negative_error (extra / oob imm / empty / wrong kind)
// Differential: candidate=encode_hint, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[Imm(n)] <-> `hint #n`

use super::encode_hint;
use super::EncodeResult;
use crate::backend::arm::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;

/// ARM ARM HINT with imm=0: 1101 0101 0000 0011 0010 0000 000 11111
const HINT_TEMPLATE: u32 = 0xd503201f;
const HINT_HI: u32 = 0xd5032; // bits[31:12]
const IMM7_MASK: u32 = 0xfe0; // bits[11:5]

fn cfg() -> ProptestConfig {
    ProptestConfig::with_cases(CASES)
}

/// ARM ARM HINT word for a valid 7-bit imm. Independent of the SUT mask.
fn arm_hint_word(imm: u8) -> u32 {
    debug_assert!(imm <= 127);
    HINT_TEMPLATE | ((imm as u32) << 5)
}

fn sut_word(ops: &[Operand]) -> Result<u32, String> {
    match encode_hint(ops)? {
        EncodeResult::Word(w) => Ok(w),
        other => Err(format!("expected Word, got {:?}", other)),
    }
}

fn parse_llvm_encoding(stdout: &str) -> Result<u32, String> {
    let marker = "encoding: [";
    let start = stdout
        .find(marker)
        .ok_or_else(|| format!("no encoding in stdout: {stdout}"))?;
    let rest = &stdout[start + marker.len()..];
    let end = rest
        .find(']')
        .ok_or_else(|| format!("no closing bracket: {stdout}"))?;
    let inner = &rest[..end];
    let mut bytes = [0u8; 4];
    let parts: Vec<&str> = inner.split(',').collect();
    if parts.len() != 4 {
        return Err(format!("expected 4 bytes, got {inner}"));
    }
    for (i, p) in parts.iter().enumerate() {
        let p = p.trim();
        let hex = p
            .strip_prefix("0x")
            .ok_or_else(|| format!("non-hex byte {p}"))?;
        bytes[i] = u8::from_str_radix(hex, 16).map_err(|e| e.to_string())?;
    }
    Ok(u32::from_le_bytes(bytes))
}

fn llvm_mc_word(asm: &str) -> Result<u32, String> {
    let mut child = Command::new(LLVM_MC)
        .args(["-triple=aarch64", "-show-encoding"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("spawn llvm-mc: {e}"))?;
    {
        let mut stdin = child.stdin.take().ok_or("llvm-mc stdin")?;
        stdin
            .write_all(asm.as_bytes())
            .map_err(|e| format!("write llvm-mc: {e}"))?;
        stdin
            .write_all(b"\n")
            .map_err(|e| format!("write llvm-mc: {e}"))?;
    }
    let out = child
        .wait_with_output()
        .map_err(|e| format!("wait llvm-mc: {e}"))?;
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    if !out.status.success() || stderr.contains("error:") {
        return Err(format!("llvm-mc error: {stderr}"));
    }
    parse_llvm_encoding(&stdout)
}

fn extra_operand() -> impl Strategy<Value = Operand> {
    prop_oneof![
        (0u32..=31).prop_map(|n| Operand::Reg(format!("x{n}"))),
        (0u32..=31).prop_map(|n| Operand::Reg(format!("w{n}"))),
        Just(Operand::Reg("sp".into())),
        Just(Operand::Reg("xzr".into())),
        (-4i64..=32).prop_map(Operand::Imm),
        Just(Operand::Barrier("sy".into())),
        Just(Operand::Cond("eq".into())),
        Just(Operand::Label(".L0".into())),
        Just(Operand::Symbol("foo".into())),
    ]
}

fn extra_asm(extra: &Operand) -> String {
    match extra {
        Operand::Reg(r) => r.clone(),
        Operand::Imm(n) => format!("#{n}"),
        Operand::Barrier(b) => b.clone(),
        Operand::Cond(c) => c.clone(),
        Operand::Label(l) => l.clone(),
        Operand::Symbol(s) => s.clone(),
        _ => "x0".to_string(),
    }
}

fn oob_imm() -> impl Strategy<Value = i64> {
    prop_oneof![
        Just(-1i64),
        Just(128i64),
        Just(129i64),
        Just(-2i64),
        Just(-128i64),
        Just(-127i64),
        Just(i64::MIN),
        Just(i64::MAX),
        Just(255i64),
        Just(256i64),
        Just(0x10000i64),
        Just(1i64 << 32),
        Just((1i64 << 32) + 1),
        Just((1i64 << 7) + 42),
        (-4096i64..=-2),
        (128i64..=70000),
    ]
}

fn wrong_kind() -> impl Strategy<Value = Operand> {
    prop_oneof![
        (0u32..=31).prop_map(|n| Operand::Reg(format!("x{n}"))),
        (0u32..=31).prop_map(|n| Operand::Reg(format!("w{n}"))),
        Just(Operand::Reg("sp".into())),
        Just(Operand::Reg("xzr".into())),
        Just(Operand::Reg("d0".into())),
        Just(Operand::Symbol("hint".into())),
        Just(Operand::Barrier("sy".into())),
        Just(Operand::Cond("eq".into())),
        Just(Operand::Label(".LBB0".into())),
        Just(Operand::Mem {
            base: "x0".into(),
            offset: 0,
        }),
        Just(Operand::Shift {
            kind: "lsl".into(),
            amount: 0,
        }),
        Just(Operand::Extend {
            kind: "sxtw".into(),
            amount: 0,
        }),
    ]
}

fn asm_for_wrong(op: &Operand) -> Option<String> {
    match op {
        Operand::Reg(r) => Some(format!("hint {r}")),
        Operand::Symbol(s) => Some(format!("hint {s}")),
        Operand::Barrier(b) => Some(format!("hint {b}")),
        Operand::Cond(c) => Some(format!("hint {c}")),
        Operand::Label(l) => Some(format!("hint {l}")),
        Operand::Mem { base, offset } => Some(format!("hint [{base}, #{offset}]")),
        Operand::Shift { kind, amount } => Some(format!("hint {kind} #{amount}")),
        Operand::Extend { kind, amount } => Some(format!("hint {kind} #{amount}")),
        _ => None,
    }
}

#[test]
fn encode_hint_kat_llvm_mc_imm0() {
    let mc = llvm_mc_word("hint #0").expect("llvm-mc hint #0");
    assert_eq!(mc, 0xd503201f, "llvm-mc KAT mapping broken for hint #0");
    let sut = sut_word(&[Operand::Imm(0)]).expect("SUT KAT imm0");
    assert_eq!(sut, mc);
}

#[test]
fn encode_hint_kat_llvm_mc_imm1() {
    let mc = llvm_mc_word("hint #1").expect("llvm-mc hint #1");
    assert_eq!(mc, 0xd503203f, "llvm-mc KAT mapping broken for hint #1");
    let sut = sut_word(&[Operand::Imm(1)]).expect("SUT KAT imm1");
    assert_eq!(sut, mc);
}

#[test]
fn encode_hint_kat_llvm_mc_imm7() {
    let mc = llvm_mc_word("hint #7").expect("llvm-mc hint #7");
    assert_eq!(mc, 0xd50320ff, "llvm-mc KAT mapping broken for hint #7");
    let sut = sut_word(&[Operand::Imm(7)]).expect("SUT KAT imm7");
    assert_eq!(sut, mc);
}

#[test]
fn encode_hint_kat_llvm_mc_imm127() {
    let mc = llvm_mc_word("hint #127").expect("llvm-mc hint #127");
    assert_eq!(mc, 0xd5032fff, "llvm-mc KAT mapping broken for hint #127");
    let sut = sut_word(&[Operand::Imm(127)]).expect("SUT KAT imm127");
    assert_eq!(sut, mc);
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_hint_diff_imm(imm in 0i64..=127) {
        let asm = format!("hint #{imm}");
        let mc = llvm_mc_word(&asm).expect(&asm);
        let expect = arm_hint_word(imm as u8);
        prop_assert_eq!(mc, expect, "llvm-mc vs ARM ARM for {}", asm);
        let sut = sut_word(&[Operand::Imm(imm)]).expect(&asm);
        prop_assert_eq!(sut, mc, "SUT vs llvm-mc for {}", asm);
    }

    #[test]
    fn encode_hint_inv_arm_layout(imm in 0i64..=127) {
        let w = sut_word(&[Operand::Imm(imm)]).expect("valid imm7 must encode");
        let expect = arm_hint_word(imm as u8);
        prop_assert_eq!(w, expect, "ARM HINT word for #{}", imm);
        prop_assert_eq!(w >> 12, HINT_HI, "bits[31:12] HINT group");
        prop_assert_eq!((w >> 5) & 0x7f, imm as u32, "imm7 field bits[11:5]");
        prop_assert_eq!(w & 0x1f, 0b11111, "Rt bits[4:0]=11111");
    }

    #[test]
    fn encode_hint_meta_imm_isolation(imm1 in 0i64..=127, imm2 in 0i64..=127) {
        let w1 = sut_word(&[Operand::Imm(imm1)]).expect("imm1");
        let w2 = sut_word(&[Operand::Imm(imm2)]).expect("imm2");
        let w0 = sut_word(&[Operand::Imm(0)]).expect("imm0");
        prop_assert_eq!(
            (w1 ^ w2) & !IMM7_MASK,
            0,
            "different imms must differ only in bits[11:5]"
        );
        if imm1 != imm2 {
            prop_assert_ne!(w1, w2, "different imm7 must change the word");
        }
        prop_assert_eq!(
            w1 ^ w0,
            (imm1 as u32) << 5,
            "encode(imm) XOR encode(0) must equal imm << 5"
        );
    }

    #[test]
    fn encode_hint_neg_extra(imm in 0i64..=127, extra in extra_operand()) {
        let asm = format!("hint #{imm}, {}", extra_asm(&extra));
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted extra {}",
            asm
        );
        let ops = [Operand::Imm(imm), extra];
        prop_assert!(
            encode_hint(&ops).is_err(),
            "extra operand must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_hint_neg_oob_imm(imm in oob_imm()) {
        prop_assume!(!(0..=127).contains(&imm));
        let asm = format!("hint #{imm}");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        prop_assert!(
            encode_hint(&[Operand::Imm(imm)]).is_err(),
            "imm {} outside 0..=127 must Err (llvm-mc rejects {})",
            imm,
            asm
        );
    }

    #[test]
    fn encode_hint_neg_empty(_n in 0u32..8) {
        prop_assert!(
            llvm_mc_word("hint").is_err(),
            "llvm-mc unexpectedly accepted omitted-operand hint"
        );
        prop_assert!(
            encode_hint(&[]).is_err(),
            "empty operands must Err (gas/llvm-mc reject omitted hint immediate)"
        );
    }

    #[test]
    fn encode_hint_neg_wrong_kind(op in wrong_kind()) {
        if let Some(asm) = asm_for_wrong(&op) {
            prop_assert!(
                llvm_mc_word(&asm).is_err(),
                "llvm-mc unexpectedly accepted {}",
                asm
            );
        }
        prop_assert!(
            encode_hint(&[op.clone()]).is_err(),
            "non-Imm first operand must Err, got {:?}",
            encode_hint(&[op])
        );
    }
}

#[test]
fn test_encode_hint_regression_extra_x0() {
    assert!(
        llvm_mc_word("hint #0, x0").is_err(),
        "llvm-mc must reject extra operand"
    );
    let ops = [Operand::Imm(0), Operand::Reg("x0".into())];
    assert!(
        encode_hint(&ops).is_err(),
        "hint #0, x0 must Err (gas/llvm-mc reject extra operands)"
    );
}

#[test]
fn test_encode_hint_regression_imm_neg1() {
    assert!(
        llvm_mc_word("hint #-1").is_err(),
        "llvm-mc must reject hint #-1"
    );
    assert!(
        encode_hint(&[Operand::Imm(-1)]).is_err(),
        "hint #-1 must Err (imm out of 0..=127)"
    );
}

#[test]
fn test_encode_hint_regression_imm_128() {
    assert!(
        llvm_mc_word("hint #128").is_err(),
        "llvm-mc must reject hint #128"
    );
    assert!(
        encode_hint(&[Operand::Imm(128)]).is_err(),
        "hint #128 must Err (imm out of 0..=127)"
    );
}
