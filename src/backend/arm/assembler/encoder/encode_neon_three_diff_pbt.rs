// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md:230 NEON widen/long lists smull/umull/smlal/umlal/smlsl/umlsl/saddw/uaddw/ssubw/usubw (+ 2 variants);
//   encoder/mod.rs:1-7 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:314/325 smull/umull vector => encode_neon_three_diff;
//   encoder/mod.rs:681-688 uabal/sabal/uabdl/sabdl (+2);
//   encoder/mod.rs:815-905 usubl/ssubl/usubw/ssubw/uaddl/saddl/uaddw/saddw/umlal/smlal/umlsl/smlsl/umull2/smull2;
//   neon.rs:81 Encode NEON three-different instructions: USUBL, SSUBL, UADDL, SADDL, etc.;
//   neon.rs:83 These instructions have wider destination than source operands;
//   neon.rs:84 Format: 0 Q U 01110 size 1 Rm opcode 00 Rn Rd;
//   neon.rs:88 is_high: true for the "2" variant (upper half, Q=1);
//   ARM ARM Advanced SIMD three-different LONG Tb in {8B,16B,4H,8H,2S,4S}, Ta = widen(Tb);
//   WIDE Vd.Ta, Vn.Ta, Vm.Tb; size:Q=11:x reserved.
// Stronger considered:
//   - State machine: rejected — encode_neon_three_diff is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree three-diff decoder
//   - Differential vs encode_neon_three_diff_narrow / encode_neon_pmull: rejected — same-job gate
//     (narrowing ADDHN vs widening LONG/WIDE; polynomial PMULL vs integer)
// Weaker available: algebraic.metamorphic (U / opcode / Q), algebraic.invariant (word layout),
//   negative_error (arity / extra / dest Ta mismatch / GPR dest)
// Differential: candidate=encode_neon_three_diff, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping LONG=[RegArrangement(Vd,Ta), RegArrangement(Vn,Tb), RegArrangement(Vm,Tb)] + (U, opcode, is_high)
//     <-> `mnemonic{2} Vd.Ta, Vn.Tb, Vm.Tb`;
//   WIDE=[RegArrangement(Vd,Ta), RegArrangement(Vn,Ta), RegArrangement(Vm,Tb)] + (U, opcode, is_high)
//     <-> `mnemonic{2} Vd.Ta, Vn.Ta, Vm.Tb`

use super::encode_neon_three_diff;
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

/// Integer three-different LONG mnemonics (wider dest, matching narrow sources).
fn long_table() -> impl Strategy<Value = (u32, u32, &'static str)> {
    prop::sample::select(vec![
        (0u32, 0b0000u32, "saddl"),
        (1u32, 0b0000u32, "uaddl"),
        (0u32, 0b0010u32, "ssubl"),
        (1u32, 0b0010u32, "usubl"),
        (0u32, 0b0101u32, "sabal"),
        (1u32, 0b0101u32, "uabal"),
        (0u32, 0b0111u32, "sabdl"),
        (1u32, 0b0111u32, "uabdl"),
        (0u32, 0b1000u32, "smlal"),
        (1u32, 0b1000u32, "umlal"),
        (0u32, 0b1010u32, "smlsl"),
        (1u32, 0b1010u32, "umlsl"),
        (0u32, 0b1100u32, "smull"),
        (1u32, 0b1100u32, "umull"),
    ])
}

fn wide_table() -> impl Strategy<Value = (u32, u32, &'static str)> {
    prop::sample::select(vec![
        (0u32, 0b0001u32, "saddw"),
        (1u32, 0b0001u32, "uaddw"),
        (0u32, 0b0011u32, "ssubw"),
        (1u32, 0b0011u32, "usubw"),
    ])
}

fn widen(tb: &str) -> &'static str {
    match tb {
        "8b" | "16b" => "8h",
        "4h" | "8h" => "4s",
        "2s" | "4s" => "2d",
        _ => "8h",
    }
}

fn is_high_tb(tb: &str) -> bool {
    matches!(tb, "16b" | "8h" | "4s")
}

fn size_of(tb: &str) -> u32 {
    match tb {
        "8b" | "16b" => 0b00,
        "4h" | "8h" => 0b01,
        "2s" | "4s" => 0b10,
        _ => 0,
    }
}

fn mnem2(base: &str, high: bool) -> String {
    if high {
        format!("{base}2")
    } else {
        base.to_string()
    }
}

fn ops_long(rd: u32, rn: u32, rm: u32, tb: &str) -> Vec<Operand> {
    let ta = widen(tb);
    vec![arr(rd, ta), arr(rn, tb), arr(rm, tb)]
}

fn asm_long(rd: u32, rn: u32, rm: u32, tb: &str, mnemonic: &str) -> String {
    let ta = widen(tb);
    let m = mnem2(mnemonic, is_high_tb(tb));
    format!("{m} v{rd}.{ta}, v{rn}.{tb}, v{rm}.{tb}")
}

fn ops_wide(rd: u32, rn: u32, rm: u32, ta: &str, tb: &str) -> Vec<Operand> {
    vec![arr(rd, ta), arr(rn, ta), arr(rm, tb)]
}

fn asm_wide(rd: u32, rn: u32, rm: u32, ta: &str, tb: &str, mnemonic: &str, high: bool) -> String {
    let m = mnem2(mnemonic, high);
    format!("{m} v{rd}.{ta}, v{rn}.{ta}, v{rm}.{tb}")
}

fn sut_word(ops: &[Operand], u_bit: u32, opcode: u32, is_high: bool) -> Result<u32, String> {
    match encode_neon_three_diff(ops, u_bit, opcode, is_high)? {
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

fn long_tb() -> impl Strategy<Value = &'static str> {
    prop::sample::select(vec!["8b", "16b", "4h", "8h", "2s", "4s"])
}

fn any_t() -> impl Strategy<Value = &'static str> {
    prop::sample::select(vec![
        "8b", "16b", "4h", "8h", "2s", "4s", "1d", "2d", "1q",
    ])
}

fn wide_pair() -> impl Strategy<Value = (&'static str, &'static str, bool)> {
    prop::sample::select(vec![
        ("8h", "8b", false),
        ("8h", "16b", true),
        ("4s", "4h", false),
        ("4s", "8h", true),
        ("2d", "2s", false),
        ("2d", "4s", true),
    ])
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_neon_three_diff_kat_llvm_mc() {
    let kat: &[(&str, u32)] = &[
        ("saddl v0.8h, v1.8b, v2.8b", 0x0e220020),
        ("saddl2 v0.8h, v1.16b, v2.16b", 0x4e220020),
        ("saddl v0.4s, v1.4h, v2.4h", 0x0e620020),
        ("saddl2 v0.4s, v1.8h, v2.8h", 0x4e620020),
        ("saddl v0.2d, v1.2s, v2.2s", 0x0ea20020),
        ("saddl2 v0.2d, v1.4s, v2.4s", 0x4ea20020),
        ("uaddl v0.8h, v1.8b, v2.8b", 0x2e220020),
        ("ssubl v0.8h, v1.8b, v2.8b", 0x0e222020),
        ("usubl v0.8h, v1.8b, v2.8b", 0x2e222020),
        ("smull v0.8h, v1.8b, v2.8b", 0x0e22c020),
        ("umull v0.8h, v1.8b, v2.8b", 0x2e22c020),
        ("saddw v0.8h, v1.8h, v2.8b", 0x0e221020),
        ("saddw2 v0.8h, v1.8h, v2.16b", 0x4e221020),
        ("saddw v0.2d, v1.2d, v2.2s", 0x0ea21020),
    ];
    for &(asm, want) in kat {
        let mc = llvm_mc_word(asm).unwrap_or_else(|e| panic!("llvm-mc KAT {asm}: {e}"));
        assert_eq!(mc, want, "llvm-mc KAT mapping broken for {asm}");
    }
    let sut_long: &[(u32, u32, u32, &str, u32, u32, bool, u32)] = &[
        (0, 1, 2, "8b", 0, 0b0000, false, 0x0e220020),
        (0, 1, 2, "16b", 0, 0b0000, true, 0x4e220020),
        (0, 1, 2, "4h", 0, 0b0000, false, 0x0e620020),
        (0, 1, 2, "8h", 0, 0b0000, true, 0x4e620020),
        (0, 1, 2, "2s", 0, 0b0000, false, 0x0ea20020),
        (0, 1, 2, "4s", 0, 0b0000, true, 0x4ea20020),
        (0, 1, 2, "8b", 1, 0b0000, false, 0x2e220020),
        (0, 1, 2, "8b", 0, 0b0010, false, 0x0e222020),
        (0, 1, 2, "8b", 1, 0b0010, false, 0x2e222020),
        (0, 1, 2, "8b", 0, 0b1100, false, 0x0e22c020),
        (0, 1, 2, "8b", 1, 0b1100, false, 0x2e22c020),
    ];
    for &(rd, rn, rm, tb, u, opcode, high, want) in sut_long {
        let asm = format!("LONG rd={rd} rn={rn} rm={rm} tb={tb} u={u} opc={opcode} high={high}");
        let sut = sut_word(&ops_long(rd, rn, rm, tb), u, opcode, high)
            .unwrap_or_else(|e| panic!("SUT KAT {asm}: {e}"));
        assert_eq!(sut, want, "SUT KAT mismatch for {asm}");
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_three_diff_diff_llvm_mc_long(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        tb in long_tb(),
        insn in long_table(),
    ) {
        let (u, opcode, mnemonic) = insn;
        let high = is_high_tb(tb);
        let asm = asm_long(rd, rn, rm, tb, mnemonic);
        let ops = ops_long(rd, rn, rm, tb);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&ops, u, opcode, high)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for {}", asm);
    }

    #[test]
    fn encode_neon_three_diff_diff_llvm_mc_wide(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        pair in wide_pair(),
        insn in wide_table(),
    ) {
        let (ta, tb, high) = pair;
        let (u, opcode, mnemonic) = insn;
        let asm = asm_wide(rd, rn, rm, ta, tb, mnemonic, high);
        let ops = ops_wide(rd, rn, rm, ta, tb);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&ops, u, opcode, high)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for {}", asm);
    }

    #[test]
    fn encode_neon_three_diff_invariant_arm_fields(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        tb in long_tb(),
        u in 0u32..=1,
        opcode in 0u32..=15,
    ) {
        let high = is_high_tb(tb);
        let w = sut_word(&ops_long(rd, rn, rm, tb), u, opcode, high)
            .unwrap_or_else(|e| panic!("SUT rejected: {}", e));
        let q = if high { 1u32 } else { 0 };
        let size = size_of(tb);
        prop_assert_eq!((w >> 31) & 1, 0u32, "bit31=0");
        prop_assert_eq!((w >> 30) & 1, q, "Q");
        prop_assert_eq!((w >> 29) & 1, u, "U");
        prop_assert_eq!((w >> 24) & 0x1F, 0b01110u32, "bits[28:24]=01110");
        prop_assert_eq!((w >> 22) & 3, size, "size");
        prop_assert_eq!((w >> 21) & 1, 1u32, "bit21=1");
        prop_assert_eq!((w >> 16) & 0x1F, rm, "Rm");
        prop_assert_eq!((w >> 12) & 0xF, opcode, "opcode");
        prop_assert_eq!((w >> 10) & 3, 0u32, "bits[11:10]=00");
        prop_assert_eq!((w >> 5) & 0x1F, rn, "Rn");
        prop_assert_eq!(w & 0x1F, rd, "Rd");
    }

    #[test]
    fn encode_neon_three_diff_metamorphic_u_opcode_q(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        opcode1 in 0u32..=15,
        opcode2 in 0u32..=15,
    ) {
        let ops8 = ops_long(rd, rn, rm, "8b");
        let w_u0 = sut_word(&ops8, 0, opcode1, false)
            .unwrap_or_else(|e| panic!("SUT rejected u=0: {}", e));
        let w_u1 = sut_word(&ops8, 1, opcode1, false)
            .unwrap_or_else(|e| panic!("SUT rejected u=1: {}", e));
        let w_op2 = sut_word(&ops8, 0, opcode2, false)
            .unwrap_or_else(|e| panic!("SUT rejected opcode2: {}", e));
        let w_q = sut_word(&ops8, 0, opcode1, true)
            .unwrap_or_else(|e| panic!("SUT rejected is_high: {}", e));
        prop_assert_eq!(
            (w_u0 ^ w_u1) & !(1u32 << 29),
            0u32,
            "changing only U must differ only in bit 29"
        );
        prop_assert_eq!((w_u1 >> 29) & 1, 1u32, "U=1");
        prop_assert_eq!((w_u0 >> 29) & 1, 0u32, "U=0");
        prop_assert_eq!(
            (w_u0 ^ w_op2) & !(0xFu32 << 12),
            0u32,
            "changing only opcode must differ only in bits[15:12]"
        );
        prop_assert_eq!((w_op2 >> 12) & 0xF, opcode2, "opcode field");
        prop_assert_eq!(
            (w_u0 ^ w_q) & !(1u32 << 30),
            0u32,
            "is_high must differ only in Q bit 30"
        );
        prop_assert_eq!((w_q >> 30) & 1, 1u32, "Q=1 for is_high");
        prop_assert_eq!((w_u0 >> 30) & 1, 0u32, "Q=0 for 8b / is_high=false");
    }

    #[test]
    fn encode_neon_three_diff_neg_arity(
        n in 0usize..=2,
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        insn in long_table(),
    ) {
        let (u, opcode, mnemonic) = insn;
        let all = ops_long(rd, rn, rm, "8b");
        let arity_ops: Vec<Operand> = all.iter().take(n).cloned().collect();
        let arity_asm = match n {
            0 => mnemonic.to_string(),
            1 => format!("{mnemonic} v{rd}.8h"),
            _ => format!("{mnemonic} v{rd}.8h, v{rn}.8b"),
        };
        prop_assert!(
            llvm_mc_word(&arity_asm).is_err(),
            "llvm-mc unexpectedly accepted arity-{} {}",
            n,
            arity_asm
        );
        prop_assert!(
            encode_neon_three_diff(&arity_ops, u, opcode, false).is_err(),
            "arity {} must Err (llvm-mc rejects {})",
            n,
            arity_asm
        );
    }

    #[test]
    fn encode_neon_three_diff_neg_extra_operand(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        extra in reg_num(),
        tb in long_tb(),
        insn in long_table(),
    ) {
        let (u, opcode, mnemonic) = insn;
        let high = is_high_tb(tb);
        let asm = format!("{}, v{}.{tb}", asm_long(rd, rn, rm, tb, mnemonic), extra);
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted 4-operand {}",
            asm
        );
        let mut ops = ops_long(rd, rn, rm, tb);
        ops.push(arr(extra, tb));
        prop_assert!(
            encode_neon_three_diff(&ops, u, opcode, high).is_err(),
            "4 operands must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_three_diff_neg_dest_ta_mismatch(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        tb in long_tb(),
        td in any_t(),
        insn in long_table(),
    ) {
        prop_assume!(td != widen(tb));
        let (u, opcode, mnemonic) = insn;
        let high = is_high_tb(tb);
        let m = mnem2(mnemonic, high);
        let asm = format!("{m} v{rd}.{td}, v{rn}.{tb}, v{rm}.{tb}");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted dest mismatch {}",
            asm
        );
        let ops = vec![arr(rd, td), arr(rn, tb), arr(rm, tb)];
        prop_assert!(
            encode_neon_three_diff(&ops, u, opcode, high).is_err(),
            "dest Ta must be widen(Tb); llvm-mc rejects {}",
            asm
        );
    }

    #[test]
    fn encode_neon_three_diff_neg_unsupported_src(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        tb in prop::sample::select(vec!["1d", "2d", "1q", ""]),
        insn in long_table(),
    ) {
        let (u, opcode, mnemonic) = insn;
        let asm = format!("{mnemonic} v{rd}.8h, v{rn}.{tb}, v{rm}.{tb}");
        if !tb.is_empty() {
            prop_assert!(
                llvm_mc_word(&asm).is_err(),
                "llvm-mc unexpectedly accepted unsupported Tb {}",
                asm
            );
        }
        let ops = vec![arr(rd, "8h"), arr(rn, tb), arr(rm, tb)];
        prop_assert!(
            encode_neon_three_diff(&ops, u, opcode, false).is_err(),
            "unsupported source arrangement {} must Err",
            tb
        );
    }

    #[test]
    fn encode_neon_three_diff_neg_rm_tb_mismatch(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        tb in long_tb(),
        tm in any_t(),
        insn in long_table(),
    ) {
        prop_assume!(tm != tb);
        let (u, opcode, mnemonic) = insn;
        let high = is_high_tb(tb);
        let m = mnem2(mnemonic, high);
        let ta = widen(tb);
        let asm = format!("{m} v{rd}.{ta}, v{rn}.{tb}, v{rm}.{tm}");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted Rm mismatch {}",
            asm
        );
        let ops = vec![arr(rd, ta), arr(rn, tb), arr(rm, tm)];
        prop_assert!(
            encode_neon_three_diff(&ops, u, opcode, high).is_err(),
            "Rm Tb must match Vn Tb; llvm-mc rejects {}",
            asm
        );
    }

    #[test]
    fn encode_neon_three_diff_neg_gpr_or_bare(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        insn in long_table(),
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
                    arr(rd, "8h"),
                    Operand::Reg(format!("v{}", rn)),
                    arr(rm, "8b"),
                ],
                format!("{mnemonic} v{rd}.8h, v{rn}, v{rm}.8b"),
            ),
            2 => (
                vec![
                    arr(rd, "8h"),
                    arr(rn, "8b"),
                    Operand::Reg(format!("x{}", rm)),
                ],
                format!("{mnemonic} v{rd}.8h, v{rn}.8b, x{rm}"),
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
                        arrangement: "8h".to_string(),
                    },
                    arr(rn, "8b"),
                    arr(rm, "8b"),
                ],
                format!("{mnemonic} x{rd}.8h, v{rn}.8b, v{rm}.8b"),
            ),
        };
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        prop_assert!(
            encode_neon_three_diff(&ops, u, opcode, false).is_err(),
            "GPR/bare/non-arrangement kind={} must Err (llvm-mc rejects {})",
            kind,
            asm
        );
    }
}

/// Deterministic regression: extra operand ignored (from neg_extra_operand).
#[test]
fn test_encode_neon_three_diff_regression_extra_operand() {
    let mut ops = ops_long(0, 0, 0, "8b");
    ops.push(arr(0, "8b"));
    assert!(
        encode_neon_three_diff(&ops, 0, 0b0000, false).is_err(),
        "saddl v0.8h, v0.8b, v0.8b, v0.8b must Err (gas/llvm-mc reject a fourth operand)"
    );
}

/// Deterministic regression: dest Ta discarded (from neg_dest_ta_mismatch).
#[test]
fn test_encode_neon_three_diff_regression_dest_ta_mismatch() {
    let ops = vec![arr(0, "8b"), arr(0, "8b"), arr(0, "8b")];
    assert!(
        encode_neon_three_diff(&ops, 0, 0b0000, false).is_err(),
        "saddl v0.8b, v0.8b, v0.8b must Err (ARM/llvm-mc require Vd.8h for Tb=8b)"
    );
}

/// Deterministic regression: GPR Rm encoded (from neg_gpr_or_bare kind=2).
#[test]
fn test_encode_neon_three_diff_regression_gpr_rm() {
    let ops = vec![arr(0, "8h"), arr(0, "8b"), Operand::Reg("x0".into())];
    assert!(
        encode_neon_three_diff(&ops, 0, 0b0000, false).is_err(),
        "saddl v0.8h, v0.8b, x0 must Err (gas/llvm-mc require Vm.Tb)"
    );
}

/// Deterministic regression: Rm arrangement ignored (from neg_rm_tb_mismatch).
#[test]
fn test_encode_neon_three_diff_regression_rm_tb_mismatch() {
    let ops = vec![arr(0, "8h"), arr(0, "8b"), arr(0, "16b")];
    assert!(
        encode_neon_three_diff(&ops, 0, 0b0000, false).is_err(),
        "saddl v0.8h, v0.8b, v0.16b must Err (ARM/llvm-mc require matching Tb)"
    );
}

/// Deterministic regression: WIDE size/Q taken from Vn not Vm (from diff_llvm_mc_wide).
#[test]
fn test_encode_neon_three_diff_regression_wide_vn_size() {
    let ops = ops_wide(0, 0, 0, "8h", "8b");
    let sut = sut_word(&ops, 0, 0b0001, false)
        .expect("SUT must encode valid saddw v0.8h, v0.8h, v0.8b");
    let mc = llvm_mc_word("saddw v0.8h, v0.8h, v0.8b")
        .expect("llvm-mc KAT saddw v0.8h, v0.8h, v0.8b");
    assert_eq!(
        sut, mc,
        "saddw v0.8h, v0.8h, v0.8b must match llvm-mc (size/Q from narrow Vm.8b, not Vn.8h)"
    );
}
