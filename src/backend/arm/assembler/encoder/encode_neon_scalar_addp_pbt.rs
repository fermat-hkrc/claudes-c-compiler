// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md:237 NEON scalar lists addp (scalar);
//   encoder/mod.rs:1-7 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:641-646 "addp" => encode_neon_scalar_addp when dest is Dd and arity==2;
//   neon.rs:1801 NEON scalar ADDP: addp Dd, Vn.2d;
//   neon.rs:1812 Scalar ADDP: 01 0 11110 11 11000 11011 10 Rn Rd;
//   ARM ARM Advanced SIMD scalar pairwise ADDP: dest Dd only, source Vn.2D only.
// Stronger considered:
//   - State machine: rejected — encode_neon_scalar_addp is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree scalar ADDP decoder
//   - Differential vs encode_neon_three_same / encode_neon_faddp: rejected — same-job gate
//     (vector ADDP Vd.T and float FADDP are different opcodes)
// Weaker available: algebraic.metamorphic (Rd/Rn fields), algebraic.invariant (word layout),
//   negative_error (arity / extra / wrong register class / bad arrangement / non-register)
// Differential: candidate=encode_neon_scalar_addp, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[Reg(Dd), RegArrangement(Vn,"2d")] <-> `addp Dd, Vn.2d`

use super::encode_neon_scalar_addp;
use super::EncodeResult;
use crate::backend::arm::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;
const ARM_SISD_ADDP: u32 = 0x5ef1b800;

fn cfg() -> ProptestConfig {
    ProptestConfig::with_cases(CASES)
}

fn dreg(n: u32) -> Operand {
    Operand::Reg(format!("d{}", n))
}

fn v2d(n: u32) -> Operand {
    Operand::RegArrangement {
        reg: format!("v{}", n),
        arrangement: "2d".into(),
    }
}

fn ops_ok(rd: u32, rn: u32) -> Vec<Operand> {
    vec![dreg(rd), v2d(rn)]
}

fn asm_ok(rd: u32, rn: u32) -> String {
    format!("addp d{rd}, v{rn}.2d")
}

fn sut_word(ops: &[Operand]) -> Result<u32, String> {
    match encode_neon_scalar_addp(ops)? {
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

fn non_d_pfx() -> impl Strategy<Value = &'static str> {
    prop::sample::select(vec![
        "s", "h", "b", "x", "w", "q", "v", "sp", "xzr", "wsp", "wzr", "lr",
    ])
}

fn non_v_pfx() -> impl Strategy<Value = &'static str> {
    prop::sample::select(vec!["s", "h", "b", "x", "w", "q", "d"])
}

fn bad_arr() -> impl Strategy<Value = &'static str> {
    prop::sample::select(vec!["8b", "16b", "4h", "8h", "2s", "4s", "1d"])
}

fn named_reg(pfx: &str, n: u32) -> (Operand, String) {
    match pfx {
        "sp" | "wsp" | "xzr" | "wzr" | "lr" => (Operand::Reg(pfx.to_string()), pfx.to_string()),
        _ => {
            let name = format!("{pfx}{n}");
            (Operand::Reg(name.clone()), name)
        }
    }
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_neon_scalar_addp_kat_llvm_mc() {
    let kat: &[(&str, u32)] = &[
        ("addp d0, v1.2d", 0x5ef1b820),
        ("addp d31, v31.2d", 0x5ef1bbff),
        ("addp d0, v0.2d", 0x5ef1b800),
        ("addp d15, v16.2d", 0x5ef1ba0f),
        ("addp D0, V1.2D", 0x5ef1b820),
        ("ADDP d0, v1.2d", 0x5ef1b820),
    ];
    for &(asm, want) in kat {
        let mc = llvm_mc_word(asm).unwrap_or_else(|e| panic!("llvm-mc KAT {asm}: {e}"));
        assert_eq!(mc, want, "llvm-mc KAT mapping broken for {asm}");
    }
    let sut_kat: &[(u32, u32, u32)] = &[
        (0, 1, 0x5ef1b820),
        (31, 31, 0x5ef1bbff),
        (0, 0, 0x5ef1b800),
        (15, 16, 0x5ef1ba0f),
    ];
    for &(rd, rn, want) in sut_kat {
        let asm = asm_ok(rd, rn);
        let sut = sut_word(&ops_ok(rd, rn)).unwrap_or_else(|e| panic!("SUT KAT {asm}: {e}"));
        assert_eq!(sut, want, "SUT KAT mismatch for {asm}");
    }
}

proptest! {
    #![proptest_config(cfg())]

    // Oracle: differential — llvm-mc AArch64 assembler
    #[test]
    fn encode_neon_scalar_addp_diff_llvm_mc(
        rd in reg_num(),
        rn in reg_num(),
    ) {
        let asm = asm_ok(rd, rn);
        let ops = ops_ok(rd, rn);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&ops)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for {}", asm);
    }

    // Oracle: algebraic.metamorphic — Rd/Rn field isolation
    #[test]
    fn encode_neon_scalar_addp_meta_rd_rn(
        rd1 in reg_num(),
        rd2 in reg_num(),
        rn1 in reg_num(),
        rn2 in reg_num(),
    ) {
        let w11 = sut_word(&ops_ok(rd1, rn1))
            .unwrap_or_else(|e| panic!("SUT rejected rd1/rn1: {}", e));
        let w21 = sut_word(&ops_ok(rd2, rn1))
            .unwrap_or_else(|e| panic!("SUT rejected rd2: {}", e));
        let w12 = sut_word(&ops_ok(rd1, rn2))
            .unwrap_or_else(|e| panic!("SUT rejected rn2: {}", e));
        prop_assert_eq!(
            (w11 ^ w21) & !0x1Fu32,
            0u32,
            "changing only Rd must differ only in bits[4:0]"
        );
        prop_assert_eq!(w21 & 0x1F, rd2, "Rd field");
        prop_assert_eq!(
            (w11 ^ w12) & !(0x1Fu32 << 5),
            0u32,
            "changing only Rn must differ only in bits[9:5]"
        );
        prop_assert_eq!((w12 >> 5) & 0x1F, rn2, "Rn field");
    }

    // Oracle: algebraic.invariant — ARM SISD ADDP field layout
    #[test]
    fn encode_neon_scalar_addp_invariant_arm_fields(
        rd in reg_num(),
        rn in reg_num(),
    ) {
        let w = sut_word(&ops_ok(rd, rn))
            .unwrap_or_else(|e| panic!("SUT rejected: {}", e));
        prop_assert_eq!((w >> 30) & 3, 0b01u32, "bits[31:30]=01");
        prop_assert_eq!((w >> 29) & 1, 0u32, "bit29=0");
        prop_assert_eq!((w >> 24) & 0x1F, 0b11110u32, "bits[28:24]=11110");
        prop_assert_eq!((w >> 22) & 3, 0b11u32, "bits[23:22]=11");
        prop_assert_eq!((w >> 17) & 0x1F, 0b11000u32, "bits[21:17]=11000");
        prop_assert_eq!((w >> 12) & 0x1F, 0b11011u32, "bits[16:12]=11011");
        prop_assert_eq!((w >> 10) & 3, 0b10u32, "bits[11:10]=10");
        prop_assert_eq!((w >> 5) & 0x1F, rn, "Rn");
        prop_assert_eq!(w & 0x1F, rd, "Rd");
        prop_assert_eq!(w, ARM_SISD_ADDP | (rn << 5) | rd, "template | Rn | Rd");
    }

    // Oracle: negative_error — arity 0..=1
    #[test]
    fn encode_neon_scalar_addp_neg_arity(
        n in 0usize..=1,
        rd in reg_num(),
        rn in reg_num(),
    ) {
        let all = ops_ok(rd, rn);
        let arity_ops: Vec<Operand> = all.iter().take(n).cloned().collect();
        let arity_asm = match n {
            0 => "addp".to_string(),
            _ => format!("addp d{rd}"),
        };
        prop_assert!(
            llvm_mc_word(&arity_asm).is_err(),
            "llvm-mc unexpectedly accepted arity-{} {}",
            n,
            arity_asm
        );
        prop_assert!(
            encode_neon_scalar_addp(&arity_ops).is_err(),
            "arity {} must Err (llvm-mc rejects {})",
            n,
            arity_asm
        );
    }

    // Oracle: negative_error — third operand
    #[test]
    fn encode_neon_scalar_addp_neg_extra_operand(
        rd in reg_num(),
        rn in reg_num(),
        extra in reg_num(),
    ) {
        let asm = format!("{}, d{}", asm_ok(rd, rn), extra);
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted 3-operand {}",
            asm
        );
        let mut ops = ops_ok(rd, rn);
        ops.push(dreg(extra));
        prop_assert!(
            encode_neon_scalar_addp(&ops).is_err(),
            "3 operands must Err (llvm-mc rejects {})",
            asm
        );
    }

    // Oracle: negative_error — non-D dest / non-V source / bare source
    #[test]
    fn encode_neon_scalar_addp_neg_wrong_reg_class(
        rd in reg_num(),
        rn in reg_num(),
        which in 0u8..=2,
        dest_pfx in non_d_pfx(),
        src_pfx in non_v_pfx(),
    ) {
        let (ops, asm) = match which {
            0 => {
                let (bad, bad_name) = named_reg(dest_pfx, rd);
                let ops = vec![bad, v2d(rn)];
                let asm = format!("addp {bad_name}, v{rn}.2d");
                (ops, asm)
            }
            1 => {
                let src_name = format!("{src_pfx}{rn}");
                let ops = vec![
                    dreg(rd),
                    Operand::RegArrangement {
                        reg: src_name.clone(),
                        arrangement: "2d".into(),
                    },
                ];
                let asm = format!("addp d{rd}, {src_name}.2d");
                (ops, asm)
            }
            _ => {
                let ops = vec![dreg(rd), Operand::Reg(format!("v{rn}"))];
                let asm = format!("addp d{rd}, v{rn}");
                (ops, asm)
            }
        };
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        prop_assert!(
            encode_neon_scalar_addp(&ops).is_err(),
            "wrong register class which={} must Err (llvm-mc rejects {})",
            which,
            asm
        );
    }

    // Oracle: negative_error — source arrangement ≠ 2d
    #[test]
    fn encode_neon_scalar_addp_neg_bad_arrangement(
        rd in reg_num(),
        rn in reg_num(),
        arr in bad_arr(),
    ) {
        let asm = format!("addp d{rd}, v{rn}.{arr}");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let ops = vec![
            dreg(rd),
            Operand::RegArrangement {
                reg: format!("v{rn}"),
                arrangement: arr.to_string(),
            },
        ];
        prop_assert!(
            encode_neon_scalar_addp(&ops).is_err(),
            "arrangement .{} must Err (llvm-mc rejects {})",
            arr,
            asm
        );
    }

    // Oracle: negative_error — Imm/Mem/Label in either slot
    #[test]
    fn encode_neon_scalar_addp_neg_nonreg(
        rd in reg_num(),
        rn in reg_num(),
        kind in 0u8..=2,
        slot in 0usize..=1,
    ) {
        let bad = match kind {
            0 => Operand::Imm(0),
            1 => Operand::Mem {
                base: format!("x{}", rd),
                offset: 0,
            },
            _ => Operand::Label("L0".into()),
        };
        let mut ops = ops_ok(rd, rn);
        ops[slot] = bad;
        let asm = match (kind, slot) {
            (0, 0) => format!("addp #0, v{rn}.2d"),
            (1, 0) => format!("addp [x{rd}], v{rn}.2d"),
            (2, 0) => format!("addp L0, v{rn}.2d"),
            (0, 1) => format!("addp d{rd}, #0"),
            (1, 1) => format!("addp d{rd}, [x{rd}]"),
            _ => format!("addp d{rd}, L0"),
        };
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        prop_assert!(
            encode_neon_scalar_addp(&ops).is_err(),
            "non-register operand slot={} kind={} must Err (llvm-mc rejects {})",
            slot,
            kind,
            asm
        );
    }

    // Oracle: differential — uppercase D/V / mnemonic alt-spellings vs llvm-mc
    #[test]
    fn encode_neon_scalar_addp_diff_alt_spellings(
        rd in reg_num(),
        rn in reg_num(),
    ) {
        let asm = format!("ADDP D{rd}, V{rn}.2D");
        let ops = vec![
            Operand::Reg(format!("D{rd}")),
            Operand::RegArrangement {
                reg: format!("V{rn}"),
                arrangement: "2d".into(),
            },
        ];
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected alt-spelling {}: {}", asm, e));
        let sut = sut_word(&ops)
            .unwrap_or_else(|e| panic!("SUT rejected alt-spelling {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for alt-spelling {}", asm);
    }
}

/// Deterministic regression: extra operand ignored (from neg_extra_operand).
#[test]
fn test_encode_neon_scalar_addp_regression_extra_operand() {
    let mut ops = ops_ok(0, 0);
    ops.push(dreg(0));
    assert!(
        encode_neon_scalar_addp(&ops).is_err(),
        "addp d0, v0.2d, d0 must Err (gas/llvm-mc reject a third operand)"
    );
}

/// Deterministic regression: non-D dest encoded (from neg_wrong_reg_class which=0).
#[test]
fn test_encode_neon_scalar_addp_regression_wrong_dest_class() {
    let ops = vec![Operand::Reg("s0".into()), v2d(0)];
    assert!(
        encode_neon_scalar_addp(&ops).is_err(),
        "addp s0, v0.2d must Err (ARM/gas/llvm-mc require Dd dest)"
    );
}

/// Deterministic regression: non-V source prefix encoded (from neg_wrong_reg_class which=1).
#[test]
fn test_encode_neon_scalar_addp_regression_src_prefix() {
    let ops = vec![
        dreg(0),
        Operand::RegArrangement {
            reg: "x0".into(),
            arrangement: "2d".into(),
        },
    ];
    assert!(
        encode_neon_scalar_addp(&ops).is_err(),
        "addp d0, x0.2d must Err (ARM/gas/llvm-mc require Vn.2d source)"
    );
}
