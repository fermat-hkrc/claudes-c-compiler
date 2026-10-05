// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md:224 NEON three-same lists cmeq, cmhi, cmhs, cmge, cmgt, cmtst, sqadd, uqadd, sqsub, uqsub, sshl, ushl, …;
//   encoder/mod.rs:1-7 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:567-638 cmeq/cmhi/…/addp => encode_neon_three_same;
//   neon.rs:58 Encode NEON three-same-register instructions: CMEQ, UQSUB, SQSUB, CMHI, etc.;
//   neon.rs:60 Layout: 0 Q U 01110 size 1 Rm opcode 1 Rn Rd;
//   ARM ARM Advanced SIMD three-same T in {8B,16B,4H,8H,2S,4S,2D}; size:Q=11:0 reserved.
// Stronger considered:
//   - State machine: rejected — encode_neon_three_same is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree three-same decoder
//   - Differential vs encode_neon_add_sub / encode_neon_mul: rejected — independence / same-job gate
//     (shared get_neon_reg / neon_arr_to_q_size; MUL is a single hardcoded opcode)
// Weaker available: algebraic.metamorphic (U / opcode / Q), algebraic.invariant (word layout),
//   negative_error (arity / extra / reserved 1d / mismatch / GPR dest)
// Differential: candidate=encode_neon_three_same, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[RegArrangement(Vd,T), RegArrangement(Vn,T), RegArrangement(Vm,T)] + (U, opcode)
//     <-> `mnemonic Vd.T, Vn.T, Vm.T` for T in {8b,16b,4h,8h,2s,4s,2d}

use super::encode_neon_three_same;
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

/// Integer three-same mnemonics whose ARM T includes 8B..2D (not size=11 reserved).
fn insn_table() -> impl Strategy<Value = (u32, u32, &'static str)> {
    prop::sample::select(vec![
        (1u32, 0b10001u32, "cmeq"),
        (1u32, 0b00110u32, "cmhi"),
        (1u32, 0b00111u32, "cmhs"),
        (0u32, 0b00111u32, "cmge"),
        (0u32, 0b00110u32, "cmgt"),
        (0u32, 0b10001u32, "cmtst"),
        (0u32, 0b00001u32, "sqadd"),
        (1u32, 0b00001u32, "uqadd"),
        (0u32, 0b00101u32, "sqsub"),
        (1u32, 0b00101u32, "uqsub"),
        (0u32, 0b01000u32, "sshl"),
        (1u32, 0b01000u32, "ushl"),
        (0u32, 0b01001u32, "sqshl"),
        (1u32, 0b01001u32, "uqshl"),
        (0u32, 0b01010u32, "srshl"),
        (1u32, 0b01010u32, "urshl"),
        (0u32, 0b01011u32, "sqrshl"),
        (1u32, 0b01011u32, "uqrshl"),
        (0u32, 0b10111u32, "addp"),
    ])
}

fn ops_t(rd: u32, rn: u32, rm: u32, t: &str) -> Vec<Operand> {
    vec![arr(rd, t), arr(rn, t), arr(rm, t)]
}

fn asm_t(rd: u32, rn: u32, rm: u32, t: &str, mnemonic: &str) -> String {
    format!("{mnemonic} v{rd}.{t}, v{rn}.{t}, v{rm}.{t}")
}

fn sut_word(ops: &[Operand], u_bit: u32, opcode: u32) -> Result<u32, String> {
    match encode_neon_three_same(ops, u_bit, opcode)? {
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
    prop::sample::select(vec!["8b", "16b", "4h", "8h", "2s", "4s", "2d"])
}

fn any_t() -> impl Strategy<Value = &'static str> {
    prop::sample::select(vec![
        "8b", "16b", "4h", "8h", "2s", "4s", "1d", "2d", "1q",
    ])
}

fn is_valid_three_same_t(t: &str) -> bool {
    matches!(t, "8b" | "16b" | "4h" | "8h" | "2s" | "4s" | "2d")
}

fn q_size_of(t: &str) -> (u32, u32) {
    match t {
        "8b" => (0, 0b00),
        "16b" => (1, 0b00),
        "4h" => (0, 0b01),
        "8h" => (1, 0b01),
        "2s" => (0, 0b10),
        "4s" => (1, 0b10),
        "2d" => (1, 0b11),
        "1d" => (0, 0b11),
        _ => (0, 0),
    }
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_neon_three_same_kat_llvm_mc() {
    let kat: &[(&str, u32)] = &[
        ("cmeq v0.8b, v1.8b, v2.8b", 0x2e228c20),
        ("cmeq v0.16b, v1.16b, v2.16b", 0x6e228c20),
        ("cmeq v0.2d, v1.2d, v2.2d", 0x6ee28c20),
        ("cmhi v0.8b, v1.8b, v2.8b", 0x2e223420),
        ("cmgt v0.4s, v1.4s, v2.4s", 0x4ea23420),
        ("uqsub v31.8h, v30.8h, v29.8h", 0x6e7d2fdf),
        ("sqadd v0.2d, v1.2d, v2.2d", 0x4ee20c20),
        ("sshl v15.4h, v16.4h, v17.4h", 0x0e71460f),
    ];
    for &(asm, want) in kat {
        let mc = llvm_mc_word(asm).unwrap_or_else(|e| panic!("llvm-mc KAT {asm}: {e}"));
        assert_eq!(mc, want, "llvm-mc KAT mapping broken for {asm}");
    }
    let sut_kat: &[(u32, u32, u32, &str, u32, u32, u32)] = &[
        (0, 1, 2, "8b", 1, 0b10001, 0x2e228c20),
        (0, 1, 2, "16b", 1, 0b10001, 0x6e228c20),
        (0, 1, 2, "2d", 1, 0b10001, 0x6ee28c20),
        (0, 1, 2, "8b", 1, 0b00110, 0x2e223420),
        (0, 1, 2, "4s", 0, 0b00110, 0x4ea23420),
        (31, 30, 29, "8h", 1, 0b00101, 0x6e7d2fdf),
        (0, 1, 2, "2d", 0, 0b00001, 0x4ee20c20),
        (15, 16, 17, "4h", 0, 0b01000, 0x0e71460f),
    ];
    for &(rd, rn, rm, t, u, opcode, want) in sut_kat {
        let asm = format!("rd={rd} rn={rn} rm={rm} t={t} u={u} opc={opcode}");
        let sut = sut_word(&ops_t(rd, rn, rm, t), u, opcode)
            .unwrap_or_else(|e| panic!("SUT KAT {asm}: {e}"));
        assert_eq!(sut, want, "SUT KAT mismatch for {asm}");
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_three_same_diff_llvm_mc(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        t in valid_t(),
        insn in insn_table(),
    ) {
        let (u, opcode, mnemonic) = insn;
        let asm = asm_t(rd, rn, rm, t, mnemonic);
        let ops = ops_t(rd, rn, rm, t);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&ops, u, opcode)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for {}", asm);
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_three_same_invariant_arm_fields(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        t in valid_t(),
        u in 0u32..=1,
        opcode in 0u32..=31,
    ) {
        let w = sut_word(&ops_t(rd, rn, rm, t), u, opcode)
            .unwrap_or_else(|e| panic!("SUT rejected: {}", e));
        let (q, size) = q_size_of(t);
        prop_assert_eq!((w >> 31) & 1, 0u32, "bit31=0");
        prop_assert_eq!((w >> 30) & 1, q, "Q");
        prop_assert_eq!((w >> 29) & 1, u, "U");
        prop_assert_eq!((w >> 24) & 0x1F, 0b01110u32, "bits[28:24]=01110");
        prop_assert_eq!((w >> 22) & 3, size, "size");
        prop_assert_eq!((w >> 21) & 1, 1u32, "bit21=1");
        prop_assert_eq!((w >> 16) & 0x1F, rm, "Rm");
        prop_assert_eq!((w >> 11) & 0x1F, opcode, "opcode");
        prop_assert_eq!((w >> 10) & 1, 1u32, "bit10=1");
        prop_assert_eq!((w >> 5) & 0x1F, rn, "Rn");
        prop_assert_eq!(w & 0x1F, rd, "Rd");
    }

    #[test]
    fn encode_neon_three_same_metamorphic_u_opcode_q(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        opcode1 in 0u32..=31,
        opcode2 in 0u32..=31,
    ) {
        let w_u0 = sut_word(&ops_t(rd, rn, rm, "8b"), 0, opcode1)
            .unwrap_or_else(|e| panic!("SUT rejected u=0: {}", e));
        let w_u1 = sut_word(&ops_t(rd, rn, rm, "8b"), 1, opcode1)
            .unwrap_or_else(|e| panic!("SUT rejected u=1: {}", e));
        let w_op2 = sut_word(&ops_t(rd, rn, rm, "8b"), 0, opcode2)
            .unwrap_or_else(|e| panic!("SUT rejected opcode2: {}", e));
        let w_16b = sut_word(&ops_t(rd, rn, rm, "16b"), 0, opcode1)
            .unwrap_or_else(|e| panic!("SUT rejected 16b: {}", e));
        prop_assert_eq!(
            (w_u0 ^ w_u1) & !(1u32 << 29),
            0u32,
            "changing only U must differ only in bit 29"
        );
        prop_assert_eq!((w_u1 >> 29) & 1, 1u32, "U=1");
        prop_assert_eq!((w_u0 >> 29) & 1, 0u32, "U=0");
        prop_assert_eq!(
            (w_u0 ^ w_op2) & !(0x1Fu32 << 11),
            0u32,
            "changing only opcode must differ only in bits[15:11]"
        );
        prop_assert_eq!((w_op2 >> 11) & 0x1F, opcode2, "opcode field");
        prop_assert_eq!(
            (w_u0 ^ w_16b) & !(1u32 << 30),
            0u32,
            "8b vs 16b must differ only in Q bit 30"
        );
        prop_assert_eq!((w_16b >> 30) & 1, 1u32, "Q=1 for 16b");
        prop_assert_eq!((w_u0 >> 30) & 1, 0u32, "Q=0 for 8b");
    }

    #[test]
    fn encode_neon_three_same_neg_arity(
        n in 0usize..=2,
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        insn in insn_table(),
    ) {
        let (u, opcode, mnemonic) = insn;
        let all = ops_t(rd, rn, rm, "8b");
        let arity_ops: Vec<Operand> = all.iter().take(n).cloned().collect();
        let arity_asm = match n {
            0 => mnemonic.to_string(),
            1 => format!("{mnemonic} v{rd}.8b"),
            _ => format!("{mnemonic} v{rd}.8b, v{rn}.8b"),
        };
        prop_assert!(
            llvm_mc_word(&arity_asm).is_err(),
            "llvm-mc unexpectedly accepted arity-{} {}",
            n,
            arity_asm
        );
        prop_assert!(
            encode_neon_three_same(&arity_ops, u, opcode).is_err(),
            "arity {} must Err (llvm-mc rejects {})",
            n,
            arity_asm
        );
    }

    #[test]
    fn encode_neon_three_same_neg_extra_operand(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        extra in reg_num(),
        t in valid_t(),
        insn in insn_table(),
    ) {
        let (u, opcode, mnemonic) = insn;
        let asm = format!("{}, v{}.{t}", asm_t(rd, rn, rm, t, mnemonic), extra);
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted 4-operand {}",
            asm
        );
        let mut ops = ops_t(rd, rn, rm, t);
        ops.push(arr(extra, t));
        prop_assert!(
            encode_neon_three_same(&ops, u, opcode).is_err(),
            "4 operands must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_three_same_neg_invalid_t(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        insn in insn_table(),
        td in any_t(),
        tn in any_t(),
        tm in any_t(),
    ) {
        prop_assume!(!(is_valid_three_same_t(td) && td == tn && tn == tm));
        let (u, opcode, mnemonic) = insn;
        let asm = format!("{mnemonic} v{rd}.{td}, v{rn}.{tn}, v{rm}.{tm}");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted invalid T {}",
            asm
        );
        let ops = vec![arr(rd, td), arr(rn, tn), arr(rm, tm)];
        prop_assert!(
            encode_neon_three_same(&ops, u, opcode).is_err(),
            "invalid/mismatched/reserved T must Err (ARM T in {{8B,16B,4H,8H,2S,4S,2D}} matching; llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_three_same_neg_gpr_or_bare(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        insn in insn_table(),
        kind in 0u8..=4,
        fp_prefix in prop::sample::select(vec!["x", "w", "d", "s", "q", "h", "b"]),
    ) {
        let (u, opcode, mnemonic) = insn;
        let (ops, asm) = match kind {
            0 => (
                vec![
                    Operand::Reg(format!("{}{}", fp_prefix, rd)),
                    arr(rn, "8b"),
                    arr(rm, "8b"),
                ],
                format!("{mnemonic} {fp_prefix}{rd}, v{rn}.8b, v{rm}.8b"),
            ),
            1 => (
                vec![
                    arr(rd, "8b"),
                    Operand::Reg(format!("v{}", rn)),
                    arr(rm, "8b"),
                ],
                format!("{mnemonic} v{rd}.8b, v{rn}, v{rm}.8b"),
            ),
            2 => (
                vec![
                    arr(rd, "8b"),
                    arr(rn, "8b"),
                    Operand::Reg(format!("x{}", rm)),
                ],
                format!("{mnemonic} v{rd}.8b, v{rn}.8b, x{rm}"),
            ),
            3 => (
                vec![
                    Operand::Reg(format!("v{}", rd)),
                    arr(rn, "8b"),
                    arr(rm, "8b"),
                ],
                format!("{mnemonic} v{rd}, v{rn}.8b, v{rm}.8b"),
            ),
            _ => (
                vec![
                    Operand::RegArrangement {
                        reg: format!("x{}", rd),
                        arrangement: "8b".to_string(),
                    },
                    arr(rn, "8b"),
                    arr(rm, "8b"),
                ],
                format!("{mnemonic} x{rd}.8b, v{rn}.8b, v{rm}.8b"),
            ),
        };
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        prop_assert!(
            encode_neon_three_same(&ops, u, opcode).is_err(),
            "GPR/bare/non-arrangement kind={} must Err (llvm-mc rejects {})",
            kind,
            asm
        );
    }

    #[test]
    fn encode_neon_three_same_neg_invalid_name_nonreg(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        insn in insn_table(),
        slot in 0usize..=2,
        kind in 0u8..=5,
    ) {
        let (u, opcode, _mnemonic) = insn;
        let bad = match kind {
            0 => Operand::RegArrangement {
                reg: "v32".into(),
                arrangement: "8b".into(),
            },
            1 => Operand::RegArrangement {
                reg: "foo".into(),
                arrangement: "8b".into(),
            },
            2 => Operand::RegArrangement {
                reg: String::new(),
                arrangement: "8b".into(),
            },
            3 => Operand::Imm(0),
            4 => Operand::Mem {
                base: format!("x{}", rd),
                offset: 0,
            },
            _ => Operand::Symbol("L0".into()),
        };
        let mut ops = ops_t(rd, rn, rm, "8b");
        ops[slot] = bad;
        prop_assert!(
            encode_neon_three_same(&ops, u, opcode).is_err(),
            "invalid name / non-register slot={} kind={} must Err",
            slot,
            kind
        );
    }
}

/// Deterministic regression: extra operand ignored (from neg_extra_operand).
#[test]
fn test_encode_neon_three_same_regression_extra_operand() {
    let mut ops = ops_t(0, 0, 0, "8b");
    ops.push(arr(0, "8b"));
    assert!(
        encode_neon_three_same(&ops, 1, 0b10001).is_err(),
        "cmeq v0.8b, v0.8b, v0.8b, v0.8b must Err (gas/llvm-mc reject a fourth operand)"
    );
}

/// Deterministic regression: mismatched T encoded (from neg_invalid_t).
#[test]
fn test_encode_neon_three_same_regression_mismatched_t() {
    let ops = vec![arr(0, "8b"), arr(0, "16b"), arr(0, "8b")];
    assert!(
        encode_neon_three_same(&ops, 1, 0b10001).is_err(),
        "cmeq v0.8b, v0.16b, v0.8b must Err (ARM/gas/llvm-mc require matching T)"
    );
}

/// Deterministic regression: reserved .1d encoded (from neg_invalid_t domain).
#[test]
fn test_encode_neon_three_same_regression_reserved_1d() {
    let ops = ops_t(0, 0, 0, "1d");
    assert!(
        encode_neon_three_same(&ops, 1, 0b10001).is_err(),
        "cmeq v0.1d, v0.1d, v0.1d must Err (ARM size:Q=11:0 reserved; llvm-mc rejects)"
    );
}

/// Deterministic regression: bare V source encoded (from neg_gpr_or_bare kind=1).
#[test]
fn test_encode_neon_three_same_regression_bare_src() {
    let ops = vec![arr(0, "8b"), Operand::Reg("v0".into()), arr(0, "8b")];
    assert!(
        encode_neon_three_same(&ops, 1, 0b10001).is_err(),
        "cmeq v0.8b, v0, v0.8b must Err (gas/llvm-mc require Vn.T)"
    );
}

/// Deterministic regression: GPR dest with arrangement encoded (from neg_gpr_or_bare kind=4).
#[test]
fn test_encode_neon_three_same_regression_gpr_dest() {
    let ops = vec![
        Operand::RegArrangement {
            reg: "x0".into(),
            arrangement: "8b".to_string(),
        },
        arr(0, "8b"),
        arr(0, "8b"),
    ];
    assert!(
        encode_neon_three_same(&ops, 1, 0b10001).is_err(),
        "cmeq x0.8b, v0.8b, v0.8b must Err (gas/llvm-mc require Vd.T)"
    );
}
