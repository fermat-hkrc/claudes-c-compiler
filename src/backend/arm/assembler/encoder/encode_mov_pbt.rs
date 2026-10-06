// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md:214 Data Processing lists mov;
//   README.md:287-291 wide-immediate MOVZ/MOVN/ORR-bitmask then movz+movk Words;
//   README.md:293-296 SP as ADD #0, register MOV as ORR XZR, NEON as INS/UMOV/ORR;
//   encoder/mod.rs:3 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:373 "mov" => encode_mov;
//   data_processing.rs:11/24/46/64/85/123/129 encoding comments;
//   ARM ARM C6 MOV (register) / MOV (to/from SP) / MOV (wide immediate);
//   ARM ARM Advanced SIMD MOV alias of ORR/INS/UMOV.
// Stronger considered:
//   - State machine: rejected — encode_mov is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree MOV decoder
//   - encode_movz / encode_movk / encode_movn as differential sibling: rejected —
//     same-job gate (explicit wide-move vs MOV alias; shared helpers)
//   - encode_neon_ins / encode_neon_umov as differential sibling: rejected —
//     same-job gate (INS/UMOV mnemonics; shared parse_reg_num)
// Weaker available: algebraic.metamorphic (sf bit), algebraic.invariant (ARM fields),
//   negative_error (arity / extra / mixed width / FP / SP-imm / lane OOB)
// Differential: candidate=encode_mov, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=operands <-> `mov` asm text.
//   Wide-immediate Words: README.md:287 extension; reconstruct from ARM move-wide fields.

use super::encode_mov;
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

fn gpr(is_64: bool, n: u32, as_sp: bool) -> String {
    if n == 31 {
        if as_sp {
            if is_64 {
                "sp".into()
            } else {
                "wsp".into()
            }
        } else if is_64 {
            "xzr".into()
        } else {
            "wzr".into()
        }
    } else {
        format!("{}{}", if is_64 { "x" } else { "w" }, n)
    }
}

fn vreg(n: u32) -> String {
    format!("v{n}")
}

fn sut_result(ops: &[Operand]) -> Result<EncodeResult, String> {
    encode_mov(ops)
}

fn sut_word(ops: &[Operand]) -> Result<u32, String> {
    match encode_mov(ops)? {
        EncodeResult::Word(w) => Ok(w),
        other => Err(format!("expected Word, got {other:?}")),
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

fn llvm_mc_disasm(word: u32) -> Result<String, String> {
    let b = word.to_le_bytes();
    let hex = format!("0x{:02x} 0x{:02x} 0x{:02x} 0x{:02x}", b[0], b[1], b[2], b[3]);
    let mut child = Command::new(LLVM_MC)
        .args(["-triple=aarch64", "-disassemble"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("spawn llvm-mc disasm: {e}"))?;
    {
        let mut stdin = child.stdin.take().ok_or("llvm-mc stdin")?;
        stdin
            .write_all(hex.as_bytes())
            .map_err(|e| format!("write llvm-mc: {e}"))?;
        stdin.write_all(b"\n").map_err(|e| format!("write llvm-mc: {e}"))?;
    }
    let out = child
        .wait_with_output()
        .map_err(|e| format!("wait llvm-mc: {e}"))?;
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    if !out.status.success() {
        return Err(format!("llvm-mc disasm failed: {stdout}"));
    }
    Ok(stdout)
}

fn parse_hash_imm_token(disasm: &str) -> Result<i64, String> {
    let hash = disasm.find('#').ok_or_else(|| format!("no #imm in {disasm}"))?;
    let rest = disasm[hash + 1..].trim();
    let tok = rest
        .split(|c: char| c == ',' || c.is_whitespace())
        .next()
        .unwrap_or("");
    let tok = tok.trim_end_matches(|c: char| !c.is_ascii_hexdigit() && c != 'x' && c != '-');
    if let Some(hex) = tok.strip_prefix("0x").or_else(|| tok.strip_prefix("0X")) {
        u64::from_str_radix(hex, 16)
            .map(|v| v as i64)
            .map_err(|e| e.to_string())
    } else {
        tok.parse::<i64>().map_err(|e| format!("parse {tok}: {e}"))
    }
}

fn parse_lsl(disasm: &str) -> u32 {
    let lower = disasm.to_ascii_lowercase();
    if let Some(i) = lower.find("lsl") {
        let rest = lower[i + 3..].trim().trim_start_matches('#').trim();
        let tok = rest
            .split(|c: char| !c.is_ascii_digit())
            .next()
            .unwrap_or("0");
        tok.parse().unwrap_or(0)
    } else {
        0
    }
}

/// Materialized immediate of a MOVZ/MOVN/MOV/ORR-XZR encoding, from llvm-mc disassembly.
fn materialized_imm(disasm: &str, is_64: bool) -> Result<i64, String> {
    let imm = parse_hash_imm_token(disasm)?;
    let shift = parse_lsl(disasm);
    let raw = (imm as u64) << shift;
    let mask = if is_64 { u64::MAX } else { 0xFFFF_FFFF };
    let lower = disasm.to_ascii_lowercase();
    let val = if lower.contains("movn") {
        (!raw) & mask
    } else {
        raw & mask
    };
    Ok(val as i64)
}

fn imm_equal(is_64: bool, a: i64, b: i64) -> bool {
    if is_64 {
        a as u64 == b as u64
    } else {
        (a as u32) == (b as u32)
    }
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

/// Reconstruct a 64-bit immediate from a MOVZ/MOVK(/MOVN) sequence per ARM ARM
/// Move wide (immediate): sf opc 100101 hw imm16 Rd. opc 10=MOVZ, 11=MOVK, 00=MOVN.
fn reconstruct_wide(words: &[u32], is_64: bool) -> Result<u64, String> {
    let width_mask = if is_64 { u64::MAX } else { 0xFFFF_FFFF };
    let mut val: u64 = 0;
    let mut seen_first = false;
    for &w in words {
        let opc = (w >> 29) & 0b11;
        let hw = (w >> 21) & 0b11;
        let imm16 = ((w >> 5) & 0xFFFF) as u64;
        let shift = hw * 16;
        match opc {
            0b10 => {
                // MOVZ: zero other halfwords, insert imm16
                val = imm16 << shift;
                seen_first = true;
            }
            0b11 => {
                // MOVK: insert imm16 into existing
                if !seen_first {
                    return Err("MOVK before MOVZ".into());
                }
                val = (val & !(0xFFFFu64 << shift)) | (imm16 << shift);
            }
            0b00 => {
                // MOVN: NOT of (imm16 << shift), other bits 1
                val = (!(imm16 << shift)) & width_mask;
                if !is_64 {
                    // 32-bit MOVN zero-extends into the 64-bit view used below
                    val &= 0xFFFF_FFFF;
                }
                seen_first = true;
            }
            _ => return Err(format!("not a move-wide opc={opc:#x} word={w:#x}")),
        }
    }
    if !seen_first {
        return Err("empty wide sequence".into());
    }
    Ok(val & width_mask)
}

fn extra_operand() -> impl Strategy<Value = Operand> {
    prop_oneof![
        (0u32..=31).prop_map(|n| Operand::Reg(format!("x{n}"))),
        Just(Operand::Imm(0)),
        Just(Operand::Imm(-1)),
        Just(Operand::Shift {
            kind: "lsl".into(),
            amount: 0,
        }),
        Just(Operand::Label("L0".into())),
        Just(Operand::Symbol("foo".into())),
        Just(Operand::Cond("eq".into())),
    ]
}

fn mov_imm_32() -> impl Strategy<Value = i64> {
    prop_oneof![
        Just(0i64),
        Just(1i64),
        Just(0xFFFFi64),
        Just(-1i64),
        Just(-65536i64),
        Just(-65537i64),
        Just(0x10000i64),
        Just(0x01010101i64),
        0i64..=0xFFFF,
        (-0x10000i64)..=-1,
        (1u16..=u16::MAX).prop_map(|k| (k as i64) << 16),
        (-0x8000_0000i64)..0x8000_0000i64,
    ]
}

fn mov_imm_64() -> impl Strategy<Value = i64> {
    prop_oneof![
        mov_imm_32(),
        Just(0x0101010101010101u64 as i64),
        (1u16..=u16::MAX).prop_map(|k| (k as i64) << 32),
        (1u16..=u16::MAX).prop_map(|k| (k as i64) << 48),
        prop::sample::select(vec![
            0x00FF00FF00FF00FFu64 as i64,
            0x0000FFFF0000FFFFu64 as i64,
            0xFFFFFFFF00000000u64 as i64,
            0x7FFFFFFFFFFFFFFFi64,
            i64::MIN,
            i64::MAX,
        ]),
    ]
}

fn mov_imm_for_width() -> impl Strategy<Value = (bool, i64)> {
    prop_oneof![
        mov_imm_32().prop_map(|i| (false, i)),
        mov_imm_64().prop_map(|i| (true, i)),
    ]
}

fn mov_imm_too_wide_for_w() -> impl Strategy<Value = i64> {
    prop_oneof![
        Just(0x0101010101010101u64 as i64),
        Just(0x100000000i64),
        Just(i64::MAX),
        Just(i64::MIN),
        (1u16..=u16::MAX).prop_map(|k| (k as i64) << 32),
    ]
}

fn neon_arr_8_16() -> impl Strategy<Value = &'static str> {
    prop::sample::select(vec!["8b", "16b"])
}

fn elem_size_all() -> impl Strategy<Value = &'static str> {
    prop::sample::select(vec!["b", "h", "s", "d"])
}

fn imax(ts: &str) -> u32 {
    match ts {
        "b" => 15,
        "h" => 7,
        "s" => 3,
        "d" => 1,
        _ => 0,
    }
}

fn ins_gpr_name(ts: &str, n: u32) -> String {
    if ts == "d" {
        gpr(true, n, false)
    } else {
        gpr(false, n, false)
    }
}

fn umov_dest(ts: &str, n: u32) -> String {
    // ARM MOV-to-general alias exists only for S (W) and D (X).
    if ts == "d" {
        gpr(true, n, false)
    } else {
        gpr(false, n, false)
    }
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_mov_kat_llvm_mc_x0_x1() {
    let want = 0xaa0103e0u32;
    let mc = llvm_mc_word("mov x0, x1").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [Operand::Reg("x0".into()), Operand::Reg("x1".into())];
    let sut = sut_word(&ops).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_mov_kat_llvm_mc_sp_x1() {
    let want = 0x9100003fu32;
    let mc = llvm_mc_word("mov sp, x1").expect("llvm-mc KAT sp");
    assert_eq!(mc, want, "llvm-mc KAT sp mapping broken");
    let ops = [Operand::Reg("sp".into()), Operand::Reg("x1".into())];
    let sut = sut_word(&ops).expect("SUT KAT sp");
    assert_eq!(sut, want);
}

#[test]
fn encode_mov_kat_llvm_mc_x0_imm0() {
    let want = 0xd2800000u32;
    let mc = llvm_mc_word("mov x0, #0").expect("llvm-mc KAT imm0");
    assert_eq!(mc, want, "llvm-mc KAT imm0 mapping broken");
    let ops = [Operand::Reg("x0".into()), Operand::Imm(0)];
    let sut = sut_word(&ops).expect("SUT KAT imm0");
    assert_eq!(sut, want);
}

#[test]
fn encode_mov_kat_llvm_mc_neon_16b() {
    let want = 0x4ea11c20u32;
    let mc = llvm_mc_word("mov v0.16b, v1.16b").expect("llvm-mc KAT 16b");
    assert_eq!(mc, want, "llvm-mc KAT 16b mapping broken");
    let ops = [
        Operand::RegArrangement {
            reg: "v0".into(),
            arrangement: "16b".into(),
        },
        Operand::RegArrangement {
            reg: "v1".into(),
            arrangement: "16b".into(),
        },
    ];
    let sut = sut_word(&ops).expect("SUT KAT 16b");
    assert_eq!(sut, want);
}

#[test]
fn encode_mov_kat_llvm_mc_ins_d() {
    let want = 0x4e181c20u32;
    let mc = llvm_mc_word("mov v0.d[1], x1").expect("llvm-mc KAT ins");
    assert_eq!(mc, want, "llvm-mc KAT ins mapping broken");
    let ops = [
        Operand::RegLane {
            reg: "v0".into(),
            elem_size: "d".into(),
            index: 1,
        },
        Operand::Reg("x1".into()),
    ];
    let sut = sut_word(&ops).expect("SUT KAT ins");
    assert_eq!(sut, want);
}

#[test]
fn encode_mov_kat_llvm_mc_umov_d() {
    let want = 0x4e183c00u32;
    let mc = llvm_mc_word("mov x0, v0.d[1]").expect("llvm-mc KAT umov");
    assert_eq!(mc, want, "llvm-mc KAT umov mapping broken");
    let ops = [
        Operand::Reg("x0".into()),
        Operand::RegLane {
            reg: "v0".into(),
            elem_size: "d".into(),
            index: 1,
        },
    ];
    let sut = sut_word(&ops).expect("SUT KAT umov");
    assert_eq!(sut, want);
}

#[test]
fn encode_mov_kat_llvm_mc_ins_elem() {
    let want = 0x6e1c0420u32;
    let mc = llvm_mc_word("mov v0.s[3], v1.s[0]").expect("llvm-mc KAT elem");
    assert_eq!(mc, want, "llvm-mc KAT elem mapping broken");
    let ops = [
        Operand::RegLane {
            reg: "v0".into(),
            elem_size: "s".into(),
            index: 3,
        },
        Operand::RegLane {
            reg: "v1".into(),
            elem_size: "s".into(),
            index: 0,
        },
    ];
    let sut = sut_word(&ops).expect("SUT KAT elem");
    assert_eq!(sut, want);
}

/// Regression: mov w0, wsp must be ADD (WSP form), not ORR WZR.
#[test]
fn test_encode_mov_regression_wsp() {
    let want = llvm_mc_word("mov w0, wsp").expect("llvm-mc wsp");
    let ops = [Operand::Reg("w0".into()), Operand::Reg("wsp".into())];
    let sut = sut_word(&ops).expect("SUT wsp");
    assert_eq!(sut, want, "mov w0, wsp must match ADD WSP encoding {want:#x}");
}

/// Regression: extra operand must be rejected (gas).
#[test]
fn test_encode_mov_regression_extra_operand() {
    let ops = [
        Operand::Reg("x0".into()),
        Operand::Reg("x0".into()),
        Operand::Reg("x0".into()),
    ];
    assert!(encode_mov(&ops).is_err(), "extra operand must Err");
}

/// Regression: mixed X/W must be rejected.
#[test]
fn test_encode_mov_regression_mixed_width() {
    let ops = [Operand::Reg("x0".into()), Operand::Reg("w0".into())];
    assert!(encode_mov(&ops).is_err(), "mixed width must Err");
}

/// Regression: FP scalar d0,d1 is not integer MOV.
#[test]
fn test_encode_mov_regression_fp_scalar() {
    let ops = [Operand::Reg("d0".into()), Operand::Reg("d1".into())];
    assert!(encode_mov(&ops).is_err(), "FP scalar mov must Err");
}

/// Regression: mov sp, #0 must be rejected (MOVZ Rd cannot be SP).
#[test]
fn test_encode_mov_regression_sp_imm() {
    let ops = [Operand::Reg("sp".into()), Operand::Imm(0)];
    assert!(encode_mov(&ops).is_err(), "mov sp, #0 must Err");
}

/// Regression: lane index 16 is out of range for .b.
#[test]
fn test_encode_mov_regression_lane_oob() {
    let ops = [
        Operand::RegLane {
            reg: "v0".into(),
            elem_size: "b".into(),
            index: 16,
        },
        Operand::Reg("w0".into()),
    ];
    assert!(encode_mov(&ops).is_err(), "v0.b[16] must Err");
}

/// Regression: mismatched 16b vs 8b arrangements must be rejected.
#[test]
fn test_encode_mov_regression_arr_mismatch() {
    let ops = [
        Operand::RegArrangement {
            reg: "v0".into(),
            arrangement: "16b".into(),
        },
        Operand::RegArrangement {
            reg: "v1".into(),
            arrangement: "8b".into(),
        },
    ];
    assert!(encode_mov(&ops).is_err(), "16b vs 8b must Err");
}

/// Regression: gas-invalid mov v0.4s, v1.4s must Err (or match llvm-mc Q=1 if accepted).
#[test]
fn test_encode_mov_regression_vec_4s() {
    let ops = [
        Operand::RegArrangement {
            reg: "v0".into(),
            arrangement: "4s".into(),
        },
        Operand::RegArrangement {
            reg: "v1".into(),
            arrangement: "4s".into(),
        },
    ];
    assert!(encode_mov(&ops).is_err(), "mov v0.4s, v1.4s must Err under gas contract");
}

/// Regression: 64-bit literal on W dest must be rejected (gas), not truncated.
#[test]
fn test_encode_mov_regression_w_large_imm() {
    let ops = [Operand::Reg("w0".into()), Operand::Imm(0x0101010101010101u64 as i64)];
    assert!(encode_mov(&ops).is_err(), "mov w0, #0x0101010101010101 must Err");
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_mov_diff_gpr_reg(
        rd in 0u32..=31,
        rm in 0u32..=31,
        is_64 in any::<bool>(),
        rd_sp in any::<bool>(),
        rm_sp in any::<bool>(),
    ) {
        let rd_n = gpr(is_64, rd, rd_sp && rd == 31);
        let rm_n = gpr(is_64, rm, rm_sp && rm == 31);
        let ops = [Operand::Reg(rd_n.clone()), Operand::Reg(rm_n.clone())];
        let asm = format!("mov {rd_n}, {rm_n}");
        let mc = llvm_mc_word(&asm).unwrap_or_else(|e| panic!("llvm-mc rejected valid {asm}: {e}"));
        let sut = sut_word(&ops).unwrap_or_else(|e| panic!("SUT rejected valid {asm}: {e}"));
        prop_assert_eq!(sut, mc, "mismatch for {}", asm);
    }

    #[test]
    fn encode_mov_diff_imm(
        rd in 0u32..=30,
        (is_64, imm) in mov_imm_for_width(),
    ) {
        let rd_n = gpr(is_64, rd, false);
        let ops = [Operand::Reg(rd_n.clone()), Operand::Imm(imm)];
        let asm = format!("mov {rd_n}, #{imm}");
        let mc = llvm_mc_word(&asm);
        match (mc, sut_result(&ops)) {
            (Ok(w), Ok(EncodeResult::Word(s))) => {
                if s == w {
                    // exact encoding match
                } else {
                    // Alias choice (MOVZ/MOVN/ORR-bitmask) is allowed by README.md:287;
                    // the materialized immediate must still equal the source #imm.
                    let dis = llvm_mc_disasm(s).map_err(|e| TestCaseError::fail(e))?;
                    let got = materialized_imm(&dis, is_64).map_err(|e| TestCaseError::fail(e))?;
                    prop_assert!(
                        imm_equal(is_64, got, imm),
                        "imm value mismatch for {} sut={:#x} ({}) llvm-mc={:#x} got={} want={}",
                        asm, s, dis.trim(), w, got, imm
                    );
                }
            }
            (Ok(w), Ok(EncodeResult::Words(ws))) => {
                let got = reconstruct_wide(&ws, is_64).map_err(|e| TestCaseError::fail(e))?;
                prop_assert!(
                    imm_equal(is_64, got as i64, imm),
                    "wide vs llvm-mc Word for {} mc={:#x} got={:#x} want={}",
                    asm, w, got, imm
                );
            }
            (Ok(w), Ok(other)) => {
                return Err(TestCaseError::fail(format!(
                    "llvm-mc produced single word {w:#x} for {asm}, SUT got {other:?}"
                )));
            }
            (Ok(_), Err(e)) => {
                return Err(TestCaseError::fail(format!("SUT rejected valid {asm}: {e}")));
            }
            (Err(_), Ok(EncodeResult::Words(ws))) => {
                // README.md:287 documented movz+movk expansion.
                let got = reconstruct_wide(&ws, is_64)
                    .map_err(|e| TestCaseError::fail(e))?;
                let want = if is_64 { imm as u64 } else { (imm as u64) & 0xFFFF_FFFF };
                prop_assert_eq!(got, want, "wide reconstruct for {} words={:?}", asm, ws);
            }
            (Err(_), Ok(EncodeResult::Word(s))) => {
                // gas/llvm-mc reject; SUT emitted a single (wrong) instruction.
                return Err(TestCaseError::fail(format!(
                    "llvm-mc/gas reject {asm} but SUT encoded Word({s:#x})"
                )));
            }
            (Err(_), Ok(other)) => {
                return Err(TestCaseError::fail(format!(
                    "llvm-mc/gas reject {asm} but SUT encoded {other:?}"
                )));
            }
            (Err(_), Err(_)) => {}
        }
    }

    #[test]
    fn encode_mov_diff_neon(
        vd in 0u32..=31,
        vn in 0u32..=31,
        arr in neon_arr_8_16(),
        ts in elem_size_all(),
        idx_d in 0u32..=15,
        idx_n in 0u32..=15,
        gpr_n in 0u32..=31,
        form in 0u8..=3,
    ) {
        let (ops, asm) = match form {
            0 => {
                // Vector MOV: 8b / 16b only (gas domain).
                let ops = [
                    Operand::RegArrangement { reg: vreg(vd), arrangement: arr.to_string() },
                    Operand::RegArrangement { reg: vreg(vn), arrangement: arr.to_string() },
                ];
                let asm = format!("mov {}.{}, {}.{}", vreg(vd), arr, vreg(vn), arr);
                (ops.to_vec(), asm)
            }
            1 => {
                // INS from GPR. Index in range. Width matches Ts.
                let max = imax(ts);
                let idx = idx_d % (max + 1);
                let rn = ins_gpr_name(ts, gpr_n);
                let ops = [
                    Operand::RegLane { reg: vreg(vd), elem_size: ts.to_string(), index: idx },
                    Operand::Reg(rn.clone()),
                ];
                let asm = format!("mov {}.{}[{}], {}", vreg(vd), ts, idx, rn);
                (ops.to_vec(), asm)
            }
            2 => {
                // UMOV / MOV-to-general: only .s (W) and .d (X).
                let ts = if ts == "d" || ts == "s" { ts } else if idx_d % 2 == 0 { "s" } else { "d" };
                let max = imax(ts);
                let idx = idx_d % (max + 1);
                let rd_n = umov_dest(ts, gpr_n);
                let ops = [
                    Operand::Reg(rd_n.clone()),
                    Operand::RegLane { reg: vreg(vn), elem_size: ts.to_string(), index: idx },
                ];
                let asm = format!("mov {}, {}.{}[{}]", rd_n, vreg(vn), ts, idx);
                (ops.to_vec(), asm)
            }
            _ => {
                let max = imax(ts);
                let i1 = idx_d % (max + 1);
                let i2 = idx_n % (max + 1);
                let ops = [
                    Operand::RegLane { reg: vreg(vd), elem_size: ts.to_string(), index: i1 },
                    Operand::RegLane { reg: vreg(vn), elem_size: ts.to_string(), index: i2 },
                ];
                let asm = format!("mov {}.{}[{}], {}.{}[{}]", vreg(vd), ts, i1, vreg(vn), ts, i2);
                (ops.to_vec(), asm)
            }
        };
        let mc = llvm_mc_word(&asm).unwrap_or_else(|e| panic!("llvm-mc rejected valid {asm}: {e}"));
        let sut = sut_word(&ops).unwrap_or_else(|e| panic!("SUT rejected valid {asm}: {e}"));
        prop_assert_eq!(sut, mc, "mismatch for {}", asm);
    }

    #[test]
    fn encode_mov_diff_alt_spellings(
        n in 0u32..=30,
        m in 0u32..=30,
        use_lr_rd in any::<bool>(),
        use_lr_rm in any::<bool>(),
        upper in any::<bool>(),
    ) {
        let mut rd_n = if use_lr_rd && n == 30 { "lr".to_string() } else { format!("x{n}") };
        let mut rm_n = if use_lr_rm && m == 30 { "lr".to_string() } else { format!("x{m}") };
        if upper {
            rd_n = rd_n.to_ascii_uppercase();
            rm_n = rm_n.to_ascii_uppercase();
        }
        let ops = [Operand::Reg(rd_n.clone()), Operand::Reg(rm_n.clone())];
        let asm = format!("mov {rd_n}, {rm_n}");
        let mc = llvm_mc_word(&asm).unwrap_or_else(|e| panic!("llvm-mc rejected {asm}: {e}"));
        let sut = sut_word(&ops).unwrap_or_else(|e| panic!("SUT rejected {asm}: {e}"));
        prop_assert_eq!(sut, mc, "mismatch for {}", asm);
    }

    #[test]
    fn encode_mov_metamorphic_sf(rd in 0u32..=30, rm in 0u32..=30) {
        let xops = [Operand::Reg(format!("x{rd}")), Operand::Reg(format!("x{rm}"))];
        let wops = [Operand::Reg(format!("w{rd}")), Operand::Reg(format!("w{rm}"))];
        let xw = sut_word(&xops).unwrap_or_else(|e| panic!("SUT X: {e}"));
        let ww = sut_word(&wops).unwrap_or_else(|e| panic!("SUT W: {e}"));
        prop_assert_eq!(xw ^ ww, 1u32 << 31, "sf isolation rd={} rm={} x={:#x} w={:#x}", rd, rm, xw, ww);
    }

    #[test]
    fn encode_mov_invariant_arm_fields(
        rd in 0u32..=31,
        rm in 0u32..=31,
        is_64 in any::<bool>(),
        rd_sp in any::<bool>(),
        rm_sp in any::<bool>(),
    ) {
        let rd_is_sp = rd_sp && rd == 31;
        let rm_is_sp = rm_sp && rm == 31;
        let rd_n = gpr(is_64, rd, rd_is_sp);
        let rm_n = gpr(is_64, rm, rm_is_sp);
        let ops = [Operand::Reg(rd_n.clone()), Operand::Reg(rm_n.clone())];
        let word = sut_word(&ops).unwrap_or_else(|e| panic!("SUT {rd_n},{rm_n}: {e}"));
        let sf = (word >> 31) & 1;
        let want_sf = if is_64 { 1 } else { 0 };
        prop_assert_eq!(sf, want_sf);
        prop_assert_eq!(word & 0x1f, rd);
        if rd_is_sp || rm_is_sp {
            // ADD (immediate): sf 0 0 10001 sh=0 imm12=0 Rn Rd
            let op = (word >> 30) & 1;
            let sbit = (word >> 29) & 1;
            let opc = (word >> 24) & 0x1f;
            let sh = (word >> 22) & 0x3;
            let imm12 = (word >> 10) & 0xfff;
            let rn = (word >> 5) & 0x1f;
            prop_assert_eq!(op, 0, "ADD op");
            prop_assert_eq!(sbit, 0, "ADD S");
            prop_assert_eq!(opc, 0b10001, "ADD 10001");
            prop_assert_eq!(sh, 0, "ADD sh");
            prop_assert_eq!(imm12, 0, "ADD imm12");
            prop_assert_eq!(rn, rm);
        } else {
            // ORR Rd, XZR, Rm: sf 01 01010 00 0 Rm 000000 11111 Rd
            let opc = (word >> 29) & 0b11;
            let bits28_24 = (word >> 24) & 0x1f;
            let nbit = (word >> 21) & 1;
            let imm6 = (word >> 10) & 0x3f;
            let rn = (word >> 5) & 0x1f;
            let rm_f = (word >> 16) & 0x1f;
            prop_assert_eq!(opc, 0b01, "ORR opc");
            prop_assert_eq!(bits28_24, 0b01010, "ORR 01010");
            prop_assert_eq!(nbit, 0, "ORR N");
            prop_assert_eq!(imm6, 0, "ORR imm6");
            prop_assert_eq!(rn, 0b11111, "ORR Rn=XZR");
            prop_assert_eq!(rm_f, rm);
        }
    }

    #[test]
    fn encode_mov_neg_arity(len in 0usize..=1, n in 0u32..=31) {
        let ops: Vec<Operand> = (0..len).map(|_| Operand::Reg(format!("x{n}"))).collect();
        let r = sut_result(&ops);
        prop_assert!(r.is_err(), "arity {} must Err, got {:?}", len, r);
    }

    #[test]
    fn encode_mov_neg_extra(
        rd in 0u32..=30,
        rm in 0u32..=30,
        extra in extra_operand(),
    ) {
        let ops = [
            Operand::Reg(format!("x{rd}")),
            Operand::Reg(format!("x{rm}")),
            extra,
        ];
        let r = sut_result(&ops);
        prop_assert!(r.is_err(), "extra operand must Err, got {:?}", r);
    }

    #[test]
    fn encode_mov_neg_mixed(rd in 0u32..=30, rm in 0u32..=30) {
        let ops = [Operand::Reg(format!("x{rd}")), Operand::Reg(format!("w{rm}"))];
        let r = sut_result(&ops);
        prop_assert!(r.is_err(), "mixed width must Err, got {:?}", r);
    }

    #[test]
    fn encode_mov_neg_fp(fp_n in 0u32..=31) {
        let ops = [
            Operand::Reg(format!("d{fp_n}")),
            Operand::Reg(format!("d{}", (fp_n + 1) % 32)),
        ];
        let r = sut_result(&ops);
        prop_assert!(r.is_err(), "FP scalar mov must Err, got {:?}", r);
    }

    #[test]
    fn encode_mov_neg_sp_imm(imm in mov_imm_32()) {
        let ops = [Operand::Reg("sp".into()), Operand::Imm(imm)];
        let r = sut_result(&ops);
        prop_assert!(r.is_err(), "mov sp, #imm must Err, got {:?}", r);
    }

    #[test]
    fn encode_mov_neg_w_large_imm(rd in 0u32..=30, imm in mov_imm_too_wide_for_w()) {
        let ops = [Operand::Reg(format!("w{rd}")), Operand::Imm(imm)];
        let r = sut_result(&ops);
        prop_assert!(r.is_err(), "W dest with 64-bit imm must Err, got {:?}", r);
    }

    #[test]
    fn encode_mov_neg_lane_oob(vd in 0u32..=31, rm in 0u32..=30, idx in 16u32..=31) {
        let ops = [
            Operand::RegLane { reg: vreg(vd), elem_size: "b".into(), index: idx },
            Operand::Reg(format!("w{rm}")),
        ];
        let r = sut_result(&ops);
        prop_assert!(r.is_err(), "lane OOB must Err, got {:?}", r);
    }

    #[test]
    fn encode_mov_neg_arr_mismatch(vd in 0u32..=31, vn in 0u32..=31) {
        let ops = [
            Operand::RegArrangement { reg: vreg(vd), arrangement: "16b".into() },
            Operand::RegArrangement { reg: vreg(vn), arrangement: "8b".into() },
        ];
        let r = sut_result(&ops);
        prop_assert!(r.is_err(), "16b vs 8b must Err, got {:?}", r);
    }

    #[test]
    fn encode_mov_neg_vec_4s(vd in 0u32..=31, vn in 0u32..=31) {
        let ops = [
            Operand::RegArrangement { reg: vreg(vd), arrangement: "4s".into() },
            Operand::RegArrangement { reg: vreg(vn), arrangement: "4s".into() },
        ];
        let r = sut_result(&ops);
        prop_assert!(r.is_err(), "gas-invalid 4s vector mov must Err, got {:?}", r);
    }
}
