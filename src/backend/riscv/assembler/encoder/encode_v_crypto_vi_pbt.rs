// Oracle: reference — RISC-V Cryptography Extensions Volume II (Zvksh/Zvksed VI)
// Evidence: vector.rs:189-195 "Encode Zvksh/Zvksed crypto instructions with VI format
//   vsm3c.vi, vsm4k.vi: funct6 | vm=1 | vs2 | uimm5 | 010 | vd | OP_V_CRYPTO";
//   encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words";
//   encoder/mod.rs:445 OP_V_CRYPTO = 0b1110111 "Vector crypto (Zvk*) — uses OP-P
//     encoding space per RVV Crypto spec";
//   encoder/mod.rs:1018 "vsm3c.vi" => encode_v_crypto_vi(operands, 0b101011);
//   encoder/mod.rs:1021 "vsm4k.vi" => encode_v_crypto_vi(operands, 0b100001);
//   assembler/README.md:14 Zvksh/Zvksed; assembler/README.md:109 vector.rs;
//   RISC-V Cryptography Extensions Volume II: opcode=1010111 (OP-V), funct3=010,
//   vm=1 (not maskable), vd, uimm5, vs2, funct6 vsm3c.vi=101011 / vsm4k.vi=100001.
// Stronger considered:
//   - State machine: rejected — encode_v_crypto_vi is a pure function with no lifecycle
//   - Algebraic round-trip via in-tree decoder: rejected — no Zvk VI decoder
//   - llvm-mc differential: rejected — LLVM 15.0.6 does not recognize vsm3c.vi/vsm4k.vi
//   - encode_v_arith_vi as differential sibling: rejected — same-job gate
//     (OPIVI funct3=011 / OP-V, not crypto VI)
// Weaker available: algebraic.invariant (field unpack), algebraic.metamorphic
//   (field isolation, vd/vs2 swap), negative_error (arity / bad regs / extra / oob / v0.t)
// Reference: candidate=encode_v_crypto_vi,
//   reference=RISC-V Cryptography Extensions Volume II (claimed by README + OP_V_CRYPTO comment),
//   SUT-boundary=internal-helper of RISC-V assembler,
//   mapping=[Reg(vd), Reg(vs2), Imm(uimm5)] + funct6
//     <-> `vsm3c.vi/vsm4k.vi vd, vs2, uimm5`.

use super::{encode_v_crypto_vi, EncodeResult};
use crate::backend::riscv::assembler::parser::Operand;
use proptest::prelude::*;

const CASES: u32 = 1000;
const OP_V: u32 = 0b1010111;
const OP_V_CRYPTO: u32 = 0b1110111;
const FUNCT3_CRYPTO_VI: u32 = 0b010;
const FUNCT6_VSM3C_VI: u32 = 0b101011;
const FUNCT6_VSM4K_VI: u32 = 0b100001;

const CRYPTO_VI_FAMILY: [(&str, u32); 2] = [
    ("vsm3c.vi", FUNCT6_VSM3C_VI),
    ("vsm4k.vi", FUNCT6_VSM4K_VI),
];

fn cfg() -> ProptestConfig {
    ProptestConfig::with_cases(CASES)
}

fn vn(n: u32) -> String {
    format!("v{n}")
}

fn vreg(n: u32) -> Operand {
    Operand::Reg(vn(n))
}

fn ops3(vd: u32, vs2: u32, imm: i64) -> Vec<Operand> {
    vec![vreg(vd), vreg(vs2), Operand::Imm(imm)]
}

fn sut_word(ops: &[Operand], funct6: u32) -> Result<u32, String> {
    match encode_v_crypto_vi(ops, funct6)? {
        EncodeResult::Word(w) => Ok(w),
        other => Err(format!("expected Word, got {other:?}")),
    }
}

/// Unpack crypto VI per RISC-V Vector Crypto (not a copy of encode_v_crypto_vi).
fn unpack_crypto_vi(word: u32) -> (u32, u32, u32, u32, u32, u32, u32) {
    let opcode = word & 0x7f;
    let vd = (word >> 7) & 0x1f;
    let funct3 = (word >> 12) & 0x7;
    let uimm5 = (word >> 15) & 0x1f;
    let vs2 = (word >> 20) & 0x1f;
    let vm = (word >> 25) & 1;
    let funct6 = word >> 26;
    (opcode, vd, funct3, uimm5, vs2, vm, funct6)
}

fn spec_word(vd: u32, vs2: u32, uimm: i64, funct6: u32) -> u32 {
    (funct6 << 26)
        | (1u32 << 25)
        | (vs2 << 20)
        | (((uimm as u32) & 0x1F) << 15)
        | (FUNCT3_CRYPTO_VI << 12)
        | (vd << 7)
        | OP_V
}

fn vreg_n() -> impl Strategy<Value = u32> {
    0u32..=31
}

fn funct6_n() -> impl Strategy<Value = u32> {
    0u32..=63
}

fn family_kind() -> impl Strategy<Value = usize> {
    0usize..=1
}

fn uimm5() -> impl Strategy<Value = i64> {
    prop_oneof![Just(0i64), Just(16i64), Just(31i64), 0i64..=31]
}

fn extra_operand() -> impl Strategy<Value = Operand> {
    prop_oneof![
        Just(Operand::Imm(0)),
        Just(Operand::Imm(-1)),
        Just(Operand::Imm(208)),
        (0u32..=31).prop_map(|n| Operand::Reg(format!("x{n}"))),
        (0u32..=31).prop_map(|n| Operand::Reg(vn(n))),
        Just(Operand::Symbol("foo".into())),
        Just(Operand::Symbol("v0.t".into())),
        Just(Operand::Label("L0".into())),
        Just(Operand::SymbolOffset("foo".into(), 4)),
        Just(Operand::Mem {
            base: "sp".into(),
            offset: 8,
        }),
        Just(Operand::FenceArg("iorw".into())),
        Just(Operand::Csr("mstatus".into())),
        Just(Operand::RoundingMode("rne".into())),
    ]
}

fn bad_vreg() -> impl Strategy<Value = Operand> {
    prop_oneof![
        (0u32..=31).prop_map(|n| Operand::Reg(format!("x{n}"))),
        Just(Operand::Reg("a0".into())),
        Just(Operand::Reg("fa0".into())),
        Just(Operand::Reg("ft0".into())),
        Just(Operand::Reg("f0".into())),
        Just(Operand::Reg("zero".into())),
        Just(Operand::Reg("sp".into())),
        Just(Operand::Reg("v32".into())),
        Just(Operand::Imm(0)),
        Just(Operand::Imm(31)),
        Just(Operand::Symbol("foo".into())),
        Just(Operand::Label("L0".into())),
        Just(Operand::Mem {
            base: "sp".into(),
            offset: 0,
        }),
        Just(Operand::FenceArg("iorw".into())),
        Just(Operand::Csr("mstatus".into())),
        Just(Operand::RoundingMode("rne".into())),
    ]
}

fn bad_imm() -> impl Strategy<Value = Operand> {
    prop_oneof![
        (0u32..=31).prop_map(|n| Operand::Reg(vn(n))),
        (0u32..=31).prop_map(|n| Operand::Reg(format!("x{n}"))),
        Just(Operand::Reg("a0".into())),
        Just(Operand::Reg("fa0".into())),
        Just(Operand::Symbol("foo".into())),
        Just(Operand::Label("L0".into())),
        Just(Operand::Mem {
            base: "sp".into(),
            offset: 0,
        }),
        Just(Operand::FenceArg("iorw".into())),
        Just(Operand::Csr("mstatus".into())),
        Just(Operand::RoundingMode("rne".into())),
        Just(Operand::SymbolOffset("foo".into(), 4)),
    ]
}

fn short_ops() -> impl Strategy<Value = Vec<Operand>> {
    prop_oneof![
        Just(vec![]),
        vreg_n().prop_map(|vd| vec![vreg(vd)]),
        (vreg_n(), vreg_n()).prop_map(|(a, b)| vec![vreg(a), vreg(b)]),
    ]
}

fn uimm_oob() -> impl Strategy<Value = i64> {
    prop_oneof![
        Just(-1i64),
        Just(-16i64),
        Just(32i64),
        Just(i64::MIN),
        Just(i64::MAX),
        -1000i64..=-1,
        32i64..=1000,
    ]
}

/// Known-answer gate: rustdoc packing with OP_V_CRYPTO (proves the symbol is linked).
#[test]
fn encode_v_crypto_vi_kat_vsm3c_v0_v0_0() {
    let want = 0xae002077u32;
    let sut = sut_word(&ops3(0, 0, 0), FUNCT6_VSM3C_VI).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_v_crypto_vi_kat_vsm3c_v1_v2_3() {
    let want = 0xae21a0f7u32;
    let sut = sut_word(&ops3(1, 2, 3), FUNCT6_VSM3C_VI).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_v_crypto_vi_kat_vsm3c_v31_v30_31() {
    let want = 0xafefaff7u32;
    let sut = sut_word(&ops3(31, 30, 31), FUNCT6_VSM3C_VI).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_v_crypto_vi_kat_vsm4k_v1_v2_3() {
    let want = 0x8621a0f7u32;
    let sut = sut_word(&ops3(1, 2, 3), FUNCT6_VSM4K_VI).expect("SUT KAT");
    assert_eq!(sut, want);
}

/// Reference KAT: Volume II uses OP-V (1010111), not OP-P.
#[test]
fn encode_v_crypto_vi_kat_spec_opcode_vsm3c_v1_v2_3() {
    let want = spec_word(1, 2, 3, FUNCT6_VSM3C_VI);
    assert_eq!(want, 0xae21a0d7u32, "spec encoding drifted");
    let sut = sut_word(&ops3(1, 2, 3), FUNCT6_VSM3C_VI).expect("SUT KAT");
    assert_eq!(
        sut, want,
        "SUT {:#010x} != Volume II OP-V {:#010x} for vsm3c.vi v1, v2, 3",
        sut, want
    );
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_v_crypto_vi_spec_opcode(
        vd in vreg_n(),
        vs2 in vreg_n(),
        uimm in uimm5(),
        kind in family_kind(),
    ) {
        let (_, funct6) = CRYPTO_VI_FAMILY[kind];
        let sut = sut_word(&ops3(vd, vs2, uimm), funct6)
            .unwrap_or_else(|e| panic!("SUT rejected valid vd={vd} vs2={vs2} uimm={uimm}: {e}"));
        prop_assert_eq!(
            sut & 0x7f,
            OP_V,
            "opcode must be OP-V 1010111 per RISC-V Crypto Volume II; got {:#09b}",
            sut & 0x7f
        );
        prop_assert_eq!(
            sut,
            spec_word(vd, vs2, uimm, funct6),
            "SUT {:#010x} != Volume II {:#010x}",
            sut,
            spec_word(vd, vs2, uimm, funct6)
        );
    }

    #[test]
    fn encode_v_crypto_vi_format_fields(
        vd in vreg_n(),
        vs2 in vreg_n(),
        uimm in uimm5(),
        funct6 in funct6_n(),
    ) {
        let w = sut_word(&ops3(vd, vs2, uimm), funct6)
            .unwrap_or_else(|e| panic!("SUT rejected: {e}"));
        let (opcode, got_vd, funct3, got_uimm, got_vs2, vm, got_f6) = unpack_crypto_vi(w);
        prop_assert_eq!(got_vd, vd, "vd");
        prop_assert_eq!(funct3, FUNCT3_CRYPTO_VI, "funct3 must be 010");
        prop_assert_eq!(got_uimm, (uimm as u32) & 0x1F, "uimm5");
        prop_assert_eq!(got_vs2, vs2, "vs2");
        prop_assert_eq!(vm, 1, "vm must be 1 (unmasked / not maskable)");
        prop_assert_eq!(got_f6, funct6, "funct6");
        // Opcode is claimed as OP_V_CRYPTO in rustdoc; Volume II says OP-V.
        // Record both so a mismatch is visible without hiding packing.
        let _ = opcode;
        let _ = OP_V_CRYPTO;
    }

    #[test]
    fn encode_v_crypto_vi_field_isolation(
        vd_a in vreg_n(),
        vd_b in vreg_n(),
        vs2_a in vreg_n(),
        vs2_b in vreg_n(),
        uimm_a in uimm5(),
        uimm_b in uimm5(),
        f6_a in funct6_n(),
        f6_b in funct6_n(),
    ) {
        let wa = sut_word(&ops3(vd_a, vs2_a, uimm_a), f6_a)
            .unwrap_or_else(|e| panic!("a rejected: {e}"));
        let wb = sut_word(&ops3(vd_b, vs2_a, uimm_a), f6_a)
            .unwrap_or_else(|e| panic!("b rejected: {e}"));
        let vd_mask = 0x1fu32 << 7;
        prop_assert_eq!(wa & !vd_mask, wb & !vd_mask, "non-vd bits independent of vd");
        prop_assert_eq!((wa >> 7) & 0x1f, vd_a);
        prop_assert_eq!((wb >> 7) & 0x1f, vd_b);

        let wc = sut_word(&ops3(vd_a, vs2_b, uimm_a), f6_a)
            .unwrap_or_else(|e| panic!("c rejected: {e}"));
        let vs2_mask = 0x1fu32 << 20;
        prop_assert_eq!(wa & !vs2_mask, wc & !vs2_mask, "non-vs2 bits independent of vs2");
        prop_assert_eq!((wc >> 20) & 0x1f, vs2_b);

        let wd = sut_word(&ops3(vd_a, vs2_a, uimm_b), f6_a)
            .unwrap_or_else(|e| panic!("d rejected: {e}"));
        let uimm_mask = 0x1fu32 << 15;
        prop_assert_eq!(wa & !uimm_mask, wd & !uimm_mask, "non-uimm5 bits independent of uimm");
        prop_assert_eq!((wd >> 15) & 0x1f, (uimm_b as u32) & 0x1F);

        let we = sut_word(&ops3(vd_a, vs2_a, uimm_a), f6_b)
            .unwrap_or_else(|e| panic!("e rejected: {e}"));
        let f6_mask = 0x3fu32 << 26;
        prop_assert_eq!(wa & !f6_mask, we & !f6_mask, "non-funct6 bits independent of funct6");
        prop_assert_eq!(we >> 26, f6_b);
    }

    #[test]
    fn encode_v_crypto_vi_vd_vs2_swap(
        vd in vreg_n(),
        vs2 in vreg_n(),
        uimm in uimm5(),
        funct6 in funct6_n(),
    ) {
        let w = sut_word(&ops3(vd, vs2, uimm), funct6)
            .unwrap_or_else(|e| panic!("w rejected: {e}"));
        let w2 = sut_word(&ops3(vs2, vd, uimm), funct6)
            .unwrap_or_else(|e| panic!("w2 rejected: {e}"));
        let vd_mask = 0x1fu32 << 7;
        let vs2_mask = 0x1fu32 << 20;
        prop_assert_eq!((w >> 7) & 0x1f, vd);
        prop_assert_eq!((w >> 20) & 0x1f, vs2);
        prop_assert_eq!((w2 >> 7) & 0x1f, vs2);
        prop_assert_eq!((w2 >> 20) & 0x1f, vd);
        prop_assert_eq!(
            w & !(vd_mask | vs2_mask),
            w2 & !(vd_mask | vs2_mask),
            "non vd/vs2 bits must be preserved under operand swap"
        );
    }

    #[test]
    fn encode_v_crypto_vi_neg_arity_bad_regs(
        ops in short_ops(),
        bad_v in bad_vreg(),
        bad_i in bad_imm(),
        pos in 0usize..=1,
        funct6 in funct6_n(),
    ) {
        prop_assert!(
            encode_v_crypto_vi(&ops, funct6).is_err(),
            "arity {} must Err; got {:?}",
            ops.len(),
            encode_v_crypto_vi(&ops, funct6)
        );
        let mut bad_ops = ops3(0, 1, 0);
        bad_ops[pos] = bad_v.clone();
        prop_assert!(
            encode_v_crypto_vi(&bad_ops, funct6).is_err(),
            "non-vector operand {:?} at pos {pos} must Err; got {:?}",
            bad_v,
            encode_v_crypto_vi(&bad_ops, funct6)
        );
        let mut bad_imm_ops = ops3(0, 1, 0);
        bad_imm_ops[2] = bad_i.clone();
        prop_assert!(
            encode_v_crypto_vi(&bad_imm_ops, funct6).is_err(),
            "non-Imm operand {:?} at uimm5 must Err; got {:?}",
            bad_i,
            encode_v_crypto_vi(&bad_imm_ops, funct6)
        );
    }

    #[test]
    fn encode_v_crypto_vi_neg_extra(
        vd in vreg_n(),
        vs2 in vreg_n(),
        uimm in uimm5(),
        extra in extra_operand(),
        kind in family_kind(),
    ) {
        let (_, funct6) = CRYPTO_VI_FAMILY[kind];
        let mut ops = ops3(vd, vs2, uimm);
        ops.push(extra.clone());
        prop_assert!(
            encode_v_crypto_vi(&ops, funct6).is_err(),
            "extra operand {:?} must Err for crypto VI; got {:?}",
            extra,
            encode_v_crypto_vi(&ops, funct6)
        );
    }

    #[test]
    fn encode_v_crypto_vi_neg_uimm_oob(
        vd in vreg_n(),
        vs2 in vreg_n(),
        uimm in uimm_oob(),
        kind in family_kind(),
    ) {
        let (mnem, funct6) = CRYPTO_VI_FAMILY[kind];
        let ops = ops3(vd, vs2, uimm);
        prop_assert!(
            encode_v_crypto_vi(&ops, funct6).is_err(),
            "{mnem} v{vd}, v{vs2}, {uimm} must Err (uimm5 domain [0, 31]); got {:?}",
            encode_v_crypto_vi(&ops, funct6)
        );
    }

    #[test]
    fn encode_v_crypto_vi_neg_mask_v0t(
        vd in vreg_n(),
        vs2 in vreg_n(),
        uimm in uimm5(),
        kind in family_kind(),
    ) {
        let (_, funct6) = CRYPTO_VI_FAMILY[kind];
        let mut ops = ops3(vd, vs2, uimm);
        ops.push(Operand::Symbol("v0.t".into()));
        prop_assert!(
            encode_v_crypto_vi(&ops, funct6).is_err(),
            "trailing v0.t must Err (Zvksh/Zvksed VI is not maskable); got {:?}",
            encode_v_crypto_vi(&ops, funct6)
        );
    }
}

/// Regression: extra operand after a complete vd, vs2, uimm crypto VI must Err.
#[test]
fn test_encode_v_crypto_vi_regression_extra_operand() {
    let mut ops = ops3(0, 0, 0);
    ops.push(Operand::Imm(0));
    let got = encode_v_crypto_vi(&ops, FUNCT6_VSM3C_VI);
    assert!(
        got.is_err(),
        "vsm3c.vi v0, v0, 0, 0 must Err; got {got:?}"
    );
}

/// Regression: trailing v0.t must Err (not maskable).
#[test]
fn test_encode_v_crypto_vi_regression_mask_v0t() {
    let mut ops = ops3(1, 2, 3);
    ops.push(Operand::Symbol("v0.t".into()));
    let got = encode_v_crypto_vi(&ops, FUNCT6_VSM3C_VI);
    assert!(
        got.is_err(),
        "vsm3c.vi v1, v2, 3, v0.t must Err; got {got:?}"
    );
}

/// Regression: uimm 32 is outside uimm5 [0, 31] and must Err.
#[test]
fn test_encode_v_crypto_vi_regression_uimm_oob_32() {
    let got = encode_v_crypto_vi(&ops3(1, 2, 32), FUNCT6_VSM3C_VI);
    assert!(
        got.is_err(),
        "vsm3c.vi v1, v2, 32 must Err (uimm5 [0, 31]); got {got:?}"
    );
}

/// Regression: uimm -1 is outside uimm5 [0, 31] and must Err.
#[test]
fn test_encode_v_crypto_vi_regression_uimm_oob_m1() {
    let got = encode_v_crypto_vi(&ops3(1, 2, -1), FUNCT6_VSM4K_VI);
    assert!(
        got.is_err(),
        "vsm4k.vi v1, v2, -1 must Err (uimm5 [0, 31]); got {got:?}"
    );
}

/// Regression: opcode must be OP-V 1010111 per Volume II.
#[test]
fn test_encode_v_crypto_vi_regression_spec_opcode() {
    let got = sut_word(&ops3(1, 2, 3), FUNCT6_VSM3C_VI).expect("encode");
    assert_eq!(
        got & 0x7f,
        OP_V,
        "vsm3c.vi opcode must be OP-V; got {:#09b}",
        got & 0x7f
    );
}
