// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md:226 NEON scalar lists add/sub (d-regs);
//   encoder/mod.rs:1-7 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:305-310 "add"/"sub" => encode_neon_scalar_three_same when dest is Dd;
//   neon.rs:1789 NEON scalar three-same: ADD/SUB Dd, Dn, Dm;
//   neon.rs:1790 Encode scalar NEON three-same: 01 U 11110 size 1 Rm opcode 1 Rn Rd;
//   ARM ARM Advanced SIMD scalar three-same ADD/SUB: size=11 (D only), opcode=10000, U=0 ADD / U=1 SUB.
// Stronger considered:
//   - State machine: rejected — encode_neon_scalar_three_same is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree scalar ADD/SUB decoder
//   - Differential vs encode_neon_three_same / encode_neon_add_sub: rejected — same-job gate
//     (vector Vd.T vs scalar Dd, Dn, Dm)
// Weaker available: algebraic.metamorphic (Rd/Rn/Rm/U fields), algebraic.invariant (word layout),
//   negative_error (arity / extra / wrong register class / non-register)
// Differential: candidate=encode_neon_scalar_three_same, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[Reg(Dd), Reg(Dn), Reg(Dm)] + (U=0|1, opcode=10000, size=11)
//     <-> `add|sub Dd, Dn, Dm`

use super::encode_neon_scalar_three_same;
use super::EncodeResult;
use crate::backend::arm::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;
const OPCODE_ADD_SUB: u32 = 0b10000;
const SIZE_D: u32 = 0b11;

fn cfg() -> ProptestConfig {
    ProptestConfig::with_cases(CASES)
}

fn mnemonic(is_sub: bool) -> &'static str {
    if is_sub {
        "sub"
    } else {
        "add"
    }
}

fn u_bit(is_sub: bool) -> u32 {
    if is_sub {
        1
    } else {
        0
    }
}

fn dreg(n: u32) -> Operand {
    Operand::Reg(format!("d{}", n))
}

fn ops_d(rd: u32, rn: u32, rm: u32) -> Vec<Operand> {
    vec![dreg(rd), dreg(rn), dreg(rm)]
}

fn asm_d(rd: u32, rn: u32, rm: u32, is_sub: bool) -> String {
    format!("{} d{rd}, d{rn}, d{rm}", mnemonic(is_sub))
}

fn sut_word(ops: &[Operand], is_sub: bool) -> Result<u32, String> {
    sut_word_fields(ops, u_bit(is_sub), OPCODE_ADD_SUB, SIZE_D)
}

fn sut_word_fields(ops: &[Operand], u: u32, opcode: u32, size: u32) -> Result<u32, String> {
    match encode_neon_scalar_three_same(ops, u, opcode, size)? {
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

fn reg_num() -> impl Strategy<Value = u32> {
    prop_oneof![Just(0u32), Just(1u32), Just(30u32), Just(31u32), 0u32..=31]
}

fn non_d_pfx() -> impl Strategy<Value = &'static str> {
    prop::sample::select(vec![
        "s", "h", "b", "x", "w", "q", "v", "sp", "xzr", "wsp", "wzr", "lr",
    ])
}

fn non_d_operand(pfx: &str, n: u32) -> (Operand, String) {
    match pfx {
        "sp" | "wsp" | "xzr" | "wzr" | "lr" => (Operand::Reg(pfx.to_string()), pfx.to_string()),
        _ => {
            let name = format!("{pfx}{n}");
            (Operand::Reg(name.clone()), name)
        }
    }
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_neon_scalar_three_same_kat_llvm_mc() {
    let kat: &[(&str, u32)] = &[
        ("add d0, d1, d2", 0x5ee28420),
        ("sub d0, d1, d2", 0x7ee28420),
        ("add d31, d31, d31", 0x5eff87ff),
        ("add d0, d0, d0", 0x5ee08400),
        ("add d15, d16, d17", 0x5ef1860f),
        ("sub d31, d0, d1", 0x7ee1841f),
        ("add D0, D1, D2", 0x5ee28420),
        ("ADD d0, d1, d2", 0x5ee28420),
    ];
    for &(asm, want) in kat {
        let mc = llvm_mc_word(asm).unwrap_or_else(|e| panic!("llvm-mc KAT {asm}: {e}"));
        assert_eq!(mc, want, "llvm-mc KAT mapping broken for {asm}");
    }
    let sut_kat: &[(u32, u32, u32, bool, u32)] = &[
        (0, 1, 2, false, 0x5ee28420),
        (0, 1, 2, true, 0x7ee28420),
        (31, 31, 31, false, 0x5eff87ff),
        (0, 0, 0, false, 0x5ee08400),
        (15, 16, 17, false, 0x5ef1860f),
        (31, 0, 1, true, 0x7ee1841f),
    ];
    for &(rd, rn, rm, is_sub, want) in sut_kat {
        let asm = asm_d(rd, rn, rm, is_sub);
        let sut = sut_word(&ops_d(rd, rn, rm), is_sub)
            .unwrap_or_else(|e| panic!("SUT KAT {asm}: {e}"));
        assert_eq!(sut, want, "SUT KAT mismatch for {asm}");
    }
}

proptest! {
    #![proptest_config(cfg())]

    // Oracle: differential — llvm-mc AArch64 assembler
    #[test]
    fn encode_neon_scalar_three_same_diff_llvm_mc(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        is_sub in any::<bool>(),
    ) {
        let asm = asm_d(rd, rn, rm, is_sub);
        let ops = ops_d(rd, rn, rm);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&ops, is_sub)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for {}", asm);
    }

    // Oracle: algebraic.metamorphic — Rd/Rn/Rm/U field isolation
    #[test]
    fn encode_neon_scalar_three_same_meta_rd_rn_rm_u(
        rd1 in reg_num(),
        rd2 in reg_num(),
        rn1 in reg_num(),
        rn2 in reg_num(),
        rm1 in reg_num(),
        rm2 in reg_num(),
    ) {
        let w1110 = sut_word(&ops_d(rd1, rn1, rm1), false)
            .unwrap_or_else(|e| panic!("SUT rejected rd1/rn1/rm1: {}", e));
        let w2110 = sut_word(&ops_d(rd2, rn1, rm1), false)
            .unwrap_or_else(|e| panic!("SUT rejected rd2: {}", e));
        let w1210 = sut_word(&ops_d(rd1, rn2, rm1), false)
            .unwrap_or_else(|e| panic!("SUT rejected rn2: {}", e));
        let w1120 = sut_word(&ops_d(rd1, rn1, rm2), false)
            .unwrap_or_else(|e| panic!("SUT rejected rm2: {}", e));
        let w1111 = sut_word(&ops_d(rd1, rn1, rm1), true)
            .unwrap_or_else(|e| panic!("SUT rejected sub: {}", e));
        prop_assert_eq!(
            (w1110 ^ w2110) & !0x1Fu32,
            0u32,
            "changing only Rd must differ only in bits[4:0]"
        );
        prop_assert_eq!(w2110 & 0x1F, rd2, "Rd field");
        prop_assert_eq!(
            (w1110 ^ w1210) & !(0x1Fu32 << 5),
            0u32,
            "changing only Rn must differ only in bits[9:5]"
        );
        prop_assert_eq!((w1210 >> 5) & 0x1F, rn2, "Rn field");
        prop_assert_eq!(
            (w1110 ^ w1120) & !(0x1Fu32 << 16),
            0u32,
            "changing only Rm must differ only in bits[20:16]"
        );
        prop_assert_eq!((w1120 >> 16) & 0x1F, rm2, "Rm field");
        prop_assert_eq!(
            (w1110 ^ w1111) & !(1u32 << 29),
            0u32,
            "add vs sub must differ only in U bit 29"
        );
        prop_assert_eq!((w1111 >> 29) & 1, 1u32, "U=1 for sub");
        prop_assert_eq!((w1110 >> 29) & 1, 0u32, "U=0 for add");
    }

    // Oracle: algebraic.invariant — ARM scalar three-same field layout
    #[test]
    fn encode_neon_scalar_three_same_invariant_arm_fields(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        u in 0u32..=1u32,
        opcode in 0u32..=31u32,
        size in 0u32..=3u32,
    ) {
        let w = sut_word_fields(&ops_d(rd, rn, rm), u, opcode, size)
            .unwrap_or_else(|e| panic!("SUT rejected: {}", e));
        prop_assert_eq!((w >> 30) & 3, 0b01u32, "bits[31:30]=01");
        prop_assert_eq!((w >> 29) & 1, u, "U");
        prop_assert_eq!((w >> 24) & 0x1F, 0b11110u32, "bits[28:24]=11110");
        prop_assert_eq!((w >> 22) & 3, size, "size");
        prop_assert_eq!((w >> 21) & 1, 1u32, "bit21=1");
        prop_assert_eq!((w >> 16) & 0x1F, rm, "Rm");
        prop_assert_eq!((w >> 11) & 0x1F, opcode, "opcode");
        prop_assert_eq!((w >> 10) & 1, 1u32, "bit10=1");
        prop_assert_eq!((w >> 5) & 0x1F, rn, "Rn");
        prop_assert_eq!(w & 0x1F, rd, "Rd");
    }

    // Oracle: negative_error — arity 0..=2
    #[test]
    fn encode_neon_scalar_three_same_neg_arity(
        n in 0usize..=2,
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        is_sub in any::<bool>(),
    ) {
        let all = ops_d(rd, rn, rm);
        let arity_ops: Vec<Operand> = all.iter().take(n).cloned().collect();
        let mn = mnemonic(is_sub);
        let arity_asm = match n {
            0 => mn.to_string(),
            1 => format!("{mn} d{rd}"),
            _ => format!("{mn} d{rd}, d{rn}"),
        };
        prop_assert!(
            llvm_mc_word(&arity_asm).is_err(),
            "llvm-mc unexpectedly accepted arity-{} {}",
            n,
            arity_asm
        );
        prop_assert!(
            encode_neon_scalar_three_same(&arity_ops, u_bit(is_sub), OPCODE_ADD_SUB, SIZE_D).is_err(),
            "arity {} must Err (llvm-mc rejects {})",
            n,
            arity_asm
        );
    }

    // Oracle: negative_error — fourth operand
    #[test]
    fn encode_neon_scalar_three_same_neg_extra_operand(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        extra in reg_num(),
        is_sub in any::<bool>(),
    ) {
        let asm = format!("{}, d{}", asm_d(rd, rn, rm, is_sub), extra);
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted 4-operand {}",
            asm
        );
        let mut ops = ops_d(rd, rn, rm);
        ops.push(dreg(extra));
        prop_assert!(
            encode_neon_scalar_three_same(&ops, u_bit(is_sub), OPCODE_ADD_SUB, SIZE_D).is_err(),
            "4 operands must Err (llvm-mc rejects {})",
            asm
        );
    }

    // Oracle: negative_error — non-D source while dest is Dd (caller-reachable)
    #[test]
    fn encode_neon_scalar_three_same_neg_wrong_reg_class(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        is_sub in any::<bool>(),
        slot in 1usize..=2,
        pfx in non_d_pfx(),
    ) {
        let (bad, bad_name) = non_d_operand(pfx, if slot == 1 { rn } else { rm });
        let mut ops = ops_d(rd, rn, rm);
        ops[slot] = bad;
        let mn = mnemonic(is_sub);
        let asm = if slot == 1 {
            format!("{mn} d{rd}, {bad_name}, d{rm}")
        } else {
            format!("{mn} d{rd}, d{rn}, {bad_name}")
        };
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        prop_assert!(
            encode_neon_scalar_three_same(&ops, u_bit(is_sub), OPCODE_ADD_SUB, SIZE_D).is_err(),
            "non-D source slot={} pfx={} must Err (llvm-mc rejects {})",
            slot,
            pfx,
            asm
        );
    }

    // Oracle: negative_error — Imm/Mem/Label in any slot
    #[test]
    fn encode_neon_scalar_three_same_neg_nonreg(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        is_sub in any::<bool>(),
        kind in 0u8..=2,
        slot in 0usize..=2,
    ) {
        let mn = mnemonic(is_sub);
        let bad = match kind {
            0 => Operand::Imm(0),
            1 => Operand::Mem {
                base: format!("x{}", rd),
                offset: 0,
            },
            _ => Operand::Label("L0".into()),
        };
        let mut ops = ops_d(rd, rn, rm);
        ops[slot] = bad;
        let asm = match (kind, slot) {
            (0, 0) => format!("{mn} #0, d{rn}, d{rm}"),
            (1, 0) => format!("{mn} [x{rd}], d{rn}, d{rm}"),
            (2, 0) => format!("{mn} L0, d{rn}, d{rm}"),
            _ => format!("{mn} nonreg slot={slot} kind={kind}"),
        };
        if slot == 0 {
            prop_assert!(
                llvm_mc_word(&asm).is_err(),
                "llvm-mc unexpectedly accepted {}",
                asm
            );
        }
        prop_assert!(
            encode_neon_scalar_three_same(&ops, u_bit(is_sub), OPCODE_ADD_SUB, SIZE_D).is_err(),
            "non-register operand slot={} kind={} must Err",
            slot,
            kind
        );
    }

    // Oracle: differential — uppercase D / mnemonic alt-spellings vs llvm-mc
    #[test]
    fn encode_neon_scalar_three_same_diff_alt_spellings(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        is_sub in any::<bool>(),
    ) {
        let mn = if is_sub { "SUB" } else { "ADD" };
        let asm = format!("{mn} D{rd}, D{rn}, D{rm}");
        let ops = vec![
            Operand::Reg(format!("D{}", rd)),
            Operand::Reg(format!("D{}", rn)),
            Operand::Reg(format!("D{}", rm)),
        ];
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected alt-spelling {}: {}", asm, e));
        let sut = sut_word(&ops, is_sub)
            .unwrap_or_else(|e| panic!("SUT rejected alt-spelling {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for alt-spelling {}", asm);
    }
}

/// Deterministic regression: extra operand ignored (from neg_extra_operand).
#[test]
fn test_encode_neon_scalar_three_same_regression_extra_operand() {
    let mut ops = ops_d(0, 0, 0);
    ops.push(dreg(0));
    assert!(
        encode_neon_scalar_three_same(&ops, 0, OPCODE_ADD_SUB, SIZE_D).is_err(),
        "add d0, d0, d0, d0 must Err (gas/llvm-mc reject a fourth operand)"
    );
}

/// Deterministic regression: non-D source encoded (from neg_wrong_reg_class).
#[test]
fn test_encode_neon_scalar_three_same_regression_wrong_reg_class() {
    let ops = vec![dreg(0), Operand::Reg("s0".into()), dreg(0)];
    assert!(
        encode_neon_scalar_three_same(&ops, 0, OPCODE_ADD_SUB, SIZE_D).is_err(),
        "add d0, s0, d0 must Err (ARM/gas/llvm-mc require Dd, Dn, Dm)"
    );
}
