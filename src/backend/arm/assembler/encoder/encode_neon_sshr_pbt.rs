// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md:228 NEON shifts lists sshr;
//   encoder/mod.rs:1-7 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:699 "sshr" => encode_neon_sshr(operands);
//   neon.rs:1204 Encode NEON SSHR (signed shift right immediate);
//   neon.rs:1215-1216 SSHR Vd.T, Vn.T, #shift / 0 Q 0 0 11110 immh:immb 000001 Rn Rd  (U=0);
//   ARM ARM Advanced SIMD shift-by-immediate SSHR: T in {8B,16B,4H,8H,2S,4S,2D};
//   shift in [1, esize]; matching arrangements; U=0; opcode=000001; immh != 0000;
//   immh:immb = 2*esize - shift.
// Stronger considered:
//   - State machine: rejected — encode_neon_sshr is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree SSHR decoder
//   - Differential vs encode_neon_shift_imm: rejected — independence gate (near-copy USHR body)
//   - Differential vs encode_neon_ushr: rejected — same-job gate (USHR / U=1)
//   - Differential vs encode_neon_shift_right: rejected — generic shift-right opcode table
// Weaker available: algebraic.metamorphic (Rd/Rn/Q fields), algebraic.invariant (word layout, U=0),
//   negative_error (arity / extra / reserved 1d / mismatch / shift OOB / GPR dest)
// Differential: candidate=encode_neon_sshr, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[RegArrangement(Vd,T), RegArrangement(Vn,T), Imm(shift)]
//     <-> `sshr Vd.T, Vn.T, #shift` for T in {8b,16b,4h,8h,2s,4s,2d}, shift in [1, esize]

use super::encode_neon_sshr;
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

fn ops_t(rd: u32, rn: u32, t: &str, shift: i64) -> Vec<Operand> {
    vec![arr(rd, t), arr(rn, t), Operand::Imm(shift)]
}

fn asm_t(rd: u32, rn: u32, t: &str, shift: i64) -> String {
    format!("sshr v{rd}.{t}, v{rn}.{t}, #{shift}")
}

fn sut_word(ops: &[Operand]) -> Result<u32, String> {
    match encode_neon_sshr(ops)? {
        EncodeResult::Word(w) => Ok(w),
        other => Err(format!("expected Word, got {:?}", other)),
    }
}

fn esize(t: &str) -> i64 {
    match t {
        "8b" | "16b" => 8,
        "4h" | "8h" => 16,
        "2s" | "4s" => 32,
        "2d" => 64,
        _ => 0,
    }
}

fn q_of(t: &str) -> u32 {
    match t {
        "16b" | "8h" | "4s" | "2d" => 1,
        _ => 0,
    }
}

fn is_valid_sshr_t(t: &str) -> bool {
    matches!(t, "8b" | "16b" | "4h" | "8h" | "2s" | "4s" | "2d")
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

fn any_t() -> impl Strategy<Value = &'static str> {
    prop::sample::select(vec![
        "8b", "16b", "4h", "8h", "2s", "4s", "1d", "2d", "1q",
    ])
}

/// Co-generate (T, shift) on the documented ARM SSHR domain, pinning 1 and esize.
fn valid_t_shift() -> impl Strategy<Value = (&'static str, i64)> {
    prop_oneof![
        (Just("8b"), prop_oneof![Just(1i64), Just(8i64), 1i64..=8]),
        (Just("16b"), prop_oneof![Just(1i64), Just(8i64), 1i64..=8]),
        (Just("4h"), prop_oneof![Just(1i64), Just(16i64), 1i64..=16]),
        (Just("8h"), prop_oneof![Just(1i64), Just(16i64), 1i64..=16]),
        (Just("2s"), prop_oneof![Just(1i64), Just(32i64), 1i64..=32]),
        (Just("4s"), prop_oneof![Just(1i64), Just(32i64), 1i64..=32]),
        (Just("2d"), prop_oneof![Just(1i64), Just(64i64), 1i64..=64]),
    ]
}

/// Co-generate (T, shift) outside [1, esize], pinning 0, esize+1, negatives, and u32 wrap.
fn oob_t_shift() -> impl Strategy<Value = (&'static str, i64)> {
    prop_oneof![
        (
            Just("8b"),
            prop_oneof![
                Just(0i64),
                Just(9i64),
                Just(-1i64),
                Just(16i64),
                Just(1i64 << 32),
                Just((1i64 << 32) + 1),
            ]
        ),
        (
            Just("16b"),
            prop_oneof![Just(0i64), Just(9i64), Just(-1i64), Just(65i64)]
        ),
        (
            Just("4h"),
            prop_oneof![Just(0i64), Just(17i64), Just(-1i64), Just(32i64)]
        ),
        (
            Just("8h"),
            prop_oneof![Just(0i64), Just(17i64), Just(-1i64), Just(33i64)]
        ),
        (
            Just("2s"),
            prop_oneof![Just(0i64), Just(33i64), Just(-1i64), Just(64i64)]
        ),
        (
            Just("4s"),
            prop_oneof![Just(0i64), Just(33i64), Just(-1i64), Just(65i64)]
        ),
        (
            Just("2d"),
            prop_oneof![Just(0i64), Just(65i64), Just(-1i64), Just(128i64)]
        ),
    ]
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_neon_sshr_kat_llvm_mc() {
    let kat: &[(&str, u32)] = &[
        ("sshr v0.8b, v1.8b, #1", 0x0f0f0420),
        ("sshr v0.16b, v1.16b, #8", 0x4f080420),
        ("sshr v0.4h, v1.4h, #1", 0x0f1f0420),
        ("sshr v0.8h, v1.8h, #16", 0x4f100420),
        ("sshr v0.2s, v1.2s, #1", 0x0f3f0420),
        ("sshr v0.4s, v1.4s, #32", 0x4f200420),
        ("sshr v0.2d, v1.2d, #1", 0x4f7f0420),
        ("sshr v0.2d, v1.2d, #64", 0x4f400420),
        ("sshr v31.8b, v31.8b, #8", 0x0f0807ff),
        ("sshr v0.8b, v0.8b, #1", 0x0f0f0400),
        ("sshr v15.4s, v16.4s, #17", 0x4f2f060f),
    ];
    for &(asm, want) in kat {
        let mc = llvm_mc_word(asm).unwrap_or_else(|e| panic!("llvm-mc KAT {asm}: {e}"));
        assert_eq!(mc, want, "llvm-mc KAT mapping broken for {asm}");
    }
    let sut_kat: &[(u32, u32, &str, i64, u32)] = &[
        (0, 1, "8b", 1, 0x0f0f0420),
        (0, 1, "16b", 8, 0x4f080420),
        (0, 1, "4h", 1, 0x0f1f0420),
        (0, 1, "8h", 16, 0x4f100420),
        (0, 1, "2s", 1, 0x0f3f0420),
        (0, 1, "4s", 32, 0x4f200420),
        (0, 1, "2d", 1, 0x4f7f0420),
        (0, 1, "2d", 64, 0x4f400420),
        (31, 31, "8b", 8, 0x0f0807ff),
        (0, 0, "8b", 1, 0x0f0f0400),
        (15, 16, "4s", 17, 0x4f2f060f),
    ];
    for &(rd, rn, t, shift, want) in sut_kat {
        let asm = asm_t(rd, rn, t, shift);
        let sut = sut_word(&ops_t(rd, rn, t, shift))
            .unwrap_or_else(|e| panic!("SUT KAT {asm}: {e}"));
        assert_eq!(sut, want, "SUT KAT mismatch for {asm}");
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_sshr_diff_llvm_mc(
        rd in reg_num(),
        rn in reg_num(),
        t_shift in valid_t_shift(),
    ) {
        let (t, shift) = t_shift;
        let asm = asm_t(rd, rn, t, shift);
        let ops = ops_t(rd, rn, t, shift);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&ops)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for {}", asm);
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_sshr_metamorphic_rd_rn_q(
        rd1 in reg_num(),
        rd2 in reg_num(),
        rn1 in reg_num(),
        rn2 in reg_num(),
    ) {
        let w11 = sut_word(&ops_t(rd1, rn1, "8b", 1))
            .unwrap_or_else(|e| panic!("SUT rejected rd1/rn1: {}", e));
        let w21 = sut_word(&ops_t(rd2, rn1, "8b", 1))
            .unwrap_or_else(|e| panic!("SUT rejected rd2: {}", e));
        let w12 = sut_word(&ops_t(rd1, rn2, "8b", 1))
            .unwrap_or_else(|e| panic!("SUT rejected rn2: {}", e));
        let w16 = sut_word(&ops_t(rd1, rn1, "16b", 1))
            .unwrap_or_else(|e| panic!("SUT rejected 16b: {}", e));
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
        prop_assert_eq!(
            (w11 ^ w16) & !(1u32 << 30),
            0u32,
            "8b vs 16b must differ only in Q bit 30"
        );
        prop_assert_eq!((w11 >> 30) & 1, 0u32, "Q=0 for 8b");
        prop_assert_eq!((w16 >> 30) & 1, 1u32, "Q=1 for 16b");
    }

    #[test]
    fn encode_neon_sshr_invariant_arm_fields(
        rd in reg_num(),
        rn in reg_num(),
        t_shift in valid_t_shift(),
    ) {
        let (t, shift) = t_shift;
        let w = sut_word(&ops_t(rd, rn, t, shift))
            .unwrap_or_else(|e| panic!("SUT rejected: {}", e));
        let q = q_of(t);
        let immh_immb = (2 * esize(t) as u32).wrapping_sub(shift as u32) & 0x7F;
        prop_assert_eq!((w >> 31) & 1, 0u32, "bit31=0");
        prop_assert_eq!((w >> 30) & 1, q, "Q");
        prop_assert_eq!((w >> 29) & 1, 0u32, "U=0");
        prop_assert_eq!((w >> 23) & 0x3F, 0b011110u32, "bits[28:23]=011110");
        prop_assert_eq!((w >> 16) & 0x7F, immh_immb, "immh:immb = 2*esize - shift");
        prop_assert_eq!((w >> 10) & 0x3F, 0b000001u32, "bits[15:10]=000001");
        prop_assert_eq!((w >> 5) & 0x1F, rn, "Rn");
        prop_assert_eq!(w & 0x1F, rd, "Rd");
    }

    #[test]
    fn encode_neon_sshr_neg_arity(
        n in 0usize..=2,
        rd in reg_num(),
        rn in reg_num(),
    ) {
        let all = ops_t(rd, rn, "8b", 1);
        let arity_ops: Vec<Operand> = all.iter().take(n).cloned().collect();
        let arity_asm = match n {
            0 => "sshr".to_string(),
            1 => format!("sshr v{rd}.8b"),
            _ => format!("sshr v{rd}.8b, v{rn}.8b"),
        };
        prop_assert!(
            llvm_mc_word(&arity_asm).is_err(),
            "llvm-mc unexpectedly accepted arity-{} {}",
            n,
            arity_asm
        );
        prop_assert!(
            encode_neon_sshr(&arity_ops).is_err(),
            "arity {} must Err (llvm-mc rejects {})",
            n,
            arity_asm
        );
    }

    #[test]
    fn encode_neon_sshr_neg_extra_operand(
        rd in reg_num(),
        rn in reg_num(),
        extra in reg_num(),
        t_shift in valid_t_shift(),
    ) {
        let (t, shift) = t_shift;
        let asm = format!("{}, v{}.{t}", asm_t(rd, rn, t, shift), extra);
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted 4-operand {}",
            asm
        );
        let mut ops = ops_t(rd, rn, t, shift);
        ops.push(arr(extra, t));
        prop_assert!(
            encode_neon_sshr(&ops).is_err(),
            "4 operands must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_sshr_neg_invalid_t(
        rd in reg_num(),
        rn in reg_num(),
        td in any_t(),
        tn in any_t(),
        shift in 1i64..=64,
    ) {
        prop_assume!(td != tn || !is_valid_sshr_t(td));
        if is_valid_sshr_t(td) {
            prop_assume!(shift >= 1 && shift <= esize(td));
        }
        let asm = format!("sshr v{rd}.{td}, v{rn}.{tn}, #{shift}");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted invalid T {}",
            asm
        );
        let ops = vec![arr(rd, td), arr(rn, tn), Operand::Imm(shift)];
        prop_assert!(
            encode_neon_sshr(&ops).is_err(),
            "invalid/mismatched/reserved T must Err (ARM SSHR T in {{8B,16B,4H,8H,2S,4S,2D}} matching; llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_sshr_neg_shift_oob(
        rd in reg_num(),
        rn in reg_num(),
        t_shift in oob_t_shift(),
    ) {
        let (t, shift) = t_shift;
        prop_assume!(shift < 1 || shift > esize(t));
        let asm = asm_t(rd, rn, t, shift);
        if shift.abs() < 10_000 {
            prop_assert!(
                llvm_mc_word(&asm).is_err(),
                "llvm-mc unexpectedly accepted OOB shift {}",
                asm
            );
        }
        let ops = ops_t(rd, rn, t, shift);
        prop_assert!(
            encode_neon_sshr(&ops).is_err(),
            "shift {} not in [1, {}] must Err (llvm-mc rejects {})",
            shift,
            esize(t),
            asm
        );
    }

    #[test]
    fn encode_neon_sshr_neg_gpr_or_bare(
        rd in reg_num(),
        rn in reg_num(),
        kind in 0u8..=4,
        fp_prefix in prop::sample::select(vec!["x", "w", "d", "s", "q", "h", "b"]),
    ) {
        let (ops, asm) = match kind {
            0 => (
                vec![
                    Operand::Reg(format!("{}{}", fp_prefix, rd)),
                    arr(rn, "8b"),
                    Operand::Imm(1),
                ],
                format!("sshr {fp_prefix}{rd}, v{rn}.8b, #1"),
            ),
            1 => (
                vec![
                    arr(rd, "8b"),
                    Operand::Reg(format!("v{}", rn)),
                    Operand::Imm(1),
                ],
                format!("sshr v{rd}.8b, v{rn}, #1"),
            ),
            2 => (
                vec![
                    Operand::Reg(format!("v{}", rd)),
                    arr(rn, "8b"),
                    Operand::Imm(1),
                ],
                format!("sshr v{rd}, v{rn}.8b, #1"),
            ),
            3 => (
                vec![
                    Operand::RegArrangement {
                        reg: format!("x{}", rd),
                        arrangement: "8b".to_string(),
                    },
                    arr(rn, "8b"),
                    Operand::Imm(1),
                ],
                format!("sshr x{rd}.8b, v{rn}.8b, #1"),
            ),
            _ => (
                vec![
                    arr(rd, "8b"),
                    Operand::Reg(format!("x{}", rn)),
                    Operand::Imm(1),
                ],
                format!("sshr v{rd}.8b, x{rn}, #1"),
            ),
        };
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        prop_assert!(
            encode_neon_sshr(&ops).is_err(),
            "GPR/bare/non-arrangement kind={} must Err (llvm-mc rejects {})",
            kind,
            asm
        );
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_sshr_diff_alt_spellings(
        rd in reg_num(),
        rn in reg_num(),
        t_shift in valid_t_shift(),
    ) {
        let (t, shift) = t_shift;
        let t_u = t.to_ascii_uppercase();
        let asm = format!("SSHR V{rd}.{t_u}, V{rn}.{t_u}, #{shift}");
        let ops = vec![
            Operand::RegArrangement {
                reg: format!("V{}", rd),
                arrangement: t.to_string(),
            },
            Operand::RegArrangement {
                reg: format!("V{}", rn),
                arrangement: t.to_string(),
            },
            Operand::Imm(shift),
        ];
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected alt-spelling {}: {}", asm, e));
        let sut = sut_word(&ops)
            .unwrap_or_else(|e| panic!("SUT rejected alt-spelling {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for alt-spelling {}", asm);
    }

    #[test]
    fn encode_neon_sshr_neg_nonreg(
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
        let mut ops = ops_t(rd, rn, "8b", 1);
        ops[slot] = bad;
        let asm = match (kind, slot) {
            (0, 0) => "sshr #0, v0.8b, #1".to_string(),
            (1, 0) => format!("sshr [x{rd}], v{rn}.8b, #1"),
            (2, 0) => "sshr L0, v0.8b, #1".to_string(),
            _ => format!("sshr nonreg slot={slot} kind={kind}"),
        };
        if slot == 0 {
            prop_assert!(
                llvm_mc_word(&asm).is_err(),
                "llvm-mc unexpectedly accepted {}",
                asm
            );
        }
        prop_assert!(
            encode_neon_sshr(&ops).is_err(),
            "non-register operand slot={} kind={} must Err",
            slot,
            kind
        );
    }
}

/// Deterministic regression: extra operand ignored (from neg_extra_operand).
#[test]
fn test_encode_neon_sshr_regression_extra_operand() {
    let mut ops = ops_t(0, 0, "8b", 1);
    ops.push(arr(0, "8b"));
    assert!(
        encode_neon_sshr(&ops).is_err(),
        "sshr v0.8b, v0.8b, #1, v0.8b must Err (gas/llvm-mc reject a fourth operand)"
    );
}

/// Deterministic regression: GPR dest with arrangement encoded (from neg_gpr_or_bare kind=3).
#[test]
fn test_encode_neon_sshr_regression_gpr_dest() {
    let ops = vec![
        Operand::RegArrangement {
            reg: "x0".into(),
            arrangement: "8b".to_string(),
        },
        arr(0, "8b"),
        Operand::Imm(1),
    ];
    assert!(
        encode_neon_sshr(&ops).is_err(),
        "sshr x0.8b, v0.8b, #1 must Err (gas/llvm-mc require Vd.T)"
    );
}

/// Deterministic regression: bare Vn source encoded (from neg_gpr_or_bare kind=1).
#[test]
fn test_encode_neon_sshr_regression_bare_src() {
    let ops = vec![
        arr(0, "8b"),
        Operand::Reg("v0".into()),
        Operand::Imm(1),
    ];
    assert!(
        encode_neon_sshr(&ops).is_err(),
        "sshr v0.8b, v0, #1 must Err (gas/llvm-mc require Vn.T)"
    );
}

/// Deterministic regression: mismatched T encoded (from neg_invalid_t).
#[test]
fn test_encode_neon_sshr_regression_mismatched_t() {
    let ops = vec![arr(0, "2s"), arr(0, "8b"), Operand::Imm(1)];
    assert!(
        encode_neon_sshr(&ops).is_err(),
        "sshr v0.2s, v0.8b, #1 must Err (ARM/gas/llvm-mc require matching T)"
    );
}

/// Deterministic regression: shift 0 encoded (from neg_shift_oob).
#[test]
fn test_encode_neon_sshr_regression_shift_oob() {
    assert!(
        encode_neon_sshr(&ops_t(0, 0, "8b", 0)).is_err(),
        "sshr v0.8b, v0.8b, #0 must Err (ARM/llvm-mc require shift in [1, 8])"
    );
}
