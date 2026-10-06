// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md:217 Extensions table lists sxtb;
//   encoder/mod.rs:3 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:435 "sxtb" => encode_sxtb(operands);
//   ARM ARM C6 SXTB is the alias of SBFM Rd, Rn, #0, #7:
//     sf 00 100110 N=sf immr=0 imms=7 Rn Rd;
//     forms SXTB Wd, Wn and SXTB Xd, Wn (source is always W).
// Stronger considered:
//   - State machine: rejected — encode_sxtb is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree SXTB/SBFM decoder
//   - encode_sxth / encode_sxtw / encode_uxtb as differential siblings: rejected —
//     same-job gate (imms=15 / 32-bit-only / UBFM opc=10; shared get_reg / same crate)
//   - encode_sbfm as independent differential: rejected — shared get_reg / same crate;
//     alias equality is algebraic.metamorphic, not an independent implementation
// Weaker available: algebraic.metamorphic (SBFM #0,#7 alias, sf/N bit X vs W, field isolation),
//   algebraic.invariant (ARM field layout), negative_error (arity / extra / Wd+Xn / SP / FP)
// Differential: candidate=encode_sxtb, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[Reg(Rd), Reg(Wn)] <-> `sxtb Rd, Wn`

use super::encode_sbfm;
use super::encode_sxtb;
use super::EncodeResult;
use crate::backend::arm::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;
const SXTB32_BASE: u32 = 0x1300_1C00; // sf=0 N=0 immr=0 imms=7, Rd=Rn=0
const SXTB64_BASE: u32 = 0x9340_1C00; // sf=1 N=1 immr=0 imms=7, Rd=Rn=0

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
    match encode_sxtb(ops)? {
        EncodeResult::Word(w) => Ok(w),
        other => Err(format!("expected Word, got {:?}", other)),
    }
}

fn sbfm_word(ops: &[Operand]) -> Result<u32, String> {
    match encode_sbfm(ops)? {
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
fn encode_sxtb_kat_llvm_mc_w0_w1() {
    let want = 0x1300_1c20u32;
    let mc = llvm_mc_word("sxtb w0, w1").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [Operand::Reg("w0".into()), Operand::Reg("w1".into())];
    let sut = sut_word(&ops).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_sxtb_kat_llvm_mc_x0_w1() {
    let want = 0x9340_1c20u32;
    let mc = llvm_mc_word("sxtb x0, w1").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [Operand::Reg("x0".into()), Operand::Reg("w1".into())];
    let sut = sut_word(&ops).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_sxtb_kat_llvm_mc_wzr_wzr() {
    let want = 0x1300_1fffu32;
    let mc = llvm_mc_word("sxtb wzr, wzr").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [Operand::Reg("wzr".into()), Operand::Reg("wzr".into())];
    let sut = sut_word(&ops).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_sxtb_kat_llvm_mc_alias_sbfm() {
    let want = 0x1300_1c20u32;
    let mc_sxtb = llvm_mc_word("sxtb w0, w1").expect("llvm-mc SXTB KAT");
    let mc_sbfm = llvm_mc_word("sbfm w0, w1, #0, #7").expect("llvm-mc SBFM KAT");
    assert_eq!(mc_sxtb, want, "llvm-mc SXTB KAT mapping broken");
    assert_eq!(mc_sbfm, want, "llvm-mc SBFM KAT mapping broken");
    let ops = [Operand::Reg("w0".into()), Operand::Reg("w1".into())];
    let sut = sut_word(&ops).expect("SUT KAT");
    assert_eq!(sut, want);
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_sxtb_diff_valid_gpr(
        rd in 0u32..=31,
        rn in 0u32..=31,
        is_64 in any::<bool>(),
        use_lr in any::<bool>(),
    ) {
        let dest = if use_lr && is_64 && rd == 30 {
            "lr".to_string()
        } else {
            gpr(is_64, rd)
        };
        let src = wreg(rn);
        let asm = format!("sxtb {}, {}", dest, src);
        let ops = [Operand::Reg(dest), Operand::Reg(src)];
        let mc = llvm_mc_word(&asm).expect("llvm-mc");
        let sut = sut_word(&ops).expect("SUT");
        prop_assert_eq!(sut, mc, "SXTB mismatch for {}", asm);
    }

    #[test]
    fn encode_sxtb_alias_sbfm(
        rd in 0u32..=31,
        rn in 0u32..=31,
        is_64 in any::<bool>(),
    ) {
        let dest = gpr(is_64, rd);
        let src = gpr(is_64, rn);
        let sxtb_ops = [Operand::Reg(dest.clone()), Operand::Reg(src.clone())];
        let sbfm_ops = [
            Operand::Reg(dest.clone()),
            Operand::Reg(src.clone()),
            Operand::Imm(0),
            Operand::Imm(7),
        ];
        let sut = sut_word(&sxtb_ops).expect("SUT SXTB");
        let alias = sbfm_word(&sbfm_ops).expect("SUT SBFM #0,#7");
        prop_assert_eq!(sut, alias, "SXTB must equal SBFM with #0,#7");
        let mc_src = wreg(rn);
        let mc = llvm_mc_word(&format!("sxtb {}, {}", dest, mc_src)).expect("llvm-mc SXTB");
        prop_assert_eq!(sut, mc);
    }

    #[test]
    fn encode_sxtb_arm_fields(
        rd in 0u32..=31,
        rn in 0u32..=31,
        is_64 in any::<bool>(),
    ) {
        let ops = [Operand::Reg(gpr(is_64, rd)), Operand::Reg(wreg(rn))];
        let w = sut_word(&ops).expect("SUT");
        let base = if is_64 { SXTB64_BASE } else { SXTB32_BASE };
        let want = base | (rn << 5) | rd;
        prop_assert_eq!(w, want, "ARM ARM SXTB field layout");
        prop_assert_eq!(w >> 31, u32::from(is_64), "sf");
        prop_assert_eq!((w >> 29) & 0b11, 0, "opc must be 00 (SBFM)");
        prop_assert_eq!((w >> 23) & 0x3f, 0b100110, "bits[28:23]");
        prop_assert_eq!((w >> 22) & 1, u32::from(is_64), "N must equal sf");
        prop_assert_eq!((w >> 16) & 0x1f, 0, "immr must be 0");
        prop_assert_eq!((w >> 10) & 0x3f, 7, "imms must be 7");
        prop_assert_eq!((w >> 5) & 0x1f, rn, "Rn");
        prop_assert_eq!(w & 0x1f, rd, "Rd");
    }

    #[test]
    fn encode_sxtb_neg_arity(
        len in 0usize..=1,
        a in 0u32..=31,
        is_64 in any::<bool>(),
        junk in extra_operand(),
        use_junk in any::<bool>(),
    ) {
        let mut ops = Vec::new();
        if len >= 1 {
            ops.push(if use_junk {
                junk.clone()
            } else {
                Operand::Reg(gpr(is_64, a))
            });
        }
        prop_assert!(
            encode_sxtb(&ops).is_err(),
            "SXTB with {} operands must Err (llvm-mc: too few operands)",
            len
        );
    }

    #[test]
    fn encode_sxtb_neg_extra_operand(
        rd in 0u32..=31,
        rn in 0u32..=31,
        is_64 in any::<bool>(),
        extra in extra_operand(),
    ) {
        let ops = vec![
            Operand::Reg(gpr(is_64, rd)),
            Operand::Reg(wreg(rn)),
            extra,
        ];
        prop_assert!(
            encode_sxtb(&ops).is_err(),
            "SXTB has no 3rd operand; extra operand must Err (llvm-mc rejects it)"
        );
    }

    #[test]
    fn encode_sxtb_neg_wd_xn(
        rd in 0u32..=31,
        rn in 0u32..=31,
    ) {
        let ops = [Operand::Reg(wreg(rd)), Operand::Reg(xreg(rn))];
        prop_assert!(
            encode_sxtb(&ops).is_err(),
            "SXTB Wd, Xn must Err (llvm-mc rejects sxtb {}, {})",
            wreg(rd),
            xreg(rn)
        );
    }

    #[test]
    fn encode_sxtb_neg_sp(
        which in 0u32..=1,
        is_64_sp in any::<bool>(),
        a in 0u32..=30,
        dest64 in any::<bool>(),
    ) {
        let sp = if is_64_sp { "sp" } else { "wsp" };
        let mut names = [gpr(dest64, a), wreg(a)];
        names[which as usize] = sp.to_string();
        let ops = [
            Operand::Reg(names[0].clone()),
            Operand::Reg(names[1].clone()),
        ];
        prop_assert!(
            encode_sxtb(&ops).is_err(),
            "SP/WSP is not a valid SXTB operand (which={} names={:?})",
            which,
            names
        );
    }

    #[test]
    fn encode_sxtb_neg_fp(
        which in 0u32..=1,
        prefix in prop::sample::select(vec!["d", "s", "q", "v", "h", "b"]),
        n in prop_oneof![Just(0u32), Just(31u32), 0u32..=31],
    ) {
        let fp = format!("{}{}", prefix, n);
        let mut ops = vec![
            Operand::Reg("w0".into()),
            Operand::Reg("w1".into()),
        ];
        ops[which as usize] = Operand::Reg(fp.clone());
        prop_assert!(
            encode_sxtb(&ops).is_err(),
            "FP/SIMD register {} is not a valid SXTB operand (which={})",
            fp,
            which
        );
    }

    #[test]
    fn encode_sxtb_diff_alt_spellings(
        rd in 0u32..=31,
        rn in 0u32..=31,
        is_64 in any::<bool>(),
        dest_spell in 0u32..=4,
        src_spell in 0u32..=3,
    ) {
        // Cover x31/w31 (not just xzr/wzr), uppercase, LR, and X-source for 64-bit dest
        // (llvm-mc canonicalizes sxtb Xd, Xn to sxtb Xd, Wn).
        let dest = match dest_spell {
            0 if is_64 && rd == 31 => "x31".to_string(),
            1 if is_64 && rd == 31 => "XZR".to_string(),
            2 if is_64 && rd == 30 => "LR".to_string(),
            3 => gpr(is_64, rd).to_uppercase(),
            _ => gpr(is_64, rd),
        };
        let src = match src_spell {
            0 if rn == 31 => "w31".to_string(),
            1 => wreg(rn).to_uppercase(),
            2 if is_64 => xreg(rn), // Xd, Xn accepted by llvm-mc
            _ => wreg(rn),
        };
        let asm = format!("sxtb {}, {}", dest, src);
        let ops = [Operand::Reg(dest), Operand::Reg(src)];
        let mc = llvm_mc_word(&asm).expect("llvm-mc");
        let sut = sut_word(&ops).expect("SUT");
        prop_assert_eq!(sut, mc, "SXTB alt-spelling mismatch for {}", asm);
    }

    #[test]
    fn encode_sxtb_neg_nonreg(
        which in 0u32..=1,
        bad in non_reg_operand(),
    ) {
        let mut ops = vec![
            Operand::Reg("w0".into()),
            Operand::Reg("w1".into()),
        ];
        ops[which as usize] = bad;
        prop_assert!(
            encode_sxtb(&ops).is_err(),
            "non-register operand at slot {} must Err",
            which
        );
    }

    #[test]
    fn encode_sxtb_neg_invalid_name(
        which in 0u32..=1,
        name in invalid_name(),
    ) {
        let mut ops = vec![
            Operand::Reg("w0".into()),
            Operand::Reg("w1".into()),
        ];
        ops[which as usize] = Operand::Reg(name.clone());
        prop_assert!(
            encode_sxtb(&ops).is_err(),
            "invalid register name {:?} at slot {} must Err",
            name,
            which
        );
    }

    #[test]
    fn encode_sxtb_meta_rd_rn(
        rd in 0u32..=30,
        rn in 0u32..=30,
        is_64 in any::<bool>(),
    ) {
        let base = sut_word(&[
            Operand::Reg(gpr(is_64, rd)),
            Operand::Reg(wreg(rn)),
        ])
        .expect("base");
        let rd1 = sut_word(&[
            Operand::Reg(gpr(is_64, rd + 1)),
            Operand::Reg(wreg(rn)),
        ])
        .expect("rd+1");
        let rn1 = sut_word(&[
            Operand::Reg(gpr(is_64, rd)),
            Operand::Reg(wreg(rn + 1)),
        ])
        .expect("rn+1");
        prop_assert_eq!((rd1 ^ base) & !0x1fu32, 0, "Rd+1 only in bits[4:0]");
        prop_assert_eq!(rd1 & 0x1f, rd + 1);
        prop_assert_eq!((rn1 ^ base) & !(0x1fu32 << 5), 0, "Rn+1 only in bits[9:5]");
        prop_assert_eq!((rn1 >> 5) & 0x1f, rn + 1);
    }
}

#[test]
fn test_encode_sxtb_regression_extra_operand() {
    let ops = [
        Operand::Reg("w0".into()),
        Operand::Reg("w0".into()),
        Operand::Reg("x0".into()),
    ];
    assert!(
        encode_sxtb(&ops).is_err(),
        "SXTB w0, w0, x0 must Err; extra operand is invalid (llvm-mc rejects it)"
    );
}

#[test]
fn test_encode_sxtb_regression_wd_xn() {
    let ops = [Operand::Reg("w0".into()), Operand::Reg("x0".into())];
    assert!(
        encode_sxtb(&ops).is_err(),
        "SXTB w0, x0 must Err; 32-bit dest requires Wn source (llvm-mc rejects it)"
    );
}

#[test]
fn test_encode_sxtb_regression_sp() {
    let ops = [Operand::Reg("wsp".into()), Operand::Reg("w0".into())];
    assert!(
        encode_sxtb(&ops).is_err(),
        "SXTB wsp, w0 must Err; register 31 is WZR not WSP (llvm-mc rejects it)"
    );
}

#[test]
fn test_encode_sxtb_regression_fp() {
    let ops = [Operand::Reg("d0".into()), Operand::Reg("w1".into())];
    assert!(
        encode_sxtb(&ops).is_err(),
        "SXTB d0, w1 must Err; FP/SIMD registers are not SXTB operands (llvm-mc rejects it)"
    );
}
