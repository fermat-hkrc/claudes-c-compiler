// Oracle: differential — llvm-mc RISC-V assembler (RVV unit-stride vector load)
// Evidence: vector.rs:76-79 "Encode vector unit-stride load: vle{8,16,32,64}.v vd, (rs1)" /
//   "Format: nf[31:29] | mew[28] | mop[27:26]=00 | vm[25] | lumop[24:20] | rs1[19:15] | width[14:12] | vd[11:7] | 0000111" /
//   "For unit-stride: mop=00, lumop=00000 (or 01011 for whole-reg/mask)";
//   encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words";
//   encoder/mod.rs:954-957 vle{8,16,32,64}.v => encode_vload(operands, width, 0);
//   encoder/mod.rs:966 "vlm.v" => encode_vload(operands, 0b000, 0x0B);
//   assembler/README.md:14 V (vector) standard extension; assembler/README.md:109 vector.rs RVV;
//   RISC-V V 1.0: opcode=0000111, width EEW, vd, rs1, lumop, vm=1 unmasked, mop=00, mew=0, nf=000.
// Stronger considered:
//   - State machine: rejected — encode_vload is a pure function with no lifecycle
//   - Algebraic round-trip via in-tree decoder: rejected — no vector-load decoder
//   - encode_vstore as differential sibling: rejected — same-job gate (STORE-FP opcode)
// Weaker available: algebraic.invariant (field unpack), algebraic.metamorphic
//   (ABI vs xN; Mem vs Reg), negative_error (arity / bad regs / extra / nonzero offset)
// Differential: candidate=encode_vload,
//   reference=llvm-mc -triple=riscv64 -mattr=+v -show-encoding,
//   SUT-boundary=internal-helper of RISC-V assembler,
//   mapping=[Reg(vd), Mem{base:rs1, offset:0}] + (width, lumop)
//     <-> `vle{8,16,32,64}.v vd, (rs1)` / `vlm.v vd, (rs1)`.

use super::{encode_vload, EncodeResult};
use crate::backend::riscv::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;
const OP_LOAD_FP: u32 = 0b0000111;

const ABI: [&str; 32] = [
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3",
    "a4", "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11",
    "t3", "t4", "t5", "t6",
];

/// (mnemonic, width[14:12], lumop[24:20]) for the five dispatched unit-stride loads.
const VLE_FAMILY: [(&str, u32, u32); 5] = [
    ("vle8.v", 0b000, 0),
    ("vle16.v", 0b101, 0),
    ("vle32.v", 0b110, 0),
    ("vle64.v", 0b111, 0),
    ("vlm.v", 0b000, 0x0B),
];

fn cfg() -> ProptestConfig {
    ProptestConfig::with_cases(CASES)
}

fn xn(n: u32) -> String {
    format!("x{n}")
}

fn vn(n: u32) -> String {
    format!("v{n}")
}

fn abi_name(n: u32) -> &'static str {
    ABI[n as usize]
}

fn vreg(n: u32) -> Operand {
    Operand::Reg(vn(n))
}

fn mem0(base: &str) -> Operand {
    Operand::Mem {
        base: base.to_string(),
        offset: 0,
    }
}

fn ops_mem(vd: u32, rs1: &str) -> Vec<Operand> {
    vec![vreg(vd), mem0(rs1)]
}

fn sut_word(ops: &[Operand], width: u32, lumop: u32) -> Result<u32, String> {
    match encode_vload(ops, width, lumop)? {
        EncodeResult::Word(w) => Ok(w),
        other => Err(format!("expected Word, got {other:?}")),
    }
}

/// Unpack unit-stride vector load per RISC-V V 1.0 (not a copy of encode_vload).
fn unpack_vload(word: u32) -> (u32, u32, u32, u32, u32, u32, u32, u32, u32) {
    let opcode = word & 0x7f;
    let vd = (word >> 7) & 0x1f;
    let width = (word >> 12) & 0x7;
    let rs1 = (word >> 15) & 0x1f;
    let lumop = (word >> 20) & 0x1f;
    let vm = (word >> 25) & 1;
    let mop = (word >> 26) & 0x3;
    let mew = (word >> 28) & 1;
    let nf = word >> 29;
    (opcode, vd, width, rs1, lumop, vm, mop, mew, nf)
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

fn gpr_name() -> impl Strategy<Value = String> {
    prop_oneof![
        (0u32..=31).prop_map(xn),
        (0u32..=31).prop_map(|n| abi_name(n).to_string()),
        Just("fp".into()),
    ]
}

fn vreg_n() -> impl Strategy<Value = u32> {
    0u32..=31
}

fn gpr_n() -> impl Strategy<Value = u32> {
    0u32..=31
}

fn width_n() -> impl Strategy<Value = u32> {
    0u32..=7
}

fn lumop_n() -> impl Strategy<Value = u32> {
    0u32..=31
}

fn vle_kind() -> impl Strategy<Value = usize> {
    0usize..=4
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

fn bad_vd() -> impl Strategy<Value = Operand> {
    prop_oneof![
        (0u32..=31).prop_map(|n| Operand::Reg(xn(n))),
        Just(Operand::Reg("a0".into())),
        Just(Operand::Reg("fa0".into())),
        Just(Operand::Reg("ft0".into())),
        Just(Operand::Reg("f0".into())),
        Just(Operand::Reg("v32".into())),
        Just(Operand::Imm(0)),
        Just(Operand::Imm(31)),
        Just(Operand::Symbol("foo".into())),
        Just(Operand::Label("L0".into())),
        Just(Operand::Mem {
            base: "sp".into(),
            offset: 0,
        }),
    ]
}

fn bad_rs1_base() -> impl Strategy<Value = String> {
    prop_oneof![
        (0u32..=31).prop_map(vn),
        Just("fa0".into()),
        Just("ft0".into()),
        Just("f0".into()),
        Just("fs0".into()),
        Just("v32".into()),
        Just("x32".into()),
        Just("foo".into()),
    ]
}

fn short_ops() -> impl Strategy<Value = Vec<Operand>> {
    prop_oneof![
        Just(vec![]),
        vreg_n().prop_map(|vd| vec![vreg(vd)]),
    ]
}

fn nonzero_off() -> impl Strategy<Value = i64> {
    prop_oneof![
        Just(1i64),
        Just(-1i64),
        Just(8i64),
        Just(-16i64),
        Just(i64::MIN),
        Just(i64::MAX),
        (-4096i64..=4096).prop_filter("nonzero", |o| *o != 0),
    ]
}

fn bad_op1() -> impl Strategy<Value = Operand> {
    prop_oneof![
        Just(Operand::Imm(0)),
        Just(Operand::Imm(8)),
        Just(Operand::Symbol("foo".into())),
        Just(Operand::Label("L0".into())),
        Just(Operand::SymbolOffset("foo".into(), 4)),
        Just(Operand::MemSymbol {
            base: "a0".into(),
            symbol: "%lo(foo)".into(),
            modifier: String::new(),
        }),
        Just(Operand::FenceArg("iorw".into())),
        Just(Operand::Csr("mstatus".into())),
        Just(Operand::RoundingMode("rne".into())),
    ]
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_vload_kat_llvm_mc_vle8_v0_a0() {
    let want = 0x02050007u32;
    let mc = llvm_mc_word("vle8.v v0, (a0)").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT drifted: {mc:#x} != {want:#x}");
    let sut = sut_word(&ops_mem(0, "a0"), 0b000, 0).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_vload_kat_llvm_mc_vle16_v1_a1() {
    let want = 0x0205d087u32;
    let mc = llvm_mc_word("vle16.v v1, (a1)").expect("llvm-mc KAT");
    assert_eq!(mc, want);
    let sut = sut_word(&ops_mem(1, "a1"), 0b101, 0).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_vload_kat_llvm_mc_vle32_v2_sp() {
    let want = 0x02016107u32;
    let mc = llvm_mc_word("vle32.v v2, (sp)").expect("llvm-mc KAT");
    assert_eq!(mc, want);
    let sut = sut_word(&ops_mem(2, "sp"), 0b110, 0).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_vload_kat_llvm_mc_vle64_v31_zero() {
    let want = 0x02007f87u32;
    let mc = llvm_mc_word("vle64.v v31, (zero)").expect("llvm-mc KAT");
    assert_eq!(mc, want);
    let sut = sut_word(&ops_mem(31, "zero"), 0b111, 0).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_vload_kat_llvm_mc_vlm_v0_a0() {
    let want = 0x02b50007u32;
    let mc = llvm_mc_word("vlm.v v0, (a0)").expect("llvm-mc KAT");
    assert_eq!(mc, want);
    let sut = sut_word(&ops_mem(0, "a0"), 0b000, 0x0B).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_vload_kat_llvm_mc_vle8_v0_x10() {
    let want = 0x02050007u32;
    let mc = llvm_mc_word("vle8.v v0, (x10)").expect("llvm-mc KAT");
    assert_eq!(mc, want);
    let sut = sut_word(&ops_mem(0, "x10"), 0b000, 0).expect("SUT KAT");
    assert_eq!(sut, want);
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_vload_diff_llvm_mc(
        vd in vreg_n(),
        rs1 in gpr_name(),
        kind in vle_kind(),
    ) {
        let (mnem, width, lumop) = VLE_FAMILY[kind];
        let asm = format!("{mnem} v{vd}, ({rs1})");
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {asm}: {e}"));
        let sut = sut_word(&ops_mem(vd, &rs1), width, lumop)
            .unwrap_or_else(|e| panic!("SUT rejected valid {asm}: {e}"));
        prop_assert_eq!(sut, mc, "SUT {:#010x} != llvm-mc {:#010x} for {}", sut, mc, asm);
    }

    #[test]
    fn encode_vload_format_fields(
        vd in vreg_n(),
        rs1 in gpr_n(),
        width in width_n(),
        lumop in lumop_n(),
    ) {
        let w = sut_word(&ops_mem(vd, &xn(rs1)), width, lumop)
            .unwrap_or_else(|e| panic!("SUT rejected: {e}"));
        let (opcode, got_vd, got_width, got_rs1, got_lumop, vm, mop, mew, nf) = unpack_vload(w);
        prop_assert_eq!(opcode, OP_LOAD_FP, "opcode");
        prop_assert_eq!(got_vd, vd, "vd");
        prop_assert_eq!(got_width, width, "width");
        prop_assert_eq!(got_rs1, rs1, "rs1");
        prop_assert_eq!(got_lumop, lumop, "lumop");
        prop_assert_eq!(vm, 1, "vm must be 1 (unmasked) for two-operand form");
        prop_assert_eq!(mop, 0, "mop must be 00 (unit-stride)");
        prop_assert_eq!(mew, 0, "mew must be 0");
        prop_assert_eq!(nf, 0, "nf must be 000");
    }

    #[test]
    fn encode_vload_mem_reg_alias(
        vd in vreg_n(),
        rs1 in gpr_name(),
        width in width_n(),
        lumop in lumop_n(),
    ) {
        let via_mem = sut_word(&ops_mem(vd, &rs1), width, lumop)
            .unwrap_or_else(|e| panic!("Mem rejected: {e}"));
        let via_reg = sut_word(&[vreg(vd), Operand::Reg(rs1.clone())], width, lumop)
            .unwrap_or_else(|e| panic!("Reg rejected: {e}"));
        prop_assert_eq!(via_mem, via_reg);
    }

    #[test]
    fn encode_vload_abi_xn_alias(
        vd in vreg_n(),
        n in gpr_n(),
        width in width_n(),
        lumop in lumop_n(),
    ) {
        let via_x = sut_word(&ops_mem(vd, &xn(n)), width, lumop)
            .unwrap_or_else(|e| panic!("xN rejected: {e}"));
        let via_abi = sut_word(&ops_mem(vd, abi_name(n)), width, lumop)
            .unwrap_or_else(|e| panic!("ABI rejected: {e}"));
        prop_assert_eq!(via_abi, via_x);
        if n == 8 {
            let via_fp = sut_word(&ops_mem(vd, "fp"), width, lumop)
                .unwrap_or_else(|e| panic!("fp rejected: {e}"));
            prop_assert_eq!(via_fp, via_x);
        }
    }

    #[test]
    fn encode_vload_field_isolation(
        vd_a in vreg_n(),
        vd_b in vreg_n(),
        rs1_a in gpr_n(),
        rs1_b in gpr_n(),
        width_a in width_n(),
        width_b in width_n(),
        lumop_a in lumop_n(),
        lumop_b in lumop_n(),
    ) {
        let wa = sut_word(&ops_mem(vd_a, &xn(rs1_a)), width_a, lumop_a)
            .unwrap_or_else(|e| panic!("a rejected: {e}"));
        let wb = sut_word(&ops_mem(vd_b, &xn(rs1_a)), width_a, lumop_a)
            .unwrap_or_else(|e| panic!("b rejected: {e}"));
        let vd_mask = 0x1fu32 << 7;
        prop_assert_eq!(wa & !vd_mask, wb & !vd_mask, "non-vd bits independent of vd");
        prop_assert_eq!((wa >> 7) & 0x1f, vd_a);
        prop_assert_eq!((wb >> 7) & 0x1f, vd_b);

        let wc = sut_word(&ops_mem(vd_a, &xn(rs1_b)), width_a, lumop_a)
            .unwrap_or_else(|e| panic!("c rejected: {e}"));
        let rs1_mask = 0x1fu32 << 15;
        prop_assert_eq!(wa & !rs1_mask, wc & !rs1_mask, "non-rs1 bits independent of rs1");
        prop_assert_eq!((wc >> 15) & 0x1f, rs1_b);

        let wd = sut_word(&ops_mem(vd_a, &xn(rs1_a)), width_b, lumop_a)
            .unwrap_or_else(|e| panic!("d rejected: {e}"));
        let width_mask = 0x7u32 << 12;
        prop_assert_eq!(wa & !width_mask, wd & !width_mask, "non-width bits independent of width");
        prop_assert_eq!((wd >> 12) & 0x7, width_b);

        let we = sut_word(&ops_mem(vd_a, &xn(rs1_a)), width_a, lumop_b)
            .unwrap_or_else(|e| panic!("e rejected: {e}"));
        let lumop_mask = 0x1fu32 << 20;
        prop_assert_eq!(wa & !lumop_mask, we & !lumop_mask, "non-lumop bits independent of lumop");
        prop_assert_eq!((we >> 20) & 0x1f, lumop_b);
    }

    #[test]
    fn encode_vload_neg_arity_bad_regs(
        ops in short_ops(),
        bad in bad_vd(),
        bad_base in bad_rs1_base(),
        width in width_n(),
        lumop in lumop_n(),
    ) {
        prop_assert!(
            encode_vload(&ops, width, lumop).is_err(),
            "arity {} must Err (llvm-mc too few operands); got {:?}",
            ops.len(),
            encode_vload(&ops, width, lumop)
        );
        let bad_vd_ops = vec![bad.clone(), mem0("a0")];
        prop_assert!(
            encode_vload(&bad_vd_ops, width, lumop).is_err(),
            "non-vector vd {:?} must Err (llvm-mc invalid operand); got {:?}",
            bad,
            encode_vload(&bad_vd_ops, width, lumop)
        );
        let bad_rs1_ops = vec![vreg(0), mem0(&bad_base)];
        prop_assert!(
            encode_vload(&bad_rs1_ops, width, lumop).is_err(),
            "non-GPR rs1 {} must Err (llvm-mc invalid operand); got {:?}",
            bad_base,
            encode_vload(&bad_rs1_ops, width, lumop)
        );
    }

    #[test]
    fn encode_vload_neg_extra(
        vd in vreg_n(),
        rs1 in gpr_name(),
        extra in extra_operand(),
        kind in vle_kind(),
    ) {
        let (_, width, lumop) = VLE_FAMILY[kind];
        let mut ops = ops_mem(vd, &rs1);
        ops.push(extra.clone());
        prop_assert!(
            encode_vload(&ops, width, lumop).is_err(),
            "extra operand {:?} must Err for vload (llvm-mc rejects extra); got {:?}",
            extra,
            encode_vload(&ops, width, lumop)
        );
    }

    #[test]
    fn encode_vload_neg_nonzero_offset(
        vd in vreg_n(),
        rs1 in gpr_name(),
        off in nonzero_off(),
        bad1 in bad_op1(),
        width in width_n(),
        lumop in lumop_n(),
    ) {
        let mem_ops = vec![
            vreg(vd),
            Operand::Mem {
                base: rs1.clone(),
                offset: off,
            },
        ];
        prop_assert!(
            encode_vload(&mem_ops, width, lumop).is_err(),
            "nonzero offset {off} must Err (llvm-mc invalid operand); got {:?}",
            encode_vload(&mem_ops, width, lumop)
        );
        let bad_ops = vec![vreg(vd), bad1.clone()];
        prop_assert!(
            encode_vload(&bad_ops, width, lumop).is_err(),
            "operand1 {:?} must Err (expected (rs1)); got {:?}",
            bad1,
            encode_vload(&bad_ops, width, lumop)
        );
    }
}

/// Regression: extra operand after a complete vd, (rs1) vload must Err.
#[test]
fn test_encode_vload_regression_extra_operand() {
    let mut ops = ops_mem(0, "x0");
    ops.push(Operand::Imm(0));
    let got = encode_vload(&ops, 0b000, 0);
    assert!(
        got.is_err(),
        "vle8.v v0, (x0), 0 must Err (llvm-mc invalid operand); got {got:?}"
    );
}
