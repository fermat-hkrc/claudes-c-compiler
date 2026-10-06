// Oracle: differential — llvm-mc RISC-V assembler (C.JR CR-type halfword)
// Evidence: src/backend/riscv/assembler/README.md:13 C (compressed 16-bit);
//   README.md:108 compressed.rs RVC 16-bit instructions;
//   encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words";
//   encoder/mod.rs:107 EncodeResult::Half; encoder/mod.rs:930 "c.jr" => encode_c_jr;
//   compressed.rs:49 "c.jr rs1";
//   compress.rs:568 C.JR: jalr x0, 0(rs1); compress.rs:566 rd == 0 && rs1 != 0;
//   RISC-V Unprivileged ISA C.JR CR-type: [15:12]=1000, [11:7]=rs1, [6:2]=0,
//   [1:0]=10. rs1≠x0 (rs1=x0 is reserved).
// Stronger considered:
//   - State machine: rejected — encode_c_jr is a pure function with no lifecycle
//   - Algebraic round-trip via in-tree decoder: rejected — no C.JR decoder
//   - try_compress_rv64 as differential sibling: rejected —
//     same-job gate (post-encode compress of 32-bit JALR, not c.jr mnemonic)
// Weaker available: algebraic.invariant (CR-type field unpack), algebraic.metamorphic
//   (ABI vs xN alias, field isolation), negative_error (rs1=x0, extra, arity/FP)
// Differential: candidate=encode_c_jr, reference=llvm-mc -triple=riscv64 -mattr=+c
//   -show-encoding, SUT-boundary=internal-helper of RISC-V assembler,
//   mapping=[Reg(rs1)] <-> `c.jr rs1` with rs1 ≠ x0.

use super::{encode_c_jr, EncodeResult};
use crate::backend::riscv::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;

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

/// Unpack C.JR CR-type per RISC-V unprivileged ISA (not a copy of encode_c_jr).
fn unpack_c_jr(half: u16) -> (u16, u16, u32, u32) {
    let op = half & 0b11;
    let funct4 = (half >> 12) & 0b1111;
    let rs1 = ((half >> 7) & 0x1f) as u32;
    let rs2 = ((half >> 2) & 0x1f) as u32;
    (op, funct4, rs1, rs2)
}

fn sut_half(ops: &[Operand]) -> Result<u16, String> {
    match encode_c_jr(ops)? {
        EncodeResult::Half(h) => Ok(h),
        other => Err(format!("expected Half, got {other:?}")),
    }
}

fn parse_llvm_encoding(stdout: &str) -> Result<u16, String> {
    let marker = "encoding: [";
    let start = stdout
        .find(marker)
        .ok_or_else(|| format!("no encoding in stdout: {stdout}"))?;
    let rest = &stdout[start + marker.len()..];
    let end = rest
        .find(']')
        .ok_or_else(|| format!("no closing bracket: {stdout}"))?;
    let inner = &rest[..end];
    let parts: Vec<&str> = inner.split(',').collect();
    if parts.len() != 2 {
        return Err(format!("expected 2 bytes, got {inner}"));
    }
    let mut bytes = [0u8; 2];
    for (i, p) in parts.iter().enumerate() {
        let p = p.trim();
        let hex = p
            .strip_prefix("0x")
            .ok_or_else(|| format!("non-hex byte {p}"))?;
        bytes[i] = u8::from_str_radix(hex, 16).map_err(|e| e.to_string())?;
    }
    Ok(u16::from_le_bytes(bytes))
}

fn llvm_mc_half(asm: &str) -> Result<u16, String> {
    let mut child = Command::new(LLVM_MC)
        .args(["-triple=riscv64", "-mattr=+c", "-show-encoding"])
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

fn llvm_mc_rejects(asm: &str) -> bool {
    llvm_mc_half(asm).is_err()
}

fn gpr_nz() -> impl Strategy<Value = u32> {
    1u32..=31
}

fn gpr_nz_name() -> impl Strategy<Value = String> {
    gpr_nz().prop_flat_map(|n| {
        prop_oneof![
            Just(xn(n)),
            Just(abi_name(n).to_string()),
            Just(if n == 8 { "fp".into() } else { xn(n) }),
        ]
    })
}

fn extra_operand() -> impl Strategy<Value = Operand> {
    prop_oneof![
        Just(Operand::Imm(0)),
        Just(Operand::Imm(-1)),
        (0u32..=31).prop_map(|n| Operand::Reg(xn(n))),
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

fn fp_name() -> impl Strategy<Value = String> {
    prop_oneof![
        (0u32..=31).prop_map(|n| format!("f{n}")),
        Just("fa0".into()),
        Just("ft0".into()),
        Just("fs0".into()),
        Just("fa7".into()),
        Just("ft11".into()),
    ]
}

fn empty_ops() -> impl Strategy<Value = Vec<Operand>> {
    Just(vec![])
}

fn x0_name() -> impl Strategy<Value = String> {
    prop_oneof![Just("x0".to_string()), Just("zero".to_string())]
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_c_jr_kat_llvm_mc_ra() {
    let want = 0x8082u16;
    let mc = llvm_mc_half("c.jr ra").expect("llvm-mc KAT ra");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_half(&[reg("ra")]).expect("SUT KAT ra");
    assert_eq!(sut, want);
}

#[test]
fn encode_c_jr_kat_llvm_mc_x1() {
    let want = 0x8082u16;
    let mc = llvm_mc_half("c.jr x1").expect("llvm-mc KAT x1");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_half(&[reg("x1")]).expect("SUT KAT x1");
    assert_eq!(sut, want);
}

#[test]
fn encode_c_jr_kat_llvm_mc_sp() {
    let want = 0x8102u16;
    let mc = llvm_mc_half("c.jr sp").expect("llvm-mc KAT sp");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_half(&[reg("sp")]).expect("SUT KAT sp");
    assert_eq!(sut, want);
}

#[test]
fn encode_c_jr_kat_llvm_mc_a0() {
    let want = 0x8502u16;
    let mc = llvm_mc_half("c.jr a0").expect("llvm-mc KAT a0");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_half(&[reg("a0")]).expect("SUT KAT a0");
    assert_eq!(sut, want);
}

#[test]
fn encode_c_jr_kat_llvm_mc_x31() {
    let want = 0x8f82u16;
    let mc = llvm_mc_half("c.jr x31").expect("llvm-mc KAT x31");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_half(&[reg("x31")]).expect("SUT KAT x31");
    assert_eq!(sut, want);
}

#[test]
fn encode_c_jr_kat_llvm_mc_t0() {
    let want = 0x8282u16;
    let mc = llvm_mc_half("c.jr t0").expect("llvm-mc KAT t0");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_half(&[reg("t0")]).expect("SUT KAT t0");
    assert_eq!(sut, want);
}

/// Regression: extra operand must be rejected (C.JR is one-operand; llvm-mc errors).
#[test]
fn test_encode_c_jr_regression_extra_operand() {
    let ops = [reg("x1"), Operand::Imm(0)];
    assert!(
        encode_c_jr(&ops).is_err(),
        "c.jr x1 with a second operand must Err; got {:?}",
        encode_c_jr(&ops)
    );
}

/// Regression: rs1=x0 must be rejected (that encoding is reserved, not C.JR).
#[test]
fn test_encode_c_jr_regression_rs1_x0() {
    let ops = [reg("x0")];
    assert!(
        encode_c_jr(&ops).is_err(),
        "c.jr x0 must Err (rs1=x0 is reserved); got {:?}",
        encode_c_jr(&ops)
    );
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_c_jr_diff_llvm_mc(rs1 in gpr_nz_name()) {
        let asm = format!("c.jr {}", rs1);
        let mc = llvm_mc_half(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_half(&[reg(&rs1)])
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "SUT {:04x} != llvm-mc {:04x} for {}", sut, mc, asm);
    }

    #[test]
    fn encode_c_jr_cr_type_fields(rs1 in gpr_nz()) {
        let h = sut_half(&[reg(&xn(rs1))])
            .unwrap_or_else(|e| panic!("SUT rejected x{}: {}", rs1, e));
        let (op, funct4, got_rs1, got_rs2) = unpack_c_jr(h);
        prop_assert_eq!(op, 0b10, "op");
        prop_assert_eq!(funct4, 0b1000, "funct4");
        prop_assert_eq!(got_rs1, rs1, "rs1");
        prop_assert_eq!(got_rs2, 0, "rs2 must be 0 for C.JR");
    }

    #[test]
    fn encode_c_jr_abi_xn_alias(n in gpr_nz()) {
        let via_x = sut_half(&[reg(&xn(n))])
            .unwrap_or_else(|e| panic!("xN rejected: {}", e));
        let via_abi = sut_half(&[reg(abi_name(n))])
            .unwrap_or_else(|e| panic!("ABI rejected: {}", e));
        prop_assert_eq!(via_abi, via_x);
        if n == 8 {
            let via_fp = sut_half(&[reg("fp")])
                .unwrap_or_else(|e| panic!("fp rs1 rejected: {}", e));
            prop_assert_eq!(via_fp, via_x);
        }
    }

    #[test]
    fn encode_c_jr_field_isolation(rs1_a in gpr_nz(), rs1_b in gpr_nz(), rs1 in gpr_nz()) {
        let ha = sut_half(&[reg(&xn(rs1_a))])
            .unwrap_or_else(|e| panic!("rs1_a rejected: {}", e));
        let hb = sut_half(&[reg(&xn(rs1_b))])
            .unwrap_or_else(|e| panic!("rs1_b rejected: {}", e));
        let rs1_mask = !0x0f80u16;
        prop_assert_eq!(ha & rs1_mask, hb & rs1_mask, "op/funct4/rs2 bits must be independent of rs1");
        let h = sut_half(&[reg(&xn(rs1))])
            .unwrap_or_else(|e| panic!("rs1 rejected: {}", e));
        prop_assert_eq!((h >> 7) & 0x1f, rs1 as u16, "rs1 field must equal rs1");
    }

    #[test]
    fn encode_c_jr_neg_rs1_x0(name in x0_name()) {
        let asm = format!("c.jr {}", name);
        prop_assert!(
            llvm_mc_rejects(&asm),
            "reference unexpectedly accepted {}",
            asm
        );
        let ops = [reg(&name)];
        prop_assert!(
            encode_c_jr(&ops).is_err(),
            "rs1={} must Err (llvm-mc rejects; encoding is reserved); got {:?}",
            name,
            encode_c_jr(&ops)
        );
    }

    #[test]
    fn encode_c_jr_neg_extra(rs1 in gpr_nz_name(), extra in extra_operand()) {
        let asm = format!("c.jr {}", rs1);
        let ops = vec![reg(&rs1), extra];
        prop_assert!(
            encode_c_jr(&ops).is_err(),
            "extra operand must Err for {} (llvm-mc rejects extra operands); got {:?}",
            asm,
            encode_c_jr(&ops)
        );
    }

    #[test]
    fn encode_c_jr_neg_arity_fp(ops in empty_ops(), fp in fp_name()) {
        prop_assert!(
            encode_c_jr(&ops).is_err(),
            "arity {} must Err, got {:?}",
            ops.len(),
            encode_c_jr(&ops)
        );
        let fp_rs1 = [reg(&fp)];
        prop_assert!(
            encode_c_jr(&fp_rs1).is_err(),
            "FP src {} must Err (llvm-mc invalid operand); got {:?}",
            fp,
            encode_c_jr(&fp_rs1)
        );
    }
}
