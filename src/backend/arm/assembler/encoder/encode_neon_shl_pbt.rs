// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md:228 NEON shifts lists shl;
//   encoder/mod.rs:1-7 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:702 "shl" => encode_neon_shl(operands);
//   neon.rs:1230 Encode NEON SHL (shift left immediate);
//   neon.rs:1241-1243 SHL Vd.T, Vn.T, #shift / 0 Q 0 0 11110 immh:immb 010101 Rn Rd / immh:immb = element_size + shift;
//   ARM ARM Advanced SIMD shift-by-immediate SHL: T in {8B,16B,4H,8H,2S,4S,2D};
//   shift in [0, esize-1]; matching arrangements; U=0; opcode=010101; immh != 0000;
//   immh:immb = esize + shift.
// Stronger considered:
//   - State machine: rejected — encode_neon_shl is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree SHL decoder
//   - Differential vs encode_neon_shift_left_imm: rejected — independence gate (generic left-shift helper)
//   - Differential vs encode_neon_sli: rejected — same-job gate (SLI / U=1)
//   - Differential vs encode_neon_sshr / encode_neon_ushr: rejected — right-shift encodings
// Weaker available: algebraic.metamorphic (Rd/Rn/Q fields), algebraic.invariant (word layout, U=0, opcode 010101),
//   negative_error (arity / extra / reserved 1d / mismatch / shift OOB / GPR dest)
// Differential: candidate=encode_neon_shl, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[RegArrangement(Vd,T), RegArrangement(Vn,T), Imm(shift)]
//     <-> `shl Vd.T, Vn.T, #shift` for T in {8b,16b,4h,8h,2s,4s,2d}, shift in [0, esize-1]

use super::encode_neon_shl;
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
    format!("shl v{rd}.{t}, v{rn}.{t}, #{shift}")
}

fn sut_word(ops: &[Operand]) -> Result<u32, String> {
    match encode_neon_shl(ops)? {
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

fn is_valid_shl_t(t: &str) -> bool {
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

/// Co-generate (T, shift) on the documented ARM SHL domain, pinning 0 and esize-1.
fn valid_t_shift() -> impl Strategy<Value = (&'static str, i64)> {
    prop_oneof![
        (Just("8b"), prop_oneof![Just(0i64), Just(7i64), 0i64..=7]),
        (Just("16b"), prop_oneof![Just(0i64), Just(7i64), 0i64..=7]),
        (Just("4h"), prop_oneof![Just(0i64), Just(15i64), 0i64..=15]),
        (Just("8h"), prop_oneof![Just(0i64), Just(15i64), 0i64..=15]),
        (Just("2s"), prop_oneof![Just(0i64), Just(31i64), 0i64..=31]),
        (Just("4s"), prop_oneof![Just(0i64), Just(31i64), 0i64..=31]),
        (Just("2d"), prop_oneof![Just(0i64), Just(63i64), 0i64..=63]),
    ]
}

/// Co-generate (T, shift) outside [0, esize-1], pinning -1, esize, esize+1, and wrap.
fn oob_t_shift() -> impl Strategy<Value = (&'static str, i64)> {
    prop_oneof![
        (
            Just("8b"),
            prop_oneof![
                Just(-1i64),
                Just(8i64),
                Just(9i64),
                Just(16i64),
                Just(1i64 << 32),
                Just((1i64 << 32) + 1),
            ]
        ),
        (
            Just("16b"),
            prop_oneof![Just(-1i64), Just(8i64), Just(9i64), Just(65i64)]
        ),
        (
            Just("4h"),
            prop_oneof![Just(-1i64), Just(16i64), Just(17i64), Just(32i64)]
        ),
        (
            Just("8h"),
            prop_oneof![Just(-1i64), Just(16i64), Just(17i64), Just(33i64)]
        ),
        (
            Just("2s"),
            prop_oneof![Just(-1i64), Just(32i64), Just(33i64), Just(64i64)]
        ),
        (
            Just("4s"),
            prop_oneof![Just(-1i64), Just(32i64), Just(33i64), Just(65i64)]
        ),
        (
            Just("2d"),
            prop_oneof![Just(-1i64), Just(64i64), Just(65i64), Just(128i64)]
        ),
    ]
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_neon_shl_kat_llvm_mc() {
    let kat: &[(&str, u32)] = &[
        ("shl v0.8b, v1.8b, #0", 0x0f085420),
        ("shl v0.8b, v1.8b, #7", 0x0f0f5420),
        ("shl v0.16b, v1.16b, #0", 0x4f085420),
        ("shl v0.4h, v1.4h, #0", 0x0f105420),
        ("shl v0.8h, v1.8h, #15", 0x4f1f5420),
        ("shl v0.2s, v1.2s, #0", 0x0f205420),
        ("shl v0.4s, v1.4s, #31", 0x4f3f5420),
        ("shl v0.2d, v1.2d, #0", 0x4f405420),
        ("shl v0.2d, v1.2d, #63", 0x4f7f5420),
        ("shl v31.8b, v31.8b, #1", 0x0f0957ff),
        ("shl v0.8b, v0.8b, #0", 0x0f085400),
        ("shl v15.4s, v16.4s, #16", 0x4f30560f),
    ];
    for &(asm, want) in kat {
        let mc = llvm_mc_word(asm).unwrap_or_else(|e| panic!("llvm-mc KAT {asm}: {e}"));
        assert_eq!(mc, want, "llvm-mc KAT mapping broken for {asm}");
    }
    let sut_kat: &[(u32, u32, &str, i64, u32)] = &[
        (0, 1, "8b", 0, 0x0f085420),
        (0, 1, "8b", 7, 0x0f0f5420),
        (0, 1, "16b", 0, 0x4f085420),
        (0, 1, "4h", 0, 0x0f105420),
        (0, 1, "8h", 15, 0x4f1f5420),
        (0, 1, "2s", 0, 0x0f205420),
        (0, 1, "4s", 31, 0x4f3f5420),
        (0, 1, "2d", 0, 0x4f405420),
        (0, 1, "2d", 63, 0x4f7f5420),
        (31, 31, "8b", 1, 0x0f0957ff),
        (0, 0, "8b", 0, 0x0f085400),
        (15, 16, "4s", 16, 0x4f30560f),
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
    fn encode_neon_shl_diff_llvm_mc(
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
    fn encode_neon_shl_metamorphic_rd_rn_q(
        rd1 in reg_num(),
        rd2 in reg_num(),
        rn1 in reg_num(),
        rn2 in reg_num(),
    ) {
        let w11 = sut_word(&ops_t(rd1, rn1, "8b", 0))
            .unwrap_or_else(|e| panic!("SUT rejected rd1/rn1: {}", e));
        let w21 = sut_word(&ops_t(rd2, rn1, "8b", 0))
            .unwrap_or_else(|e| panic!("SUT rejected rd2: {}", e));
        let w12 = sut_word(&ops_t(rd1, rn2, "8b", 0))
            .unwrap_or_else(|e| panic!("SUT rejected rn2: {}", e));
        let w16 = sut_word(&ops_t(rd1, rn1, "16b", 0))
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
    fn encode_neon_shl_invariant_arm_fields(
        rd in reg_num(),
        rn in reg_num(),
        t_shift in valid_t_shift(),
    ) {
        let (t, shift) = t_shift;
        let w = sut_word(&ops_t(rd, rn, t, shift))
            .unwrap_or_else(|e| panic!("SUT rejected: {}", e));
        let q = q_of(t);
        let immh_immb = (esize(t) as u32).wrapping_add(shift as u32) & 0x7F;
        prop_assert_eq!((w >> 31) & 1, 0u32, "bit31=0");
        prop_assert_eq!((w >> 30) & 1, q, "Q");
        prop_assert_eq!((w >> 29) & 1, 0u32, "U=0");
        prop_assert_eq!((w >> 23) & 0x3F, 0b011110u32, "bits[28:23]=011110");
        prop_assert_eq!((w >> 16) & 0x7F, immh_immb, "immh:immb = esize + shift");
        prop_assert_eq!((w >> 10) & 0x3F, 0b010101u32, "bits[15:10]=010101");
        prop_assert_eq!((w >> 5) & 0x1F, rn, "Rn");
        prop_assert_eq!(w & 0x1F, rd, "Rd");
    }

    #[test]
    fn encode_neon_shl_neg_arity(
        n in 0usize..=2,
        rd in reg_num(),
        rn in reg_num(),
    ) {
        let all = ops_t(rd, rn, "8b", 0);
        let arity_ops: Vec<Operand> = all.iter().take(n).cloned().collect();
        let arity_asm = match n {
            0 => "shl".to_string(),
            1 => format!("shl v{rd}.8b"),
            _ => format!("shl v{rd}.8b, v{rn}.8b"),
        };
        prop_assert!(
            llvm_mc_word(&arity_asm).is_err(),
            "llvm-mc unexpectedly accepted arity-{} {}",
            n,
            arity_asm
        );
        prop_assert!(
            encode_neon_shl(&arity_ops).is_err(),
            "arity {} must Err (llvm-mc rejects {})",
            n,
            arity_asm
        );
    }

    #[test]
    fn encode_neon_shl_neg_extra_operand(
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
            encode_neon_shl(&ops).is_err(),
            "4 operands must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_shl_neg_invalid_t(
        rd in reg_num(),
        rn in reg_num(),
        td in any_t(),
        tn in any_t(),
        shift in 0i64..=63,
    ) {
        prop_assume!(td != tn || !is_valid_shl_t(td));
        if is_valid_shl_t(td) {
            prop_assume!(shift >= 0 && shift < esize(td));
        }
        let asm = format!("shl v{rd}.{td}, v{rn}.{tn}, #{shift}");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted invalid T {}",
            asm
        );
        let ops = vec![arr(rd, td), arr(rn, tn), Operand::Imm(shift)];
        prop_assert!(
            encode_neon_shl(&ops).is_err(),
            "invalid/mismatched/reserved T must Err (ARM SHL T in {{8B,16B,4H,8H,2S,4S,2D}} matching; llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_shl_neg_shift_oob(
        rd in reg_num(),
        rn in reg_num(),
        t_shift in oob_t_shift(),
    ) {
        let (t, shift) = t_shift;
        prop_assume!(shift < 0 || shift >= esize(t));
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
            encode_neon_shl(&ops).is_err(),
            "shift {} not in [0, {}) must Err (llvm-mc rejects {})",
            shift,
            esize(t),
            asm
        );
    }

    #[test]
    fn encode_neon_shl_neg_gpr_or_bare(
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
                    Operand::Imm(0),
                ],
                format!("shl {fp_prefix}{rd}, v{rn}.8b, #0"),
            ),
            1 => (
                vec![
                    arr(rd, "8b"),
                    Operand::Reg(format!("v{}", rn)),
                    Operand::Imm(0),
                ],
                format!("shl v{rd}.8b, v{rn}, #0"),
            ),
            2 => (
                vec![
                    Operand::Reg(format!("v{}", rd)),
                    arr(rn, "8b"),
                    Operand::Imm(0),
                ],
                format!("shl v{rd}, v{rn}.8b, #0"),
            ),
            3 => (
                vec![
                    Operand::RegArrangement {
                        reg: format!("x{}", rd),
                        arrangement: "8b".to_string(),
                    },
                    arr(rn, "8b"),
                    Operand::Imm(0),
                ],
                format!("shl x{rd}.8b, v{rn}.8b, #0"),
            ),
            _ => (
                vec![
                    arr(rd, "8b"),
                    Operand::Reg(format!("x{}", rn)),
                    Operand::Imm(0),
                ],
                format!("shl v{rd}.8b, x{rn}, #0"),
            ),
        };
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        prop_assert!(
            encode_neon_shl(&ops).is_err(),
            "GPR/bare/non-arrangement kind={} must Err (llvm-mc rejects {})",
            kind,
            asm
        );
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_shl_diff_alt_spellings(
        rd in reg_num(),
        rn in reg_num(),
        t_shift in valid_t_shift(),
    ) {
        let (t, shift) = t_shift;
        let t_u = t.to_ascii_uppercase();
        let asm = format!("SHL V{rd}.{t_u}, V{rn}.{t_u}, #{shift}");
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
    fn encode_neon_shl_neg_nonreg(
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
        let mut ops = ops_t(rd, rn, "8b", 0);
        ops[slot] = bad;
        let asm = match (kind, slot) {
            (0, 0) => "shl #0, v0.8b, #0".to_string(),
            (1, 0) => format!("shl [x{rd}], v{rn}.8b, #0"),
            (2, 0) => "shl L0, v0.8b, #0".to_string(),
            _ => format!("shl nonreg slot={slot} kind={kind}"),
        };
        if slot == 0 {
            prop_assert!(
                llvm_mc_word(&asm).is_err(),
                "llvm-mc unexpectedly accepted {}",
                asm
            );
        }
        prop_assert!(
            encode_neon_shl(&ops).is_err(),
            "non-register operand slot={} kind={} must Err",
            slot,
            kind
        );
    }
}

/// Deterministic regression: extra operand ignored (from neg_extra_operand).
#[test]
fn test_encode_neon_shl_regression_extra_operand() {
    let mut ops = ops_t(0, 0, "8b", 0);
    ops.push(arr(0, "8b"));
    assert!(
        encode_neon_shl(&ops).is_err(),
        "shl v0.8b, v0.8b, #0, v0.8b must Err (gas/llvm-mc reject a fourth operand)"
    );
}

/// Deterministic regression: GPR dest with arrangement encoded (from neg_gpr_or_bare kind=3).
#[test]
fn test_encode_neon_shl_regression_gpr_dest() {
    let ops = vec![
        Operand::RegArrangement {
            reg: "x0".into(),
            arrangement: "8b".to_string(),
        },
        arr(0, "8b"),
        Operand::Imm(0),
    ];
    assert!(
        encode_neon_shl(&ops).is_err(),
        "shl x0.8b, v0.8b, #0 must Err (gas/llvm-mc require Vd.T)"
    );
}

/// Deterministic regression: bare Vn source encoded (from neg_gpr_or_bare kind=1).
#[test]
fn test_encode_neon_shl_regression_bare_src() {
    let ops = vec![
        arr(0, "8b"),
        Operand::Reg("v0".into()),
        Operand::Imm(0),
    ];
    assert!(
        encode_neon_shl(&ops).is_err(),
        "shl v0.8b, v0, #0 must Err (gas/llvm-mc require Vn.T)"
    );
}

/// Deterministic regression: mismatched T encoded (from neg_invalid_t).
#[test]
fn test_encode_neon_shl_regression_mismatched_t() {
    let ops = vec![arr(0, "2s"), arr(0, "4s"), Operand::Imm(0)];
    assert!(
        encode_neon_shl(&ops).is_err(),
        "shl v0.2s, v0.4s, #0 must Err (ARM/gas/llvm-mc require matching T)"
    );
}

/// Deterministic regression: shift -1 panics / is not Err (from neg_shift_oob).
#[test]
fn test_encode_neon_shl_regression_shift_oob() {
    assert!(
        encode_neon_shl(&ops_t(0, 0, "8b", 8)).is_err(),
        "shl v0.8b, v0.8b, #8 must Err (ARM/llvm-mc require shift in [0, 7])"
    );
}

/// Deterministic regression: negative shift overflows in debug (from neg_shift_oob).
#[test]
fn test_encode_neon_shl_regression_shift_neg() {
    assert!(
        encode_neon_shl(&ops_t(0, 0, "8b", -1)).is_err(),
        "shl v0.8b, v0.8b, #-1 must Err (ARM/llvm-mc require shift in [0, 7])"
    );
}
