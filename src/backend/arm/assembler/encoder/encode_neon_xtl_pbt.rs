// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md:230 NEON widen/long lists sshll/ushll/sxtl/uxtl (+ 2 variants);
//   encoder/mod.rs:1-7 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:895-898 "uxtl"/"uxtl2"/"sxtl"/"sxtl2" => encode_neon_xtl;
//   neon.rs:160 "Encode NEON UXTL/SXTL (unsigned/signed extend long).";
//   neon.rs:161 "These are aliases for USHLL/SSHLL with shift #0.";
//   neon.rs:162 "Format: 0 Q U 011110 immh immb 10100 1 Rn Rd";
//   ARM ARM Advanced SIMD shift-by-immediate UXTL/SXTL (USHLL/SSHLL #0):
//   Ta in {8H,4S,2D}; Tb is 8B/4H/2S (Q=0) or 16B/8H/4S (Q=1);
//   U=1 unsigned / U=0 signed; immh from source esize; immb=000; bits[15:10]=101001.
// Stronger considered:
//   - State machine: rejected — encode_neon_xtl is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree UXTL decoder
//   - Differential vs encode_neon_shll: rejected as primary — independence gate
//     (same encoding family; alias holds only at shift #0)
//   - Differential vs encode_neon_shl: rejected — same-job gate (same-width SHL)
// Weaker available: algebraic.metamorphic (Rd/Rn/U/Q fields), algebraic.invariant (word layout),
//   negative_error (arity / extra / mismatched Ta/Tb / GPR dest)
// Differential: candidate=encode_neon_xtl, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[RegArrangement(Vd,Ta), RegArrangement(Vn,Tb)] + (u_bit, is_high)
//     <-> `{uxtl|sxtl}{2?} Vd.Ta, Vn.Tb`

use super::encode_neon_xtl;
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

fn mnem(u_bit: u32, is_high: bool) -> &'static str {
    match (u_bit, is_high) {
        (0, false) => "sxtl",
        (0, true) => "sxtl2",
        (1, false) => "uxtl",
        _ => "uxtl2",
    }
}

fn immh_of(tb: &str) -> u32 {
    match tb {
        "8b" | "16b" => 0b0001,
        "4h" | "8h" => 0b0010,
        "2s" | "4s" => 0b0100,
        _ => 0,
    }
}

fn mandated_tb(ta: &str, is_high: bool) -> Option<&'static str> {
    match (ta, is_high) {
        ("8h", false) => Some("8b"),
        ("8h", true) => Some("16b"),
        ("4s", false) => Some("4h"),
        ("4s", true) => Some("8h"),
        ("2d", false) => Some("2s"),
        ("2d", true) => Some("4s"),
        _ => None,
    }
}

fn is_valid_pair(ta: &str, tb: &str, is_high: bool) -> bool {
    mandated_tb(ta, is_high) == Some(tb)
}

fn ops2(rd: u32, rn: u32, ta: &str, tb: &str) -> Vec<Operand> {
    vec![arr(rd, ta), arr(rn, tb)]
}

fn asm2(rd: u32, rn: u32, ta: &str, tb: &str, u_bit: u32, is_high: bool) -> String {
    format!("{} v{rd}.{ta}, v{rn}.{tb}", mnem(u_bit, is_high))
}

fn sut_word(ops: &[Operand], u_bit: u32, is_high: bool) -> Result<u32, String> {
    match encode_neon_xtl(ops, u_bit, is_high)? {
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

fn u_bit() -> impl Strategy<Value = u32> {
    0u32..=1u32
}

fn valid_pair() -> impl Strategy<Value = (&'static str, &'static str, bool)> {
    prop::sample::select(vec![
        ("8h", "8b", false),
        ("8h", "16b", true),
        ("4s", "4h", false),
        ("4s", "8h", true),
        ("2d", "2s", false),
        ("2d", "4s", true),
    ])
}

fn any_arr() -> impl Strategy<Value = &'static str> {
    prop::sample::select(vec![
        "8b", "16b", "4h", "8h", "2s", "4s", "1d", "2d", "1q",
    ])
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_neon_xtl_kat_llvm_mc_uxtl_v0_8h_v1_8b() {
    let want = 0x2f08a420u32;
    let mc = llvm_mc_word("uxtl v0.8h, v1.8b").expect("llvm-mc KAT uxtl");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken for uxtl v0.8h, v1.8b");
    let ops = ops2(0, 1, "8h", "8b");
    assert_eq!(sut_word(&ops, 1, false).expect("SUT KAT uxtl"), want);

    let want_s = 0x0f08a420u32;
    let mc_s = llvm_mc_word("sxtl v0.8h, v1.8b").expect("llvm-mc KAT sxtl");
    assert_eq!(mc_s, want_s, "llvm-mc KAT mapping broken for sxtl v0.8h, v1.8b");
    assert_eq!(sut_word(&ops, 0, false).expect("SUT KAT sxtl"), want_s);
}

/// Known-answer: Q / U / 2-suffix / 4h / 2s / v31 bounds.
#[test]
fn encode_neon_xtl_kat_llvm_mc_variants() {
    let want_u2 = 0x6f08a420u32;
    let mc_u2 = llvm_mc_word("uxtl2 v0.8h, v1.16b").expect("llvm-mc KAT uxtl2");
    assert_eq!(mc_u2, want_u2, "llvm-mc KAT mapping broken for uxtl2");
    assert_eq!(
        sut_word(&ops2(0, 1, "8h", "16b"), 1, true).expect("SUT KAT uxtl2"),
        want_u2
    );

    let want_s2 = 0x4f10a420u32;
    let mc_s2 = llvm_mc_word("sxtl2 v0.4s, v1.8h").expect("llvm-mc KAT sxtl2 4s");
    assert_eq!(mc_s2, want_s2, "llvm-mc KAT mapping broken for sxtl2 4s");
    assert_eq!(
        sut_word(&ops2(0, 1, "4s", "8h"), 0, true).expect("SUT KAT sxtl2 4s"),
        want_s2
    );

    let want_2s = 0x2f20a420u32;
    let mc_2s = llvm_mc_word("uxtl v0.2d, v1.2s").expect("llvm-mc KAT uxtl 2d");
    assert_eq!(mc_2s, want_2s, "llvm-mc KAT mapping broken for uxtl 2d");
    assert_eq!(
        sut_word(&ops2(0, 1, "2d", "2s"), 1, false).expect("SUT KAT uxtl 2d"),
        want_2s
    );

    let want_4h = 0x2f10a420u32;
    let mc_4h = llvm_mc_word("uxtl v0.4s, v1.4h").expect("llvm-mc KAT uxtl 4s");
    assert_eq!(mc_4h, want_4h, "llvm-mc KAT mapping broken for uxtl 4s");
    assert_eq!(
        sut_word(&ops2(0, 1, "4s", "4h"), 1, false).expect("SUT KAT uxtl 4s"),
        want_4h
    );

    let want_u2d = 0x6f20a420u32;
    let mc_u2d = llvm_mc_word("uxtl2 v0.2d, v1.4s").expect("llvm-mc KAT uxtl2 2d");
    assert_eq!(mc_u2d, want_u2d, "llvm-mc KAT mapping broken for uxtl2 2d");
    assert_eq!(
        sut_word(&ops2(0, 1, "2d", "4s"), 1, true).expect("SUT KAT uxtl2 2d"),
        want_u2d
    );

    let want_31 = 0x0f08a7ffu32;
    let mc_31 = llvm_mc_word("sxtl v31.8h, v31.8b").expect("llvm-mc KAT v31");
    assert_eq!(mc_31, want_31, "llvm-mc KAT mapping broken for sxtl v31");
    assert_eq!(
        sut_word(&ops2(31, 31, "8h", "8b"), 0, false).expect("SUT KAT v31"),
        want_31
    );

    let want_same = 0x2f08a400u32;
    let mc_same = llvm_mc_word("uxtl v0.8h, v0.8b").expect("llvm-mc KAT same-reg");
    assert_eq!(mc_same, want_same, "llvm-mc KAT mapping broken for same-reg");
    assert_eq!(
        sut_word(&ops2(0, 0, "8h", "8b"), 1, false).expect("SUT KAT same-reg"),
        want_same
    );

    let want_mid = 0x0f10a60fu32;
    let mc_mid = llvm_mc_word("sxtl v15.4s, v16.4h").expect("llvm-mc KAT v15/v16");
    assert_eq!(mc_mid, want_mid, "llvm-mc KAT mapping broken for v15/v16");
    assert_eq!(
        sut_word(&ops2(15, 16, "4s", "4h"), 0, false).expect("SUT KAT v15/v16"),
        want_mid
    );
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_xtl_diff_llvm_mc(
        rd in reg_num(),
        rn in reg_num(),
        pair in valid_pair(),
        u_bit in u_bit(),
    ) {
        let (ta, tb, is_high) = pair;
        let asm = asm2(rd, rn, ta, tb, u_bit, is_high);
        let ops = ops2(rd, rn, ta, tb);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&ops, u_bit, is_high)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for {}", asm);
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_xtl_meta_rd_rn_q_u(
        rd in reg_num(),
        rd2 in reg_num(),
        rn in reg_num(),
        rn2 in reg_num(),
        pair in valid_pair(),
        u_bit in u_bit(),
    ) {
        let (ta, tb, is_high) = pair;
        let w = sut_word(&ops2(rd, rn, ta, tb), u_bit, is_high)
            .unwrap_or_else(|e| panic!("SUT rejected valid: {}", e));
        let w_rd = sut_word(&ops2(rd2, rn, ta, tb), u_bit, is_high)
            .unwrap_or_else(|e| panic!("SUT rejected rd2: {}", e));
        prop_assert_eq!(w ^ w_rd, rd ^ rd2, "changing Rd must differ only in bits[4:0]");

        let w_rn = sut_word(&ops2(rd, rn2, ta, tb), u_bit, is_high)
            .unwrap_or_else(|e| panic!("SUT rejected rn2: {}", e));
        prop_assert_eq!(w ^ w_rn, (rn ^ rn2) << 5, "changing Rn must differ only in bits[9:5]");

        let w_u = sut_word(&ops2(rd, rn, ta, tb), 1 - u_bit, is_high)
            .unwrap_or_else(|e| panic!("SUT rejected flipped U: {}", e));
        prop_assert_eq!(w ^ w_u, 1u32 << 29, "u_bit must toggle only U (bit 29)");

        let tb_flip = mandated_tb(ta, !is_high).expect("Ta has both Q variants");
        let w_q = sut_word(&ops2(rd, rn, ta, tb_flip), u_bit, !is_high)
            .unwrap_or_else(|e| panic!("SUT rejected flipped Q: {}", e));
        prop_assert_eq!(w ^ w_q, 1u32 << 30, "is_high must toggle only Q (bit 30)");
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_xtl_inv_arm_layout(
        rd in reg_num(),
        rn in reg_num(),
        pair in valid_pair(),
        u_bit in u_bit(),
    ) {
        let (ta, tb, is_high) = pair;
        let w = sut_word(&ops2(rd, rn, ta, tb), u_bit, is_high)
            .unwrap_or_else(|e| panic!("SUT rejected valid layout: {}", e));
        let q = if is_high { 1u32 } else { 0 };
        prop_assert_eq!((w >> 31) & 1, 0u32, "bit 31 must be 0");
        prop_assert_eq!((w >> 30) & 1, q, "Q bit");
        prop_assert_eq!((w >> 29) & 1, u_bit, "U bit");
        prop_assert_eq!((w >> 23) & 0b111111, 0b011110u32, "bits[28:23]=011110");
        prop_assert_eq!((w >> 19) & 0b1111, immh_of(tb), "immh from source esize");
        prop_assert_eq!((w >> 16) & 0b111, 0u32, "immb=000 (shift=0)");
        prop_assert_eq!((w >> 10) & 0b111111, 0b101001u32, "opcode 101001");
        prop_assert_eq!((w >> 5) & 0b11111, rn, "Rn");
        prop_assert_eq!(w & 0b11111, rd, "Rd");
        let _ = ta;
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_xtl_neg_arity(
        rd in reg_num(),
        ta in any_arr(),
        u_bit in u_bit(),
        is_high in any::<bool>(),
        n in 0usize..=1,
    ) {
        let ops: Vec<Operand> = if n == 0 {
            vec![]
        } else {
            vec![arr(rd, ta)]
        };
        prop_assert!(
            encode_neon_xtl(&ops, u_bit, is_high).is_err(),
            "arity {} must Err (uxtl/sxtl requires 2 operands)",
            n
        );
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_xtl_neg_extra_operand(
        rd in reg_num(),
        rn in reg_num(),
        extra in reg_num(),
        pair in valid_pair(),
        u_bit in u_bit(),
    ) {
        let (ta, tb, is_high) = pair;
        let asm = format!("{}, v{}.{ta}", asm2(rd, rn, ta, tb, u_bit, is_high), extra);
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted 3-operand {}",
            asm
        );
        let mut ops = ops2(rd, rn, ta, tb);
        ops.push(arr(extra, ta));
        prop_assert!(
            encode_neon_xtl(&ops, u_bit, is_high).is_err(),
            "3 operands must Err (llvm-mc rejects {})",
            asm
        );
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_xtl_neg_mismatched_ta_tb(
        rd in reg_num(),
        rn in reg_num(),
        ta in any_arr(),
        tb in any_arr(),
        u_bit in u_bit(),
        is_high in any::<bool>(),
    ) {
        prop_assume!(!is_valid_pair(ta, tb, is_high));
        let asm = asm2(rd, rn, ta, tb, u_bit, is_high);
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted invalid arrangement {}",
            asm
        );
        let ops = ops2(rd, rn, ta, tb);
        prop_assert!(
            encode_neon_xtl(&ops, u_bit, is_high).is_err(),
            "invalid/mismatched Ta/Tb must Err (ARM UXTL Ta in {{8H,4S,2D}} with matching Tb; llvm-mc rejects {})",
            asm
        );
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_xtl_neg_gpr_or_bare(
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
                ],
                format!("uxtl {fp_prefix}{rd}, v{rn}.8b"),
            ),
            1 => (
                vec![
                    arr(rd, "8h"),
                    Operand::Reg(format!("v{}", rn)),
                ],
                format!("uxtl v{rd}.8h, v{rn}"),
            ),
            2 => (
                vec![
                    Operand::Reg(format!("v{}", rd)),
                    arr(rn, "8b"),
                ],
                format!("uxtl v{rd}, v{rn}.8b"),
            ),
            3 => (
                vec![
                    Operand::RegArrangement {
                        reg: format!("x{}", rd),
                        arrangement: "8h".to_string(),
                    },
                    arr(rn, "8b"),
                ],
                format!("uxtl x{rd}.8h, v{rn}.8b"),
            ),
            _ => (
                vec![
                    arr(rd, "8h"),
                    Operand::Reg(format!("x{}", rn)),
                ],
                format!("uxtl v{rd}.8h, x{rn}"),
            ),
        };
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        prop_assert!(
            encode_neon_xtl(&ops, 1, false).is_err(),
            "GPR/bare/non-arrangement kind={} must Err (llvm-mc rejects {})",
            kind,
            asm
        );
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_xtl_diff_alt_spellings(
        rd in reg_num(),
        rn in reg_num(),
        pair in valid_pair(),
        u_bit in u_bit(),
    ) {
        let (ta, tb, is_high) = pair;
        let ta_u = ta.to_ascii_uppercase();
        let tb_u = tb.to_ascii_uppercase();
        let m = mnem(u_bit, is_high).to_ascii_uppercase();
        let asm = format!("{m} V{rd}.{ta_u}, V{rn}.{tb_u}");
        let ops = vec![
            Operand::RegArrangement {
                reg: format!("V{}", rd),
                arrangement: ta.to_string(),
            },
            Operand::RegArrangement {
                reg: format!("V{}", rn),
                arrangement: tb.to_string(),
            },
        ];
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected alt-spelling {}: {}", asm, e));
        let sut = sut_word(&ops, u_bit, is_high)
            .unwrap_or_else(|e| panic!("SUT rejected alt-spelling {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for alt-spelling {}", asm);
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_xtl_neg_nonreg(
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
        let mut ops = ops2(rd, rn, "8h", "8b");
        ops[slot] = bad;
        let asm = match (kind, slot) {
            (0, 0) => "uxtl #0, v0.8b".to_string(),
            (1, 0) => format!("uxtl [x{rd}], v{rn}.8b"),
            (2, 0) => "uxtl L0, v0.8b".to_string(),
            _ => format!("uxtl nonreg slot={slot} kind={kind}"),
        };
        if slot == 0 {
            prop_assert!(
                llvm_mc_word(&asm).is_err(),
                "llvm-mc unexpectedly accepted {}",
                asm
            );
        }
        prop_assert!(
            encode_neon_xtl(&ops, 1, false).is_err(),
            "non-register operand slot={} kind={} must Err",
            slot,
            kind
        );
    }
}

/// Deterministic regression: extra operand ignored (from neg_extra_operand).
#[test]
fn test_encode_neon_xtl_regression_extra_operand() {
    let mut ops = ops2(0, 0, "8h", "8b");
    ops.push(arr(0, "8h"));
    assert!(
        encode_neon_xtl(&ops, 0, false).is_err(),
        "sxtl v0.8h, v0.8b, v0.8h must Err (gas/llvm-mc reject a third operand)"
    );
}

/// Deterministic regression: mismatched dest Ta encoded (from neg_mismatched_ta_tb).
#[test]
fn test_encode_neon_xtl_regression_mismatched_dest_ta() {
    assert!(
        encode_neon_xtl(&ops2(0, 0, "8b", "8b"), 0, false).is_err(),
        "sxtl v0.8b, v0.8b must Err (ARM/gas/llvm-mc require Ta=8h for Tb=8b, Q=0)"
    );
}

/// Deterministic regression: GPR dest encoded as V (from neg_gpr_or_bare).
#[test]
fn test_encode_neon_xtl_regression_gpr_dest() {
    let ops = vec![Operand::Reg("x0".into()), arr(0, "8b")];
    assert!(
        encode_neon_xtl(&ops, 1, false).is_err(),
        "uxtl x0, v0.8b must Err (gas/llvm-mc require Vd.Ta)"
    );
}
