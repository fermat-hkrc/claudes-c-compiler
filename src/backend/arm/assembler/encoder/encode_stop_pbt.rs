// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md "accepts the same textual assembly that GCC's gas would consume";
//   encoder/mod.rs:1-7 32-bit AArch64 words; encoder/mod.rs:1065-1068 stadd*|stclr*|steor*|stset* => encode_stop;
//   ARM ARM STADD is the store alias of LDADD with Rt=WZR/XZR:
//   size 111000 A R 1 Rs 0 opc 00 Rn Rt; A=0; Rt=31;
//   size 00=byte 01=half 10=word 11=doubleword;
//   opc: STADD=000, STCLR=001, STEOR=010, STSET=011;
//   R=release (STADDL/STADDLB/STADDLH); Rs is ZR not SP; Rn is Xn|SP not ZR;
//   STADDB/STADDH require W registers; optional offset only #0.
// Stronger considered:
//   - State machine: rejected — encode_stop is a pure function with no lifecycle
//   - Algebraic round-trip via in-tree decoder: rejected — no STADD decoder
//   - encode_cas as differential sibling: rejected — different LSE class (size 001000, Rt2=11111)
//   - encode_swp as differential sibling: rejected — o3=1 opc=00000 (swap, not store-op)
//   - encode_ldop as differential sibling: rejected — 3-operand live-Rt form (same-job gate)
// Weaker available: algebraic.invariant (ARM field unpack, A=0 Rt=31), algebraic.metamorphic (Rs/Rn/R/opc),
//   negative_error (arity / extra / SP-as-Rs / XZR-as-base / W-base / FP / staddb-X / offset)
// Differential: candidate=encode_stop, reference=llvm-mc -triple=aarch64 -mattr=+lse -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[Reg(Rs), Mem{Xn|SP, 0}] <-> `stadd*|stclr*|steor*|stset* Rs, [Xn|SP]`

use super::load_store::encode_stop;
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
        0 => "stadd",
        1 => "stclr",
        2 => "steor",
        3 => "stset",
        _ => "stadd",
    }
}

fn suffix_str(suf: u32) -> &'static str {
    match suf {
        0 => "",
        1 => "l",
        2 => "b",
        3 => "lb",
        4 => "h",
        5 => "lh",
        _ => "",
    }
}

fn mnemonic(op: u32, suf: u32) -> String {
    format!("{}{}", op_name(op), suffix_str(suf))
}

fn is_byte(suf: u32) -> bool {
    suf == 2 || suf == 3
}
fn is_half(suf: u32) -> bool {
    suf == 4 || suf == 5
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
fn expected_r(suf: u32) -> u32 {
    match suf {
        1 | 3 | 5 => 1,
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

fn valid_ops(suf: u32, rs: u32, rn: u32, is_64: bool) -> Vec<Operand> {
    let wide = data_is_64(suf, is_64);
    vec![
        Operand::Reg(gpr(rs, wide)),
        Operand::Mem {
            base: base(rn),
            offset: 0,
        },
    ]
}

fn sut_word(mnemonic: &str, ops: &[Operand]) -> Result<u32, String> {
    match encode_stop(mnemonic, ops)? {
        EncodeResult::Word(w) => Ok(w),
        other => Err(format!("expected Word, got {:?}", other)),
    }
}

/// Unpack LDADD/STADD fields per ARM ARM (not a copy of the SUT packer).
fn unpack_stop(word: u32) -> (u32, u32, u32, u32, u32, u32, u32, u32, u32) {
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

fn fixed_stop_bits(word: u32) -> bool {
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
    prop_oneof![Just(0u32), Just(1u32), Just(2u32), Just(4u32), 0u32..=5]
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
fn encode_stop_kat_llvm_mc_stadd_w0_x1() {
    let want = 0xB820003Fu32;
    let mc = llvm_mc_word("stadd w0, [x1]").expect("llvm-mc KAT stadd w");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = valid_ops(0, 0, 1, false);
    let sut = sut_word("stadd", &ops).expect("SUT KAT stadd w");
    assert_eq!(sut, want);
}

#[test]
fn encode_stop_kat_llvm_mc_stadd_x0_x1() {
    let want = 0xF820003Fu32;
    let mc = llvm_mc_word("stadd x0, [x1]").expect("llvm-mc KAT stadd x");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = valid_ops(0, 0, 1, true);
    let sut = sut_word("stadd", &ops).expect("SUT KAT stadd x");
    assert_eq!(sut, want);
}

#[test]
fn encode_stop_kat_llvm_mc_staddl_w0_x1() {
    let want = 0xB860003Fu32;
    let mc = llvm_mc_word("staddl w0, [x1]").expect("llvm-mc KAT staddl");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = valid_ops(1, 0, 1, false);
    let sut = sut_word("staddl", &ops).expect("SUT KAT staddl");
    assert_eq!(sut, want);
}

#[test]
fn encode_stop_kat_llvm_mc_staddb_w0_x1() {
    let want = 0x3820003Fu32;
    let mc = llvm_mc_word("staddb w0, [x1]").expect("llvm-mc KAT staddb");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = valid_ops(2, 0, 1, false);
    let sut = sut_word("staddb", &ops).expect("SUT KAT staddb");
    assert_eq!(sut, want);
}

#[test]
fn encode_stop_kat_llvm_mc_staddh_w0_x1() {
    let want = 0x7820003Fu32;
    let mc = llvm_mc_word("staddh w0, [x1]").expect("llvm-mc KAT staddh");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = valid_ops(4, 0, 1, false);
    let sut = sut_word("staddh", &ops).expect("SUT KAT staddh");
    assert_eq!(sut, want);
}

#[test]
fn encode_stop_kat_llvm_mc_stclr_w0_x1() {
    let want = 0xB820103Fu32;
    let mc = llvm_mc_word("stclr w0, [x1]").expect("llvm-mc KAT stclr");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = valid_ops(0, 0, 1, false);
    let sut = sut_word("stclr", &ops).expect("SUT KAT stclr");
    assert_eq!(sut, want);
}

#[test]
fn encode_stop_kat_llvm_mc_steor_w0_x1() {
    let want = 0xB820203Fu32;
    let mc = llvm_mc_word("steor w0, [x1]").expect("llvm-mc KAT steor");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = valid_ops(0, 0, 1, false);
    let sut = sut_word("steor", &ops).expect("SUT KAT steor");
    assert_eq!(sut, want);
}

#[test]
fn encode_stop_kat_llvm_mc_stset_w0_x1() {
    let want = 0xB820303Fu32;
    let mc = llvm_mc_word("stset w0, [x1]").expect("llvm-mc KAT stset");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = valid_ops(0, 0, 1, false);
    let sut = sut_word("stset", &ops).expect("SUT KAT stset");
    assert_eq!(sut, want);
}

#[test]
fn encode_stop_kat_llvm_mc_stadd_xzr_sp() {
    let want = 0xF83F03FFu32;
    let mc = llvm_mc_word("stadd xzr, [sp]").expect("llvm-mc KAT zr/sp");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = valid_ops(0, 31, 31, true);
    let sut = sut_word("stadd", &ops).expect("SUT KAT zr/sp");
    assert_eq!(sut, want);
}

proptest! {
    #![proptest_config(cfg())]

    // Oracle: differential
    // Target: encoder.load_store.encode_stop
    #[test]
    fn encode_stop_diff_llvm_mc(
        op in op_strat(),
        suf in suf_strat(),
        rs in reg_edge(),
        rn in reg_edge(),
        is_64 in any::<bool>(),
    ) {
        let wide = data_is_64(suf, is_64);
        let mnem = mnemonic(op, suf);
        let asm = format!("{} {}, [{}]", mnem, gpr(rs, wide), base(rn));
        let ops = valid_ops(suf, rs, rn, is_64);
        let sut = sut_word(&mnem, &ops)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
    }
}

proptest! {
    #![proptest_config(cfg())]

    // Oracle: algebraic.invariant
    // Target: encoder.load_store.encode_stop
    #[test]
    fn encode_stop_arm_fields(
        op in op_strat(),
        suf in suf_strat(),
        rs in reg_edge(),
        rn in reg_edge(),
        is_64 in any::<bool>(),
    ) {
        let wide = data_is_64(suf, is_64);
        let mnem = mnemonic(op, suf);
        let ops = valid_ops(suf, rs, rn, is_64);
        let word = sut_word(&mnem, &ops)
            .unwrap_or_else(|e| panic!("SUT rejected valid STOP: {}", e));
        let (size, a, r, got_rs, o3, opc, lo, got_rn, got_rt) = unpack_stop(word);
        prop_assert_eq!(size, expected_size(suf, wide), "size mismatch word={:#010x}", word);
        prop_assert_eq!(a, 0, "A must be 0 for store alias word={:#010x}", word);
        prop_assert_eq!(r, expected_r(suf), "R bit mismatch word={:#010x}", word);
        prop_assert_eq!(got_rs, rs, "Rs field mismatch word={:#010x}", word);
        prop_assert_eq!(got_rt, 31, "Rt must be XZR/WZR word={:#010x}", word);
        prop_assert_eq!(got_rn, rn, "Rn field mismatch word={:#010x}", word);
        prop_assert_eq!(o3, 0, "o3 must be 0 word={:#010x}", word);
        prop_assert_eq!(opc, expected_opc(op), "opc mismatch word={:#010x}", word);
        prop_assert_eq!(lo, 0, "bits[11:10] must be 00 word={:#010x}", word);
        prop_assert!(fixed_stop_bits(word), "fixed STOP bits violated word={:#010x}", word);
    }

    // Oracle: algebraic.metamorphic
    // Target: encoder.load_store.encode_stop
    #[test]
    fn encode_stop_metamorphic_regs_r_opc(
        rs in 0u32..=30,
        rn in 0u32..=30,
        is_64 in any::<bool>(),
    ) {
        let base_ops = valid_ops(0, rs, rn, is_64);
        let base_w = sut_word("stadd", &base_ops)
            .unwrap_or_else(|e| panic!("SUT rejected stadd: {}", e));
        let rn1 = sut_word("stadd", &valid_ops(0, rs, rn + 1, is_64))
            .unwrap_or_else(|e| panic!("SUT rejected rn+1: {}", e));
        prop_assert_eq!(rn1 & !(0x1fu32 << 5), base_w & !(0x1fu32 << 5), "Rn+1 must change only bits[9:5]");
        prop_assert_eq!((rn1 >> 5) & 0x1f, rn + 1);
        let rs1 = sut_word("stadd", &valid_ops(0, rs + 1, rn, is_64))
            .unwrap_or_else(|e| panic!("SUT rejected rs+1: {}", e));
        prop_assert_eq!(rs1 & !(0x1fu32 << 16), base_w & !(0x1fu32 << 16), "Rs+1 must change only bits[20:16]");
        prop_assert_eq!((rs1 >> 16) & 0x1f, rs + 1);
        prop_assert_eq!(base_w & 0x1f, 31, "Rt is always XZR");
        let l_w = sut_word("staddl", &base_ops)
            .unwrap_or_else(|e| panic!("SUT rejected staddl: {}", e));
        prop_assert_eq!(l_w ^ base_w, 1u32 << 22, "STADDL vs STADD must differ only by R at bit 22");
        let clr_w = sut_word("stclr", &base_ops)
            .unwrap_or_else(|e| panic!("SUT rejected stclr: {}", e));
        prop_assert_eq!(clr_w ^ base_w, 1u32 << 12, "STCLR vs STADD must differ only by opc at bits[14:12]");
        let eor_w = sut_word("steor", &base_ops)
            .unwrap_or_else(|e| panic!("SUT rejected steor: {}", e));
        prop_assert_eq!(eor_w ^ base_w, 2u32 << 12, "STEOR vs STADD must differ only by opc at bits[14:12]");
        let set_w = sut_word("stset", &base_ops)
            .unwrap_or_else(|e| panic!("SUT rejected stset: {}", e));
        prop_assert_eq!(set_w ^ base_w, 3u32 << 12, "STSET vs STADD must differ only by opc at bits[14:12]");
        let upper = sut_word("STADD", &base_ops)
            .unwrap_or_else(|e| panic!("SUT rejected uppercase STADD: {}", e));
        prop_assert_eq!(upper, base_w, "uppercase mnemonic must match lowercase");
    }

    // Oracle: negative_error
    // Target: encoder.load_store.encode_stop
    // Invalid domain: a third operand. llvm-mc/gas reject extra operands.
    #[test]
    fn encode_stop_neg_extra_operand(
        op in op_strat(),
        suf in suf_strat(),
        rs in reg_edge(),
        rn in reg_edge(),
        is_64 in any::<bool>(),
        extra in extra_operand(),
    ) {
        let mnem = mnemonic(op, suf);
        let mut ops = valid_ops(suf, rs, rn, is_64);
        ops.push(extra);
        prop_assert!(
            encode_stop(&mnem, &ops).is_err(),
            "extra operand must Err; llvm-mc/gas reject a 3rd STOP operand"
        );
    }

    // Oracle: negative_error
    // Target: encoder.load_store.encode_stop
    // Invalid domain: SP/WSP as Rs; W/WSP/XZR/x31 as base.
    #[test]
    fn encode_stop_neg_sp_zr_base(
        op in op_strat(),
        suf in suf_strat(),
        n in 0u32..=30,
        kind in 0u32..=6,
        is_64 in any::<bool>(),
    ) {
        let wide = data_is_64(suf, is_64);
        let mnem = mnemonic(op, suf);
        let rs = gpr(n, wide);
        let rn = base((n + 1) % 31);
        let ops: Vec<Operand> = match kind {
            0 => vec![
                Operand::Reg("sp".into()),
                Operand::Mem { base: rn.clone(), offset: 0 },
            ],
            1 => vec![
                Operand::Reg("wsp".into()),
                Operand::Mem { base: rn.clone(), offset: 0 },
            ],
            2 => vec![
                Operand::Reg(rs.clone()),
                Operand::Mem { base: format!("w{}", n), offset: 0 },
            ],
            3 => vec![
                Operand::Reg(rs.clone()),
                Operand::Mem { base: "wsp".into(), offset: 0 },
            ],
            4 => vec![
                Operand::Reg(rs.clone()),
                Operand::Mem { base: "xzr".into(), offset: 0 },
            ],
            5 => vec![
                Operand::Reg(rs.clone()),
                Operand::Mem { base: "x31".into(), offset: 0 },
            ],
            _ => vec![
                Operand::Reg(rs),
                Operand::Mem { base: "wzr".into(), offset: 0 },
            ],
        };
        prop_assert!(
            encode_stop(&mnem, &ops).is_err(),
            "SP/WSP as Rs or W/WSP/XZR/x31 as base must Err; llvm-mc/gas reject"
        );
    }

    // Oracle: negative_error
    // Target: encoder.load_store.encode_stop
    // Invalid domain: FP/SIMD Rs, staddb/staddh with X registers.
    #[test]
    fn encode_stop_neg_fp_xbyte(
        n in 0u32..=30,
        kind in 0u32..=5,
        fp in fp_kind(),
    ) {
        let (mnem, ops): (&str, Vec<Operand>) = match kind {
            0 => (
                "stadd",
                vec![
                    Operand::Reg(format!("{}{}", fp, n)),
                    Operand::Mem { base: format!("x{}", (n + 1) % 31), offset: 0 },
                ],
            ),
            1 => (
                "staddb",
                vec![
                    Operand::Reg(format!("x{}", n)),
                    Operand::Mem { base: format!("x{}", (n + 1) % 31), offset: 0 },
                ],
            ),
            2 => (
                "staddh",
                vec![
                    Operand::Reg(format!("x{}", n)),
                    Operand::Mem { base: format!("x{}", (n + 1) % 31), offset: 0 },
                ],
            ),
            3 => (
                "staddlb",
                vec![
                    Operand::Reg(format!("x{}", n)),
                    Operand::Mem { base: format!("x{}", (n + 1) % 31), offset: 0 },
                ],
            ),
            4 => (
                "staddlh",
                vec![
                    Operand::Reg(format!("x{}", n)),
                    Operand::Mem { base: format!("x{}", (n + 1) % 31), offset: 0 },
                ],
            ),
            _ => (
                "stsetlh",
                vec![
                    Operand::Reg(format!("x{}", n)),
                    Operand::Mem { base: format!("x{}", (n + 1) % 31), offset: 0 },
                ],
            ),
        };
        prop_assert!(
            encode_stop(mnem, &ops).is_err(),
            "FP Rs or staddb/staddh with X must Err; llvm-mc/gas reject"
        );
    }

    // Oracle: negative_error
    // Target: encoder.load_store.encode_stop
    // Invalid domain: fewer than 2 operands, or second operand not a bare Mem.
    #[test]
    fn encode_stop_neg_arity_and_shape(
        shape in 0u32..=7,
        n in 0u32..=30,
        off in -8i64..=8,
    ) {
        let rs = format!("x{}", n);
        let ops: Vec<Operand> = match shape {
            0 => vec![],
            1 => vec![Operand::Reg(rs)],
            2 => vec![
                Operand::Reg(rs),
                Operand::Imm(0),
            ],
            3 => vec![
                Operand::Reg(rs),
                Operand::Symbol("foo".into()),
            ],
            4 => vec![
                Operand::Reg(rs),
                Operand::MemPreIndex { base: format!("x{}", (n + 1) % 31), offset: off },
            ],
            5 => vec![
                Operand::Reg(rs),
                Operand::MemPostIndex { base: format!("x{}", (n + 1) % 31), offset: off },
            ],
            6 => vec![
                Operand::Reg(rs),
                Operand::MemRegOffset {
                    base: format!("x{}", (n + 1) % 31),
                    index: format!("x{}", (n + 2) % 31),
                    extend: None,
                    shift: None,
                },
            ],
            _ => vec![
                Operand::Reg(rs),
                Operand::Cond("eq".into()),
            ],
        };
        prop_assert!(
            encode_stop("stadd", &ops).is_err(),
            "arity < 2 or non-Mem second operand must Err"
        );
    }

    // Oracle: negative_error
    // Target: encoder.load_store.encode_stop
    // Invalid domain: Mem offset != 0. ARM optional offset is only #0.
    #[test]
    fn encode_stop_neg_nonzero_offset(
        op in op_strat(),
        suf in suf_strat(),
        rs in reg_edge(),
        rn in 0u32..=30,
        is_64 in any::<bool>(),
        off in nonzero_offset(),
    ) {
        let wide = data_is_64(suf, is_64);
        let mnem = mnemonic(op, suf);
        let ops = vec![
            Operand::Reg(gpr(rs, wide)),
            Operand::Mem { base: base(rn), offset: off },
        ];
        prop_assert!(
            encode_stop(&mnem, &ops).is_err(),
            "nonzero Mem offset must Err; gas: optional immediate offset can only be 0"
        );
    }

    // Oracle: negative_error (sweep)
    // Target: encoder.load_store.encode_stop
    // Invalid domain: unparsable register/base names (foo/x32/empty/r0).
    #[test]
    fn encode_stop_neg_invalid_name(
        slot in 0u32..=1,
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
        let mut ops = valid_ops(0, 0, 1, true);
        match slot {
            0 => ops[0] = Operand::Reg(name),
            _ => {
                ops[1] = Operand::Mem {
                    base: name,
                    offset: 0,
                }
            }
        }
        prop_assert!(
            encode_stop("stadd", &ops).is_err(),
            "invalid register/base name must Err"
        );
    }

    // Oracle: differential (sweep)
    // Target: encoder.load_store.encode_stop
    // Documented: mnemonic is lowercased before suffix parse; llvm-mc accepts STADD/Stadd/STADDB.
    #[test]
    fn encode_stop_diff_alt_spellings(
        op in op_strat(),
        suf in suf_strat(),
        rs in reg_edge(),
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
            "{} {}, [{}]",
            cased,
            gpr(rs, wide),
            base(rn)
        );
        let ops = valid_ops(suf, rs, rn, is_64);
        let sut = sut_word(&cased, &ops)
            .unwrap_or_else(|e| panic!("SUT rejected alt spelling {}: {}", asm, e));
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected alt spelling {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for alt spelling {}", asm);
    }

    // Oracle: negative_error (sweep)
    // Target: encoder.load_store.encode_stop
    // Invalid domain: mnemonic that is not stadd/stclr/steor/stset.
    #[test]
    fn encode_stop_neg_unknown_op(
        name in prop_oneof![
            Just("stfoo".to_string()),
            Just("swp".to_string()),
            Just("cas".to_string()),
            Just("st".to_string()),
            Just("add".to_string()),
            Just("".to_string()),
            Just("ldadd".to_string()),
        ],
    ) {
        let ops = valid_ops(0, 0, 1, true);
        prop_assert!(
            encode_stop(&name, &ops).is_err(),
            "unknown st atomic op must Err"
        );
    }
}

#[test]
fn test_encode_stop_regression_extra_operand() {
    let ops = [
        Operand::Reg("w0".into()),
        Operand::Mem {
            base: "x0".into(),
            offset: 0,
        },
        Operand::Reg("x2".into()),
    ];
    assert!(
        encode_stop("stadd", &ops).is_err(),
        "stadd w0, [x0], x2 must Err; llvm-mc/gas reject a 3rd operand"
    );
}

#[test]
fn test_encode_stop_regression_sp_as_rs() {
    let ops = [
        Operand::Reg("sp".into()),
        Operand::Mem {
            base: "x1".into(),
            offset: 0,
        },
    ];
    assert!(
        encode_stop("stadd", &ops).is_err(),
        "stadd sp, [x1] must Err; llvm-mc/gas reject SP as Rs"
    );
}

#[test]
fn test_encode_stop_regression_fp_reg() {
    let ops = [
        Operand::Reg("b0".into()),
        Operand::Mem {
            base: "x1".into(),
            offset: 0,
        },
    ];
    assert!(
        encode_stop("stadd", &ops).is_err(),
        "stadd b0, [x1] must Err; llvm-mc/gas require integer registers"
    );
}

#[test]
fn test_encode_stop_regression_nonzero_offset() {
    let ops = [
        Operand::Reg("w0".into()),
        Operand::Mem {
            base: "x0".into(),
            offset: -1,
        },
    ];
    assert!(
        encode_stop("stadd", &ops).is_err(),
        "stadd w0, [x0, #-1] must Err; gas: optional immediate offset can only be 0"
    );
}
