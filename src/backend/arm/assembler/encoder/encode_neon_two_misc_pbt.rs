// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md:225 NEON two-misc lists abs, neg, cls, clz, rev16, rev32, sqabs, sqneg;
//   encoder/mod.rs:1-7 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:324-326 "neg" RegArrangement => encode_neon_two_misc(operands, 1, 0b01011);
//   encoder/mod.rs:616 "abs" => encode_neon_two_misc(operands, 0, 0b01011);
//   encoder/mod.rs:618-628 cls/clz/rev16/rev32; mod.rs:630-633 saddlp/uaddlp/sadalp/uadalp;
//   encoder/mod.rs:637-645 sqabs/sqneg RegArrangement;
//   neon.rs:1405 Encode NEON two-reg misc: ABS, NEG, CLS, CLZ, etc.;
//   neon.rs:1406 Format: 0 Q U 01110 size 10000 opcode 10 Rn Rd;
//   ARM ARM Advanced SIMD two-register miscellaneous
//     (matching-T ABS/NEG/SQABS/SQNEG T in {8B,16B,4H,8H,2S,4S,2D};
//      CLS/CLZ T in {8B,16B,4H,8H,2S,4S}; REV16 T in {8B,16B};
//      REV32 T in {8B,16B,4H,8H};
//      SADDLP/UADDLP/SADALP/UADALP dest Ta from source Tb, size from source esize).
// Stronger considered:
//   - State machine: rejected — encode_neon_two_misc is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree two-misc decoder
//   - Differential vs encode_neon_float_two_misc: rejected — same-job gate (float UCVTF/FCVTZS)
//   - Differential vs encode_neon_two_misc_narrow: rejected — XTN/SQXTN narrow
//   - Differential vs encode_cnt / encode_neon_not / encode_neon_rev64 / encode_neon_rbit:
//     rejected — dedicated encoders, different opcodes (same-job gate)
//   - Differential vs encode_neon_scalar_two_misc: rejected — scalar vs vector
// Weaker available: algebraic.metamorphic (Rd/Rn/U/Q fields), algebraic.invariant (word layout),
//   negative_error (arity / extra / reserved T / T mismatch / GPR dest)
// Differential: candidate=encode_neon_two_misc, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[RegArrangement(Vd,T), RegArrangement(Vn,T)] <-> `{mnem} Vd.T, Vn.T`
//     and pairwise [RegArrangement(Vd,Ta), RegArrangement(Vn,Tb)] <-> `{mnem} Vd.Ta, Vn.Tb`.
//   Argument mapping: ARM (U, opcode) for the mnemonic, operands passed through.

use super::encode_neon_two_misc;
use super::EncodeResult;
use crate::backend::arm::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;

const OPC_ABS: u32 = 0b01011;
const OPC_CLS: u32 = 0b00100;
const OPC_REV16: u32 = 0b00001;
const OPC_REV32: u32 = 0b00000;
const OPC_SQABS: u32 = 0b00111;
const OPC_SADDLP: u32 = 0b00010;
const OPC_SADALP: u32 = 0b00110;

fn cfg() -> ProptestConfig {
    ProptestConfig::with_cases(CASES)
}

fn arr(reg: u32, t: &str) -> Operand {
    Operand::RegArrangement {
        reg: format!("v{}", reg),
        arrangement: t.to_string(),
    }
}

fn gpr(prefix: &str, n: u32) -> Operand {
    Operand::Reg(format!("{}{}", prefix, n))
}

fn sut_word(ops: &[Operand], u_bit: u32, opcode: u32) -> Result<u32, String> {
    match encode_neon_two_misc(ops, u_bit, opcode)? {
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

/// Matching-T two-misc: (mnemonic, U, opcode, T) with ARM-legal T for that opcode.
fn matching_case() -> impl Strategy<Value = (&'static str, u32, u32, &'static str)> {
    prop_oneof![
        (
            Just("abs"),
            Just(0u32),
            Just(OPC_ABS),
            prop::sample::select(vec!["8b", "16b", "4h", "8h", "2s", "4s", "2d"])
        ),
        (
            Just("neg"),
            Just(1u32),
            Just(OPC_ABS),
            prop::sample::select(vec!["8b", "16b", "4h", "8h", "2s", "4s", "2d"])
        ),
        (
            Just("cls"),
            Just(0u32),
            Just(OPC_CLS),
            prop::sample::select(vec!["8b", "16b", "4h", "8h", "2s", "4s"])
        ),
        (
            Just("clz"),
            Just(1u32),
            Just(OPC_CLS),
            prop::sample::select(vec!["8b", "16b", "4h", "8h", "2s", "4s"])
        ),
        (
            Just("rev16"),
            Just(0u32),
            Just(OPC_REV16),
            prop::sample::select(vec!["8b", "16b"])
        ),
        (
            Just("rev32"),
            Just(1u32),
            Just(OPC_REV32),
            prop::sample::select(vec!["8b", "16b", "4h", "8h"])
        ),
        (
            Just("sqabs"),
            Just(0u32),
            Just(OPC_SQABS),
            prop::sample::select(vec!["8b", "16b", "4h", "8h", "2s", "4s", "2d"])
        ),
        (
            Just("sqneg"),
            Just(1u32),
            Just(OPC_SQABS),
            prop::sample::select(vec!["8b", "16b", "4h", "8h", "2s", "4s", "2d"])
        ),
    ]
}

fn abs_t() -> impl Strategy<Value = &'static str> {
    prop::sample::select(vec!["8b", "16b", "4h", "8h", "2s", "4s", "2d"])
}

/// Pairwise-long: (mnemonic, U, opcode) × (Tb, Ta).
fn pairwise_mnem() -> impl Strategy<Value = (&'static str, u32, u32)> {
    prop::sample::select(vec![
        ("saddlp", 0u32, OPC_SADDLP),
        ("uaddlp", 1u32, OPC_SADDLP),
        ("sadalp", 0u32, OPC_SADALP),
        ("uadalp", 1u32, OPC_SADALP),
    ])
}

fn pairwise_tb_ta() -> impl Strategy<Value = (&'static str, &'static str)> {
    prop::sample::select(vec![
        ("8b", "4h"),
        ("16b", "8h"),
        ("4h", "2s"),
        ("8h", "4s"),
        ("2s", "1d"),
        ("4s", "2d"),
    ])
}

/// Opcode-illegal T that llvm-mc rejects and neon_arr_to_q_size still maps.
fn reserved_case() -> impl Strategy<Value = (&'static str, u32, u32, &'static str)> {
    prop::sample::select(vec![
        ("abs", 0u32, OPC_ABS, "1d"),
        ("neg", 1u32, OPC_ABS, "1d"),
        ("cls", 0u32, OPC_CLS, "2d"),
        ("clz", 1u32, OPC_CLS, "2d"),
        ("rev16", 0u32, OPC_REV16, "4h"),
        ("rev16", 0u32, OPC_REV16, "8h"),
        ("rev16", 0u32, OPC_REV16, "2s"),
        ("rev16", 0u32, OPC_REV16, "4s"),
        ("rev16", 0u32, OPC_REV16, "2d"),
        ("rev32", 1u32, OPC_REV32, "2s"),
        ("rev32", 1u32, OPC_REV32, "4s"),
        ("rev32", 1u32, OPC_REV32, "2d"),
        ("sqabs", 0u32, OPC_SQABS, "1d"),
        ("sqneg", 1u32, OPC_SQABS, "1d"),
    ])
}

/// ARM Advanced SIMD two-register miscellaneous packing from dest T
/// (correct for matching-T forms; pairwise uses source esize instead).
fn q_size(t: &str) -> (u32, u32) {
    match t {
        "8b" => (0, 0b00),
        "16b" => (1, 0b00),
        "4h" => (0, 0b01),
        "8h" => (1, 0b01),
        "2s" => (0, 0b10),
        "4s" => (1, 0b10),
        "1d" => (0, 0b11),
        "2d" => (1, 0b11),
        _ => (0, 0),
    }
}

fn arm_two_misc_word(rd: u32, rn: u32, u_bit: u32, opcode: u32, t: &str) -> u32 {
    let (q, size) = q_size(t);
    (q << 30)
        | (u_bit << 29)
        | (0b01110 << 24)
        | (size << 22)
        | (0b10000 << 17)
        | (opcode << 12)
        | (0b10 << 10)
        | (rn << 5)
        | rd
}

// Reference KAT gate (must pass before PBT): llvm-mc mapping + matching-T SUT.
#[test]
fn encode_neon_two_misc_kat_llvm_mc_abs_v0_8b_v1_8b() {
    let want = 0x0e20b820u32;
    let mc = llvm_mc_word("abs v0.8b, v1.8b").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[arr(0, "8b"), arr(1, "8b")], 0, OPC_ABS).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_neon_two_misc_kat_llvm_mc_abs_v0_16b_v1_16b() {
    let want = 0x4e20b820u32;
    let mc = llvm_mc_word("abs v0.16b, v1.16b").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[arr(0, "16b"), arr(1, "16b")], 0, OPC_ABS).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_neon_two_misc_kat_llvm_mc_abs_v0_2d_v1_2d() {
    let want = 0x4ee0b820u32;
    let mc = llvm_mc_word("abs v0.2d, v1.2d").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[arr(0, "2d"), arr(1, "2d")], 0, OPC_ABS).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_neon_two_misc_kat_llvm_mc_neg_v0_8b_v1_8b() {
    let want = 0x2e20b820u32;
    let mc = llvm_mc_word("neg v0.8b, v1.8b").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[arr(0, "8b"), arr(1, "8b")], 1, OPC_ABS).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_neon_two_misc_kat_llvm_mc_cls_v0_8b_v1_8b() {
    let want = 0x0e204820u32;
    let mc = llvm_mc_word("cls v0.8b, v1.8b").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[arr(0, "8b"), arr(1, "8b")], 0, OPC_CLS).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_neon_two_misc_kat_llvm_mc_clz_v0_8b_v1_8b() {
    let want = 0x2e204820u32;
    let mc = llvm_mc_word("clz v0.8b, v1.8b").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[arr(0, "8b"), arr(1, "8b")], 1, OPC_CLS).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_neon_two_misc_kat_llvm_mc_rev16_v0_8b_v1_8b() {
    let want = 0x0e201820u32;
    let mc = llvm_mc_word("rev16 v0.8b, v1.8b").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[arr(0, "8b"), arr(1, "8b")], 0, OPC_REV16).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_neon_two_misc_kat_llvm_mc_sqabs_v0_8b_v1_8b() {
    let want = 0x0e207820u32;
    let mc = llvm_mc_word("sqabs v0.8b, v1.8b").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[arr(0, "8b"), arr(1, "8b")], 0, OPC_SQABS).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_neon_two_misc_kat_llvm_mc_sqneg_v0_2d_v1_2d() {
    let want = 0x6ee07820u32;
    let mc = llvm_mc_word("sqneg v0.2d, v1.2d").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[arr(0, "2d"), arr(1, "2d")], 1, OPC_SQABS).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_neon_two_misc_kat_llvm_mc_neg_v31_2d_v31_2d() {
    let want = 0x6ee0bbffu32;
    let mc = llvm_mc_word("neg v31.2d, v31.2d").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[arr(31, "2d"), arr(31, "2d")], 1, OPC_ABS).expect("SUT KAT");
    assert_eq!(sut, want);
}

/// llvm-mc mapping for pairwise-long (SUT compared in the PBT property, not here).
#[test]
fn encode_neon_two_misc_kat_llvm_mc_saddlp_mapping() {
    let want = 0x0e202820u32;
    let mc = llvm_mc_word("saddlp v0.4h, v1.8b").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_two_misc_diff_llvm_mc(
        rd in reg_num(),
        rn in reg_num(),
        case in matching_case(),
    ) {
        let (mnem, u_bit, opcode, t) = case;
        let asm = format!("{mnem} v{rd}.{t}, v{rn}.{t}");
        let mc = llvm_mc_word(&asm).expect(&asm);
        let sut = sut_word(&[arr(rd, t), arr(rn, t)], u_bit, opcode).expect(&asm);
        prop_assert_eq!(sut, mc, "SUT vs llvm-mc for {}", asm);
    }

    #[test]
    fn encode_neon_two_misc_diff_llvm_mc_pairwise(
        rd in reg_num(),
        rn in reg_num(),
        mnem_case in pairwise_mnem(),
        pair in pairwise_tb_ta(),
    ) {
        let (mnem, u_bit, opcode) = mnem_case;
        let (tb, ta) = pair;
        let asm = format!("{mnem} v{rd}.{ta}, v{rn}.{tb}");
        let mc = llvm_mc_word(&asm).expect(&asm);
        let sut = sut_word(&[arr(rd, ta), arr(rn, tb)], u_bit, opcode).expect(&asm);
        prop_assert_eq!(sut, mc, "SUT vs llvm-mc for {}", asm);
    }

    #[test]
    fn encode_neon_two_misc_meta_rd_rn_u_q(
        rd1 in reg_num(),
        rd2 in reg_num(),
        rn1 in reg_num(),
        rn2 in reg_num(),
    ) {
        let w11 = sut_word(&[arr(rd1, "8b"), arr(rn1, "8b")], 0, OPC_ABS).expect("w11");
        let w21 = sut_word(&[arr(rd2, "8b"), arr(rn1, "8b")], 0, OPC_ABS).expect("w21");
        let w12 = sut_word(&[arr(rd1, "8b"), arr(rn2, "8b")], 0, OPC_ABS).expect("w12");
        let w_u = sut_word(&[arr(rd1, "8b"), arr(rn1, "8b")], 1, OPC_ABS).expect("w_u");
        let w_q = sut_word(&[arr(rd1, "16b"), arr(rn1, "16b")], 0, OPC_ABS).expect("w_q");
        prop_assert_eq!(
            (w11 ^ w21) & !0x1Fu32,
            0,
            "changing only Rd must differ only in bits[4:0]"
        );
        prop_assert_eq!(w11 & 0x1F, rd1);
        prop_assert_eq!(w21 & 0x1F, rd2);
        prop_assert_eq!(
            (w11 ^ w12) & !(0x1Fu32 << 5),
            0,
            "changing only Rn must differ only in bits[9:5]"
        );
        prop_assert_eq!((w11 >> 5) & 0x1F, rn1);
        prop_assert_eq!((w12 >> 5) & 0x1F, rn2);
        prop_assert_eq!(w11 ^ w_u, 1u32 << 29, "U bit is bit 29");
        prop_assert_eq!(w11 ^ w_q, 1u32 << 30, "8b vs 16b differs only in Q bit 30");
    }

    #[test]
    fn encode_neon_two_misc_inv_layout(
        rd in reg_num(),
        rn in reg_num(),
        case in matching_case(),
    ) {
        let (_mnem, u_bit, opcode, t) = case;
        let w = sut_word(&[arr(rd, t), arr(rn, t)], u_bit, opcode).expect("layout");
        prop_assert_eq!(w, arm_two_misc_word(rd, rn, u_bit, opcode, t), "ARM two-misc layout");
        prop_assert_eq!(w >> 31, 0);
        let (q, size) = q_size(t);
        prop_assert_eq!((w >> 30) & 1, q);
        prop_assert_eq!((w >> 29) & 1, u_bit);
        prop_assert_eq!((w >> 24) & 0x1F, 0b01110);
        prop_assert_eq!((w >> 22) & 0b11, size);
        prop_assert_eq!((w >> 17) & 0x1F, 0b10000);
        prop_assert_eq!((w >> 12) & 0x1F, opcode);
        prop_assert_eq!((w >> 10) & 0b11, 0b10);
        prop_assert_eq!((w >> 5) & 0x1F, rn);
        prop_assert_eq!(w & 0x1F, rd);
    }

    #[test]
    fn encode_neon_two_misc_neg_arity(
        n in 0usize..=1,
        rd in reg_num(),
        t in abs_t(),
    ) {
        let ops: Vec<Operand> = match n {
            0 => vec![],
            _ => vec![arr(rd, t)],
        };
        prop_assert!(
            encode_neon_two_misc(&ops, 0, OPC_ABS).is_err(),
            "arity {} must Err (two-misc requires Vd.T, Vn.T)",
            n
        );
    }

    #[test]
    fn encode_neon_two_misc_neg_extra(
        rd in reg_num(),
        rn in reg_num(),
        extra in reg_num(),
        t in abs_t(),
    ) {
        let asm = format!("abs v{rd}.{t}, v{rn}.{t}, v{extra}.{t}");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted extra {}",
            asm
        );
        let ops = [arr(rd, t), arr(rn, t), arr(extra, t)];
        prop_assert!(
            encode_neon_two_misc(&ops, 0, OPC_ABS).is_err(),
            "extra operand must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_two_misc_neg_mismatch(
        rd in reg_num(),
        rn in reg_num(),
        td in abs_t(),
        tn in abs_t(),
    ) {
        prop_assume!(td != tn);
        let asm = format!("abs v{rd}.{td}, v{rn}.{tn}");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let ops = [arr(rd, td), arr(rn, tn)];
        prop_assert!(
            encode_neon_two_misc(&ops, 0, OPC_ABS).is_err(),
            "mismatched T must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_two_misc_neg_reserved_t(
        rd in reg_num(),
        rn in reg_num(),
        case in reserved_case(),
    ) {
        let (mnem, u_bit, opcode, t) = case;
        let asm = format!("{mnem} v{rd}.{t}, v{rn}.{t}");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let ops = [arr(rd, t), arr(rn, t)];
        prop_assert!(
            encode_neon_two_misc(&ops, u_bit, opcode).is_err(),
            "reserved T must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_two_misc_diff_alt_spellings(
        rd in reg_num(),
        rn in reg_num(),
        case in matching_case(),
    ) {
        let (mnem, u_bit, opcode, t) = case;
        let asm = format!("{mnem} V{rd}.{t}, V{rn}.{t}");
        let mc = llvm_mc_word(&asm).expect(&asm);
        let ops = [
            Operand::RegArrangement {
                reg: format!("V{rd}"),
                arrangement: t.to_string(),
            },
            Operand::RegArrangement {
                reg: format!("V{rn}"),
                arrangement: t.to_string(),
            },
        ];
        let sut = sut_word(&ops, u_bit, opcode).expect(&asm);
        prop_assert_eq!(sut, mc, "uppercase V prefix must match llvm-mc {}", asm);
    }

    #[test]
    fn encode_neon_two_misc_neg_nonreg(
        rd in reg_num(),
        rn in reg_num(),
        t in abs_t(),
        kind in 0u8..=5,
    ) {
        let (ops, asm) = match kind {
            0 => (
                vec![Operand::Imm(0), arr(rn, t)],
                format!("abs #0, v{rn}.{t}"),
            ),
            1 => (
                vec![arr(rd, t), Operand::Imm(1)],
                format!("abs v{rd}.{t}, #1"),
            ),
            2 => (
                vec![
                    Operand::Mem {
                        base: "x0".into(),
                        offset: 0,
                    },
                    arr(rn, t),
                ],
                format!("abs [x0], v{rn}.{t}"),
            ),
            3 => (
                vec![arr(rd, t), Operand::Label("L".into())],
                format!("abs v{rd}.{t}, L"),
            ),
            4 => (
                vec![gpr("x", rd), arr(rn, t)],
                format!("abs x{rd}, v{rn}.{t}"),
            ),
            _ => (
                vec![gpr("v", rd), gpr("v", rn)],
                format!("abs v{rd}, v{rn}"),
            ),
        };
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        prop_assert!(
            encode_neon_two_misc(&ops, 0, OPC_ABS).is_err(),
            "non-RegArrangement / GPR / bare V / Imm / Mem / Label must Err (llvm-mc rejects {})",
            asm
        );
    }
}

#[test]
fn test_encode_neon_two_misc_regression_pairwise_size_from_dest() {
    // saddlp v0.4h, v0.8b — llvm-mc 0x0e202800 (size from source 8b).
    // SUT uses dest 4h → size=01 → 0x0e602800 (encoding of saddlp v0.2s, v0.4h).
    let want = llvm_mc_word("saddlp v0.4h, v0.8b").expect("llvm-mc");
    assert_eq!(want, 0x0e202800);
    let sut = sut_word(&[arr(0, "4h"), arr(0, "8b")], 0, OPC_SADDLP)
        .expect("SUT encodes pairwise");
    assert_eq!(
        sut, want,
        "saddlp Vd.4h, Vn.8b must use source size=00, not dest size=01"
    );
}

#[test]
fn test_encode_neon_two_misc_regression_pairwise_2d() {
    let want = llvm_mc_word("saddlp v0.2d, v1.4s").expect("llvm-mc");
    assert_eq!(want, 0x4ea02820);
    let sut = sut_word(&[arr(0, "2d"), arr(1, "4s")], 0, OPC_SADDLP)
        .expect("SUT encodes pairwise");
    assert_eq!(
        sut, want,
        "saddlp Vd.2d, Vn.4s must use source size=10, not dest size=11"
    );
}

#[test]
fn test_encode_neon_two_misc_regression_extra_operand() {
    let ops = [arr(0, "8b"), arr(0, "8b"), arr(0, "8b")];
    assert!(
        encode_neon_two_misc(&ops, 0, OPC_ABS).is_err(),
        "abs v0.8b, v0.8b, v0.8b must Err (llvm-mc/gas reject a third operand)"
    );
}

#[test]
fn test_encode_neon_two_misc_regression_mismatch_t() {
    let ops = [arr(0, "8b"), arr(0, "16b")];
    assert!(
        encode_neon_two_misc(&ops, 0, OPC_ABS).is_err(),
        "abs v0.8b, v0.16b must Err (llvm-mc/gas: operand mismatch)"
    );
}

#[test]
fn test_encode_neon_two_misc_regression_reserved_1d() {
    let ops = [arr(0, "1d"), arr(0, "1d")];
    assert!(
        encode_neon_two_misc(&ops, 0, OPC_ABS).is_err(),
        "abs v0.1d, v0.1d must Err (vector ABS 1D is reserved; llvm-mc/gas reject)"
    );
}

#[test]
fn test_encode_neon_two_misc_regression_cls_2d() {
    let ops = [arr(0, "2d"), arr(0, "2d")];
    assert!(
        encode_neon_two_misc(&ops, 0, OPC_CLS).is_err(),
        "cls v0.2d, v0.2d must Err (CLS size=11 reserved; llvm-mc/gas reject)"
    );
}
