// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md:229 NEON narrow lists shrn/shrn2, rshrn/rshrn2;
//   encoder/mod.rs:1-7 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:648-651 "shrn"/"shrn2"/"rshrn"/"rshrn2" => encode_neon_shrn;
//   neon.rs:1434 Format: 0 Q 0 01111 0 immh immb opcode 1 Rn Rd;
//   neon.rs:1435 SHRN opcode=10000, RSHRN opcode=10001;
//   ARM ARM Advanced SIMD shift-by-immediate SHRN/RSHRN: Ta in {8H,4S,2D};
//   Tb is 8B/4H/2S (Q=0) or 16B/8H/4S (Q=1); shift in [1, dest_esize];
//   U=0; bits[15:10]=100001 (SHRN) or 100011 (RSHRN); immh != 0000;
//   immh:immb = source_esize - shift.
// Stronger considered:
//   - State machine: rejected — encode_neon_shrn is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree SHRN decoder
//   - Differential vs encode_neon_qshrn: rejected — same-job gate (saturating SQSHRN/UQSHRN)
//   - Differential vs encode_neon_sqshrun: rejected — signed-to-unsigned saturating
//   - Differential vs encode_neon_scalar_qshrn: rejected — scalar vs vector
//   - Differential vs encode_neon_three_diff_narrow: rejected — ADDHN/SUBHN
// Weaker available: algebraic.metamorphic (Rd/Rn/Q/opcode fields), algebraic.invariant (word layout),
//   negative_error (arity / extra / reserved Ta / Tb mismatch / shift OOB / GPR dest)
// Differential: candidate=encode_neon_shrn, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[RegArrangement(Vd,Tb), RegArrangement(Vn,Ta), Imm(shift)]
//     <-> `{mnem} Vd.Tb, Vn.Ta, #shift` for Ta in {8h,4s,2d}, Tb mandated by (Ta,Q)

use super::encode_neon_shrn;
use super::EncodeResult;
use crate::backend::arm::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;
const OP_SHRN: u32 = 0b100001;
const OP_RSHRN: u32 = 0b100011;

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

fn mandated_tb(ta: &str, is_high: bool) -> &'static str {
    match (ta, is_high) {
        ("8h", false) => "8b",
        ("8h", true) => "16b",
        ("4s", false) => "4h",
        ("4s", true) => "8h",
        ("2d", false) => "2s",
        ("2d", true) => "4s",
        _ => "8b",
    }
}

fn src_esize(ta: &str) -> i64 {
    match ta {
        "8h" => 16,
        "4s" => 32,
        "2d" => 64,
        _ => 0,
    }
}

fn dest_esize(ta: &str) -> i64 {
    src_esize(ta) / 2
}

fn mnem(opcode: u32, is_high: bool) -> &'static str {
    match (opcode, is_high) {
        (OP_SHRN, false) => "shrn",
        (OP_SHRN, true) => "shrn2",
        (OP_RSHRN, false) => "rshrn",
        (OP_RSHRN, true) => "rshrn2",
        _ => "shrn",
    }
}

fn is_valid_ta(ta: &str) -> bool {
    matches!(ta, "8h" | "4s" | "2d")
}

fn is_valid_pair(tb: &str, ta: &str, is_high: bool) -> bool {
    is_valid_ta(ta) && tb == mandated_tb(ta, is_high)
}

fn ops3(rd: u32, rn: u32, tb: &str, ta: &str, shift: i64) -> Vec<Operand> {
    vec![arr(rd, tb), arr(rn, ta), Operand::Imm(shift)]
}

fn asm3(rd: u32, rn: u32, tb: &str, ta: &str, shift: i64, opcode: u32, is_high: bool) -> String {
    format!(
        "{} v{rd}.{tb}, v{rn}.{ta}, #{shift}",
        mnem(opcode, is_high)
    )
}

fn sut_word(ops: &[Operand], opcode: u32, is_high: bool) -> Result<u32, String> {
    match encode_neon_shrn(ops, opcode, is_high)? {
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

fn opcode_bit() -> impl Strategy<Value = u32> {
    prop_oneof![Just(OP_SHRN), Just(OP_RSHRN)]
}

fn any_arr() -> impl Strategy<Value = &'static str> {
    prop::sample::select(vec![
        "8b", "16b", "4h", "8h", "2s", "4s", "1d", "2d", "1q",
    ])
}

/// Co-generate (Ta, shift) on the documented ARM SHRN domain, pinning 1 and dest_esize.
fn valid_ta_shift() -> impl Strategy<Value = (&'static str, i64)> {
    prop_oneof![
        (Just("8h"), prop_oneof![Just(1i64), Just(8i64), 1i64..=8]),
        (Just("4s"), prop_oneof![Just(1i64), Just(16i64), 1i64..=16]),
        (Just("2d"), prop_oneof![Just(1i64), Just(32i64), 1i64..=32]),
    ]
}

/// Co-generate (Ta, shift) outside [1, dest_esize], pinning 0, dest_esize+1, negatives.
fn oob_ta_shift() -> impl Strategy<Value = (&'static str, i64)> {
    prop_oneof![
        (
            Just("8h"),
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
            Just("4s"),
            prop_oneof![Just(0i64), Just(17i64), Just(-1i64), Just(33i64)]
        ),
        (
            Just("2d"),
            prop_oneof![Just(0i64), Just(33i64), Just(-1i64), Just(65i64)]
        ),
    ]
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_neon_shrn_kat_llvm_mc() {
    let kat: &[(&str, u32)] = &[
        ("shrn v0.8b, v1.8h, #1", 0x0f0f8420),
        ("shrn v0.8b, v1.8h, #8", 0x0f088420),
        ("shrn2 v0.16b, v1.8h, #1", 0x4f0f8420),
        ("shrn v0.4h, v1.4s, #1", 0x0f1f8420),
        ("shrn v0.4h, v1.4s, #16", 0x0f108420),
        ("shrn2 v0.8h, v1.4s, #16", 0x4f108420),
        ("shrn v0.2s, v1.2d, #1", 0x0f3f8420),
        ("shrn v0.2s, v1.2d, #32", 0x0f208420),
        ("shrn2 v0.4s, v1.2d, #1", 0x4f3f8420),
        ("rshrn v0.8b, v1.8h, #1", 0x0f0f8c20),
        ("rshrn2 v0.16b, v1.8h, #8", 0x4f088c20),
        ("shrn v31.8b, v31.8h, #8", 0x0f0887ff),
        ("shrn v0.8b, v0.8h, #1", 0x0f0f8400),
        ("shrn v15.4h, v16.4s, #9", 0x0f17860f),
    ];
    for &(asm, want) in kat {
        let mc = llvm_mc_word(asm).unwrap_or_else(|e| panic!("llvm-mc KAT {asm}: {e}"));
        assert_eq!(mc, want, "llvm-mc KAT mapping broken for {asm}");
    }
    let sut_kat: &[(u32, u32, &str, &str, i64, u32, bool, u32)] = &[
        (0, 1, "8b", "8h", 1, OP_SHRN, false, 0x0f0f8420),
        (0, 1, "8b", "8h", 8, OP_SHRN, false, 0x0f088420),
        (0, 1, "16b", "8h", 1, OP_SHRN, true, 0x4f0f8420),
        (0, 1, "4h", "4s", 1, OP_SHRN, false, 0x0f1f8420),
        (0, 1, "4h", "4s", 16, OP_SHRN, false, 0x0f108420),
        (0, 1, "8h", "4s", 16, OP_SHRN, true, 0x4f108420),
        (0, 1, "2s", "2d", 1, OP_SHRN, false, 0x0f3f8420),
        (0, 1, "2s", "2d", 32, OP_SHRN, false, 0x0f208420),
        (0, 1, "4s", "2d", 1, OP_SHRN, true, 0x4f3f8420),
        (0, 1, "8b", "8h", 1, OP_RSHRN, false, 0x0f0f8c20),
        (0, 1, "16b", "8h", 8, OP_RSHRN, true, 0x4f088c20),
        (31, 31, "8b", "8h", 8, OP_SHRN, false, 0x0f0887ff),
        (0, 0, "8b", "8h", 1, OP_SHRN, false, 0x0f0f8400),
        (15, 16, "4h", "4s", 9, OP_SHRN, false, 0x0f17860f),
    ];
    for &(rd, rn, tb, ta, shift, opcode, is_high, want) in sut_kat {
        let asm = asm3(rd, rn, tb, ta, shift, opcode, is_high);
        let sut = sut_word(&ops3(rd, rn, tb, ta, shift), opcode, is_high)
            .unwrap_or_else(|e| panic!("SUT KAT {asm}: {e}"));
        assert_eq!(sut, want, "SUT KAT mismatch for {asm}");
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_shrn_diff_llvm_mc(
        rd in reg_num(),
        rn in reg_num(),
        ta_shift in valid_ta_shift(),
        is_high in any::<bool>(),
        opcode in opcode_bit(),
    ) {
        let (ta, shift) = ta_shift;
        let tb = mandated_tb(ta, is_high);
        let asm = asm3(rd, rn, tb, ta, shift, opcode, is_high);
        let ops = ops3(rd, rn, tb, ta, shift);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&ops, opcode, is_high)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for {}", asm);
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_shrn_metamorphic_rd_rn_q(
        rd1 in reg_num(),
        rd2 in reg_num(),
        rn1 in reg_num(),
        rn2 in reg_num(),
    ) {
        let w11 = sut_word(&ops3(rd1, rn1, "8b", "8h", 1), OP_SHRN, false)
            .unwrap_or_else(|e| panic!("SUT rejected rd1/rn1: {}", e));
        let w21 = sut_word(&ops3(rd2, rn1, "8b", "8h", 1), OP_SHRN, false)
            .unwrap_or_else(|e| panic!("SUT rejected rd2: {}", e));
        let w12 = sut_word(&ops3(rd1, rn2, "8b", "8h", 1), OP_SHRN, false)
            .unwrap_or_else(|e| panic!("SUT rejected rn2: {}", e));
        let w_high = sut_word(&ops3(rd1, rn1, "16b", "8h", 1), OP_SHRN, true)
            .unwrap_or_else(|e| panic!("SUT rejected shrn2: {}", e));
        let w_r = sut_word(&ops3(rd1, rn1, "8b", "8h", 1), OP_RSHRN, false)
            .unwrap_or_else(|e| panic!("SUT rejected rshrn: {}", e));
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
            (w11 ^ w_high) & !(1u32 << 30),
            0u32,
            "SHRN vs SHRN2 must differ only in Q bit 30"
        );
        prop_assert_eq!((w11 >> 30) & 1, 0u32, "Q=0 for SHRN");
        prop_assert_eq!((w_high >> 30) & 1, 1u32, "Q=1 for SHRN2");
        prop_assert_eq!(
            (w11 ^ w_r) & !(0x3Fu32 << 10),
            0u32,
            "SHRN vs RSHRN must differ only in bits[15:10]"
        );
        prop_assert_eq!((w11 >> 10) & 0x3F, OP_SHRN, "SHRN opcode");
        prop_assert_eq!((w_r >> 10) & 0x3F, OP_RSHRN, "RSHRN opcode");
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_shrn_invariant_arm_fields(
        rd in reg_num(),
        rn in reg_num(),
        ta_shift in valid_ta_shift(),
        is_high in any::<bool>(),
        opcode in opcode_bit(),
    ) {
        let (ta, shift) = ta_shift;
        let tb = mandated_tb(ta, is_high);
        let w = sut_word(&ops3(rd, rn, tb, ta, shift), opcode, is_high)
            .unwrap_or_else(|e| panic!("SUT rejected: {}", e));
        let q = if is_high { 1u32 } else { 0 };
        let immh_immb = (src_esize(ta) as u32).wrapping_sub(shift as u32) & 0x7F;
        prop_assert_eq!((w >> 31) & 1, 0u32, "bit31=0");
        prop_assert_eq!((w >> 30) & 1, q, "Q");
        prop_assert_eq!((w >> 29) & 1, 0u32, "U=0");
        prop_assert_eq!((w >> 23) & 0x3F, 0b011110u32, "bits[28:23]=011110");
        prop_assert_eq!((w >> 16) & 0x7F, immh_immb, "immh:immb = source_esize - shift");
        prop_assert_eq!((w >> 10) & 0x3F, opcode, "bits[15:10]=opcode");
        prop_assert_eq!((w >> 5) & 0x1F, rn, "Rn");
        prop_assert_eq!(w & 0x1F, rd, "Rd");
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_shrn_neg_arity(
        n in 0usize..=2,
        rd in reg_num(),
        rn in reg_num(),
    ) {
        let all = ops3(rd, rn, "8b", "8h", 1);
        let arity_ops: Vec<Operand> = all.iter().take(n).cloned().collect();
        let arity_asm = match n {
            0 => "shrn".to_string(),
            1 => format!("shrn v{rd}.8b"),
            _ => format!("shrn v{rd}.8b, v{rn}.8h"),
        };
        prop_assert!(
            llvm_mc_word(&arity_asm).is_err(),
            "llvm-mc unexpectedly accepted arity-{} {}",
            n,
            arity_asm
        );
        prop_assert!(
            encode_neon_shrn(&arity_ops, OP_SHRN, false).is_err(),
            "arity {} must Err (llvm-mc rejects {})",
            n,
            arity_asm
        );
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_shrn_neg_extra_operand(
        rd in reg_num(),
        rn in reg_num(),
        extra in reg_num(),
        ta_shift in valid_ta_shift(),
        is_high in any::<bool>(),
        opcode in opcode_bit(),
    ) {
        let (ta, shift) = ta_shift;
        let tb = mandated_tb(ta, is_high);
        let asm = format!(
            "{}, v{}.{tb}",
            asm3(rd, rn, tb, ta, shift, opcode, is_high),
            extra
        );
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted 4-operand {}",
            asm
        );
        let mut ops = ops3(rd, rn, tb, ta, shift);
        ops.push(arr(extra, tb));
        prop_assert!(
            encode_neon_shrn(&ops, opcode, is_high).is_err(),
            "4 operands must Err (llvm-mc rejects {})",
            asm
        );
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_shrn_neg_invalid_arrangement(
        rd in reg_num(),
        rn in reg_num(),
        tb in any_arr(),
        ta in any_arr(),
        shift in 1i64..=64,
        is_high in any::<bool>(),
        opcode in opcode_bit(),
    ) {
        prop_assume!(!is_valid_pair(tb, ta, is_high));
        if is_valid_ta(ta) {
            prop_assume!(shift >= 1 && shift <= dest_esize(ta));
        }
        let asm = asm3(rd, rn, tb, ta, shift, opcode, is_high);
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted invalid arrangement {}",
            asm
        );
        let ops = ops3(rd, rn, tb, ta, shift);
        prop_assert!(
            encode_neon_shrn(&ops, opcode, is_high).is_err(),
            "invalid/mismatched Ta/Tb must Err (ARM SHRN Ta in {{8H,4S,2D}} with matching Tb; llvm-mc rejects {})",
            asm
        );
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_shrn_neg_shift_oob(
        rd in reg_num(),
        rn in reg_num(),
        ta_shift in oob_ta_shift(),
        is_high in any::<bool>(),
        opcode in opcode_bit(),
    ) {
        let (ta, shift) = ta_shift;
        prop_assume!(shift < 1 || shift > dest_esize(ta));
        let tb = mandated_tb(ta, is_high);
        let asm = asm3(rd, rn, tb, ta, shift, opcode, is_high);
        if shift.abs() < 10_000 {
            prop_assert!(
                llvm_mc_word(&asm).is_err(),
                "llvm-mc unexpectedly accepted OOB shift {}",
                asm
            );
        }
        let ops = ops3(rd, rn, tb, ta, shift);
        prop_assert!(
            encode_neon_shrn(&ops, opcode, is_high).is_err(),
            "shift {} not in [1, {}] must Err (llvm-mc rejects {})",
            shift,
            dest_esize(ta),
            asm
        );
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_shrn_neg_gpr_or_bare(
        rd in reg_num(),
        rn in reg_num(),
        kind in 0u8..=4,
        fp_prefix in prop::sample::select(vec!["x", "w", "d", "s", "q", "h", "b"]),
    ) {
        let (ops, asm) = match kind {
            0 => (
                vec![
                    Operand::Reg(format!("{}{}", fp_prefix, rd)),
                    arr(rn, "8h"),
                    Operand::Imm(1),
                ],
                format!("shrn {fp_prefix}{rd}, v{rn}.8h, #1"),
            ),
            1 => (
                vec![
                    arr(rd, "8b"),
                    Operand::Reg(format!("v{}", rn)),
                    Operand::Imm(1),
                ],
                format!("shrn v{rd}.8b, v{rn}, #1"),
            ),
            2 => (
                vec![
                    Operand::Reg(format!("v{}", rd)),
                    arr(rn, "8h"),
                    Operand::Imm(1),
                ],
                format!("shrn v{rd}, v{rn}.8h, #1"),
            ),
            3 => (
                vec![
                    Operand::RegArrangement {
                        reg: format!("x{}", rd),
                        arrangement: "8b".to_string(),
                    },
                    arr(rn, "8h"),
                    Operand::Imm(1),
                ],
                format!("shrn x{rd}.8b, v{rn}.8h, #1"),
            ),
            _ => (
                vec![
                    arr(rd, "8b"),
                    Operand::Reg(format!("x{}", rn)),
                    Operand::Imm(1),
                ],
                format!("shrn v{rd}.8b, x{rn}, #1"),
            ),
        };
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        prop_assert!(
            encode_neon_shrn(&ops, OP_SHRN, false).is_err(),
            "GPR/bare/non-arrangement kind={} must Err (llvm-mc rejects {})",
            kind,
            asm
        );
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_shrn_diff_alt_spellings(
        rd in reg_num(),
        rn in reg_num(),
        ta_shift in valid_ta_shift(),
        is_high in any::<bool>(),
        opcode in opcode_bit(),
    ) {
        let (ta, shift) = ta_shift;
        let tb = mandated_tb(ta, is_high);
        let tb_u = tb.to_ascii_uppercase();
        let ta_u = ta.to_ascii_uppercase();
        let m = mnem(opcode, is_high).to_ascii_uppercase();
        let asm = format!("{m} V{rd}.{tb_u}, V{rn}.{ta_u}, #{shift}");
        let ops = vec![
            Operand::RegArrangement {
                reg: format!("V{}", rd),
                arrangement: tb.to_string(),
            },
            Operand::RegArrangement {
                reg: format!("V{}", rn),
                arrangement: ta.to_string(),
            },
            Operand::Imm(shift),
        ];
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected alt-spelling {}: {}", asm, e));
        let sut = sut_word(&ops, opcode, is_high)
            .unwrap_or_else(|e| panic!("SUT rejected alt-spelling {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for alt-spelling {}", asm);
    }

    #[test]
    fn encode_neon_shrn_neg_nonreg(
        rd in reg_num(),
        rn in reg_num(),
        kind in 0u8..=2,
        slot in 0usize..=2,
    ) {
        let bad = match kind {
            0 => Operand::Imm(0),
            1 => Operand::Mem {
                base: format!("x{}", rd),
                offset: 0,
            },
            _ => Operand::Label("L0".into()),
        };
        let mut ops = ops3(rd, rn, "8b", "8h", 1);
        ops[slot] = bad;
        let asm = match (kind, slot) {
            (0, 0) => "shrn #0, v0.8h, #1".to_string(),
            (1, 0) => format!("shrn [x{rd}], v{rn}.8h, #1"),
            (2, 0) => "shrn L0, v0.8h, #1".to_string(),
            _ => format!("shrn nonreg slot={slot} kind={kind}"),
        };
        if slot == 0 {
            prop_assert!(
                llvm_mc_word(&asm).is_err(),
                "llvm-mc unexpectedly accepted {}",
                asm
            );
        }
        prop_assert!(
            encode_neon_shrn(&ops, OP_SHRN, false).is_err(),
            "non-register operand slot={} kind={} must Err",
            slot,
            kind
        );
    }

    #[test]
    fn encode_neon_shrn_neg_unsupported_source(
        rd in reg_num(),
        rn in reg_num(),
        ta in prop::sample::select(vec!["8b", "16b", "4h", "2s", "1d", "1q"]),
        is_high in any::<bool>(),
        opcode in opcode_bit(),
    ) {
        let tb = if is_high { "16b" } else { "8b" };
        let asm = asm3(rd, rn, tb, ta, 1, opcode, is_high);
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted unsupported Ta {}",
            asm
        );
        prop_assert!(
            encode_neon_shrn(&ops3(rd, rn, tb, ta, 1), opcode, is_high).is_err(),
            "unsupported source {} must Err (ARM SHRN Ta in {{8H,4S,2D}}; llvm-mc rejects {})",
            ta,
            asm
        );
    }
}

/// Deterministic regression: extra operand ignored (from neg_extra_operand).
#[test]
fn test_encode_neon_shrn_regression_extra_operand() {
    let mut ops = ops3(0, 0, "8b", "8h", 1);
    ops.push(arr(0, "8b"));
    assert!(
        encode_neon_shrn(&ops, OP_SHRN, false).is_err(),
        "shrn v0.8b, v0.8h, #1, v0.8b must Err (gas/llvm-mc reject a fourth operand)"
    );
}

/// Deterministic regression: mismatched dest Tb encoded (from neg_invalid_arrangement).
#[test]
fn test_encode_neon_shrn_regression_mismatched_dest_tb() {
    let ops = ops3(0, 0, "8b", "2d", 1);
    assert!(
        encode_neon_shrn(&ops, OP_SHRN, false).is_err(),
        "shrn v0.8b, v0.2d, #1 must Err (ARM/gas/llvm-mc require Tb=2s for Ta=2d, Q=0)"
    );
}

/// Deterministic regression: i64 shift truncated via as u32 (from neg_shift_oob).
#[test]
fn test_encode_neon_shrn_regression_shift_i64_trunc() {
    assert!(
        encode_neon_shrn(&ops3(0, 0, "8b", "8h", 4294967297), OP_SHRN, false).is_err(),
        "shrn v0.8b, v0.8h, #4294967297 must Err (ARM/llvm-mc require shift in [1, 8])"
    );
}

/// Deterministic regression: bare V dest without arrangement (from neg_gpr_or_bare).
#[test]
fn test_encode_neon_shrn_regression_bare_dest() {
    let ops = vec![
        Operand::Reg("v0".into()),
        arr(0, "8h"),
        Operand::Imm(1),
    ];
    assert!(
        encode_neon_shrn(&ops, OP_SHRN, false).is_err(),
        "shrn v0, v0.8h, #1 must Err (gas/llvm-mc require Vd.Tb)"
    );
}
