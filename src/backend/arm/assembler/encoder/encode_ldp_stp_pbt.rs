// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md:221 Loads/Stores table lists ldp, stp;
//   encoder/mod.rs:3 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:492-493 "ldp"/"stp" => encode_ldp_stp;
//   ARM ARM LDP/STP: opc 101 V {001 post, 010 signed, 011 pre} L imm7 Rt2 Rn Rt;
//   integer opc=00 Wt scale=4 offset in [-256,252]; opc=10 Xt scale=8 offset in [-512,504].
//   Rt is Wt/Xt (31=WZR/XZR, never SP); Rn is Xn|SP (not XZR).
// Stronger considered:
//   - State machine: rejected — encode_ldp_stp is a pure function with no lifecycle
//   - Algebraic round-trip via in-tree decoder: rejected — no LDP/STP decoder
//   - encode_ldnp_stnp as differential sibling: rejected — same-job gate
//     (LDNP/STNP non-temporal, bits[25:23]=000; shared get_reg / same crate)
// Weaker available: algebraic.invariant (ARM field unpack), algebraic.metamorphic (Rt1/Rt2/Rn/imm7, load/store, pre/post),
//   negative_error (arity / extra / SP-dest / XZR-base / W-base / mixed-width / writeback-overlap / offset range / LDP Rt1==Rt2)
// Differential: candidate=encode_ldp_stp, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=(Reg(Rt1), Reg(Rt2), Mem/Pre/Post, is_load) <-> `ldp/stp Rt1, Rt2, <addr>`

use super::encode_ldp_stp;
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

fn xt_name(n: u32) -> String {
    if n == 31 {
        "xzr".into()
    } else {
        format!("x{n}")
    }
}

fn wt_name(n: u32) -> String {
    if n == 31 {
        "wzr".into()
    } else {
        format!("w{n}")
    }
}

fn gp_rt(is_64: bool, n: u32) -> String {
    if is_64 {
        xt_name(n)
    } else {
        wt_name(n)
    }
}

fn rn_name(n: u32) -> String {
    if n == 31 {
        "sp".into()
    } else {
        format!("x{n}")
    }
}

fn scale_of(is_64: bool) -> i64 {
    if is_64 { 8 } else { 4 }
}

fn off_min(is_64: bool) -> i64 {
    if is_64 { -512 } else { -256 }
}

fn off_max(is_64: bool) -> i64 {
    if is_64 { 504 } else { 252 }
}

fn mnemonic(is_load: bool) -> &'static str {
    if is_load { "ldp" } else { "stp" }
}

fn asm_mem(rn: &str, offset: i64) -> String {
    if offset == 0 {
        format!("[{rn}]")
    } else {
        format!("[{rn}, #{offset}]")
    }
}

fn sut_word(ops: &[Operand], is_load: bool) -> Result<u32, String> {
    match encode_ldp_stp(ops, is_load)? {
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

fn unpack_pair(word: u32) -> (u32, u32, u32, u32, u32, u32, u32, u32, u32) {
    let opc = (word >> 30) & 0b11;
    let bits29_27 = (word >> 27) & 0b111;
    let v = (word >> 26) & 1;
    let mode = (word >> 23) & 0b111;
    let l = (word >> 22) & 1;
    let imm7 = (word >> 15) & 0x7F;
    let rt2 = (word >> 10) & 0x1F;
    let rn = (word >> 5) & 0x1F;
    let rt1 = word & 0x1F;
    (opc, bits29_27, v, mode, l, imm7, rt2, rn, rt1)
}

fn reg_edge() -> impl Strategy<Value = u32> {
    prop_oneof![Just(0u32), Just(1u32), Just(30u32), Just(31u32), 0u32..=31]
}

fn imm7_edge() -> impl Strategy<Value = i32> {
    prop_oneof![
        Just(-64i32),
        Just(-63i32),
        Just(-1i32),
        Just(0i32),
        Just(1i32),
        Just(62i32),
        Just(63i32),
        -64i32..=63
    ]
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
        Just(Operand::RegArrangement {
            reg: "v0".into(),
            arrangement: "8b".into(),
        }),
    ]
}

fn bad_address_kind() -> impl Strategy<Value = Operand> {
    prop_oneof![
        Just(Operand::Imm(0)),
        Just(Operand::Imm(8)),
        Just(Operand::Reg("x1".into())),
        Just(Operand::Shift {
            kind: "lsl".into(),
            amount: 3,
        }),
        Just(Operand::Cond("eq".into())),
        Just(Operand::Label("L0".into())),
        Just(Operand::Extend {
            kind: "uxtw".into(),
            amount: 0,
        }),
        Just(Operand::MemRegOffset {
            base: "x1".into(),
            index: "x2".into(),
            extend: None,
            shift: None,
        }),
    ]
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_ldp_stp_kat_llvm_mc_x0_x1_x2() {
    let want = 0xA940_0440u32;
    let mc = llvm_mc_word("ldp x0, x1, [x2]").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [
        Operand::Reg("x0".into()),
        Operand::Reg("x1".into()),
        Operand::Mem {
            base: "x2".into(),
            offset: 0,
        },
    ];
    let sut = sut_word(&ops, true).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_ldp_stp_kat_llvm_mc_x0_x1_imm8() {
    let want = 0xA940_8440u32;
    let mc = llvm_mc_word("ldp x0, x1, [x2, #8]").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [
        Operand::Reg("x0".into()),
        Operand::Reg("x1".into()),
        Operand::Mem {
            base: "x2".into(),
            offset: 8,
        },
    ];
    let sut = sut_word(&ops, true).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_ldp_stp_kat_llvm_mc_stp_w() {
    let want = 0x2900_8C82u32;
    let mc = llvm_mc_word("stp w2, w3, [x4, #4]").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [
        Operand::Reg("w2".into()),
        Operand::Reg("w3".into()),
        Operand::Mem {
            base: "x4".into(),
            offset: 4,
        },
    ];
    let sut = sut_word(&ops, false).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_ldp_stp_kat_llvm_mc_pre_post() {
    let want_post = 0xA8C1_0440u32;
    let mc_post = llvm_mc_word("ldp x0, x1, [x2], #16").expect("llvm-mc post KAT");
    assert_eq!(mc_post, want_post);
    let ops_post = [
        Operand::Reg("x0".into()),
        Operand::Reg("x1".into()),
        Operand::MemPostIndex {
            base: "x2".into(),
            offset: 16,
        },
    ];
    assert_eq!(sut_word(&ops_post, true).expect("SUT post"), want_post);

    let want_pre = 0xA981_0440u32;
    let mc_pre = llvm_mc_word("stp x0, x1, [x2, #16]!").expect("llvm-mc pre KAT");
    assert_eq!(mc_pre, want_pre);
    let ops_pre = [
        Operand::Reg("x0".into()),
        Operand::Reg("x1".into()),
        Operand::MemPreIndex {
            base: "x2".into(),
            offset: 16,
        },
    ];
    assert_eq!(sut_word(&ops_pre, false).expect("SUT pre"), want_pre);
}

#[test]
fn encode_ldp_stp_kat_llvm_mc_simd() {
    let want_d = 0x6D41_0440u32;
    let mc_d = llvm_mc_word("ldp d0, d1, [x2, #16]").expect("llvm-mc D KAT");
    assert_eq!(mc_d, want_d);
    let ops_d = [
        Operand::Reg("d0".into()),
        Operand::Reg("d1".into()),
        Operand::Mem {
            base: "x2".into(),
            offset: 16,
        },
    ];
    assert_eq!(sut_word(&ops_d, true).expect("SUT D"), want_d);

    let want_s = 0x2D40_0400u32;
    let mc_s = llvm_mc_word("ldp s0, s1, [x0]").expect("llvm-mc S KAT");
    assert_eq!(mc_s, want_s);
    let ops_s = [
        Operand::Reg("s0".into()),
        Operand::Reg("s1".into()),
        Operand::Mem {
            base: "x0".into(),
            offset: 0,
        },
    ];
    assert_eq!(sut_word(&ops_s, true).expect("SUT S"), want_s);

    let want_q = 0xAD7F_07E0u32;
    let mc_q = llvm_mc_word("ldp q0, q1, [sp, #-32]").expect("llvm-mc Q KAT");
    assert_eq!(mc_q, want_q);
    let ops_q = [
        Operand::Reg("q0".into()),
        Operand::Reg("q1".into()),
        Operand::Mem {
            base: "sp".into(),
            offset: -32,
        },
    ];
    assert_eq!(sut_word(&ops_q, true).expect("SUT Q"), want_q);
}

#[test]
fn test_encode_ldp_stp_regression_extra_operand() {
    let ops = [
        Operand::Reg("w0".into()),
        Operand::Reg("w0".into()),
        Operand::Mem {
            base: "sp".into(),
            offset: 0,
        },
        Operand::Reg("x0".into()),
    ];
    assert!(
        encode_ldp_stp(&ops, false).is_err(),
        "STP W0, W0, [SP], X0 must Err; llvm-mc rejects a fourth operand"
    );
}

#[test]
fn test_encode_ldp_stp_regression_sp_dest() {
    let ops = [
        Operand::Reg("sp".into()),
        Operand::Reg("w0".into()),
        Operand::Mem {
            base: "x0".into(),
            offset: 0,
        },
    ];
    assert!(
        encode_ldp_stp(&ops, false).is_err(),
        "STP SP, W0, [X0] must Err; SP is not a valid Rt (llvm-mc rejects it)"
    );
}

#[test]
fn test_encode_ldp_stp_regression_xzr_base() {
    let ops = [
        Operand::Reg("w0".into()),
        Operand::Reg("w0".into()),
        Operand::Mem {
            base: "xzr".into(),
            offset: 0,
        },
    ];
    assert!(
        encode_ldp_stp(&ops, false).is_err(),
        "STP W0, W0, [XZR] must Err; Rn=31 is SP not XZR (llvm-mc rejects it)"
    );
}

#[test]
fn test_encode_ldp_stp_regression_w_base() {
    let ops = [
        Operand::Reg("w0".into()),
        Operand::Reg("w0".into()),
        Operand::Mem {
            base: "w0".into(),
            offset: 0,
        },
    ];
    assert!(
        encode_ldp_stp(&ops, false).is_err(),
        "STP W0, W0, [W0] must Err; base must be Xn|SP (llvm-mc rejects it)"
    );
}

#[test]
fn test_encode_ldp_stp_regression_mixed_width() {
    let ops = [
        Operand::Reg("x0".into()),
        Operand::Reg("w0".into()),
        Operand::Mem {
            base: "x0".into(),
            offset: 0,
        },
    ];
    assert!(
        encode_ldp_stp(&ops, false).is_err(),
        "STP X0, W0, [X0] must Err; mixed X/W pair is invalid (llvm-mc rejects it)"
    );
}

#[test]
fn test_encode_ldp_stp_regression_writeback_overlap() {
    let ops = [
        Operand::Reg("w0".into()),
        Operand::Reg("w0".into()),
        Operand::MemPreIndex {
            base: "x0".into(),
            offset: 4,
        },
    ];
    assert!(
        encode_ldp_stp(&ops, false).is_err(),
        "STP W0, W0, [X0, #4]! must Err; writeback Rn==Rt is unpredictable (llvm-mc rejects it)"
    );
}

#[test]
fn test_encode_ldp_stp_regression_ldp_same_rt() {
    let ops = [
        Operand::Reg("x0".into()),
        Operand::Reg("x0".into()),
        Operand::Mem {
            base: "x1".into(),
            offset: 0,
        },
    ];
    assert!(
        encode_ldp_stp(&ops, true).is_err(),
        "LDP X0, X0, [X1] must Err; llvm-mc: unpredictable LDP instruction, Rt2==Rt"
    );
}

#[test]
fn test_encode_ldp_stp_regression_imm7_range() {
    let ops = [
        Operand::Reg("w0".into()),
        Operand::Reg("w0".into()),
        Operand::Mem {
            base: "sp".into(),
            offset: -257,
        },
    ];
    assert!(
        encode_ldp_stp(&ops, false).is_err(),
        "STP W0, W0, [SP, #-257] must Err; W-pair offset must be a multiple of 4 in [-256, 252]"
    );
}

proptest! {
    #![proptest_config(cfg())]

    // Oracle: differential — llvm-mc signed-offset LDP/STP
    // Target: encoder.load_store.encode_ldp_stp
    #[test]
    fn encode_ldp_stp_diff_signed_offset_llvm_mc(
        is_load in any::<bool>(),
        is_64 in any::<bool>(),
        rt1 in reg_edge(),
        rt2 in reg_edge(),
        rn in reg_edge(),
        imm7 in imm7_edge(),
    ) {
        prop_assume!(!is_load || rt1 != rt2);
        let dest1 = gp_rt(is_64, rt1);
        let dest2 = gp_rt(is_64, rt2);
        let base = rn_name(rn);
        let offset = (imm7 as i64) * scale_of(is_64);
        let asm = format!(
            "{} {dest1}, {dest2}, {}",
            mnemonic(is_load),
            asm_mem(&base, offset)
        );
        let ops = [
            Operand::Reg(dest1),
            Operand::Reg(dest2),
            Operand::Mem { base, offset },
        ];
        let mc = llvm_mc_word(&asm).expect("llvm-mc");
        let sut = sut_word(&ops, is_load).expect("SUT");
        prop_assert_eq!(sut, mc, "signed-offset mismatch for {}", asm);
    }

    // Oracle: differential — llvm-mc pre-index / post-index LDP/STP
    // Target: encoder.load_store.encode_ldp_stp
    #[test]
    fn encode_ldp_stp_diff_pre_post_llvm_mc(
        is_load in any::<bool>(),
        is_64 in any::<bool>(),
        rt1 in reg_edge(),
        rt2 in reg_edge(),
        rn in reg_edge(),
        imm7 in imm7_edge(),
        pre in any::<bool>(),
    ) {
        prop_assume!(!is_load || rt1 != rt2);
        prop_assume!(rn == 31 || (rn != rt1 && rn != rt2));
        let dest1 = gp_rt(is_64, rt1);
        let dest2 = gp_rt(is_64, rt2);
        let base = rn_name(rn);
        let offset = (imm7 as i64) * scale_of(is_64);
        let mnem = mnemonic(is_load);
        let (ops, asm) = if pre {
            let asm = format!("{mnem} {dest1}, {dest2}, [{base}, #{offset}]!");
            let ops = [
                Operand::Reg(dest1),
                Operand::Reg(dest2),
                Operand::MemPreIndex {
                    base: base.clone(),
                    offset,
                },
            ];
            (ops, asm)
        } else {
            let asm = format!("{mnem} {dest1}, {dest2}, [{base}], #{offset}");
            let ops = [
                Operand::Reg(dest1),
                Operand::Reg(dest2),
                Operand::MemPostIndex {
                    base: base.clone(),
                    offset,
                },
            ];
            (ops, asm)
        };
        let mc = llvm_mc_word(&asm).expect("llvm-mc");
        let sut = sut_word(&ops, is_load).expect("SUT");
        prop_assert_eq!(sut, mc, "pre/post mismatch for {}", asm);
    }

    // Oracle: algebraic.invariant — ARM LDP/STP field layout
    // Target: encoder.load_store.encode_ldp_stp
    #[test]
    fn encode_ldp_stp_inv_arm_layout(
        is_load in any::<bool>(),
        is_64 in any::<bool>(),
        rt1 in reg_edge(),
        rt2 in reg_edge(),
        rn in reg_edge(),
        imm7 in imm7_edge(),
        mode in 0u32..=2,
    ) {
        let dest1 = gp_rt(is_64, rt1);
        let dest2 = gp_rt(is_64, rt2);
        let base = rn_name(rn);
        let offset = (imm7 as i64) * scale_of(is_64);
        let addr = match mode {
            0 => Operand::MemPostIndex { base, offset },
            1 => Operand::Mem { base, offset },
            _ => Operand::MemPreIndex { base, offset },
        };
        let ops = [Operand::Reg(dest1), Operand::Reg(dest2), addr];
        let word = sut_word(&ops, is_load).expect("SUT");
        let (opc, bits29_27, v, mode_bits, l, imm7_f, rt2_f, rn_f, rt1_f) = unpack_pair(word);
        let expect_opc = if is_64 { 0b10 } else { 0b00 };
        let expect_mode = match mode {
            0 => 0b001u32,
            1 => 0b010,
            _ => 0b011,
        };
        let expect_imm7 = (imm7 as u32) & 0x7F;
        prop_assert_eq!(opc, expect_opc, "opc");
        prop_assert_eq!(bits29_27, 0b101, "bits[29:27]");
        prop_assert_eq!(v, 0, "V");
        prop_assert_eq!(mode_bits, expect_mode, "mode");
        prop_assert_eq!(l, u32::from(is_load), "L");
        prop_assert_eq!(imm7_f, expect_imm7, "imm7");
        prop_assert_eq!(rt2_f, rt2, "Rt2");
        prop_assert_eq!(rn_f, rn, "Rn");
        prop_assert_eq!(rt1_f, rt1, "Rt");
    }

    // Oracle: algebraic.metamorphic — Rt1/Rt2/Rn/imm7/L/mode isolation
    // Target: encoder.load_store.encode_ldp_stp
    #[test]
    fn encode_ldp_stp_meta_fields(
        is_load in any::<bool>(),
        is_64 in any::<bool>(),
        rt1 in 0u32..=30,
        rt2 in 0u32..=30,
        rn in 0u32..=30,
        imm7 in -64i32..=62,
    ) {
        let scale = scale_of(is_64);
        let mk = |r1: u32, r2: u32, n: u32, i: i32, load: bool, pre: bool| {
            let offset = (i as i64) * scale;
            let addr = if pre {
                Operand::MemPreIndex {
                    base: rn_name(n),
                    offset,
                }
            } else {
                Operand::MemPostIndex {
                    base: rn_name(n),
                    offset,
                }
            };
            let ops = [
                Operand::Reg(gp_rt(is_64, r1)),
                Operand::Reg(gp_rt(is_64, r2)),
                addr,
            ];
            sut_word(&ops, load).expect("SUT")
        };
        let base = mk(rt1, rt2, rn, imm7, is_load, false);
        prop_assert_eq!(mk(rt1 + 1, rt2, rn, imm7, is_load, false).wrapping_sub(base), 1, "Rt1+1");
        prop_assert_eq!(mk(rt1, rt2 + 1, rn, imm7, is_load, false).wrapping_sub(base), 1 << 10, "Rt2+1");
        prop_assert_eq!(mk(rt1, rt2, rn + 1, imm7, is_load, false).wrapping_sub(base), 1 << 5, "Rn+1");
        // imm7 = -1 encodes as 0x7F; +1 wraps the 7-bit field to 0. Check the field modulo 128.
        let w_imm = mk(rt1, rt2, rn, imm7 + 1, is_load, false);
        prop_assert_eq!(
            (w_imm >> 15) & 0x7F,
            ((imm7 + 1) as u32) & 0x7F,
            "imm7+1 field"
        );
        prop_assert_eq!(w_imm & !(0x7Fu32 << 15), base & !(0x7Fu32 << 15), "imm7+1 isolation");
        prop_assert_eq!(
            mk(rt1, rt2, rn, imm7, true, false) ^ mk(rt1, rt2, rn, imm7, false, false),
            1 << 22,
            "load XOR store"
        );
        prop_assert_eq!(
            mk(rt1, rt2, rn, imm7, is_load, true) ^ mk(rt1, rt2, rn, imm7, is_load, false),
            0b10 << 23,
            "pre XOR post"
        );
    }

    // Oracle: negative_error — arity 0/1/2 and non-memory third operand
    // Target: encoder.load_store.encode_ldp_stp
    #[test]
    fn encode_ldp_stp_neg_arity(
        is_load in any::<bool>(),
        is_64 in any::<bool>(),
        rt1 in reg_edge(),
        rt2 in reg_edge(),
        kind in bad_address_kind(),
    ) {
        prop_assert!(
            encode_ldp_stp(&[], is_load).is_err(),
            "zero operands must Err: {:?}",
            encode_ldp_stp(&[], is_load)
        );
        let one = [Operand::Reg(gp_rt(is_64, rt1))];
        prop_assert!(
            encode_ldp_stp(&one, is_load).is_err(),
            "one operand must Err: {:?}",
            encode_ldp_stp(&one, is_load)
        );
        let two = [
            Operand::Reg(gp_rt(is_64, rt1)),
            Operand::Reg(gp_rt(is_64, rt2)),
        ];
        prop_assert!(
            encode_ldp_stp(&two, is_load).is_err(),
            "two operands must Err: {:?}",
            encode_ldp_stp(&two, is_load)
        );
        let bad = [
            Operand::Reg(gp_rt(is_64, rt1)),
            Operand::Reg(gp_rt(is_64, rt2)),
            kind.clone(),
        ];
        prop_assert!(
            encode_ldp_stp(&bad, is_load).is_err(),
            "non-memory address operand must Err, got {:?} for {:?}",
            encode_ldp_stp(&bad, is_load),
            kind
        );
    }

    // Oracle: negative_error — extra (fourth) operand
    // Target: encoder.load_store.encode_ldp_stp
    #[test]
    fn encode_ldp_stp_neg_extra_operand(
        is_load in any::<bool>(),
        is_64 in any::<bool>(),
        rt1 in reg_edge(),
        rt2 in reg_edge(),
        rn in reg_edge(),
        extra in extra_operand(),
    ) {
        let four = vec![
            Operand::Reg(gp_rt(is_64, rt1)),
            Operand::Reg(gp_rt(is_64, rt2)),
            Operand::Mem {
                base: rn_name(rn),
                offset: 0,
            },
            extra.clone(),
        ];
        prop_assert!(
            encode_ldp_stp(&four, is_load).is_err(),
            "extra operand must Err (llvm-mc rejects a fourth operand); got {:?}",
            encode_ldp_stp(&four, is_load)
        );
    }

    // Oracle: negative_error — SP dest, XZR/W base, mixed width, LDP Rt1==Rt2, writeback overlap
    // Target: encoder.load_store.encode_ldp_stp
    #[test]
    fn encode_ldp_stp_neg_invalid_regs(
        is_load in any::<bool>(),
        is_64 in any::<bool>(),
        rt in 0u32..=30,
        rt2 in 0u32..=30,
        rn in 0u32..=30,
    ) {
        let mut bugs: Vec<String> = Vec::new();
        let mem_ok = Operand::Mem {
            base: rn_name(rn),
            offset: 0,
        };
        let sp_rt1 = [
            Operand::Reg("sp".into()),
            Operand::Reg(gp_rt(is_64, rt2)),
            mem_ok.clone(),
        ];
        if encode_ldp_stp(&sp_rt1, is_load).is_ok() {
            bugs.push("SP as Rt1".into());
        }
        let sp_rt2 = [
            Operand::Reg(gp_rt(is_64, rt)),
            Operand::Reg("sp".into()),
            mem_ok.clone(),
        ];
        if encode_ldp_stp(&sp_rt2, is_load).is_ok() {
            bugs.push("SP as Rt2".into());
        }
        let xzr_base = [
            Operand::Reg(gp_rt(is_64, rt)),
            Operand::Reg(gp_rt(is_64, rt2)),
            Operand::Mem {
                base: "xzr".into(),
                offset: 0,
            },
        ];
        if encode_ldp_stp(&xzr_base, is_load).is_ok() {
            bugs.push("XZR base".into());
        }
        let x31_base = [
            Operand::Reg(gp_rt(is_64, rt)),
            Operand::Reg(gp_rt(is_64, rt2)),
            Operand::Mem {
                base: "x31".into(),
                offset: 0,
            },
        ];
        if encode_ldp_stp(&x31_base, is_load).is_ok() {
            bugs.push("x31 base".into());
        }
        let w_base = [
            Operand::Reg(gp_rt(is_64, rt)),
            Operand::Reg(gp_rt(is_64, rt2)),
            Operand::Mem {
                base: wt_name(rn),
                offset: 0,
            },
        ];
        if encode_ldp_stp(&w_base, is_load).is_ok() {
            bugs.push("W base".into());
        }
        let mixed = [
            Operand::Reg(xt_name(rt)),
            Operand::Reg(wt_name(rt2)),
            mem_ok.clone(),
        ];
        if encode_ldp_stp(&mixed, is_load).is_ok() {
            bugs.push("mixed X/W pair".into());
        }
        if is_load {
            let same = [
                Operand::Reg(gp_rt(is_64, rt)),
                Operand::Reg(gp_rt(is_64, rt)),
                mem_ok.clone(),
            ];
            if encode_ldp_stp(&same, true).is_ok() {
                bugs.push("LDP Rt1==Rt2".into());
            }
        }
        let overlap_pre = [
            Operand::Reg(gp_rt(is_64, rt)),
            Operand::Reg(gp_rt(is_64, rt2)),
            Operand::MemPreIndex {
                base: format!("x{rt}"),
                offset: scale_of(is_64),
            },
        ];
        if encode_ldp_stp(&overlap_pre, is_load).is_ok() {
            bugs.push("writeback Rn==Rt1".into());
        }
        if rt2 != rt {
            let overlap_rt2 = [
                Operand::Reg(gp_rt(is_64, rt)),
                Operand::Reg(gp_rt(is_64, rt2)),
                Operand::MemPostIndex {
                    base: format!("x{rt2}"),
                    offset: scale_of(is_64),
                },
            ];
            if encode_ldp_stp(&overlap_rt2, is_load).is_ok() {
                bugs.push("writeback Rn==Rt2".into());
            }
        }
        prop_assert!(
            bugs.is_empty(),
            "accepted invalid register forms (llvm-mc rejects): {:?}",
            bugs
        );
    }

    // Oracle: negative_error — out-of-range / unaligned offset
    // Target: encoder.load_store.encode_ldp_stp
    #[test]
    fn encode_ldp_stp_neg_offset_range(
        is_load in any::<bool>(),
        is_64 in any::<bool>(),
        rt1 in reg_edge(),
        rt2 in reg_edge(),
        rn in reg_edge(),
        which_off in 0u32..=4,
        form in 0u32..=2,
    ) {
        let off = match which_off {
            0 => off_min(is_64) - 1,
            1 => off_max(is_64) + 1,
            2 => 1i64,
            3 => i64::MIN,
            _ => i64::MAX,
        };
        let base = rn_name(rn);
        let addr = match form {
            0 => Operand::Mem { base, offset: off },
            1 => Operand::MemPreIndex { base, offset: off },
            _ => Operand::MemPostIndex { base, offset: off },
        };
        let ops = [
            Operand::Reg(gp_rt(is_64, rt1)),
            Operand::Reg(gp_rt(is_64, rt2)),
            addr,
        ];
        prop_assert!(
            encode_ldp_stp(&ops, is_load).is_err(),
            "out-of-range/unaligned offset {} form {} must Err (llvm-mc range); got {:?}",
            off,
            form,
            encode_ldp_stp(&ops, is_load)
        );
    }

    // Oracle: differential — llvm-mc SIMD S/D/Q pair (sweep)
    // Target: encoder.load_store.encode_ldp_stp
    #[test]
    fn encode_ldp_stp_diff_simd_llvm_mc(
        is_load in any::<bool>(),
        kind in 0u32..=2,
        rt1 in reg_edge(),
        rt2 in reg_edge(),
        rn in reg_edge(),
        imm7 in imm7_edge(),
    ) {
        prop_assume!(!is_load || rt1 != rt2);
        let (pfx, scale) = match kind {
            0 => ('s', 4i64),
            1 => ('d', 8i64),
            _ => ('q', 16i64),
        };
        let dest1 = format!("{pfx}{rt1}");
        let dest2 = format!("{pfx}{rt2}");
        let base = rn_name(rn);
        let offset = (imm7 as i64) * scale;
        let asm = format!(
            "{} {dest1}, {dest2}, {}",
            mnemonic(is_load),
            asm_mem(&base, offset)
        );
        let ops = [
            Operand::Reg(dest1),
            Operand::Reg(dest2),
            Operand::Mem { base, offset },
        ];
        let mc = llvm_mc_word(&asm).expect("llvm-mc");
        let sut = sut_word(&ops, is_load).expect("SUT");
        prop_assert_eq!(sut, mc, "SIMD mismatch for {}", asm);
    }

    // Oracle: differential — llvm-mc alternate spellings (uppercase / lr / w31)
    // Target: encoder.load_store.encode_ldp_stp
    #[test]
    fn encode_ldp_stp_diff_alt_spellings(
        is_load in any::<bool>(),
        which in 0u32..=3,
    ) {
        let mnem = mnemonic(is_load);
        let (ops, asm) = match which {
            0 => {
                let ops = [
                    Operand::Reg("X0".into()),
                    Operand::Reg("X1".into()),
                    Operand::Mem {
                        base: "X2".into(),
                        offset: 8,
                    },
                ];
                (ops, format!("{mnem} x0, x1, [x2, #8]"))
            }
            1 => {
                let ops = [
                    Operand::Reg("wzr".into()),
                    Operand::Reg("W30".into()),
                    Operand::Mem {
                        base: "SP".into(),
                        offset: 0,
                    },
                ];
                (ops, format!("{mnem} wzr, w30, [sp]"))
            }
            2 => {
                let ops = [
                    Operand::Reg("x0".into()),
                    Operand::Reg("x1".into()),
                    Operand::Mem {
                        base: "lr".into(),
                        offset: 0,
                    },
                ];
                (ops, format!("{mnem} x0, x1, [x30]"))
            }
            _ => {
                let ops = [
                    Operand::Reg("w31".into()),
                    Operand::Reg("w0".into()),
                    Operand::Mem {
                        base: "sp".into(),
                        offset: 4,
                    },
                ];
                (ops, format!("{mnem} wzr, w0, [sp, #4]"))
            }
        };
        let mc = llvm_mc_word(&asm).expect("llvm-mc");
        let sut = sut_word(&ops, is_load).expect("SUT");
        prop_assert_eq!(sut, mc, "alt-spelling mismatch for {}", asm);
    }
}
