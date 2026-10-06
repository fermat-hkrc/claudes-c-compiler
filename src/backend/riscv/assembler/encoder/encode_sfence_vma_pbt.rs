// Oracle: differential — llvm-mc RISC-V assembler (R-type SYSTEM SFENCE.VMA)
// Evidence: src/backend/riscv/assembler/README.md:6-7 textual assembly from codegen;
//   README.md:105 system.rs System instructions;
//   README.md:352 R-type: [funct7 | rs2 | rs1 | funct3 | rd | opcode];
//   encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words";
//   encoder/mod.rs:4 "This covers the subset of instructions emitted by our codegen (RV64GC + Zbb).";
//   encoder/mod.rs:307 "R-type: funct7[31:25] | rs2[24:20] | rs1[19:15] | funct3[14:12] | rd[11:7] | opcode[6:0]";
//   encoder/mod.rs:364 OP_SYSTEM = 0b1110011;
//   encoder/mod.rs:699 "sfence.vma" => encode_sfence_vma(operands);
//   system.rs:25 "Encode sfence.vma rs1, rs2";
//   system.rs:26 "Format: funct7=0001001 | rs2 | rs1 | funct3=000 | rd=00000 | opcode=1110011";
//   system.rs:27 "If no operands: sfence.vma zero, zero";
//   system.rs:28 "If 1 operand: sfence.vma rs1, zero";
//   system.rs:29 "If 2 operands: sfence.vma rs1, rs2";
//   RISC-V Privileged ISA SFENCE.VMA: opcode=1110011, funct3=000, rd=00000,
//   funct7=0001001, rs1=vaddr, rs2=ASID.
// Stronger considered:
//   - State machine: rejected — encode_sfence_vma is a pure function with no lifecycle
//   - Algebraic round-trip via in-tree decoder: rejected — no SFENCE.VMA decoder
//   - encode_r / encode_fence / encode_csr as differential sibling:
//     rejected — same-job gate (private packer / FENCE I-type / CSR I-type)
// Weaker available: algebraic.invariant (R-type field unpack), algebraic.metamorphic
//   (empty == zero,zero; 1-operand == rs1,zero; ABI vs xN alias),
//   negative_error (extra / FP / non-Reg)
// Differential: candidate=encode_sfence_vma, reference=llvm-mc -triple=riscv64 -show-encoding,
//   SUT-boundary=internal-helper of RISC-V assembler,
//   mapping=[] <-> `sfence.vma`; [Reg(rs1)] <-> `sfence.vma rs1`;
//   [Reg(rs1), Reg(rs2)] <-> `sfence.vma rs1, rs2`.

use super::{encode_sfence_vma, EncodeResult};
use crate::backend::riscv::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;
const OP_SYSTEM: u32 = 0b1110011;
const FUNCT7_SFENCE_VMA: u32 = 0b0001001;

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

fn sut_word(ops: &[Operand]) -> Result<u32, String> {
    match encode_sfence_vma(ops)? {
        EncodeResult::Word(w) => Ok(w),
        other => Err(format!("expected Word, got {other:?}")),
    }
}

/// Unpack SYSTEM SFENCE.VMA R-type per RISC-V ISA (not a copy of encode_r).
/// Layout: funct7[31:25] | rs2[24:20] | rs1[19:15] | funct3[14:12] | rd[11:7] | opcode[6:0]
fn unpack_sfence(word: u32) -> (u32, u32, u32, u32, u32, u32) {
    let opcode = word & 0x7f;
    let rd = (word >> 7) & 0x1f;
    let funct3 = (word >> 12) & 0x7;
    let rs1 = (word >> 15) & 0x1f;
    let rs2 = (word >> 20) & 0x1f;
    let funct7 = (word >> 25) & 0x7f;
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
        if p.starts_with("0b") {
            return Err(format!("unresolved fixup encoding {inner}"));
        }
        let hex = p
            .strip_prefix("0x")
            .ok_or_else(|| format!("non-hex byte {p}"))?;
        bytes[i] = u8::from_str_radix(hex, 16).map_err(|e| e.to_string())?;
    }
    Ok(u32::from_le_bytes(bytes))
}

fn llvm_mc_word(asm: &str) -> Result<u32, String> {
    let mut child = Command::new(LLVM_MC)
        .args(["-triple=riscv64", "-show-encoding"])
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
        Just(Operand::Csr("mstatus".into())),
        Just(Operand::FenceArg("iorw".into())),
        Just(Operand::RoundingMode("rne".into())),
        Just(Operand::MemSymbol {
            base: "sp".into(),
            symbol: "foo".into(),
            modifier: "lo".into(),
        }),
    ]
}

fn fp_name() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("ft0".into()),
        Just("fs0".into()),
        Just("fa0".into()),
        Just("fa7".into()),
        Just("ft11".into()),
        Just("f0".into()),
        Just("f31".into()),
        Just("fs11".into()),
    ]
}

fn non_reg_operand() -> impl Strategy<Value = Operand> {
    prop_oneof![
        Just(Operand::Imm(-1)),
        Just(Operand::Imm(32)),
        Just(Operand::Imm(i64::MIN)),
        Just(Operand::Imm(i64::MAX)),
        Just(Operand::Symbol("foo".into())),
        Just(Operand::Label("L0".into())),
        Just(Operand::SymbolOffset("foo".into(), 4)),
        Just(Operand::Mem {
            base: "sp".into(),
            offset: 0,
        }),
        Just(Operand::Csr("mstatus".into())),
        Just(Operand::FenceArg("iorw".into())),
        Just(Operand::RoundingMode("rne".into())),
        Just(Operand::MemSymbol {
            base: "sp".into(),
            symbol: "foo".into(),
            modifier: "lo".into(),
        }),
        Just(Operand::Reg("x32".into())),
        Just(Operand::Reg("foo".into())),
        Just(Operand::Reg("v0".into())),
    ]
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_sfence_vma_kat_llvm_mc_bare() {
    let want = 0x1200_0073u32;
    let mc = llvm_mc_word("sfence.vma").expect("llvm-mc KAT sfence.vma");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[]).expect("SUT KAT sfence.vma");
    assert_eq!(sut, want);
    let via_zeros = sut_word(&[reg("zero"), reg("zero")]).expect("SUT KAT zero, zero");
    assert_eq!(via_zeros, want);
}

#[test]
fn encode_sfence_vma_kat_llvm_mc_a0() {
    let want = 0x1205_0073u32;
    let mc = llvm_mc_word("sfence.vma a0").expect("llvm-mc KAT sfence.vma a0");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("a0")]).expect("SUT KAT sfence.vma a0");
    assert_eq!(sut, want);
}

#[test]
fn encode_sfence_vma_kat_llvm_mc_a0_a1() {
    let want = 0x12b5_0073u32;
    let mc = llvm_mc_word("sfence.vma a0, a1").expect("llvm-mc KAT sfence.vma a0, a1");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("a0"), reg("a1")]).expect("SUT KAT sfence.vma a0, a1");
    assert_eq!(sut, want);
}

#[test]
fn encode_sfence_vma_kat_llvm_mc_x31_x31() {
    let want = 0x13ff_8073u32;
    let mc = llvm_mc_word("sfence.vma x31, x31").expect("llvm-mc KAT sfence.vma x31, x31");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("x31"), reg("x31")]).expect("SUT KAT sfence.vma x31, x31");
    assert_eq!(sut, want);
}

/// Regression: extra operand must be rejected (llvm-mc errors).
#[test]
fn test_encode_sfence_vma_regression_extra_operand() {
    let ops = [reg("x0"), reg("x0"), Operand::Imm(0)];
    assert!(
        encode_sfence_vma(&ops).is_err(),
        "sfence.vma x0, x0 with a third operand must Err; got {:?}",
        encode_sfence_vma(&ops)
    );
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_sfence_vma_diff_llvm_mc(
        arity in 0u8..=2u8,
        rs1 in gpr_name(),
        rs2 in gpr_name()
    ) {
        let (ops, asm) = match arity {
            0 => (Vec::new(), "sfence.vma".to_string()),
            1 => (vec![reg(&rs1)], format!("sfence.vma {}", rs1)),
            _ => (vec![reg(&rs1), reg(&rs2)], format!("sfence.vma {}, {}", rs1, rs2)),
        };
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&ops)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "SUT {:08x} != llvm-mc {:08x} for {}", sut, mc, asm);
    }

    #[test]
    fn encode_sfence_vma_r_type_fields(
        rs1 in 0u32..=31u32,
        rs2 in 0u32..=31u32
    ) {
        let w = sut_word(&[reg(&xn(rs1)), reg(&xn(rs2))])
            .unwrap_or_else(|e| panic!("SUT rejected sfence.vma x{}, x{}: {}", rs1, rs2, e));
        let (opc, rd, f3, got_rs1, got_rs2, f7) = unpack_sfence(w);
        prop_assert_eq!(opc, OP_SYSTEM, "opcode");
        prop_assert_eq!(rd, 0u32, "rd");
        prop_assert_eq!(f3, 0u32, "funct3");
        prop_assert_eq!(got_rs1, rs1, "rs1");
        prop_assert_eq!(got_rs2, rs2, "rs2");
        prop_assert_eq!(f7, FUNCT7_SFENCE_VMA, "funct7");
    }

    #[test]
    fn encode_sfence_vma_defaults(rs1 in gpr_name()) {
        let empty = sut_word(&[]).unwrap_or_else(|e| panic!("empty rejected: {}", e));
        let via_zero = sut_word(&[reg("zero"), reg("zero")])
            .unwrap_or_else(|e| panic!("zero,zero rejected: {}", e));
        let via_x0 = sut_word(&[reg("x0"), reg("x0")])
            .unwrap_or_else(|e| panic!("x0,x0 rejected: {}", e));
        prop_assert_eq!(empty, via_zero);
        prop_assert_eq!(empty, via_x0);

        let one = sut_word(&[reg(&rs1)])
            .unwrap_or_else(|e| panic!("one-operand {} rejected: {}", rs1, e));
        let one_zero = sut_word(&[reg(&rs1), reg("zero")])
            .unwrap_or_else(|e| panic!("{}, zero rejected: {}", rs1, e));
        let one_x0 = sut_word(&[reg(&rs1), reg("x0")])
            .unwrap_or_else(|e| panic!("{}, x0 rejected: {}", rs1, e));
        prop_assert_eq!(one, one_zero);
        prop_assert_eq!(one, one_x0);
    }

    #[test]
    fn encode_sfence_vma_abi_xn_alias(
        n in 0u32..=31u32,
        m in 0u32..=31u32
    ) {
        let via_x = sut_word(&[reg(&xn(n)), reg(&xn(m))])
            .unwrap_or_else(|e| panic!("xN rejected: {}", e));
        let via_abi = sut_word(&[reg(abi_name(n)), reg(abi_name(m))])
            .unwrap_or_else(|e| panic!("ABI rejected: {}", e));
        prop_assert_eq!(via_abi, via_x);
        if n == 8 {
            let via_fp = sut_word(&[reg("fp"), reg(&xn(m))])
                .unwrap_or_else(|e| panic!("fp rs1 rejected: {}", e));
            prop_assert_eq!(via_fp, via_x);
        }
        if m == 8 {
            let via_fp = sut_word(&[reg(&xn(n)), reg("fp")])
                .unwrap_or_else(|e| panic!("fp rs2 rejected: {}", e));
            prop_assert_eq!(via_fp, via_x);
        }
    }

    #[test]
    fn encode_sfence_vma_neg_extra(
        rs1 in gpr_name(),
        rs2 in gpr_name(),
        extra in extra_operand()
    ) {
        let asm = format!("sfence.vma {}, {}", rs1, rs2);
        let ops = vec![reg(&rs1), reg(&rs2), extra];
        prop_assert!(
            encode_sfence_vma(&ops).is_err(),
            "extra operand must Err for {} (llvm-mc rejects extra operands); got {:?}",
            asm,
            encode_sfence_vma(&ops)
        );
    }

    #[test]
    fn encode_sfence_vma_neg_fp(
        gpr in gpr_name(),
        fp in fp_name()
    ) {
        let one = [reg(&fp)];
        prop_assert!(
            encode_sfence_vma(&one).is_err(),
            "FP rs1 {} must Err (not a GPR); got {:?}",
            fp,
            encode_sfence_vma(&one)
        );
        let fp_rs1 = [reg(&fp), reg(&gpr)];
        prop_assert!(
            encode_sfence_vma(&fp_rs1).is_err(),
            "FP rs1 {} must Err (not a GPR); got {:?}",
            fp,
            encode_sfence_vma(&fp_rs1)
        );
        let fp_rs2 = [reg(&gpr), reg(&fp)];
        prop_assert!(
            encode_sfence_vma(&fp_rs2).is_err(),
            "FP rs2 {} must Err (not a GPR); got {:?}",
            fp,
            encode_sfence_vma(&fp_rs2)
        );
    }

    #[test]
    fn encode_sfence_vma_neg_nonreg(
        gpr in gpr_name(),
        bad in non_reg_operand()
    ) {
        let one = [bad.clone()];
        prop_assert!(
            encode_sfence_vma(&one).is_err(),
            "non-Reg/invalid slot 0 must Err; got {:?}",
            encode_sfence_vma(&one)
        );
        let as_rs2 = [reg(&gpr), bad.clone()];
        prop_assert!(
            encode_sfence_vma(&as_rs2).is_err(),
            "non-Reg/invalid slot 1 must Err; got {:?}",
            encode_sfence_vma(&as_rs2)
        );
        let as_rs1 = [bad, reg(&gpr)];
        prop_assert!(
            encode_sfence_vma(&as_rs1).is_err(),
            "non-Reg/invalid slot 0 with GPR rs2 must Err; got {:?}",
            encode_sfence_vma(&as_rs1)
        );
    }
}
