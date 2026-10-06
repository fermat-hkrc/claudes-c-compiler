// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md "accepts the same textual assembly that GCC's gas would consume";
//   encoder/mod.rs:1-7 32-bit AArch64 words; encoder/mod.rs:1050-1061 ldadd*|ldclr*|ldeor*|ldset* => encode_ldop;
//   ARM ARM LDADD: size 111000 A R 1 Rs 0 opc 00 Rn Rt; size 00=byte 01=half 10=word 11=doubleword;
//   opc: LDADD=000, LDCLR=001, LDEOR=010, LDSET=011;
//   A=acquire (LDADDA/LDADDAL), R=release (LDADDL/LDADDAL); Rs/Rt are ZR not SP; Rn is Xn|SP not ZR;
//   LDADDB/LDADDH require W registers; optional offset only #0.
// Stronger considered:
//   - State machine: rejected — encode_ldop is a pure function with no lifecycle
//   - Algebraic round-trip via in-tree decoder: rejected — no LDADD decoder
//   - encode_cas as differential sibling: rejected — different LSE class (size 001000, Rt2=11111)
//   - encode_swp as differential sibling: rejected — o3=1 opc=00000 (swap, not load-op)
//   - encode_stop as differential sibling: rejected — 2-operand Rt=ZR alias
// Weaker available: algebraic.invariant (ARM field unpack), algebraic.metamorphic (Rs/Rt/Rn/A/R/opc),
//   negative_error (arity / extra / SP-as-RsRt / XZR-as-base / W-base / mixed / FP / ldaddb-X / offset)
// Differential: candidate=encode_ldop, reference=llvm-mc -triple=aarch64 -mattr=+lse -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[Reg(Rs), Reg(Rt), Mem{Xn|SP, 0}] <-> `ldadd*|ldclr*|ldeor*|ldset* Rs, Rt, [Xn|SP]`

use super::load_store::encode_ldop;
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

fn op_name(op: u32) -> &'static str {
    match op {
        0 => "ldadd",
        1 => "ldclr",
        2 => "ldeor",
        3 => "ldset",
        _ => "ldadd",
    }
}

fn suffix_str(suf: u32) -> &'static str {
    match suf {
        0 => "",
        1 => "a",
        2 => "al",
        3 => "l",
        4 => "b",
        5 => "ab",
        6 => "alb",
        7 => "lb",
        8 => "h",
        9 => "ah",
        10 => "alh",
        11 => "lh",
        _ => "",
    }
}

fn mnemonic(op: u32, suf: u32) -> String {
    format!("{}{}", op_name(op), suffix_str(suf))
}

fn is_byte(suf: u32) -> bool {
    (4..8).contains(&suf)
}
fn is_half(suf: u32) -> bool {
    (8..12).contains(&suf)
}
fn data_is_64(suf: u32, is_64: bool) -> bool {
    if is_byte(suf) || is_half(suf) {
        false
    } else {
        is_64
    }
}
fn expected_size(suf: u32, wide: bool) -> u32 {
    if is_byte(suf) {
        0b00
    } else if is_half(suf) {
        0b01
    } else if wide {
        0b11
    } else {
        0b10
    }
}
fn expected_a(suf: u32) -> u32 {
    match suf {
        1 | 2 | 5 | 6 | 9 | 10 => 1,
        _ => 0,
    }
}
fn expected_r(suf: u32) -> u32 {
    match suf {
        2 | 3 | 6 | 7 | 10 | 11 => 1,
        _ => 0,
    }
}
fn expected_opc(op: u32) -> u32 {
    op & 0b111
}

fn gpr(n: u32, is_64: bool) -> String {
    if n == 31 {
        if is_64 {
            "xzr".into()
        } else {
            "wzr".into()
        }
    } else if is_64 {
        format!("x{}", n)
    } else {
        format!("w{}", n)
    }
}

fn base(n: u32) -> String {
    if n == 31 {
        "sp".into()
    } else {
        format!("x{}", n)
    }
}

fn valid_ops(suf: u32, rs: u32, rt: u32, rn: u32, is_64: bool) -> Vec<Operand> {
    let wide = data_is_64(suf, is_64);
    vec![
        Operand::Reg(gpr(rs, wide)),
        Operand::Reg(gpr(rt, wide)),
        Operand::Mem {
            base: base(rn),
            offset: 0,
        },
    ]
}

fn sut_word(mnemonic: &str, ops: &[Operand]) -> Result<u32, String> {
    match encode_ldop(mnemonic, ops)? {
        EncodeResult::Word(w) => Ok(w),
        other => Err(format!("expected Word, got {:?}", other)),
    }
}

/// Unpack LDADD fields per ARM ARM (not a copy of the SUT packer).
fn unpack_ldop(word: u32) -> (u32, u32, u32, u32, u32, u32, u32, u32, u32) {
    let size = (word >> 30) & 0b11;
    let a = (word >> 23) & 1;
    let r = (word >> 22) & 1;
    let rs = (word >> 16) & 0x1f;
    let o3 = (word >> 15) & 1;
    let opc = (word >> 12) & 0b111;
    let lo = (word >> 10) & 0b11;
    let rn = (word >> 5) & 0x1f;
    let rt = word & 0x1f;
    (size, a, r, rs, o3, opc, lo, rn, rt)
}

fn fixed_ldop_bits(word: u32) -> bool {
    ((word >> 24) & 0x3f) == 0b111000
        && ((word >> 21) & 1) == 1
        && ((word >> 15) & 1) == 0
        && ((word >> 10) & 0b11) == 0
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
        .args(["-triple=aarch64", "-mattr=+lse", "-show-encoding"])
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

fn reg_edge() -> impl Strategy<Value = u32> {
    prop_oneof![Just(0u32), Just(1u32), Just(30u32), Just(31u32), 0u32..=31]
}

fn op_strat() -> impl Strategy<Value = u32> {
    prop_oneof![Just(0u32), Just(1u32), Just(2u32), Just(3u32)]
}

fn suf_strat() -> impl Strategy<Value = u32> {
    prop_oneof![Just(0u32), Just(4u32), Just(8u32), 0u32..=11]
}

fn extra_operand() -> impl Strategy<Value = Operand> {
    prop_oneof![
        Just(Operand::Reg("x2".into())),
        Just(Operand::Imm(0)),
        Just(Operand::Imm(1)),
        Just(Operand::Symbol("foo".into())),
        Just(Operand::Mem {
            base: "x3".into(),
            offset: 0
        }),
    ]
}

fn nonzero_offset() -> impl Strategy<Value = i64> {
    prop_oneof![
        Just(-1i64),
        Just(1i64),
        Just(-8i64),
        Just(8i64),
        Just(i64::MIN),
        Just(i64::MAX),
        (-4096i64..=4096).prop_filter("nonzero", |x: &i64| *x != 0),
    ]
}

fn fp_kind() -> impl Strategy<Value = char> {
    prop_oneof![
        Just('b'),
        Just('h'),
        Just('s'),
        Just('d'),
        Just('q'),
        Just('v')
    ]
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_ldop_kat_llvm_mc_ldadd_x0_x1_x2() {
    let want = 0xF8200041u32;
    let mc = llvm_mc_word("ldadd x0, x1, [x2]").expect("llvm-mc KAT ldadd x");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = valid_ops(0, 0, 1, 2, true);
    let sut = sut_word("ldadd", &ops).expect("SUT KAT ldadd x");
    assert_eq!(sut, want);
}

#[test]
fn encode_ldop_kat_llvm_mc_ldadd_w0_w1_x2() {
    let want = 0xB8200041u32;
    let mc = llvm_mc_word("ldadd w0, w1, [x2]").expect("llvm-mc KAT ldadd w");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = valid_ops(0, 0, 1, 2, false);
    let sut = sut_word("ldadd", &ops).expect("SUT KAT ldadd w");
    assert_eq!(sut, want);
}

#[test]
fn encode_ldop_kat_llvm_mc_ldadda_w0_w1_x2() {
    let want = 0xB8A00041u32;
    let mc = llvm_mc_word("ldadda w0, w1, [x2]").expect("llvm-mc KAT ldadda");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = valid_ops(1, 0, 1, 2, false);
    let sut = sut_word("ldadda", &ops).expect("SUT KAT ldadda");
    assert_eq!(sut, want);
}

#[test]
fn encode_ldop_kat_llvm_mc_ldaddl_w0_w1_x2() {
    let want = 0xB8600041u32;
    let mc = llvm_mc_word("ldaddl w0, w1, [x2]").expect("llvm-mc KAT ldaddl");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = valid_ops(3, 0, 1, 2, false);
    let sut = sut_word("ldaddl", &ops).expect("SUT KAT ldaddl");
    assert_eq!(sut, want);
}

#[test]
fn encode_ldop_kat_llvm_mc_ldaddal_w0_w1_x2() {
    let want = 0xB8E00041u32;
    let mc = llvm_mc_word("ldaddal w0, w1, [x2]").expect("llvm-mc KAT ldaddal");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = valid_ops(2, 0, 1, 2, false);
    let sut = sut_word("ldaddal", &ops).expect("SUT KAT ldaddal");
    assert_eq!(sut, want);
}

#[test]
fn encode_ldop_kat_llvm_mc_ldaddb_w0_w1_x2() {
    let want = 0x38200041u32;
    let mc = llvm_mc_word("ldaddb w0, w1, [x2]").expect("llvm-mc KAT ldaddb");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = valid_ops(4, 0, 1, 2, false);
    let sut = sut_word("ldaddb", &ops).expect("SUT KAT ldaddb");
    assert_eq!(sut, want);
}

#[test]
fn encode_ldop_kat_llvm_mc_ldaddh_w0_w1_x2() {
    let want = 0x78200041u32;
    let mc = llvm_mc_word("ldaddh w0, w1, [x2]").expect("llvm-mc KAT ldaddh");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = valid_ops(8, 0, 1, 2, false);
    let sut = sut_word("ldaddh", &ops).expect("SUT KAT ldaddh");
    assert_eq!(sut, want);
}

#[test]
fn encode_ldop_kat_llvm_mc_ldclr_w0_w1_x2() {
    let want = 0xB8201041u32;
    let mc = llvm_mc_word("ldclr w0, w1, [x2]").expect("llvm-mc KAT ldclr");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = valid_ops(0, 0, 1, 2, false);
    let sut = sut_word("ldclr", &ops).expect("SUT KAT ldclr");
    assert_eq!(sut, want);
}

#[test]
fn encode_ldop_kat_llvm_mc_ldeor_w0_w1_x2() {
    let want = 0xB8202041u32;
    let mc = llvm_mc_word("ldeor w0, w1, [x2]").expect("llvm-mc KAT ldeor");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = valid_ops(0, 0, 1, 2, false);
    let sut = sut_word("ldeor", &ops).expect("SUT KAT ldeor");
    assert_eq!(sut, want);
}

#[test]
fn encode_ldop_kat_llvm_mc_ldset_w0_w1_x2() {
    let want = 0xB8203041u32;
    let mc = llvm_mc_word("ldset w0, w1, [x2]").expect("llvm-mc KAT ldset");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = valid_ops(0, 0, 1, 2, false);
    let sut = sut_word("ldset", &ops).expect("SUT KAT ldset");
    assert_eq!(sut, want);
}

#[test]
fn encode_ldop_kat_llvm_mc_ldadd_xzr_xzr_sp() {
    let want = 0xF83F03FFu32;
    let mc = llvm_mc_word("ldadd xzr, xzr, [sp]").expect("llvm-mc KAT zr/sp");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = valid_ops(0, 31, 31, 31, true);
    let sut = sut_word("ldadd", &ops).expect("SUT KAT zr/sp");
    assert_eq!(sut, want);
}

proptest! {
    #![proptest_config(cfg())]

    // Oracle: differential
    // Target: encoder.load_store.encode_ldop
    #[test]
    fn encode_ldop_diff_llvm_mc(
        op in op_strat(),
        suf in suf_strat(),
        rs in reg_edge(),
        rt in reg_edge(),
        rn in reg_edge(),
        is_64 in any::<bool>(),
    ) {
        let wide = data_is_64(suf, is_64);
        let mnem = mnemonic(op, suf);
        let asm = format!("{} {}, {}, [{}]", mnem, gpr(rs, wide), gpr(rt, wide), base(rn));
        let ops = valid_ops(suf, rs, rt, rn, is_64);
        let sut = sut_word(&mnem, &ops)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
    }

    // Oracle: algebraic.invariant
    // Target: encoder.load_store.encode_ldop
    #[test]
    fn encode_ldop_arm_fields(
        op in op_strat(),
        suf in suf_strat(),
        rs in reg_edge(),
        rt in reg_edge(),
        rn in reg_edge(),
        is_64 in any::<bool>(),
    ) {
        let wide = data_is_64(suf, is_64);
        let mnem = mnemonic(op, suf);
        let ops = valid_ops(suf, rs, rt, rn, is_64);
        let word = sut_word(&mnem, &ops)
            .unwrap_or_else(|e| panic!("SUT rejected valid LDOP: {}", e));
        let (size, a, r, got_rs, o3, opc, lo, got_rn, got_rt) = unpack_ldop(word);
        prop_assert_eq!(size, expected_size(suf, wide), "size mismatch word={:#010x}", word);
        prop_assert_eq!(a, expected_a(suf), "A bit mismatch word={:#010x}", word);
        prop_assert_eq!(r, expected_r(suf), "R bit mismatch word={:#010x}", word);
        prop_assert_eq!(got_rs, rs, "Rs field mismatch word={:#010x}", word);
        prop_assert_eq!(got_rt, rt, "Rt field mismatch word={:#010x}", word);
        prop_assert_eq!(got_rn, rn, "Rn field mismatch word={:#010x}", word);
        prop_assert_eq!(o3, 0, "o3 must be 0 word={:#010x}", word);
        prop_assert_eq!(opc, expected_opc(op), "opc mismatch word={:#010x}", word);
        prop_assert_eq!(lo, 0, "bits[11:10] must be 00 word={:#010x}", word);
        prop_assert!(fixed_ldop_bits(word), "fixed LDOP bits violated word={:#010x}", word);
    }

    // Oracle: algebraic.metamorphic
    // Target: encoder.load_store.encode_ldop
    #[test]
    fn encode_ldop_metamorphic_regs_ar_opc(
        rs in 0u32..=30,
        rt in 0u32..=30,
        rn in 0u32..=30,
        is_64 in any::<bool>(),
    ) {
        let base_ops = valid_ops(0, rs, rt, rn, is_64);
        let base_w = sut_word("ldadd", &base_ops)
            .unwrap_or_else(|e| panic!("SUT rejected ldadd: {}", e));
        let rt1 = sut_word("ldadd", &valid_ops(0, rs, rt + 1, rn, is_64))
            .unwrap_or_else(|e| panic!("SUT rejected rt+1: {}", e));
        prop_assert_eq!(rt1 & !0x1fu32, base_w & !0x1fu32, "Rt+1 must change only bits[4:0]");
        prop_assert_eq!(rt1 & 0x1f, rt + 1);
        let rn1 = sut_word("ldadd", &valid_ops(0, rs, rt, rn + 1, is_64))
            .unwrap_or_else(|e| panic!("SUT rejected rn+1: {}", e));
        prop_assert_eq!(rn1 & !(0x1fu32 << 5), base_w & !(0x1fu32 << 5), "Rn+1 must change only bits[9:5]");
        prop_assert_eq!((rn1 >> 5) & 0x1f, rn + 1);
        let rs1 = sut_word("ldadd", &valid_ops(0, rs + 1, rt, rn, is_64))
            .unwrap_or_else(|e| panic!("SUT rejected rs+1: {}", e));
        prop_assert_eq!(rs1 & !(0x1fu32 << 16), base_w & !(0x1fu32 << 16), "Rs+1 must change only bits[20:16]");
        prop_assert_eq!((rs1 >> 16) & 0x1f, rs + 1);
        let lda_w = sut_word("ldadda", &base_ops)
            .unwrap_or_else(|e| panic!("SUT rejected ldadda: {}", e));
        prop_assert_eq!(lda_w ^ base_w, 1u32 << 23, "LDADDA vs LDADD must differ only by A at bit 23");
        let ldl_w = sut_word("ldaddl", &base_ops)
            .unwrap_or_else(|e| panic!("SUT rejected ldaddl: {}", e));
        prop_assert_eq!(ldl_w ^ base_w, 1u32 << 22, "LDADDL vs LDADD must differ only by R at bit 22");
        let ldal_w = sut_word("ldaddal", &base_ops)
            .unwrap_or_else(|e| panic!("SUT rejected ldaddal: {}", e));
        prop_assert_eq!(ldal_w ^ base_w, (1u32 << 23) | (1u32 << 22), "LDADDAL vs LDADD must flip A and R");
        let clr_w = sut_word("ldclr", &base_ops)
            .unwrap_or_else(|e| panic!("SUT rejected ldclr: {}", e));
        prop_assert_eq!(clr_w ^ base_w, 1u32 << 12, "LDCLR vs LDADD must differ only by opc at bits[14:12]");
        let eor_w = sut_word("ldeor", &base_ops)
            .unwrap_or_else(|e| panic!("SUT rejected ldeor: {}", e));
        prop_assert_eq!(eor_w ^ base_w, 2u32 << 12, "LDEOR vs LDADD must differ only by opc at bits[14:12]");
        let set_w = sut_word("ldset", &base_ops)
            .unwrap_or_else(|e| panic!("SUT rejected ldset: {}", e));
        prop_assert_eq!(set_w ^ base_w, 3u32 << 12, "LDSET vs LDADD must differ only by opc at bits[14:12]");
        let upper = sut_word("LDADD", &base_ops)
            .unwrap_or_else(|e| panic!("SUT rejected uppercase LDADD: {}", e));
        prop_assert_eq!(upper, base_w, "uppercase mnemonic must match lowercase");
    }

    // Oracle: negative_error
    // Target: encoder.load_store.encode_ldop
    // Invalid domain: a fourth operand. llvm-mc/gas reject extra operands.
    #[test]
    fn encode_ldop_neg_extra_operand(
        op in op_strat(),
        suf in suf_strat(),
        rs in reg_edge(),
        rt in reg_edge(),
        rn in reg_edge(),
        is_64 in any::<bool>(),
        extra in extra_operand(),
    ) {
        let mnem = mnemonic(op, suf);
        let mut ops = valid_ops(suf, rs, rt, rn, is_64);
        ops.push(extra);
        prop_assert!(
            encode_ldop(&mnem, &ops).is_err(),
            "extra operand must Err; llvm-mc/gas reject a 4th LDOP operand"
        );
    }

    // Oracle: negative_error
    // Target: encoder.load_store.encode_ldop
    // Invalid domain: SP/WSP as Rs/Rt; W/WSP/XZR/x31 as base.
    #[test]
    fn encode_ldop_neg_sp_zr_base(
        op in op_strat(),
        suf in suf_strat(),
        n in 0u32..=30,
        kind in 0u32..=8,
        is_64 in any::<bool>(),
    ) {
        let wide = data_is_64(suf, is_64);
        let mnem = mnemonic(op, suf);
        let rs = gpr(n, wide);
        let rt = gpr((n + 1) % 31, wide);
        let rn = base((n + 2) % 31);
        let ops: Vec<Operand> = match kind {
            0 => vec![
                Operand::Reg("sp".into()),
                Operand::Reg(rt.clone()),
                Operand::Mem { base: rn.clone(), offset: 0 },
            ],
            1 => vec![
                Operand::Reg(rs.clone()),
                Operand::Reg("sp".into()),
                Operand::Mem { base: rn.clone(), offset: 0 },
            ],
            2 => vec![
                Operand::Reg("wsp".into()),
                Operand::Reg(gpr((n + 1) % 31, false)),
                Operand::Mem { base: rn.clone(), offset: 0 },
            ],
            3 => vec![
                Operand::Reg(gpr(n, false)),
                Operand::Reg("wsp".into()),
                Operand::Mem { base: rn.clone(), offset: 0 },
            ],
            4 => vec![
                Operand::Reg(rs.clone()),
                Operand::Reg(rt.clone()),
                Operand::Mem { base: format!("w{}", n), offset: 0 },
            ],
            5 => vec![
                Operand::Reg(rs.clone()),
                Operand::Reg(rt.clone()),
                Operand::Mem { base: "wsp".into(), offset: 0 },
            ],
            6 => vec![
                Operand::Reg(rs.clone()),
                Operand::Reg(rt.clone()),
                Operand::Mem { base: "xzr".into(), offset: 0 },
            ],
            7 => vec![
                Operand::Reg(rs.clone()),
                Operand::Reg(rt.clone()),
                Operand::Mem { base: "x31".into(), offset: 0 },
            ],
            _ => vec![
                Operand::Reg(rs),
                Operand::Reg(rt),
                Operand::Mem { base: "wzr".into(), offset: 0 },
            ],
        };
        prop_assert!(
            encode_ldop(&mnem, &ops).is_err(),
            "SP/WSP as Rs/Rt or W/WSP/XZR/x31 as base must Err; llvm-mc/gas reject"
        );
    }

    // Oracle: negative_error
    // Target: encoder.load_store.encode_ldop
    // Invalid domain: mixed W/X, FP/SIMD prefixes, ldaddb/ldaddh with X registers.
    #[test]
    fn encode_ldop_neg_mixed_fp_xbyte(
        n in 0u32..=30,
        kind in 0u32..=8,
        fp in fp_kind(),
    ) {
        let (mnem, ops): (&str, Vec<Operand>) = match kind {
            0 => (
                "ldadd",
                vec![
                    Operand::Reg(format!("x{}", n)),
                    Operand::Reg(format!("w{}", n)),
                    Operand::Mem { base: format!("x{}", (n + 1) % 31), offset: 0 },
                ],
            ),
            1 => (
                "ldadd",
                vec![
                    Operand::Reg(format!("w{}", n)),
                    Operand::Reg(format!("x{}", n)),
                    Operand::Mem { base: format!("x{}", (n + 1) % 31), offset: 0 },
                ],
            ),
            2 => (
                "ldadd",
                vec![
                    Operand::Reg(format!("{}{}", fp, n)),
                    Operand::Reg(format!("{}{}", fp, (n + 1) % 31)),
                    Operand::Mem { base: format!("x{}", (n + 2) % 31), offset: 0 },
                ],
            ),
            3 => (
                "ldadd",
                vec![
                    Operand::Reg(format!("{}{}", fp, n)),
                    Operand::Reg(format!("x{}", n)),
                    Operand::Mem { base: format!("x{}", (n + 1) % 31), offset: 0 },
                ],
            ),
            4 => (
                "ldaddb",
                vec![
                    Operand::Reg(format!("x{}", n)),
                    Operand::Reg(format!("x{}", (n + 1) % 31)),
                    Operand::Mem { base: format!("x{}", (n + 2) % 31), offset: 0 },
                ],
            ),
            5 => (
                "ldaddh",
                vec![
                    Operand::Reg(format!("x{}", n)),
                    Operand::Reg(format!("x{}", (n + 1) % 31)),
                    Operand::Mem { base: format!("x{}", (n + 2) % 31), offset: 0 },
                ],
            ),
            6 => (
                "ldaddb",
                vec![
                    Operand::Reg(format!("x{}", n)),
                    Operand::Reg(format!("w{}", (n + 1) % 31)),
                    Operand::Mem { base: format!("x{}", (n + 2) % 31), offset: 0 },
                ],
            ),
            7 => (
                "ldaddab",
                vec![
                    Operand::Reg(format!("x{}", n)),
                    Operand::Reg(format!("x{}", (n + 1) % 31)),
                    Operand::Mem { base: format!("x{}", (n + 2) % 31), offset: 0 },
                ],
            ),
            _ => (
                "ldsetlh",
                vec![
                    Operand::Reg(format!("x{}", n)),
                    Operand::Reg(format!("x{}", (n + 1) % 31)),
                    Operand::Mem { base: format!("x{}", (n + 2) % 31), offset: 0 },
                ],
            ),
        };
        prop_assert!(
            encode_ldop(mnem, &ops).is_err(),
            "mixed W/X, FP, or ldaddb/ldaddh with X must Err; llvm-mc/gas reject"
        );
    }

    // Oracle: negative_error
    // Target: encoder.load_store.encode_ldop
    // Invalid domain: fewer than 3 operands, or third operand not a bare Mem.
    #[test]
    fn encode_ldop_neg_arity_and_shape(
        shape in 0u32..=8,
        n in 0u32..=30,
        off in -8i64..=8,
    ) {
        let rs = format!("x{}", n);
        let rt = format!("x{}", (n + 1) % 31);
        let ops: Vec<Operand> = match shape {
            0 => vec![],
            1 => vec![Operand::Reg(rs)],
            2 => vec![Operand::Reg(rs), Operand::Reg(rt)],
            3 => vec![
                Operand::Reg(rs),
                Operand::Reg(rt),
                Operand::Imm(0),
            ],
            4 => vec![
                Operand::Reg(rs),
                Operand::Reg(rt),
                Operand::Symbol("foo".into()),
            ],
            5 => vec![
                Operand::Reg(rs),
                Operand::Reg(rt),
                Operand::MemPreIndex { base: format!("x{}", (n + 2) % 31), offset: off },
            ],
            6 => vec![
                Operand::Reg(rs),
                Operand::Reg(rt),
                Operand::MemPostIndex { base: format!("x{}", (n + 2) % 31), offset: off },
            ],
            7 => vec![
                Operand::Reg(rs),
                Operand::Reg(rt),
                Operand::MemRegOffset {
                    base: format!("x{}", (n + 2) % 31),
                    index: format!("x{}", (n + 3) % 31),
                    extend: None,
                    shift: None,
                },
            ],
            _ => vec![
                Operand::Reg(rs),
                Operand::Reg(rt),
                Operand::Cond("eq".into()),
            ],
        };
        prop_assert!(
            encode_ldop("ldadd", &ops).is_err(),
            "arity < 3 or non-Mem third operand must Err"
        );
    }

    // Oracle: negative_error
    // Target: encoder.load_store.encode_ldop
    // Invalid domain: Mem offset != 0. ARM optional offset is only #0.
    #[test]
    fn encode_ldop_neg_nonzero_offset(
        op in op_strat(),
        suf in suf_strat(),
        rs in reg_edge(),
        rt in reg_edge(),
        rn in 0u32..=30,
        is_64 in any::<bool>(),
        off in nonzero_offset(),
    ) {
        let wide = data_is_64(suf, is_64);
        let mnem = mnemonic(op, suf);
        let ops = vec![
            Operand::Reg(gpr(rs, wide)),
            Operand::Reg(gpr(rt, wide)),
            Operand::Mem { base: base(rn), offset: off },
        ];
        prop_assert!(
            encode_ldop(&mnem, &ops).is_err(),
            "nonzero Mem offset must Err; gas: optional immediate offset can only be 0"
        );
    }

    // Oracle: negative_error (sweep)
    // Target: encoder.load_store.encode_ldop
    // Invalid domain: unparsable register/base names (foo/x32/empty/r0).
    #[test]
    fn encode_ldop_neg_invalid_name(
        slot in 0u32..=2,
        name in prop_oneof![
            Just("foo".to_string()),
            Just("x32".to_string()),
            Just("w32".to_string()),
            Just("".to_string()),
            Just("r0".to_string()),
            Just("x-1".to_string()),
            Just("31".to_string()),
        ],
    ) {
        let mut ops = valid_ops(0, 0, 1, 2, true);
        match slot {
            0 => ops[0] = Operand::Reg(name),
            1 => ops[1] = Operand::Reg(name),
            _ => {
                ops[2] = Operand::Mem {
                    base: name,
                    offset: 0,
                }
            }
        }
        prop_assert!(
            encode_ldop("ldadd", &ops).is_err(),
            "invalid register/base name must Err"
        );
    }

    // Oracle: differential (sweep)
    // Target: encoder.load_store.encode_ldop
    // Documented: mnemonic is lowercased before suffix parse; llvm-mc accepts LDADD/Ldadd/LDADDB.
    #[test]
    fn encode_ldop_diff_alt_spellings(
        op in op_strat(),
        suf in suf_strat(),
        rs in reg_edge(),
        rt in reg_edge(),
        rn in reg_edge(),
        is_64 in any::<bool>(),
        mode in 0u32..=2,
    ) {
        let wide = data_is_64(suf, is_64);
        let mnem = mnemonic(op, suf);
        let cased = match mode {
            0 => mnem.to_uppercase(),
            1 => {
                let mut c: String = mnem.to_string();
                if let Some(first) = c.get_mut(0..1) {
                    first.make_ascii_uppercase();
                }
                c
            }
            _ => mnem.to_string(),
        };
        let asm = format!(
            "{} {}, {}, [{}]",
            cased,
            gpr(rs, wide),
            gpr(rt, wide),
            base(rn)
        );
        let ops = valid_ops(suf, rs, rt, rn, is_64);
        let sut = sut_word(&cased, &ops)
            .unwrap_or_else(|e| panic!("SUT rejected alt spelling {}: {}", asm, e));
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected alt spelling {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for alt spelling {}", asm);
    }

    // Oracle: negative_error (sweep)
    // Target: encoder.load_store.encode_ldop
    // Invalid domain: mnemonic that is not ldadd/ldclr/ldeor/ldset.
    #[test]
    fn encode_ldop_neg_unknown_op(
        name in prop_oneof![
            Just("ldfoo".to_string()),
            Just("swp".to_string()),
            Just("cas".to_string()),
            Just("ld".to_string()),
            Just("add".to_string()),
            Just("".to_string()),
            Just("stadd".to_string()),
        ],
    ) {
        let ops = valid_ops(0, 0, 1, 2, true);
        prop_assert!(
            encode_ldop(&name, &ops).is_err(),
            "unknown ld atomic op must Err"
        );
    }
}

#[test]
fn test_encode_ldop_regression_extra_operand() {
    let ops = [
        Operand::Reg("w0".into()),
        Operand::Reg("w0".into()),
        Operand::Mem {
            base: "x0".into(),
            offset: 0,
        },
        Operand::Reg("x2".into()),
    ];
    assert!(
        encode_ldop("ldadd", &ops).is_err(),
        "ldadd w0, w0, [x0], x2 must Err; llvm-mc/gas reject a 4th operand"
    );
}

#[test]
fn test_encode_ldop_regression_sp_as_rs() {
    let ops = [
        Operand::Reg("sp".into()),
        Operand::Reg("w1".into()),
        Operand::Mem {
            base: "x2".into(),
            offset: 0,
        },
    ];
    assert!(
        encode_ldop("ldadd", &ops).is_err(),
        "ldadd sp, w1, [x2] must Err; llvm-mc/gas reject SP as Rs"
    );
}

#[test]
fn test_encode_ldop_regression_xzr_as_base() {
    let ops = [
        Operand::Reg("x0".into()),
        Operand::Reg("x1".into()),
        Operand::Mem {
            base: "xzr".into(),
            offset: 0,
        },
    ];
    assert!(
        encode_ldop("ldadd", &ops).is_err(),
        "ldadd x0, x1, [xzr] must Err; llvm-mc/gas reject XZR as base (Rn=31 is SP)"
    );
}

#[test]
fn test_encode_ldop_regression_w_base() {
    let ops = [
        Operand::Reg("w0".into()),
        Operand::Reg("w1".into()),
        Operand::Mem {
            base: "w2".into(),
            offset: 0,
        },
    ];
    assert!(
        encode_ldop("ldadd", &ops).is_err(),
        "ldadd w0, w1, [w2] must Err; llvm-mc/gas require Xn|SP as base"
    );
}

#[test]
fn test_encode_ldop_regression_mixed_width() {
    let ops = [
        Operand::Reg("x0".into()),
        Operand::Reg("w0".into()),
        Operand::Mem {
            base: "x1".into(),
            offset: 0,
        },
    ];
    assert!(
        encode_ldop("ldadd", &ops).is_err(),
        "ldadd x0, w0, [x1] must Err; llvm-mc/gas reject mixed W/X"
    );
}

#[test]
fn test_encode_ldop_regression_fp_reg() {
    let ops = [
        Operand::Reg("s0".into()),
        Operand::Reg("s1".into()),
        Operand::Mem {
            base: "x2".into(),
            offset: 0,
        },
    ];
    assert!(
        encode_ldop("ldadd", &ops).is_err(),
        "ldadd s0, s1, [x2] must Err; llvm-mc/gas require integer registers"
    );
}

#[test]
fn test_encode_ldop_regression_ldaddb_x_reg() {
    let ops = [
        Operand::Reg("x0".into()),
        Operand::Reg("x1".into()),
        Operand::Mem {
            base: "x2".into(),
            offset: 0,
        },
    ];
    assert!(
        encode_ldop("ldaddb", &ops).is_err(),
        "ldaddb x0, x1, [x2] must Err; llvm-mc/gas require W registers for LDADDB"
    );
}

#[test]
fn test_encode_ldop_regression_nonzero_offset() {
    let ops = [
        Operand::Reg("w0".into()),
        Operand::Reg("w0".into()),
        Operand::Mem {
            base: "x0".into(),
            offset: -1,
        },
    ];
    assert!(
        encode_ldop("ldadd", &ops).is_err(),
        "ldadd w0, w0, [x0, #-1] must Err; gas: optional immediate offset can only be 0"
    );
}
