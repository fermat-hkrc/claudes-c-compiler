// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md:237 NEON scalar lists sqabs/sqneg (scalar);
//   encoder/mod.rs:1-7 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:674-682 "sqabs"/"sqneg" => encode_neon_scalar_two_misc when dest is Operand::Reg;
//   neon.rs:1818 NEON scalar two-reg misc: SQABS/SQNEG Hd,Hn / Sd,Sn / Dd,Dn;
//   neon.rs:1828 01 U 11110 size 10000 opcode 10 Rn Rd;
//   ARM ARM Advanced SIMD scalar two-register miscellaneous:
//     SQABS U=0 opcode=00111; SQNEG U=1 opcode=00111; dest/src same B/H/S/D class.
// Stronger considered:
//   - State machine: rejected — encode_neon_scalar_two_misc is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree scalar SQABS/SQNEG decoder
//   - Differential vs encode_neon_two_misc: rejected — same-job gate (vector Vd.T vs scalar)
// Weaker available: algebraic.metamorphic (Rd/Rn/U/opcode/size fields), algebraic.invariant (word layout),
//   negative_error (arity / extra / wrong register class / non-register)
// Differential: candidate=encode_neon_scalar_two_misc, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[Reg(<V><d>), Reg(<V><n>)] + (U=is_neg, opcode=00111) <-> `sqabs|sqneg <V><d>, <V><n>`

use super::encode_neon_scalar_two_misc;
use super::EncodeResult;
use crate::backend::arm::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;
const OPC_SQABS: u32 = 0b00111;

fn cfg() -> ProptestConfig {
    ProptestConfig::with_cases(CASES)
}

fn mnemonic(is_neg: bool) -> &'static str {
    if is_neg {
        "sqneg"
    } else {
        "sqabs"
    }
}

fn u_bit(is_neg: bool) -> u32 {
    if is_neg {
        1
    } else {
        0
    }
}

fn size_of(pfx: &str) -> u32 {
    match pfx.to_ascii_lowercase().as_str() {
        "b" => 0b00,
        "h" => 0b01,
        "s" => 0b10,
        "d" => 0b11,
        _ => 0,
    }
}

fn sreg(pfx: &str, n: u32) -> Operand {
    Operand::Reg(format!("{pfx}{n}"))
}

fn ops_ok(pfx: &str, rd: u32, rn: u32) -> Vec<Operand> {
    vec![sreg(pfx, rd), sreg(pfx, rn)]
}

fn asm_ok(pfx: &str, rd: u32, rn: u32, is_neg: bool) -> String {
    format!("{} {pfx}{rd}, {pfx}{rn}", mnemonic(is_neg))
}

fn sut_word(ops: &[Operand], is_neg: bool) -> Result<u32, String> {
    sut_word_fields(ops, u_bit(is_neg), OPC_SQABS)
}

fn sut_word_fields(ops: &[Operand], u: u32, opcode: u32) -> Result<u32, String> {
    match encode_neon_scalar_two_misc(ops, u, opcode)? {
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

fn simd_pfx() -> impl Strategy<Value = &'static str> {
    prop::sample::select(vec!["b", "h", "s", "d"])
}

fn bad_pfx() -> impl Strategy<Value = &'static str> {
    prop::sample::select(vec![
        "b", "h", "s", "d", "x", "w", "q", "v", "sp", "xzr", "wsp", "wzr", "lr",
    ])
}

fn named_operand(pfx: &str, n: u32) -> (Operand, String) {
    match pfx {
        "sp" | "wsp" | "xzr" | "wzr" | "lr" => (Operand::Reg(pfx.to_string()), pfx.to_string()),
        _ => {
            let name = format!("{pfx}{n}");
            (Operand::Reg(name.clone()), name)
        }
    }
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_neon_scalar_two_misc_kat_llvm_mc() {
    let kat: &[(&str, u32)] = &[
        ("sqabs d0, d1", 0x5ee07820),
        ("sqneg d0, d1", 0x7ee07820),
        ("sqabs s0, s1", 0x5ea07820),
        ("sqabs h0, h1", 0x5e607820),
        ("sqabs b0, b1", 0x5e207820),
        ("sqabs d31, d31", 0x5ee07bff),
        ("sqneg b0, b1", 0x7e207820),
        ("SQABS D0, D1", 0x5ee07820),
        ("sqabs d15, d16", 0x5ee07a0f),
        ("sqneg s31, s0", 0x7ea0781f),
    ];
    for &(asm, want) in kat {
        let mc = llvm_mc_word(asm).unwrap_or_else(|e| panic!("llvm-mc KAT {asm}: {e}"));
        assert_eq!(mc, want, "llvm-mc KAT mapping broken for {asm}");
    }
    let sut_kat: &[(&str, u32, u32, bool, u32)] = &[
        ("d", 0, 1, false, 0x5ee07820),
        ("d", 0, 1, true, 0x7ee07820),
        ("s", 0, 1, false, 0x5ea07820),
        ("h", 0, 1, false, 0x5e607820),
        ("b", 0, 1, false, 0x5e207820),
        ("d", 31, 31, false, 0x5ee07bff),
        ("b", 0, 1, true, 0x7e207820),
        ("d", 15, 16, false, 0x5ee07a0f),
        ("s", 31, 0, true, 0x7ea0781f),
    ];
    for &(pfx, rd, rn, is_neg, want) in sut_kat {
        let asm = asm_ok(pfx, rd, rn, is_neg);
        let sut = sut_word(&ops_ok(pfx, rd, rn), is_neg)
            .unwrap_or_else(|e| panic!("SUT KAT {asm}: {e}"));
        assert_eq!(sut, want, "SUT KAT mismatch for {asm}");
    }
}

proptest! {
    #![proptest_config(cfg())]

    // Oracle: differential — llvm-mc AArch64 assembler
    #[test]
    fn encode_neon_scalar_two_misc_diff_llvm_mc(
        rd in reg_num(),
        rn in reg_num(),
        pfx in simd_pfx(),
        is_neg in any::<bool>(),
    ) {
        let asm = asm_ok(pfx, rd, rn, is_neg);
        let ops = ops_ok(pfx, rd, rn);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&ops, is_neg)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for {}", asm);
    }

    // Oracle: algebraic.metamorphic — Rd/Rn/U/opcode/size field isolation
    #[test]
    fn encode_neon_scalar_two_misc_meta_rd_rn_u_opcode_size(
        rd1 in reg_num(),
        rd2 in reg_num(),
        rn1 in reg_num(),
        rn2 in reg_num(),
        u1 in 0u32..=1u32,
        u2 in 0u32..=1u32,
        opc1 in 0u32..=31u32,
        opc2 in 0u32..=31u32,
        pfx1 in simd_pfx(),
        pfx2 in simd_pfx(),
    ) {
        let w111 = sut_word_fields(&ops_ok(pfx1, rd1, rn1), u1, opc1)
            .unwrap_or_else(|e| panic!("SUT rejected base: {}", e));
        let w211 = sut_word_fields(&ops_ok(pfx1, rd2, rn1), u1, opc1)
            .unwrap_or_else(|e| panic!("SUT rejected rd2: {}", e));
        let w121 = sut_word_fields(&ops_ok(pfx1, rd1, rn2), u1, opc1)
            .unwrap_or_else(|e| panic!("SUT rejected rn2: {}", e));
        let w_u = sut_word_fields(&ops_ok(pfx1, rd1, rn1), u2, opc1)
            .unwrap_or_else(|e| panic!("SUT rejected u2: {}", e));
        let w_op = sut_word_fields(&ops_ok(pfx1, rd1, rn1), u1, opc2)
            .unwrap_or_else(|e| panic!("SUT rejected opc2: {}", e));
        let w_sz = sut_word_fields(&ops_ok(pfx2, rd1, rn1), u1, opc1)
            .unwrap_or_else(|e| panic!("SUT rejected pfx2: {}", e));
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
            (w111 ^ w_u) & !(1u32 << 29),
            0u32,
            "changing only U must differ only in bit 29"
        );
        prop_assert_eq!((w_u >> 29) & 1, u2, "U field");
        prop_assert_eq!(
            (w111 ^ w_op) & !(0x1Fu32 << 12),
            0u32,
            "changing only opcode must differ only in bits[16:12]"
        );
        prop_assert_eq!((w_op >> 12) & 0x1F, opc2, "opcode field");
        prop_assert_eq!(
            (w111 ^ w_sz) & !(0x3u32 << 22),
            0u32,
            "changing only dest prefix must differ only in size bits[23:22]"
        );
        prop_assert_eq!((w_sz >> 22) & 3, size_of(pfx2), "size field");
    }

    // Oracle: algebraic.invariant — ARM scalar two-misc field layout
    #[test]
    fn encode_neon_scalar_two_misc_invariant_arm_fields(
        rd in reg_num(),
        rn in reg_num(),
        u in 0u32..=1u32,
        opcode in 0u32..=31u32,
        pfx in simd_pfx(),
    ) {
        let w = sut_word_fields(&ops_ok(pfx, rd, rn), u, opcode)
            .unwrap_or_else(|e| panic!("SUT rejected: {}", e));
        prop_assert_eq!((w >> 30) & 3, 0b01u32, "bits[31:30]=01");
        prop_assert_eq!((w >> 29) & 1, u, "U");
        prop_assert_eq!((w >> 24) & 0x1F, 0b11110u32, "bits[28:24]=11110");
        prop_assert_eq!((w >> 22) & 3, size_of(pfx), "size");
        prop_assert_eq!((w >> 17) & 0x1F, 0b10000u32, "bits[21:17]=10000");
        prop_assert_eq!((w >> 12) & 0x1F, opcode, "opcode");
        prop_assert_eq!((w >> 10) & 3, 0b10u32, "bits[11:10]=10");
        prop_assert_eq!((w >> 5) & 0x1F, rn, "Rn");
        prop_assert_eq!(w & 0x1F, rd, "Rd");
    }

    // Oracle: negative_error — arity 0..=1
    #[test]
    fn encode_neon_scalar_two_misc_neg_arity(
        n in 0usize..=1,
        rd in reg_num(),
        rn in reg_num(),
        pfx in simd_pfx(),
        is_neg in any::<bool>(),
    ) {
        let all = ops_ok(pfx, rd, rn);
        let arity_ops: Vec<Operand> = all.iter().take(n).cloned().collect();
        let mn = mnemonic(is_neg);
        let arity_asm = if n == 0 {
            mn.to_string()
        } else {
            format!("{mn} {pfx}{rd}")
        };
        prop_assert!(
            llvm_mc_word(&arity_asm).is_err(),
            "llvm-mc unexpectedly accepted arity-{} {}",
            n,
            arity_asm
        );
        prop_assert!(
            encode_neon_scalar_two_misc(&arity_ops, u_bit(is_neg), OPC_SQABS).is_err(),
            "arity {} must Err (llvm-mc rejects {})",
            n,
            arity_asm
        );
    }

    // Oracle: negative_error — third operand
    #[test]
    fn encode_neon_scalar_two_misc_neg_extra_operand(
        rd in reg_num(),
        rn in reg_num(),
        extra in reg_num(),
        pfx in simd_pfx(),
        is_neg in any::<bool>(),
    ) {
        let asm = format!("{}, {pfx}{extra}", asm_ok(pfx, rd, rn, is_neg));
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted 3-operand {}",
            asm
        );
        let mut ops = ops_ok(pfx, rd, rn);
        ops.push(sreg(pfx, extra));
        prop_assert!(
            encode_neon_scalar_two_misc(&ops, u_bit(is_neg), OPC_SQABS).is_err(),
            "3 operands must Err (llvm-mc rejects {})",
            asm
        );
    }

    // Oracle: negative_error — dest/src class mismatch or non-B/H/S/D
    #[test]
    fn encode_neon_scalar_two_misc_neg_wrong_reg_class(
        rd in reg_num(),
        rn in reg_num(),
        dest_pfx in simd_pfx(),
        bad in bad_pfx(),
        is_neg in any::<bool>(),
        slot in 0usize..=1,
    ) {
        prop_assume!(bad != dest_pfx);
        let (bad_op, bad_name) = named_operand(bad, if slot == 0 { rd } else { rn });
        let mut ops = ops_ok(dest_pfx, rd, rn);
        ops[slot] = bad_op;
        let mn = mnemonic(is_neg);
        let asm = if slot == 0 {
            format!("{mn} {bad_name}, {dest_pfx}{rn}")
        } else {
            format!("{mn} {dest_pfx}{rd}, {bad_name}")
        };
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        prop_assert!(
            encode_neon_scalar_two_misc(&ops, u_bit(is_neg), OPC_SQABS).is_err(),
            "wrong class slot={} bad={} must Err (llvm-mc rejects {})",
            slot,
            bad,
            asm
        );
    }

    // Oracle: negative_error — Imm/Mem/Label in either slot
    #[test]
    fn encode_neon_scalar_two_misc_neg_nonreg(
        rd in reg_num(),
        rn in reg_num(),
        pfx in simd_pfx(),
        is_neg in any::<bool>(),
        kind in 0u8..=2,
        slot in 0usize..=1,
    ) {
        let mn = mnemonic(is_neg);
        let bad = match kind {
            0 => Operand::Imm(0),
            1 => Operand::Mem {
                base: format!("x{}", rd),
                offset: 0,
            },
            _ => Operand::Label("L0".into()),
        };
        let mut ops = ops_ok(pfx, rd, rn);
        ops[slot] = bad;
        let asm = match (kind, slot) {
            (0, 0) => format!("{mn} #0, {pfx}{rn}"),
            (1, 0) => format!("{mn} [x{rd}], {pfx}{rn}"),
            (2, 0) => format!("{mn} L0, {pfx}{rn}"),
            (0, 1) => format!("{mn} {pfx}{rd}, #0"),
            (1, 1) => format!("{mn} {pfx}{rd}, [x{rd}]"),
            _ => format!("{mn} {pfx}{rd}, L0"),
        };
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        prop_assert!(
            encode_neon_scalar_two_misc(&ops, u_bit(is_neg), OPC_SQABS).is_err(),
            "non-register operand slot={} kind={} must Err",
            slot,
            kind
        );
    }

    // Oracle: negative_error — dest prefix x/w/q/v (documented unsupported register type)
    #[test]
    fn encode_neon_scalar_two_misc_neg_unsupported_dest(
        rd in reg_num(),
        rn in reg_num(),
        dest_pfx in prop::sample::select(vec!["x", "w", "q", "v"]),
        src_pfx in simd_pfx(),
        is_neg in any::<bool>(),
    ) {
        let mn = mnemonic(is_neg);
        let ops = vec![sreg(dest_pfx, rd), sreg(src_pfx, rn)];
        let asm = format!("{mn} {dest_pfx}{rd}, {src_pfx}{rn}");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        prop_assert!(
            encode_neon_scalar_two_misc(&ops, u_bit(is_neg), OPC_SQABS).is_err(),
            "dest prefix {} must Err (llvm-mc rejects {})",
            dest_pfx,
            asm
        );
    }

    // Oracle: differential — uppercase prefix / mnemonic vs llvm-mc
    #[test]
    fn encode_neon_scalar_two_misc_diff_alt_spellings(
        rd in reg_num(),
        rn in reg_num(),
        pfx in simd_pfx(),
        is_neg in any::<bool>(),
    ) {
        let up = pfx.to_ascii_uppercase();
        let mn = if is_neg { "SQNEG" } else { "SQABS" };
        let asm = format!("{mn} {up}{rd}, {up}{rn}");
        let ops = vec![
            Operand::Reg(format!("{up}{rd}")),
            Operand::Reg(format!("{up}{rn}")),
        ];
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected alt-spelling {}: {}", asm, e));
        let sut = sut_word(&ops, is_neg)
            .unwrap_or_else(|e| panic!("SUT rejected alt-spelling {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for alt-spelling {}", asm);
    }
}

/// Deterministic regression: extra operand ignored (from neg_extra_operand).
#[test]
fn test_encode_neon_scalar_two_misc_regression_extra_operand() {
    let mut ops = ops_ok("b", 0, 0);
    ops.push(sreg("b", 0));
    assert!(
        encode_neon_scalar_two_misc(&ops, 0, OPC_SQABS).is_err(),
        "sqabs b0, b0, b0 must Err (gas/llvm-mc reject a third operand)"
    );
}

/// Deterministic regression: dest/src class mismatch encoded (from neg_wrong_reg_class).
#[test]
fn test_encode_neon_scalar_two_misc_regression_wrong_reg_class() {
    let ops = vec![sreg("h", 0), sreg("b", 0)];
    assert!(
        encode_neon_scalar_two_misc(&ops, 0, OPC_SQABS).is_err(),
        "sqabs h0, b0 must Err (ARM/gas/llvm-mc require matching B/H/S/D class)"
    );
}

/// Deterministic regression: dest SP treated as Sd (starts_with('s')).
#[test]
fn test_encode_neon_scalar_two_misc_regression_sp_dest() {
    let ops = vec![Operand::Reg("sp".into()), sreg("s", 0)];
    assert!(
        encode_neon_scalar_two_misc(&ops, 0, OPC_SQABS).is_err(),
        "sqabs sp, s0 must Err (llvm-mc rejects SP as a SIMD scalar dest)"
    );
}
