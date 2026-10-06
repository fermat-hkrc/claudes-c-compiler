// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md:218 Compare table lists tst; README.md:507 compare_branch.rs CMP/CMN/TST;
//   encoder/mod.rs:3 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:433 "tst" => encode_tst(operands);
//   compare_branch.rs:38 "TST Rn, op -> ANDS XZR, Rn, op";
//   ARM ARM Logical (shifted register) ANDS Rd=XZR/WZR:
//     sf 11 01010 shift 0 Rm imm6 Rn Rd;
//   ARM ARM Logical (immediate) ANDS Rd=XZR/WZR:
//     sf 11 100100 N immr imms Rn Rd;
//   llvm-mc -triple=aarch64 -show-encoding (LLVM 15.0.6);
//   gas/llvm-mc reject extra operands, SP/WSP, mixed W/X, FP, non-bitmask #0/#-1,
//   and shift amounts outside 0..31 (W) / 0..63 (X).
// Stronger considered:
//   - State machine: rejected — encode_tst is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree TST decoder
//   - encode_logical as differential sibling: rejected — same-job gate
//     (3-operand ANDS mnemonic/arity); used only as metamorphic alias
//   - encode_cmp / encode_cmn: rejected — SUBS/ADDS aliases, different ARM class
// Weaker available: algebraic.metamorphic (Rn/Rm/imm6/sf isolation; ANDS XZR alias),
//   algebraic.invariant (ARM ANDS Rd=31 opc=11 layout),
//   negative_error (arity / extra / SP / mixed / FP / invalid name)
// Differential: candidate=encode_tst, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[Reg(Wn|Xn), Reg(Wm|Xm){, Shift}] <-> `tst Wn|Xn, Wm|Xm{, shift #imm}`;
//   [Reg(Wn|Xn), Imm(bitmask)] <-> `tst Wn|Xn, #imm`

use super::encode_logical;
use super::encode_tst;
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

fn sut_word(ops: &[Operand]) -> Result<u32, String> {
    match encode_tst(ops)? {
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

fn gpr(is_64: bool, n: u32) -> String {
    match n {
        31 if is_64 => "xzr".into(),
        31 => "wzr".into(),
        n => format!("{}{}", if is_64 { "x" } else { "w" }, n.min(30)),
    }
}

fn gpr_spelling(is_64: bool, n: u32, kind: u32) -> String {
    match (is_64, n, kind % 4) {
        (true, 31, 0) => "xzr".into(),
        (true, 31, 1) => "XZR".into(),
        (true, 31, 2) => "x31".into(),
        (true, 31, _) => "X31".into(),
        (false, 31, 0) => "wzr".into(),
        (false, 31, 1) => "WZR".into(),
        (false, 31, 2) => "w31".into(),
        (false, 31, _) => "W31".into(),
        (true, 30, 1) => "lr".into(),
        (true, 30, 2) => "LR".into(),
        (true, n, 1) => format!("X{n}"),
        (false, n, 1) => format!("W{n}"),
        (true, n, _) => format!("x{n}"),
        (false, n, _) => format!("w{n}"),
    }
}

fn is_32_name(rn: &str) -> bool {
    let l = rn.to_ascii_lowercase();
    l.starts_with('w') || l == "wsp" || l == "wzr"
}

fn zr_of(rn: &str) -> String {
    if is_32_name(rn) {
        "wzr".to_string()
    } else {
        "xzr".to_string()
    }
}

fn shift_kind_code(k: u32) -> &'static str {
    match k % 4 {
        0 => "lsl",
        1 => "lsr",
        2 => "asr",
        _ => "ror",
    }
}

fn shift_code(kind: &str) -> u32 {
    match kind {
        "lsl" => 0,
        "lsr" => 1,
        "asr" => 2,
        "ror" => 3,
        _ => 0,
    }
}

fn extra_operand() -> impl Strategy<Value = Operand> {
    prop_oneof![
        (0u32..=31).prop_map(|n| Operand::Reg(format!("x{n}"))),
        (0u32..=31).prop_map(|n| Operand::Reg(format!("w{n}"))),
        Just(Operand::Reg("sp".into())),
        Just(Operand::Reg("xzr".into())),
        (-4i64..=32).prop_map(Operand::Imm),
        Just(Operand::Barrier("sy".into())),
        Just(Operand::Cond("eq".into())),
        Just(Operand::Label(".L0".into())),
        Just(Operand::Symbol("foo".into())),
        Just(Operand::Mem {
            base: "x1".into(),
            offset: 0,
        }),
    ]
}

/// Independent AArch64 logical-immediate constructor (ARM ARM, not encode_bitmask_imm).
fn bitmask_from_fields(size: u32, ones: u32, immr: u32, is_64: bool) -> u64 {
    let width = if is_64 { 64u32 } else { 32 };
    let mask = if size == 64 {
        u64::MAX
    } else {
        (1u64 << size) - 1
    };
    let base = (1u64 << ones) - 1;
    let elem = if immr % size == 0 {
        base
    } else {
        let r = immr % size;
        ((base >> r) | (base << (size - r))) & mask
    };
    let mut val = 0u64;
    let mut pos = 0u32;
    while pos < width {
        val |= elem << pos;
        pos += size;
    }
    val
}

fn bitmask_case() -> impl Strategy<Value = (bool, u32, u64)> {
    prop_oneof![
        Just((true, 0u32, 1u64)),
        Just((false, 0u32, 1u64)),
        Just((true, 0u32, 0xffu64)),
        Just((true, 0u32, 0x5555_5555_5555_5555u64)),
        (prop::sample::select(vec![2u32, 4, 8, 16, 32, 64]), any::<bool>()).prop_flat_map(
            |(size, is_64)| {
                let size = if !is_64 && size == 64 { 32 } else { size };
                let ones_max = size - 1;
                (1u32..=ones_max, 0u32..=size - 1).prop_map(move |(ones, rot)| {
                    let val = bitmask_from_fields(size, ones, rot, is_64);
                    (is_64, 0u32, val)
                })
            }
        ),
        (0u32..=31, any::<bool>(), 2u32..=6u32).prop_flat_map(|(rn, is_64, log)| {
            let size = 1u32 << log.min(if is_64 { 6 } else { 5 });
            let size = size.max(2);
            (1u32..=size - 1, 0u32..=size - 1).prop_map(move |(ones, rot)| {
                let val = bitmask_from_fields(size, ones, rot, is_64);
                (is_64, rn, val)
            })
        }),
    ]
}

fn invalid_name(which: u32) -> String {
    match which % 8 {
        0 => "x32".to_string(),
        1 => "w32".to_string(),
        2 => "foo".to_string(),
        3 => "".to_string(),
        4 => "r0".to_string(),
        5 => "x".to_string(),
        6 => "x-1".to_string(),
        _ => "x99".to_string(),
    }
}

fn wrong_reg_ops(kind: u32, n: u32) -> Vec<Operand> {
    let n = n.min(31);
    match kind % 9 {
        0 => vec![Operand::Reg("sp".into()), Operand::Reg("x0".into())],
        1 => vec![Operand::Reg("x0".into()), Operand::Reg("sp".into())],
        2 => vec![Operand::Reg("wsp".into()), Operand::Reg("w0".into())],
        3 => vec![Operand::Reg("w0".into()), Operand::Reg("wsp".into())],
        4 => vec![
            Operand::Reg(format!("x{}", n.min(30))),
            Operand::Reg(format!("w{}", n.min(30))),
        ],
        5 => vec![
            Operand::Reg(format!("w{}", n.min(30))),
            Operand::Reg(format!("x{}", n.min(30))),
        ],
        6 => vec![
            Operand::Reg(format!("d{}", n)),
            Operand::Reg(format!("d{}", n.min(30))),
        ],
        7 => vec![
            Operand::Reg(format!("x{}", n.min(30))),
            Operand::Reg(format!("d{}", n)),
        ],
        _ => vec![
            Operand::Reg(invalid_name(n)),
            Operand::Reg("x0".into()),
        ],
    }
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_tst_kat_llvm_mc_x0_x1() {
    let want = 0xea01001fu32;
    let mc = llvm_mc_word("tst x0, x1").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [Operand::Reg("x0".into()), Operand::Reg("x1".into())];
    match encode_tst(&ops) {
        Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
        other => panic!(
            "SUT KAT: expected Word({:#010x}) for tst x0, x1, got {:?}",
            want, other
        ),
    }
}

#[test]
fn encode_tst_kat_llvm_mc_w0_w1() {
    let want = 0x6a01001fu32;
    let mc = llvm_mc_word("tst w0, w1").expect("llvm-mc KAT w");
    assert_eq!(mc, want, "llvm-mc KAT w mapping broken");
    let ops = [Operand::Reg("w0".into()), Operand::Reg("w1".into())];
    match encode_tst(&ops) {
        Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
        other => panic!(
            "SUT KAT: expected Word({:#010x}) for tst w0, w1, got {:?}",
            want, other
        ),
    }
}

#[test]
fn encode_tst_kat_llvm_mc_x0_imm1() {
    let want = 0xf240001fu32;
    let mc = llvm_mc_word("tst x0, #1").expect("llvm-mc KAT imm");
    assert_eq!(mc, want, "llvm-mc KAT imm mapping broken");
    let ops = [Operand::Reg("x0".into()), Operand::Imm(1)];
    match encode_tst(&ops) {
        Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
        other => panic!(
            "SUT KAT: expected Word({:#010x}) for tst x0, #1, got {:?}",
            want, other
        ),
    }
}

#[test]
fn encode_tst_kat_llvm_mc_w0_imm1() {
    let want = 0x7200001fu32;
    let mc = llvm_mc_word("tst w0, #1").expect("llvm-mc KAT w imm");
    assert_eq!(mc, want, "llvm-mc KAT w imm mapping broken");
    let ops = [Operand::Reg("w0".into()), Operand::Imm(1)];
    match encode_tst(&ops) {
        Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
        other => panic!(
            "SUT KAT: expected Word({:#010x}) for tst w0, #1, got {:?}",
            want, other
        ),
    }
}

#[test]
fn encode_tst_kat_llvm_mc_shift_ror() {
    let want = 0xeac11c1fu32;
    let mc = llvm_mc_word("tst x0, x1, ror #7").expect("llvm-mc KAT ror");
    assert_eq!(mc, want, "llvm-mc KAT ror mapping broken");
    let ops = [
        Operand::Reg("x0".into()),
        Operand::Reg("x1".into()),
        Operand::Shift {
            kind: "ror".into(),
            amount: 7,
        },
    ];
    match encode_tst(&ops) {
        Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
        other => panic!(
            "SUT KAT: expected Word({:#010x}) for tst x0, x1, ror #7, got {:?}",
            want, other
        ),
    }
}

#[test]
fn encode_tst_kat_ands_alias() {
    let mc = llvm_mc_word("ands xzr, x0, x1").expect("llvm-mc ANDS alias");
    let ops = [Operand::Reg("x0".into()), Operand::Reg("x1".into())];
    match encode_tst(&ops) {
        Ok(EncodeResult::Word(w)) => assert_eq!(w, mc),
        other => panic!("TST must match ANDS XZR alias, got {:?}", other),
    }
}

#[test]
fn test_encode_tst_regression_extra_operand() {
    let ops = [
        Operand::Reg("x0".into()),
        Operand::Reg("x1".into()),
        Operand::Reg("x2".into()),
    ];
    assert!(
        encode_tst(&ops).is_err(),
        "TST must reject extra operand (llvm-mc: expected shift)"
    );
}

#[test]
fn test_encode_tst_regression_sp_rn() {
    let ops = [Operand::Reg("sp".into()), Operand::Reg("x0".into())];
    assert!(
        encode_tst(&ops).is_err(),
        "TST must reject SP as Rn"
    );
}

#[test]
fn test_encode_tst_regression_sp_rm() {
    let ops = [Operand::Reg("x0".into()), Operand::Reg("sp".into())];
    assert!(
        encode_tst(&ops).is_err(),
        "TST must reject SP as Rm"
    );
}

#[test]
fn test_encode_tst_regression_mixed_width() {
    let ops = [Operand::Reg("x0".into()), Operand::Reg("w1".into())];
    assert!(
        encode_tst(&ops).is_err(),
        "TST must reject mixed W/X"
    );
}

#[test]
fn test_encode_tst_regression_fp_reg() {
    let ops = [Operand::Reg("d0".into()), Operand::Reg("d1".into())];
    assert!(
        encode_tst(&ops).is_err(),
        "TST must reject FP registers"
    );
}

#[test]
fn test_encode_tst_regression_shift_oor() {
    let ops = [
        Operand::Reg("w0".into()),
        Operand::Reg("w1".into()),
        Operand::Shift {
            kind: "lsl".into(),
            amount: 32,
        },
    ];
    assert!(
        encode_tst(&ops).is_err(),
        "TST must reject W-form lsl #32 (llvm-mc range 0..31)"
    );
}

proptest! {
    #![proptest_config(cfg())]

    // Oracle: differential
    // Target: encoder.compare_branch.encode_tst
    #[test]
    fn encode_tst_diff_valid_reg(
        rn in 0u32..=31,
        rm in 0u32..=31,
        is_64 in any::<bool>(),
        n_kind in 0u32..=3,
        m_kind in 0u32..=3,
        use_shift in any::<bool>(),
        sk in 0u32..=3,
        raw_amt in 0u32..=63,
    ) {
        let rn_s = gpr_spelling(is_64, rn, n_kind);
        let rm_s = gpr_spelling(is_64, rm, m_kind);
        let max_amt = if is_64 { 63u32 } else { 31u32 };
        let mut ops = vec![Operand::Reg(rn_s.clone()), Operand::Reg(rm_s.clone())];
        let mut asm = format!("tst {}, {}", rn_s, rm_s);
        if use_shift {
            let amt = raw_amt.min(max_amt);
            let kind = shift_kind_code(sk);
            ops.push(Operand::Shift {
                kind: kind.to_string(),
                amount: amt,
            });
            asm = format!("tst {}, {}, {} #{}", rn_s, rm_s, kind, amt);
        }
        let sut = match encode_tst(&ops) {
            Ok(EncodeResult::Word(w)) => w,
            other => {
                return Err(TestCaseError::fail(format!(
                    "SUT rejected valid TST {}: {:?}",
                    asm, other
                )));
            }
        };
        let mc = llvm_mc_word(&asm).map_err(|e| {
            TestCaseError::fail(format!("llvm-mc rejected valid TST {}: {}", asm, e))
        })?;
        prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
    }

    // Oracle: differential
    // Target: encoder.compare_branch.encode_tst
    #[test]
    fn encode_tst_diff_valid_imm(case in bitmask_case(), n_kind in 0u32..=3) {
        let (is_64, rn, imm) = case;
        let rn_s = gpr_spelling(is_64, rn, n_kind);
        let ops = [Operand::Reg(rn_s.clone()), Operand::Imm(imm as i64)];
        let asm = format!("tst {}, #{:#x}", rn_s, imm);
        let sut = match encode_tst(&ops) {
            Ok(EncodeResult::Word(w)) => w,
            other => {
                return Err(TestCaseError::fail(format!(
                    "SUT rejected valid TST {}: {:?}",
                    asm, other
                )));
            }
        };
        let mc = llvm_mc_word(&asm).map_err(|e| {
            TestCaseError::fail(format!("llvm-mc rejected valid TST {}: {}", asm, e))
        })?;
        prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
    }

    // Oracle: algebraic.invariant
    // Target: encoder.compare_branch.encode_tst
    #[test]
    fn encode_tst_arm_fields(
        rn in 0u32..=31,
        rm in 0u32..=31,
        is_64 in any::<bool>(),
        sk in 0u32..=3,
        raw_amt in 0u32..=63,
    ) {
        let max_amt = if is_64 { 63u32 } else { 31u32 };
        let amt = raw_amt.min(max_amt);
        let kind = shift_kind_code(sk);
        let ops = [
            Operand::Reg(gpr(is_64, rn)),
            Operand::Reg(gpr(is_64, rm)),
            Operand::Shift {
                kind: kind.to_string(),
                amount: amt,
            },
        ];
        let w = sut_word(&ops).expect("SUT");
        let sf = if is_64 { 1u32 } else { 0 };
        let st = shift_code(kind);
        prop_assert_eq!(w & 0x1f, 31u32, "Rd must be XZR/WZR (31)");
        prop_assert_eq!((w >> 29) & 0b11, 0b11u32, "opc must be ANDS (11)");
        prop_assert_eq!((w >> 24) & 0x1f, 0b01010u32, "shifted-register opcode");
        prop_assert_eq!((w >> 21) & 1, 0u32, "N must be 0");
        prop_assert_eq!(w >> 31, sf, "sf from Rn width");
        prop_assert_eq!((w >> 5) & 0x1f, rn, "Rn");
        prop_assert_eq!((w >> 16) & 0x1f, rm, "Rm");
        prop_assert_eq!((w >> 22) & 0b11, st, "shift type");
        prop_assert_eq!((w >> 10) & 0x3f, amt, "imm6");
    }

    // Oracle: algebraic.metamorphic
    // Target: encoder.compare_branch.encode_tst
    #[test]
    fn encode_tst_meta_vs_ands(
        rn in 0u32..=31,
        rm in 0u32..=31,
        is_64 in any::<bool>(),
        use_imm in any::<bool>(),
        use_shift in any::<bool>(),
        sk in 0u32..=3,
        raw_amt in 0u32..=63,
        imm in 1i64..=0xff,
    ) {
        let rn_s = gpr(is_64, rn);
        let mut ops = vec![Operand::Reg(rn_s.clone())];
        if use_imm {
            ops.push(Operand::Imm(imm));
        } else {
            ops.push(Operand::Reg(gpr(is_64, rm)));
            if use_shift {
                let max_amt = if is_64 { 63u32 } else { 31u32 };
                ops.push(Operand::Shift {
                    kind: shift_kind_code(sk).to_string(),
                    amount: raw_amt.min(max_amt),
                });
            }
        }
        let zr = zr_of(&rn_s);
        let mut ands_ops = vec![Operand::Reg(zr)];
        ands_ops.extend(ops.iter().cloned());
        let tst = encode_tst(&ops);
        let ands = encode_logical(&ands_ops, 0b11);
        prop_assert_eq!(
            format!("{:?}", tst),
            format!("{:?}", ands),
            "TST must equal ANDS ZR, ..."
        );
    }

    // Oracle: algebraic.metamorphic
    // Target: encoder.compare_branch.encode_tst
    #[test]
    fn encode_tst_metamorphic_fields(
        rn in 0u32..=30,
        rm in 0u32..=30,
        is_64 in any::<bool>(),
        amt in 0u32..=30,
    ) {
        let max_amt = if is_64 { 63u32 } else { 31u32 };
        let amt = amt.min(max_amt.saturating_sub(1));
        let ops = |n: u32, m: u32, a: u32, w64: bool| {
            [
                Operand::Reg(gpr(w64, n)),
                Operand::Reg(gpr(w64, m)),
                Operand::Shift {
                    kind: "lsl".into(),
                    amount: a,
                },
            ]
        };
        let w = sut_word(&ops(rn, rm, amt, is_64)).expect("base");
        let w_rn = sut_word(&ops(rn + 1, rm, amt, is_64)).expect("Rn+1");
        let w_rm = sut_word(&ops(rn, rm + 1, amt, is_64)).expect("Rm+1");
        let w_amt = sut_word(&ops(rn, rm, amt + 1, is_64)).expect("amt+1");
        let w_sf = sut_word(&ops(rn, rm, amt, !is_64)).expect("sf flip");
        prop_assert_eq!(w_rn, w + (1 << 5), "Rn+1 must increment bits[9:5] only");
        prop_assert_eq!(w_rm, w + (1 << 16), "Rm+1 must increment bits[20:16] only");
        prop_assert_eq!(w_amt, w + (1 << 10), "amt+1 must increment bits[15:10] only");
        prop_assert_eq!(w_sf ^ w, 1u32 << 31, "W vs X must flip only sf bit 31");
    }

    // Oracle: negative_error
    // Target: encoder.compare_branch.encode_tst
    #[test]
    fn encode_tst_neg_arity(arity in 0u32..=1u32, n in 0u32..=30u32) {
        let ops: Vec<Operand> = if arity == 0 {
            vec![]
        } else {
            vec![Operand::Reg(format!("x{}", n))]
        };
        prop_assert!(
            encode_tst(&ops).is_err(),
            "tst with {} operand(s) must Err (llvm-mc: too few operands)",
            arity
        );
    }

    // Oracle: negative_error
    // Target: encoder.compare_branch.encode_tst
    #[test]
    fn encode_tst_neg_extra_operand(
        rn in 0u32..=31,
        rm in 0u32..=31,
        is_64 in any::<bool>(),
        extra in extra_operand(),
    ) {
        let ops = vec![
            Operand::Reg(gpr(is_64, rn)),
            Operand::Reg(gpr(is_64, rm)),
            extra,
        ];
        prop_assert!(
            encode_tst(&ops).is_err(),
            "tst Rn, Rm, extra must Err (llvm-mc: invalid operand)"
        );
    }

    // Oracle: negative_error
    // Target: encoder.compare_branch.encode_tst
    #[test]
    fn encode_tst_neg_sp(which in 0u32..=3u32, n in 0u32..=30u32) {
        let ops = wrong_reg_ops(which, n); // 0..3 = SP/WSP as Rn or Rm
        prop_assert!(
            encode_tst(&ops).is_err(),
            "tst SP/WSP kind={} must Err (llvm-mc: invalid operand)",
            which
        );
    }

    // Oracle: negative_error
    // Target: encoder.compare_branch.encode_tst
    #[test]
    fn encode_tst_neg_mixed_width(n in 0u32..=30u32, x_first in any::<bool>()) {
        let ops = if x_first {
            vec![
                Operand::Reg(format!("x{n}")),
                Operand::Reg(format!("w{n}")),
            ]
        } else {
            vec![
                Operand::Reg(format!("w{n}")),
                Operand::Reg(format!("x{n}")),
            ]
        };
        prop_assert!(
            encode_tst(&ops).is_err(),
            "tst mixed W/X must Err (llvm-mc: expected compatible register)"
        );
    }

    // Oracle: negative_error
    // Target: encoder.compare_branch.encode_tst
    #[test]
    fn encode_tst_neg_fp_reg(
        n in 0u32..=31,
        m in 0u32..=31,
        pref in 0u32..=5,
        as_rm in any::<bool>(),
    ) {
        let p = ["d", "s", "q", "v", "h", "b"][(pref as usize) % 6];
        let ops = if as_rm {
            vec![
                Operand::Reg(format!("x{}", n.min(30))),
                Operand::Reg(format!("{p}{m}")),
            ]
        } else {
            vec![
                Operand::Reg(format!("{p}{n}")),
                Operand::Reg(format!("{p}{m}")),
            ]
        };
        prop_assert!(
            encode_tst(&ops).is_err(),
            "tst FP/SIMD register must Err (llvm-mc: invalid operand)"
        );
    }

    // Oracle: negative_error
    // Target: encoder.compare_branch.encode_tst
    #[test]
    fn encode_tst_neg_invalid_name(which in 0u32..=7u32) {
        let ops = [
            Operand::Reg(invalid_name(which)),
            Operand::Reg("x0".into()),
        ];
        prop_assert!(
            encode_tst(&ops).is_err(),
            "tst with unparsable register {} must Err",
            invalid_name(which)
        );
    }

    // Oracle: negative_error
    // Target: encoder.compare_branch.encode_tst
    #[test]
    fn encode_tst_neg_shift_oor(
        rn in 0u32..=31,
        rm in 0u32..=31,
        is_64 in any::<bool>(),
        sk in 0u32..=3,
        amt in prop_oneof![
            Just(32u32),
            Just(33u32),
            Just(63u32),
            Just(64u32),
            Just(65u32),
            Just(127u32),
            Just(u32::MAX),
            32u32..=128,
        ],
    ) {
        let max_ok = if is_64 { 63u32 } else { 31u32 };
        prop_assume!(amt > max_ok);
        let kind = shift_kind_code(sk);
        let ops = [
            Operand::Reg(gpr(is_64, rn)),
            Operand::Reg(gpr(is_64, rm)),
            Operand::Shift {
                kind: kind.to_string(),
                amount: amt,
            },
        ];
        prop_assert!(
            encode_tst(&ops).is_err(),
            "tst shift {} #{} on {} must Err (llvm-mc range 0..{})",
            kind,
            amt,
            if is_64 { "X" } else { "W" },
            max_ok
        );
    }

    // Oracle: negative_error
    // Target: encoder.compare_branch.encode_tst
    #[test]
    fn encode_tst_neg_invalid_imm(
        rn in 0u32..=31,
        is_64 in any::<bool>(),
        imm in prop_oneof![
            Just(0i64),
            Just(-1i64),
            Just(0x1111_1111i64),
            Just(3i64),
            Just(i64::MIN),
            Just(i64::MAX),
            Just(6i64),
            Just(-2i64),
        ],
    ) {
        let rn_s = gpr(is_64, rn);
        let asm = format!("tst {}, #{}", rn_s, imm);
        // Only assert Err when llvm-mc also rejects (non-bitmask).
        if llvm_mc_word(&asm).is_ok() {
            return Ok(());
        }
        let ops = [Operand::Reg(rn_s), Operand::Imm(imm)];
        prop_assert!(
            encode_tst(&ops).is_err(),
            "tst {}, #{} is not a bitmask immediate; must Err",
            gpr(is_64, rn),
            imm
        );
    }
}
