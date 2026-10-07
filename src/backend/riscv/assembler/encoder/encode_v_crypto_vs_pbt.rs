// Oracle: reference — RISC-V Cryptography Extensions Volume II (Zvksed VS)
// Evidence: vector.rs:209-210 "Encode Zvksed crypto instructions with VS format
//   vsm4r.vs: funct6 | vm=1 | vs2 | 10000 | 010 | vd | OP_V_CRYPTO";
//   encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words";
//   encoder/mod.rs:449 OP_V_CRYPTO = 0b1110111 "Vector crypto (Zvk*) — uses OP-P
//     encoding space per RVV Crypto spec";
//   encoder/mod.rs:1026 "vsm4r.vs" => encode_v_crypto_vs(operands, 0b101001);
//   assembler/README.md:14 Zvksh/Zvksed; assembler/README.md:109 vector.rs;
//   RISC-V Cryptography Extensions Volume II: opcode=1010111 (OP-V), funct3=010,
//   vm=1 (not maskable), vd, vs1=10000, vs2, funct6 vsm4r.vs=101001.
// Stronger considered:
//   - State machine: rejected — encode_v_crypto_vs is a pure function with no lifecycle
//   - Algebraic round-trip via in-tree decoder: rejected — no Zvk VS decoder
//   - llvm-mc differential: rejected — LLVM 15.0.6 does not recognize vsm4r.vs
//   - encode_v_arith_vv as differential sibling: rejected — same-job gate
//     (OPIVV funct3=000 / OP-V with vs1 a register, not crypto VS funct3=010 / vs1=10000)
// Weaker available: algebraic.invariant (field unpack), algebraic.metamorphic
//   (field isolation, operand swap), negative_error (arity / bad regs / extra / v0.t)
// Reference: candidate=encode_v_crypto_vs,
//   reference=RISC-V Cryptography Extensions Volume II (claimed by README + OP_V_CRYPTO comment),
//   SUT-boundary=internal-helper of RISC-V assembler,
//   mapping=[Reg(vd), Reg(vs2)] + funct6
//     <-> `vsm4r.vs vd, vs2`.

use super::{encode_v_crypto_vs, EncodeResult};
use crate::backend::riscv::assembler::parser::Operand;
use proptest::prelude::*;

const CASES: u32 = 1000;
const OP_V: u32 = 0b1010111;
const OP_V_CRYPTO: u32 = 0b1110111;
const FUNCT3_CRYPTO_VS: u32 = 0b010;
const FUNCT6_VSM4R_VS: u32 = 0b101001;
const VS1_VSM4R_VS: u32 = 0b10000;

fn cfg() -> ProptestConfig {
    ProptestConfig::with_cases(CASES)
}

fn vn(n: u32) -> String {
    format!("v{n}")
}

fn vreg(n: u32) -> Operand {
    Operand::Reg(vn(n))
}

fn ops2(vd: u32, vs2: u32) -> Vec<Operand> {
    vec![vreg(vd), vreg(vs2)]
}

fn sut_word(ops: &[Operand], funct6: u32) -> Result<u32, String> {
    match encode_v_crypto_vs(ops, funct6)? {
        EncodeResult::Word(w) => Ok(w),
        other => Err(format!("expected Word, got {other:?}")),
    }
}

/// Unpack crypto VS per RISC-V Vector Crypto (not a copy of encode_v_crypto_vs).
fn unpack_crypto_vs(word: u32) -> (u32, u32, u32, u32, u32, u32, u32) {
    let opcode = word & 0x7f;
    let vd = (word >> 7) & 0x1f;
    let funct3 = (word >> 12) & 0x7;
    let vs1 = (word >> 15) & 0x1f;
    let vs2 = (word >> 20) & 0x1f;
    let vm = (word >> 25) & 1;
    let funct6 = word >> 26;
    (opcode, vd, funct3, vs1, vs2, vm, funct6)
}

fn spec_word(vd: u32, vs2: u32, funct6: u32) -> u32 {
    (funct6 << 26)
        | (1u32 << 25)
        | (vs2 << 20)
        | (VS1_VSM4R_VS << 15)
        | (FUNCT3_CRYPTO_VS << 12)
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
    ]
}

/// Known-answer gate: rustdoc packing with OP_V_CRYPTO (proves the symbol is linked).
#[test]
fn encode_v_crypto_vs_kat_vsm4r_v0_v0() {
    let want = 0xa6082077u32;
    let sut = sut_word(&ops2(0, 0), FUNCT6_VSM4R_VS).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_v_crypto_vs_kat_vsm4r_v1_v2() {
    let want = 0xa62820f7u32;
    let sut = sut_word(&ops2(1, 2), FUNCT6_VSM4R_VS).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_v_crypto_vs_kat_vsm4r_v31_v30() {
    let want = 0xa7e82ff7u32;
    let sut = sut_word(&ops2(31, 30), FUNCT6_VSM4R_VS).expect("SUT KAT");
    assert_eq!(sut, want);
}

/// Reference KAT: Volume II uses OP-V (1010111), not OP-P.
#[test]
fn encode_v_crypto_vs_kat_spec_opcode_vsm4r_v1_v2() {
    let want = spec_word(1, 2, FUNCT6_VSM4R_VS);
    assert_eq!(want, 0xa62820d7u32, "spec encoding drifted");
    let sut = sut_word(&ops2(1, 2), FUNCT6_VSM4R_VS).expect("SUT KAT");
    assert_eq!(
        sut, want,
        "SUT {:#010x} != Volume II OP-V {:#010x} for vsm4r.vs v1, v2",
        sut, want
    );
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_v_crypto_vs_spec_opcode(
        vd in vreg_n(),
        vs2 in vreg_n(),
    ) {
        let sut = sut_word(&ops2(vd, vs2), FUNCT6_VSM4R_VS)
            .unwrap_or_else(|e| panic!("SUT rejected valid vd={vd} vs2={vs2}: {e}"));
        prop_assert_eq!(
            sut & 0x7f,
            OP_V,
            "opcode must be OP-V 1010111 per RISC-V Crypto Volume II; got {:#09b}",
            sut & 0x7f
        );
        prop_assert_eq!(
            sut,
            spec_word(vd, vs2, FUNCT6_VSM4R_VS),
            "SUT {:#010x} != Volume II {:#010x}",
            sut,
            spec_word(vd, vs2, FUNCT6_VSM4R_VS)
        );
    }

    #[test]
    fn encode_v_crypto_vs_format_fields(
        vd in vreg_n(),
        vs2 in vreg_n(),
        funct6 in funct6_n(),
    ) {
        let w = sut_word(&ops2(vd, vs2), funct6)
            .unwrap_or_else(|e| panic!("SUT rejected: {e}"));
        let (opcode, got_vd, funct3, got_vs1, got_vs2, vm, got_f6) = unpack_crypto_vs(w);
        prop_assert_eq!(got_vd, vd, "vd");
        prop_assert_eq!(funct3, FUNCT3_CRYPTO_VS, "funct3 must be 010");
        prop_assert_eq!(got_vs1, VS1_VSM4R_VS, "vs1 must be 10000");
        prop_assert_eq!(got_vs2, vs2, "vs2");
        prop_assert_eq!(vm, 1, "vm must be 1 (unmasked / not maskable)");
        prop_assert_eq!(got_f6, funct6, "funct6");
        let _ = opcode;
        let _ = OP_V_CRYPTO;
    }

    #[test]
    fn encode_v_crypto_vs_field_isolation(
        vd_a in vreg_n(),
        vd_b in vreg_n(),
        vs2_a in vreg_n(),
        vs2_b in vreg_n(),
        f6_a in funct6_n(),
        f6_b in funct6_n(),
    ) {
        let wa = sut_word(&ops2(vd_a, vs2_a), f6_a)
            .unwrap_or_else(|e| panic!("a rejected: {e}"));
        let wb = sut_word(&ops2(vd_b, vs2_a), f6_a)
            .unwrap_or_else(|e| panic!("b rejected: {e}"));
        let vd_mask = 0x1fu32 << 7;
        prop_assert_eq!(wa & !vd_mask, wb & !vd_mask, "non-vd bits independent of vd");
        prop_assert_eq!((wa >> 7) & 0x1f, vd_a);
        prop_assert_eq!((wb >> 7) & 0x1f, vd_b);

        let wc = sut_word(&ops2(vd_a, vs2_b), f6_a)
            .unwrap_or_else(|e| panic!("c rejected: {e}"));
        let vs2_mask = 0x1fu32 << 20;
        prop_assert_eq!(wa & !vs2_mask, wc & !vs2_mask, "non-vs2 bits independent of vs2");
        prop_assert_eq!((wc >> 20) & 0x1f, vs2_b);

        let we = sut_word(&ops2(vd_a, vs2_a), f6_b)
            .unwrap_or_else(|e| panic!("e rejected: {e}"));
        let f6_mask = 0x3fu32 << 26;
        prop_assert_eq!(wa & !f6_mask, we & !f6_mask, "non-funct6 bits independent of funct6");
        prop_assert_eq!(we >> 26, f6_b);

        let vs1_mask = 0x1fu32 << 15;
        prop_assert_eq!((wa & vs1_mask) >> 15, VS1_VSM4R_VS, "vs1 stays 10000");
        prop_assert_eq!((wb & vs1_mask) >> 15, VS1_VSM4R_VS, "vs1 stays 10000 after vd change");
        prop_assert_eq!((wc & vs1_mask) >> 15, VS1_VSM4R_VS, "vs1 stays 10000 after vs2 change");
        prop_assert_eq!((we & vs1_mask) >> 15, VS1_VSM4R_VS, "vs1 stays 10000 after funct6 change");
    }

    #[test]
    fn encode_v_crypto_vs_operand_swap(
        vd in vreg_n(),
        vs2 in vreg_n(),
        funct6 in funct6_n(),
    ) {
        let w = sut_word(&ops2(vd, vs2), funct6)
            .unwrap_or_else(|e| panic!("w rejected: {e}"));
        let w_vd_vs2 = sut_word(&ops2(vs2, vd), funct6)
            .unwrap_or_else(|e| panic!("vd/vs2 swap rejected: {e}"));
        let vd_mask = 0x1fu32 << 7;
        let vs2_mask = 0x1fu32 << 20;
        prop_assert_eq!((w >> 7) & 0x1f, vd);
        prop_assert_eq!((w >> 20) & 0x1f, vs2);
        prop_assert_eq!((w_vd_vs2 >> 7) & 0x1f, vs2);
        prop_assert_eq!((w_vd_vs2 >> 20) & 0x1f, vd);
        prop_assert_eq!(
            w & !(vd_mask | vs2_mask),
            w_vd_vs2 & !(vd_mask | vs2_mask),
            "non vd/vs2 bits must be preserved under operand 0/1 swap"
        );
        prop_assert_eq!((w >> 15) & 0x1f, VS1_VSM4R_VS);
        prop_assert_eq!((w_vd_vs2 >> 15) & 0x1f, VS1_VSM4R_VS);
    }

    #[test]
    fn encode_v_crypto_vs_neg_arity_bad_regs(
        ops in short_ops(),
        bad_v in bad_vreg(),
        pos in 0usize..=1,
        funct6 in funct6_n(),
    ) {
        prop_assert!(
            encode_v_crypto_vs(&ops, funct6).is_err(),
            "arity {} must Err; got {:?}",
            ops.len(),
            encode_v_crypto_vs(&ops, funct6)
        );
        let mut bad_ops = ops2(0, 1);
        bad_ops[pos] = bad_v.clone();
        prop_assert!(
            encode_v_crypto_vs(&bad_ops, funct6).is_err(),
            "non-vector operand {:?} at pos {pos} must Err; got {:?}",
            bad_v,
            encode_v_crypto_vs(&bad_ops, funct6)
        );
    }

    #[test]
    fn encode_v_crypto_vs_neg_extra(
        vd in vreg_n(),
        vs2 in vreg_n(),
        extra in extra_operand(),
    ) {
        let mut ops = ops2(vd, vs2);
        ops.push(extra.clone());
        prop_assert!(
            encode_v_crypto_vs(&ops, FUNCT6_VSM4R_VS).is_err(),
            "extra operand {:?} must Err for crypto VS; got {:?}",
            extra,
            encode_v_crypto_vs(&ops, FUNCT6_VSM4R_VS)
        );
    }

    #[test]
    fn encode_v_crypto_vs_neg_mask_v0t(
        vd in vreg_n(),
        vs2 in vreg_n(),
    ) {
        let mut ops = ops2(vd, vs2);
        ops.push(Operand::Symbol("v0.t".into()));
        prop_assert!(
            encode_v_crypto_vs(&ops, FUNCT6_VSM4R_VS).is_err(),
            "trailing v0.t must Err (Zvksed VS is not maskable); got {:?}",
            encode_v_crypto_vs(&ops, FUNCT6_VSM4R_VS)
        );
    }
}

/// Regression: extra operand after a complete vd, vs2 crypto VS must Err.
#[test]
fn test_encode_v_crypto_vs_regression_extra_operand() {
    let mut ops = ops2(0, 0);
    ops.push(Operand::Imm(0));
    let got = encode_v_crypto_vs(&ops, FUNCT6_VSM4R_VS);
    assert!(
        got.is_err(),
        "vsm4r.vs v0, v0, 0 must Err; got {got:?}"
    );
}

/// Regression: trailing v0.t must Err (not maskable).
#[test]
fn test_encode_v_crypto_vs_regression_mask_v0t() {
    let mut ops = ops2(1, 2);
    ops.push(Operand::Symbol("v0.t".into()));
    let got = encode_v_crypto_vs(&ops, FUNCT6_VSM4R_VS);
    assert!(
        got.is_err(),
        "vsm4r.vs v1, v2, v0.t must Err; got {got:?}"
    );
}

/// Regression: opcode must be OP-V 1010111 per Volume II.
#[test]
fn test_encode_v_crypto_vs_regression_spec_opcode() {
    let got = sut_word(&ops2(1, 2), FUNCT6_VSM4R_VS).expect("encode");
    assert_eq!(
        got & 0x7f,
        OP_V,
        "vsm4r.vs opcode must be OP-V; got {:#09b}",
        got & 0x7f
    );
}
