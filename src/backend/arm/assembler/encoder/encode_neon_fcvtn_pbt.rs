// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md:225 NEON two-misc lists fcvtn/fcvtl;
//   encoder/mod.rs:3 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:539-540 "fcvtn"/"fcvtn2" => encode_neon_fcvtn;
//   neon.rs:1651 "FCVTN: single→half or double→single narrowing float convert";
//   ARM ARM Advanced SIMD two-register miscellaneous FCVTN{2} Vd.Tb, Vn.Ta:
//   Tb=4H Ta=4S (Q=0) or 8H (Q=1); Tb=2S Ta=2D (Q=0) or 4S (Q=1);
//   U=0; opcode=10110; sz=0 single→half, sz=1 double→single.
// Stronger considered:
//   - State machine: rejected — encode_neon_fcvtn is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree FCVTN decoder
//   - Differential vs encode_neon_fcvtl: rejected — same-job gate (narrow vs widen)
//   - Differential vs encode_neon_xtl / encode_neon_two_misc: rejected — different encoding class
// Weaker available: algebraic.metamorphic (Rd/Rn/Q fields), algebraic.invariant (word layout),
//   negative_error (arity / extra / mismatched Ta/Tb / GPR dest)
// Differential: candidate=encode_neon_fcvtn, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[RegArrangement(Vd,Tb), RegArrangement(Vn,Ta)] + is_high
//     <-> `fcvtn{2?} Vd.Tb, Vn.Ta`

use super::encode_neon_fcvtn;
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

fn mnem(is_high: bool) -> &'static str {
    if is_high {
        "fcvtn2"
    } else {
        "fcvtn"
    }
}

fn mandated_tb(ta: &str, is_high: bool) -> Option<&'static str> {
    match (ta, is_high) {
        ("4s", false) => Some("4h"),
        ("4s", true) => Some("8h"),
        ("2d", false) => Some("2s"),
        ("2d", true) => Some("4s"),
        _ => None,
    }
}

fn is_valid_pair(tb: &str, ta: &str, is_high: bool) -> bool {
    mandated_tb(ta, is_high) == Some(tb)
}

fn ops2(rd: u32, rn: u32, tb: &str, ta: &str) -> Vec<Operand> {
    vec![arr(rd, tb), arr(rn, ta)]
}

fn asm2(rd: u32, rn: u32, tb: &str, ta: &str, is_high: bool) -> String {
    format!("{} v{rd}.{tb}, v{rn}.{ta}", mnem(is_high))
}

fn sut_word(ops: &[Operand], is_high: bool) -> Result<u32, String> {
    match encode_neon_fcvtn(ops, is_high)? {
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

fn valid_pair() -> impl Strategy<Value = (&'static str, &'static str, bool)> {
    // (tb, ta, is_high)
    prop::sample::select(vec![
        ("4h", "4s", false),
        ("8h", "4s", true),
        ("2s", "2d", false),
        ("4s", "2d", true),
    ])
}

fn any_arr() -> impl Strategy<Value = &'static str> {
    prop::sample::select(vec![
        "8b", "16b", "4h", "8h", "2s", "4s", "1d", "2d", "1q",
    ])
}

fn arm_fcvtn_layout(w: u32, rd: u32, rn: u32, ta: &str, is_high: bool) -> bool {
    let q = if is_high { 1u32 } else { 0 };
    let sz = if ta == "2d" { 1u32 } else { 0 };
    ((w >> 31) & 1) == 0
        && ((w >> 30) & 1) == q
        && ((w >> 29) & 1) == 0
        && ((w >> 24) & 0b11111) == 0b01110
        && ((w >> 23) & 1) == 0
        && ((w >> 22) & 1) == sz
        && ((w >> 17) & 0b11111) == 0b10000
        && ((w >> 12) & 0b11111) == 0b10110
        && ((w >> 10) & 0b11) == 0b10
        && ((w >> 5) & 0b11111) == rn
        && (w & 0b11111) == rd
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_neon_fcvtn_kat_llvm_mc_v0_4h_v1_4s() {
    let want = 0x0e216820u32;
    let mc = llvm_mc_word("fcvtn v0.4h, v1.4s").expect("llvm-mc KAT fcvtn");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken for fcvtn v0.4h, v1.4s");
    let ops = ops2(0, 1, "4h", "4s");
    assert_eq!(sut_word(&ops, false).expect("SUT KAT fcvtn"), want);
}

/// Known-answer: Q / 2-suffix / 2d / v31 bounds.
#[test]
fn encode_neon_fcvtn_kat_llvm_mc_variants() {
    let want_2 = 0x4e216820u32;
    let mc_2 = llvm_mc_word("fcvtn2 v0.8h, v1.4s").expect("llvm-mc KAT fcvtn2");
    assert_eq!(mc_2, want_2, "llvm-mc KAT mapping broken for fcvtn2");
    assert_eq!(
        sut_word(&ops2(0, 1, "8h", "4s"), true).expect("SUT KAT fcvtn2"),
        want_2
    );

    let want_2d = 0x0e616820u32;
    let mc_2d = llvm_mc_word("fcvtn v0.2s, v1.2d").expect("llvm-mc KAT fcvtn 2d");
    assert_eq!(mc_2d, want_2d, "llvm-mc KAT mapping broken for fcvtn 2d");
    assert_eq!(
        sut_word(&ops2(0, 1, "2s", "2d"), false).expect("SUT KAT fcvtn 2d"),
        want_2d
    );

    let want_2d2 = 0x4e616820u32;
    let mc_2d2 = llvm_mc_word("fcvtn2 v0.4s, v1.2d").expect("llvm-mc KAT fcvtn2 2d");
    assert_eq!(mc_2d2, want_2d2, "llvm-mc KAT mapping broken for fcvtn2 2d");
    assert_eq!(
        sut_word(&ops2(0, 1, "4s", "2d"), true).expect("SUT KAT fcvtn2 2d"),
        want_2d2
    );

    let want_31 = 0x0e216bffu32;
    let mc_31 = llvm_mc_word("fcvtn v31.4h, v31.4s").expect("llvm-mc KAT v31");
    assert_eq!(mc_31, want_31, "llvm-mc KAT mapping broken for fcvtn v31");
    assert_eq!(
        sut_word(&ops2(31, 31, "4h", "4s"), false).expect("SUT KAT v31"),
        want_31
    );

    let want_same = 0x0e216800u32;
    let mc_same = llvm_mc_word("fcvtn v0.4h, v0.4s").expect("llvm-mc KAT same-reg");
    assert_eq!(mc_same, want_same, "llvm-mc KAT mapping broken for same-reg");
    assert_eq!(
        sut_word(&ops2(0, 0, "4h", "4s"), false).expect("SUT KAT same-reg"),
        want_same
    );

    let want_mid = 0x0e616a0fu32;
    let mc_mid = llvm_mc_word("fcvtn v15.2s, v16.2d").expect("llvm-mc KAT v15/v16");
    assert_eq!(mc_mid, want_mid, "llvm-mc KAT mapping broken for v15/v16");
    assert_eq!(
        sut_word(&ops2(15, 16, "2s", "2d"), false).expect("SUT KAT v15/v16"),
        want_mid
    );
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_fcvtn_diff_llvm_mc(
        rd in reg_num(),
        rn in reg_num(),
        pair in valid_pair(),
    ) {
        let (tb, ta, is_high) = pair;
        let asm = asm2(rd, rn, tb, ta, is_high);
        let ops = ops2(rd, rn, tb, ta);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&ops, is_high)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for {}", asm);
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_fcvtn_meta_rd_rn_q(
        rd in reg_num(),
        rd2 in reg_num(),
        rn in reg_num(),
        rn2 in reg_num(),
        pair in valid_pair(),
    ) {
        let (tb, ta, is_high) = pair;
        let w = sut_word(&ops2(rd, rn, tb, ta), is_high)
            .unwrap_or_else(|e| panic!("SUT rejected valid: {}", e));
        let w_rd = sut_word(&ops2(rd2, rn, tb, ta), is_high)
            .unwrap_or_else(|e| panic!("SUT rejected rd2: {}", e));
        prop_assert_eq!(w ^ w_rd, rd ^ rd2, "changing Rd must differ only in bits[4:0]");

        let w_rn = sut_word(&ops2(rd, rn2, tb, ta), is_high)
            .unwrap_or_else(|e| panic!("SUT rejected rn2: {}", e));
        prop_assert_eq!(w ^ w_rn, (rn ^ rn2) << 5, "changing Rn must differ only in bits[9:5]");

        let tb_flip = mandated_tb(ta, !is_high).expect("Ta has both Q variants");
        let w_q = sut_word(&ops2(rd, rn, tb_flip, ta), !is_high)
            .unwrap_or_else(|e| panic!("SUT rejected flipped Q: {}", e));
        prop_assert_eq!(w ^ w_q, 1u32 << 30, "is_high must toggle only Q (bit 30)");
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_fcvtn_inv_arm_layout(
        rd in reg_num(),
        rn in reg_num(),
        pair in valid_pair(),
    ) {
        let (tb, ta, is_high) = pair;
        let w = sut_word(&ops2(rd, rn, tb, ta), is_high)
            .unwrap_or_else(|e| panic!("SUT rejected valid layout: {}", e));
        prop_assert!(
            arm_fcvtn_layout(w, rd, rn, ta, is_high),
            "ARM FCVTN layout violated for {} word={:#010x}",
            asm2(rd, rn, tb, ta, is_high),
            w
        );
        let _ = tb;
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_fcvtn_neg_arity(
        rd in reg_num(),
        ta in any_arr(),
        is_high in any::<bool>(),
        n in 0usize..=1,
    ) {
        let ops: Vec<Operand> = if n == 0 {
            vec![]
        } else {
            vec![arr(rd, ta)]
        };
        prop_assert!(
            encode_neon_fcvtn(&ops, is_high).is_err(),
            "arity {} must Err (fcvtn requires 2 operands)",
            n
        );
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_fcvtn_neg_extra_operand(
        rd in reg_num(),
        rn in reg_num(),
        extra in reg_num(),
        pair in valid_pair(),
    ) {
        let (tb, ta, is_high) = pair;
        let asm = format!("{}, v{}.{tb}", asm2(rd, rn, tb, ta, is_high), extra);
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted 3-operand {}",
            asm
        );
        let mut ops = ops2(rd, rn, tb, ta);
        ops.push(arr(extra, tb));
        prop_assert!(
            encode_neon_fcvtn(&ops, is_high).is_err(),
            "3 operands must Err (llvm-mc rejects {})",
            asm
        );
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_fcvtn_neg_mismatched_ta_tb(
        rd in reg_num(),
        rn in reg_num(),
        tb in any_arr(),
        ta in any_arr(),
        is_high in any::<bool>(),
    ) {
        prop_assume!(!is_valid_pair(tb, ta, is_high));
        let asm = asm2(rd, rn, tb, ta, is_high);
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted invalid arrangement {}",
            asm
        );
        let ops = ops2(rd, rn, tb, ta);
        prop_assert!(
            encode_neon_fcvtn(&ops, is_high).is_err(),
            "invalid/mismatched Tb/Ta must Err (ARM FCVTN Ta in {{4S,2D}} with matching Tb; llvm-mc rejects {})",
            asm
        );
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_fcvtn_neg_gpr_or_bare(
        rd in reg_num(),
        rn in reg_num(),
        kind in 0u8..=4,
        fp_prefix in prop::sample::select(vec!["x", "w", "d", "s", "q", "h", "b"]),
    ) {
        let (ops, asm) = match kind {
            0 => (
                vec![
                    Operand::Reg(format!("{}{}", fp_prefix, rd)),
                    arr(rn, "4s"),
                ],
                format!("fcvtn {fp_prefix}{rd}, v{rn}.4s"),
            ),
            1 => (
                vec![
                    arr(rd, "4h"),
                    Operand::Reg(format!("v{}", rn)),
                ],
                format!("fcvtn v{rd}.4h, v{rn}"),
            ),
            2 => (
                vec![
                    Operand::Reg(format!("v{}", rd)),
                    arr(rn, "4s"),
                ],
                format!("fcvtn v{rd}, v{rn}.4s"),
            ),
            3 => (
                vec![
                    Operand::RegArrangement {
                        reg: format!("x{}", rd),
                        arrangement: "4h".to_string(),
                    },
                    arr(rn, "4s"),
                ],
                format!("fcvtn x{rd}.4h, v{rn}.4s"),
            ),
            _ => (
                vec![
                    arr(rd, "4h"),
                    Operand::Reg(format!("x{}", rn)),
                ],
                format!("fcvtn v{rd}.4h, x{rn}"),
            ),
        };
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        prop_assert!(
            encode_neon_fcvtn(&ops, false).is_err(),
            "GPR/bare/non-arrangement kind={} must Err (llvm-mc rejects {})",
            kind,
            asm
        );
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_fcvtn_diff_alt_spellings(
        rd in reg_num(),
        rn in reg_num(),
        pair in valid_pair(),
    ) {
        let (tb, ta, is_high) = pair;
        let tb_u = tb.to_ascii_uppercase();
        let ta_u = ta.to_ascii_uppercase();
        let m = mnem(is_high).to_ascii_uppercase();
        let asm = format!("{m} V{rd}.{tb_u}, V{rn}.{ta_u}");
        let ops = vec![
            Operand::RegArrangement {
                reg: format!("V{}", rd),
                arrangement: tb.to_string(),
            },
            Operand::RegArrangement {
                reg: format!("V{}", rn),
                arrangement: ta.to_string(),
            },
        ];
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected alt-spelling {}: {}", asm, e));
        let sut = sut_word(&ops, is_high)
            .unwrap_or_else(|e| panic!("SUT rejected alt-spelling {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for alt-spelling {}", asm);
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_fcvtn_neg_nonreg(
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
        let mut ops = ops2(rd, rn, "4h", "4s");
        ops[slot] = bad;
        let asm = match (kind, slot) {
            (0, 0) => "fcvtn #0, v0.4s".to_string(),
            (1, 0) => format!("fcvtn [x{rd}], v{rn}.4s"),
            (2, 0) => "fcvtn L0, v0.4s".to_string(),
            _ => format!("fcvtn nonreg slot={slot} kind={kind}"),
        };
        if slot == 0 {
            prop_assert!(
                llvm_mc_word(&asm).is_err(),
                "llvm-mc unexpectedly accepted {}",
                asm
            );
        }
        prop_assert!(
            encode_neon_fcvtn(&ops, false).is_err(),
            "non-register operand slot={} kind={} must Err",
            slot,
            kind
        );
    }
}

/// Deterministic regression: extra operand ignored (from neg_extra_operand).
#[test]
fn test_encode_neon_fcvtn_regression_extra_operand() {
    let mut ops = ops2(0, 0, "4h", "4s");
    ops.push(arr(0, "4h"));
    assert!(
        encode_neon_fcvtn(&ops, false).is_err(),
        "fcvtn v0.4h, v0.4s, v0.4h must Err (gas/llvm-mc reject a third operand)"
    );
}

/// Deterministic regression: dest 8b / source 2s encoded (from neg_mismatched_ta_tb).
#[test]
fn test_encode_neon_fcvtn_regression_mismatched_ta_tb() {
    assert!(
        encode_neon_fcvtn(&ops2(0, 0, "8b", "2s"), false).is_err(),
        "fcvtn v0.8b, v0.2s must Err (ARM/gas/llvm-mc require Ta in {{4S,2D}} with matching Tb)"
    );
}

/// Deterministic regression: dest 8b with valid source 4s still encodes (dest arrangement discarded).
#[test]
fn test_encode_neon_fcvtn_regression_mismatched_dest_tb() {
    assert!(
        encode_neon_fcvtn(&ops2(0, 0, "8b", "4s"), false).is_err(),
        "fcvtn v0.8b, v0.4s must Err (ARM FCVTN Vn.4S requires Vd.4H)"
    );
}

/// Deterministic regression: bare V dest encoded as Vd.Tb (from neg_gpr_or_bare kind=2).
#[test]
fn test_encode_neon_fcvtn_regression_bare_dest() {
    let ops = vec![
        Operand::Reg("v0".into()),
        arr(0, "4s"),
    ];
    assert!(
        encode_neon_fcvtn(&ops, false).is_err(),
        "fcvtn v0, v0.4s must Err (gas/llvm-mc require Vd.Tb)"
    );
}
