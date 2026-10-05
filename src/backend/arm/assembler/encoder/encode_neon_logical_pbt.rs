// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md:224 NEON three-same lists and, orr, eor;
//   encoder/mod.rs:1-7 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:295-297 "and"/"orr"/"eor" => encode_logical(operands, opc);
//   data_processing.rs:461-463 NEON vector form passes through to encode_neon_logical;
//   neon.rs:296 Encode NEON logical operations: ORR/AND/EOR Vd.T, Vn.T, Vm.T;
//   neon.rs:306-308 AND/ORR/EOR encodings;
//   ARM ARM Advanced SIMD three-same AND/ORR/EOR; T in {8B,16B}.
// Stronger considered:
//   - State machine: rejected — encode_neon_logical is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree vector-logical decoder
//   - Differential vs encode_neon_bic / encode_neon_bsl / encode_orn: rejected — same-job gate
//     (BIC size=01, BSL U=1 size=01, ORN size=11; not interchangeable with AND/ORR/EOR)
// Weaker available: algebraic.metamorphic (Rd/Rn/Rm fields, opc U/size), algebraic.invariant (word layout),
//   negative_error (arity / extra / invalid T / mismatched T / GPR dest / ANDS opc)
// Differential: candidate=encode_neon_logical, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[RegArrangement(Vd,T), RegArrangement(Vn,T), RegArrangement(Vm,T)] + opc
//     <-> `{and,orr,eor} Vd.T, Vn.T, Vm.T`

use super::encode_neon_logical;
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

fn mnemonic(opc: u32) -> &'static str {
    match opc {
        0b00 => "and",
        0b01 => "orr",
        0b10 => "eor",
        0b11 => "ands",
        _ => "and",
    }
}

fn sut_word(ops: &[Operand], opc: u32) -> Result<u32, String> {
    match encode_neon_logical(ops, opc)? {
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

fn valid_opc() -> impl Strategy<Value = u32> {
    prop::sample::select(vec![0b00u32, 0b01, 0b10])
}

/// ARM Advanced SIMD three-same logical:
/// AND: 0 Q 0 01110 size=00 1 Rm 000111 Rn Rd = 0x0e201c00 | (Q<<30) | (Rm<<16) | (Rn<<5) | Rd
/// ORR: 0 Q 0 01110 size=10 1 Rm 000111 Rn Rd = 0x0ea01c00 | ...
/// EOR: 0 Q 1 01110 size=00 1 Rm 000111 Rn Rd = 0x2e201c00 | ...
fn arm_logical_word(rd: u32, rn: u32, rm: u32, t: &str, opc: u32) -> u32 {
    let q = if t == "16b" { 1u32 } else { 0 };
    let (u_bit, size) = match opc {
        0b00 => (0u32, 0b00u32),
        0b01 => (0, 0b10),
        0b10 => (1, 0b00),
        _ => (0, 0),
    };
    (q << 30)
        | (u_bit << 29)
        | (0b01110 << 24)
        | (size << 22)
        | (1 << 21)
        | (rm << 16)
        | (0b000111 << 10)
        | (rn << 5)
        | rd
}

// Reference KAT gate (must pass before PBT).
#[test]
fn encode_neon_logical_kat_llvm_mc_and_v0_8b_v1_v2() {
    let want = 0x0e221c20u32;
    let mc = llvm_mc_word("and v0.8b, v1.8b, v2.8b").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [arr(0, "8b"), arr(1, "8b"), arr(2, "8b")];
    let sut = sut_word(&ops, 0b00).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_neon_logical_kat_llvm_mc_and_v0_16b_v1_v2() {
    let want = 0x4e221c20u32;
    let mc = llvm_mc_word("and v0.16b, v1.16b, v2.16b").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [arr(0, "16b"), arr(1, "16b"), arr(2, "16b")];
    let sut = sut_word(&ops, 0b00).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_neon_logical_kat_llvm_mc_orr_v0_8b_v1_v2() {
    let want = 0x0ea21c20u32;
    let mc = llvm_mc_word("orr v0.8b, v1.8b, v2.8b").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [arr(0, "8b"), arr(1, "8b"), arr(2, "8b")];
    let sut = sut_word(&ops, 0b01).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_neon_logical_kat_llvm_mc_orr_v0_16b_v1_v2() {
    let want = 0x4ea21c20u32;
    let mc = llvm_mc_word("orr v0.16b, v1.16b, v2.16b").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [arr(0, "16b"), arr(1, "16b"), arr(2, "16b")];
    let sut = sut_word(&ops, 0b01).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_neon_logical_kat_llvm_mc_eor_v0_8b_v1_v2() {
    let want = 0x2e221c20u32;
    let mc = llvm_mc_word("eor v0.8b, v1.8b, v2.8b").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [arr(0, "8b"), arr(1, "8b"), arr(2, "8b")];
    let sut = sut_word(&ops, 0b10).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_neon_logical_kat_llvm_mc_eor_v0_16b_v1_v2() {
    let want = 0x6e221c20u32;
    let mc = llvm_mc_word("eor v0.16b, v1.16b, v2.16b").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [arr(0, "16b"), arr(1, "16b"), arr(2, "16b")];
    let sut = sut_word(&ops, 0b10).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_neon_logical_kat_llvm_mc_and_v31_16b() {
    let want = 0x4e3f1fffu32;
    let mc = llvm_mc_word("and v31.16b, v31.16b, v31.16b").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [arr(31, "16b"), arr(31, "16b"), arr(31, "16b")];
    let sut = sut_word(&ops, 0b00).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_neon_logical_kat_llvm_mc_eor_v15_16b_v16_v17() {
    let want = 0x6e311e0fu32;
    let mc = llvm_mc_word("eor v15.16b, v16.16b, v17.16b").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [arr(15, "16b"), arr(16, "16b"), arr(17, "16b")];
    let sut = sut_word(&ops, 0b10).expect("SUT KAT");
    assert_eq!(sut, want);
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_logical_diff_llvm_mc(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        t in valid_t(),
        opc in valid_opc(),
    ) {
        let mnem = mnemonic(opc);
        let asm = format!("{mnem} v{rd}.{t}, v{rn}.{t}, v{rm}.{t}");
        let mc = llvm_mc_word(&asm).expect(&asm);
        let sut = sut_word(&[arr(rd, t), arr(rn, t), arr(rm, t)], opc).expect(&asm);
        prop_assert_eq!(sut, mc, "SUT vs llvm-mc for {}", asm);
    }

    #[test]
    fn encode_neon_logical_meta_rd_rn_rm(
        rd1 in reg_num(),
        rd2 in reg_num(),
        rn1 in reg_num(),
        rn2 in reg_num(),
        rm1 in reg_num(),
        rm2 in reg_num(),
        t in valid_t(),
        opc in valid_opc(),
    ) {
        let w111 = sut_word(&[arr(rd1, t), arr(rn1, t), arr(rm1, t)], opc).expect("w111");
        let w211 = sut_word(&[arr(rd2, t), arr(rn1, t), arr(rm1, t)], opc).expect("w211");
        let w121 = sut_word(&[arr(rd1, t), arr(rn2, t), arr(rm1, t)], opc).expect("w121");
        let w112 = sut_word(&[arr(rd1, t), arr(rn1, t), arr(rm2, t)], opc).expect("w112");
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
    fn encode_neon_logical_inv_layout(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        t in valid_t(),
        opc in valid_opc(),
    ) {
        let w = sut_word(&[arr(rd, t), arr(rn, t), arr(rm, t)], opc).expect("layout");
        prop_assert_eq!(w, arm_logical_word(rd, rn, rm, t, opc), "ARM logical layout");
        prop_assert_eq!(w >> 31, 0);
        let q = if t == "16b" { 1u32 } else { 0 };
        prop_assert_eq!((w >> 30) & 1, q);
        let (u_bit, size) = match opc {
            0b00 => (0u32, 0b00u32),
            0b01 => (0, 0b10),
            _ => (1, 0b00),
        };
        prop_assert_eq!((w >> 29) & 1, u_bit);
        prop_assert_eq!((w >> 24) & 0x1F, 0b01110);
        prop_assert_eq!((w >> 22) & 0b11, size);
        prop_assert_eq!((w >> 21) & 1, 1);
        prop_assert_eq!((w >> 16) & 0x1F, rm);
        prop_assert_eq!((w >> 10) & 0x3F, 0b000111);
        prop_assert_eq!((w >> 5) & 0x1F, rn);
        prop_assert_eq!(w & 0x1F, rd);
        let w8 = sut_word(&[arr(rd, "8b"), arr(rn, "8b"), arr(rm, "8b")], opc).expect("q8");
        let w16 = sut_word(&[arr(rd, "16b"), arr(rn, "16b"), arr(rm, "16b")], opc).expect("q16");
        prop_assert_eq!(w8 ^ w16, 1u32 << 30, "8b vs 16b must differ only in Q");
    }

    #[test]
    fn encode_neon_logical_meta_opc(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        t in valid_t(),
    ) {
        let ops = [arr(rd, t), arr(rn, t), arr(rm, t)];
        let wand = sut_word(&ops, 0b00).expect("and");
        let worr = sut_word(&ops, 0b01).expect("orr");
        let weor = sut_word(&ops, 0b10).expect("eor");
        prop_assert_eq!(
            (wand ^ worr) & !(0b11u32 << 22),
            0,
            "AND vs ORR must differ only in size bits[23:22]"
        );
        prop_assert_eq!((wand >> 22) & 0b11, 0b00);
        prop_assert_eq!((worr >> 22) & 0b11, 0b10);
        prop_assert_eq!(
            (wand ^ weor) & !(1u32 << 29),
            0,
            "AND vs EOR must differ only in U bit 29"
        );
        prop_assert_eq!((wand >> 29) & 1, 0);
        prop_assert_eq!((weor >> 29) & 1, 1);
        prop_assert_eq!(
            (worr ^ weor) & !((1u32 << 29) | (0b11u32 << 22)),
            0,
            "ORR vs EOR must differ only in U and size"
        );
    }

    #[test]
    fn encode_neon_logical_neg_extra(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        extra in reg_num(),
        t in valid_t(),
        opc in valid_opc(),
    ) {
        let mnem = mnemonic(opc);
        let asm = format!("{mnem} v{rd}.{t}, v{rn}.{t}, v{rm}.{t}, v{extra}.{t}");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted extra {}",
            asm
        );
        let ops = [arr(rd, t), arr(rn, t), arr(rm, t), arr(extra, t)];
        prop_assert!(
            encode_neon_logical(&ops, opc).is_err(),
            "extra operand must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_logical_neg_mismatch_t(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        td in valid_t(),
        tn in valid_t(),
        tm in valid_t(),
        opc in valid_opc(),
    ) {
        prop_assume!(!(td == tn && tn == tm));
        let mnem = mnemonic(opc);
        let asm = format!("{mnem} v{rd}.{td}, v{rn}.{tn}, v{rm}.{tm}");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let ops = [arr(rd, td), arr(rn, tn), arr(rm, tm)];
        prop_assert!(
            encode_neon_logical(&ops, opc).is_err(),
            "mismatched T must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_logical_neg_invalid_t(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        t in invalid_t(),
        opc in valid_opc(),
    ) {
        let mnem = mnemonic(opc);
        let asm = format!("{mnem} v{rd}.{t}, v{rn}.{t}, v{rm}.{t}");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let ops = [arr(rd, t), arr(rn, t), arr(rm, t)];
        prop_assert!(
            encode_neon_logical(&ops, opc).is_err(),
            "invalid T must Err (only .8b/.16b; llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_logical_neg_gpr_bare_sp(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        t in valid_t(),
        opc in valid_opc(),
        kind in 0u8..=6,
    ) {
        let mnem = mnemonic(opc);
        // Dest is always arranged NEON so the case is caller-reachable via
        // encode_logical's RegArrangement dispatch (data_processing.rs:461).
        // GPR/bare/SP/FP appear as Vn/Vm (or extra dest-like mix).
        let (ops, asm, check_llvm) = match kind {
            0 => {
                let asm = format!("{mnem} v{rd}.{t}, x{rn}, x{rm}");
                let ops = vec![arr(rd, t), gpr("x", rn), gpr("x", rm)];
                (ops, asm, true)
            }
            1 => {
                let asm = format!("{mnem} v{rd}.{t}, w{rn}, v{rm}.{t}");
                let ops = vec![arr(rd, t), gpr("w", rn), arr(rm, t)];
                (ops, asm, true)
            }
            2 => {
                let asm = format!("{mnem} v{rd}.{t}, sp, v{rm}.{t}");
                let ops = vec![arr(rd, t), Operand::Reg("sp".into()), arr(rm, t)];
                (ops, asm, true)
            }
            3 => {
                let asm = format!("{mnem} v{rd}.{t}, v{rn}, v{rm}");
                let ops = vec![arr(rd, t), gpr("v", rn), gpr("v", rm)];
                (ops, asm, true)
            }
            4 => {
                let asm = format!("{mnem} v{rd}.{t}, d{rn}, d{rm}");
                let ops = vec![arr(rd, t), gpr("d", rn), gpr("d", rm)];
                (ops, asm, true)
            }
            5 => {
                let asm = format!("{mnem} v{rd}.{t}, s{rn}, v{rm}.{t}");
                let ops = vec![arr(rd, t), gpr("s", rn), arr(rm, t)];
                (ops, asm, true)
            }
            _ => {
                let asm = format!("{mnem} v{rd}.{t}, q{rn}, v{rm}.{t}");
                let ops = vec![arr(rd, t), gpr("q", rn), arr(rm, t)];
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
            encode_neon_logical(&ops, opc).is_err(),
            "non-arranged NEON / GPR / SP / FP source must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_logical_neg_ands(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        t in valid_t(),
    ) {
        let asm = format!("ands v{rd}.{t}, v{rn}.{t}, v{rm}.{t}");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        prop_assert!(
            encode_neon_logical(&[arr(rd, t), arr(rn, t), arr(rm, t)], 0b11).is_err(),
            "ANDS is not a NEON instruction (neon.rs:313); must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_logical_neg_arity(
        n in 0usize..=2,
        rd in reg_num(),
        t in valid_t(),
        opc in valid_opc(),
    ) {
        let ops: Vec<Operand> = match n {
            0 => vec![],
            1 => vec![arr(rd, t)],
            _ => vec![arr(rd, t), arr(rd, t)],
        };
        prop_assert!(
            encode_neon_logical(&ops, opc).is_err(),
            "arity {} must Err (AND/ORR/EOR require 3 operands)",
            n
        );
    }

    #[test]
    fn encode_neon_logical_neg_unsupported_opc(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        t in valid_t(),
        opc in prop_oneof![Just(4u32), Just(5u32), Just(7u32), Just(15u32), 16u32..=255],
    ) {
        prop_assert!(
            encode_neon_logical(&[arr(rd, t), arr(rn, t), arr(rm, t)], opc).is_err(),
            "opc={opc} outside {{0,1,2,3}} must Err"
        );
    }

    #[test]
    fn encode_neon_logical_diff_alt_spellings(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        t in valid_t(),
        opc in valid_opc(),
    ) {
        let mnem = mnemonic(opc);
        let asm = format!("{mnem} V{rd}.{t}, V{rn}.{t}, V{rm}.{t}");
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
        let sut = sut_word(&ops, opc).expect(&asm);
        prop_assert_eq!(sut, mc, "uppercase V prefix must match llvm-mc {}", asm);
    }
}

#[test]
fn test_encode_neon_logical_regression_extra_operand() {
    let ops = [arr(0, "8b"), arr(0, "8b"), arr(0, "8b"), arr(0, "8b")];
    assert!(
        encode_neon_logical(&ops, 0b00).is_err(),
        "and v0.8b, v0.8b, v0.8b, v0.8b must Err (llvm-mc/gas reject a fourth operand)"
    );
}

#[test]
fn test_encode_neon_logical_regression_invalid_t() {
    let ops = [arr(0, "4h"), arr(0, "4h"), arr(0, "4h")];
    assert!(
        encode_neon_logical(&ops, 0b00).is_err(),
        "and v0.4h, v0.4h, v0.4h must Err (only .8b/.16b; llvm-mc: invalid operand)"
    );
}

#[test]
fn test_encode_neon_logical_regression_mismatch_t() {
    let ops = [arr(0, "8b"), arr(0, "8b"), arr(0, "16b")];
    assert!(
        encode_neon_logical(&ops, 0b00).is_err(),
        "and v0.8b, v0.8b, v0.16b must Err (llvm-mc/gas: operand mismatch)"
    );
}

#[test]
fn test_encode_neon_logical_regression_gpr_src() {
    let ops = [arr(0, "8b"), gpr("x", 0), gpr("x", 0)];
    assert!(
        encode_neon_logical(&ops, 0b00).is_err(),
        "and v0.8b, x0, x0 must Err (llvm-mc/gas reject GPR sources on vector AND)"
    );
}

#[test]
fn test_encode_neon_logical_regression_ands() {
    let ops = [arr(0, "16b"), arr(1, "16b"), arr(2, "16b")];
    assert!(
        encode_neon_logical(&ops, 0b11).is_err(),
        "ands v0.16b, v1.16b, v2.16b must Err (ANDS is not a NEON instruction; llvm-mc rejects it)"
    );
}
