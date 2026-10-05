// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md:225 NEON three-same lists bsl;
//   encoder/mod.rs:1-7 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:699 "bsl" => encode_neon_bsl;
//   neon.rs:734 Encode NEON BSL (bitwise select): BSL Vd.T, Vn.T, Vm.T;
//   neon.rs:745 BSL Vd.T, Vn.T, Vm.T: 0 Q 1 01110 01 1 Rm 000111 Rn Rd;
//   ARM ARM Advanced SIMD three-same (BSL); T in {8B,16B}.
// Stronger considered:
//   - State machine: rejected — encode_neon_bsl is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree BSL decoder
//   - Differential vs encode_neon_bic / encode_neon_bitwise_insert: rejected — same-job gate
//     (BIC/BIT/BIF are different three-same opcodes, not interchangeable with BSL)
// Weaker available: algebraic.metamorphic (Rd/Rn/Rm fields), algebraic.invariant (word layout),
//   negative_error (arity / extra / invalid T / mismatched T / GPR dest)
// Differential: candidate=encode_neon_bsl, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[RegArrangement(Vd,T), RegArrangement(Vn,T), RegArrangement(Vm,T)] <-> `bsl Vd.T, Vn.T, Vm.T`

use super::encode_neon_bsl;
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

fn arr(reg: u32, t: &str) -> Operand {
    Operand::RegArrangement {
        reg: format!("v{}", reg),
        arrangement: t.to_string(),
    }
}

fn gpr(prefix: &str, n: u32) -> Operand {
    Operand::Reg(format!("{}{}", prefix, n))
}

fn sut_word(ops: &[Operand]) -> Result<u32, String> {
    match encode_neon_bsl(ops)? {
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

fn valid_t() -> impl Strategy<Value = &'static str> {
    prop::sample::select(vec!["8b", "16b"])
}

fn invalid_t() -> impl Strategy<Value = &'static str> {
    prop::sample::select(vec!["4h", "8h", "2s", "4s", "2d", "1d", "4b", "8d", "2h", "1s"])
}

/// ARM Advanced SIMD three-same BSL:
/// 0 Q 1 01110 size=01 1 Rm 000111 Rn Rd = 0x2e601c00 | (Q<<30) | (Rm<<16) | (Rn<<5) | Rd
fn arm_bsl_word(rd: u32, rn: u32, rm: u32, t: &str) -> u32 {
    let q = if t == "16b" { 1u32 } else { 0 };
    0x2e601c00 | (q << 30) | (rm << 16) | (rn << 5) | rd
}

// Reference KAT gate (must pass before PBT).
#[test]
fn encode_neon_bsl_kat_llvm_mc_v0_8b_v1_v2() {
    let want = 0x2e621c20u32;
    let mc = llvm_mc_word("bsl v0.8b, v1.8b, v2.8b").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [arr(0, "8b"), arr(1, "8b"), arr(2, "8b")];
    let sut = sut_word(&ops).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_neon_bsl_kat_llvm_mc_v0_16b_v1_v2() {
    let want = 0x6e621c20u32;
    let mc = llvm_mc_word("bsl v0.16b, v1.16b, v2.16b").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [arr(0, "16b"), arr(1, "16b"), arr(2, "16b")];
    let sut = sut_word(&ops).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_neon_bsl_kat_llvm_mc_v31_8b_v31_v31() {
    let want = 0x2e7f1fffu32;
    let mc = llvm_mc_word("bsl v31.8b, v31.8b, v31.8b").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [arr(31, "8b"), arr(31, "8b"), arr(31, "8b")];
    let sut = sut_word(&ops).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_neon_bsl_kat_llvm_mc_v31_16b_v0_v1() {
    let want = 0x6e611c1fu32;
    let mc = llvm_mc_word("bsl v31.16b, v0.16b, v1.16b").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [arr(31, "16b"), arr(0, "16b"), arr(1, "16b")];
    let sut = sut_word(&ops).expect("SUT KAT");
    assert_eq!(sut, want);
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_bsl_diff_llvm_mc(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        t in valid_t(),
    ) {
        let asm = format!("bsl v{rd}.{t}, v{rn}.{t}, v{rm}.{t}");
        let mc = llvm_mc_word(&asm).expect(&asm);
        let sut = sut_word(&[arr(rd, t), arr(rn, t), arr(rm, t)]).expect(&asm);
        prop_assert_eq!(sut, mc, "SUT vs llvm-mc for {}", asm);
    }

    #[test]
    fn encode_neon_bsl_meta_rd_rn_rm(
        rd1 in reg_num(),
        rd2 in reg_num(),
        rn1 in reg_num(),
        rn2 in reg_num(),
        rm1 in reg_num(),
        rm2 in reg_num(),
        t in valid_t(),
    ) {
        let w111 = sut_word(&[arr(rd1, t), arr(rn1, t), arr(rm1, t)]).expect("w111");
        let w211 = sut_word(&[arr(rd2, t), arr(rn1, t), arr(rm1, t)]).expect("w211");
        let w121 = sut_word(&[arr(rd1, t), arr(rn2, t), arr(rm1, t)]).expect("w121");
        let w112 = sut_word(&[arr(rd1, t), arr(rn1, t), arr(rm2, t)]).expect("w112");
        prop_assert_eq!(
            (w111 ^ w211) & !0x1Fu32,
            0,
            "changing only Rd must differ only in bits[4:0]"
        );
        prop_assert_eq!(w111 & 0x1F, rd1);
        prop_assert_eq!(w211 & 0x1F, rd2);
        prop_assert_eq!(
            (w111 ^ w121) & !(0x1Fu32 << 5),
            0,
            "changing only Rn must differ only in bits[9:5]"
        );
        prop_assert_eq!((w111 >> 5) & 0x1F, rn1);
        prop_assert_eq!((w121 >> 5) & 0x1F, rn2);
        prop_assert_eq!(
            (w111 ^ w112) & !(0x1Fu32 << 16),
            0,
            "changing only Rm must differ only in bits[20:16]"
        );
        prop_assert_eq!((w111 >> 16) & 0x1F, rm1);
        prop_assert_eq!((w112 >> 16) & 0x1F, rm2);
    }

    #[test]
    fn encode_neon_bsl_inv_layout(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        t in valid_t(),
    ) {
        let w = sut_word(&[arr(rd, t), arr(rn, t), arr(rm, t)]).expect("layout");
        prop_assert_eq!(w, arm_bsl_word(rd, rn, rm, t), "ARM BSL layout");
        prop_assert_eq!(w >> 31, 0);
        let q = if t == "16b" { 1u32 } else { 0 };
        prop_assert_eq!((w >> 30) & 1, q);
        prop_assert_eq!((w >> 24) & 0x3F, 0b101110);
        prop_assert_eq!((w >> 22) & 0b11, 0b01);
        prop_assert_eq!((w >> 21) & 1, 1);
        prop_assert_eq!((w >> 16) & 0x1F, rm);
        prop_assert_eq!((w >> 10) & 0x3F, 0b000111);
        prop_assert_eq!((w >> 5) & 0x1F, rn);
        prop_assert_eq!(w & 0x1F, rd);
        let w8 = sut_word(&[arr(rd, "8b"), arr(rn, "8b"), arr(rm, "8b")]).expect("q8");
        let w16 = sut_word(&[arr(rd, "16b"), arr(rn, "16b"), arr(rm, "16b")]).expect("q16");
        prop_assert_eq!(w8 ^ w16, 1u32 << 30, "8b vs 16b must differ only in Q");
    }

    #[test]
    fn encode_neon_bsl_neg_arity(n in 0usize..=2, rd in reg_num(), t in valid_t()) {
        let ops: Vec<Operand> = match n {
            0 => vec![],
            1 => vec![arr(rd, t)],
            _ => vec![arr(rd, t), arr(rd, t)],
        };
        prop_assert!(
            encode_neon_bsl(&ops).is_err(),
            "arity {} must Err (bsl requires 3 operands)",
            n
        );
    }

    #[test]
    fn encode_neon_bsl_neg_extra(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        extra in reg_num(),
        t in valid_t(),
    ) {
        let asm = format!("bsl v{rd}.{t}, v{rn}.{t}, v{rm}.{t}, v{extra}.{t}");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted extra {}",
            asm
        );
        let ops = [arr(rd, t), arr(rn, t), arr(rm, t), arr(extra, t)];
        prop_assert!(
            encode_neon_bsl(&ops).is_err(),
            "extra operand must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_bsl_neg_invalid_t(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        t in invalid_t(),
    ) {
        let asm = format!("bsl v{rd}.{t}, v{rn}.{t}, v{rm}.{t}");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let ops = [arr(rd, t), arr(rn, t), arr(rm, t)];
        prop_assert!(
            encode_neon_bsl(&ops).is_err(),
            "invalid T must Err (only .8b/.16b; llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_bsl_neg_mismatch_t(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        td in valid_t(),
        tn in valid_t(),
        tm in valid_t(),
    ) {
        prop_assume!(!(td == tn && tn == tm));
        let asm = format!("bsl v{rd}.{td}, v{rn}.{tn}, v{rm}.{tm}");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let ops = [arr(rd, td), arr(rn, tn), arr(rm, tm)];
        prop_assert!(
            encode_neon_bsl(&ops).is_err(),
            "mismatched T must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_bsl_neg_gpr_bare_sp(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        t in valid_t(),
        kind in 0u8..=6,
    ) {
        let (ops, asm) = match kind {
            0 => {
                let asm = format!("bsl x{rd}, x{rn}, x{rm}");
                let ops = vec![gpr("x", rd), gpr("x", rn), gpr("x", rm)];
                (ops, asm)
            }
            1 => {
                let asm = format!("bsl w{rd}, v{rn}.{t}, v{rm}.{t}");
                let ops = vec![gpr("w", rd), arr(rn, t), arr(rm, t)];
                (ops, asm)
            }
            2 => {
                let asm = "bsl sp, v0.8b, v1.8b".to_string();
                let ops = vec![Operand::Reg("sp".into()), arr(0, "8b"), arr(1, "8b")];
                (ops, asm)
            }
            3 => {
                let asm = format!("bsl v{rd}, v{rn}, v{rm}");
                let ops = vec![gpr("v", rd), gpr("v", rn), gpr("v", rm)];
                (ops, asm)
            }
            4 => {
                let asm = format!("bsl d{rd}, d{rn}, d{rm}");
                let ops = vec![gpr("d", rd), gpr("d", rn), gpr("d", rm)];
                (ops, asm)
            }
            5 => {
                let asm = format!("bsl s{rd}, v{rn}.{t}, v{rm}.{t}");
                let ops = vec![gpr("s", rd), arr(rn, t), arr(rm, t)];
                (ops, asm)
            }
            _ => {
                let asm = format!("bsl q{rd}, v{rn}.{t}, v{rm}.{t}");
                let ops = vec![gpr("q", rd), arr(rn, t), arr(rm, t)];
                (ops, asm)
            }
        };
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        prop_assert!(
            encode_neon_bsl(&ops).is_err(),
            "non-arranged NEON / GPR / SP / FP must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_bsl_diff_alt_spellings(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        t in valid_t(),
    ) {
        let asm = format!("bsl V{rd}.{t}, V{rn}.{t}, V{rm}.{t}");
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
            Operand::RegArrangement {
                reg: format!("V{rm}"),
                arrangement: t.to_string(),
            },
        ];
        let sut = sut_word(&ops).expect(&asm);
        prop_assert_eq!(sut, mc, "uppercase V prefix must match llvm-mc {}", asm);
    }
}

#[test]
fn test_encode_neon_bsl_regression_extra_operand() {
    let ops = [arr(0, "8b"), arr(0, "8b"), arr(0, "8b"), arr(0, "8b")];
    assert!(
        encode_neon_bsl(&ops).is_err(),
        "bsl v0.8b, v0.8b, v0.8b, v0.8b must Err (llvm-mc/gas reject a fourth operand)"
    );
}

#[test]
fn test_encode_neon_bsl_regression_invalid_t() {
    let ops = [arr(0, "4h"), arr(0, "4h"), arr(0, "4h")];
    assert!(
        encode_neon_bsl(&ops).is_err(),
        "bsl v0.4h, v0.4h, v0.4h must Err (only .8b/.16b; llvm-mc: invalid operand)"
    );
}

#[test]
fn test_encode_neon_bsl_regression_mismatch_t() {
    let ops = [arr(0, "8b"), arr(0, "8b"), arr(0, "16b")];
    assert!(
        encode_neon_bsl(&ops).is_err(),
        "bsl v0.8b, v0.8b, v0.16b must Err (llvm-mc/gas: operand mismatch)"
    );
}

#[test]
fn test_encode_neon_bsl_regression_gpr_dest() {
    let ops = [gpr("x", 0), gpr("x", 0), gpr("x", 0)];
    assert!(
        encode_neon_bsl(&ops).is_err(),
        "bsl x0, x0, x0 must Err (llvm-mc/gas reject GPR operands)"
    );
}

#[test]
fn test_encode_neon_bsl_regression_bare_v() {
    let ops = [gpr("v", 0), gpr("v", 1), gpr("v", 2)];
    assert!(
        encode_neon_bsl(&ops).is_err(),
        "bsl v0, v1, v2 must Err (llvm-mc/gas require .8b/.16b arrangement)"
    );
}

#[test]
fn test_encode_neon_bsl_regression_sp() {
    let ops = [Operand::Reg("sp".into()), arr(0, "8b"), arr(1, "8b")];
    assert!(
        encode_neon_bsl(&ops).is_err(),
        "bsl sp, v0.8b, v1.8b must Err (llvm-mc/gas reject SP)"
    );
}
