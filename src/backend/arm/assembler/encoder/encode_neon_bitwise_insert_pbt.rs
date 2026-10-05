// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md:224 NEON three-same lists bif, bit, bsl;
//   encoder/mod.rs:1-7 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:754-755 "bit" => encode_neon_bitwise_insert(operands, 0b10); "bif" => ... 0b11;
//   neon.rs:1664 Encodes BIT (size=10) and BIF (size=11) instructions;
//   neon.rs:1666 Format: 0 Q 1 01110 ss 1 Rm 000111 Rn Rd;
//   ARM ARM Advanced SIMD three-same (BIT/BIF); T in {8B,16B}.
// Stronger considered:
//   - State machine: rejected — encode_neon_bitwise_insert is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree BIT/BIF decoder
//   - Differential vs encode_neon_bsl / encode_neon_bic: rejected — same-job gate
//     (BSL/BIC are different three-same opcodes, not interchangeable with BIT/BIF)
// Weaker available: algebraic.metamorphic (Rd/Rn/Rm/size fields), algebraic.invariant (word layout),
//   negative_error (arity / extra / invalid T / mismatched T / GPR dest)
// Differential: candidate=encode_neon_bitwise_insert, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[RegArrangement(Vd,T), RegArrangement(Vn,T), RegArrangement(Vm,T)] + size
//     <-> `bit|bif Vd.T, Vn.T, Vm.T`

use super::encode_neon_bitwise_insert;
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

fn mnem(size: u32) -> &'static str {
    match size {
        0b10 => "bit",
        0b11 => "bif",
        _ => "bit",
    }
}

fn sut_word(ops: &[Operand], size: u32) -> Result<u32, String> {
    match encode_neon_bitwise_insert(ops, size)? {
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

fn size_bit() -> impl Strategy<Value = u32> {
    prop::sample::select(vec![0b10u32, 0b11])
}

/// ARM Advanced SIMD three-same BIT/BIF:
/// 0 Q 1 01110 ss 1 Rm 000111 Rn Rd
fn arm_bit_bif_word(rd: u32, rn: u32, rm: u32, t: &str, size: u32) -> u32 {
    let q = if t == "16b" { 1u32 } else { 0 };
    (q << 30)
        | (1 << 29)
        | (0b01110 << 24)
        | (size << 22)
        | (1 << 21)
        | (rm << 16)
        | (0b000111 << 10)
        | (rn << 5)
        | rd
}

#[test]
fn encode_neon_bitwise_insert_kat_llvm_mc_bit_v0_8b_v1_v2() {
    let want = 0x2ea21c20u32;
    let mc = llvm_mc_word("bit v0.8b, v1.8b, v2.8b").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [arr(0, "8b"), arr(1, "8b"), arr(2, "8b")];
    let sut = sut_word(&ops, 0b10).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_neon_bitwise_insert_kat_llvm_mc_bif_v0_8b_v1_v2() {
    let want = 0x2ee21c20u32;
    let mc = llvm_mc_word("bif v0.8b, v1.8b, v2.8b").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [arr(0, "8b"), arr(1, "8b"), arr(2, "8b")];
    let sut = sut_word(&ops, 0b11).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_neon_bitwise_insert_kat_llvm_mc_bit_v0_16b_v1_v2() {
    let want = 0x6ea21c20u32;
    let mc = llvm_mc_word("bit v0.16b, v1.16b, v2.16b").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [arr(0, "16b"), arr(1, "16b"), arr(2, "16b")];
    let sut = sut_word(&ops, 0b10).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_neon_bitwise_insert_kat_llvm_mc_bif_v31_16b_v0_v1() {
    let want = 0x6ee11c1fu32;
    let mc = llvm_mc_word("bif v31.16b, v0.16b, v1.16b").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [arr(31, "16b"), arr(0, "16b"), arr(1, "16b")];
    let sut = sut_word(&ops, 0b11).expect("SUT KAT");
    assert_eq!(sut, want);
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_bitwise_insert_diff_llvm_mc(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        t in valid_t(),
        size in size_bit(),
    ) {
        let asm = format!("{} v{rd}.{t}, v{rn}.{t}, v{rm}.{t}", mnem(size));
        let mc = llvm_mc_word(&asm).expect(&asm);
        let sut = sut_word(&[arr(rd, t), arr(rn, t), arr(rm, t)], size).expect(&asm);
        prop_assert_eq!(sut, mc, "SUT vs llvm-mc for {}", asm);
    }

    #[test]
    fn encode_neon_bitwise_insert_meta_rd_rn_rm_size(
        rd1 in reg_num(),
        rd2 in reg_num(),
        rn1 in reg_num(),
        rn2 in reg_num(),
        rm1 in reg_num(),
        rm2 in reg_num(),
        t in valid_t(),
        size in size_bit(),
        size2 in size_bit(),
    ) {
        let w111 = sut_word(&[arr(rd1, t), arr(rn1, t), arr(rm1, t)], size).expect("w111");
        let w211 = sut_word(&[arr(rd2, t), arr(rn1, t), arr(rm1, t)], size).expect("w211");
        let w121 = sut_word(&[arr(rd1, t), arr(rn2, t), arr(rm1, t)], size).expect("w121");
        let w112 = sut_word(&[arr(rd1, t), arr(rn1, t), arr(rm2, t)], size).expect("w112");
        let w111b = sut_word(&[arr(rd1, t), arr(rn1, t), arr(rm1, t)], size2).expect("w111b");
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
        prop_assert_eq!(
            (w111 ^ w111b) & !(0b11u32 << 22),
            0,
            "changing only size must differ only in bits[23:22]"
        );
        prop_assert_eq!((w111 >> 22) & 0b11, size);
        prop_assert_eq!((w111b >> 22) & 0b11, size2);
    }

    #[test]
    fn encode_neon_bitwise_insert_inv_arm_layout(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        t in valid_t(),
        size in size_bit(),
    ) {
        let w = sut_word(&[arr(rd, t), arr(rn, t), arr(rm, t)], size).expect("layout");
        prop_assert_eq!(w, arm_bit_bif_word(rd, rn, rm, t, size), "ARM BIT/BIF layout");
        prop_assert_eq!(w >> 31, 0);
        let q = if t == "16b" { 1u32 } else { 0 };
        prop_assert_eq!((w >> 30) & 1, q);
        prop_assert_eq!((w >> 29) & 1, 1);
        prop_assert_eq!((w >> 24) & 0x1F, 0b01110);
        prop_assert_eq!((w >> 22) & 0b11, size);
        prop_assert_eq!((w >> 21) & 1, 1);
        prop_assert_eq!((w >> 16) & 0x1F, rm);
        prop_assert_eq!((w >> 10) & 0x3F, 0b000111);
        prop_assert_eq!((w >> 5) & 0x1F, rn);
        prop_assert_eq!(w & 0x1F, rd);
        let w8 = sut_word(&[arr(rd, "8b"), arr(rn, "8b"), arr(rm, "8b")], size).expect("q8");
        let w16 = sut_word(&[arr(rd, "16b"), arr(rn, "16b"), arr(rm, "16b")], size).expect("q16");
        prop_assert_eq!(w8 ^ w16, 1u32 << 30, "8b vs 16b must differ only in Q");
    }

    #[test]
    fn encode_neon_bitwise_insert_neg_arity(n in 0usize..=2, rd in reg_num(), t in valid_t(), size in size_bit()) {
        let ops: Vec<Operand> = match n {
            0 => vec![],
            1 => vec![arr(rd, t)],
            _ => vec![arr(rd, t), arr(rd, t)],
        };
        prop_assert!(
            encode_neon_bitwise_insert(&ops, size).is_err(),
            "arity {} must Err (bit/bif requires 3 operands)",
            n
        );
    }

    #[test]
    fn encode_neon_bitwise_insert_neg_extra(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        extra in reg_num(),
        t in valid_t(),
        size in size_bit(),
    ) {
        let asm = format!("{} v{rd}.{t}, v{rn}.{t}, v{rm}.{t}, v{extra}.{t}", mnem(size));
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted extra {}",
            asm
        );
        let ops = [arr(rd, t), arr(rn, t), arr(rm, t), arr(extra, t)];
        prop_assert!(
            encode_neon_bitwise_insert(&ops, size).is_err(),
            "extra operand must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_bitwise_insert_neg_invalid_t(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        t in invalid_t(),
        size in size_bit(),
    ) {
        let asm = format!("{} v{rd}.{t}, v{rn}.{t}, v{rm}.{t}", mnem(size));
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let ops = [arr(rd, t), arr(rn, t), arr(rm, t)];
        prop_assert!(
            encode_neon_bitwise_insert(&ops, size).is_err(),
            "invalid T must Err (only .8b/.16b; llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_bitwise_insert_neg_mismatch_t(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        td in valid_t(),
        tn in valid_t(),
        tm in valid_t(),
        size in size_bit(),
    ) {
        prop_assume!(!(td == tn && tn == tm));
        let asm = format!("{} v{rd}.{td}, v{rn}.{tn}, v{rm}.{tm}", mnem(size));
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let ops = [arr(rd, td), arr(rn, tn), arr(rm, tm)];
        prop_assert!(
            encode_neon_bitwise_insert(&ops, size).is_err(),
            "mismatched T must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_bitwise_insert_neg_gpr_bare_sp(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        t in valid_t(),
        size in size_bit(),
        kind in 0u8..=6,
    ) {
        let m = mnem(size);
        let (ops, asm) = match kind {
            0 => {
                let asm = format!("{m} x{rd}, x{rn}, x{rm}");
                let ops = vec![gpr("x", rd), gpr("x", rn), gpr("x", rm)];
                (ops, asm)
            }
            1 => {
                let asm = format!("{m} w{rd}, v{rn}.{t}, v{rm}.{t}");
                let ops = vec![gpr("w", rd), arr(rn, t), arr(rm, t)];
                (ops, asm)
            }
            2 => {
                let asm = format!("{m} sp, v0.8b, v1.8b");
                let ops = vec![Operand::Reg("sp".into()), arr(0, "8b"), arr(1, "8b")];
                (ops, asm)
            }
            3 => {
                let asm = format!("{m} v{rd}, v{rn}, v{rm}");
                let ops = vec![gpr("v", rd), gpr("v", rn), gpr("v", rm)];
                (ops, asm)
            }
            4 => {
                let asm = format!("{m} d{rd}, d{rn}, d{rm}");
                let ops = vec![gpr("d", rd), gpr("d", rn), gpr("d", rm)];
                (ops, asm)
            }
            5 => {
                let asm = format!("{m} s{rd}, v{rn}.{t}, v{rm}.{t}");
                let ops = vec![gpr("s", rd), arr(rn, t), arr(rm, t)];
                (ops, asm)
            }
            _ => {
                let asm = format!("{m} q{rd}, v{rn}.{t}, v{rm}.{t}");
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
            encode_neon_bitwise_insert(&ops, size).is_err(),
            "non-arranged NEON / GPR / SP / FP must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_bitwise_insert_diff_alt_spellings(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        t in valid_t(),
        size in size_bit(),
    ) {
        let asm = format!("{} V{rd}.{t}, V{rn}.{t}, V{rm}.{t}", mnem(size));
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
        let sut = sut_word(&ops, size).expect(&asm);
        prop_assert_eq!(sut, mc, "uppercase V prefix must match llvm-mc {}", asm);
    }
}

#[test]
fn test_encode_neon_bitwise_insert_regression_extra_operand() {
    let ops = [arr(0, "8b"), arr(0, "8b"), arr(0, "8b"), arr(0, "8b")];
    assert!(
        encode_neon_bitwise_insert(&ops, 0b10).is_err(),
        "bit v0.8b, v0.8b, v0.8b, v0.8b must Err (llvm-mc/gas reject a fourth operand)"
    );
}

#[test]
fn test_encode_neon_bitwise_insert_regression_invalid_t() {
    let ops = [arr(0, "4h"), arr(0, "4h"), arr(0, "4h")];
    assert!(
        encode_neon_bitwise_insert(&ops, 0b10).is_err(),
        "bit v0.4h, v0.4h, v0.4h must Err (only .8b/.16b; llvm-mc: invalid operand)"
    );
}

#[test]
fn test_encode_neon_bitwise_insert_regression_mismatch_t() {
    let ops = [arr(0, "16b"), arr(0, "8b"), arr(0, "8b")];
    assert!(
        encode_neon_bitwise_insert(&ops, 0b10).is_err(),
        "bit v0.16b, v0.8b, v0.8b must Err (llvm-mc/gas: operand mismatch)"
    );
}

#[test]
fn test_encode_neon_bitwise_insert_regression_gpr_dest() {
    let ops = [gpr("x", 0), gpr("x", 0), gpr("x", 0)];
    assert!(
        encode_neon_bitwise_insert(&ops, 0b10).is_err(),
        "bit x0, x0, x0 must Err (llvm-mc/gas reject GPR operands)"
    );
}

#[test]
fn test_encode_neon_bitwise_insert_regression_bare_v() {
    let ops = [gpr("v", 0), gpr("v", 1), gpr("v", 2)];
    assert!(
        encode_neon_bitwise_insert(&ops, 0b10).is_err(),
        "bit v0, v1, v2 must Err (llvm-mc/gas require .8b/.16b arrangement)"
    );
}

#[test]
fn test_encode_neon_bitwise_insert_regression_sp() {
    let ops = [Operand::Reg("sp".into()), arr(0, "8b"), arr(1, "8b")];
    assert!(
        encode_neon_bitwise_insert(&ops, 0b10).is_err(),
        "bit sp, v0.8b, v1.8b must Err (llvm-mc/gas reject SP)"
    );
}
