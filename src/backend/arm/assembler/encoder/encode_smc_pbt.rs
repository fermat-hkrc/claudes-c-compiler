// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   encoder/mod.rs:3 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:985 "smc" => encode_smc(operands);
//   ARM ARM SMC: 1101 0100 000 imm16 00011 = 0xD4000003 | (imm16 << 5);
//   imm16 ∈ 0..=65535;
//   llvm-mc -triple=aarch64 -show-encoding (LLVM 15.0.6);
//   gas: "immediate value out of range 0 to 65535", "missing immediate expression",
//        "unexpected characters following instruction", "immediate operand required".
// Stronger considered:
//   - State machine: rejected — encode_smc is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree SMC decoder
//   - Differential vs encode_svc/encode_hvc/encode_brk: rejected — same-job gate
//     (SVC op2=001, HVC op2=010, SMC op2=011, BRK different group)
// Weaker available: algebraic.metamorphic (imm16 field isolation),
//   algebraic.invariant (ARM SMC layout),
//   negative_error (extra / oob imm / empty / wrong kind)
// Differential: candidate=encode_smc, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[Imm(n)] <-> `smc #n`

use super::encode_smc;
use super::EncodeResult;
use crate::backend::arm::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;

/// ARM ARM SMC with imm16=0: 1101 0100 000 0000000000000000 00011
const SMC_TEMPLATE: u32 = 0xd4000003;
const SMC_HI: u32 = 0b11010100000; // bits[31:21]
const IMM16_MASK: u32 = 0x1fffe0; // bits[20:5]

fn cfg() -> ProptestConfig {
    ProptestConfig::with_cases(CASES)
}

/// ARM ARM SMC word for a valid imm16. Independent of the SUT mask.
fn arm_smc_word(imm: u16) -> u32 {
    SMC_TEMPLATE | ((imm as u32) << 5)
}

fn sut_word(ops: &[Operand]) -> Result<u32, String> {
    match encode_smc(ops)? {
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
        Just(65536i64),
        Just(65537i64),
        Just(-2i64),
        Just(-65536i64),
        Just(-65535i64),
        Just(i64::MIN),
        Just(i64::MAX),
        Just(0x10000i64),
        Just(0x1ffffi64),
        Just(0x10001i64),
        Just(1i64 << 32),
        Just((1i64 << 32) + 1),
        Just((1i64 << 16) + 42),
        (-4096i64..=-2),
        (65536i64..=70000),
    ]
}

fn wrong_kind() -> impl Strategy<Value = Operand> {
    prop_oneof![
        (0u32..=31).prop_map(|n| Operand::Reg(format!("x{n}"))),
        (0u32..=31).prop_map(|n| Operand::Reg(format!("w{n}"))),
        Just(Operand::Reg("sp".into())),
        Just(Operand::Reg("xzr".into())),
        Just(Operand::Reg("d0".into())),
        Just(Operand::Symbol("smc".into())),
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
        Operand::Reg(r) => Some(format!("smc {r}")),
        Operand::Symbol(s) => Some(format!("smc {s}")),
        Operand::Barrier(b) => Some(format!("smc {b}")),
        Operand::Cond(c) => Some(format!("smc {c}")),
        Operand::Label(l) => Some(format!("smc {l}")),
        Operand::Mem { base, offset } => Some(format!("smc [{base}, #{offset}]")),
        Operand::Shift { kind, amount } => Some(format!("smc {kind} #{amount}")),
        Operand::Extend { kind, amount } => Some(format!("smc {kind} #{amount}")),
        _ => None,
    }
}

#[test]
fn encode_smc_kat_llvm_mc_imm0() {
    let mc = llvm_mc_word("smc #0").expect("llvm-mc smc #0");
    assert_eq!(mc, 0xd4000003, "llvm-mc KAT mapping broken for smc #0");
    let sut = sut_word(&[Operand::Imm(0)]).expect("SUT KAT imm0");
    assert_eq!(sut, mc);
}

#[test]
fn encode_smc_kat_llvm_mc_imm1() {
    let mc = llvm_mc_word("smc #1").expect("llvm-mc smc #1");
    assert_eq!(mc, 0xd4000023, "llvm-mc KAT mapping broken for smc #1");
    let sut = sut_word(&[Operand::Imm(1)]).expect("SUT KAT imm1");
    assert_eq!(sut, mc);
}

#[test]
fn encode_smc_kat_llvm_mc_imm65535() {
    let mc = llvm_mc_word("smc #65535").expect("llvm-mc smc #65535");
    assert_eq!(mc, 0xd41fffe3, "llvm-mc KAT mapping broken for smc #65535");
    let sut = sut_word(&[Operand::Imm(65535)]).expect("SUT KAT imm65535");
    assert_eq!(sut, mc);
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_smc_diff_imm(imm in 0i64..=65535) {
        let asm = format!("smc #{imm}");
        let mc = llvm_mc_word(&asm).expect(&asm);
        let expect = arm_smc_word(imm as u16);
        prop_assert_eq!(mc, expect, "llvm-mc vs ARM ARM for {}", asm);
        let sut = sut_word(&[Operand::Imm(imm)]).expect(&asm);
        prop_assert_eq!(sut, mc, "SUT vs llvm-mc for {}", asm);
    }

    #[test]
    fn encode_smc_inv_arm_layout(imm in 0i64..=65535) {
        let w = sut_word(&[Operand::Imm(imm)]).expect("valid imm16 must encode");
        let expect = arm_smc_word(imm as u16);
        prop_assert_eq!(w, expect, "ARM SMC word for #{}", imm);
        prop_assert_eq!(w >> 21, SMC_HI, "bits[31:21] SMC group");
        prop_assert_eq!((w >> 5) & 0xffff, imm as u32, "imm16 field bits[20:5]");
        prop_assert_eq!(w & 0x1f, 0b00011, "op2/Rt bits[4:0]=00011");
    }

    #[test]
    fn encode_smc_meta_imm_isolation(imm1 in 0i64..=65535, imm2 in 0i64..=65535) {
        let w1 = sut_word(&[Operand::Imm(imm1)]).expect("imm1");
        let w2 = sut_word(&[Operand::Imm(imm2)]).expect("imm2");
        let w0 = sut_word(&[Operand::Imm(0)]).expect("imm0");
        prop_assert_eq!(
            (w1 ^ w2) & !IMM16_MASK,
            0,
            "different imms must differ only in bits[20:5]"
        );
        if imm1 != imm2 {
            prop_assert_ne!(w1, w2, "different imm16 must change the word");
        }
        prop_assert_eq!(
            w1 ^ w0,
            (imm1 as u32) << 5,
            "encode(imm) XOR encode(0) must equal imm << 5"
        );
    }

    #[test]
    fn encode_smc_neg_extra(imm in 0i64..=65535, extra in extra_operand()) {
        let asm = format!("smc #{imm}, {}", extra_asm(&extra));
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted extra {}",
            asm
        );
        let ops = [Operand::Imm(imm), extra];
        prop_assert!(
            encode_smc(&ops).is_err(),
            "extra operand must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_smc_neg_oob_imm(imm in oob_imm()) {
        prop_assume!(!(0..=65535).contains(&imm));
        let asm = format!("smc #{imm}");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        prop_assert!(
            encode_smc(&[Operand::Imm(imm)]).is_err(),
            "imm {} outside 0..=65535 must Err (llvm-mc rejects {})",
            imm,
            asm
        );
    }

    #[test]
    fn encode_smc_neg_empty(_n in 0u32..8) {
        prop_assert!(
            llvm_mc_word("smc").is_err(),
            "llvm-mc unexpectedly accepted omitted-operand smc"
        );
        prop_assert!(
            encode_smc(&[]).is_err(),
            "empty operands must Err (gas/llvm-mc reject omitted smc immediate)"
        );
    }

    #[test]
    fn encode_smc_neg_wrong_kind(op in wrong_kind()) {
        if let Some(asm) = asm_for_wrong(&op) {
            prop_assert!(
                llvm_mc_word(&asm).is_err(),
                "llvm-mc unexpectedly accepted {}",
                asm
            );
        }
        prop_assert!(
            encode_smc(&[op.clone()]).is_err(),
            "non-Imm first operand must Err, got {:?}",
            encode_smc(&[op])
        );
    }
}

#[test]
fn test_encode_smc_regression_extra_x0() {
    assert!(
        llvm_mc_word("smc #0, x0").is_err(),
        "llvm-mc must reject extra operand"
    );
    let ops = [Operand::Imm(0), Operand::Reg("x0".into())];
    assert!(
        encode_smc(&ops).is_err(),
        "smc #0, x0 must Err (gas/llvm-mc reject extra operands)"
    );
}

#[test]
fn test_encode_smc_regression_imm_neg1() {
    assert!(
        llvm_mc_word("smc #-1").is_err(),
        "llvm-mc must reject smc #-1"
    );
    assert!(
        encode_smc(&[Operand::Imm(-1)]).is_err(),
        "smc #-1 must Err (imm out of 0..=65535)"
    );
}

#[test]
fn test_encode_smc_regression_imm_65536() {
    assert!(
        llvm_mc_word("smc #65536").is_err(),
        "llvm-mc must reject smc #65536"
    );
    assert!(
        encode_smc(&[Operand::Imm(65536)]).is_err(),
        "smc #65536 must Err (imm out of 0..=65535)"
    );
}
