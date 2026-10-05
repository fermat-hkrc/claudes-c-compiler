// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md:225 NEON two-misc lists rev64;
//   encoder/mod.rs:1-7 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:750 "rev64" => encode_neon_rev64;
//   neon.rs:751 Encode NEON REV64: reverse elements within 64-bit doublewords;
//   neon.rs:761 REV64 Vd.T, Vn.T: 0 Q 0 01110 size 10 0000 0000 10 Rn Rd;
//   ARM ARM Advanced SIMD two-register miscellaneous (REV64); T in {8B,16B,4H,8H,2S,4S}.
// Stronger considered:
//   - State machine: rejected — encode_neon_rev64 is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree REV64 decoder
//   - Differential vs encode_cnt / encode_neon_not / encode_neon_rbit: rejected — same-job gate
//     (CNT/NOT/RBIT are different two-misc opcodes, not interchangeable with REV64)
//   - Differential vs encode_rev: rejected — scalar REV; public dispatch does not route `rev64` there
// Weaker available: algebraic.metamorphic (Rd/Rn fields), algebraic.invariant (word layout),
//   negative_error (arity / extra / invalid T / mismatched T / non-NEON operands)
// Differential: candidate=encode_neon_rev64, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[RegArrangement(Vd,T), RegArrangement(Vn,T)] <-> `rev64 Vd.T, Vn.T`

use super::encode_neon_rev64;
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
    match encode_neon_rev64(ops)? {
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
    prop::sample::select(vec!["8b", "16b", "4h", "8h", "2s", "4s"])
}

fn invalid_t() -> impl Strategy<Value = &'static str> {
    prop::sample::select(vec!["1d", "2d", "4b", "8d", "1s", "2h", "8s", "32b"])
}

fn mismatched_t() -> impl Strategy<Value = (&'static str, &'static str)> {
    valid_t().prop_flat_map(|td| {
        let others: Vec<&'static str> = ["8b", "16b", "4h", "8h", "2s", "4s"]
            .into_iter()
            .filter(|t| *t != td)
            .collect();
        prop::sample::select(others).prop_map(move |tn| (td, tn))
    })
}

/// ARM Advanced SIMD two-register miscellaneous REV64:
/// 0 Q 0 01110 size 10 0000 0000 10 Rn Rd
/// T in {8B,16B,4H,8H,2S,4S}; Q=1 iff T in {16B,8H,4S}; size = B:00 H:01 S:10
fn arm_rev64_word(rd: u32, rn: u32, t: &str) -> u32 {
    let (q, size) = match t {
        "8b" => (0u32, 0u32),
        "16b" => (1, 0),
        "4h" => (0, 1),
        "8h" => (1, 1),
        "2s" => (0, 2),
        "4s" => (1, 2),
        _ => unreachable!("valid T only"),
    };
    (q << 30) | (0b001110 << 24) | (size << 22) | (0b100000 << 16) | (0b000010 << 10) | (rn << 5) | rd
}

// Reference KAT gate (must pass before PBT).
#[test]
fn encode_neon_rev64_kat_llvm_mc_v0_8b_v1_8b() {
    let want = 0x0e200820u32;
    let mc = llvm_mc_word("rev64 v0.8b, v1.8b").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [arr(0, "8b"), arr(1, "8b")];
    let sut = sut_word(&ops).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_neon_rev64_kat_llvm_mc_v0_16b_v1_16b() {
    let want = 0x4e200820u32;
    let mc = llvm_mc_word("rev64 v0.16b, v1.16b").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [arr(0, "16b"), arr(1, "16b")];
    let sut = sut_word(&ops).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_neon_rev64_kat_llvm_mc_v0_4h_v1_4h() {
    let want = 0x0e600820u32;
    let mc = llvm_mc_word("rev64 v0.4h, v1.4h").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [arr(0, "4h"), arr(1, "4h")];
    let sut = sut_word(&ops).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_neon_rev64_kat_llvm_mc_v0_4s_v1_4s() {
    let want = 0x4ea00820u32;
    let mc = llvm_mc_word("rev64 v0.4s, v1.4s").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [arr(0, "4s"), arr(1, "4s")];
    let sut = sut_word(&ops).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_neon_rev64_kat_llvm_mc_v31_8b_v31_8b() {
    let want = 0x0e200bffu32;
    let mc = llvm_mc_word("rev64 v31.8b, v31.8b").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [arr(31, "8b"), arr(31, "8b")];
    let sut = sut_word(&ops).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_neon_rev64_kat_llvm_mc_v31_4s_v0_4s() {
    let want = 0x4ea0081fu32;
    let mc = llvm_mc_word("rev64 v31.4s, v0.4s").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [arr(31, "4s"), arr(0, "4s")];
    let sut = sut_word(&ops).expect("SUT KAT");
    assert_eq!(sut, want);
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_rev64_diff_llvm_mc(
        rd in reg_num(),
        rn in reg_num(),
        t in valid_t(),
    ) {
        let asm = format!("rev64 v{rd}.{t}, v{rn}.{t}");
        let mc = llvm_mc_word(&asm).expect(&asm);
        let sut = sut_word(&[arr(rd, t), arr(rn, t)]).expect(&asm);
        prop_assert_eq!(sut, mc, "SUT vs llvm-mc for {}", asm);
    }

    #[test]
    fn encode_neon_rev64_meta_rd_rn(
        rd1 in reg_num(),
        rd2 in reg_num(),
        rn1 in reg_num(),
        rn2 in reg_num(),
        t in valid_t(),
    ) {
        let w11 = sut_word(&[arr(rd1, t), arr(rn1, t)]).expect("w11");
        let w21 = sut_word(&[arr(rd2, t), arr(rn1, t)]).expect("w21");
        let w12 = sut_word(&[arr(rd1, t), arr(rn2, t)]).expect("w12");
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
    }

    #[test]
    fn encode_neon_rev64_inv_layout(
        rd in reg_num(),
        rn in reg_num(),
        t in valid_t(),
    ) {
        let w = sut_word(&[arr(rd, t), arr(rn, t)]).expect("layout");
        prop_assert_eq!(w, arm_rev64_word(rd, rn, t), "ARM REV64 layout");
        prop_assert_eq!(w >> 31, 0);
        let (q, size) = match t {
            "8b" => (0u32, 0u32),
            "16b" => (1, 0),
            "4h" => (0, 1),
            "8h" => (1, 1),
            "2s" => (0, 2),
            "4s" => (1, 2),
            _ => unreachable!(),
        };
        prop_assert_eq!((w >> 30) & 1, q);
        prop_assert_eq!((w >> 24) & 0x3F, 0b001110);
        prop_assert_eq!((w >> 22) & 0b11, size);
        prop_assert_eq!((w >> 16) & 0x3F, 0b100000);
        prop_assert_eq!((w >> 10) & 0x3F, 0b000010);
        prop_assert_eq!((w >> 5) & 0x1F, rn);
        prop_assert_eq!(w & 0x1F, rd);
    }

    #[test]
    fn encode_neon_rev64_neg_arity(n in 0usize..=1, rd in reg_num(), t in valid_t()) {
        let ops: Vec<Operand> = match n {
            0 => vec![],
            _ => vec![arr(rd, t)],
        };
        prop_assert!(
            encode_neon_rev64(&ops).is_err(),
            "arity {} must Err (rev64 requires 2 operands)",
            n
        );
    }

    #[test]
    fn encode_neon_rev64_neg_extra(
        rd in reg_num(),
        rn in reg_num(),
        extra in reg_num(),
        t in valid_t(),
    ) {
        let asm = format!("rev64 v{rd}.{t}, v{rn}.{t}, v{extra}.{t}");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted extra {}",
            asm
        );
        let ops = [arr(rd, t), arr(rn, t), arr(extra, t)];
        prop_assert!(
            encode_neon_rev64(&ops).is_err(),
            "extra operand must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_rev64_neg_invalid_t(
        rd in reg_num(),
        rn in reg_num(),
        t in invalid_t(),
    ) {
        let asm = format!("rev64 v{rd}.{t}, v{rn}.{t}");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let ops = [arr(rd, t), arr(rn, t)];
        prop_assert!(
            encode_neon_rev64(&ops).is_err(),
            "invalid T must Err (only .8b/.16b/.4h/.8h/.2s/.4s; llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_rev64_neg_mismatch_t(
        rd in reg_num(),
        rn in reg_num(),
        (td, tn) in mismatched_t(),
    ) {
        let asm = format!("rev64 v{rd}.{td}, v{rn}.{tn}");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let ops = [arr(rd, td), arr(rn, tn)];
        prop_assert!(
            encode_neon_rev64(&ops).is_err(),
            "mismatched T must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_rev64_neg_non_neon(
        rd in reg_num(),
        rn in reg_num(),
        t in valid_t(),
        kind in 0u8..=7,
    ) {
        let (ops, asm, check_llvm) = match kind {
            0 => {
                let asm = format!("rev64 x{rd}, x{rn}");
                let ops = vec![gpr("x", rd), gpr("x", rn)];
                // llvm-mc aliases this to scalar `rev`; gas rejects. NEON encoder must Err.
                (ops, asm, false)
            }
            1 => {
                let asm = format!("rev64 w{rd}, v{rn}.{t}");
                let ops = vec![gpr("w", rd), arr(rn, t)];
                (ops, asm, true)
            }
            2 => {
                let asm = "rev64 sp, v0.8b".to_string();
                let ops = vec![Operand::Reg("sp".into()), arr(0, "8b")];
                (ops, asm, true)
            }
            3 => {
                let asm = format!("rev64 v{rd}, v{rn}");
                let ops = vec![gpr("v", rd), gpr("v", rn)];
                (ops, asm, true)
            }
            4 => {
                let asm = format!("rev64 d{rd}, d{rn}");
                let ops = vec![gpr("d", rd), gpr("d", rn)];
                (ops, asm, true)
            }
            5 => {
                let asm = format!("rev64 s{rd}, v{rn}.{t}");
                let ops = vec![gpr("s", rd), arr(rn, t)];
                (ops, asm, true)
            }
            6 => {
                let asm = format!("rev64 q{rd}, v{rn}.{t}");
                let ops = vec![gpr("q", rd), arr(rn, t)];
                (ops, asm, true)
            }
            _ => {
                let asm = format!("rev64 v{rd}.{t}, x{rn}");
                let ops = vec![arr(rd, t), gpr("x", rn)];
                (ops, asm, true)
            }
        };
        if check_llvm {
            prop_assert!(
                llvm_mc_word(&asm).is_err(),
                "llvm-mc unexpectedly accepted {}",
                asm
            );
        }
        prop_assert!(
            encode_neon_rev64(&ops).is_err(),
            "non-arranged NEON / GPR / SP / FP must Err (gas rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_rev64_diff_alt_spellings(
        rd in reg_num(),
        rn in reg_num(),
        t in valid_t(),
    ) {
        let asm = format!("rev64 V{rd}.{t}, V{rn}.{t}");
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
        let sut = sut_word(&ops).expect(&asm);
        prop_assert_eq!(sut, mc, "uppercase V prefix must match llvm-mc {}", asm);
    }
}

#[test]
fn test_encode_neon_rev64_regression_extra_operand() {
    let ops = [arr(0, "8b"), arr(0, "8b"), arr(0, "8b")];
    assert!(
        encode_neon_rev64(&ops).is_err(),
        "rev64 v0.8b, v0.8b, v0.8b must Err (llvm-mc/gas reject a third operand)"
    );
}

#[test]
fn test_encode_neon_rev64_regression_invalid_t() {
    let ops = [arr(0, "1d"), arr(0, "1d")];
    assert!(
        encode_neon_rev64(&ops).is_err(),
        "rev64 v0.1d, v0.1d must Err (size=11 reserved; llvm-mc/gas reject .1d/.2d)"
    );
}

#[test]
fn test_encode_neon_rev64_regression_invalid_t_2d() {
    let ops = [arr(0, "2d"), arr(0, "2d")];
    assert!(
        encode_neon_rev64(&ops).is_err(),
        "rev64 v0.2d, v0.2d must Err (size=11 reserved; llvm-mc/gas reject .2d)"
    );
}

#[test]
fn test_encode_neon_rev64_regression_mismatch_t() {
    let ops = [arr(0, "8b"), arr(0, "16b")];
    assert!(
        encode_neon_rev64(&ops).is_err(),
        "rev64 v0.8b, v0.16b must Err (llvm-mc/gas: operand mismatch)"
    );
}

#[test]
fn test_encode_neon_rev64_regression_gpr_src() {
    let ops = [arr(0, "8b"), gpr("x", 0)];
    assert!(
        encode_neon_rev64(&ops).is_err(),
        "rev64 v0.8b, x0 must Err (gas: operand 2 must be a SIMD vector register)"
    );
}
