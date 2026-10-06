// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md "accepts the same textual assembly that GCC's gas would consume";
//   encoder/mod.rs:1-7 32-bit AArch64 words; encoder/mod.rs:1045-1047 swp* => encode_swp;
//   ARM ARM SWP: size 111000 A R 1 Rs 1 000 00 Rn Rt; size 00=byte 01=half 10=word 11=doubleword;
//   A=acquire (SWPA/SWPAL), R=release (SWPL/SWPAL); Rs/Rt are ZR not SP; Rn is Xn|SP not ZR;
//   SWPB/SWPH require W registers; optional offset only #0.
// Stronger considered:
//   - State machine: rejected — encode_swp is a pure function with no lifecycle
//   - Algebraic round-trip via in-tree decoder: rejected — no SWP decoder
//   - encode_cas as differential sibling: rejected — different LSE class (size 001000, Rt2=11111)
//   - encode_ldop / encode_stop as differential sibling: rejected — different opc / 2-operand alias
// Weaker available: algebraic.invariant (ARM field unpack), algebraic.metamorphic (Rs/Rt/Rn/A/R),
//   negative_error (arity / extra / SP-as-RsRt / XZR-as-base / W-base / mixed / FP / swpb-X / offset)
// Differential: candidate=encode_swp, reference=llvm-mc -triple=aarch64 -mattr=+lse -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[Reg(Rs), Reg(Rt), Mem{Xn|SP, 0}] <-> `swp* Rs, Rt, [Xn|SP]`

use super::load_store::encode_swp;
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

fn mnemonic(v: u32) -> &'static str {
    match v {
        0 => "swp",
        1 => "swpa",
        2 => "swpal",
        3 => "swpl",
        4 => "swpb",
        5 => "swpab",
        6 => "swpalb",
        7 => "swplb",
        8 => "swph",
        9 => "swpah",
        10 => "swpalh",
        11 => "swplh",
        _ => "swp",
    }
}

fn is_byte(v: u32) -> bool {
    (4..8).contains(&v)
}
fn is_half(v: u32) -> bool {
    (8..12).contains(&v)
}
fn data_is_64(v: u32, is_64: bool) -> bool {
    if is_byte(v) || is_half(v) {
        false
    } else {
        is_64
    }
}
fn expected_size(v: u32, wide: bool) -> u32 {
    if is_byte(v) {
        0b00
    } else if is_half(v) {
        0b01
    } else if wide {
        0b11
    } else {
        0b10
    }
}
fn expected_a(v: u32) -> u32 {
    match v {
        1 | 2 | 5 | 6 | 9 | 10 => 1,
        _ => 0,
    }
}
fn expected_r(v: u32) -> u32 {
    match v {
        2 | 3 | 6 | 7 | 10 | 11 => 1,
        _ => 0,
    }
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

fn valid_ops(v: u32, rs: u32, rt: u32, rn: u32, is_64: bool) -> Vec<Operand> {
    let wide = data_is_64(v, is_64);
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
    match encode_swp(mnemonic, ops)? {
        EncodeResult::Word(w) => Ok(w),
        other => Err(format!("expected Word, got {:?}", other)),
    }
}

/// Unpack SWP fields per ARM ARM (not a copy of the SUT packer).
fn unpack_swp(word: u32) -> (u32, u32, u32, u32, u32, u32, u32, u32) {
    let size = (word >> 30) & 0b11;
    let a = (word >> 23) & 1;
    let r = (word >> 22) & 1;
    let rs = (word >> 16) & 0x1f;
    let o3 = (word >> 15) & 1;
    let opc = (word >> 10) & 0x1f;
    let rn = (word >> 5) & 0x1f;
    let rt = word & 0x1f;
    (size, a, r, rs, o3, opc, rn, rt)
}

fn fixed_swp_bits(word: u32) -> bool {
    ((word >> 24) & 0x3f) == 0b111000
        && ((word >> 21) & 1) == 1
        && ((word >> 15) & 1) == 1
        && ((word >> 10) & 0x1f) == 0
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
    let out = child.wait_with_output().map_err(|e| format!("wait llvm-mc: {e}"))?;
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

fn variant_strat() -> impl Strategy<Value = u32> {
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
    prop_oneof![Just('b'), Just('h'), Just('s'), Just('d'), Just('q'), Just('v')]
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_swp_kat_llvm_mc_swp_x0_x1_x2() {
    let want = 0xF8208041u32;
    let mc = llvm_mc_word("swp x0, x1, [x2]").expect("llvm-mc KAT swp x");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = valid_ops(0, 0, 1, 2, true);
    let sut = sut_word("swp", &ops).expect("SUT KAT swp x");
    assert_eq!(sut, want);
}

#[test]
fn encode_swp_kat_llvm_mc_swp_w0_w1_x2() {
    let want = 0xB8208041u32;
    let mc = llvm_mc_word("swp w0, w1, [x2]").expect("llvm-mc KAT swp w");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = valid_ops(0, 0, 1, 2, false);
    let sut = sut_word("swp", &ops).expect("SUT KAT swp w");
    assert_eq!(sut, want);
}

#[test]
fn encode_swp_kat_llvm_mc_swpa_w0_w1_x2() {
    let want = 0xB8A08041u32;
    let mc = llvm_mc_word("swpa w0, w1, [x2]").expect("llvm-mc KAT swpa");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = valid_ops(1, 0, 1, 2, false);
    let sut = sut_word("swpa", &ops).expect("SUT KAT swpa");
    assert_eq!(sut, want);
}

#[test]
fn encode_swp_kat_llvm_mc_swpl_w0_w1_x2() {
    let want = 0xB8608041u32;
    let mc = llvm_mc_word("swpl w0, w1, [x2]").expect("llvm-mc KAT swpl");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = valid_ops(3, 0, 1, 2, false);
    let sut = sut_word("swpl", &ops).expect("SUT KAT swpl");
    assert_eq!(sut, want);
}

#[test]
fn encode_swp_kat_llvm_mc_swpal_w0_w1_x2() {
    let want = 0xB8E08041u32;
    let mc = llvm_mc_word("swpal w0, w1, [x2]").expect("llvm-mc KAT swpal");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = valid_ops(2, 0, 1, 2, false);
    let sut = sut_word("swpal", &ops).expect("SUT KAT swpal");
    assert_eq!(sut, want);
}

#[test]
fn encode_swp_kat_llvm_mc_swpb_w0_w1_x2() {
    let want = 0x38208041u32;
    let mc = llvm_mc_word("swpb w0, w1, [x2]").expect("llvm-mc KAT swpb");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = valid_ops(4, 0, 1, 2, false);
    let sut = sut_word("swpb", &ops).expect("SUT KAT swpb");
    assert_eq!(sut, want);
}

#[test]
fn encode_swp_kat_llvm_mc_swph_w0_w1_x2() {
    let want = 0x78208041u32;
    let mc = llvm_mc_word("swph w0, w1, [x2]").expect("llvm-mc KAT swph");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = valid_ops(8, 0, 1, 2, false);
    let sut = sut_word("swph", &ops).expect("SUT KAT swph");
    assert_eq!(sut, want);
}

#[test]
fn encode_swp_kat_llvm_mc_swp_xzr_xzr_sp() {
    let want = 0xF83F83FFu32;
    let mc = llvm_mc_word("swp xzr, xzr, [sp]").expect("llvm-mc KAT zr/sp");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = valid_ops(0, 31, 31, 31, true);
    let sut = sut_word("swp", &ops).expect("SUT KAT zr/sp");
    assert_eq!(sut, want);
}

proptest! {
    #![proptest_config(cfg())]

    // Oracle: differential
    // Target: encoder.load_store.encode_swp
    #[test]
    fn encode_swp_diff_llvm_mc(
        v in variant_strat(),
        rs in reg_edge(),
        rt in reg_edge(),
        rn in reg_edge(),
        is_64 in any::<bool>(),
    ) {
        let wide = data_is_64(v, is_64);
        let mnem = mnemonic(v);
        let asm = format!("{} {}, {}, [{}]", mnem, gpr(rs, wide), gpr(rt, wide), base(rn));
        let ops = valid_ops(v, rs, rt, rn, is_64);
        let sut = sut_word(mnem, &ops)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
    }

    // Oracle: algebraic.invariant
    // Target: encoder.load_store.encode_swp
    #[test]
    fn encode_swp_arm_fields(
        v in variant_strat(),
        rs in reg_edge(),
        rt in reg_edge(),
        rn in reg_edge(),
        is_64 in any::<bool>(),
    ) {
        let wide = data_is_64(v, is_64);
        let mnem = mnemonic(v);
        let ops = valid_ops(v, rs, rt, rn, is_64);
        let word = sut_word(mnem, &ops)
            .unwrap_or_else(|e| panic!("SUT rejected valid SWP: {}", e));
        let (size, a, r, got_rs, o3, opc, got_rn, got_rt) = unpack_swp(word);
        prop_assert_eq!(size, expected_size(v, wide), "size mismatch word={:#010x}", word);
        prop_assert_eq!(a, expected_a(v), "A bit mismatch word={:#010x}", word);
        prop_assert_eq!(r, expected_r(v), "R bit mismatch word={:#010x}", word);
        prop_assert_eq!(got_rs, rs, "Rs field mismatch word={:#010x}", word);
        prop_assert_eq!(got_rt, rt, "Rt field mismatch word={:#010x}", word);
        prop_assert_eq!(got_rn, rn, "Rn field mismatch word={:#010x}", word);
        prop_assert_eq!(o3, 1, "o3 must be 1 word={:#010x}", word);
        prop_assert_eq!(opc, 0, "opc must be 00000 word={:#010x}", word);
        prop_assert!(fixed_swp_bits(word), "fixed SWP bits violated word={:#010x}", word);
    }

    // Oracle: algebraic.metamorphic
    // Target: encoder.load_store.encode_swp
    #[test]
    fn encode_swp_metamorphic_regs_ar(
        rs in 0u32..=30,
        rt in 0u32..=30,
        rn in 0u32..=30,
        is_64 in any::<bool>(),
    ) {
        let base_ops = valid_ops(0, rs, rt, rn, is_64);
        let base_w = sut_word("swp", &base_ops)
            .unwrap_or_else(|e| panic!("SUT rejected swp: {}", e));
        let rt1 = sut_word("swp", &valid_ops(0, rs, rt + 1, rn, is_64))
            .unwrap_or_else(|e| panic!("SUT rejected rt+1: {}", e));
        prop_assert_eq!(rt1 & !0x1fu32, base_w & !0x1fu32, "Rt+1 must change only bits[4:0]");
        prop_assert_eq!(rt1 & 0x1f, rt + 1);
        let rn1 = sut_word("swp", &valid_ops(0, rs, rt, rn + 1, is_64))
            .unwrap_or_else(|e| panic!("SUT rejected rn+1: {}", e));
        prop_assert_eq!(rn1 & !(0x1fu32 << 5), base_w & !(0x1fu32 << 5), "Rn+1 must change only bits[9:5]");
        prop_assert_eq!((rn1 >> 5) & 0x1f, rn + 1);
        let rs1 = sut_word("swp", &valid_ops(0, rs + 1, rt, rn, is_64))
            .unwrap_or_else(|e| panic!("SUT rejected rs+1: {}", e));
        prop_assert_eq!(rs1 & !(0x1fu32 << 16), base_w & !(0x1fu32 << 16), "Rs+1 must change only bits[20:16]");
        prop_assert_eq!((rs1 >> 16) & 0x1f, rs + 1);
        let swpa_w = sut_word("swpa", &base_ops)
            .unwrap_or_else(|e| panic!("SUT rejected swpa: {}", e));
        prop_assert_eq!(swpa_w ^ base_w, 1u32 << 23, "SWPA vs SWP must differ only by A at bit 23");
        let swpl_w = sut_word("swpl", &base_ops)
            .unwrap_or_else(|e| panic!("SUT rejected swpl: {}", e));
        prop_assert_eq!(swpl_w ^ base_w, 1u32 << 22, "SWPL vs SWP must differ only by R at bit 22");
        let swpal_w = sut_word("swpal", &base_ops)
            .unwrap_or_else(|e| panic!("SUT rejected swpal: {}", e));
        prop_assert_eq!(swpal_w ^ base_w, (1u32 << 23) | (1u32 << 22), "SWPAL vs SWP must flip A and R");
        let upper = sut_word("SWP", &base_ops)
            .unwrap_or_else(|e| panic!("SUT rejected uppercase SWP: {}", e));
        prop_assert_eq!(upper, base_w, "uppercase mnemonic must match lowercase");
    }

    // Oracle: negative_error
    // Target: encoder.load_store.encode_swp
    // Invalid domain: a fourth operand. llvm-mc/gas reject extra operands.
    #[test]
    fn encode_swp_neg_extra_operand(
        v in variant_strat(),
        rs in reg_edge(),
        rt in reg_edge(),
        rn in reg_edge(),
        is_64 in any::<bool>(),
        extra in extra_operand(),
    ) {
        let mnem = mnemonic(v);
        let mut ops = valid_ops(v, rs, rt, rn, is_64);
        ops.push(extra);
        prop_assert!(
            encode_swp(mnem, &ops).is_err(),
            "extra operand must Err; llvm-mc/gas reject a 4th SWP operand"
        );
    }

    // Oracle: negative_error
    // Target: encoder.load_store.encode_swp
    // Invalid domain: SP/WSP as Rs/Rt; W/WSP/XZR/x31 as base.
    #[test]
    fn encode_swp_neg_sp_zr_base(
        v in variant_strat(),
        n in 0u32..=30,
        kind in 0u32..=8,
        is_64 in any::<bool>(),
    ) {
        let wide = data_is_64(v, is_64);
        let mnem = mnemonic(v);
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
            encode_swp(mnem, &ops).is_err(),
            "SP/WSP as Rs/Rt or W/WSP/XZR/x31 as base must Err; llvm-mc/gas reject"
        );
    }

    // Oracle: negative_error
    // Target: encoder.load_store.encode_swp
    // Invalid domain: mixed W/X, FP/SIMD prefixes, swpb/swph with X registers.
    #[test]
    fn encode_swp_neg_mixed_fp_xbyte(
        n in 0u32..=30,
        kind in 0u32..=8,
        fp in fp_kind(),
    ) {
        let (mnem, ops): (&str, Vec<Operand>) = match kind {
            0 => (
                "swp",
                vec![
                    Operand::Reg(format!("x{}", n)),
                    Operand::Reg(format!("w{}", n)),
                    Operand::Mem { base: format!("x{}", (n + 1) % 31), offset: 0 },
                ],
            ),
            1 => (
                "swp",
                vec![
                    Operand::Reg(format!("w{}", n)),
                    Operand::Reg(format!("x{}", n)),
                    Operand::Mem { base: format!("x{}", (n + 1) % 31), offset: 0 },
                ],
            ),
            2 => (
                "swp",
                vec![
                    Operand::Reg(format!("{}{}", fp, n)),
                    Operand::Reg(format!("{}{}", fp, (n + 1) % 31)),
                    Operand::Mem { base: format!("x{}", (n + 2) % 31), offset: 0 },
                ],
            ),
            3 => (
                "swp",
                vec![
                    Operand::Reg(format!("{}{}", fp, n)),
                    Operand::Reg(format!("x{}", n)),
                    Operand::Mem { base: format!("x{}", (n + 1) % 31), offset: 0 },
                ],
            ),
            4 => (
                "swpb",
                vec![
                    Operand::Reg(format!("x{}", n)),
                    Operand::Reg(format!("x{}", (n + 1) % 31)),
                    Operand::Mem { base: format!("x{}", (n + 2) % 31), offset: 0 },
                ],
            ),
            5 => (
                "swph",
                vec![
                    Operand::Reg(format!("x{}", n)),
                    Operand::Reg(format!("x{}", (n + 1) % 31)),
                    Operand::Mem { base: format!("x{}", (n + 2) % 31), offset: 0 },
                ],
            ),
            6 => (
                "swpb",
                vec![
                    Operand::Reg(format!("x{}", n)),
                    Operand::Reg(format!("w{}", (n + 1) % 31)),
                    Operand::Mem { base: format!("x{}", (n + 2) % 31), offset: 0 },
                ],
            ),
            7 => (
                "swpab",
                vec![
                    Operand::Reg(format!("x{}", n)),
                    Operand::Reg(format!("x{}", (n + 1) % 31)),
                    Operand::Mem { base: format!("x{}", (n + 2) % 31), offset: 0 },
                ],
            ),
            _ => (
                "swpalh",
                vec![
                    Operand::Reg(format!("x{}", n)),
                    Operand::Reg(format!("x{}", (n + 1) % 31)),
                    Operand::Mem { base: format!("x{}", (n + 2) % 31), offset: 0 },
                ],
            ),
        };
        prop_assert!(
            encode_swp(mnem, &ops).is_err(),
            "mixed W/X, FP, or swpb/swph with X must Err; llvm-mc/gas reject"
        );
    }

    // Oracle: negative_error
    // Target: encoder.load_store.encode_swp
    // Invalid domain: fewer than 3 operands, or third operand not a bare Mem.
    #[test]
    fn encode_swp_neg_arity_and_shape(
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
            encode_swp("swp", &ops).is_err(),
            "arity < 3 or non-Mem third operand must Err"
        );
    }

    // Oracle: negative_error
    // Target: encoder.load_store.encode_swp
    // Invalid domain: Mem offset != 0. ARM optional offset is only #0.
    #[test]
    fn encode_swp_neg_nonzero_offset(
        v in variant_strat(),
        rs in reg_edge(),
        rt in reg_edge(),
        rn in 0u32..=30,
        is_64 in any::<bool>(),
        off in nonzero_offset(),
    ) {
        let wide = data_is_64(v, is_64);
        let mnem = mnemonic(v);
        let ops = vec![
            Operand::Reg(gpr(rs, wide)),
            Operand::Reg(gpr(rt, wide)),
            Operand::Mem { base: base(rn), offset: off },
        ];
        prop_assert!(
            encode_swp(mnem, &ops).is_err(),
            "nonzero Mem offset must Err; gas: optional immediate offset can only be 0"
        );
    }

    // Oracle: negative_error (sweep)
    // Target: encoder.load_store.encode_swp
    // Invalid domain: unparsable register/base names (foo/x32/empty/r0).
    #[test]
    fn encode_swp_neg_invalid_name(
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
            encode_swp("swp", &ops).is_err(),
            "invalid register/base name must Err"
        );
    }

    // Oracle: differential (sweep)
    // Target: encoder.load_store.encode_swp
    // Documented: mnemonic is lowercased before suffix parse; llvm-mc accepts SWP/Swp/SWPB.
    #[test]
    fn encode_swp_diff_alt_spellings(
        v in variant_strat(),
        rs in reg_edge(),
        rt in reg_edge(),
        rn in reg_edge(),
        is_64 in any::<bool>(),
        mode in 0u32..=2,
    ) {
        let wide = data_is_64(v, is_64);
        let mnem = mnemonic(v);
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
        let ops = valid_ops(v, rs, rt, rn, is_64);
        let sut = sut_word(&cased, &ops)
            .unwrap_or_else(|e| panic!("SUT rejected alt spelling {}: {}", asm, e));
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected alt spelling {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for alt spelling {}", asm);
    }
}

#[test]
fn test_encode_swp_regression_extra_operand() {
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
        encode_swp("swp", &ops).is_err(),
        "swp w0, w0, [x0], x2 must Err; llvm-mc/gas reject a 4th operand"
    );
}

#[test]
fn test_encode_swp_regression_sp_as_rs() {
    let ops = [
        Operand::Reg("sp".into()),
        Operand::Reg("w1".into()),
        Operand::Mem {
            base: "x2".into(),
            offset: 0,
        },
    ];
    assert!(
        encode_swp("swp", &ops).is_err(),
        "swp sp, w1, [x2] must Err; llvm-mc/gas reject SP as Rs"
    );
}

#[test]
fn test_encode_swp_regression_xzr_as_base() {
    let ops = [
        Operand::Reg("x0".into()),
        Operand::Reg("x1".into()),
        Operand::Mem {
            base: "xzr".into(),
            offset: 0,
        },
    ];
    assert!(
        encode_swp("swp", &ops).is_err(),
        "swp x0, x1, [xzr] must Err; llvm-mc/gas reject XZR as base (Rn=31 is SP)"
    );
}

#[test]
fn test_encode_swp_regression_w_base() {
    let ops = [
        Operand::Reg("w0".into()),
        Operand::Reg("w1".into()),
        Operand::Mem {
            base: "w2".into(),
            offset: 0,
        },
    ];
    assert!(
        encode_swp("swp", &ops).is_err(),
        "swp w0, w1, [w2] must Err; llvm-mc/gas require Xn|SP as base"
    );
}

#[test]
fn test_encode_swp_regression_mixed_width() {
    let ops = [
        Operand::Reg("x0".into()),
        Operand::Reg("w0".into()),
        Operand::Mem {
            base: "x1".into(),
            offset: 0,
        },
    ];
    assert!(
        encode_swp("swp", &ops).is_err(),
        "swp x0, w0, [x1] must Err; llvm-mc/gas reject mixed W/X"
    );
}

#[test]
fn test_encode_swp_regression_fp_reg() {
    let ops = [
        Operand::Reg("s0".into()),
        Operand::Reg("s1".into()),
        Operand::Mem {
            base: "x2".into(),
            offset: 0,
        },
    ];
    assert!(
        encode_swp("swp", &ops).is_err(),
        "swp s0, s1, [x2] must Err; llvm-mc/gas require integer registers"
    );
}

#[test]
fn test_encode_swp_regression_swpb_x_reg() {
    let ops = [
        Operand::Reg("x0".into()),
        Operand::Reg("x1".into()),
        Operand::Mem {
            base: "x2".into(),
            offset: 0,
        },
    ];
    assert!(
        encode_swp("swpb", &ops).is_err(),
        "swpb x0, x1, [x2] must Err; llvm-mc/gas require W registers for SWPB"
    );
}

#[test]
fn test_encode_swp_regression_nonzero_offset() {
    let ops = [
        Operand::Reg("w0".into()),
        Operand::Reg("w0".into()),
        Operand::Mem {
            base: "x0".into(),
            offset: -1,
        },
    ];
    assert!(
        encode_swp("swp", &ops).is_err(),
        "swp w0, w0, [x0, #-1] must Err; gas: optional immediate offset can only be 0"
    );
}
