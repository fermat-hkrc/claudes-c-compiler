// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md:233 NEON permute lists ext;
//   encoder/mod.rs:1-7 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:675 "ext" => encode_neon_ext;
//   neon.rs:404 Encode NEON EXT Vd.T, Vn.T, Vm.T, #index;
//   neon.rs:412 Encoding: 0 Q 10 1110 00 0 Rm 0 imm4 0 Rn Rd;
//   ARM ARM Advanced SIMD extract (EXT). T in {8B,16B}, matching arrangements,
//   index 0-7 (8B) / 0-15 (16B), Q=1 iff T=16B.
// Stronger considered:
//   - State machine: rejected — encode_neon_ext is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree EXT decoder
//   - Differential vs encode_neon_tbl / encode_neon_tbx / encode_neon_zip_uzp: rejected — same-job gate
//     (TBL/TBX table lookup, ZIP/UZP permute with size field vs EXT extract 0 Q 101110)
// Weaker available: algebraic.metamorphic (Rd/Rn/Rm fields), algebraic.invariant (word layout),
//   negative_error (arity / extra / invalid T / index OOR / mismatch / GPR dest)
// Differential: candidate=encode_neon_ext, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[RegArrangement(Vd,T), RegArrangement(Vn,T), RegArrangement(Vm,T), Imm(i)]
//     <-> `ext Vd.T, Vn.T, Vm.T, #i`

use super::encode_neon_ext;
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

fn imax(t: &str) -> u32 {
    match t {
        "16b" => 15,
        _ => 7,
    }
}

fn arr(reg: u32, t: &str) -> Operand {
    Operand::RegArrangement {
        reg: vreg(reg),
        arrangement: t.to_string(),
    }
}

fn ext_ops(rd: u32, rn: u32, rm: u32, t: &str, i: i64) -> Vec<Operand> {
    vec![arr(rd, t), arr(rn, t), arr(rm, t), Operand::Imm(i)]
}

fn ext_asm(rd: u32, rn: u32, rm: u32, t: &str, i: i64) -> String {
    format!("ext v{}.{t}, v{}.{t}, v{}.{t}, #{i}", rd, rn, rm)
}

fn sut_word(ops: &[Operand]) -> Result<u32, String> {
    match encode_neon_ext(ops)? {
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

fn t_size() -> impl Strategy<Value = &'static str> {
    prop::sample::select(vec!["8b", "16b"])
}

fn t_and_idx() -> impl Strategy<Value = (&'static str, u32)> {
    t_size().prop_flat_map(|t| {
        let m = imax(t);
        (Just(t), prop_oneof![Just(0u32), Just(m), 0u32..=m])
    })
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_neon_ext_kat_llvm_mc() {
    let want = 0x6e021820u32;
    let mc = llvm_mc_word("ext v0.16b, v1.16b, v2.16b, #3").expect("llvm-mc KAT 16b #3");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken for ext v0.16b, v1.16b, v2.16b, #3");
    assert_eq!(
        sut_word(&ext_ops(0, 1, 2, "16b", 3)).expect("SUT KAT 16b #3"),
        want
    );

    let want8 = 0x2e021820u32;
    let mc8 = llvm_mc_word("ext v0.8b, v1.8b, v2.8b, #3").expect("llvm-mc KAT 8b #3");
    assert_eq!(mc8, want8);
    assert_eq!(
        sut_word(&ext_ops(0, 1, 2, "8b", 3)).expect("SUT KAT 8b #3"),
        want8
    );

    let want0 = 0x2e020020u32;
    let mc0 = llvm_mc_word("ext v0.8b, v1.8b, v2.8b, #0").expect("llvm-mc KAT 8b #0");
    assert_eq!(mc0, want0);
    assert_eq!(
        sut_word(&ext_ops(0, 1, 2, "8b", 0)).expect("SUT KAT 8b #0"),
        want0
    );

    let want15 = 0x6e1d7bdfu32;
    let mc15 = llvm_mc_word("ext v31.16b, v30.16b, v29.16b, #15").expect("llvm-mc KAT v31 #15");
    assert_eq!(mc15, want15);
    assert_eq!(
        sut_word(&ext_ops(31, 30, 29, "16b", 15)).expect("SUT KAT v31 #15"),
        want15
    );

    let want7 = 0x2e1f381fu32;
    let mc7 = llvm_mc_word("ext v31.8b, v0.8b, v31.8b, #7").expect("llvm-mc KAT 8b #7");
    assert_eq!(mc7, want7);
    assert_eq!(
        sut_word(&ext_ops(31, 0, 31, "8b", 7)).expect("SUT KAT 8b #7"),
        want7
    );

    let want_i0_16 = 0x6e020020u32;
    let mc_i0 = llvm_mc_word("ext v0.16b, v1.16b, v2.16b, #0").expect("llvm-mc KAT 16b #0");
    assert_eq!(mc_i0, want_i0_16);
    assert_eq!(
        sut_word(&ext_ops(0, 1, 2, "16b", 0)).expect("SUT KAT 16b #0"),
        want_i0_16
    );
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_ext_diff_llvm_mc(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        (t, i) in t_and_idx(),
    ) {
        let asm = ext_asm(rd, rn, rm, t, i as i64);
        let ops = ext_ops(rd, rn, rm, t, i as i64);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&ops)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for {}", asm);
    }

    #[test]
    fn encode_neon_ext_metamorphic_rd_rn_rm(
        rd1 in reg_num(),
        rd2 in reg_num(),
        rn1 in reg_num(),
        rn2 in reg_num(),
        rm1 in reg_num(),
        rm2 in reg_num(),
        (t, i) in t_and_idx(),
    ) {
        let w111 = sut_word(&ext_ops(rd1, rn1, rm1, t, i as i64))
            .unwrap_or_else(|e| panic!("SUT rejected rd1/rn1/rm1: {}", e));
        let w211 = sut_word(&ext_ops(rd2, rn1, rm1, t, i as i64))
            .unwrap_or_else(|e| panic!("SUT rejected rd2: {}", e));
        let w121 = sut_word(&ext_ops(rd1, rn2, rm1, t, i as i64))
            .unwrap_or_else(|e| panic!("SUT rejected rn2: {}", e));
        let w112 = sut_word(&ext_ops(rd1, rn1, rm2, t, i as i64))
            .unwrap_or_else(|e| panic!("SUT rejected rm2: {}", e));
        prop_assert_eq!(
            (w111 ^ w211) & !0x1Fu32,
            0u32,
            "changing only Rd must differ only in bits[4:0]"
        );
        prop_assert_eq!(w211 & 0x1F, rd2, "Rd field");
        prop_assert_eq!(
            (w111 ^ w121) & !(0x1Fu32 << 5),
            0u32,
            "changing only Rn must differ only in bits[9:5]"
        );
        prop_assert_eq!((w121 >> 5) & 0x1F, rn2, "Rn field");
        prop_assert_eq!(
            (w111 ^ w112) & !(0x1Fu32 << 16),
            0u32,
            "changing only Rm must differ only in bits[20:16]"
        );
        prop_assert_eq!((w112 >> 16) & 0x1F, rm2, "Rm field");
    }

    #[test]
    fn encode_neon_ext_invariant_arm_fields(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        (t, i) in t_and_idx(),
    ) {
        let w = sut_word(&ext_ops(rd, rn, rm, t, i as i64))
            .unwrap_or_else(|e| panic!("SUT rejected: {}", e));
        let q = if t == "16b" { 1u32 } else { 0 };
        prop_assert_eq!((w >> 31) & 1, 0u32, "bit 31 must be 0");
        prop_assert_eq!((w >> 30) & 1, q, "Q/bit30 = 1 iff T=16b");
        prop_assert_eq!((w >> 24) & 0x3F, 0b101110u32, "bits[29:24]=101110");
        prop_assert_eq!((w >> 21) & 0x7, 0u32, "bits[23:21]=000");
        prop_assert_eq!((w >> 16) & 0x1F, rm, "Rm");
        prop_assert_eq!((w >> 15) & 1, 0u32, "bit 15 = 0");
        prop_assert_eq!((w >> 11) & 0xF, i, "imm4");
        prop_assert_eq!((w >> 10) & 1, 0u32, "bit 10 = 0");
        prop_assert_eq!((w >> 5) & 0x1F, rn, "Rn");
        prop_assert_eq!(w & 0x1F, rd, "Rd");
    }

    #[test]
    fn encode_neon_ext_diff_alt_spellings(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        (t, i) in t_and_idx(),
    ) {
        let t_up = t.to_uppercase();
        let asm = format!(
            "ext V{}.{t_up}, V{}.{t_up}, V{}.{t_up}, #{i}",
            rd, rn, rm
        );
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
            Operand::Imm(i as i64),
        ];
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected alt-spelling {}: {}", asm, e));
        let sut = sut_word(&ops)
            .unwrap_or_else(|e| panic!("SUT rejected alt-spelling {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for alt-spelling {}", asm);
    }

    #[test]
    fn encode_neon_ext_neg_arity(
        n in 0usize..=3,
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        (t, i) in t_and_idx(),
    ) {
        let all = ext_ops(rd, rn, rm, t, i as i64);
        let arity_ops: Vec<Operand> = all.iter().take(n).cloned().collect();
        let arity_asm = match n {
            0 => "ext".to_string(),
            1 => format!("ext v{}.{}", rd, t),
            2 => format!("ext v{}.{t}, v{}.{t}", rd, rn),
            _ => format!("ext v{}.{t}, v{}.{t}, v{}.{t}", rd, rn, rm),
        };
        prop_assert!(
            llvm_mc_word(&arity_asm).is_err(),
            "llvm-mc unexpectedly accepted arity-{} {}",
            n,
            arity_asm
        );
        prop_assert!(
            encode_neon_ext(&arity_ops).is_err(),
            "arity {} must Err (llvm-mc rejects {})",
            n,
            arity_asm
        );
    }

    #[test]
    fn encode_neon_ext_neg_extra_operand(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        extra in reg_num(),
        (t, i) in t_and_idx(),
    ) {
        let asm = format!("{}, v{}.{}", ext_asm(rd, rn, rm, t, i as i64), extra, t);
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted 5-operand {}",
            asm
        );
        let mut ops = ext_ops(rd, rn, rm, t, i as i64);
        ops.push(arr(extra, t));
        prop_assert!(
            encode_neon_ext(&ops).is_err(),
            "5 operands must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_ext_neg_invalid_t(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        t in prop::sample::select(vec!["8h", "4h", "4s", "2s", "2d", "1d", "b", "h", "s", "d"]),
        i in 0u32..=7,
    ) {
        let asm = ext_asm(rd, rn, rm, t, i as i64);
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted invalid T {}",
            asm
        );
        prop_assert!(
            encode_neon_ext(&ext_ops(rd, rn, rm, t, i as i64)).is_err(),
            "invalid T {} must Err (llvm-mc/gas reject {})",
            t,
            asm
        );
    }

    #[test]
    fn encode_neon_ext_neg_index_oor(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        t in t_size(),
        over in 1u32..=16,
        neg in 1i64..=32,
    ) {
        let i = imax(t) + over;
        let asm = ext_asm(rd, rn, rm, t, i as i64);
        // gas rejects OOR (README gas contract). llvm-mc wraps; do not require llvm-mc Err.
        prop_assert!(
            encode_neon_ext(&ext_ops(rd, rn, rm, t, i as i64)).is_err(),
            "index {} for .{} must Err (gas range [0, {}]; asm {})",
            i,
            t,
            imax(t),
            asm
        );
        let nasm = ext_asm(rd, rn, rm, t, -neg);
        prop_assert!(
            encode_neon_ext(&ext_ops(rd, rn, rm, t, -neg)).is_err(),
            "negative index {} must Err (gas rejects {}; asm {})",
            -neg,
            t,
            nasm
        );
    }

    #[test]
    fn encode_neon_ext_neg_mismatched_t(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        t_d in t_size(),
        t_n in t_size(),
        t_m in t_size(),
        i in 0u32..=7,
    ) {
        prop_assume!(t_d != t_n || t_d != t_m);
        let asm = format!(
            "ext v{}.{t_d}, v{}.{t_n}, v{}.{t_m}, #{i}",
            rd, rn, rm
        );
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted mismatched T {}",
            asm
        );
        let ops = vec![
            arr(rd, t_d),
            arr(rn, t_n),
            arr(rm, t_m),
            Operand::Imm(i as i64),
        ];
        prop_assert!(
            encode_neon_ext(&ops).is_err(),
            "mismatched T must Err (llvm-mc/gas reject {})",
            asm
        );
    }

    #[test]
    fn encode_neon_ext_neg_gpr_or_bare(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        (t, i) in t_and_idx(),
        kind in 0u8..=4,
        fp_prefix in prop::sample::select(vec!["x", "w", "d", "s", "q", "h", "b"]),
    ) {
        let (ops, asm) = match kind {
            0 => (
                vec![
                    Operand::Reg(format!("{}{}", fp_prefix, rd)),
                    arr(rn, t),
                    arr(rm, t),
                    Operand::Imm(i as i64),
                ],
                format!(
                    "ext {}{}, v{}.{t}, v{}.{t}, #{i}",
                    fp_prefix, rd, rn, rm
                ),
            ),
            1 => (
                vec![
                    arr(rd, t),
                    Operand::Reg(format!("v{}", rn)),
                    arr(rm, t),
                    Operand::Imm(i as i64),
                ],
                format!("ext v{}.{t}, v{}, v{}.{t}, #{i}", rd, rn, rm),
            ),
            2 => (
                vec![
                    arr(rd, t),
                    arr(rn, t),
                    Operand::Reg(format!("x{}", rm)),
                    Operand::Imm(i as i64),
                ],
                format!("ext v{}.{t}, v{}.{t}, x{}, #{i}", rd, rn, rm),
            ),
            3 => (
                vec![
                    Operand::Reg(format!("v{}", rd)),
                    arr(rn, t),
                    arr(rm, t),
                    Operand::Imm(i as i64),
                ],
                format!("ext v{}, v{}.{t}, v{}.{t}, #{i}", rd, rn, rm),
            ),
            _ => (
                ext_ops(rd, rn, rm, t, i as i64),
                ext_asm(rd, rn, rm, t, i as i64),
            ),
        };
        if kind == 4 {
            // kind 4: non-Imm fourth operand
            let mut ops = ext_ops(rd, rn, rm, t, i as i64);
            ops[3] = arr(0, t);
            let asm = format!(
                "ext v{}.{t}, v{}.{t}, v{}.{t}, v0.{t}",
                rd, rn, rm
            );
            prop_assert!(
                llvm_mc_word(&asm).is_err(),
                "llvm-mc unexpectedly accepted non-imm index {}",
                asm
            );
            prop_assert!(
                encode_neon_ext(&ops).is_err(),
                "non-Imm index must Err (llvm-mc rejects {})",
                asm
            );
            return Ok(());
        }
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        prop_assert!(
            encode_neon_ext(&ops).is_err(),
            "GPR/bare/non-arrangement kind={} must Err (llvm-mc rejects {})",
            kind,
            asm
        );
    }
}

/// Deterministic regression: extra operand ignored (from neg_extra_operand).
#[test]
fn test_encode_neon_ext_regression_extra_operand() {
    let mut ops = ext_ops(0, 1, 2, "16b", 3);
    ops.push(arr(3, "16b"));
    assert!(
        encode_neon_ext(&ops).is_err(),
        "ext v0.16b, v1.16b, v2.16b, #3, v3.16b must Err (gas/llvm-mc reject a fifth operand)"
    );
}

/// Deterministic regression: invalid arrangement encoded as Q=0 (from neg_invalid_t).
#[test]
fn test_encode_neon_ext_regression_invalid_t() {
    assert!(
        encode_neon_ext(&ext_ops(0, 1, 2, "8h", 1)).is_err(),
        "ext v0.8h, v1.8h, v2.8h, #1 must Err (gas/llvm-mc accept only .8b/.16b)"
    );
    assert!(
        encode_neon_ext(&ext_ops(0, 1, 2, "4s", 1)).is_err(),
        "ext v0.4s, v1.4s, v2.4s, #1 must Err (gas/llvm-mc accept only .8b/.16b)"
    );
    assert!(
        encode_neon_ext(&ext_ops(0, 1, 2, "2d", 1)).is_err(),
        "ext v0.2d, v1.2d, v2.2d, #1 must Err (gas/llvm-mc accept only .8b/.16b)"
    );
}

/// Deterministic regression: out-of-range index masked (from neg_index_oor).
#[test]
fn test_encode_neon_ext_regression_index_oor() {
    assert!(
        encode_neon_ext(&ext_ops(0, 1, 2, "8b", 8)).is_err(),
        "ext v0.8b, v1.8b, v2.8b, #8 must Err (gas range for .8b is [0, 7])"
    );
    assert!(
        encode_neon_ext(&ext_ops(0, 1, 2, "16b", 16)).is_err(),
        "ext v0.16b, v1.16b, v2.16b, #16 must Err (gas range for .16b is [0, 15])"
    );
    assert!(
        encode_neon_ext(&ext_ops(0, 1, 2, "8b", -1)).is_err(),
        "ext v0.8b, v1.8b, v2.8b, #-1 must Err (gas rejects negative index)"
    );
}

/// Deterministic regression: mismatched arrangements ignored (from neg_mismatched_t).
#[test]
fn test_encode_neon_ext_regression_mismatched_t() {
    let ops = vec![
        arr(0, "16b"),
        arr(1, "8b"),
        arr(2, "16b"),
        Operand::Imm(3),
    ];
    assert!(
        encode_neon_ext(&ops).is_err(),
        "ext v0.16b, v1.8b, v2.16b, #3 must Err (gas/llvm-mc require matching T)"
    );
}

/// Deterministic regression: GPR dest encoded via Operand::Reg (from neg_gpr_or_bare).
#[test]
fn test_encode_neon_ext_regression_gpr_dest() {
    let ops = vec![
        Operand::Reg("x0".into()),
        arr(1, "16b"),
        arr(2, "16b"),
        Operand::Imm(3),
    ];
    assert!(
        encode_neon_ext(&ops).is_err(),
        "ext x0, v1.16b, v2.16b, #3 must Err (gas/llvm-mc reject GPR dest)"
    );
}

/// Deterministic regression: bare V dest without arrangement (from neg_gpr_or_bare).
#[test]
fn test_encode_neon_ext_regression_bare_v_dest() {
    let ops = vec![
        Operand::Reg("v0".into()),
        arr(1, "8b"),
        arr(2, "8b"),
        Operand::Imm(3),
    ];
    assert!(
        encode_neon_ext(&ops).is_err(),
        "ext v0, v1.8b, v2.8b, #3 must Err (gas/llvm-mc require Vd.T)"
    );
}
