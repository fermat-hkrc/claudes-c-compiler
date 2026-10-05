// Oracle: differential — llvm-mc AArch64 assembler (+aes)
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md:230 NEON widen/long lists pmull (+ 2 variants);
//   encoder/mod.rs:1-7 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:779-780 "pmull"/"pmull2" => encode_neon_pmull;
//   neon.rs:1127 Encode NEON PMULL/PMULL2 (polynomial multiply long);
//   neon.rs:1138-1139 PMULL Vd.1q, Vn.1d, Vm.1d / PMULL2 Vd.1q, Vn.2d, Vm.2d size=11;
//   ARM ARM Advanced SIMD three-different PMULL{2} Ta in {8H,1Q}.
// Stronger considered:
//   - State machine: rejected — encode_neon_pmull is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree PMULL decoder
//   - Differential vs encode_neon_three_diff / encode_neon_pmul: rejected — same-job gate
//     (three_diff is integer widening; PMUL is three-same 8-bit, not long)
// Weaker available: algebraic.metamorphic (Rd/Rn/Rm/Q fields), algebraic.invariant (word layout),
//   negative_error (arity / extra / invalid T / mismatch / GPR dest)
// Differential: candidate=encode_neon_pmull, reference=llvm-mc -triple=aarch64 -mattr=+aes -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[RegArrangement(Vd,1q), RegArrangement(Vn,1d|2d), RegArrangement(Vm,1d|2d)]
//     <-> `pmull{2} Vd.1q, Vn.{1d|2d}, Vm.{1d|2d}`
//     and [RegArrangement(Vd,8h), RegArrangement(Vn,8b|16b), RegArrangement(Vm,8b|16b)]
//     <-> `pmull{2} Vd.8h, Vn.{8b|16b}, Vm.{8b|16b}`

use super::encode_neon_pmull;
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

fn tb_1q(is_pmull2: bool) -> &'static str {
    if is_pmull2 {
        "2d"
    } else {
        "1d"
    }
}

fn tb_8h(is_pmull2: bool) -> &'static str {
    if is_pmull2 {
        "16b"
    } else {
        "8b"
    }
}

fn mnemonic(is_pmull2: bool) -> &'static str {
    if is_pmull2 {
        "pmull2"
    } else {
        "pmull"
    }
}

fn ops_ta(rd: u32, rn: u32, rm: u32, ta: &str, tb: &str) -> Vec<Operand> {
    vec![arr(rd, ta), arr(rn, tb), arr(rm, tb)]
}

fn asm_ta(rd: u32, rn: u32, rm: u32, ta: &str, tb: &str, is_pmull2: bool) -> String {
    format!(
        "{} v{rd}.{ta}, v{rn}.{tb}, v{rm}.{tb}",
        mnemonic(is_pmull2)
    )
}

fn sut_word(ops: &[Operand], is_pmull2: bool) -> Result<u32, String> {
    match encode_neon_pmull(ops, is_pmull2)? {
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
        .args(["-triple=aarch64", "-mattr=+aes", "-show-encoding"])
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

fn is_valid_pmull_triple(ta: &str, tb: &str, is_pmull2: bool) -> bool {
    match (ta, tb, is_pmull2) {
        ("1q", "1d", false) => true,
        ("1q", "2d", true) => true,
        ("8h", "8b", false) => true,
        ("8h", "16b", true) => true,
        _ => false,
    }
}

/// Known-answer gate for the llvm-mc differential connection (1q and 8h forms).
#[test]
fn encode_neon_pmull_kat_llvm_mc() {
    // 1q / size=11 (crypto PMULL, needs +aes)
    let kat_1q: &[(&str, u32)] = &[
        ("pmull v0.1q, v1.1d, v2.1d", 0x0ee2e020),
        ("pmull2 v0.1q, v1.2d, v2.2d", 0x4ee2e020),
        ("pmull v31.1q, v31.1d, v31.1d", 0x0effe3ff),
        ("pmull2 v31.1q, v31.2d, v31.2d", 0x4effe3ff),
        ("pmull v15.1q, v16.1d, v17.1d", 0x0ef1e20f),
        ("pmull v0.1q, v0.1d, v0.1d", 0x0ee0e000),
        ("pmull2 v5.1q, v10.2d, v20.2d", 0x4ef4e145),
    ];
    for &(asm, want) in kat_1q {
        let mc = llvm_mc_word(asm).unwrap_or_else(|e| panic!("llvm-mc KAT {asm}: {e}"));
        assert_eq!(mc, want, "llvm-mc KAT mapping broken for {asm}");
    }
    // 8h / size=00 (baseline AdvSIMD)
    let kat_8h: &[(&str, u32)] = &[
        ("pmull v0.8h, v1.8b, v2.8b", 0x0e22e020),
        ("pmull2 v0.8h, v1.16b, v2.16b", 0x4e22e020),
        ("pmull v31.8h, v31.8b, v31.8b", 0x0e3fe3ff),
        ("pmull2 v31.8h, v31.16b, v31.16b", 0x4e3fe3ff),
    ];
    for &(asm, want) in kat_8h {
        let mc = llvm_mc_word(asm).unwrap_or_else(|e| panic!("llvm-mc KAT {asm}: {e}"));
        assert_eq!(mc, want, "llvm-mc KAT mapping broken for {asm}");
    }
    // SUT 1q form (documented at neon.rs:1138-1139)
    let sut_1q: &[(u32, u32, u32, bool, u32)] = &[
        (0, 1, 2, false, 0x0ee2e020),
        (0, 1, 2, true, 0x4ee2e020),
        (31, 31, 31, false, 0x0effe3ff),
        (31, 31, 31, true, 0x4effe3ff),
        (15, 16, 17, false, 0x0ef1e20f),
        (0, 0, 0, false, 0x0ee0e000),
        (5, 10, 20, true, 0x4ef4e145),
    ];
    for &(rd, rn, rm, is2, want) in sut_1q {
        let tb = tb_1q(is2);
        let asm = asm_ta(rd, rn, rm, "1q", tb, is2);
        let sut = sut_word(&ops_ta(rd, rn, rm, "1q", tb), is2)
            .unwrap_or_else(|e| panic!("SUT KAT {asm}: {e}"));
        assert_eq!(sut, want, "SUT 1q KAT mismatch for {asm}");
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_pmull_diff_llvm_mc_1q(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        is_pmull2 in any::<bool>(),
    ) {
        let tb = tb_1q(is_pmull2);
        let asm = asm_ta(rd, rn, rm, "1q", tb, is_pmull2);
        let ops = ops_ta(rd, rn, rm, "1q", tb);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&ops, is_pmull2)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for {}", asm);
    }

    #[test]
    fn encode_neon_pmull_diff_llvm_mc_8h(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        is_pmull2 in any::<bool>(),
    ) {
        let tb = tb_8h(is_pmull2);
        let asm = asm_ta(rd, rn, rm, "8h", tb, is_pmull2);
        let ops = ops_ta(rd, rn, rm, "8h", tb);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&ops, is_pmull2)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for {}", asm);
    }

    #[test]
    fn encode_neon_pmull_metamorphic_rd_rn_rm_q(
        rd1 in reg_num(),
        rd2 in reg_num(),
        rn1 in reg_num(),
        rn2 in reg_num(),
        rm1 in reg_num(),
        rm2 in reg_num(),
    ) {
        let w1110 = sut_word(&ops_ta(rd1, rn1, rm1, "1q", "1d"), false)
            .unwrap_or_else(|e| panic!("SUT rejected rd1/rn1/rm1: {}", e));
        let w2110 = sut_word(&ops_ta(rd2, rn1, rm1, "1q", "1d"), false)
            .unwrap_or_else(|e| panic!("SUT rejected rd2: {}", e));
        let w1210 = sut_word(&ops_ta(rd1, rn2, rm1, "1q", "1d"), false)
            .unwrap_or_else(|e| panic!("SUT rejected rn2: {}", e));
        let w1120 = sut_word(&ops_ta(rd1, rn1, rm2, "1q", "1d"), false)
            .unwrap_or_else(|e| panic!("SUT rejected rm2: {}", e));
        let w1111 = sut_word(&ops_ta(rd1, rn1, rm1, "1q", "2d"), true)
            .unwrap_or_else(|e| panic!("SUT rejected pmull2: {}", e));
        prop_assert_eq!(
            (w1110 ^ w2110) & !0x1Fu32,
            0u32,
            "changing only Rd must differ only in bits[4:0]"
        );
        prop_assert_eq!(w2110 & 0x1F, rd2, "Rd field");
        prop_assert_eq!(
            (w1110 ^ w1210) & !(0x1Fu32 << 5),
            0u32,
            "changing only Rn must differ only in bits[9:5]"
        );
        prop_assert_eq!((w1210 >> 5) & 0x1F, rn2, "Rn field");
        prop_assert_eq!(
            (w1110 ^ w1120) & !(0x1Fu32 << 16),
            0u32,
            "changing only Rm must differ only in bits[20:16]"
        );
        prop_assert_eq!((w1120 >> 16) & 0x1F, rm2, "Rm field");
        prop_assert_eq!(
            (w1110 ^ w1111) & !(1u32 << 30),
            0u32,
            "pmull vs pmull2 must differ only in Q bit 30"
        );
        prop_assert_eq!((w1111 >> 30) & 1, 1u32, "Q=1 for pmull2");
        prop_assert_eq!((w1110 >> 30) & 1, 0u32, "Q=0 for pmull");
    }

    #[test]
    fn encode_neon_pmull_invariant_arm_64bit_fields(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        is_pmull2 in any::<bool>(),
    ) {
        let tb = tb_1q(is_pmull2);
        let w = sut_word(&ops_ta(rd, rn, rm, "1q", tb), is_pmull2)
            .unwrap_or_else(|e| panic!("SUT rejected: {}", e));
        prop_assert_eq!((w >> 31) & 1, 0u32, "bit31=0");
        prop_assert_eq!((w >> 30) & 1, if is_pmull2 { 1 } else { 0 }, "Q");
        prop_assert_eq!((w >> 24) & 0x3F, 0b001110u32, "bits[29:24]=001110");
        prop_assert_eq!((w >> 22) & 3, 0b11u32, "size=11");
        prop_assert_eq!((w >> 21) & 1, 1u32, "bit21=1");
        prop_assert_eq!((w >> 16) & 0x1F, rm, "Rm");
        prop_assert_eq!((w >> 11) & 0x1F, 0b11100u32, "bits[15:11]=11100");
        prop_assert_eq!((w >> 10) & 1, 0u32, "bit10=0");
        prop_assert_eq!((w >> 5) & 0x1F, rn, "Rn");
        prop_assert_eq!(w & 0x1F, rd, "Rd");
    }

    #[test]
    fn encode_neon_pmull_neg_arity(
        n in 0usize..=2,
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        is_pmull2 in any::<bool>(),
    ) {
        let tb = tb_1q(is_pmull2);
        let all = ops_ta(rd, rn, rm, "1q", tb);
        let arity_ops: Vec<Operand> = all.iter().take(n).cloned().collect();
        let mn = mnemonic(is_pmull2);
        let arity_asm = match n {
            0 => mn.to_string(),
            1 => format!("{mn} v{rd}.1q"),
            _ => format!("{mn} v{rd}.1q, v{rn}.{tb}"),
        };
        prop_assert!(
            llvm_mc_word(&arity_asm).is_err(),
            "llvm-mc unexpectedly accepted arity-{} {}",
            n,
            arity_asm
        );
        prop_assert!(
            encode_neon_pmull(&arity_ops, is_pmull2).is_err(),
            "arity {} must Err (llvm-mc rejects {})",
            n,
            arity_asm
        );
    }

    #[test]
    fn encode_neon_pmull_neg_extra_operand(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        extra in reg_num(),
        is_pmull2 in any::<bool>(),
    ) {
        let tb = tb_1q(is_pmull2);
        let asm = format!("{}, v{}.{tb}", asm_ta(rd, rn, rm, "1q", tb, is_pmull2), extra);
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted 4-operand {}",
            asm
        );
        let mut ops = ops_ta(rd, rn, rm, "1q", tb);
        ops.push(arr(extra, tb));
        prop_assert!(
            encode_neon_pmull(&ops, is_pmull2).is_err(),
            "4 operands must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_pmull_neg_invalid_t(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        is_pmull2 in any::<bool>(),
        td in any_t(),
        tn in any_t(),
        tm in any_t(),
    ) {
        prop_assume!(!is_valid_pmull_triple(td, tn, is_pmull2) || tn != tm);
        // If (td,tn,is_pmull2) is valid and tn==tm, this is the legal form — skip.
        if is_valid_pmull_triple(td, tn, is_pmull2) && tn == tm {
            return Ok(());
        }
        let mn = mnemonic(is_pmull2);
        let asm = format!("{mn} v{rd}.{td}, v{rn}.{tn}, v{rm}.{tm}");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted invalid T {}",
            asm
        );
        let ops = vec![arr(rd, td), arr(rn, tn), arr(rm, tm)];
        prop_assert!(
            encode_neon_pmull(&ops, is_pmull2).is_err(),
            "invalid Ta/Tb must Err (ARM PMULL Ta in {{8H,1Q}} with matching Tb; llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_pmull_neg_gpr_or_bare(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        is_pmull2 in any::<bool>(),
        kind in 0u8..=4,
        fp_prefix in prop::sample::select(vec!["x", "w", "d", "s", "q", "h", "b"]),
    ) {
        let tb = tb_1q(is_pmull2);
        let mn = mnemonic(is_pmull2);
        let (ops, asm) = match kind {
            0 => (
                vec![
                    Operand::Reg(format!("{}{}", fp_prefix, rd)),
                    arr(rn, tb),
                    arr(rm, tb),
                ],
                format!("{mn} {fp_prefix}{rd}, v{rn}.{tb}, v{rm}.{tb}"),
            ),
            1 => (
                vec![
                    arr(rd, "1q"),
                    Operand::Reg(format!("v{}", rn)),
                    arr(rm, tb),
                ],
                format!("{mn} v{rd}.1q, v{rn}, v{rm}.{tb}"),
            ),
            2 => (
                vec![
                    arr(rd, "1q"),
                    arr(rn, tb),
                    Operand::Reg(format!("x{}", rm)),
                ],
                format!("{mn} v{rd}.1q, v{rn}.{tb}, x{rm}"),
            ),
            3 => (
                vec![
                    Operand::Reg(format!("v{}", rd)),
                    arr(rn, tb),
                    arr(rm, tb),
                ],
                format!("{mn} v{rd}, v{rn}.{tb}, v{rm}.{tb}"),
            ),
            _ => (
                vec![
                    Operand::RegArrangement {
                        reg: format!("x{}", rd),
                        arrangement: "1q".to_string(),
                    },
                    arr(rn, tb),
                    arr(rm, tb),
                ],
                format!("{mn} x{rd}.1q, v{rn}.{tb}, v{rm}.{tb}"),
            ),
        };
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        prop_assert!(
            encode_neon_pmull(&ops, is_pmull2).is_err(),
            "GPR/bare/non-arrangement kind={} must Err (llvm-mc rejects {})",
            kind,
            asm
        );
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_pmull_diff_alt_spellings(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        is_pmull2 in any::<bool>(),
    ) {
        let tb = tb_1q(is_pmull2);
        let tb_u = if is_pmull2 { "2D" } else { "1D" };
        let mn = if is_pmull2 { "PMULL2" } else { "PMULL" };
        let asm = format!("{mn} V{rd}.1Q, V{rn}.{tb_u}, V{rm}.{tb_u}");
        let ops = vec![
            Operand::RegArrangement {
                reg: format!("V{}", rd),
                arrangement: "1q".to_string(),
            },
            Operand::RegArrangement {
                reg: format!("V{}", rn),
                arrangement: tb.to_string(),
            },
            Operand::RegArrangement {
                reg: format!("V{}", rm),
                arrangement: tb.to_string(),
            },
        ];
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected alt-spelling {}: {}", asm, e));
        let sut = sut_word(&ops, is_pmull2)
            .unwrap_or_else(|e| panic!("SUT rejected alt-spelling {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for alt-spelling {}", asm);
    }

    #[test]
    fn encode_neon_pmull_neg_nonreg(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        is_pmull2 in any::<bool>(),
        kind in 0u8..=2,
        slot in 0usize..=2,
    ) {
        let tb = tb_1q(is_pmull2);
        let mn = mnemonic(is_pmull2);
        let bad = match kind {
            0 => Operand::Imm(0),
            1 => Operand::Mem {
                base: format!("x{}", rd),
                offset: 0,
            },
            _ => Operand::Label("L0".into()),
        };
        let mut ops = ops_ta(rd, rn, rm, "1q", tb);
        ops[slot] = bad;
        let asm = match (kind, slot) {
            (0, 0) => format!("{mn} #0, v{rn}.{tb}, v{rm}.{tb}"),
            (1, 0) => format!("{mn} [x{rd}], v{rn}.{tb}, v{rm}.{tb}"),
            (2, 0) => format!("{mn} L0, v{rn}.{tb}, v{rm}.{tb}"),
            _ => format!("{mn} nonreg slot={slot} kind={kind}"),
        };
        if slot == 0 {
            prop_assert!(
                llvm_mc_word(&asm).is_err(),
                "llvm-mc unexpectedly accepted {}",
                asm
            );
        }
        prop_assert!(
            encode_neon_pmull(&ops, is_pmull2).is_err(),
            "non-register operand slot={} kind={} must Err",
            slot,
            kind
        );
    }
}

/// Deterministic regression: 8-bit PMULL encoded as 64-bit (from diff_llvm_mc_8h).
#[test]
fn test_encode_neon_pmull_regression_8h_as_64bit() {
    let ops = ops_ta(0, 0, 0, "8h", "8b");
    let sut = sut_word(&ops, false).expect("SUT must encode valid pmull v0.8h, v0.8b, v0.8b");
    assert_eq!(
        sut, 0x0e20e000,
        "pmull v0.8h, v0.8b, v0.8b must encode size=00 (llvm-mc 0x0e20e000), not size=11"
    );
}

/// Deterministic regression: extra operand ignored (from neg_extra_operand).
#[test]
fn test_encode_neon_pmull_regression_extra_operand() {
    let mut ops = ops_ta(0, 0, 0, "1q", "1d");
    ops.push(arr(0, "1d"));
    assert!(
        encode_neon_pmull(&ops, false).is_err(),
        "pmull v0.1q, v0.1d, v0.1d, v0.1d must Err (gas/llvm-mc reject a fourth operand)"
    );
}

/// Deterministic regression: invalid .8b arrangement encoded (from neg_invalid_t).
#[test]
fn test_encode_neon_pmull_regression_invalid_t() {
    let ops = ops_ta(0, 0, 0, "8b", "8b");
    assert!(
        encode_neon_pmull(&ops, false).is_err(),
        "pmull v0.8b, v0.8b, v0.8b must Err (ARM PMULL Ta is 8H or 1Q; gas/llvm-mc reject)"
    );
}

/// Deterministic regression: GPR dest encoded (from neg_gpr_or_bare).
#[test]
fn test_encode_neon_pmull_regression_gpr_dest() {
    let ops = vec![
        Operand::Reg("x0".into()),
        arr(0, "1d"),
        arr(0, "1d"),
    ];
    assert!(
        encode_neon_pmull(&ops, false).is_err(),
        "pmull x0, v0.1d, v0.1d must Err (gas/llvm-mc require Vd.1Q or Vd.8H)"
    );
}
