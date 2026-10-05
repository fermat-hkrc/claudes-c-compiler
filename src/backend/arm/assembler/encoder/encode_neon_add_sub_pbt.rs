// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md:224 NEON three-same lists add, sub;
//   encoder/mod.rs:1-7 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:259-266 "add"/"sub" => encode_add_sub / scalar three-same;
//   data_processing.rs:296-300 NEON vector form ADD/SUB Vd.T, Vn.T, Vm.T => encode_neon_add_sub;
//   neon.rs:1163 Encode NEON ADD/SUB (vector integer): ADD/SUB Vd.T, Vn.T, Vm.T;
//   neon.rs:1171-1172 ADD 0 Q 0 01110 size 1 Rm 10000 1 Rn Rd / SUB U=1;
//   ARM ARM Advanced SIMD three-same ADD/SUB T in {8B,16B,4H,8H,2S,4S,2D}; size:Q=11:0 reserved.
// Stronger considered:
//   - State machine: rejected — encode_neon_add_sub is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree vector ADD/SUB decoder
//   - Differential vs encode_neon_three_same / encode_neon_scalar_three_same: rejected — same-job gate
//     (three_same is a generic opcode table; scalar three-same is Dd,Dn,Dm)
// Weaker available: algebraic.metamorphic (Rd/Rn/Rm/U fields), algebraic.invariant (word layout),
//   negative_error (arity / extra / reserved 1d / mismatch / GPR dest)
// Differential: candidate=encode_neon_add_sub, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[RegArrangement(Vd,T), RegArrangement(Vn,T), RegArrangement(Vm,T)]
//     <-> `add|sub Vd.T, Vn.T, Vm.T` for T in {8b,16b,4h,8h,2s,4s,2d}

use super::encode_neon_add_sub;
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

fn mnemonic(is_sub: bool) -> &'static str {
    if is_sub {
        "sub"
    } else {
        "add"
    }
}

fn ops_t(rd: u32, rn: u32, rm: u32, t: &str) -> Vec<Operand> {
    vec![arr(rd, t), arr(rn, t), arr(rm, t)]
}

fn asm_t(rd: u32, rn: u32, rm: u32, t: &str, is_sub: bool) -> String {
    format!("{} v{rd}.{t}, v{rn}.{t}, v{rm}.{t}", mnemonic(is_sub))
}

fn sut_word(ops: &[Operand], is_sub: bool) -> Result<u32, String> {
    match encode_neon_add_sub(ops, is_sub)? {
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

fn is_valid_add_sub_t(t: &str) -> bool {
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
fn encode_neon_add_sub_kat_llvm_mc() {
    let kat: &[(&str, u32)] = &[
        ("add v0.8b, v1.8b, v2.8b", 0x0e228420),
        ("add v0.16b, v1.16b, v2.16b", 0x4e228420),
        ("add v0.4h, v1.4h, v2.4h", 0x0e628420),
        ("add v0.8h, v1.8h, v2.8h", 0x4e628420),
        ("add v0.2s, v1.2s, v2.2s", 0x0ea28420),
        ("add v0.4s, v1.4s, v2.4s", 0x4ea28420),
        ("add v0.2d, v1.2d, v2.2d", 0x4ee28420),
        ("sub v0.8b, v1.8b, v2.8b", 0x2e228420),
        ("sub v0.16b, v1.16b, v2.16b", 0x6e228420),
        ("sub v0.2d, v1.2d, v2.2d", 0x6ee28420),
        ("add v31.8b, v31.8b, v31.8b", 0x0e3f87ff),
        ("add v0.8b, v0.8b, v0.8b", 0x0e208400),
        ("add v15.4s, v16.4s, v17.4s", 0x4eb1860f),
    ];
    for &(asm, want) in kat {
        let mc = llvm_mc_word(asm).unwrap_or_else(|e| panic!("llvm-mc KAT {asm}: {e}"));
        assert_eq!(mc, want, "llvm-mc KAT mapping broken for {asm}");
    }
    let sut_kat: &[(u32, u32, u32, &str, bool, u32)] = &[
        (0, 1, 2, "8b", false, 0x0e228420),
        (0, 1, 2, "16b", false, 0x4e228420),
        (0, 1, 2, "4h", false, 0x0e628420),
        (0, 1, 2, "8h", false, 0x4e628420),
        (0, 1, 2, "2s", false, 0x0ea28420),
        (0, 1, 2, "4s", false, 0x4ea28420),
        (0, 1, 2, "2d", false, 0x4ee28420),
        (0, 1, 2, "8b", true, 0x2e228420),
        (0, 1, 2, "16b", true, 0x6e228420),
        (0, 1, 2, "2d", true, 0x6ee28420),
        (31, 31, 31, "8b", false, 0x0e3f87ff),
        (0, 0, 0, "8b", false, 0x0e208400),
        (15, 16, 17, "4s", false, 0x4eb1860f),
    ];
    for &(rd, rn, rm, t, is_sub, want) in sut_kat {
        let asm = asm_t(rd, rn, rm, t, is_sub);
        let sut = sut_word(&ops_t(rd, rn, rm, t), is_sub)
            .unwrap_or_else(|e| panic!("SUT KAT {asm}: {e}"));
        assert_eq!(sut, want, "SUT KAT mismatch for {asm}");
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_add_sub_diff_llvm_mc(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        t in valid_t(),
        is_sub in any::<bool>(),
    ) {
        let asm = asm_t(rd, rn, rm, t, is_sub);
        let ops = ops_t(rd, rn, rm, t);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&ops, is_sub)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for {}", asm);
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_add_sub_metamorphic_rd_rn_rm_u(
        rd1 in reg_num(),
        rd2 in reg_num(),
        rn1 in reg_num(),
        rn2 in reg_num(),
        rm1 in reg_num(),
        rm2 in reg_num(),
    ) {
        let w1110 = sut_word(&ops_t(rd1, rn1, rm1, "8b"), false)
            .unwrap_or_else(|e| panic!("SUT rejected rd1/rn1/rm1: {}", e));
        let w2110 = sut_word(&ops_t(rd2, rn1, rm1, "8b"), false)
            .unwrap_or_else(|e| panic!("SUT rejected rd2: {}", e));
        let w1210 = sut_word(&ops_t(rd1, rn2, rm1, "8b"), false)
            .unwrap_or_else(|e| panic!("SUT rejected rn2: {}", e));
        let w1120 = sut_word(&ops_t(rd1, rn1, rm2, "8b"), false)
            .unwrap_or_else(|e| panic!("SUT rejected rm2: {}", e));
        let w1111 = sut_word(&ops_t(rd1, rn1, rm1, "8b"), true)
            .unwrap_or_else(|e| panic!("SUT rejected sub: {}", e));
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
            (w1110 ^ w1111) & !(1u32 << 29),
            0u32,
            "add vs sub must differ only in U bit 29"
        );
        prop_assert_eq!((w1111 >> 29) & 1, 1u32, "U=1 for sub");
        prop_assert_eq!((w1110 >> 29) & 1, 0u32, "U=0 for add");
    }

    #[test]
    fn encode_neon_add_sub_invariant_arm_fields(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        t in valid_t(),
        is_sub in any::<bool>(),
    ) {
        let w = sut_word(&ops_t(rd, rn, rm, t), is_sub)
            .unwrap_or_else(|e| panic!("SUT rejected: {}", e));
        let (q, size) = q_size_of(t);
        prop_assert_eq!((w >> 31) & 1, 0u32, "bit31=0");
        prop_assert_eq!((w >> 30) & 1, q, "Q");
        prop_assert_eq!((w >> 29) & 1, if is_sub { 1 } else { 0 }, "U");
        prop_assert_eq!((w >> 24) & 0x1F, 0b01110u32, "bits[28:24]=01110");
        prop_assert_eq!((w >> 22) & 3, size, "size");
        prop_assert_eq!((w >> 21) & 1, 1u32, "bit21=1");
        prop_assert_eq!((w >> 16) & 0x1F, rm, "Rm");
        prop_assert_eq!((w >> 11) & 0x1F, 0b10000u32, "bits[15:11]=10000");
        prop_assert_eq!((w >> 10) & 1, 1u32, "bit10=1");
        prop_assert_eq!((w >> 5) & 0x1F, rn, "Rn");
        prop_assert_eq!(w & 0x1F, rd, "Rd");
    }

    #[test]
    fn encode_neon_add_sub_neg_arity(
        n in 0usize..=2,
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        is_sub in any::<bool>(),
    ) {
        let all = ops_t(rd, rn, rm, "8b");
        let arity_ops: Vec<Operand> = all.iter().take(n).cloned().collect();
        let mn = mnemonic(is_sub);
        let arity_asm = match n {
            0 => mn.to_string(),
            1 => format!("{mn} v{rd}.8b"),
            _ => format!("{mn} v{rd}.8b, v{rn}.8b"),
        };
        prop_assert!(
            llvm_mc_word(&arity_asm).is_err(),
            "llvm-mc unexpectedly accepted arity-{} {}",
            n,
            arity_asm
        );
        prop_assert!(
            encode_neon_add_sub(&arity_ops, is_sub).is_err(),
            "arity {} must Err (llvm-mc rejects {})",
            n,
            arity_asm
        );
    }

    #[test]
    fn encode_neon_add_sub_neg_extra_operand(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        extra in reg_num(),
        t in valid_t(),
        is_sub in any::<bool>(),
    ) {
        let asm = format!("{}, v{}.{t}", asm_t(rd, rn, rm, t, is_sub), extra);
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted 4-operand {}",
            asm
        );
        let mut ops = ops_t(rd, rn, rm, t);
        ops.push(arr(extra, t));
        prop_assert!(
            encode_neon_add_sub(&ops, is_sub).is_err(),
            "4 operands must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_add_sub_neg_invalid_t(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        is_sub in any::<bool>(),
        td in any_t(),
        tn in any_t(),
        tm in any_t(),
    ) {
        prop_assume!(!(is_valid_add_sub_t(td) && td == tn && tn == tm));
        let mn = mnemonic(is_sub);
        let asm = format!("{mn} v{rd}.{td}, v{rn}.{tn}, v{rm}.{tm}");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted invalid T {}",
            asm
        );
        let ops = vec![arr(rd, td), arr(rn, tn), arr(rm, tm)];
        prop_assert!(
            encode_neon_add_sub(&ops, is_sub).is_err(),
            "invalid/mismatched/reserved T must Err (ARM ADD/SUB T in {{8B,16B,4H,8H,2S,4S,2D}} matching; llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_add_sub_neg_gpr_or_bare(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        is_sub in any::<bool>(),
        kind in 0u8..=4,
        fp_prefix in prop::sample::select(vec!["x", "w", "d", "s", "q", "h", "b"]),
    ) {
        let mn = mnemonic(is_sub);
        let (ops, asm) = match kind {
            0 => (
                vec![
                    Operand::Reg(format!("{}{}", fp_prefix, rd)),
                    arr(rn, "8b"),
                    arr(rm, "8b"),
                ],
                format!("{mn} {fp_prefix}{rd}, v{rn}.8b, v{rm}.8b"),
            ),
            1 => (
                vec![
                    arr(rd, "8b"),
                    Operand::Reg(format!("v{}", rn)),
                    arr(rm, "8b"),
                ],
                format!("{mn} v{rd}.8b, v{rn}, v{rm}.8b"),
            ),
            2 => (
                vec![
                    arr(rd, "8b"),
                    arr(rn, "8b"),
                    Operand::Reg(format!("x{}", rm)),
                ],
                format!("{mn} v{rd}.8b, v{rn}.8b, x{rm}"),
            ),
            3 => (
                vec![
                    Operand::Reg(format!("v{}", rd)),
                    arr(rn, "8b"),
                    arr(rm, "8b"),
                ],
                format!("{mn} v{rd}, v{rn}.8b, v{rm}.8b"),
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
                format!("{mn} x{rd}.8b, v{rn}.8b, v{rm}.8b"),
            ),
        };
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        prop_assert!(
            encode_neon_add_sub(&ops, is_sub).is_err(),
            "GPR/bare/non-arrangement kind={} must Err (llvm-mc rejects {})",
            kind,
            asm
        );
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_add_sub_diff_alt_spellings(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        t in valid_t(),
        is_sub in any::<bool>(),
    ) {
        let t_u = t.to_ascii_uppercase();
        let mn = if is_sub { "SUB" } else { "ADD" };
        let asm = format!("{mn} V{rd}.{t_u}, V{rn}.{t_u}, V{rm}.{t_u}");
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
        let sut = sut_word(&ops, is_sub)
            .unwrap_or_else(|e| panic!("SUT rejected alt-spelling {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for alt-spelling {}", asm);
    }

    #[test]
    fn encode_neon_add_sub_neg_nonreg(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        is_sub in any::<bool>(),
        kind in 0u8..=2,
        slot in 0usize..=2,
    ) {
        let mn = mnemonic(is_sub);
        let bad = match kind {
            0 => Operand::Imm(0),
            1 => Operand::Mem {
                base: format!("x{}", rd),
                offset: 0,
            },
            _ => Operand::Label("L0".into()),
        };
        let mut ops = ops_t(rd, rn, rm, "8b");
        ops[slot] = bad;
        let asm = match (kind, slot) {
            (0, 0) => format!("{mn} #0, v{rn}.8b, v{rm}.8b"),
            (1, 0) => format!("{mn} [x{rd}], v{rn}.8b, v{rm}.8b"),
            (2, 0) => format!("{mn} L0, v{rn}.8b, v{rm}.8b"),
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
            encode_neon_add_sub(&ops, is_sub).is_err(),
            "non-register operand slot={} kind={} must Err",
            slot,
            kind
        );
    }
}

/// Deterministic regression: extra operand ignored (from neg_extra_operand).
#[test]
fn test_encode_neon_add_sub_regression_extra_operand() {
    let mut ops = ops_t(0, 0, 0, "8b");
    ops.push(arr(0, "8b"));
    assert!(
        encode_neon_add_sub(&ops, false).is_err(),
        "add v0.8b, v0.8b, v0.8b, v0.8b must Err (gas/llvm-mc reject a fourth operand)"
    );
}

/// Deterministic regression: mismatched T encoded (from neg_invalid_t).
#[test]
fn test_encode_neon_add_sub_regression_mismatched_t() {
    let ops = vec![arr(0, "8b"), arr(0, "8b"), arr(0, "16b")];
    assert!(
        encode_neon_add_sub(&ops, false).is_err(),
        "add v0.8b, v0.8b, v0.16b must Err (ARM/gas/llvm-mc require matching T)"
    );
}

/// Deterministic regression: reserved .1d encoded (from neg_invalid_t).
#[test]
fn test_encode_neon_add_sub_regression_reserved_1d() {
    let ops = ops_t(0, 0, 0, "1d");
    assert!(
        encode_neon_add_sub(&ops, false).is_err(),
        "add v0.1d, v0.1d, v0.1d must Err (ARM size:Q=11:0 reserved; llvm-mc rejects)"
    );
}

/// Deterministic regression: bare V source encoded (from neg_gpr_or_bare).
#[test]
fn test_encode_neon_add_sub_regression_bare_src() {
    let ops = vec![arr(0, "8b"), Operand::Reg("v0".into()), arr(0, "8b")];
    assert!(
        encode_neon_add_sub(&ops, false).is_err(),
        "add v0.8b, v0, v0.8b must Err (gas/llvm-mc require Vn.T)"
    );
}

/// Deterministic regression: GPR dest with arrangement encoded (from neg_gpr_or_bare).
#[test]
fn test_encode_neon_add_sub_regression_gpr_dest() {
    let ops = vec![
        Operand::RegArrangement {
            reg: "x0".into(),
            arrangement: "8b".to_string(),
        },
        arr(0, "8b"),
        arr(0, "8b"),
    ];
    assert!(
        encode_neon_add_sub(&ops, false).is_err(),
        "add x0.8b, v0.8b, v0.8b must Err (gas/llvm-mc require Vd.T)"
    );
}
