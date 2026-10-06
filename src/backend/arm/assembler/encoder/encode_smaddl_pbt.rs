// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md:214 Data Processing table lists smaddl;
//   encoder/mod.rs:3 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:405 "smaddl" => encode_smaddl(operands);
//   data_processing.rs:652 "Encode SMADDL Xd, Wn, Wm, Xa (signed multiply-add long)";
//   data_processing.rs:658 "SMADDL: 1 00 11011 001 Rm 0 Ra Rn Rd";
//   ARM ARM Data-processing (3 source) SMADDL: sf=1 U=0 11011 001 Rm o0=0 Ra Rn Rd;
//     Rd/Ra are Xd/Xa (XZR not SP); Rn/Rm are Wn/Wm (WZR not WSP);
//     SMULL Xd, Wn, Wm is the alias of SMADDL Xd, Wn, Wm, XZR.
// Stronger considered:
//   - State machine: rejected — encode_smaddl is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree SMADDL decoder
//   - encode_umaddl as differential sibling: rejected — same-job gate (U=1 unsigned)
//   - encode_smull as independent differential: rejected — shared get_reg / same TU;
//     alias equality is algebraic.metamorphic, not an independent implementation
// Weaker available: algebraic.metamorphic (SMULL Ra=XZR alias, U bit vs UMADDL, field isolation),
//   algebraic.invariant (ARM field layout), negative_error (arity / extra / SP / FP / wrong width)
// Differential: candidate=encode_smaddl, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[Reg(Xd), Reg(Wn), Reg(Wm), Reg(Xa)] <-> `smaddl Xd, Wn, Wm, Xa`

use super::encode_smaddl;
use super::encode_smull;
use super::encode_umaddl;
use super::EncodeResult;
use crate::backend::arm::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;
const SMADDL_BASE: u32 = 0x9B20_0000; // sf=1 U=0 o0=0, Rd=Rn=Rm=Ra=0

fn cfg() -> ProptestConfig {
    ProptestConfig::with_cases(CASES)
}

fn xreg(n: u32) -> String {
    if n == 31 {
        "xzr".into()
    } else {
        format!("x{n}")
    }
}

fn wreg(n: u32) -> String {
    if n == 31 {
        "wzr".into()
    } else {
        format!("w{n}")
    }
}

fn gpr(is_64: bool, n: u32) -> String {
    if is_64 {
        xreg(n)
    } else {
        wreg(n)
    }
}

fn sut_word(ops: &[Operand]) -> Result<u32, String> {
    match encode_smaddl(ops)? {
        EncodeResult::Word(w) => Ok(w),
        other => Err(format!("expected Word, got {:?}", other)),
    }
}

fn umaddl_word(ops: &[Operand]) -> Result<u32, String> {
    match encode_umaddl(ops)? {
        EncodeResult::Word(w) => Ok(w),
        other => Err(format!("expected Word, got {:?}", other)),
    }
}

fn smull_word(ops: &[Operand]) -> Result<u32, String> {
    match encode_smull(ops)? {
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
fn encode_smaddl_kat_llvm_mc_x0_w1_w2_x3() {
    let want = 0x9b220c20u32;
    let mc = llvm_mc_word("smaddl x0, w1, w2, x3").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [
        Operand::Reg("x0".into()),
        Operand::Reg("w1".into()),
        Operand::Reg("w2".into()),
        Operand::Reg("x3".into()),
    ];
    let sut = sut_word(&ops).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_smaddl_kat_llvm_mc_xzr_wzr_wzr_xzr() {
    let want = 0x9b3f7fffu32;
    let mc = llvm_mc_word("smaddl xzr, wzr, wzr, xzr").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [
        Operand::Reg("xzr".into()),
        Operand::Reg("wzr".into()),
        Operand::Reg("wzr".into()),
        Operand::Reg("xzr".into()),
    ];
    let sut = sut_word(&ops).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_smaddl_kat_llvm_mc_alias_smull_xzr() {
    let want = 0x9b227c20u32;
    let mc_smaddl = llvm_mc_word("smaddl x0, w1, w2, xzr").expect("llvm-mc SMADDL ZR KAT");
    let mc_smull = llvm_mc_word("smull x0, w1, w2").expect("llvm-mc SMULL KAT");
    assert_eq!(mc_smaddl, want, "llvm-mc SMADDL ZR KAT mapping broken");
    assert_eq!(mc_smull, want, "llvm-mc SMULL KAT mapping broken");
    let ops = [
        Operand::Reg("x0".into()),
        Operand::Reg("w1".into()),
        Operand::Reg("w2".into()),
        Operand::Reg("xzr".into()),
    ];
    let sut = sut_word(&ops).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_smaddl_kat_llvm_mc_lr_w1_w2_x30() {
    let want = 0x9b22783eu32;
    let mc = llvm_mc_word("smaddl lr, w1, w2, x30").expect("llvm-mc LR KAT");
    assert_eq!(mc, want, "llvm-mc LR KAT mapping broken");
    let ops = [
        Operand::Reg("lr".into()),
        Operand::Reg("w1".into()),
        Operand::Reg("w2".into()),
        Operand::Reg("x30".into()),
    ];
    let sut = sut_word(&ops).expect("SUT KAT");
    assert_eq!(sut, want);
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_smaddl_diff_valid_gpr(
        rd in 0u32..=31,
        rn in 0u32..=31,
        rm in 0u32..=31,
        ra in 0u32..=31,
        use_lr in any::<bool>(),
    ) {
        let dest = if use_lr && rd == 30 {
            "lr".to_string()
        } else {
            xreg(rd)
        };
        let src_n = wreg(rn);
        let src_m = wreg(rm);
        let acc = if use_lr && ra == 30 {
            "lr".to_string()
        } else {
            xreg(ra)
        };
        let asm = format!("smaddl {}, {}, {}, {}", dest, src_n, src_m, acc);
        let ops = [
            Operand::Reg(dest),
            Operand::Reg(src_n),
            Operand::Reg(src_m),
            Operand::Reg(acc),
        ];
        let mc = llvm_mc_word(&asm).expect("llvm-mc");
        let sut = sut_word(&ops).expect("SUT");
        prop_assert_eq!(sut, mc, "SMADDL mismatch for {}", asm);
    }

    #[test]
    fn encode_smaddl_alias_smull_xzr(
        rd in 0u32..=31,
        rn in 0u32..=31,
        rm in 0u32..=31,
    ) {
        let dest = xreg(rd);
        let src_n = wreg(rn);
        let src_m = wreg(rm);
        let smaddl_ops = [
            Operand::Reg(dest.clone()),
            Operand::Reg(src_n.clone()),
            Operand::Reg(src_m.clone()),
            Operand::Reg("xzr".into()),
        ];
        let smull_ops = [
            Operand::Reg(dest.clone()),
            Operand::Reg(src_n.clone()),
            Operand::Reg(src_m.clone()),
        ];
        let sut = sut_word(&smaddl_ops).expect("SUT SMADDL XZR");
        let alias = smull_word(&smull_ops).expect("SUT SMULL");
        prop_assert_eq!(sut, alias, "SMADDL with Ra=XZR must equal SMULL");
        let mc_smaddl = llvm_mc_word(&format!("smaddl {}, {}, {}, xzr", dest, src_n, src_m))
            .expect("llvm-mc SMADDL XZR");
        let mc_smull = llvm_mc_word(&format!("smull {}, {}, {}", dest, src_n, src_m))
            .expect("llvm-mc SMULL");
        prop_assert_eq!(sut, mc_smaddl);
        prop_assert_eq!(sut, mc_smull);
    }

    #[test]
    fn encode_smaddl_xor_umaddl_u_bit(
        rd in 0u32..=31,
        rn in 0u32..=31,
        rm in 0u32..=31,
        ra in 0u32..=31,
    ) {
        let ops = [
            Operand::Reg(xreg(rd)),
            Operand::Reg(wreg(rn)),
            Operand::Reg(wreg(rm)),
            Operand::Reg(xreg(ra)),
        ];
        let s = sut_word(&ops).expect("SMADDL");
        let u = umaddl_word(&ops).expect("UMADDL");
        prop_assert_eq!(s ^ u, 1u32 << 23, "SMADDL XOR UMADDL must be U bit 23");
    }

    #[test]
    fn encode_smaddl_arm_fields(
        rd in 0u32..=31,
        rn in 0u32..=31,
        rm in 0u32..=31,
        ra in 0u32..=31,
    ) {
        let ops = [
            Operand::Reg(xreg(rd)),
            Operand::Reg(wreg(rn)),
            Operand::Reg(wreg(rm)),
            Operand::Reg(xreg(ra)),
        ];
        let w = sut_word(&ops).expect("SUT");
        let want = SMADDL_BASE | (rm << 16) | (ra << 10) | (rn << 5) | rd;
        prop_assert_eq!(w, want, "ARM ARM SMADDL field layout");
        prop_assert_eq!(w >> 31, 1, "sf must be 1");
        prop_assert_eq!((w >> 21) & 0x3ff, 0b00_11011_001, "bits[30:21]");
        prop_assert_eq!((w >> 16) & 0x1f, rm, "Rm");
        prop_assert_eq!((w >> 15) & 1, 0, "o0 must be 0");
        prop_assert_eq!((w >> 10) & 0x1f, ra, "Ra");
        prop_assert_eq!((w >> 5) & 0x1f, rn, "Rn");
        prop_assert_eq!(w & 0x1f, rd, "Rd");
    }

    #[test]
    fn encode_smaddl_neg_arity(
        len in 0usize..=3,
        a in 0u32..=31,
        b in 0u32..=31,
        junk in non_reg_operand(),
        use_junk in any::<bool>(),
    ) {
        let mut ops = Vec::new();
        if len >= 1 {
            ops.push(if use_junk {
                junk.clone()
            } else {
                Operand::Reg(xreg(a))
            });
        }
        if len >= 2 {
            ops.push(Operand::Reg(wreg(b)));
        }
        if len >= 3 {
            ops.push(Operand::Reg(wreg(a)));
        }
        prop_assert!(
            encode_smaddl(&ops).is_err(),
            "SMADDL with {} operands must Err (llvm-mc: too few operands)",
            len
        );
    }

    #[test]
    fn encode_smaddl_neg_extra_operand(
        rd in 0u32..=31,
        rn in 0u32..=31,
        rm in 0u32..=31,
        ra in 0u32..=31,
        extra in extra_operand(),
    ) {
        let ops = vec![
            Operand::Reg(xreg(rd)),
            Operand::Reg(wreg(rn)),
            Operand::Reg(wreg(rm)),
            Operand::Reg(xreg(ra)),
            extra,
        ];
        prop_assert!(
            encode_smaddl(&ops).is_err(),
            "SMADDL has no 5th operand; extra operand must Err (llvm-mc rejects it)"
        );
    }

    #[test]
    fn encode_smaddl_neg_wrong_width(
        rd in 0u32..=30,
        rn in 0u32..=30,
        rm in 0u32..=30,
        ra in 0u32..=30,
        rd64 in any::<bool>(),
        rn64 in any::<bool>(),
        rm64 in any::<bool>(),
        ra64 in any::<bool>(),
    ) {
        prop_assume!(!(rd64 && !rn64 && !rm64 && ra64));
        let ops = [
            Operand::Reg(gpr(rd64, rd)),
            Operand::Reg(gpr(rn64, rn)),
            Operand::Reg(gpr(rm64, rm)),
            Operand::Reg(gpr(ra64, ra)),
        ];
        prop_assert!(
            encode_smaddl(&ops).is_err(),
            "SMADDL requires Xd, Wn, Wm, Xa; rd64={} rn64={} rm64={} ra64={} must Err (llvm-mc rejects it)",
            rd64,
            rn64,
            rm64,
            ra64
        );
    }

    #[test]
    fn encode_smaddl_neg_sp(
        which in 0u32..=3,
        is_64 in any::<bool>(),
        a in 0u32..=30,
        b in 0u32..=30,
    ) {
        let sp = if is_64 { "sp" } else { "wsp" };
        let mut names = [xreg(a), wreg(a), wreg(b), xreg(b)];
        names[which as usize] = sp.to_string();
        let ops = [
            Operand::Reg(names[0].clone()),
            Operand::Reg(names[1].clone()),
            Operand::Reg(names[2].clone()),
            Operand::Reg(names[3].clone()),
        ];
        prop_assert!(
            encode_smaddl(&ops).is_err(),
            "SP/WSP is not a valid SMADDL operand (which={} names={:?})",
            which,
            names
        );
    }

        #[test]
        fn encode_smaddl_diff_alt_spellings(
            rd in 0u32..=31,
            rn in 0u32..=31,
            rm in 0u32..=31,
            ra in 0u32..=31,
            dest_spell in 0u32..=4,
            src_spell in 0u32..=2,
            acc_spell in 0u32..=4,
        ) {
            let dest = match dest_spell {
                0 if rd == 31 => "x31".to_string(),
                1 if rd == 31 => "XZR".to_string(),
                2 if rd == 30 => "LR".to_string(),
                3 => xreg(rd).to_uppercase(),
                _ => xreg(rd),
            };
            let src_n = match src_spell {
                0 if rn == 31 => "w31".to_string(),
                1 => wreg(rn).to_uppercase(),
                _ => wreg(rn),
            };
            let src_m = match src_spell {
                0 if rm == 31 => "w31".to_string(),
                1 => wreg(rm).to_uppercase(),
                _ => wreg(rm),
            };
            let acc = match acc_spell {
                0 if ra == 31 => "x31".to_string(),
                1 if ra == 31 => "XZR".to_string(),
                2 if ra == 30 => "LR".to_string(),
                3 => xreg(ra).to_uppercase(),
                _ => xreg(ra),
            };
            let asm = format!("smaddl {}, {}, {}, {}", dest, src_n, src_m, acc);
            let ops = [
                Operand::Reg(dest),
                Operand::Reg(src_n),
                Operand::Reg(src_m),
                Operand::Reg(acc),
            ];
            let mc = llvm_mc_word(&asm).expect("llvm-mc");
            let sut = sut_word(&ops).expect("SUT");
            prop_assert_eq!(sut, mc, "SMADDL alt-spelling mismatch for {}", asm);
        }

        #[test]
        fn encode_smaddl_neg_fp(
            which in 0u32..=3,
            prefix in prop::sample::select(vec!["d", "s", "q", "v", "h", "b"]),
            n in prop_oneof![Just(0u32), Just(31u32), 0u32..=31],
        ) {
            let fp = format!("{}{}", prefix, n);
            let mut ops = vec![
                Operand::Reg("x0".into()),
                Operand::Reg("w1".into()),
                Operand::Reg("w2".into()),
                Operand::Reg("x3".into()),
            ];
            ops[which as usize] = Operand::Reg(fp.clone());
            prop_assert!(
                encode_smaddl(&ops).is_err(),
                "FP/SIMD register {} is not a valid SMADDL operand (which={})",
                fp,
                which
            );
        }

        #[test]
        fn encode_smaddl_neg_nonreg(
            which in 0u32..=3,
            bad in non_reg_operand(),
        ) {
            let mut ops = vec![
                Operand::Reg("x0".into()),
                Operand::Reg("w1".into()),
                Operand::Reg("w2".into()),
                Operand::Reg("x3".into()),
            ];
            ops[which as usize] = bad;
            prop_assert!(
                encode_smaddl(&ops).is_err(),
                "non-register operand at slot {} must Err",
                which
            );
        }

        #[test]
        fn encode_smaddl_neg_invalid_name(
            which in 0u32..=3,
            name in invalid_name(),
        ) {
            let mut ops = vec![
                Operand::Reg("x0".into()),
                Operand::Reg("w1".into()),
                Operand::Reg("w2".into()),
                Operand::Reg("x3".into()),
            ];
            ops[which as usize] = Operand::Reg(name.clone());
            prop_assert!(
                encode_smaddl(&ops).is_err(),
                "invalid register name {:?} at slot {} must Err",
                name,
                which
            );
        }

        #[test]
        fn encode_smaddl_meta_rd_rn_rm_ra(
            rd in 0u32..=30,
            rn in 0u32..=30,
            rm in 0u32..=30,
            ra in 0u32..=30,
        ) {
            let base_ops = [
                Operand::Reg(xreg(rd)),
                Operand::Reg(wreg(rn)),
                Operand::Reg(wreg(rm)),
                Operand::Reg(xreg(ra)),
            ];
            let base = sut_word(&base_ops).expect("base");
            let rd1 = sut_word(&[
                Operand::Reg(xreg(rd + 1)),
                Operand::Reg(wreg(rn)),
                Operand::Reg(wreg(rm)),
                Operand::Reg(xreg(ra)),
            ]).expect("rd+1");
            let rn1 = sut_word(&[
                Operand::Reg(xreg(rd)),
                Operand::Reg(wreg(rn + 1)),
                Operand::Reg(wreg(rm)),
                Operand::Reg(xreg(ra)),
            ]).expect("rn+1");
            let rm1 = sut_word(&[
                Operand::Reg(xreg(rd)),
                Operand::Reg(wreg(rn)),
                Operand::Reg(wreg(rm + 1)),
                Operand::Reg(xreg(ra)),
            ]).expect("rm+1");
            let ra1 = sut_word(&[
                Operand::Reg(xreg(rd)),
                Operand::Reg(wreg(rn)),
                Operand::Reg(wreg(rm)),
                Operand::Reg(xreg(ra + 1)),
            ]).expect("ra+1");
            prop_assert_eq!((rd1 ^ base) & !0x1fu32, 0, "Rd+1 only in bits[4:0]");
            prop_assert_eq!(rd1 & 0x1f, rd + 1);
            prop_assert_eq!((rn1 ^ base) & !(0x1fu32 << 5), 0, "Rn+1 only in bits[9:5]");
            prop_assert_eq!((rn1 >> 5) & 0x1f, rn + 1);
            prop_assert_eq!((rm1 ^ base) & !(0x1fu32 << 16), 0, "Rm+1 only in bits[20:16]");
            prop_assert_eq!((rm1 >> 16) & 0x1f, rm + 1);
            prop_assert_eq!((ra1 ^ base) & !(0x1fu32 << 10), 0, "Ra+1 only in bits[14:10]");
            prop_assert_eq!((ra1 >> 10) & 0x1f, ra + 1);
        }
    }

    #[test]
    fn test_encode_smaddl_regression_extra_operand() {
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("x0".into()),
            Operand::Reg("x0".into()),
        ];
        assert!(
            encode_smaddl(&ops).is_err(),
            "SMADDL x0, w0, w0, x0, x0 must Err; extra operand is invalid (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_smaddl_regression_wrong_width() {
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
        ];
        assert!(
            encode_smaddl(&ops).is_err(),
            "SMADDL w0, w0, w0, w0 must Err; dest must be Xd and acc must be Xa (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_smaddl_regression_sp() {
        let ops = [
            Operand::Reg("wsp".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("x0".into()),
        ];
        assert!(
            encode_smaddl(&ops).is_err(),
            "SMADDL wsp, w0, w0, x0 must Err; register 31 is WZR not WSP (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_smaddl_regression_fp() {
        let ops = [
            Operand::Reg("d0".into()),
            Operand::Reg("w1".into()),
            Operand::Reg("w2".into()),
            Operand::Reg("x3".into()),
        ];
        assert!(
            encode_smaddl(&ops).is_err(),
            "SMADDL d0, w1, w2, x3 must Err; FP/SIMD registers are not SMADDL operands (llvm-mc rejects it)"
        );
    }
