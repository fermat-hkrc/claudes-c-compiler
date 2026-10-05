// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md:233 NEON permute lists zip1, zip2, uzp1, uzp2, trn1, trn2;
//   encoder/mod.rs:1-7 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:682-683 "trn1"/"trn2" => encode_neon_zip_uzp(operands, 0b010/0b110, false);
//   encoder/mod.rs:770-773 "uzp1"/"uzp2"/"zip1"/"zip2" => encode_neon_zip_uzp;
//   neon.rs:1093 Encode NEON UZP1/UZP2/ZIP1/ZIP2;
//   neon.rs:1099-1102 UZP1/UZP2/ZIP1/ZIP2: 0 Q 0 01110 size 0 Rm 0 opc 10 Rn Rd;
//   ARM ARM Advanced SIMD permute (ZIP/UZP/TRN). T in {8B,16B,4H,8H,2S,4S,2D},
//   matching arrangements, size:Q=11:0 (1D) reserved.
// Stronger considered:
//   - State machine: rejected — encode_neon_zip_uzp is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree ZIP/UZP/TRN decoder
//   - Differential vs encode_neon_ext / encode_neon_tbl / encode_neon_tbx: rejected — same-job gate
//     (EXT extract with imm4, TBL/TBX table lookup vs ZIP/UZP/TRN permute 0 Q 0 01110)
// Weaker available: algebraic.metamorphic (Rd/Rn/Rm fields), algebraic.invariant (word layout),
//   negative_error (arity / extra / reserved 1d / mismatch / GPR dest)
// Differential: candidate=encode_neon_zip_uzp, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[RegArrangement(Vd,T), RegArrangement(Vn,T), RegArrangement(Vm,T)]
//     <-> `{zip1|zip2|uzp1|uzp2|trn1|trn2} Vd.T, Vn.T, Vm.T`

use super::encode_neon_zip_uzp;
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

fn zip_ops(rd: u32, rn: u32, rm: u32, t: &str) -> Vec<Operand> {
    vec![arr(rd, t), arr(rn, t), arr(rm, t)]
}

fn zip_asm(mnemonic: &str, rd: u32, rn: u32, rm: u32, t: &str) -> String {
    format!("{mnemonic} v{rd}.{t}, v{rn}.{t}, v{rm}.{t}")
}

fn opc(mnemonic: &str) -> u32 {
    match mnemonic {
        "uzp1" => 0b001,
        "trn1" => 0b010,
        "zip1" => 0b011,
        "uzp2" => 0b101,
        "trn2" => 0b110,
        "zip2" => 0b111,
        _ => panic!("unknown mnemonic {mnemonic}"),
    }
}

fn q_size(t: &str) -> (u32, u32) {
    match t {
        "8b" => (0, 0b00),
        "16b" => (1, 0b00),
        "4h" => (0, 0b01),
        "8h" => (1, 0b01),
        "2s" => (0, 0b10),
        "4s" => (1, 0b10),
        "2d" => (1, 0b11),
        _ => panic!("unknown T {t}"),
    }
}

fn sut_word(ops: &[Operand], op_bits: u32) -> Result<u32, String> {
    match encode_neon_zip_uzp(ops, op_bits, false)? {
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

fn t_size() -> impl Strategy<Value = &'static str> {
    prop::sample::select(vec!["8b", "16b", "4h", "8h", "2s", "4s", "2d"])
}

fn mnemonic() -> impl Strategy<Value = &'static str> {
    prop::sample::select(vec!["zip1", "zip2", "uzp1", "uzp2", "trn1", "trn2"])
}

fn opc_bits() -> impl Strategy<Value = u32> {
    prop::sample::select(vec![0b001u32, 0b010, 0b011, 0b101, 0b110, 0b111])
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_neon_zip_uzp_kat_llvm_mc() {
    let cases: &[(&str, u32, u32, u32, &str, u32)] = &[
        ("zip1", 0, 1, 2, "8b", 0x0e023820),
        ("zip1", 0, 1, 2, "16b", 0x4e023820),
        ("zip1", 0, 1, 2, "4h", 0x0e423820),
        ("zip1", 0, 1, 2, "8h", 0x4e423820),
        ("zip1", 0, 1, 2, "2s", 0x0e823820),
        ("zip1", 0, 1, 2, "4s", 0x4e823820),
        ("zip1", 0, 1, 2, "2d", 0x4ec23820),
        ("zip2", 0, 1, 2, "8b", 0x0e027820),
        ("uzp1", 0, 1, 2, "8b", 0x0e021820),
        ("uzp2", 0, 1, 2, "8b", 0x0e025820),
        ("trn1", 0, 1, 2, "8b", 0x0e022820),
        ("trn2", 0, 1, 2, "8b", 0x0e026820),
        ("zip1", 31, 30, 29, "16b", 0x4e1d3bdf),
        ("zip2", 31, 0, 1, "4s", 0x4e81781f),
        ("uzp1", 15, 16, 17, "2d", 0x4ed11a0f),
    ];
    for &(m, rd, rn, rm, t, want) in cases {
        let asm = zip_asm(m, rd, rn, rm, t);
        let mc = llvm_mc_word(&asm).unwrap_or_else(|e| panic!("llvm-mc KAT {asm}: {e}"));
        assert_eq!(mc, want, "llvm-mc KAT mapping broken for {asm}");
        let sut = sut_word(&zip_ops(rd, rn, rm, t), opc(m))
            .unwrap_or_else(|e| panic!("SUT KAT {asm}: {e}"));
        assert_eq!(sut, want, "SUT KAT mismatch for {asm}");
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_zip_uzp_diff_llvm_mc(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        t in t_size(),
        m in mnemonic(),
    ) {
        let asm = zip_asm(m, rd, rn, rm, t);
        let ops = zip_ops(rd, rn, rm, t);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&ops, opc(m))
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for {}", asm);
    }

    #[test]
    fn encode_neon_zip_uzp_metamorphic_rd_rn_rm(
        rd1 in reg_num(),
        rd2 in reg_num(),
        rn1 in reg_num(),
        rn2 in reg_num(),
        rm1 in reg_num(),
        rm2 in reg_num(),
        t in t_size(),
        op_bits in opc_bits(),
    ) {
        let w111 = sut_word(&zip_ops(rd1, rn1, rm1, t), op_bits)
            .unwrap_or_else(|e| panic!("SUT rejected rd1/rn1/rm1: {}", e));
        let w211 = sut_word(&zip_ops(rd2, rn1, rm1, t), op_bits)
            .unwrap_or_else(|e| panic!("SUT rejected rd2: {}", e));
        let w121 = sut_word(&zip_ops(rd1, rn2, rm1, t), op_bits)
            .unwrap_or_else(|e| panic!("SUT rejected rn2: {}", e));
        let w112 = sut_word(&zip_ops(rd1, rn1, rm2, t), op_bits)
            .unwrap_or_else(|e| panic!("SUT rejected rm2: {}", e));
        prop_assert_eq!(
            (w111 ^ w211) & !0x1Fu32,
            0u32,
            "changing only Rd must differ only in bits[4:0]"
        );
        prop_assert_eq!(w211 & 0x1F, rd2, "Rd field");
        prop_assert_eq!(
            (w111 ^ w121) & !(0x1Fu32 << 5),
            0u32,
            "changing only Rn must differ only in bits[9:5]"
        );
        prop_assert_eq!((w121 >> 5) & 0x1F, rn2, "Rn field");
        prop_assert_eq!(
            (w111 ^ w112) & !(0x1Fu32 << 16),
            0u32,
            "changing only Rm must differ only in bits[20:16]"
        );
        prop_assert_eq!((w112 >> 16) & 0x1F, rm2, "Rm field");
    }

    #[test]
    fn encode_neon_zip_uzp_invariant_arm_fields(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        t in t_size(),
        op_bits in opc_bits(),
    ) {
        let w = sut_word(&zip_ops(rd, rn, rm, t), op_bits)
            .unwrap_or_else(|e| panic!("SUT rejected: {}", e));
        let (q, size) = q_size(t);
        prop_assert_eq!((w >> 31) & 1, 0u32, "bit 31 must be 0");
        prop_assert_eq!((w >> 30) & 1, q, "Q/bit30");
        prop_assert_eq!((w >> 24) & 0x3F, 0b001110u32, "bits[29:24]=001110");
        prop_assert_eq!((w >> 22) & 3, size, "size bits[23:22]");
        prop_assert_eq!((w >> 21) & 1, 0u32, "bit 21 = 0");
        prop_assert_eq!((w >> 16) & 0x1F, rm, "Rm");
        prop_assert_eq!((w >> 15) & 1, 0u32, "bit 15 = 0");
        prop_assert_eq!((w >> 12) & 7, op_bits, "opc bits[14:12]");
        prop_assert_eq!((w >> 10) & 3, 0b10u32, "bits[11:10]=10");
        prop_assert_eq!((w >> 5) & 0x1F, rn, "Rn");
        prop_assert_eq!(w & 0x1F, rd, "Rd");
    }

    #[test]
    fn encode_neon_zip_uzp_neg_arity(
        n in 0usize..=2,
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        t in t_size(),
        m in mnemonic(),
    ) {
        let all = zip_ops(rd, rn, rm, t);
        let arity_ops: Vec<Operand> = all.iter().take(n).cloned().collect();
        let arity_asm = match n {
            0 => m.to_string(),
            1 => format!("{m} v{rd}.{t}"),
            _ => format!("{m} v{rd}.{t}, v{rn}.{t}"),
        };
        prop_assert!(
            llvm_mc_word(&arity_asm).is_err(),
            "llvm-mc unexpectedly accepted arity-{} {}",
            n,
            arity_asm
        );
        prop_assert!(
            encode_neon_zip_uzp(&arity_ops, opc(m), false).is_err(),
            "arity {} must Err (llvm-mc rejects {})",
            n,
            arity_asm
        );
    }

    #[test]
    fn encode_neon_zip_uzp_neg_extra_operand(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        extra in reg_num(),
        t in t_size(),
        m in mnemonic(),
    ) {
        let asm = format!("{}, v{}.{}", zip_asm(m, rd, rn, rm, t), extra, t);
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted 4-operand {}",
            asm
        );
        let mut ops = zip_ops(rd, rn, rm, t);
        ops.push(arr(extra, t));
        prop_assert!(
            encode_neon_zip_uzp(&ops, opc(m), false).is_err(),
            "4 operands must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_zip_uzp_neg_reserved_1d(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        m in mnemonic(),
    ) {
        let asm = zip_asm(m, rd, rn, rm, "1d");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted reserved 1d {}",
            asm
        );
        prop_assert!(
            encode_neon_zip_uzp(&zip_ops(rd, rn, rm, "1d"), opc(m), false).is_err(),
            "reserved 1d must Err (llvm-mc/gas reject {})",
            asm
        );
    }

    #[test]
    fn encode_neon_zip_uzp_neg_mismatched_t(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        td in t_size(),
        tn in t_size(),
        tm in t_size(),
        m in mnemonic(),
    ) {
        prop_assume!(td != tn || td != tm);
        let asm = format!("{m} v{rd}.{td}, v{rn}.{tn}, v{rm}.{tm}");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted mismatched T {}",
            asm
        );
        let ops = vec![arr(rd, td), arr(rn, tn), arr(rm, tm)];
        prop_assert!(
            encode_neon_zip_uzp(&ops, opc(m), false).is_err(),
            "mismatched T must Err (llvm-mc/gas reject {})",
            asm
        );
    }

    #[test]
    fn encode_neon_zip_uzp_neg_gpr_or_bare(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        t in t_size(),
        m in mnemonic(),
        kind in 0u8..=4,
        fp_prefix in prop::sample::select(vec!["x", "w", "d", "s", "q", "h", "b"]),
    ) {
        let (ops, asm) = match kind {
            0 => (
                vec![
                    Operand::Reg(format!("{}{}", fp_prefix, rd)),
                    arr(rn, t),
                    arr(rm, t),
                ],
                format!("{m} {fp_prefix}{rd}, v{rn}.{t}, v{rm}.{t}"),
            ),
            1 => (
                vec![
                    arr(rd, t),
                    Operand::Reg(format!("v{}", rn)),
                    arr(rm, t),
                ],
                format!("{m} v{rd}.{t}, v{rn}, v{rm}.{t}"),
            ),
            2 => (
                vec![
                    arr(rd, t),
                    arr(rn, t),
                    Operand::Reg(format!("x{}", rm)),
                ],
                format!("{m} v{rd}.{t}, v{rn}.{t}, x{rm}"),
            ),
            3 => (
                vec![
                    Operand::Reg(format!("v{}", rd)),
                    arr(rn, t),
                    arr(rm, t),
                ],
                format!("{m} v{rd}, v{rn}.{t}, v{rm}.{t}"),
            ),
            _ => (
                vec![
                    Operand::RegArrangement {
                        reg: format!("x{}", rd),
                        arrangement: t.to_string(),
                    },
                    arr(rn, t),
                    arr(rm, t),
                ],
                format!("{m} x{rd}.{t}, v{rn}.{t}, v{rm}.{t}"),
            ),
        };
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        prop_assert!(
            encode_neon_zip_uzp(&ops, opc(m), false).is_err(),
            "GPR/bare/non-arrangement kind={} must Err (llvm-mc rejects {})",
            kind,
            asm
        );
    }

    #[test]
    fn encode_neon_zip_uzp_diff_alt_spellings(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        t in t_size(),
        m in mnemonic(),
    ) {
        let t_up = t.to_uppercase();
        let asm = format!("{m} V{rd}.{t_up}, V{rn}.{t_up}, V{rm}.{t_up}");
        let ops = vec![
            Operand::RegArrangement {
                reg: format!("V{}", rd),
                arrangement: t.to_string(),
            },
            Operand::RegArrangement {
                reg: format!("V{}", rn),
                arrangement: t.to_string(),
            },
            Operand::RegArrangement {
                reg: format!("V{}", rm),
                arrangement: t.to_string(),
            },
        ];
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected alt-spelling {}: {}", asm, e));
        let sut = sut_word(&ops, opc(m))
            .unwrap_or_else(|e| panic!("SUT rejected alt-spelling {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for alt-spelling {}", asm);
    }
}

/// Deterministic regression: extra operand ignored (from neg_extra_operand).
#[test]
fn test_encode_neon_zip_uzp_regression_extra_operand() {
    let mut ops = zip_ops(0, 0, 0, "8b");
    ops.push(arr(0, "8b"));
    assert!(
        encode_neon_zip_uzp(&ops, opc("zip1"), false).is_err(),
        "zip1 v0.8b, v0.8b, v0.8b, v0.8b must Err (gas/llvm-mc reject a fourth operand)"
    );
}

/// Deterministic regression: reserved 1D arrangement encoded (from neg_reserved_1d).
#[test]
fn test_encode_neon_zip_uzp_regression_reserved_1d() {
    assert!(
        encode_neon_zip_uzp(&zip_ops(0, 0, 0, "1d"), opc("zip1"), false).is_err(),
        "zip1 v0.1d, v0.1d, v0.1d must Err (ARM size:Q=11:0 reserved; gas/llvm-mc reject)"
    );
}

/// Deterministic regression: mismatched arrangements ignored (from neg_mismatched_t).
#[test]
fn test_encode_neon_zip_uzp_regression_mismatched_t() {
    let ops = vec![arr(0, "8b"), arr(0, "8b"), arr(0, "16b")];
    assert!(
        encode_neon_zip_uzp(&ops, opc("zip1"), false).is_err(),
        "zip1 v0.8b, v0.8b, v0.16b must Err (gas/llvm-mc require matching T)"
    );
}

/// Deterministic regression: bare V source encoded (from neg_gpr_or_bare).
#[test]
fn test_encode_neon_zip_uzp_regression_bare_src() {
    let ops = vec![arr(0, "8b"), Operand::Reg("v0".into()), arr(0, "8b")];
    assert!(
        encode_neon_zip_uzp(&ops, opc("zip1"), false).is_err(),
        "zip1 v0.8b, v0, v0.8b must Err (gas/llvm-mc require Vn.T)"
    );
}
