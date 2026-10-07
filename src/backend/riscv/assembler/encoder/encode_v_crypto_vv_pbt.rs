// Oracle: reference — RISC-V Cryptography Extensions Volume II (Zvksh VV)
// Evidence: vector.rs:199-200 "Encode Zvksh crypto instructions with VV format
//   vsm3me.vv: funct6 | vm=1 | vs2 | vs1 | 010 | vd | OP_V_CRYPTO";
//   encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words";
//   encoder/mod.rs:447 OP_V_CRYPTO = 0b1110111 "Vector crypto (Zvk*) — uses OP-P
//     encoding space per RVV Crypto spec";
//   encoder/mod.rs:1021 "vsm3me.vv" => encode_v_crypto_vv(operands, 0b100000);
//   assembler/README.md:14 Zvksh/Zvksed; assembler/README.md:109 vector.rs;
//   RISC-V Cryptography Extensions Volume II: opcode=1010111 (OP-V), funct3=010,
//   vm=1 (not maskable), vd, vs1, vs2, funct6 vsm3me.vv=100000.
// Stronger considered:
//   - State machine: rejected — encode_v_crypto_vv is a pure function with no lifecycle
//   - Algebraic round-trip via in-tree decoder: rejected — no Zvk VV decoder
//   - llvm-mc differential: rejected — LLVM 15.0.6 does not recognize vsm3me.vv
//   - encode_v_arith_vv as differential sibling: rejected — same-job gate
//     (OPIVV funct3=000 / OP-V, not crypto VV funct3=010)
// Weaker available: algebraic.invariant (field unpack), algebraic.metamorphic
//   (field isolation, operand swap), negative_error (arity / bad regs / extra / v0.t)
// Reference: candidate=encode_v_crypto_vv,
//   reference=RISC-V Cryptography Extensions Volume II (claimed by README + OP_V_CRYPTO comment),
//   SUT-boundary=internal-helper of RISC-V assembler,
//   mapping=[Reg(vd), Reg(vs2), Reg(vs1)] + funct6
//     <-> `vsm3me.vv vd, vs2, vs1`.

use super::{encode_v_crypto_vv, EncodeResult};
use crate::backend::riscv::assembler::parser::Operand;
use proptest::prelude::*;

const CASES: u32 = 1000;
const OP_V: u32 = 0b1010111;
const OP_V_CRYPTO: u32 = 0b1110111;
const FUNCT3_CRYPTO_VV: u32 = 0b010;
const FUNCT6_VSM3ME_VV: u32 = 0b100000;

fn cfg() -> ProptestConfig {
    ProptestConfig::with_cases(CASES)
}

fn vn(n: u32) -> String {
    format!("v{n}")
}

fn vreg(n: u32) -> Operand {
    Operand::Reg(vn(n))
}

fn ops3(vd: u32, vs2: u32, vs1: u32) -> Vec<Operand> {
    vec![vreg(vd), vreg(vs2), vreg(vs1)]
}

fn sut_word(ops: &[Operand], funct6: u32) -> Result<u32, String> {
    match encode_v_crypto_vv(ops, funct6)? {
        EncodeResult::Word(w) => Ok(w),
        other => Err(format!("expected Word, got {other:?}")),
    }
}

/// Unpack crypto VV per RISC-V Vector Crypto (not a copy of encode_v_crypto_vv).
fn unpack_crypto_vv(word: u32) -> (u32, u32, u32, u32, u32, u32, u32) {
    let opcode = word & 0x7f;
    let vd = (word >> 7) & 0x1f;
    let funct3 = (word >> 12) & 0x7;
    let vs1 = (word >> 15) & 0x1f;
    let vs2 = (word >> 20) & 0x1f;
    let vm = (word >> 25) & 1;
    let funct6 = word >> 26;
    (opcode, vd, funct3, vs1, vs2, vm, funct6)
}

fn spec_word(vd: u32, vs2: u32, vs1: u32, funct6: u32) -> u32 {
    (funct6 << 26)
        | (1u32 << 25)
        | (vs2 << 20)
        | (vs1 << 15)
        | (FUNCT3_CRYPTO_VV << 12)
        | (vd << 7)
        | OP_V
}

fn vreg_n() -> impl Strategy<Value = u32> {
    0u32..=31
}

fn funct6_n() -> impl Strategy<Value = u32> {
    0u32..=63
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

fn short_ops() -> impl Strategy<Value = Vec<Operand>> {
    prop_oneof![
        Just(vec![]),
        vreg_n().prop_map(|vd| vec![vreg(vd)]),
        (vreg_n(), vreg_n()).prop_map(|(a, b)| vec![vreg(a), vreg(b)]),
    ]
}

/// Known-answer gate: rustdoc packing with OP_V_CRYPTO (proves the symbol is linked).
#[test]
fn encode_v_crypto_vv_kat_vsm3me_v0_v0_v0() {
    let want = 0x82002077u32;
    let sut = sut_word(&ops3(0, 0, 0), FUNCT6_VSM3ME_VV).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_v_crypto_vv_kat_vsm3me_v1_v2_v3() {
    let want = 0x8221a0f7u32;
    let sut = sut_word(&ops3(1, 2, 3), FUNCT6_VSM3ME_VV).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_v_crypto_vv_kat_vsm3me_v31_v30_v29() {
    let want = 0x83eeaff7u32;
    let sut = sut_word(&ops3(31, 30, 29), FUNCT6_VSM3ME_VV).expect("SUT KAT");
    assert_eq!(sut, want);
}

/// Reference KAT: Volume II uses OP-V (1010111), not OP-P.
#[test]
fn encode_v_crypto_vv_kat_spec_opcode_vsm3me_v1_v2_v3() {
    let want = spec_word(1, 2, 3, FUNCT6_VSM3ME_VV);
    assert_eq!(want, 0x8221a0d7u32, "spec encoding drifted");
    let sut = sut_word(&ops3(1, 2, 3), FUNCT6_VSM3ME_VV).expect("SUT KAT");
    assert_eq!(
        sut, want,
        "SUT {:#010x} != Volume II OP-V {:#010x} for vsm3me.vv v1, v2, v3",
        sut, want
    );
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_v_crypto_vv_spec_opcode(
        vd in vreg_n(),
        vs2 in vreg_n(),
        vs1 in vreg_n(),
    ) {
        let sut = sut_word(&ops3(vd, vs2, vs1), FUNCT6_VSM3ME_VV)
            .unwrap_or_else(|e| panic!("SUT rejected valid vd={vd} vs2={vs2} vs1={vs1}: {e}"));
        prop_assert_eq!(
            sut & 0x7f,
            OP_V,
            "opcode must be OP-V 1010111 per RISC-V Crypto Volume II; got {:#09b}",
            sut & 0x7f
        );
        prop_assert_eq!(
            sut,
            spec_word(vd, vs2, vs1, FUNCT6_VSM3ME_VV),
            "SUT {:#010x} != Volume II {:#010x}",
            sut,
            spec_word(vd, vs2, vs1, FUNCT6_VSM3ME_VV)
        );
    }

    #[test]
    fn encode_v_crypto_vv_format_fields(
        vd in vreg_n(),
        vs2 in vreg_n(),
        vs1 in vreg_n(),
        funct6 in funct6_n(),
    ) {
        let w = sut_word(&ops3(vd, vs2, vs1), funct6)
            .unwrap_or_else(|e| panic!("SUT rejected: {e}"));
        let (opcode, got_vd, funct3, got_vs1, got_vs2, vm, got_f6) = unpack_crypto_vv(w);
        prop_assert_eq!(got_vd, vd, "vd");
        prop_assert_eq!(funct3, FUNCT3_CRYPTO_VV, "funct3 must be 010");
        prop_assert_eq!(got_vs1, vs1, "vs1");
        prop_assert_eq!(got_vs2, vs2, "vs2");
        prop_assert_eq!(vm, 1, "vm must be 1 (unmasked / not maskable)");
        prop_assert_eq!(got_f6, funct6, "funct6");
        // Opcode is claimed as OP_V_CRYPTO in rustdoc; Volume II says OP-V.
        // Record both so a mismatch is visible without hiding packing.
        let _ = opcode;
        let _ = OP_V_CRYPTO;
    }

    #[test]
    fn encode_v_crypto_vv_field_isolation(
        vd_a in vreg_n(),
        vd_b in vreg_n(),
        vs2_a in vreg_n(),
        vs2_b in vreg_n(),
        vs1_a in vreg_n(),
        vs1_b in vreg_n(),
        f6_a in funct6_n(),
        f6_b in funct6_n(),
    ) {
        let wa = sut_word(&ops3(vd_a, vs2_a, vs1_a), f6_a)
            .unwrap_or_else(|e| panic!("a rejected: {e}"));
        let wb = sut_word(&ops3(vd_b, vs2_a, vs1_a), f6_a)
            .unwrap_or_else(|e| panic!("b rejected: {e}"));
        let vd_mask = 0x1fu32 << 7;
        prop_assert_eq!(wa & !vd_mask, wb & !vd_mask, "non-vd bits independent of vd");
        prop_assert_eq!((wa >> 7) & 0x1f, vd_a);
        prop_assert_eq!((wb >> 7) & 0x1f, vd_b);

        let wc = sut_word(&ops3(vd_a, vs2_b, vs1_a), f6_a)
            .unwrap_or_else(|e| panic!("c rejected: {e}"));
        let vs2_mask = 0x1fu32 << 20;
        prop_assert_eq!(wa & !vs2_mask, wc & !vs2_mask, "non-vs2 bits independent of vs2");
        prop_assert_eq!((wc >> 20) & 0x1f, vs2_b);

        let wd = sut_word(&ops3(vd_a, vs2_a, vs1_b), f6_a)
            .unwrap_or_else(|e| panic!("d rejected: {e}"));
        let vs1_mask = 0x1fu32 << 15;
        prop_assert_eq!(wa & !vs1_mask, wd & !vs1_mask, "non-vs1 bits independent of vs1");
        prop_assert_eq!((wd >> 15) & 0x1f, vs1_b);

        let we = sut_word(&ops3(vd_a, vs2_a, vs1_a), f6_b)
            .unwrap_or_else(|e| panic!("e rejected: {e}"));
        let f6_mask = 0x3fu32 << 26;
        prop_assert_eq!(wa & !f6_mask, we & !f6_mask, "non-funct6 bits independent of funct6");
        prop_assert_eq!(we >> 26, f6_b);
    }

    #[test]
    fn encode_v_crypto_vv_operand_swap(
        vd in vreg_n(),
        vs2 in vreg_n(),
        vs1 in vreg_n(),
        funct6 in funct6_n(),
    ) {
        let w = sut_word(&ops3(vd, vs2, vs1), funct6)
            .unwrap_or_else(|e| panic!("w rejected: {e}"));
        let w_vd_vs2 = sut_word(&ops3(vs2, vd, vs1), funct6)
            .unwrap_or_else(|e| panic!("vd/vs2 swap rejected: {e}"));
        let vd_mask = 0x1fu32 << 7;
        let vs2_mask = 0x1fu32 << 20;
        let vs1_mask = 0x1fu32 << 15;
        prop_assert_eq!((w >> 7) & 0x1f, vd);
        prop_assert_eq!((w >> 20) & 0x1f, vs2);
        prop_assert_eq!((w >> 15) & 0x1f, vs1);
        prop_assert_eq!((w_vd_vs2 >> 7) & 0x1f, vs2);
        prop_assert_eq!((w_vd_vs2 >> 20) & 0x1f, vd);
        prop_assert_eq!(
            w & !(vd_mask | vs2_mask),
            w_vd_vs2 & !(vd_mask | vs2_mask),
            "non vd/vs2 bits must be preserved under operand 0/1 swap"
        );

        let w_vs2_vs1 = sut_word(&ops3(vd, vs1, vs2), funct6)
            .unwrap_or_else(|e| panic!("vs2/vs1 swap rejected: {e}"));
        prop_assert_eq!((w_vs2_vs1 >> 20) & 0x1f, vs1);
        prop_assert_eq!((w_vs2_vs1 >> 15) & 0x1f, vs2);
        prop_assert_eq!(
            w & !(vs2_mask | vs1_mask),
            w_vs2_vs1 & !(vs2_mask | vs1_mask),
            "non vs2/vs1 bits must be preserved under operand 1/2 swap"
        );
    }

    #[test]
    fn encode_v_crypto_vv_neg_arity_bad_regs(
        ops in short_ops(),
        bad_v in bad_vreg(),
        pos in 0usize..=2,
        funct6 in funct6_n(),
    ) {
        prop_assert!(
            encode_v_crypto_vv(&ops, funct6).is_err(),
            "arity {} must Err; got {:?}",
            ops.len(),
            encode_v_crypto_vv(&ops, funct6)
        );
        let mut bad_ops = ops3(0, 1, 2);
        bad_ops[pos] = bad_v.clone();
        prop_assert!(
            encode_v_crypto_vv(&bad_ops, funct6).is_err(),
            "non-vector operand {:?} at pos {pos} must Err; got {:?}",
            bad_v,
            encode_v_crypto_vv(&bad_ops, funct6)
        );
    }

    #[test]
    fn encode_v_crypto_vv_neg_extra(
        vd in vreg_n(),
        vs2 in vreg_n(),
        vs1 in vreg_n(),
        extra in extra_operand(),
    ) {
        let mut ops = ops3(vd, vs2, vs1);
        ops.push(extra.clone());
        prop_assert!(
            encode_v_crypto_vv(&ops, FUNCT6_VSM3ME_VV).is_err(),
            "extra operand {:?} must Err for crypto VV; got {:?}",
            extra,
            encode_v_crypto_vv(&ops, FUNCT6_VSM3ME_VV)
        );
    }

    #[test]
    fn encode_v_crypto_vv_neg_mask_v0t(
        vd in vreg_n(),
        vs2 in vreg_n(),
        vs1 in vreg_n(),
    ) {
        let mut ops = ops3(vd, vs2, vs1);
        ops.push(Operand::Symbol("v0.t".into()));
        prop_assert!(
            encode_v_crypto_vv(&ops, FUNCT6_VSM3ME_VV).is_err(),
            "trailing v0.t must Err (Zvksh VV is not maskable); got {:?}",
            encode_v_crypto_vv(&ops, FUNCT6_VSM3ME_VV)
        );
    }
}

/// Regression: extra operand after a complete vd, vs2, vs1 crypto VV must Err.
#[test]
fn test_encode_v_crypto_vv_regression_extra_operand() {
    let mut ops = ops3(0, 0, 0);
    ops.push(Operand::Imm(0));
    let got = encode_v_crypto_vv(&ops, FUNCT6_VSM3ME_VV);
    assert!(
        got.is_err(),
        "vsm3me.vv v0, v0, v0, 0 must Err; got {got:?}"
    );
}

/// Regression: trailing v0.t must Err (not maskable).
#[test]
fn test_encode_v_crypto_vv_regression_mask_v0t() {
    let mut ops = ops3(1, 2, 3);
    ops.push(Operand::Symbol("v0.t".into()));
    let got = encode_v_crypto_vv(&ops, FUNCT6_VSM3ME_VV);
    assert!(
        got.is_err(),
        "vsm3me.vv v1, v2, v3, v0.t must Err; got {got:?}"
    );
}

/// Regression: opcode must be OP-V 1010111 per Volume II.
#[test]
fn test_encode_v_crypto_vv_regression_spec_opcode() {
    let got = sut_word(&ops3(1, 2, 3), FUNCT6_VSM3ME_VV).expect("encode");
    assert_eq!(
        got & 0x7f,
        OP_V,
        "vsm3me.vv opcode must be OP-V; got {:#09b}",
        got & 0x7f
    );
}
