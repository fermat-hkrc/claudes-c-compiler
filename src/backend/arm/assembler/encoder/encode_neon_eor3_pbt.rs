// Oracle: differential — llvm-mc AArch64 assembler (+sha3)
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md:238 NEON crypto lists eor3;
//   encoder/mod.rs:1-7 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:776 "eor3" => encode_neon_eor3(operands);
//   neon.rs:1111 Encode NEON EOR3 (three-way XOR, SHA3 extension): EOR3 Vd.16b, Vn.16b, Vm.16b, Vk.16b;
//   neon.rs:1119-1120 EOR3 Vd.16b form; Encoding: 11001110 000 Rm 0 Rk(4:0) 00 Rn Rd;
//   ARM ARM Cryptographic three-register SHA3 EOR3 Vd.16B, Vn.16B, Vm.16B, Va.16B.
// Stronger considered:
//   - State machine: rejected — encode_neon_eor3 is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree EOR3 decoder
//   - Differential vs encode_neon_aes / encode_neon_logical: rejected — same-job gate
//     (AES is two-reg 01001110 crypto; three-same EOR has no fourth register)
// Weaker available: algebraic.metamorphic (Rd/Rn/Rm/Rk fields), algebraic.invariant (word layout),
//   negative_error (arity / extra / T≠16b / mismatch / GPR dest)
// Differential: candidate=encode_neon_eor3, reference=llvm-mc -triple=aarch64 -mattr=+sha3 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[RegArrangement(Vd,16b), RegArrangement(Vn,16b), RegArrangement(Vm,16b), RegArrangement(Vk,16b)]
//     <-> `eor3 Vd.16b, Vn.16b, Vm.16b, Vk.16b`

use super::encode_neon_eor3;
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

fn vreg(n: u32) -> String {
    format!("v{}", n)
}

fn arr(reg: u32, t: &str) -> Operand {
    Operand::RegArrangement {
        reg: vreg(reg),
        arrangement: t.to_string(),
    }
}

fn eor3_ops(rd: u32, rn: u32, rm: u32, rk: u32, t: &str) -> Vec<Operand> {
    vec![arr(rd, t), arr(rn, t), arr(rm, t), arr(rk, t)]
}

fn eor3_asm(rd: u32, rn: u32, rm: u32, rk: u32, t: &str) -> String {
    format!("eor3 v{rd}.{t}, v{rn}.{t}, v{rm}.{t}, v{rk}.{t}")
}

fn sut_word(ops: &[Operand]) -> Result<u32, String> {
    match encode_neon_eor3(ops)? {
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
        .args(["-triple=aarch64", "-mattr=+sha3", "-show-encoding"])
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

fn invalid_t() -> impl Strategy<Value = &'static str> {
    prop::sample::select(vec!["8b", "4h", "8h", "2s", "4s", "2d", "1d"])
}

fn any_t() -> impl Strategy<Value = &'static str> {
    prop::sample::select(vec!["8b", "16b", "4h", "8h", "2s", "4s", "2d"])
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_neon_eor3_kat_llvm_mc() {
    let cases: &[(u32, u32, u32, u32, u32)] = &[
        (0, 1, 2, 3, 0xce020c20),
        (31, 31, 31, 31, 0xce1f7fff),
        (15, 16, 17, 18, 0xce114a0f),
        (0, 0, 0, 0, 0xce000000),
        (5, 10, 20, 30, 0xce147945),
    ];
    for &(rd, rn, rm, rk, want) in cases {
        let asm = eor3_asm(rd, rn, rm, rk, "16b");
        let mc = llvm_mc_word(&asm).unwrap_or_else(|e| panic!("llvm-mc KAT {asm}: {e}"));
        assert_eq!(mc, want, "llvm-mc KAT mapping broken for {asm}");
        let sut = sut_word(&eor3_ops(rd, rn, rm, rk, "16b"))
            .unwrap_or_else(|e| panic!("SUT KAT {asm}: {e}"));
        assert_eq!(sut, want, "SUT KAT mismatch for {asm}");
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_eor3_diff_llvm_mc(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        rk in reg_num(),
    ) {
        let asm = eor3_asm(rd, rn, rm, rk, "16b");
        let ops = eor3_ops(rd, rn, rm, rk, "16b");
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&ops)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for {}", asm);
    }

    #[test]
    fn encode_neon_eor3_metamorphic_rd_rn_rm_rk(
        rd1 in reg_num(),
        rd2 in reg_num(),
        rn1 in reg_num(),
        rn2 in reg_num(),
        rm1 in reg_num(),
        rm2 in reg_num(),
        rk1 in reg_num(),
        rk2 in reg_num(),
    ) {
        let w1111 = sut_word(&eor3_ops(rd1, rn1, rm1, rk1, "16b"))
            .unwrap_or_else(|e| panic!("SUT rejected rd1/rn1/rm1/rk1: {}", e));
        let w2111 = sut_word(&eor3_ops(rd2, rn1, rm1, rk1, "16b"))
            .unwrap_or_else(|e| panic!("SUT rejected rd2: {}", e));
        let w1211 = sut_word(&eor3_ops(rd1, rn2, rm1, rk1, "16b"))
            .unwrap_or_else(|e| panic!("SUT rejected rn2: {}", e));
        let w1121 = sut_word(&eor3_ops(rd1, rn1, rm2, rk1, "16b"))
            .unwrap_or_else(|e| panic!("SUT rejected rm2: {}", e));
        let w1112 = sut_word(&eor3_ops(rd1, rn1, rm1, rk2, "16b"))
            .unwrap_or_else(|e| panic!("SUT rejected rk2: {}", e));
        prop_assert_eq!(
            (w1111 ^ w2111) & !0x1Fu32,
            0u32,
            "changing only Rd must differ only in bits[4:0]"
        );
        prop_assert_eq!(w2111 & 0x1F, rd2, "Rd field");
        prop_assert_eq!(
            (w1111 ^ w1211) & !(0x1Fu32 << 5),
            0u32,
            "changing only Rn must differ only in bits[9:5]"
        );
        prop_assert_eq!((w1211 >> 5) & 0x1F, rn2, "Rn field");
        prop_assert_eq!(
            (w1111 ^ w1121) & !(0x1Fu32 << 16),
            0u32,
            "changing only Rm must differ only in bits[20:16]"
        );
        prop_assert_eq!((w1121 >> 16) & 0x1F, rm2, "Rm field");
        prop_assert_eq!(
            (w1111 ^ w1112) & !(0x1Fu32 << 10),
            0u32,
            "changing only Rk must differ only in bits[14:10]"
        );
        prop_assert_eq!((w1112 >> 10) & 0x1F, rk2, "Rk field");
    }

    #[test]
    fn encode_neon_eor3_invariant_arm_fields(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        rk in reg_num(),
    ) {
        let w = sut_word(&eor3_ops(rd, rn, rm, rk, "16b"))
            .unwrap_or_else(|e| panic!("SUT rejected: {}", e));
        prop_assert_eq!((w >> 24) & 0xFF, 0b11001110u32, "bits[31:24]=11001110");
        prop_assert_eq!((w >> 21) & 7, 0u32, "bits[23:21]=000");
        prop_assert_eq!((w >> 16) & 0x1F, rm, "Rm");
        prop_assert_eq!((w >> 15) & 1, 0u32, "bit 15 = 0");
        prop_assert_eq!((w >> 10) & 0x1F, rk, "Rk/Ra");
        prop_assert_eq!((w >> 5) & 0x1F, rn, "Rn");
        prop_assert_eq!(w & 0x1F, rd, "Rd");
    }

    #[test]
    fn encode_neon_eor3_neg_arity(
        n in 0usize..=3,
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        rk in reg_num(),
    ) {
        let all = eor3_ops(rd, rn, rm, rk, "16b");
        let arity_ops: Vec<Operand> = all.iter().take(n).cloned().collect();
        let arity_asm = match n {
            0 => "eor3".to_string(),
            1 => format!("eor3 v{rd}.16b"),
            2 => format!("eor3 v{rd}.16b, v{rn}.16b"),
            _ => format!("eor3 v{rd}.16b, v{rn}.16b, v{rm}.16b"),
        };
        prop_assert!(
            llvm_mc_word(&arity_asm).is_err(),
            "llvm-mc unexpectedly accepted arity-{} {}",
            n,
            arity_asm
        );
        prop_assert!(
            encode_neon_eor3(&arity_ops).is_err(),
            "arity {} must Err (llvm-mc rejects {})",
            n,
            arity_asm
        );
    }

    #[test]
    fn encode_neon_eor3_neg_extra_operand(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        rk in reg_num(),
        extra in reg_num(),
    ) {
        let asm = format!("{}, v{}.16b", eor3_asm(rd, rn, rm, rk, "16b"), extra);
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted 5-operand {}",
            asm
        );
        let mut ops = eor3_ops(rd, rn, rm, rk, "16b");
        ops.push(arr(extra, "16b"));
        prop_assert!(
            encode_neon_eor3(&ops).is_err(),
            "5 operands must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_eor3_neg_invalid_t(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        rk in reg_num(),
        t in invalid_t(),
    ) {
        let asm = eor3_asm(rd, rn, rm, rk, t);
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted invalid T {}",
            asm
        );
        prop_assert!(
            encode_neon_eor3(&eor3_ops(rd, rn, rm, rk, t)).is_err(),
            "T={} must Err (ARM SHA3 EOR3 is 16B only; llvm-mc rejects {})",
            t,
            asm
        );
    }

    #[test]
    fn encode_neon_eor3_neg_mismatched_t(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        rk in reg_num(),
        td in any_t(),
        tn in any_t(),
        tm in any_t(),
        tk in any_t(),
    ) {
        let ts = [td, tn, tm, tk];
        prop_assume!(ts.iter().any(|t| *t == "16b") && ts.iter().any(|t| *t != "16b"));
        let asm = format!("eor3 v{rd}.{td}, v{rn}.{tn}, v{rm}.{tm}, v{rk}.{tk}");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted mismatched T {}",
            asm
        );
        let ops = vec![arr(rd, td), arr(rn, tn), arr(rm, tm), arr(rk, tk)];
        prop_assert!(
            encode_neon_eor3(&ops).is_err(),
            "mismatched T must Err (llvm-mc/gas reject {})",
            asm
        );
    }

    #[test]
    fn encode_neon_eor3_neg_gpr_or_bare(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        rk in reg_num(),
        kind in 0u8..=4,
        fp_prefix in prop::sample::select(vec!["x", "w", "d", "s", "q", "h", "b"]),
    ) {
        let (ops, asm) = match kind {
            0 => (
                vec![
                    Operand::Reg(format!("{}{}", fp_prefix, rd)),
                    arr(rn, "16b"),
                    arr(rm, "16b"),
                    arr(rk, "16b"),
                ],
                format!("eor3 {fp_prefix}{rd}, v{rn}.16b, v{rm}.16b, v{rk}.16b"),
            ),
            1 => (
                vec![
                    arr(rd, "16b"),
                    Operand::Reg(format!("v{}", rn)),
                    arr(rm, "16b"),
                    arr(rk, "16b"),
                ],
                format!("eor3 v{rd}.16b, v{rn}, v{rm}.16b, v{rk}.16b"),
            ),
            2 => (
                vec![
                    arr(rd, "16b"),
                    arr(rn, "16b"),
                    Operand::Reg(format!("x{}", rm)),
                    arr(rk, "16b"),
                ],
                format!("eor3 v{rd}.16b, v{rn}.16b, x{rm}, v{rk}.16b"),
            ),
            3 => (
                vec![
                    Operand::Reg(format!("v{}", rd)),
                    arr(rn, "16b"),
                    arr(rm, "16b"),
                    arr(rk, "16b"),
                ],
                format!("eor3 v{rd}, v{rn}.16b, v{rm}.16b, v{rk}.16b"),
            ),
            _ => (
                vec![
                    Operand::RegArrangement {
                        reg: format!("x{}", rd),
                        arrangement: "16b".to_string(),
                    },
                    arr(rn, "16b"),
                    arr(rm, "16b"),
                    arr(rk, "16b"),
                ],
                format!("eor3 x{rd}.16b, v{rn}.16b, v{rm}.16b, v{rk}.16b"),
            ),
        };
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        prop_assert!(
            encode_neon_eor3(&ops).is_err(),
            "GPR/bare/non-arrangement kind={} must Err (llvm-mc rejects {})",
            kind,
            asm
        );
    }

    #[test]
    fn encode_neon_eor3_diff_alt_spellings(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        rk in reg_num(),
    ) {
        let asm = format!("EOR3 V{rd}.16B, V{rn}.16B, V{rm}.16B, V{rk}.16B");
        let ops = vec![
            Operand::RegArrangement {
                reg: format!("V{}", rd),
                arrangement: "16b".to_string(),
            },
            Operand::RegArrangement {
                reg: format!("V{}", rn),
                arrangement: "16b".to_string(),
            },
            Operand::RegArrangement {
                reg: format!("V{}", rm),
                arrangement: "16b".to_string(),
            },
            Operand::RegArrangement {
                reg: format!("V{}", rk),
                arrangement: "16b".to_string(),
            },
        ];
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected alt-spelling {}: {}", asm, e));
        let sut = sut_word(&ops)
            .unwrap_or_else(|e| panic!("SUT rejected alt-spelling {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for alt-spelling {}", asm);
    }

    #[test]
    fn encode_neon_eor3_neg_nonreg(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        rk in reg_num(),
        kind in 0u8..=2,
        slot in 0usize..=3,
    ) {
        let bad = match kind {
            0 => Operand::Imm(0),
            1 => Operand::Mem {
                base: format!("x{}", rd),
                offset: 0,
            },
            _ => Operand::Label("L0".into()),
        };
        let mut ops = eor3_ops(rd, rn, rm, rk, "16b");
        ops[slot] = bad;
        let asm = match (kind, slot) {
            (0, 0) => format!("eor3 #0, v{rn}.16b, v{rm}.16b, v{rk}.16b"),
            (1, 0) => format!("eor3 [x{rd}], v{rn}.16b, v{rm}.16b, v{rk}.16b"),
            (2, 0) => format!("eor3 L0, v{rn}.16b, v{rm}.16b, v{rk}.16b"),
            _ => format!("eor3 nonreg slot={slot} kind={kind}"),
        };
        if slot == 0 {
            prop_assert!(
                llvm_mc_word(&asm).is_err(),
                "llvm-mc unexpectedly accepted {}",
                asm
            );
        }
        prop_assert!(
            encode_neon_eor3(&ops).is_err(),
            "non-register operand slot={} kind={} must Err",
            slot,
            kind
        );
    }
}

/// Deterministic regression: extra operand ignored (from neg_extra_operand).
#[test]
fn test_encode_neon_eor3_regression_extra_operand() {
    let mut ops = eor3_ops(0, 0, 0, 0, "16b");
    ops.push(arr(0, "16b"));
    assert!(
        encode_neon_eor3(&ops).is_err(),
        "eor3 v0.16b, v0.16b, v0.16b, v0.16b, v0.16b must Err (gas/llvm-mc reject a fifth operand)"
    );
}

/// Deterministic regression: .8b arrangement encoded (from neg_invalid_t).
#[test]
fn test_encode_neon_eor3_regression_invalid_t() {
    assert!(
        encode_neon_eor3(&eor3_ops(0, 0, 0, 0, "8b")).is_err(),
        "eor3 v0.8b, v0.8b, v0.8b, v0.8b must Err (ARM SHA3 EOR3 is 16B only; gas/llvm-mc reject)"
    );
}

/// Deterministic regression: mismatched arrangements ignored (from neg_mismatched_t).
#[test]
fn test_encode_neon_eor3_regression_mismatched_t() {
    let ops = vec![arr(0, "16b"), arr(0, "16b"), arr(0, "16b"), arr(0, "8b")];
    assert!(
        encode_neon_eor3(&ops).is_err(),
        "eor3 v0.16b, v0.16b, v0.16b, v0.8b must Err (gas/llvm-mc require matching .16B)"
    );
}

/// Deterministic regression: GPR dest encoded (from neg_gpr_or_bare).
#[test]
fn test_encode_neon_eor3_regression_gpr_dest() {
    let ops = vec![
        Operand::Reg("x0".into()),
        arr(0, "16b"),
        arr(0, "16b"),
        arr(0, "16b"),
    ];
    assert!(
        encode_neon_eor3(&ops).is_err(),
        "eor3 x0, v0.16b, v0.16b, v0.16b must Err (gas/llvm-mc require Vd.16B)"
    );
}
