// Oracle: differential — llvm-mc RISC-V assembler (RVV vmv.v.v)
// Evidence: vector.rs:153-156 "vmv.v.v vd, vs1: OPIVV, funct6=010111, vm=1, vs2=0";
//   encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words";
//   encoder/mod.rs:1002 "vmv.v.v" => encode_vmv_v_v(operands) (operands passed through);
//   assembler/README.md:14 V (vector) standard extension; assembler/README.md:109 vector.rs RVV;
//   RISC-V V 1.0 vmv.v.v: opcode=1010111, funct3=000 (OPIVV), vd[11:7], vs1[19:15],
//   vs2[24:20]=00000, vm[25]=1, funct6[31:26]=010111.
// Stronger considered:
//   - State machine: rejected — encode_vmv_v_v is a pure function with no lifecycle
//   - Algebraic round-trip via in-tree decoder: rejected — no vmv.v.v decoder
//   - encode_vmv_v_x / encode_vmv_v_i / encode_v_arith_vv as differential siblings:
//     rejected — same-job gate (OPIVX funct3=100 / OPIVI funct3=011 /
//     3-operand OPIVV with vs2 in bits[24:20])
// Weaker available: algebraic.invariant (field unpack), algebraic.metamorphic
//   (vd/vs1 isolation and swap), negative_error (arity / bad regs / extra / v0.t)
// Differential: candidate=encode_vmv_v_v,
//   reference=llvm-mc -triple=riscv64 -mattr=+v -show-encoding,
//   SUT-boundary=internal-helper of RISC-V assembler,
//   mapping=[Reg(vd), Reg(vs1)] <-> `vmv.v.v vd, vs1`.

use super::{encode_vmv_v_v, EncodeResult};
use crate::backend::riscv::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;
const OP_V: u32 = 0b1010111;
const FUNCT6_VMV: u32 = 0b010111;

fn cfg() -> ProptestConfig {
    ProptestConfig::with_cases(CASES)
}

fn vn(n: u32) -> String {
    format!("v{n}")
}

fn vreg(n: u32) -> Operand {
    Operand::Reg(vn(n))
}

fn ops2(vd: u32, vs1: u32) -> Vec<Operand> {
    vec![vreg(vd), vreg(vs1)]
}

fn sut_word(ops: &[Operand]) -> Result<u32, String> {
    match encode_vmv_v_v(ops)? {
        EncodeResult::Word(w) => Ok(w),
        other => Err(format!("expected Word, got {other:?}")),
    }
}

/// Unpack OPIVV vmv.v.v per RISC-V V 1.0 (not a copy of encode_vmv_v_v).
fn unpack_vmv_v_v(word: u32) -> (u32, u32, u32, u32, u32, u32, u32) {
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

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_vmv_v_v_kat_llvm_mc_v0_v0() {
    let want = 0x5e000057u32;
    let mc = llvm_mc_word("vmv.v.v v0, v0").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT drifted: {mc:#x} != {want:#x}");
    let sut = sut_word(&ops2(0, 0)).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_vmv_v_v_kat_llvm_mc_v1_v2() {
    let want = 0x5e0100d7u32;
    let mc = llvm_mc_word("vmv.v.v v1, v2").expect("llvm-mc KAT");
    assert_eq!(mc, want);
    let sut = sut_word(&ops2(1, 2)).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_vmv_v_v_kat_llvm_mc_v31_v30() {
    let want = 0x5e0f0fd7u32;
    let mc = llvm_mc_word("vmv.v.v v31, v30").expect("llvm-mc KAT");
    assert_eq!(mc, want);
    let sut = sut_word(&ops2(31, 30)).expect("SUT KAT");
    assert_eq!(sut, want);
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_vmv_v_v_diff_llvm_mc(vd in vreg_n(), vs1 in vreg_n()) {
        let asm = format!("vmv.v.v v{vd}, v{vs1}");
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {asm}: {e}"));
        let sut = sut_word(&ops2(vd, vs1))
            .unwrap_or_else(|e| panic!("SUT rejected valid {asm}: {e}"));
        prop_assert_eq!(sut, mc, "SUT {:#010x} != llvm-mc {:#010x} for {}", sut, mc, asm);
    }

    #[test]
    fn encode_vmv_v_v_format_fields(vd in vreg_n(), vs1 in vreg_n()) {
        let w = sut_word(&ops2(vd, vs1))
            .unwrap_or_else(|e| panic!("SUT rejected: {e}"));
        let (opcode, got_vd, funct3, got_vs1, vs2, vm, funct6) = unpack_vmv_v_v(w);
        prop_assert_eq!(opcode, OP_V, "opcode");
        prop_assert_eq!(got_vd, vd, "vd");
        prop_assert_eq!(funct3, 0, "funct3 must be 000 (OPIVV)");
        prop_assert_eq!(got_vs1, vs1, "vs1");
        prop_assert_eq!(vs2, 0, "vs2 must be 0 for vmv.v.v");
        prop_assert_eq!(vm, 1, "vm must be 1 (unmasked)");
        prop_assert_eq!(funct6, FUNCT6_VMV, "funct6 must be 010111");
    }

    #[test]
    fn encode_vmv_v_v_field_isolation(
        vd_a in vreg_n(),
        vd_b in vreg_n(),
        vs1_a in vreg_n(),
        vs1_b in vreg_n(),
    ) {
        let wa = sut_word(&ops2(vd_a, vs1_a))
            .unwrap_or_else(|e| panic!("a rejected: {e}"));
        let wb = sut_word(&ops2(vd_b, vs1_a))
            .unwrap_or_else(|e| panic!("b rejected: {e}"));
        let vd_mask = 0x1fu32 << 7;
        prop_assert_eq!(wa & !vd_mask, wb & !vd_mask, "non-vd bits independent of vd");
        prop_assert_eq!((wa >> 7) & 0x1f, vd_a);
        prop_assert_eq!((wb >> 7) & 0x1f, vd_b);

        let wc = sut_word(&ops2(vd_a, vs1_b))
            .unwrap_or_else(|e| panic!("c rejected: {e}"));
        let vs1_mask = 0x1fu32 << 15;
        prop_assert_eq!(wa & !vs1_mask, wc & !vs1_mask, "non-vs1 bits independent of vs1");
        prop_assert_eq!((wc >> 15) & 0x1f, vs1_b);
    }

    #[test]
    fn encode_vmv_v_v_vd_vs1_swap(vd in vreg_n(), vs1 in vreg_n()) {
        let w = sut_word(&ops2(vd, vs1))
            .unwrap_or_else(|e| panic!("w rejected: {e}"));
        let wp = sut_word(&ops2(vs1, vd))
            .unwrap_or_else(|e| panic!("w' rejected: {e}"));
        prop_assert_eq!((w >> 7) & 0x1f, vd, "operand 0 is vd in bits[11:7]");
        prop_assert_eq!((w >> 15) & 0x1f, vs1, "operand 1 is vs1 in bits[19:15]");
        prop_assert_eq!((wp >> 7) & 0x1f, vs1, "swapped operand 0 is vd field");
        prop_assert_eq!((wp >> 15) & 0x1f, vd, "swapped operand 1 is vs1 field");
        let src_mask = (0x1fu32 << 7) | (0x1fu32 << 15);
        prop_assert_eq!(
            w & !src_mask,
            wp & !src_mask,
            "non-vd/vs1 bits independent of vd/vs1 swap"
        );
    }

    #[test]
    fn encode_vmv_v_v_neg_arity_bad_regs(
        ops in short_ops(),
        bad in bad_vreg(),
        pos in 0usize..=1,
    ) {
        prop_assert!(
            encode_vmv_v_v(&ops).is_err(),
            "arity {} must Err (llvm-mc too few operands); got {:?}",
            ops.len(),
            encode_vmv_v_v(&ops)
        );
        let mut bad_ops = ops2(0, 1);
        bad_ops[pos] = bad.clone();
        prop_assert!(
            encode_vmv_v_v(&bad_ops).is_err(),
            "non-vector operand {:?} at pos {pos} must Err (llvm-mc invalid operand); got {:?}",
            bad,
            encode_vmv_v_v(&bad_ops)
        );
    }

    #[test]
    fn encode_vmv_v_v_neg_extra(
        vd in vreg_n(),
        vs1 in vreg_n(),
        extra in extra_operand(),
    ) {
        let mut ops = ops2(vd, vs1);
        ops.push(extra.clone());
        prop_assert!(
            encode_vmv_v_v(&ops).is_err(),
            "extra operand {:?} must Err for vmv.v.v (llvm-mc rejects extra); got {:?}",
            extra,
            encode_vmv_v_v(&ops)
        );
    }

    #[test]
    fn encode_vmv_v_v_neg_mask_v0t(vd in vreg_n(), vs1 in vreg_n()) {
        let mut ops = ops2(vd, vs1);
        ops.push(Operand::Symbol("v0.t".into()));
        prop_assert!(
            encode_vmv_v_v(&ops).is_err(),
            "trailing v0.t must Err for vmv.v.v (llvm-mc rejects mask on vmv.v.v); got {:?}",
            encode_vmv_v_v(&ops)
        );
    }
}

/// Regression: extra operand after a complete vd, vs1 vmv.v.v must Err.
#[test]
fn test_encode_vmv_v_v_regression_extra_operand() {
    let mut ops = ops2(0, 0);
    ops.push(Operand::Imm(0));
    let got = encode_vmv_v_v(&ops);
    assert!(
        got.is_err(),
        "vmv.v.v v0, v0, 0 must Err (llvm-mc invalid operand); got {got:?}"
    );
}

/// Regression: trailing v0.t on vmv.v.v must Err (unmasked-only).
#[test]
fn test_encode_vmv_v_v_regression_mask_v0t() {
    let mut ops = ops2(1, 2);
    ops.push(Operand::Symbol("v0.t".into()));
    let got = encode_vmv_v_v(&ops);
    assert!(
        got.is_err(),
        "vmv.v.v v1, v2, v0.t must Err (llvm-mc invalid operand); got {got:?}"
    );
}
