// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md:225 NEON two-misc lists xtn/xtn2, uqxtn/uqxtn2, sqxtn/sqxtn2, sqxtun/sqxtun2;
//   encoder/mod.rs:3 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:676-677 "sqxtun"/"sqxtun2" => encode_neon_two_misc_narrow;
//   encoder/mod.rs:941-946 "uqxtn"/"uqxtn2"/"sqxtn"/"sqxtn2"/"xtn"/"xtn2" => encode_neon_two_misc_narrow;
//   neon.rs:202 "Encode NEON two-register miscellaneous narrowing: UQXTN, SQXTN, XTN";
//   neon.rs:204 "Format: 0 Q U 01110 size 10000 opcode 10 Rn Rd";
//   ARM ARM Advanced SIMD two-register miscellaneous XTN{2}/SQXTN{2}/UQXTN{2}/SQXTUN{2} Vd.Tb, Vn.Ta:
//   Tb=8B Ta=8H (Q=0) or 16B (Q=1); Tb=4H Ta=4S (Q=0) or 8H (Q=1); Tb=2S Ta=2D (Q=0) or 4S (Q=1);
//   XTN U=0 opcode=10010; SQXTN U=0 opcode=10100; UQXTN U=1 opcode=10100; SQXTUN U=1 opcode=10010.
// Stronger considered:
//   - State machine: rejected — encode_neon_two_misc_narrow is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree XTN/SQXTN decoder
//   - Differential vs encode_neon_two_misc: rejected — same-job gate (matching-T ABS/NEG vs narrowing)
//   - Differential vs encode_neon_fcvtn: rejected — same-job gate (FP convert vs integer extract)
//   - Differential vs encode_neon_xtl / encode_neon_shrn / encode_neon_qshrn: rejected — widen / shift-narrow
// Weaker available: algebraic.metamorphic (Rd/Rn/Q/U/opcode fields), algebraic.invariant (word layout),
//   negative_error (arity / extra / mismatched Tb/Ta / GPR dest)
// Differential: candidate=encode_neon_two_misc_narrow, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[RegArrangement(Vd,Tb), RegArrangement(Vn,Ta)] + (U, opcode, is_high)
//     <-> `{xtn|sqxtn|uqxtn|sqxtun}{2?} Vd.Tb, Vn.Ta`

use super::encode_neon_two_misc_narrow;
use super::EncodeResult;
use crate::backend::arm::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;

const OPC_XTN: u32 = 0b10010;
const OPC_SQXTN: u32 = 0b10100;

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

fn mnem(base: &str, is_high: bool) -> String {
    if is_high {
        format!("{base}2")
    } else {
        base.to_string()
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

fn is_valid_pair(tb: &str, ta: &str, is_high: bool) -> bool {
    mandated_tb(ta, is_high) == Some(tb)
}

fn size_of_ta(ta: &str) -> Option<u32> {
    match ta {
        "8h" => Some(0b00),
        "4s" => Some(0b01),
        "2d" => Some(0b10),
        _ => None,
    }
}

fn ops2(rd: u32, rn: u32, tb: &str, ta: &str) -> Vec<Operand> {
    vec![arr(rd, tb), arr(rn, ta)]
}

fn asm2(base: &str, rd: u32, rn: u32, tb: &str, ta: &str, is_high: bool) -> String {
    format!("{} v{rd}.{tb}, v{rn}.{ta}", mnem(base, is_high))
}

fn sut_word(ops: &[Operand], u_bit: u32, opcode: u32, is_high: bool) -> Result<u32, String> {
    match encode_neon_two_misc_narrow(ops, u_bit, opcode, is_high)? {
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
    prop::sample::select(vec![
        ("8b", "8h", false),
        ("16b", "8h", true),
        ("4h", "4s", false),
        ("8h", "4s", true),
        ("2s", "2d", false),
        ("4s", "2d", true),
    ])
}

fn family() -> impl Strategy<Value = (&'static str, u32, u32)> {
    prop::sample::select(vec![
        ("xtn", 0u32, OPC_XTN),
        ("sqxtn", 0u32, OPC_SQXTN),
        ("uqxtn", 1u32, OPC_SQXTN),
        ("sqxtun", 1u32, OPC_XTN),
    ])
}

fn any_arr() -> impl Strategy<Value = &'static str> {
    prop::sample::select(vec![
        "8b", "16b", "4h", "8h", "2s", "4s", "1d", "2d", "1q",
    ])
}

fn arm_narrow_layout(
    w: u32,
    rd: u32,
    rn: u32,
    ta: &str,
    u_bit: u32,
    opcode: u32,
    is_high: bool,
) -> bool {
    let q = if is_high { 1u32 } else { 0 };
    let size = match size_of_ta(ta) {
        Some(s) => s,
        None => return false,
    };
    ((w >> 31) & 1) == 0
        && ((w >> 30) & 1) == q
        && ((w >> 29) & 1) == u_bit
        && ((w >> 24) & 0b11111) == 0b01110
        && ((w >> 22) & 0b11) == size
        && ((w >> 17) & 0b11111) == 0b10000
        && ((w >> 12) & 0b11111) == opcode
        && ((w >> 10) & 0b11) == 0b10
        && ((w >> 5) & 0b11111) == rn
        && (w & 0b11111) == rd
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_neon_two_misc_narrow_kat_llvm_mc_xtn_v0_8b_v1_8h() {
    let want = 0x0e212820u32;
    let mc = llvm_mc_word("xtn v0.8b, v1.8h").expect("llvm-mc KAT xtn");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken for xtn v0.8b, v1.8h");
    let ops = ops2(0, 1, "8b", "8h");
    assert_eq!(
        sut_word(&ops, 0, OPC_XTN, false).expect("SUT KAT xtn"),
        want
    );
}

/// Known-answer: Q / 2-suffix / 4s / 2d / v31 / families.
#[test]
fn encode_neon_two_misc_narrow_kat_llvm_mc_variants() {
    let want_2 = 0x4e212820u32;
    let mc_2 = llvm_mc_word("xtn2 v0.16b, v1.8h").expect("llvm-mc KAT xtn2");
    assert_eq!(mc_2, want_2, "llvm-mc KAT mapping broken for xtn2");
    assert_eq!(
        sut_word(&ops2(0, 1, "16b", "8h"), 0, OPC_XTN, true).expect("SUT KAT xtn2"),
        want_2
    );

    let want_4s = 0x0e612820u32;
    let mc_4s = llvm_mc_word("xtn v0.4h, v1.4s").expect("llvm-mc KAT xtn 4s");
    assert_eq!(mc_4s, want_4s, "llvm-mc KAT mapping broken for xtn 4s");
    assert_eq!(
        sut_word(&ops2(0, 1, "4h", "4s"), 0, OPC_XTN, false).expect("SUT KAT xtn 4s"),
        want_4s
    );

    let want_2d = 0x0ea12820u32;
    let mc_2d = llvm_mc_word("xtn v0.2s, v1.2d").expect("llvm-mc KAT xtn 2d");
    assert_eq!(mc_2d, want_2d, "llvm-mc KAT mapping broken for xtn 2d");
    assert_eq!(
        sut_word(&ops2(0, 1, "2s", "2d"), 0, OPC_XTN, false).expect("SUT KAT xtn 2d"),
        want_2d
    );

    let want_2d2 = 0x4ea12820u32;
    let mc_2d2 = llvm_mc_word("xtn2 v0.4s, v1.2d").expect("llvm-mc KAT xtn2 2d");
    assert_eq!(mc_2d2, want_2d2, "llvm-mc KAT mapping broken for xtn2 2d");
    assert_eq!(
        sut_word(&ops2(0, 1, "4s", "2d"), 0, OPC_XTN, true).expect("SUT KAT xtn2 2d"),
        want_2d2
    );

    let want_sq = 0x0e214820u32;
    let mc_sq = llvm_mc_word("sqxtn v0.8b, v1.8h").expect("llvm-mc KAT sqxtn");
    assert_eq!(mc_sq, want_sq, "llvm-mc KAT mapping broken for sqxtn");
    assert_eq!(
        sut_word(&ops2(0, 1, "8b", "8h"), 0, OPC_SQXTN, false).expect("SUT KAT sqxtn"),
        want_sq
    );

    let want_uq = 0x2e214820u32;
    let mc_uq = llvm_mc_word("uqxtn v0.8b, v1.8h").expect("llvm-mc KAT uqxtn");
    assert_eq!(mc_uq, want_uq, "llvm-mc KAT mapping broken for uqxtn");
    assert_eq!(
        sut_word(&ops2(0, 1, "8b", "8h"), 1, OPC_SQXTN, false).expect("SUT KAT uqxtn"),
        want_uq
    );

    let want_sqtun = 0x2e212820u32;
    let mc_sqtun = llvm_mc_word("sqxtun v0.8b, v1.8h").expect("llvm-mc KAT sqxtun");
    assert_eq!(mc_sqtun, want_sqtun, "llvm-mc KAT mapping broken for sqxtun");
    assert_eq!(
        sut_word(&ops2(0, 1, "8b", "8h"), 1, OPC_XTN, false).expect("SUT KAT sqxtun"),
        want_sqtun
    );

    let want_31 = 0x0e212bffu32;
    let mc_31 = llvm_mc_word("xtn v31.8b, v31.8h").expect("llvm-mc KAT v31");
    assert_eq!(mc_31, want_31, "llvm-mc KAT mapping broken for xtn v31");
    assert_eq!(
        sut_word(&ops2(31, 31, "8b", "8h"), 0, OPC_XTN, false).expect("SUT KAT v31"),
        want_31
    );

    let want_same = 0x0e212800u32;
    let mc_same = llvm_mc_word("xtn v0.8b, v0.8h").expect("llvm-mc KAT same-reg");
    assert_eq!(mc_same, want_same, "llvm-mc KAT mapping broken for same-reg");
    assert_eq!(
        sut_word(&ops2(0, 0, "8b", "8h"), 0, OPC_XTN, false).expect("SUT KAT same-reg"),
        want_same
    );

    let want_mid = 0x0e612a0fu32;
    let mc_mid = llvm_mc_word("xtn v15.4h, v16.4s").expect("llvm-mc KAT v15/v16");
    assert_eq!(mc_mid, want_mid, "llvm-mc KAT mapping broken for xtn v15/v16");
    assert_eq!(
        sut_word(&ops2(15, 16, "4h", "4s"), 0, OPC_XTN, false).expect("SUT KAT v15/v16"),
        want_mid
    );
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_two_misc_narrow_diff_llvm_mc(
        rd in reg_num(),
        rn in reg_num(),
        pair in valid_pair(),
        fam in family(),
    ) {
        let (tb, ta, is_high) = pair;
        let (base, u_bit, opcode) = fam;
        let asm = asm2(base, rd, rn, tb, ta, is_high);
        let ops = ops2(rd, rn, tb, ta);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&ops, u_bit, opcode, is_high)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for {}", asm);
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_two_misc_narrow_meta_rd_rn_q_u_opcode(
        rd in reg_num(),
        rd2 in reg_num(),
        rn in reg_num(),
        rn2 in reg_num(),
        pair in valid_pair(),
        fam in family(),
    ) {
        let (tb, ta, is_high) = pair;
        let (_base, u_bit, opcode) = fam;
        let w = sut_word(&ops2(rd, rn, tb, ta), u_bit, opcode, is_high)
            .unwrap_or_else(|e| panic!("SUT rejected valid: {}", e));
        let w_rd = sut_word(&ops2(rd2, rn, tb, ta), u_bit, opcode, is_high)
            .unwrap_or_else(|e| panic!("SUT rejected rd2: {}", e));
        prop_assert_eq!(w ^ w_rd, rd ^ rd2, "changing Rd must differ only in bits[4:0]");

        let w_rn = sut_word(&ops2(rd, rn2, tb, ta), u_bit, opcode, is_high)
            .unwrap_or_else(|e| panic!("SUT rejected rn2: {}", e));
        prop_assert_eq!(w ^ w_rn, (rn ^ rn2) << 5, "changing Rn must differ only in bits[9:5]");

        let tb_flip = mandated_tb(ta, !is_high).expect("Ta has both Q variants");
        let w_q = sut_word(&ops2(rd, rn, tb_flip, ta), u_bit, opcode, !is_high)
            .unwrap_or_else(|e| panic!("SUT rejected flipped Q: {}", e));
        prop_assert_eq!(w ^ w_q, 1u32 << 30, "is_high must toggle only Q (bit 30)");

        let u2 = u_bit ^ 1;
        let w_u = sut_word(&ops2(rd, rn, tb, ta), u2, opcode, is_high)
            .unwrap_or_else(|e| panic!("SUT rejected flipped U: {}", e));
        prop_assert_eq!(w ^ w_u, 1u32 << 29, "u_bit must toggle only U (bit 29)");

        let opc2 = if opcode == OPC_XTN { OPC_SQXTN } else { OPC_XTN };
        let w_opc = sut_word(&ops2(rd, rn, tb, ta), u_bit, opc2, is_high)
            .unwrap_or_else(|e| panic!("SUT rejected flipped opcode: {}", e));
        prop_assert_eq!(
            w ^ w_opc,
            (opcode ^ opc2) << 12,
            "opcode must occupy only bits[16:12]"
        );
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_two_misc_narrow_inv_arm_layout(
        rd in reg_num(),
        rn in reg_num(),
        pair in valid_pair(),
        fam in family(),
    ) {
        let (tb, ta, is_high) = pair;
        let (base, u_bit, opcode) = fam;
        let w = sut_word(&ops2(rd, rn, tb, ta), u_bit, opcode, is_high)
            .unwrap_or_else(|e| panic!("SUT rejected valid layout: {}", e));
        prop_assert!(
            arm_narrow_layout(w, rd, rn, ta, u_bit, opcode, is_high),
            "ARM two-misc-narrow layout violated for {} word={:#010x}",
            asm2(base, rd, rn, tb, ta, is_high),
            w
        );
        let _ = tb;
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_two_misc_narrow_neg_arity(
        rd in reg_num(),
        ta in any_arr(),
        is_high in any::<bool>(),
        u_bit in 0u32..=1,
        opcode in prop::sample::select(vec![OPC_XTN, OPC_SQXTN]),
        n in 0usize..=1,
    ) {
        let ops: Vec<Operand> = if n == 0 {
            vec![]
        } else {
            vec![arr(rd, ta)]
        };
        prop_assert!(
            encode_neon_two_misc_narrow(&ops, u_bit, opcode, is_high).is_err(),
            "arity {} must Err (NEON two-reg narrow requires 2 operands)",
            n
        );
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_two_misc_narrow_neg_extra_operand(
        rd in reg_num(),
        rn in reg_num(),
        extra in reg_num(),
        pair in valid_pair(),
        fam in family(),
    ) {
        let (tb, ta, is_high) = pair;
        let (base, u_bit, opcode) = fam;
        let asm = format!("{}, v{}.{tb}", asm2(base, rd, rn, tb, ta, is_high), extra);
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted 3-operand {}",
            asm
        );
        let mut ops = ops2(rd, rn, tb, ta);
        ops.push(arr(extra, tb));
        prop_assert!(
            encode_neon_two_misc_narrow(&ops, u_bit, opcode, is_high).is_err(),
            "3 operands must Err (llvm-mc rejects {})",
            asm
        );
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_two_misc_narrow_neg_mismatched_tb_ta(
        rd in reg_num(),
        rn in reg_num(),
        tb in any_arr(),
        ta in any_arr(),
        is_high in any::<bool>(),
        fam in family(),
    ) {
        prop_assume!(!is_valid_pair(tb, ta, is_high));
        let (base, u_bit, opcode) = fam;
        let asm = asm2(base, rd, rn, tb, ta, is_high);
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted invalid arrangement {}",
            asm
        );
        let ops = ops2(rd, rn, tb, ta);
        prop_assert!(
            encode_neon_two_misc_narrow(&ops, u_bit, opcode, is_high).is_err(),
            "invalid/mismatched Tb/Ta must Err (ARM XTN Ta in {{8H,4S,2D}} with matching Tb; llvm-mc rejects {})",
            asm
        );
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_two_misc_narrow_neg_gpr_or_bare(
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
                ],
                format!("xtn {fp_prefix}{rd}, v{rn}.8h"),
            ),
            1 => (
                vec![
                    arr(rd, "8b"),
                    Operand::Reg(format!("v{}", rn)),
                ],
                format!("xtn v{rd}.8b, v{rn}"),
            ),
            2 => (
                vec![
                    Operand::Reg(format!("v{}", rd)),
                    arr(rn, "8h"),
                ],
                format!("xtn v{rd}, v{rn}.8h"),
            ),
            3 => (
                vec![
                    Operand::RegArrangement {
                        reg: format!("x{}", rd),
                        arrangement: "8b".to_string(),
                    },
                    arr(rn, "8h"),
                ],
                format!("xtn x{rd}.8b, v{rn}.8h"),
            ),
            _ => (
                vec![
                    arr(rd, "8b"),
                    Operand::Reg(format!("x{}", rn)),
                ],
                format!("xtn v{rd}.8b, x{rn}"),
            ),
        };
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        prop_assert!(
            encode_neon_two_misc_narrow(&ops, 0, OPC_XTN, false).is_err(),
            "GPR/bare/non-arrangement kind={} must Err (llvm-mc rejects {})",
            kind,
            asm
        );
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_two_misc_narrow_diff_alt_spellings(
        rd in reg_num(),
        rn in reg_num(),
        pair in valid_pair(),
        fam in family(),
    ) {
        let (tb, ta, is_high) = pair;
        let (base, u_bit, opcode) = fam;
        let tb_u = tb.to_ascii_uppercase();
        let ta_u = ta.to_ascii_uppercase();
        let m = mnem(base, is_high).to_ascii_uppercase();
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
        let sut = sut_word(&ops, u_bit, opcode, is_high)
            .unwrap_or_else(|e| panic!("SUT rejected alt-spelling {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for alt-spelling {}", asm);
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_two_misc_narrow_neg_nonreg(
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
        let mut ops = ops2(rd, rn, "8b", "8h");
        ops[slot] = bad;
        let asm = match (kind, slot) {
            (0, 0) => "xtn #0, v0.8h".to_string(),
            (1, 0) => format!("xtn [x{rd}], v{rn}.8h"),
            (2, 0) => "xtn L0, v0.8h".to_string(),
            _ => format!("xtn nonreg slot={slot} kind={kind}"),
        };
        if slot == 0 {
            prop_assert!(
                llvm_mc_word(&asm).is_err(),
                "llvm-mc unexpectedly accepted {}",
                asm
            );
        }
        prop_assert!(
            encode_neon_two_misc_narrow(&ops, 0, OPC_XTN, false).is_err(),
            "non-register operand slot={} kind={} must Err",
            slot,
            kind
        );
    }
}

/// Deterministic regression: extra operand ignored (from neg_extra_operand).
#[test]
fn test_encode_neon_two_misc_narrow_regression_extra_operand() {
    let mut ops = ops2(0, 0, "8b", "8h");
    ops.push(arr(0, "8b"));
    assert!(
        encode_neon_two_misc_narrow(&ops, 0, OPC_XTN, false).is_err(),
        "xtn v0.8b, v0.8h, v0.8b must Err (gas/llvm-mc reject a third operand)"
    );
}

/// Deterministic regression: dest 8h / source 4s encoded as XTN Q=0 (from neg_mismatched_tb_ta).
#[test]
fn test_encode_neon_two_misc_narrow_regression_mismatched_tb_ta() {
    assert!(
        encode_neon_two_misc_narrow(&ops2(0, 0, "8h", "4s"), 0, OPC_XTN, false).is_err(),
        "xtn v0.8h, v0.4s must Err (ARM XTN Vn.4S requires Vd.4H; 8H is XTN2)"
    );
}

/// Deterministic regression: dest 8h with valid source 8h still encodes (dest arrangement discarded).
#[test]
fn test_encode_neon_two_misc_narrow_regression_mismatched_dest_tb() {
    assert!(
        encode_neon_two_misc_narrow(&ops2(0, 0, "8h", "8h"), 0, OPC_XTN, false).is_err(),
        "xtn v0.8h, v0.8h must Err (ARM XTN Vn.8H requires Vd.8B)"
    );
}

/// Deterministic regression: bare V dest encoded as Vd.Tb (from neg_gpr_or_bare kind=2).
#[test]
fn test_encode_neon_two_misc_narrow_regression_bare_dest() {
    let ops = vec![Operand::Reg("v0".into()), arr(0, "8h")];
    assert!(
        encode_neon_two_misc_narrow(&ops, 0, OPC_XTN, false).is_err(),
        "xtn v0, v0.8h must Err (gas/llvm-mc require Vd.Tb)"
    );
}

/// Deterministic regression: GPR dest encoded as Vd (from neg_gpr_or_bare kind=0).
#[test]
fn test_encode_neon_two_misc_narrow_regression_gpr_dest() {
    let ops = vec![Operand::Reg("x0".into()), arr(0, "8h")];
    assert!(
        encode_neon_two_misc_narrow(&ops, 0, OPC_XTN, false).is_err(),
        "xtn x0, v0.8h must Err (gas/llvm-mc require Vd.Tb)"
    );
}
