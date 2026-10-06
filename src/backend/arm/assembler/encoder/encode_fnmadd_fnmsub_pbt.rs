// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md "accepts the same textual assembly that GCC's gas would consume";
//   encoder/mod.rs "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:565-566 fnmadd/fnmsub dispatch to encode_fnmadd_fnmsub;
//   ARM ARM Floating-point data-processing (3 source):
//   M=0 S=0 11111 ftype o1 Rm o0 Ra Rn Rd;
//   FNMADD o1=1 o0=0, FNMSUB o1=1 o0=1; ftype 00=S, 01=D, 11=H;
//   fp_scalar.rs:143-144 purpose comment (Rd = -Ra +/- (Rn * Rm);
//   Format: 0 00 11111 ftype 1 Rm o1 Ra Rn Rd);
//   README.md:223 lists scalar fnmadd/fnmsub.
// Stronger considered:
//   - State machine: rejected — encode_fnmadd_fnmsub is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree FNMADD/FNMSUB decoder
//   - encode_fmadd_fmsub as differential sibling:
//     rejected — same-job gate fails (o1=0 non-negated fused class)
//   - encode_fp_arith: rejected — 2-source FP class
//   - encode_madd: rejected — integer MADD
// Weaker available: algebraic.metamorphic (ftype/o0/Rd/Rn/Rm/Ra),
//   algebraic.invariant (ARM fields), negative_error (arity / extra / wrong type / nonreg)
// Differential: candidate=encode_fnmadd_fnmsub, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler (caller-reachable from encode_instruction),
//   mapping=[Reg(Sd|Dd|Hd), Reg(Sn|Dn|Hn), Reg(Sm|Dm|Hm), Reg(Sa|Da|Ha)]+is_sub
//   <-> `fnmadd|fnmsub Sd|Dd|Hd, Sn|Dn|Hn, Sm|Dm|Hm, Sa|Da|Ha`

use super::encode_fnmadd_fnmsub;
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

fn fp(is_d: bool, n: u32) -> String {
    format!("{}{}", if is_d { "d" } else { "s" }, n)
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

fn mnem(is_sub: bool) -> &'static str {
    if is_sub {
        "fnmsub"
    } else {
        "fnmadd"
    }
}

fn sut_word(ops: &[Operand], is_sub: bool) -> Result<u32, String> {
    match encode_fnmadd_fnmsub(ops, is_sub)? {
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

fn llvm_mc_word_with(asm: &str, extra_args: &[&str]) -> Result<u32, String> {
    let mut args = vec!["-triple=aarch64", "-show-encoding"];
    args.extend_from_slice(extra_args);
    let mut child = Command::new(LLVM_MC)
        .args(&args)
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

fn llvm_mc_word(asm: &str) -> Result<u32, String> {
    llvm_mc_word_with(asm, &[])
}

fn llvm_mc_fp16_word(asm: &str) -> Result<u32, String> {
    llvm_mc_word_with(asm, &["-mattr=+fullfp16"])
}

fn extra_operand() -> impl Strategy<Value = Operand> {
    prop_oneof![
        (0u32..=31).prop_map(|n| Operand::Reg(format!("s{n}"))),
        Just(Operand::Imm(0)),
        Just(Operand::Imm(1)),
        Just(Operand::Imm(32)),
        Just(Operand::Shift {
            kind: "lsl".into(),
            amount: 0,
        }),
        Just(Operand::RegArrangement {
            reg: "v0".into(),
            arrangement: "4s".into(),
        }),
    ]
}

fn fp_spelling(is_d: bool, n: u32, kind: u32) -> String {
    match kind {
        0 => fp(is_d, n).to_uppercase(),
        _ => fp(is_d, n),
    }
}

fn wrong_type_quad() -> impl Strategy<Value = (String, String, String, String)> {
    let n = 0u32..=31;
    prop_oneof![
        // mixed S/D on one slot
        (n.clone(), n.clone(), n.clone(), n.clone(), 0u32..=3).prop_map(|(d, s, m, a, slot)| {
            let mut t = [fp(false, d), fp(false, s), fp(false, m), fp(false, a)];
            let nums = [d, s, m, a];
            t[slot as usize] = fp(true, nums[slot as usize]);
            (t[0].clone(), t[1].clone(), t[2].clone(), t[3].clone())
        }),
        // GPR in one slot, matching S elsewhere
        (n.clone(), n.clone(), n.clone(), n.clone(), 0u32..=3, any::<bool>()).prop_map(
            |(d, s, m, a, slot, is64)| {
                let mut t = [fp(false, d), fp(false, s), fp(false, m), fp(false, a)];
                let nums = [d, s, m, a];
                t[slot as usize] = gpr(is64, nums[slot as usize]);
                (t[0].clone(), t[1].clone(), t[2].clone(), t[3].clone())
            },
        ),
        // Q/V/B in one slot
        (n.clone(), n.clone(), n.clone(), n.clone(), 0u32..=3, 0u32..=2).prop_map(
            |(d, s, m, a, slot, p)| {
                let pref = ["q", "v", "b"][p as usize];
                let mut t = [fp(false, d), fp(false, s), fp(false, m), fp(false, a)];
                let nums = [d, s, m, a];
                t[slot as usize] = format!("{pref}{}", nums[slot as usize]);
                (t[0].clone(), t[1].clone(), t[2].clone(), t[3].clone())
            },
        ),
        // SP/WSP in one slot
        (n.clone(), n.clone(), n.clone(), n.clone(), 0u32..=3, any::<bool>()).prop_map(
            |(d, s, m, a, slot, is64)| {
                let sp = if is64 { "sp" } else { "wsp" };
                let mut t = [fp(false, d), fp(false, s), fp(false, m), fp(false, a)];
                t[slot as usize] = sp.to_string();
                (t[0].clone(), t[1].clone(), t[2].clone(), t[3].clone())
            },
        ),
        // all-GPR
        (n.clone(), n.clone(), n.clone(), n.clone(), any::<bool>()).prop_map(|(d, s, m, a, is64)| {
            (gpr(is64, d), gpr(is64, s), gpr(is64, m), gpr(is64, a))
        }),
    ]
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_fnmadd_fnmsub_kat_llvm_mc_fnmadd_s0_s1_s2_s3() {
    let want = 0x1f220c20u32;
    let mc = llvm_mc_word("fnmadd s0, s1, s2, s3").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [
        Operand::Reg("s0".into()),
        Operand::Reg("s1".into()),
        Operand::Reg("s2".into()),
        Operand::Reg("s3".into()),
    ];
    let sut = sut_word(&ops, false).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_fnmadd_fnmsub_kat_llvm_mc_fnmadd_d0_d1_d2_d3() {
    let want = 0x1f620c20u32;
    let mc = llvm_mc_word("fnmadd d0, d1, d2, d3").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [
        Operand::Reg("d0".into()),
        Operand::Reg("d1".into()),
        Operand::Reg("d2".into()),
        Operand::Reg("d3".into()),
    ];
    let sut = sut_word(&ops, false).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_fnmadd_fnmsub_kat_llvm_mc_fnmsub_s0_s1_s2_s3() {
    let want = 0x1f228c20u32;
    let mc = llvm_mc_word("fnmsub s0, s1, s2, s3").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [
        Operand::Reg("s0".into()),
        Operand::Reg("s1".into()),
        Operand::Reg("s2".into()),
        Operand::Reg("s3".into()),
    ];
    let sut = sut_word(&ops, true).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_fnmadd_fnmsub_kat_llvm_mc_fnmsub_d0_d1_d2_d3() {
    let want = 0x1f628c20u32;
    let mc = llvm_mc_word("fnmsub d0, d1, d2, d3").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [
        Operand::Reg("d0".into()),
        Operand::Reg("d1".into()),
        Operand::Reg("d2".into()),
        Operand::Reg("d3".into()),
    ];
    let sut = sut_word(&ops, true).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_fnmadd_fnmsub_kat_llvm_mc_fnmadd_s31_s31_s31_s31() {
    let want = 0x1f3f7fffu32;
    let mc = llvm_mc_word("fnmadd s31, s31, s31, s31").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [
        Operand::Reg("s31".into()),
        Operand::Reg("s31".into()),
        Operand::Reg("s31".into()),
        Operand::Reg("s31".into()),
    ];
    let sut = sut_word(&ops, false).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_fnmadd_fnmsub_kat_llvm_mc_uppercase() {
    let want = 0x1f220c20u32;
    let mc = llvm_mc_word("fnmadd S0, S1, S2, S3").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [
        Operand::Reg("S0".into()),
        Operand::Reg("S1".into()),
        Operand::Reg("S2".into()),
        Operand::Reg("S3".into()),
    ];
    let sut = sut_word(&ops, false).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_fnmadd_fnmsub_kat_llvm_mc_half_fnmadd_h0_h1_h2_h3() {
    let want = 0x1fe20c20u32;
    let mc = llvm_mc_fp16_word("fnmadd h0, h1, h2, h3").expect("llvm-mc fp16 KAT");
    assert_eq!(mc, want, "llvm-mc fp16 KAT mapping broken");
}

#[test]
fn test_encode_fnmadd_fnmsub_regression_extra_operand() {
    let ops = [
        Operand::Reg("s0".into()),
        Operand::Reg("s1".into()),
        Operand::Reg("s2".into()),
        Operand::Reg("s3".into()),
        Operand::Reg("s0".into()),
    ];
    assert!(
        encode_fnmadd_fnmsub(&ops, false).is_err(),
        "FNMADD must reject a 5th operand"
    );
}

#[test]
fn test_encode_fnmadd_fnmsub_regression_mixed_sd() {
    let ops = [
        Operand::Reg("s0".into()),
        Operand::Reg("d1".into()),
        Operand::Reg("s2".into()),
        Operand::Reg("s3".into()),
    ];
    assert!(
        encode_fnmadd_fnmsub(&ops, false).is_err(),
        "FNMADD must reject mixed S/D operands"
    );
}

#[test]
fn test_encode_fnmadd_fnmsub_regression_half_ftype() {
    let ops = [
        Operand::Reg("h0".into()),
        Operand::Reg("h1".into()),
        Operand::Reg("h2".into()),
        Operand::Reg("h3".into()),
    ];
    let sut = sut_word(&ops, false).expect("H,H,H,H is a valid fp16 FNMADD");
    assert_eq!(
        sut, 0x1fe20c20u32,
        "H registers must use ftype=11 (0x1fe20c20), not ftype=00 S"
    );
}

#[test]
fn test_encode_fnmadd_fnmsub_regression_gpr() {
    let ops = [
        Operand::Reg("x0".into()),
        Operand::Reg("s1".into()),
        Operand::Reg("s2".into()),
        Operand::Reg("s3".into()),
    ];
    assert!(
        encode_fnmadd_fnmsub(&ops, false).is_err(),
        "FNMADD must reject GPR dest"
    );
}

#[test]
fn test_encode_fnmadd_fnmsub_regression_sp() {
    let ops = [
        Operand::Reg("s0".into()),
        Operand::Reg("s1".into()),
        Operand::Reg("s2".into()),
        Operand::Reg("sp".into()),
    ];
    assert!(
        encode_fnmadd_fnmsub(&ops, false).is_err(),
        "FNMADD must reject SP as Ra"
    );
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_fnmadd_fnmsub_diff_valid(
        rd in 0u32..=31,
        rn in 0u32..=31,
        rm in 0u32..=31,
        ra in 0u32..=31,
        is_d in any::<bool>(),
        is_sub in any::<bool>(),
        dest_kind in 0u32..=1,
        n_kind in 0u32..=1,
        m_kind in 0u32..=1,
        a_kind in 0u32..=1,
    ) {
        let dest = fp_spelling(is_d, rd, dest_kind);
        let src_n = fp_spelling(is_d, rn, n_kind);
        let src_m = fp_spelling(is_d, rm, m_kind);
        let src_a = fp_spelling(is_d, ra, a_kind);
        let asm = format!("{} {dest}, {src_n}, {src_m}, {src_a}", mnem(is_sub));
        let ops = [
            Operand::Reg(dest.clone()),
            Operand::Reg(src_n.clone()),
            Operand::Reg(src_m.clone()),
            Operand::Reg(src_a.clone()),
        ];
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {asm}: {e}"));
        let sut = sut_word(&ops, is_sub)
            .unwrap_or_else(|e| panic!("SUT rejected valid {asm}: {e}"));
        prop_assert_eq!(sut, mc, "FNMADD/FNMSUB mismatch for {}", asm);
    }

    #[test]
    fn encode_fnmadd_fnmsub_arm_fields(
        rd in 0u32..=31,
        rn in 0u32..=31,
        rm in 0u32..=31,
        ra in 0u32..=31,
        is_d in any::<bool>(),
        is_sub in any::<bool>(),
    ) {
        let ops = [
            Operand::Reg(fp(is_d, rd)),
            Operand::Reg(fp(is_d, rn)),
            Operand::Reg(fp(is_d, rm)),
            Operand::Reg(fp(is_d, ra)),
        ];
        let w = sut_word(&ops, is_sub).expect("SUT");
        let ftype = if is_d { 0b01u32 } else { 0b00 };
        let o0 = if is_sub { 1u32 } else { 0 };
        let want = (0b00011111u32 << 24)
            | (ftype << 22)
            | (1 << 21)
            | (rm << 16)
            | (o0 << 15)
            | (ra << 10)
            | (rn << 5)
            | rd;
        prop_assert_eq!(w, want, "ARM ARM FP 3-source field layout (o1=1)");
        prop_assert_eq!((w >> 24) & 0xff, 0b00011111, "bits[31:24] M=0 S=0 11111");
        prop_assert_eq!((w >> 22) & 0b11, ftype, "ftype");
        prop_assert_eq!((w >> 21) & 1, 1, "o1/bit21 must be 1 for FNMADD/FNMSUB");
        prop_assert_eq!((w >> 16) & 0x1f, rm, "Rm");
        prop_assert_eq!((w >> 15) & 1, o0, "o0");
        prop_assert_eq!((w >> 10) & 0x1f, ra, "Ra");
        prop_assert_eq!((w >> 5) & 0x1f, rn, "Rn");
        prop_assert_eq!(w & 0x1f, rd, "Rd");
    }

    #[test]
    fn encode_fnmadd_fnmsub_metamorphic_fields(
        rd in 0u32..=30,
        rn in 0u32..=30,
        rm in 0u32..=30,
        ra in 0u32..=30,
        is_d in any::<bool>(),
    ) {
        let ops = |d: u32, n: u32, m: u32, a: u32, sd: bool| {
            [
                Operand::Reg(fp(sd, d)),
                Operand::Reg(fp(sd, n)),
                Operand::Reg(fp(sd, m)),
                Operand::Reg(fp(sd, a)),
            ]
        };
        let w = sut_word(&ops(rd, rn, rm, ra, is_d), false).expect("base FNMADD");
        let w_rd = sut_word(&ops(rd + 1, rn, rm, ra, is_d), false).expect("Rd+1");
        let w_rn = sut_word(&ops(rd, rn + 1, rm, ra, is_d), false).expect("Rn+1");
        let w_ra = sut_word(&ops(rd, rn, rm, ra + 1, is_d), false).expect("Ra+1");
        let w_rm = sut_word(&ops(rd, rn, rm + 1, ra, is_d), false).expect("Rm+1");
        let w_ft = sut_word(&ops(rd, rn, rm, ra, !is_d), false).expect("ftype flip");
        let w_sub = sut_word(&ops(rd, rn, rm, ra, is_d), true).expect("FNMSUB");
        prop_assert_eq!(w_rd, w + 1, "Rd+1 must increment bits[4:0] only");
        prop_assert_eq!(w_rn, w + (1 << 5), "Rn+1 must increment bits[9:5] only");
        prop_assert_eq!(w_ra, w + (1 << 10), "Ra+1 must increment bits[14:10] only");
        prop_assert_eq!(w_rm, w + (1 << 16), "Rm+1 must increment bits[20:16] only");
        prop_assert_eq!(w_ft ^ w, 1u32 << 22, "S vs D must flip only ftype bit 22");
        prop_assert_eq!(w_sub ^ w, 1u32 << 15, "FNMADD XOR FNMSUB must be o0 bit 15");
    }

    #[test]
    fn encode_fnmadd_fnmsub_neg_arity(
        len in 0usize..=3,
        n in 0u32..=31,
        is_sub in any::<bool>(),
    ) {
        let mut ops = Vec::new();
        for _ in 0..len {
            ops.push(Operand::Reg(fp(false, n)));
        }
        prop_assert!(
            encode_fnmadd_fnmsub(&ops, is_sub).is_err(),
            "FNMADD/FNMSUB with {} operands must Err (llvm-mc: too few operands)",
            len
        );
    }

    #[test]
    fn encode_fnmadd_fnmsub_neg_extra_operand(
        rd in 0u32..=31,
        rn in 0u32..=31,
        rm in 0u32..=31,
        ra in 0u32..=31,
        is_d in any::<bool>(),
        is_sub in any::<bool>(),
        extra in extra_operand(),
    ) {
        let ops = vec![
            Operand::Reg(fp(is_d, rd)),
            Operand::Reg(fp(is_d, rn)),
            Operand::Reg(fp(is_d, rm)),
            Operand::Reg(fp(is_d, ra)),
            extra,
        ];
        prop_assert!(
            encode_fnmadd_fnmsub(&ops, is_sub).is_err(),
            "FNMADD/FNMSUB has no 5th operand; extra must Err"
        );
    }

    #[test]
    fn encode_fnmadd_fnmsub_neg_wrong_types(
        (dest, src_n, src_m, src_a) in wrong_type_quad(),
        is_sub in any::<bool>(),
    ) {
        let ops = [
            Operand::Reg(dest.clone()),
            Operand::Reg(src_n.clone()),
            Operand::Reg(src_m.clone()),
            Operand::Reg(src_a.clone()),
        ];
        prop_assert!(
            encode_fnmadd_fnmsub(&ops, is_sub).is_err(),
            "FNMADD/FNMSUB requires matching Sd/Dd/Hd quadruples; dest={} n={} m={} a={} must Err",
            dest,
            src_n,
            src_m,
            src_a
        );
    }

    #[test]
    fn encode_fnmadd_fnmsub_diff_half(
        rd in 0u32..=31,
        rn in 0u32..=31,
        rm in 0u32..=31,
        ra in 0u32..=31,
        is_sub in any::<bool>(),
    ) {
        let dest = format!("h{rd}");
        let src_n = format!("h{rn}");
        let src_m = format!("h{rm}");
        let src_a = format!("h{ra}");
        let asm = format!("{} {dest}, {src_n}, {src_m}, {src_a}", mnem(is_sub));
        let ops = [
            Operand::Reg(dest.clone()),
            Operand::Reg(src_n),
            Operand::Reg(src_m),
            Operand::Reg(src_a),
        ];
        let mc = llvm_mc_fp16_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc fp16 rejected valid {asm}: {e}"));
        let sut = sut_word(&ops, is_sub)
            .unwrap_or_else(|e| panic!("SUT rejected valid fp16 {asm}: {e}"));
        prop_assert_eq!(sut, mc, "FNMADD/FNMSUB half-precision mismatch for {}", asm);
    }

    #[test]
    fn encode_fnmadd_fnmsub_neg_nonreg(
        which in 0u32..=3,
        kind in 0u32..=5,
        is_sub in any::<bool>(),
    ) {
        let bad = match kind {
            0 => Operand::Imm(0),
            1 => Operand::Symbol("foo".into()),
            2 => Operand::Label("1f".into()),
            3 => Operand::Mem {
                base: "x0".into(),
                offset: 0,
            },
            4 => Operand::Cond("eq".into()),
            _ => Operand::Shift {
                kind: "lsl".into(),
                amount: 0,
            },
        };
        let mut ops = vec![
            Operand::Reg("s0".into()),
            Operand::Reg("s1".into()),
            Operand::Reg("s2".into()),
            Operand::Reg("s3".into()),
        ];
        ops[which as usize] = bad;
        prop_assert!(
            encode_fnmadd_fnmsub(&ops, is_sub).is_err(),
            "non-register at slot {} must Err",
            which
        );
    }

    #[test]
    fn encode_fnmadd_fnmsub_neg_invalid_name(
        which in 0u32..=3,
        name in prop::sample::select(vec![
            "foo", "s32", "d32", "h32", "x32", "r0", "s", "d", "",
        ]),
        is_sub in any::<bool>(),
    ) {
        let mut ops = vec![
            Operand::Reg("s0".into()),
            Operand::Reg("s1".into()),
            Operand::Reg("s2".into()),
            Operand::Reg("s3".into()),
        ];
        ops[which as usize] = Operand::Reg(name.to_string());
        prop_assert!(
            encode_fnmadd_fnmsub(&ops, is_sub).is_err(),
            "invalid name {:?} at slot {} must Err",
            name,
            which
        );
    }
}
