// Oracle: differential — llvm-mc RISC-V assembler (RVV OPIVI vector-immediate arith)
// Evidence: vector.rs:143-150 "Encode vector arithmetic VI (vector-immediate):
//   funct6[31:26] | vm[25] | vs2[24:20] | simm5[19:15] | funct3[14:12]=011 | vd[11:7] | OP_V";
//   encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words";
//   encoder/mod.rs:980-997 vadd.vi/vand.vi/vor.vi/vxor.vi/vslideup.vi/vslidedown.vi
//     => encode_v_arith_vi(operands, funct6);
//   assembler/README.md:14 V (vector) standard extension; assembler/README.md:109 vector.rs RVV;
//   RISC-V V 1.0 OPIVI: opcode=1010111, funct3=011, vd, simm5, vs2, vm=1 unmasked, funct6.
// Stronger considered:
//   - State machine: rejected — encode_v_arith_vi is a pure function with no lifecycle
//   - Algebraic round-trip via in-tree decoder: rejected — no OPIVI decoder
//   - encode_v_arith_vv / encode_v_arith_vx as differential siblings: rejected —
//     same-job gate (OPIVV funct3=000 / OPIVX funct3=100)
// Weaker available: algebraic.invariant (field unpack, simm5 two's complement),
//   algebraic.metamorphic (field isolation), negative_error (arity / bad regs / extra / oob)
// Differential: candidate=encode_v_arith_vi,
//   reference=llvm-mc -triple=riscv64 -mattr=+v -show-encoding,
//   SUT-boundary=internal-helper of RISC-V assembler,
//   mapping=[Reg(vd), Reg(vs2), Imm(simm5)] + funct6
//     <-> `vadd.vi/vand.vi/vor.vi/vxor.vi vd, vs2, simm5`
//     or `vslideup.vi/vslidedown.vi vd, vs2, uimm5`.

use super::{encode_v_arith_vi, EncodeResult};
use crate::backend::riscv::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;
const OP_V: u32 = 0b1010111;
const FUNCT3_OPIVI: u32 = 0b011;

/// (mnemonic, funct6[31:26], signed_imm)
/// signed_imm=true  => simm5 in [-16, 15] (vadd/vand/vor/vxor.vi)
/// signed_imm=false => uimm5 in [0, 31]   (vslideup/vslidedown.vi)
const OPIVI_FAMILY: [(&str, u32, bool); 6] = [
    ("vadd.vi", 0b000000, true),
    ("vand.vi", 0b001001, true),
    ("vor.vi", 0b001010, true),
    ("vxor.vi", 0b001011, true),
    ("vslideup.vi", 0b001110, false),
    ("vslidedown.vi", 0b001111, false),
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
    match encode_v_arith_vi(ops, funct6)? {
        EncodeResult::Word(w) => Ok(w),
        other => Err(format!("expected Word, got {other:?}")),
    }
}

/// Unpack OPIVI per RISC-V V 1.0 (not a copy of encode_v_arith_vi).
fn unpack_opivi(word: u32) -> (u32, u32, u32, u32, u32, u32, u32) {
    let opcode = word & 0x7f;
    let vd = (word >> 7) & 0x1f;
    let funct3 = (word >> 12) & 0x7;
    let simm5 = (word >> 15) & 0x1f;
    let vs2 = (word >> 20) & 0x1f;
    let vm = (word >> 25) & 1;
    let funct6 = word >> 26;
    (opcode, vd, funct3, simm5, vs2, vm, funct6)
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
        .args(["-triple=riscv64", "-mattr=+v", "-show-encoding"])
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

fn vreg_n() -> impl Strategy<Value = u32> {
    0u32..=31
}

fn vd_nonzero() -> impl Strategy<Value = u32> {
    1u32..=31
}

fn funct6_n() -> impl Strategy<Value = u32> {
    0u32..=63
}

fn opivi_kind() -> impl Strategy<Value = usize> {
    0usize..=5
}

fn simm5() -> impl Strategy<Value = i64> {
    prop_oneof![
        Just(-16i64),
        Just(-1i64),
        Just(0i64),
        Just(15i64),
        -16i64..=15,
    ]
}

fn uimm5() -> impl Strategy<Value = i64> {
    prop_oneof![
        Just(0i64),
        Just(16i64),
        Just(31i64),
        0i64..=31,
    ]
}

fn extra_operand() -> impl Strategy<Value = Operand> {
    prop_oneof![
        Just(Operand::Imm(0)),
        Just(Operand::Imm(-1)),
        Just(Operand::Imm(208)),
        (0u32..=31).prop_map(|n| Operand::Reg(format!("x{n}"))),
        (0u32..=31).prop_map(|n| Operand::Reg(vn(n))),
        Just(Operand::Symbol("foo".into())),
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

fn signed_oob() -> impl Strategy<Value = i64> {
    prop_oneof![
        Just(-17i64),
        Just(16i64),
        Just(31i64),
        Just(32i64),
        Just(i64::MIN),
        Just(i64::MAX),
        -1000i64..=-17,
        16i64..=1000,
    ]
}

fn slide_oob() -> impl Strategy<Value = i64> {
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

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_v_arith_vi_kat_llvm_mc_vadd_v0_v0_0() {
    let want = 0x02003057u32;
    let mc = llvm_mc_word("vadd.vi v0, v0, 0").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT drifted: {mc:#x} != {want:#x}");
    let sut = sut_word(&ops3(0, 0, 0), 0b000000).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_v_arith_vi_kat_llvm_mc_vadd_v1_v2_3() {
    let want = 0x0221b0d7u32;
    let mc = llvm_mc_word("vadd.vi v1, v2, 3").expect("llvm-mc KAT");
    assert_eq!(mc, want);
    let sut = sut_word(&ops3(1, 2, 3), 0b000000).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_v_arith_vi_kat_llvm_mc_vadd_v31_v30_15() {
    let want = 0x03e7bfd7u32;
    let mc = llvm_mc_word("vadd.vi v31, v30, 15").expect("llvm-mc KAT");
    assert_eq!(mc, want);
    let sut = sut_word(&ops3(31, 30, 15), 0b000000).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_v_arith_vi_kat_llvm_mc_vadd_v1_v2_m1() {
    let want = 0x022fb0d7u32;
    let mc = llvm_mc_word("vadd.vi v1, v2, -1").expect("llvm-mc KAT");
    assert_eq!(mc, want);
    let sut = sut_word(&ops3(1, 2, -1), 0b000000).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_v_arith_vi_kat_llvm_mc_vadd_v1_v2_m16() {
    let want = 0x022830d7u32;
    let mc = llvm_mc_word("vadd.vi v1, v2, -16").expect("llvm-mc KAT");
    assert_eq!(mc, want);
    let sut = sut_word(&ops3(1, 2, -16), 0b000000).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_v_arith_vi_kat_llvm_mc_vand_v1_v2_3() {
    let want = 0x2621b0d7u32;
    let mc = llvm_mc_word("vand.vi v1, v2, 3").expect("llvm-mc KAT");
    assert_eq!(mc, want);
    let sut = sut_word(&ops3(1, 2, 3), 0b001001).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_v_arith_vi_kat_llvm_mc_vor_v1_v2_3() {
    let want = 0x2a21b0d7u32;
    let mc = llvm_mc_word("vor.vi v1, v2, 3").expect("llvm-mc KAT");
    assert_eq!(mc, want);
    let sut = sut_word(&ops3(1, 2, 3), 0b001010).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_v_arith_vi_kat_llvm_mc_vxor_v1_v2_3() {
    let want = 0x2e21b0d7u32;
    let mc = llvm_mc_word("vxor.vi v1, v2, 3").expect("llvm-mc KAT");
    assert_eq!(mc, want);
    let sut = sut_word(&ops3(1, 2, 3), 0b001011).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_v_arith_vi_kat_llvm_mc_vslideup_v1_v2_3() {
    let want = 0x3a21b0d7u32;
    let mc = llvm_mc_word("vslideup.vi v1, v2, 3").expect("llvm-mc KAT");
    assert_eq!(mc, want);
    let sut = sut_word(&ops3(1, 2, 3), 0b001110).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_v_arith_vi_kat_llvm_mc_vslidedown_v1_v2_3() {
    let want = 0x3e21b0d7u32;
    let mc = llvm_mc_word("vslidedown.vi v1, v2, 3").expect("llvm-mc KAT");
    assert_eq!(mc, want);
    let sut = sut_word(&ops3(1, 2, 3), 0b001111).expect("SUT KAT");
    assert_eq!(sut, want);
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_v_arith_vi_diff_llvm_mc(
        vd in vreg_n(),
        vs2 in vreg_n(),
        simm in simm5(),
        uimm in uimm5(),
        kind in opivi_kind(),
    ) {
        let (mnem, funct6, signed) = OPIVI_FAMILY[kind];
        let imm = if signed { simm } else { uimm };
        // llvm-mc refuses vslideup when vd overlaps vs2 (architectural
        // register-group constraint, not an encoding-layout rule). Skip those.
        prop_assume!(!(mnem == "vslideup.vi" && vd == vs2));
        let asm = format!("{mnem} v{vd}, v{vs2}, {imm}");
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {asm}: {e}"));
        let sut = sut_word(&ops3(vd, vs2, imm), funct6)
            .unwrap_or_else(|e| panic!("SUT rejected valid {asm}: {e}"));
        prop_assert_eq!(sut, mc, "SUT {:#010x} != llvm-mc {:#010x} for {}", sut, mc, asm);
    }

    #[test]
    fn encode_v_arith_vi_format_fields(
        vd in vreg_n(),
        vs2 in vreg_n(),
        simm in simm5(),
        funct6 in funct6_n(),
    ) {
        let w = sut_word(&ops3(vd, vs2, simm), funct6)
            .unwrap_or_else(|e| panic!("SUT rejected: {e}"));
        let (opcode, got_vd, funct3, got_simm, got_vs2, vm, got_f6) = unpack_opivi(w);
        prop_assert_eq!(opcode, OP_V, "opcode");
        prop_assert_eq!(got_vd, vd, "vd");
        prop_assert_eq!(funct3, FUNCT3_OPIVI, "funct3 must be 011 (OPIVI)");
        prop_assert_eq!(got_simm, (simm as u32) & 0x1F, "simm5");
        prop_assert_eq!(got_vs2, vs2, "vs2");
        prop_assert_eq!(vm, 1, "vm must be 1 (unmasked) for three-operand form");
        prop_assert_eq!(got_f6, funct6, "funct6");
    }

    #[test]
    fn encode_v_arith_vi_field_isolation(
        vd_a in vreg_n(),
        vd_b in vreg_n(),
        vs2_a in vreg_n(),
        vs2_b in vreg_n(),
        simm_a in simm5(),
        simm_b in simm5(),
        f6_a in funct6_n(),
        f6_b in funct6_n(),
    ) {
        let wa = sut_word(&ops3(vd_a, vs2_a, simm_a), f6_a)
            .unwrap_or_else(|e| panic!("a rejected: {e}"));
        let wb = sut_word(&ops3(vd_b, vs2_a, simm_a), f6_a)
            .unwrap_or_else(|e| panic!("b rejected: {e}"));
        let vd_mask = 0x1fu32 << 7;
        prop_assert_eq!(wa & !vd_mask, wb & !vd_mask, "non-vd bits independent of vd");
        prop_assert_eq!((wa >> 7) & 0x1f, vd_a);
        prop_assert_eq!((wb >> 7) & 0x1f, vd_b);

        let wc = sut_word(&ops3(vd_a, vs2_b, simm_a), f6_a)
            .unwrap_or_else(|e| panic!("c rejected: {e}"));
        let vs2_mask = 0x1fu32 << 20;
        prop_assert_eq!(wa & !vs2_mask, wc & !vs2_mask, "non-vs2 bits independent of vs2");
        prop_assert_eq!((wc >> 20) & 0x1f, vs2_b);

        let wd = sut_word(&ops3(vd_a, vs2_a, simm_b), f6_a)
            .unwrap_or_else(|e| panic!("d rejected: {e}"));
        let simm_mask = 0x1fu32 << 15;
        prop_assert_eq!(wa & !simm_mask, wd & !simm_mask, "non-simm5 bits independent of simm");
        prop_assert_eq!((wd >> 15) & 0x1f, (simm_b as u32) & 0x1F);

        let we = sut_word(&ops3(vd_a, vs2_a, simm_a), f6_b)
            .unwrap_or_else(|e| panic!("e rejected: {e}"));
        let f6_mask = 0x3fu32 << 26;
        prop_assert_eq!(wa & !f6_mask, we & !f6_mask, "non-funct6 bits independent of funct6");
        prop_assert_eq!(we >> 26, f6_b);
    }

    #[test]
    fn encode_v_arith_vi_simm5_twos_complement(
        vd in vreg_n(),
        vs2 in vreg_n(),
        simm in simm5(),
        funct6 in funct6_n(),
    ) {
        let w = sut_word(&ops3(vd, vs2, simm), funct6)
            .unwrap_or_else(|e| panic!("SUT rejected: {e}"));
        prop_assert_eq!(
            (w >> 15) & 0x1f,
            (simm as u32) & 0x1F,
            "simm5 two's complement bits[19:15] for {}",
            simm
        );
    }

    #[test]
    fn encode_v_arith_vi_neg_arity_bad_regs(
        ops in short_ops(),
        bad_v in bad_vreg(),
        bad_i in bad_imm(),
        pos in 0usize..=1,
        funct6 in funct6_n(),
    ) {
        prop_assert!(
            encode_v_arith_vi(&ops, funct6).is_err(),
            "arity {} must Err (llvm-mc too few operands); got {:?}",
            ops.len(),
            encode_v_arith_vi(&ops, funct6)
        );
        let mut bad_ops = ops3(0, 1, 0);
        bad_ops[pos] = bad_v.clone();
        prop_assert!(
            encode_v_arith_vi(&bad_ops, funct6).is_err(),
            "non-vector operand {:?} at pos {pos} must Err (llvm-mc invalid operand); got {:?}",
            bad_v,
            encode_v_arith_vi(&bad_ops, funct6)
        );
        let mut bad_imm_ops = ops3(0, 1, 0);
        bad_imm_ops[2] = bad_i.clone();
        prop_assert!(
            encode_v_arith_vi(&bad_imm_ops, funct6).is_err(),
            "non-Imm operand {:?} at simm5 must Err (llvm-mc invalid operand); got {:?}",
            bad_i,
            encode_v_arith_vi(&bad_imm_ops, funct6)
        );
    }

    #[test]
    fn encode_v_arith_vi_neg_extra(
        vd in vreg_n(),
        vs2 in vreg_n(),
        simm in simm5(),
        extra in extra_operand(),
        kind in opivi_kind(),
    ) {
        let (_, funct6, _) = OPIVI_FAMILY[kind];
        let mut ops = ops3(vd, vs2, simm);
        ops.push(extra.clone());
        prop_assert!(
            encode_v_arith_vi(&ops, funct6).is_err(),
            "extra operand {:?} must Err for OPIVI (llvm-mc rejects extra); got {:?}",
            extra,
            encode_v_arith_vi(&ops, funct6)
        );
    }

    #[test]
    fn encode_v_arith_vi_mask_v0t(
        vd in vd_nonzero(),
        vs2 in vreg_n(),
        simm in simm5(),
        uimm in uimm5(),
        kind in opivi_kind(),
    ) {
        let (mnem, funct6, signed) = OPIVI_FAMILY[kind];
        let imm = if signed { simm } else { uimm };
        prop_assume!(!(mnem == "vslideup.vi" && vd == vs2));
        let asm = format!("{mnem} v{vd}, v{vs2}, {imm}, v0.t");
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {asm}: {e}"));
        let mut ops = ops3(vd, vs2, imm);
        ops.push(Operand::Symbol("v0.t".into()));
        let sut = sut_word(&ops, funct6)
            .unwrap_or_else(|e| panic!("SUT rejected {asm}: {e}"));
        prop_assert_eq!(
            sut, mc,
            "SUT {:#010x} != llvm-mc {:#010x} for masked {}",
            sut, mc, asm
        );
    }

    #[test]
    fn encode_v_arith_vi_neg_imm_oob(
        vd in vreg_n(),
        vs2 in vreg_n(),
        s_oob in signed_oob(),
        u_oob in slide_oob(),
        kind in opivi_kind(),
    ) {
        let (mnem, funct6, signed) = OPIVI_FAMILY[kind];
        let imm = if signed { s_oob } else { u_oob };
        let ops = ops3(vd, vs2, imm);
        prop_assert!(
            encode_v_arith_vi(&ops, funct6).is_err(),
            "{mnem} v{vd}, v{vs2}, {imm} must Err (llvm-mc immediate out of range); got {:?}",
            encode_v_arith_vi(&ops, funct6)
        );
    }
}

/// Regression: extra operand after a complete vd, vs2, imm OPIVI must Err.
#[test]
fn test_encode_v_arith_vi_regression_extra_operand() {
    let mut ops = ops3(0, 0, 0);
    ops.push(Operand::Imm(0));
    let got = encode_v_arith_vi(&ops, 0b000000);
    assert!(
        got.is_err(),
        "vadd.vi v0, v0, 0, 0 must Err (llvm-mc invalid operand); got {got:?}"
    );
}

/// Regression: trailing v0.t must encode vm=0 to match llvm-mc.
#[test]
fn test_encode_v_arith_vi_regression_mask_v0t() {
    let mut ops = ops3(1, 2, 3);
    ops.push(Operand::Symbol("v0.t".into()));
    let got = sut_word(&ops, 0b000000);
    let want = 0x0021b0d7u32; // vadd.vi v1, v2, 3, v0.t
    assert_eq!(
        got.expect("SUT must encode masked form, not Err"),
        want,
        "vadd.vi v1, v2, 3, v0.t must equal llvm-mc 0x0021b0d7 (vm=0)"
    );
}

/// Regression: signed simm5 16 is out of [-16, 15] and must Err.
#[test]
fn test_encode_v_arith_vi_regression_simm_oob_16() {
    let got = encode_v_arith_vi(&ops3(1, 2, 16), 0b000000);
    assert!(
        got.is_err(),
        "vadd.vi v1, v2, 16 must Err (llvm-mc range [-16, 15]); got {got:?}"
    );
}

/// Regression: signed simm5 -17 is out of [-16, 15] and must Err.
#[test]
fn test_encode_v_arith_vi_regression_simm_oob_m17() {
    let got = encode_v_arith_vi(&ops3(1, 2, -17), 0b000000);
    assert!(
        got.is_err(),
        "vadd.vi v1, v2, -17 must Err (llvm-mc range [-16, 15]); got {got:?}"
    );
}

/// Regression: vslideup.vi uimm 32 is out of [0, 31] and must Err.
#[test]
fn test_encode_v_arith_vi_regression_uimm_oob_32() {
    let got = encode_v_arith_vi(&ops3(1, 2, 32), 0b001110);
    assert!(
        got.is_err(),
        "vslideup.vi v1, v2, 32 must Err (llvm-mc range [0, 31]); got {got:?}"
    );
}
