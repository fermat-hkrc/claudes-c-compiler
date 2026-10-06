// Oracle: differential — llvm-mc RISC-V assembler (RVV vsetvl)
// Evidence: vector.rs:68-69 "Encode vsetvl rd, rs1, rs2" /
//   "Format: [1000000][rs2][rs1][111][rd][1010111]";
//   encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words";
//   encoder/mod.rs:948-949 "vsetvl" => encode_vsetvl(operands) (operands passed through);
//   assembler/README.md:14 V (vector) standard extension; assembler/README.md:109 vector.rs RVV;
//   RISC-V V 1.0: opcode=1010111, funct3=111, bits[31:25]=1000000,
//   rs2 in [24:20], rs1 in [19:15], rd in [11:7].
// Stronger considered:
//   - State machine: rejected — encode_vsetvl is a pure function with no lifecycle
//   - Algebraic round-trip via in-tree decoder: rejected — no vsetvl decoder
//   - encode_vsetvli / encode_vsetivli as differential sibling: rejected —
//     same-job gate (vsetvli bit31=0 and vtypei; vsetivli bits[31:30]=11 and uimm)
// Weaker available: algebraic.invariant (field unpack), algebraic.metamorphic
//   (ABI vs xN alias; field isolation), negative_error (arity / FP / extra / nonreg)
// Differential: candidate=encode_vsetvl,
//   reference=llvm-mc -triple=riscv64 -mattr=+v -show-encoding,
//   SUT-boundary=internal-helper of RISC-V assembler,
//   mapping=[Reg(rd), Reg(rs1), Reg(rs2)] <-> `vsetvl rd, rs1, rs2`.

use super::{encode_vsetvl, EncodeResult};
use crate::backend::riscv::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;
const OP_V: u32 = 0b1010111;
const FUNCT7_VSETVL: u32 = 0b1000000;

const ABI: [&str; 32] = [
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3",
    "a4", "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11",
    "t3", "t4", "t5", "t6",
];

fn cfg() -> ProptestConfig {
    ProptestConfig::with_cases(CASES)
}

fn xn(n: u32) -> String {
    format!("x{n}")
}

fn abi_name(n: u32) -> &'static str {
    ABI[n as usize]
}

fn reg(name: &str) -> Operand {
    Operand::Reg(name.to_string())
}

fn ops3(rd: &str, rs1: &str, rs2: &str) -> Vec<Operand> {
    vec![reg(rd), reg(rs1), reg(rs2)]
}

fn sut_word(ops: &[Operand]) -> Result<u32, String> {
    match encode_vsetvl(ops)? {
        EncodeResult::Word(w) => Ok(w),
        other => Err(format!("expected Word, got {other:?}")),
    }
}

/// Unpack vsetvl per RISC-V V 1.0 (not a copy of encode_vsetvl).
fn unpack_vsetvl(word: u32) -> (u32, u32, u32, u32, u32, u32) {
    let opcode = word & 0x7f;
    let rd = (word >> 7) & 0x1f;
    let funct3 = (word >> 12) & 0x7;
    let rs1 = (word >> 15) & 0x1f;
    let rs2 = (word >> 20) & 0x1f;
    let funct7 = word >> 25;
    (opcode, rd, funct3, rs1, rs2, funct7)
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

fn gpr_n() -> impl Strategy<Value = u32> {
    0u32..=31
}

fn extra_operand() -> impl Strategy<Value = Operand> {
    prop_oneof![
        Just(Operand::Imm(0)),
        Just(Operand::Imm(-1)),
        Just(Operand::Imm(208)),
        (0u32..=31).prop_map(|n| Operand::Reg(xn(n))),
        Just(Operand::Symbol("foo".into())),
        Just(Operand::Symbol("e8".into())),
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

fn fp_or_vreg() -> impl Strategy<Value = String> {
    prop_oneof![
        (0u32..=31).prop_map(|n| format!("f{n}")),
        (0u32..=31).prop_map(|n| format!("v{n}")),
        Just("fa0".into()),
        Just("ft0".into()),
        Just("fs0".into()),
        Just("fa7".into()),
        Just("ft11".into()),
        Just("v0".into()),
        Just("v31".into()),
    ]
}

fn short_ops() -> impl Strategy<Value = Vec<Operand>> {
    prop_oneof![
        Just(vec![]),
        gpr_name().prop_map(|rd| vec![reg(&rd)]),
        (gpr_name(), gpr_name()).prop_map(|(rd, rs1)| vec![reg(&rd), reg(&rs1)]),
    ]
}

fn nonreg_operand() -> impl Strategy<Value = Operand> {
    prop_oneof![
        Just(Operand::Imm(-1)),
        Just(Operand::Imm(32)),
        Just(Operand::Imm(208)),
        Just(Operand::Imm(i64::MIN)),
        Just(Operand::Imm(i64::MAX)),
        Just(Operand::Symbol("foo".into())),
        Just(Operand::Symbol("e8".into())),
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

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_vsetvl_kat_llvm_mc_a0_a1_a2() {
    let want = 0x80c5f557u32;
    let mc = llvm_mc_word("vsetvl a0, a1, a2").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT drifted: {mc:#x} != {want:#x}");
    let sut = sut_word(&ops3("a0", "a1", "a2")).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_vsetvl_kat_llvm_mc_zero_ra_sp() {
    let want = 0x8020f057u32;
    let mc = llvm_mc_word("vsetvl zero, ra, sp").expect("llvm-mc KAT");
    assert_eq!(mc, want);
    let sut = sut_word(&ops3("zero", "ra", "sp")).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_vsetvl_kat_llvm_mc_x0_x0_x0() {
    let want = 0x80007057u32;
    let mc = llvm_mc_word("vsetvl x0, x0, x0").expect("llvm-mc KAT");
    assert_eq!(mc, want);
    let sut = sut_word(&ops3("x0", "x0", "x0")).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_vsetvl_kat_llvm_mc_x31_x31_x31() {
    let want = 0x81ffffd7u32;
    let mc = llvm_mc_word("vsetvl x31, x31, x31").expect("llvm-mc KAT");
    assert_eq!(mc, want);
    let sut = sut_word(&ops3("x31", "x31", "x31")).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_vsetvl_kat_llvm_mc_x10_x11_x12() {
    let want = 0x80c5f557u32;
    let mc = llvm_mc_word("vsetvl x10, x11, x12").expect("llvm-mc KAT");
    assert_eq!(mc, want);
    let sut = sut_word(&ops3("x10", "x11", "x12")).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_vsetvl_kat_llvm_mc_t0_t1_t2() {
    let want = 0x807372d7u32;
    let mc = llvm_mc_word("vsetvl t0, t1, t2").expect("llvm-mc KAT");
    assert_eq!(mc, want);
    let sut = sut_word(&ops3("t0", "t1", "t2")).expect("SUT KAT");
    assert_eq!(sut, want);
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_vsetvl_diff_llvm_mc(
        rd in gpr_name(),
        rs1 in gpr_name(),
        rs2 in gpr_name(),
    ) {
        let asm = format!("vsetvl {rd}, {rs1}, {rs2}");
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {asm}: {e}"));
        let sut = sut_word(&ops3(&rd, &rs1, &rs2))
            .unwrap_or_else(|e| panic!("SUT rejected valid {asm}: {e}"));
        prop_assert_eq!(sut, mc, "SUT {:#010x} != llvm-mc {:#010x} for {}", sut, mc, asm);
    }

    #[test]
    fn encode_vsetvl_format_fields(
        rd in gpr_n(),
        rs1 in gpr_n(),
        rs2 in gpr_n(),
    ) {
        let w = sut_word(&ops3(&xn(rd), &xn(rs1), &xn(rs2)))
            .unwrap_or_else(|e| panic!("SUT rejected: {e}"));
        let (opcode, got_rd, funct3, got_rs1, got_rs2, funct7) = unpack_vsetvl(w);
        prop_assert_eq!(opcode, OP_V, "opcode");
        prop_assert_eq!(funct3, 0b111, "funct3");
        prop_assert_eq!(funct7, FUNCT7_VSETVL, "funct7 must be 1000000 for vsetvl");
        prop_assert_eq!(got_rd, rd, "rd");
        prop_assert_eq!(got_rs1, rs1, "rs1");
        prop_assert_eq!(got_rs2, rs2, "rs2");
    }

    #[test]
    fn encode_vsetvl_abi_xn_alias(
        n in gpr_n(),
        m in gpr_n(),
        k in gpr_n(),
    ) {
        let via_x = sut_word(&ops3(&xn(n), &xn(m), &xn(k)))
            .unwrap_or_else(|e| panic!("xN rejected: {e}"));
        let via_abi = sut_word(&ops3(abi_name(n), abi_name(m), abi_name(k)))
            .unwrap_or_else(|e| panic!("ABI rejected: {e}"));
        prop_assert_eq!(via_abi, via_x);
        if n == 8 {
            let via_fp = sut_word(&ops3("fp", &xn(m), &xn(k)))
                .unwrap_or_else(|e| panic!("fp rd rejected: {e}"));
            prop_assert_eq!(via_fp, via_x);
        }
        if m == 8 {
            let via_fp = sut_word(&ops3(&xn(n), "fp", &xn(k)))
                .unwrap_or_else(|e| panic!("fp rs1 rejected: {e}"));
            prop_assert_eq!(via_fp, via_x);
        }
        if k == 8 {
            let via_fp = sut_word(&ops3(&xn(n), &xn(m), "fp"))
                .unwrap_or_else(|e| panic!("fp rs2 rejected: {e}"));
            prop_assert_eq!(via_fp, via_x);
        }
    }

    #[test]
    fn encode_vsetvl_field_isolation(
        rd_a in gpr_n(),
        rd_b in gpr_n(),
        rs1_a in gpr_n(),
        rs1_b in gpr_n(),
        rs2_a in gpr_n(),
        rs2_b in gpr_n(),
    ) {
        let wa = sut_word(&ops3(&xn(rd_a), &xn(rs1_a), &xn(rs2_a)))
            .unwrap_or_else(|e| panic!("a rejected: {e}"));
        let wb = sut_word(&ops3(&xn(rd_b), &xn(rs1_a), &xn(rs2_a)))
            .unwrap_or_else(|e| panic!("b rejected: {e}"));
        let rd_mask = 0x1fu32 << 7;
        prop_assert_eq!(wa & !rd_mask, wb & !rd_mask, "non-rd bits independent of rd");
        prop_assert_eq!((wa >> 7) & 0x1f, rd_a);
        prop_assert_eq!((wb >> 7) & 0x1f, rd_b);

        let wc = sut_word(&ops3(&xn(rd_a), &xn(rs1_b), &xn(rs2_a)))
            .unwrap_or_else(|e| panic!("c rejected: {e}"));
        let rs1_mask = 0x1fu32 << 15;
        prop_assert_eq!(wa & !rs1_mask, wc & !rs1_mask, "non-rs1 bits independent of rs1");
        prop_assert_eq!((wc >> 15) & 0x1f, rs1_b);

        let wd = sut_word(&ops3(&xn(rd_a), &xn(rs1_a), &xn(rs2_b)))
            .unwrap_or_else(|e| panic!("d rejected: {e}"));
        let rs2_mask = 0x1fu32 << 20;
        prop_assert_eq!(wa & !rs2_mask, wd & !rs2_mask, "non-rs2 bits independent of rs2");
        prop_assert_eq!((wd >> 20) & 0x1f, rs2_b);
    }

    #[test]
    fn encode_vsetvl_neg_arity_fp(ops in short_ops(), fp in fp_or_vreg()) {
        prop_assert!(
            encode_vsetvl(&ops).is_err(),
            "arity {} must Err (llvm-mc too few operands); got {:?}",
            ops.len(),
            encode_vsetvl(&ops)
        );
        let fp_rd = ops3(&fp, "a1", "a2");
        prop_assert!(
            encode_vsetvl(&fp_rd).is_err(),
            "FP/vector rd {} must Err (llvm-mc invalid operand); got {:?}",
            fp,
            encode_vsetvl(&fp_rd)
        );
        let fp_rs1 = ops3("a0", &fp, "a2");
        prop_assert!(
            encode_vsetvl(&fp_rs1).is_err(),
            "FP/vector rs1 {} must Err (llvm-mc invalid operand); got {:?}",
            fp,
            encode_vsetvl(&fp_rs1)
        );
        let fp_rs2 = ops3("a0", "a1", &fp);
        prop_assert!(
            encode_vsetvl(&fp_rs2).is_err(),
            "FP/vector rs2 {} must Err (llvm-mc invalid operand); got {:?}",
            fp,
            encode_vsetvl(&fp_rs2)
        );
    }

    #[test]
    fn encode_vsetvl_neg_extra(
        rd in gpr_name(),
        rs1 in gpr_name(),
        rs2 in gpr_name(),
        extra in extra_operand(),
    ) {
        let mut ops = ops3(&rd, &rs1, &rs2);
        ops.push(extra.clone());
        prop_assert!(
            encode_vsetvl(&ops).is_err(),
            "extra operand {:?} must Err for vsetvl (llvm-mc rejects extra); got {:?}",
            extra,
            encode_vsetvl(&ops)
        );
    }

    #[test]
    fn encode_vsetvl_neg_nonreg(
        pos in 0usize..=2,
        bad in nonreg_operand(),
        rd in gpr_name(),
        rs1 in gpr_name(),
        rs2 in gpr_name(),
    ) {
        let mut ops = ops3(&rd, &rs1, &rs2);
        ops[pos] = bad.clone();
        prop_assert!(
            encode_vsetvl(&ops).is_err(),
            "non-register {:?} at operand {} must Err (llvm-mc invalid operand); got {:?}",
            bad,
            pos,
            encode_vsetvl(&ops)
        );
    }
}

/// Regression: extra operand after a complete 3-GPR vsetvl must Err.
#[test]
fn test_encode_vsetvl_regression_extra_operand() {
    let mut ops = ops3("x0", "x0", "x0");
    ops.push(Operand::Imm(0));
    let got = encode_vsetvl(&ops);
    assert!(
        got.is_err(),
        "vsetvl x0, x0, x0, 0 must Err (llvm-mc invalid operand); got {got:?}"
    );
}
