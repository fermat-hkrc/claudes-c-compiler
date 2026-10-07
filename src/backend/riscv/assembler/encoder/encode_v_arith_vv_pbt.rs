// Oracle: differential — llvm-mc RISC-V assembler (RVV OPIVV vector-vector arith)
// Evidence: vector.rs:122-129 "Encode vector arithmetic VV (vector-vector):
//   funct6[31:26] | vm[25] | vs2[24:20] | vs1[19:15] | funct3[14:12]=000 | vd[11:7] | OP_V";
//   encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words";
//   encoder/mod.rs:974-987 vadd.vv/vsub.vv/vand.vv/vor.vv/vxor.vv => encode_v_arith_vv(operands, funct6);
//   assembler/README.md:14 V (vector) standard extension; assembler/README.md:109 vector.rs RVV;
//   RISC-V V 1.0 OPIVV: opcode=1010111, funct3=000, vd, vs1, vs2, vm=1 unmasked, funct6.
// Stronger considered:
//   - State machine: rejected — encode_v_arith_vv is a pure function with no lifecycle
//   - Algebraic round-trip via in-tree decoder: rejected — no OPIVV decoder
//   - encode_v_arith_vx / encode_v_arith_vi as differential siblings: rejected —
//     same-job gate (OPIVX funct3=100 / OPIVI funct3=011)
// Weaker available: algebraic.invariant (field unpack), algebraic.metamorphic
//   (vs2/vs1 swap), negative_error (arity / bad regs / extra)
// Differential: candidate=encode_v_arith_vv,
//   reference=llvm-mc -triple=riscv64 -mattr=+v -show-encoding,
//   SUT-boundary=internal-helper of RISC-V assembler,
//   mapping=[Reg(vd), Reg(vs2), Reg(vs1)] + funct6
//     <-> `vadd.vv/vsub.vv/vand.vv/vor.vv/vxor.vv vd, vs2, vs1`.

use super::{encode_v_arith_vv, EncodeResult};
use crate::backend::riscv::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;
const OP_V: u32 = 0b1010111;

/// (mnemonic, funct6[31:26]) for the five dispatched OPIVV integer arith mnemonics.
const OPIVV_FAMILY: [(&str, u32); 5] = [
    ("vadd.vv", 0b000000),
    ("vsub.vv", 0b000010),
    ("vand.vv", 0b001001),
    ("vor.vv", 0b001010),
    ("vxor.vv", 0b001011),
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

fn ops3(vd: u32, vs2: u32, vs1: u32) -> Vec<Operand> {
    vec![vreg(vd), vreg(vs2), vreg(vs1)]
}

fn sut_word(ops: &[Operand], funct6: u32) -> Result<u32, String> {
    match encode_v_arith_vv(ops, funct6)? {
        EncodeResult::Word(w) => Ok(w),
        other => Err(format!("expected Word, got {other:?}")),
    }
}

/// Unpack OPIVV per RISC-V V 1.0 (not a copy of encode_v_arith_vv).
fn unpack_opivv(word: u32) -> (u32, u32, u32, u32, u32, u32, u32) {
    let opcode = word & 0x7f;
    let vd = (word >> 7) & 0x1f;
    let funct3 = (word >> 12) & 0x7;
    let vs1 = (word >> 15) & 0x1f;
    let vs2 = (word >> 20) & 0x1f;
    let vm = (word >> 25) & 1;
    let funct6 = word >> 26;
    (opcode, vd, funct3, vs1, vs2, vm, funct6)
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

fn opivv_kind() -> impl Strategy<Value = usize> {
    0usize..=4
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

fn short_ops() -> impl Strategy<Value = Vec<Operand>> {
    prop_oneof![
        Just(vec![]),
        vreg_n().prop_map(|vd| vec![vreg(vd)]),
        (vreg_n(), vreg_n()).prop_map(|(a, b)| vec![vreg(a), vreg(b)]),
    ]
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_v_arith_vv_kat_llvm_mc_vadd_v0_v0_v0() {
    let want = 0x02000057u32;
    let mc = llvm_mc_word("vadd.vv v0, v0, v0").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT drifted: {mc:#x} != {want:#x}");
    let sut = sut_word(&ops3(0, 0, 0), 0b000000).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_v_arith_vv_kat_llvm_mc_vadd_v1_v2_v3() {
    let want = 0x022180d7u32;
    let mc = llvm_mc_word("vadd.vv v1, v2, v3").expect("llvm-mc KAT");
    assert_eq!(mc, want);
    let sut = sut_word(&ops3(1, 2, 3), 0b000000).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_v_arith_vv_kat_llvm_mc_vadd_v31_v30_v29() {
    let want = 0x03ee8fd7u32;
    let mc = llvm_mc_word("vadd.vv v31, v30, v29").expect("llvm-mc KAT");
    assert_eq!(mc, want);
    let sut = sut_word(&ops3(31, 30, 29), 0b000000).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_v_arith_vv_kat_llvm_mc_vsub_v1_v2_v3() {
    let want = 0x0a2180d7u32;
    let mc = llvm_mc_word("vsub.vv v1, v2, v3").expect("llvm-mc KAT");
    assert_eq!(mc, want);
    let sut = sut_word(&ops3(1, 2, 3), 0b000010).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_v_arith_vv_kat_llvm_mc_vand_v1_v2_v3() {
    let want = 0x262180d7u32;
    let mc = llvm_mc_word("vand.vv v1, v2, v3").expect("llvm-mc KAT");
    assert_eq!(mc, want);
    let sut = sut_word(&ops3(1, 2, 3), 0b001001).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_v_arith_vv_kat_llvm_mc_vor_v1_v2_v3() {
    let want = 0x2a2180d7u32;
    let mc = llvm_mc_word("vor.vv v1, v2, v3").expect("llvm-mc KAT");
    assert_eq!(mc, want);
    let sut = sut_word(&ops3(1, 2, 3), 0b001010).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_v_arith_vv_kat_llvm_mc_vxor_v1_v2_v3() {
    let want = 0x2e2180d7u32;
    let mc = llvm_mc_word("vxor.vv v1, v2, v3").expect("llvm-mc KAT");
    assert_eq!(mc, want);
    let sut = sut_word(&ops3(1, 2, 3), 0b001011).expect("SUT KAT");
    assert_eq!(sut, want);
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_v_arith_vv_diff_llvm_mc(
        vd in vreg_n(),
        vs2 in vreg_n(),
        vs1 in vreg_n(),
        kind in opivv_kind(),
    ) {
        let (mnem, funct6) = OPIVV_FAMILY[kind];
        let asm = format!("{mnem} v{vd}, v{vs2}, v{vs1}");
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {asm}: {e}"));
        let sut = sut_word(&ops3(vd, vs2, vs1), funct6)
            .unwrap_or_else(|e| panic!("SUT rejected valid {asm}: {e}"));
        prop_assert_eq!(sut, mc, "SUT {:#010x} != llvm-mc {:#010x} for {}", sut, mc, asm);
    }

    #[test]
    fn encode_v_arith_vv_format_fields(
        vd in vreg_n(),
        vs2 in vreg_n(),
        vs1 in vreg_n(),
        funct6 in funct6_n(),
    ) {
        let w = sut_word(&ops3(vd, vs2, vs1), funct6)
            .unwrap_or_else(|e| panic!("SUT rejected: {e}"));
        let (opcode, got_vd, funct3, got_vs1, got_vs2, vm, got_f6) = unpack_opivv(w);
        prop_assert_eq!(opcode, OP_V, "opcode");
        prop_assert_eq!(got_vd, vd, "vd");
        prop_assert_eq!(funct3, 0, "funct3 must be 000 (OPIVV)");
        prop_assert_eq!(got_vs1, vs1, "vs1");
        prop_assert_eq!(got_vs2, vs2, "vs2");
        prop_assert_eq!(vm, 1, "vm must be 1 (unmasked) for three-operand form");
        prop_assert_eq!(got_f6, funct6, "funct6");
    }

    #[test]
    fn encode_v_arith_vv_field_isolation(
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
    fn encode_v_arith_vv_vs2_vs1_swap(
        vd in vreg_n(),
        vs2 in vreg_n(),
        vs1 in vreg_n(),
        funct6 in funct6_n(),
    ) {
        let w = sut_word(&ops3(vd, vs2, vs1), funct6)
            .unwrap_or_else(|e| panic!("w rejected: {e}"));
        let wp = sut_word(&ops3(vd, vs1, vs2), funct6)
            .unwrap_or_else(|e| panic!("w' rejected: {e}"));
        prop_assert_eq!((w >> 20) & 0x1f, vs2, "operand 1 is vs2 in bits[24:20]");
        prop_assert_eq!((w >> 15) & 0x1f, vs1, "operand 2 is vs1 in bits[19:15]");
        prop_assert_eq!((wp >> 20) & 0x1f, vs1, "swapped operand 1 is vs2 field");
        prop_assert_eq!((wp >> 15) & 0x1f, vs2, "swapped operand 2 is vs1 field");
        let src_mask = (0x1fu32 << 20) | (0x1fu32 << 15);
        prop_assert_eq!(
            w & !src_mask,
            wp & !src_mask,
            "non-vs1/vs2 bits independent of vs2/vs1 swap"
        );
    }

    #[test]
    fn encode_v_arith_vv_neg_arity_bad_regs(
        ops in short_ops(),
        bad in bad_vreg(),
        pos in 0usize..=2,
        funct6 in funct6_n(),
    ) {
        prop_assert!(
            encode_v_arith_vv(&ops, funct6).is_err(),
            "arity {} must Err (llvm-mc too few operands); got {:?}",
            ops.len(),
            encode_v_arith_vv(&ops, funct6)
        );
        let mut bad_ops = ops3(0, 1, 2);
        bad_ops[pos] = bad.clone();
        prop_assert!(
            encode_v_arith_vv(&bad_ops, funct6).is_err(),
            "non-vector operand {:?} at pos {pos} must Err (llvm-mc invalid operand); got {:?}",
            bad,
            encode_v_arith_vv(&bad_ops, funct6)
        );
    }

    #[test]
    fn encode_v_arith_vv_neg_extra(
        vd in vreg_n(),
        vs2 in vreg_n(),
        vs1 in vreg_n(),
        extra in extra_operand(),
        kind in opivv_kind(),
    ) {
        let (_, funct6) = OPIVV_FAMILY[kind];
        let mut ops = ops3(vd, vs2, vs1);
        ops.push(extra.clone());
        prop_assert!(
            encode_v_arith_vv(&ops, funct6).is_err(),
            "extra operand {:?} must Err for OPIVV (llvm-mc rejects extra); got {:?}",
            extra,
            encode_v_arith_vv(&ops, funct6)
        );
    }

    #[test]
    fn encode_v_arith_vv_mask_v0t(
        vd in vd_nonzero(),
        vs2 in vreg_n(),
        vs1 in vreg_n(),
        kind in opivv_kind(),
    ) {
        let (mnem, funct6) = OPIVV_FAMILY[kind];
        let asm = format!("{mnem} v{vd}, v{vs2}, v{vs1}, v0.t");
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {asm}: {e}"));
        let mut ops = ops3(vd, vs2, vs1);
        ops.push(Operand::Symbol("v0.t".into()));
        let sut = sut_word(&ops, funct6)
            .unwrap_or_else(|e| panic!("SUT rejected {asm}: {e}"));
        prop_assert_eq!(
            sut, mc,
            "SUT {:#010x} != llvm-mc {:#010x} for masked {}",
            sut, mc, asm
        );
    }
}

/// Regression: extra operand after a complete vd, vs2, vs1 OPIVV must Err.
#[test]
fn test_encode_v_arith_vv_regression_extra_operand() {
    let mut ops = ops3(0, 0, 0);
    ops.push(Operand::Imm(0));
    let got = encode_v_arith_vv(&ops, 0b000000);
    assert!(
        got.is_err(),
        "vadd.vv v0, v0, v0, 0 must Err (llvm-mc invalid operand); got {got:?}"
    );
}

/// Regression: trailing v0.t must encode vm=0 to match llvm-mc.
#[test]
fn test_encode_v_arith_vv_regression_mask_v0t() {
    let mut ops = ops3(1, 2, 3);
    ops.push(Operand::Symbol("v0.t".into()));
    let got = sut_word(&ops, 0b000000);
    let want = 0x002180d7u32; // vadd.vv v1, v2, v3, v0.t
    assert_eq!(
        got.expect("SUT must encode masked form, not Err"),
        want,
        "vadd.vv v1, v2, v3, v0.t must equal llvm-mc 0x002180d7 (vm=0)"
    );
}
