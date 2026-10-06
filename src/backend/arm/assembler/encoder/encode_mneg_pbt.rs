// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md:214 Data Processing table lists mneg;
//   encoder/mod.rs:3 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:409 "mneg" => encode_mneg(operands);
//   data_processing.rs:676 "Encode MNEG Xd, Xn, Xm -> MSUB Xd, Xn, Xm, XZR";
//   data_processing.rs:682 "MSUB with Ra=XZR: sf 00 11011 000 Rm 1 11111 Rn Rd";
//   ARM ARM Data-processing (3 source) MNEG is the alias of MSUB Rd, Rn, Rm, ZR:
//     sf 00 11011 000 Rm o0=1 Ra=11111 Rn Rd;
//     Rd/Rn/Rm are same-width GPRs (XZR/WZR not SP/WSP).
// Stronger considered:
//   - State machine: rejected — encode_mneg is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree MNEG decoder
//   - encode_msub as differential sibling: rejected — same-job gate (4-operand MSUB vs
//     3-operand alias; shared get_reg / same TU); alias equality is algebraic.metamorphic
//   - encode_mul as differential sibling: rejected — same-job gate (o0=0 MADD-ZR)
// Weaker available: algebraic.metamorphic (MSUB Ra=ZR alias, sf bit X vs W, field isolation),
//   algebraic.invariant (ARM field layout), negative_error (arity / extra / SP / FP / mixed width)
// Differential: candidate=encode_mneg, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[Reg(Rd), Reg(Rn), Reg(Rm)] <-> `mneg Rd, Rn, Rm`

use super::encode_mneg;
use super::encode_msub;
use super::EncodeResult;
use crate::backend::arm::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;

fn cfg() -> ProptestConfig {
    ProptestConfig::with_cases(CASES)
}

fn gpr(is_64: bool, n: u32) -> String {
    if n == 31 {
        if is_64 {
            "xzr".into()
        } else {
            "wzr".into()
        }
    } else {
        format!("{}{}", if is_64 { "x" } else { "w" }, n)
    }
}

fn sut_word(ops: &[Operand]) -> Result<u32, String> {
    match encode_mneg(ops)? {
        EncodeResult::Word(w) => Ok(w),
        other => Err(format!("expected Word, got {:?}", other)),
    }
}

fn msub_word(ops: &[Operand]) -> Result<u32, String> {
    match encode_msub(ops)? {
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
    let out = child.wait_with_output().map_err(|e| format!("wait llvm-mc: {e}"))?;
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
        Just(Operand::Imm(0)),
        Just(Operand::Imm(-1)),
        Just(Operand::Shift {
            kind: "lsl".into(),
            amount: 0,
        }),
        Just(Operand::RegArrangement {
            reg: "v0".into(),
            arrangement: "8h".into(),
        }),
    ]
}

fn non_reg_operand() -> impl Strategy<Value = Operand> {
    prop_oneof![
        any::<i64>().prop_map(Operand::Imm),
        Just(Operand::Shift {
            kind: "lsl".into(),
            amount: 0,
        }),
        Just(Operand::Mem {
            base: "x0".into(),
            offset: 0,
        }),
        Just(Operand::Label("L0".into())),
        Just(Operand::Symbol("foo".into())),
        Just(Operand::Cond("eq".into())),
        Just(Operand::RegArrangement {
            reg: "v0".into(),
            arrangement: "8b".into(),
        }),
    ]
}

fn invalid_name() -> impl Strategy<Value = String> {
    prop::sample::select(vec![
        "foo".into(),
        "x32".into(),
        "w32".into(),
        "x".into(),
        "r0".into(),
        "".into(),
        "x-1".into(),
        "x99".into(),
        "w".into(),
    ])
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_mneg_kat_llvm_mc_x0_x1_x2() {
    let want = 0x9b02fc20u32;
    let mc = llvm_mc_word("mneg x0, x1, x2").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [
        Operand::Reg("x0".into()),
        Operand::Reg("x1".into()),
        Operand::Reg("x2".into()),
    ];
    let sut = sut_word(&ops).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_mneg_kat_llvm_mc_w0_w1_w2() {
    let want = 0x1b02fc20u32;
    let mc = llvm_mc_word("mneg w0, w1, w2").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [
        Operand::Reg("w0".into()),
        Operand::Reg("w1".into()),
        Operand::Reg("w2".into()),
    ];
    let sut = sut_word(&ops).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_mneg_kat_llvm_mc_xzr_xzr_xzr() {
    let want = 0x9b1fffffu32;
    let mc = llvm_mc_word("mneg xzr, xzr, xzr").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [
        Operand::Reg("xzr".into()),
        Operand::Reg("xzr".into()),
        Operand::Reg("xzr".into()),
    ];
    let sut = sut_word(&ops).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_mneg_kat_llvm_mc_lr_x1_x2() {
    let want = 0x9b02fc3eu32;
    let mc = llvm_mc_word("mneg lr, x1, x2").expect("llvm-mc LR KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [
        Operand::Reg("lr".into()),
        Operand::Reg("x1".into()),
        Operand::Reg("x2".into()),
    ];
    let sut = sut_word(&ops).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_mneg_kat_llvm_mc_alias_msub_zr() {
    let want = 0x9b02fc20u32;
    let mc_mneg = llvm_mc_word("mneg x0, x1, x2").expect("llvm-mc MNEG KAT");
    let mc_msub = llvm_mc_word("msub x0, x1, x2, xzr").expect("llvm-mc MSUB ZR KAT");
    assert_eq!(mc_mneg, want, "llvm-mc MNEG KAT mapping broken");
    assert_eq!(mc_msub, want, "llvm-mc MSUB ZR KAT mapping broken");
    let ops = [
        Operand::Reg("x0".into()),
        Operand::Reg("x1".into()),
        Operand::Reg("x2".into()),
    ];
    let sut = sut_word(&ops).expect("SUT KAT");
    assert_eq!(sut, want);
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_mneg_diff_gpr(
        rd in 0u32..=31,
        rn in 0u32..=31,
        rm in 0u32..=31,
        is_64 in any::<bool>(),
        use_lr in any::<bool>(),
    ) {
        let dest = if use_lr && is_64 && rd == 30 {
            "lr".to_string()
        } else {
            gpr(is_64, rd)
        };
        let src_n = gpr(is_64, rn);
        let src_m = gpr(is_64, rm);
        let asm = format!("mneg {}, {}, {}", dest, src_n, src_m);
        let ops = [
            Operand::Reg(dest),
            Operand::Reg(src_n),
            Operand::Reg(src_m),
        ];
        let sut = sut_word(&ops)
            .unwrap_or_else(|e| panic!("SUT rejected valid MNEG {}: {}", asm, e));
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid MNEG {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
    }

    #[test]
    fn encode_mneg_alias_msub_zr(
        rd in 0u32..=31,
        rn in 0u32..=31,
        rm in 0u32..=31,
        is_64 in any::<bool>(),
    ) {
        let dest = gpr(is_64, rd);
        let src_n = gpr(is_64, rn);
        let src_m = gpr(is_64, rm);
        let zr = gpr(is_64, 31);
        let mneg_asm = format!("mneg {}, {}, {}", dest, src_n, src_m);
        let msub_asm = format!("msub {}, {}, {}, {}", dest, src_n, src_m, zr);
        let mneg_ops = [
            Operand::Reg(dest.clone()),
            Operand::Reg(src_n.clone()),
            Operand::Reg(src_m.clone()),
        ];
        let msub_ops = [
            Operand::Reg(dest),
            Operand::Reg(src_n),
            Operand::Reg(src_m),
            Operand::Reg(zr),
        ];
        let sut = sut_word(&mneg_ops)
            .unwrap_or_else(|e| panic!("SUT rejected valid MNEG {}: {}", mneg_asm, e));
        let alias = msub_word(&msub_ops)
            .unwrap_or_else(|e| panic!("SUT rejected valid MSUB ZR {}: {}", msub_asm, e));
        prop_assert_eq!(sut, alias, "MNEG must equal MSUB with Ra=ZR");
        let mc_mneg = llvm_mc_word(&mneg_asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid MNEG {}: {}", mneg_asm, e));
        let mc_msub = llvm_mc_word(&msub_asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid MSUB ZR {}: {}", msub_asm, e));
        prop_assert_eq!(mc_mneg, mc_msub, "llvm-mc MNEG vs MSUB ZR mismatch");
        prop_assert_eq!(sut, mc_mneg, "SUT vs llvm-mc MNEG mismatch for {}", mneg_asm);
    }

    #[test]
    fn encode_mneg_metamorphic_sf_bit(
        rd in 0u32..=31,
        rn in 0u32..=31,
        rm in 0u32..=31,
    ) {
        let x_ops = [
            Operand::Reg(gpr(true, rd)),
            Operand::Reg(gpr(true, rn)),
            Operand::Reg(gpr(true, rm)),
        ];
        let w_ops = [
            Operand::Reg(gpr(false, rd)),
            Operand::Reg(gpr(false, rn)),
            Operand::Reg(gpr(false, rm)),
        ];
        let xw = sut_word(&x_ops)
            .unwrap_or_else(|e| panic!("64-bit MNEG rejected: {}", e));
        let ww = sut_word(&w_ops)
            .unwrap_or_else(|e| panic!("32-bit MNEG rejected: {}", e));
        prop_assert_eq!(
            xw ^ ww,
            1u32 << 31,
            "X vs W MNEG must differ only by sf bit 31 (x={:#010x} w={:#010x})",
            xw,
            ww
        );
    }

    #[test]
    fn encode_mneg_invariant_arm_fields(
        rd in 0u32..=31,
        rn in 0u32..=31,
        rm in 0u32..=31,
        is_64 in any::<bool>(),
    ) {
        let ops = [
            Operand::Reg(gpr(is_64, rd)),
            Operand::Reg(gpr(is_64, rn)),
            Operand::Reg(gpr(is_64, rm)),
        ];
        let w = sut_word(&ops)
            .unwrap_or_else(|e| panic!("MNEG rejected: {}", e));
        let sf = if is_64 { 1u32 } else { 0 };
        prop_assert_eq!(w & 0x1F, rd, "Rd field");
        prop_assert_eq!((w >> 5) & 0x1F, rn, "Rn field");
        prop_assert_eq!((w >> 10) & 0x1F, 31, "Ra field must be XZR/WZR (31)");
        prop_assert_eq!((w >> 15) & 1, 1, "o0 bit 15 must be 1 (MSUB/MNEG not MADD)");
        prop_assert_eq!((w >> 16) & 0x1F, rm, "Rm field");
        prop_assert_eq!((w >> 21) & 0x3FF, 0b0011011000u32, "bits 30:21 must be 0011011000");
        prop_assert_eq!((w >> 31) & 1, sf, "sf bit");
    }

    #[test]
    fn encode_mneg_neg_too_few(
        n in 0usize..=2,
        is_64 in any::<bool>(),
        r0 in 0u32..=31,
        r1 in 0u32..=31,
    ) {
        let all = [
            Operand::Reg(gpr(is_64, r0)),
            Operand::Reg(gpr(is_64, r1)),
        ];
        let ops = &all[..n.min(2)];
        prop_assert!(
            encode_mneg(ops).is_err(),
            "fewer than 3 operands must Err, n={}",
            n
        );
    }

    #[test]
    fn encode_mneg_neg_extra_operand(
        rd in 0u32..=31,
        rn in 0u32..=31,
        rm in 0u32..=31,
        is_64 in any::<bool>(),
        extra in extra_operand(),
    ) {
        let ops = vec![
            Operand::Reg(gpr(is_64, rd)),
            Operand::Reg(gpr(is_64, rn)),
            Operand::Reg(gpr(is_64, rm)),
            extra,
        ];
        prop_assert!(
            encode_mneg(&ops).is_err(),
            "MNEG has no 4th operand; extra operand must Err (llvm-mc rejects it)"
        );
    }

    #[test]
    fn encode_mneg_neg_mixed_width(
        rd in 0u32..=30,
        rn in 0u32..=30,
        rm in 0u32..=30,
        rd64 in any::<bool>(),
        rn64 in any::<bool>(),
        rm64 in any::<bool>(),
    ) {
        prop_assume!(!(rd64 == rn64 && rn64 == rm64));
        let ops = [
            Operand::Reg(gpr(rd64, rd)),
            Operand::Reg(gpr(rn64, rn)),
            Operand::Reg(gpr(rm64, rm)),
        ];
        prop_assert!(
            encode_mneg(&ops).is_err(),
            "mixed-width MNEG registers must Err (rd64={} rn64={} rm64={})",
            rd64,
            rn64,
            rm64
        );
    }

    #[test]
    fn encode_mneg_neg_sp(
        which in 0u32..=2,
        is_64 in any::<bool>(),
        a in 0u32..=30,
        b in 0u32..=30,
    ) {
        let sp = if is_64 { "sp" } else { "wsp" };
        let mut names = [
            gpr(is_64, a),
            gpr(is_64, b),
            sp.to_string(),
        ];
        names.swap(2, which as usize);
        let ops = [
            Operand::Reg(names[0].clone()),
            Operand::Reg(names[1].clone()),
            Operand::Reg(names[2].clone()),
        ];
        prop_assert!(
            encode_mneg(&ops).is_err(),
            "SP/WSP is not a valid MNEG operand (which={} names={:?})",
            which,
            names
        );
    }

    #[test]
    fn encode_mneg_neg_fp(
        which in 0u32..=2,
        prefix in prop::sample::select(vec!["d", "s", "q", "v", "h", "b"]),
        n in prop_oneof![Just(0u32), Just(31u32), 0u32..=31],
        is_64 in any::<bool>(),
    ) {
        let fp = format!("{}{}", prefix, n);
        let mut ops = vec![
            Operand::Reg(gpr(is_64, 0)),
            Operand::Reg(gpr(is_64, 1)),
            Operand::Reg(gpr(is_64, 2)),
        ];
        ops[which as usize] = Operand::Reg(fp.clone());
        prop_assert!(
            encode_mneg(&ops).is_err(),
            "FP/SIMD register {} is not a valid MNEG operand (which={})",
            fp,
            which
        );
    }

    #[test]
    fn encode_mneg_neg_nonreg(
        which in 0u32..=2,
        bad in non_reg_operand(),
        is_64 in any::<bool>(),
    ) {
        let mut ops = vec![
            Operand::Reg(gpr(is_64, 0)),
            Operand::Reg(gpr(is_64, 1)),
            Operand::Reg(gpr(is_64, 2)),
        ];
        ops[which as usize] = bad;
        prop_assert!(
            encode_mneg(&ops).is_err(),
            "non-register operand at slot {} must Err",
            which
        );
    }

    #[test]
    fn encode_mneg_neg_invalid_name(
        which in 0u32..=2,
        name in invalid_name(),
        is_64 in any::<bool>(),
    ) {
        let mut ops = vec![
            Operand::Reg(gpr(is_64, 0)),
            Operand::Reg(gpr(is_64, 1)),
            Operand::Reg(gpr(is_64, 2)),
        ];
        ops[which as usize] = Operand::Reg(name.clone());
        prop_assert!(
            encode_mneg(&ops).is_err(),
            "invalid register name {:?} at slot {} must Err",
            name,
            which
        );
    }

    #[test]
    fn encode_mneg_diff_alt_spellings(
        rd in 0u32..=31,
        rn in 0u32..=31,
        rm in 0u32..=31,
        is_64 in any::<bool>(),
        dest_spell in 0u32..=4,
        src_spell in 0u32..=2,
    ) {
        let dest = match dest_spell {
            0 if rd == 31 => if is_64 { "x31".to_string() } else { "w31".to_string() },
            1 if rd == 31 => if is_64 { "XZR".to_string() } else { "WZR".to_string() },
            2 if is_64 && rd == 30 => "LR".to_string(),
            3 => gpr(is_64, rd).to_uppercase(),
            _ => gpr(is_64, rd),
        };
        let src_n = match src_spell {
            0 if rn == 31 => if is_64 { "x31".to_string() } else { "w31".to_string() },
            1 => gpr(is_64, rn).to_uppercase(),
            _ => gpr(is_64, rn),
        };
        let src_m = match src_spell {
            0 if rm == 31 => if is_64 { "x31".to_string() } else { "w31".to_string() },
            1 => gpr(is_64, rm).to_uppercase(),
            _ => gpr(is_64, rm),
        };
        let asm = format!("mneg {}, {}, {}", dest, src_n, src_m);
        let ops = [
            Operand::Reg(dest),
            Operand::Reg(src_n),
            Operand::Reg(src_m),
        ];
        let mc = llvm_mc_word(&asm).expect("llvm-mc");
        let sut = sut_word(&ops).expect("SUT");
        prop_assert_eq!(sut, mc, "MNEG alt-spelling mismatch for {}", asm);
    }

    #[test]
    fn encode_mneg_meta_rd_rn_rm(
        rd in 0u32..=30,
        rn in 0u32..=30,
        rm in 0u32..=30,
        is_64 in any::<bool>(),
    ) {
        let base_ops = [
            Operand::Reg(gpr(is_64, rd)),
            Operand::Reg(gpr(is_64, rn)),
            Operand::Reg(gpr(is_64, rm)),
        ];
        let base = sut_word(&base_ops).expect("base");
        let rd1 = sut_word(&[
            Operand::Reg(gpr(is_64, rd + 1)),
            Operand::Reg(gpr(is_64, rn)),
            Operand::Reg(gpr(is_64, rm)),
        ]).expect("rd+1");
        let rn1 = sut_word(&[
            Operand::Reg(gpr(is_64, rd)),
            Operand::Reg(gpr(is_64, rn + 1)),
            Operand::Reg(gpr(is_64, rm)),
        ]).expect("rn+1");
        let rm1 = sut_word(&[
            Operand::Reg(gpr(is_64, rd)),
            Operand::Reg(gpr(is_64, rn)),
            Operand::Reg(gpr(is_64, rm + 1)),
        ]).expect("rm+1");
        prop_assert_eq!((rd1 ^ base) & !0x1fu32, 0, "Rd+1 only in bits[4:0]");
        prop_assert_eq!(rd1 & 0x1f, rd + 1);
        prop_assert_eq!((rn1 ^ base) & !(0x1fu32 << 5), 0, "Rn+1 only in bits[9:5]");
        prop_assert_eq!((rn1 >> 5) & 0x1f, rn + 1);
        prop_assert_eq!((rm1 ^ base) & !(0x1fu32 << 16), 0, "Rm+1 only in bits[20:16]");
        prop_assert_eq!((rm1 >> 16) & 0x1f, rm + 1);
    }
}

#[test]
fn test_encode_mneg_regression_extra_operand() {
    let ops = [
        Operand::Reg("w0".into()),
        Operand::Reg("w0".into()),
        Operand::Reg("w0".into()),
        Operand::Reg("x0".into()),
    ];
    assert!(
        encode_mneg(&ops).is_err(),
        "MNEG w0, w0, w0, x0 must Err; extra operand is invalid (llvm-mc rejects it)"
    );
}

#[test]
fn test_encode_mneg_regression_mixed_width() {
    let ops = [
        Operand::Reg("w0".into()),
        Operand::Reg("w0".into()),
        Operand::Reg("x0".into()),
    ];
    assert!(
        encode_mneg(&ops).is_err(),
        "MNEG w0, w0, x0 must Err; Rd, Rn, Rm must be the same width (llvm-mc rejects it)"
    );
}

#[test]
fn test_encode_mneg_regression_sp() {
    let ops = [
        Operand::Reg("wsp".into()),
        Operand::Reg("w0".into()),
        Operand::Reg("w0".into()),
    ];
    assert!(
        encode_mneg(&ops).is_err(),
        "MNEG wsp, w0, w0 must Err; register 31 is WZR not WSP (llvm-mc rejects it)"
    );
}

#[test]
fn test_encode_mneg_regression_fp() {
    let ops = [
        Operand::Reg("d0".into()),
        Operand::Reg("x1".into()),
        Operand::Reg("x2".into()),
    ];
    assert!(
        encode_mneg(&ops).is_err(),
        "MNEG d0, x1, x2 must Err; FP/SIMD registers are not MNEG operands (llvm-mc rejects it)"
    );
}
