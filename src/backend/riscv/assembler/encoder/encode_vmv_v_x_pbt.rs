// Oracle: differential — llvm-mc RISC-V assembler (RVV vmv.v.x)
// Evidence: vector.rs:162-169 "vmv.v.x vd, rs1: OPIVX, funct6=010111, vm=1, vs2=0";
//   encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words";
//   encoder/mod.rs:1005 "vmv.v.x" => encode_vmv_v_x(operands) (operands passed through);
//   assembler/README.md:14 V (vector) standard extension; assembler/README.md:109 vector.rs RVV;
//   RISC-V V 1.0 vmv.v.x: opcode=1010111, funct3=100 (OPIVX), vd[11:7], rs1[19:15] (GPR),
//   vs2[24:20]=00000, vm[25]=1, funct6[31:26]=010111.
// Stronger considered:
//   - State machine: rejected — encode_vmv_v_x is a pure function with no lifecycle
//   - Algebraic round-trip via in-tree decoder: rejected — no vmv.v.x decoder
//   - encode_vmv_v_v / encode_vmv_v_i / encode_v_arith_vx as differential siblings:
//     rejected — same-job gate (OPIVV funct3=000 / OPIVI funct3=011 /
//     3-operand OPIVX with vs2 in bits[24:20])
// Weaker available: algebraic.invariant (field unpack), algebraic.metamorphic
//   (vd/rs1 isolation, swap, ABI alias), negative_error (arity / bad regs / extra / v0.t)
// Differential: candidate=encode_vmv_v_x,
//   reference=llvm-mc -triple=riscv64 -mattr=+v -show-encoding,
//   SUT-boundary=internal-helper of RISC-V assembler,
//   mapping=[Reg(vd), Reg(rs1)] <-> `vmv.v.x vd, rs1`.

use super::{encode_vmv_v_x, EncodeResult};
use crate::backend::riscv::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;
const OP_V: u32 = 0b1010111;
const FUNCT6_VMV: u32 = 0b010111;
const FUNCT3_OPIVX: u32 = 0b100;

const ABI: [&str; 32] = [
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3",
    "a4", "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11",
    "t3", "t4", "t5", "t6",
];

fn cfg() -> ProptestConfig {
    ProptestConfig::with_cases(CASES)
}

fn vn(n: u32) -> String {
    format!("v{n}")
}

fn xn(n: u32) -> String {
    format!("x{n}")
}

fn vreg(n: u32) -> Operand {
    Operand::Reg(vn(n))
}

fn xreg(n: u32) -> Operand {
    Operand::Reg(xn(n))
}

fn ops2(vd: u32, rs1: u32) -> Vec<Operand> {
    vec![vreg(vd), xreg(rs1)]
}

fn sut_word(ops: &[Operand]) -> Result<u32, String> {
    match encode_vmv_v_x(ops)? {
        EncodeResult::Word(w) => Ok(w),
        other => Err(format!("expected Word, got {other:?}")),
    }
}

/// Unpack OPIVX vmv.v.x per RISC-V V 1.0 (not a copy of encode_vmv_v_x).
fn unpack_vmv_v_x(word: u32) -> (u32, u32, u32, u32, u32, u32, u32) {
    let opcode = word & 0x7f;
    let vd = (word >> 7) & 0x1f;
    let funct3 = (word >> 12) & 0x7;
    let rs1 = (word >> 15) & 0x1f;
    let vs2 = (word >> 20) & 0x1f;
    let vm = (word >> 25) & 1;
    let funct6 = word >> 26;
    (opcode, vd, funct3, rs1, vs2, vm, funct6)
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
        (0u32..=31).prop_map(|n| Operand::Reg(xn(n))),
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
        (0u32..=31).prop_map(|n| Operand::Reg(xn(n))),
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

fn bad_rs1() -> impl Strategy<Value = Operand> {
    prop_oneof![
        (0u32..=31).prop_map(|n| Operand::Reg(vn(n))),
        Just(Operand::Reg("v32".into())),
        Just(Operand::Reg("fa0".into())),
        Just(Operand::Reg("ft0".into())),
        Just(Operand::Reg("f0".into())),
        Just(Operand::Imm(-1)),
        Just(Operand::Imm(32)),
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
fn encode_vmv_v_x_kat_llvm_mc_v0_x0() {
    let want = 0x5e004057u32;
    let mc = llvm_mc_word("vmv.v.x v0, x0").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT drifted: {mc:#x} != {want:#x}");
    let sut = sut_word(&ops2(0, 0)).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_vmv_v_x_kat_llvm_mc_v1_x2() {
    let want = 0x5e0140d7u32;
    let mc = llvm_mc_word("vmv.v.x v1, x2").expect("llvm-mc KAT");
    assert_eq!(mc, want);
    let sut = sut_word(&ops2(1, 2)).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_vmv_v_x_kat_llvm_mc_v31_x30() {
    let want = 0x5e0f4fd7u32;
    let mc = llvm_mc_word("vmv.v.x v31, x30").expect("llvm-mc KAT");
    assert_eq!(mc, want);
    let sut = sut_word(&ops2(31, 30)).expect("SUT KAT");
    assert_eq!(sut, want);
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_vmv_v_x_diff_llvm_mc(vd in vreg_n(), rs1 in vreg_n()) {
        let asm = format!("vmv.v.x v{vd}, x{rs1}");
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {asm}: {e}"));
        let sut = sut_word(&ops2(vd, rs1))
            .unwrap_or_else(|e| panic!("SUT rejected valid {asm}: {e}"));
        prop_assert_eq!(sut, mc, "SUT {:#010x} != llvm-mc {:#010x} for {}", sut, mc, asm);
    }

    #[test]
    fn encode_vmv_v_x_format_fields(vd in vreg_n(), rs1 in vreg_n()) {
        let w = sut_word(&ops2(vd, rs1))
            .unwrap_or_else(|e| panic!("SUT rejected: {e}"));
        let (opcode, got_vd, funct3, got_rs1, vs2, vm, funct6) = unpack_vmv_v_x(w);
        prop_assert_eq!(opcode, OP_V, "opcode");
        prop_assert_eq!(got_vd, vd, "vd");
        prop_assert_eq!(funct3, FUNCT3_OPIVX, "funct3 must be 100 (OPIVX)");
        prop_assert_eq!(got_rs1, rs1, "rs1");
        prop_assert_eq!(vs2, 0, "vs2 must be 0 for vmv.v.x");
        prop_assert_eq!(vm, 1, "vm must be 1 (unmasked)");
        prop_assert_eq!(funct6, FUNCT6_VMV, "funct6 must be 010111");
    }

    #[test]
    fn encode_vmv_v_x_field_isolation(
        vd_a in vreg_n(),
        vd_b in vreg_n(),
        rs1_a in vreg_n(),
        rs1_b in vreg_n(),
    ) {
        let wa = sut_word(&ops2(vd_a, rs1_a))
            .unwrap_or_else(|e| panic!("a rejected: {e}"));
        let wb = sut_word(&ops2(vd_b, rs1_a))
            .unwrap_or_else(|e| panic!("b rejected: {e}"));
        let vd_mask = 0x1fu32 << 7;
        prop_assert_eq!(wa & !vd_mask, wb & !vd_mask, "non-vd bits independent of vd");
        prop_assert_eq!((wa >> 7) & 0x1f, vd_a);
        prop_assert_eq!((wb >> 7) & 0x1f, vd_b);

        let wc = sut_word(&ops2(vd_a, rs1_b))
            .unwrap_or_else(|e| panic!("c rejected: {e}"));
        let rs1_mask = 0x1fu32 << 15;
        prop_assert_eq!(wa & !rs1_mask, wc & !rs1_mask, "non-rs1 bits independent of rs1");
        prop_assert_eq!((wc >> 15) & 0x1f, rs1_b);
    }

    #[test]
    fn encode_vmv_v_x_vd_rs1_swap(vd in vreg_n(), rs1 in vreg_n()) {
        let w = sut_word(&ops2(vd, rs1))
            .unwrap_or_else(|e| panic!("w rejected: {e}"));
        let wp = sut_word(&ops2(rs1, vd))
            .unwrap_or_else(|e| panic!("w' rejected: {e}"));
        prop_assert_eq!((w >> 7) & 0x1f, vd, "operand 0 is vd in bits[11:7]");
        prop_assert_eq!((w >> 15) & 0x1f, rs1, "operand 1 is rs1 in bits[19:15]");
        prop_assert_eq!((wp >> 7) & 0x1f, rs1, "swapped operand 0 is vd field");
        prop_assert_eq!((wp >> 15) & 0x1f, vd, "swapped operand 1 is rs1 field");
        let src_mask = (0x1fu32 << 7) | (0x1fu32 << 15);
        prop_assert_eq!(
            w & !src_mask,
            wp & !src_mask,
            "non-vd/rs1 bits independent of vd/rs1 swap"
        );
    }

    #[test]
    fn encode_vmv_v_x_abi_alias(vd in vreg_n(), rs1 in vreg_n()) {
        let wx = sut_word(&ops2(vd, rs1))
            .unwrap_or_else(|e| panic!("xN rejected: {e}"));
        let abi = ABI[rs1 as usize];
        let wabi = sut_word(&[vreg(vd), Operand::Reg(abi.into())])
            .unwrap_or_else(|e| panic!("ABI {abi} rejected: {e}"));
        prop_assert_eq!(wx, wabi, "ABI alias must encode as xN");
    }

    #[test]
    fn encode_vmv_v_x_neg_arity_bad_regs(
        ops in short_ops(),
        bad_v in bad_vreg(),
        bad_x in bad_rs1(),
        pos in 0usize..=1,
    ) {
        prop_assert!(
            encode_vmv_v_x(&ops).is_err(),
            "arity {} must Err (llvm-mc too few operands); got {:?}",
            ops.len(),
            encode_vmv_v_x(&ops)
        );
        if pos == 0 {
            let mut bad_ops = ops2(0, 1);
            bad_ops[0] = bad_v.clone();
            prop_assert!(
                encode_vmv_v_x(&bad_ops).is_err(),
                "non-vector operand {:?} at vd must Err (llvm-mc invalid operand); got {:?}",
                bad_v,
                encode_vmv_v_x(&bad_ops)
            );
        } else {
            let mut bad_ops = ops2(0, 1);
            bad_ops[1] = bad_x.clone();
            prop_assert!(
                encode_vmv_v_x(&bad_ops).is_err(),
                "non-GPR operand {:?} at rs1 must Err (llvm-mc invalid operand); got {:?}",
                bad_x,
                encode_vmv_v_x(&bad_ops)
            );
        }
    }

    #[test]
    fn encode_vmv_v_x_neg_extra(
        vd in vreg_n(),
        rs1 in vreg_n(),
        extra in extra_operand(),
    ) {
        let mut ops = ops2(vd, rs1);
        ops.push(extra.clone());
        prop_assert!(
            encode_vmv_v_x(&ops).is_err(),
            "extra operand {:?} must Err for vmv.v.x (llvm-mc rejects extra); got {:?}",
            extra,
            encode_vmv_v_x(&ops)
        );
    }

    #[test]
    fn encode_vmv_v_x_neg_mask_v0t(vd in vreg_n(), rs1 in vreg_n()) {
        let mut ops = ops2(vd, rs1);
        ops.push(Operand::Symbol("v0.t".into()));
        prop_assert!(
            encode_vmv_v_x(&ops).is_err(),
            "trailing v0.t must Err for vmv.v.x (llvm-mc rejects mask on vmv.v.x); got {:?}",
            encode_vmv_v_x(&ops)
        );
    }
}

/// Regression: extra operand after a complete vd, rs1 vmv.v.x must Err.
#[test]
fn test_encode_vmv_v_x_regression_extra_operand() {
    let mut ops = ops2(0, 0);
    ops.push(Operand::Imm(0));
    let got = encode_vmv_v_x(&ops);
    assert!(
        got.is_err(),
        "vmv.v.x v0, x0, 0 must Err (llvm-mc invalid operand); got {got:?}"
    );
}

/// Regression: trailing v0.t on vmv.v.x must Err (unmasked-only).
#[test]
fn test_encode_vmv_v_x_regression_mask_v0t() {
    let mut ops = ops2(1, 2);
    ops.push(Operand::Symbol("v0.t".into()));
    let got = encode_vmv_v_x(&ops);
    assert!(
        got.is_err(),
        "vmv.v.x v1, x2, v0.t must Err (llvm-mc invalid operand); got {got:?}"
    );
}
