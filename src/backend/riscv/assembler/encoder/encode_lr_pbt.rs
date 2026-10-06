// Oracle: differential — llvm-mc RISC-V assembler (R-type LR / A-extension)
// Evidence: src/backend/riscv/assembler/README.md:6-7 textual assembly from codegen;
//   README.md:13 A (atomics) extension;
//   README.md:104 atomics.rs A-extension LR/SC and AMO;
//   README.md:306 Atomics: lr.w/d, sc.w/d, amo{swap,add,and,or,xor,min,max,minu,maxu}.w/d;
//   README.md:352 R-type: [funct7 | rs2 | rs1 | funct3 | rd | opcode];
//   encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words";
//   encoder/mod.rs:4 "This covers the subset of instructions emitted by our codegen (RV64GC + Zbb).";
//   encoder/mod.rs:300 "R-type: funct7[31:25] | rs2[24:20] | rs1[19:15] | funct3[14:12] | rd[11:7] | opcode[6:0]";
//   encoder/mod.rs:360 OP_AMO = 0b0101111;
//   encoder/mod.rs:655-656 unsuffixed lr.w/d => encode_lr;
//   atomics.rs:8 "LR: funct7 = 00010 | aq | rl, rs2 = 0";
//   atomics.rs:9 "aq=0, rl=0 by default";
//   RISC-V Unprivileged ISA LR: opcode=0101111, funct3=010 (.w)/011 (.d),
//   funct5=00010, aq[26], rl[25], rs2=0, rd, rs1; assembly `lr.width rd, (rs1)`
//   with optional integer offset that must be 0.
// Stronger considered:
//   - State machine: rejected — encode_lr is a pure function with no lifecycle
//   - Algebraic round-trip via in-tree decoder: rejected — no LR decoder
//   - encode_r / encode_lr_suffixed / encode_sc / encode_amo as differential sibling:
//     rejected — same-job gate (private packer / aqrl-suffixed mnemonic /
//     store-conditional / AMO with rs2)
// Weaker available: algebraic.invariant (R-type field unpack), algebraic.metamorphic
//   (ABI vs xN alias), negative_error (extra / nonzero offset / arity / FP / non-Mem)
// Differential: candidate=encode_lr, reference=llvm-mc -triple=riscv64 -mattr=+a -show-encoding,
//   SUT-boundary=internal-helper of RISC-V assembler,
//   mapping=[Reg(rd), Mem{base: rs1, offset: 0}] <-> `mn rd, (rs1)`.

use super::{encode_lr, EncodeResult};
use crate::backend::riscv::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;
const OP_AMO: u32 = 0b0101111;
const FUNCT5_LR: u32 = 0b00010;

const ABI: [&str; 32] = [
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3",
    "a4", "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11",
    "t3", "t4", "t5", "t6",
];

/// Unsuffixed LR mnemonics dispatched to encode_lr, with funct3.
const LR: [(&str, u32); 2] = [("lr.w", 0b010), ("lr.d", 0b011)];

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

fn mem(base: &str, offset: i64) -> Operand {
    Operand::Mem {
        base: base.to_string(),
        offset,
    }
}

/// Unpack AMO/LR R-type per RISC-V unprivileged ISA (not a copy of encode_r).
/// Layout: funct5[31:27] | aq[26] | rl[25] | rs2[24:20] | rs1[19:15] | funct3[14:12] | rd[11:7] | opcode[6:0]
fn unpack_lr(word: u32) -> (u32, u32, u32, u32, u32, u32, u32, u32) {
    let opcode = word & 0x7f;
    let rd = (word >> 7) & 0x1f;
    let funct3 = (word >> 12) & 0x7;
    let rs1 = (word >> 15) & 0x1f;
    let rs2 = (word >> 20) & 0x1f;
    let rl = (word >> 25) & 1;
    let aq = (word >> 26) & 1;
    let funct5 = (word >> 27) & 0x1f;
    (opcode, funct3, rd, rs1, rs2, funct5, aq, rl)
}

fn sut_word(ops: &[Operand], funct3: u32) -> Result<u32, String> {
    match encode_lr(ops, funct3)? {
        EncodeResult::Word(w) => Ok(w),
        other => Err(format!("expected Word, got {other:?}")),
    }
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
        .args(["-triple=riscv64", "-mattr=+a", "-show-encoding"])
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

fn lr_mn() -> impl Strategy<Value = (&'static str, u32)> {
    prop::sample::select(LR.to_vec())
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
    ]
}

fn non_mem_operand() -> impl Strategy<Value = Operand> {
    prop_oneof![
        Just(Operand::Imm(0)),
        Just(Operand::Imm(-1)),
        (0u32..=31).prop_map(|n| Operand::Reg(xn(n))),
        Just(Operand::Symbol("foo".into())),
        Just(Operand::Label("L0".into())),
        Just(Operand::SymbolOffset("foo".into(), 4)),
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

/// Nonzero mem offsets, with the documented bound 0 sampled at ±1.
fn nonzero_offset() -> impl Strategy<Value = i64> {
    prop_oneof![
        Just(1i64),
        Just(-1i64),
        Just(2i64),
        Just(-2i64),
        Just(8i64),
        Just(-8i64),
        Just(2047i64),
        Just(-2048i64),
        Just(2048i64),
        Just(-2049i64),
        Just(i64::MIN),
        Just(i64::MAX),
        1i64..=i64::MAX,
        i64::MIN..=-1,
    ]
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_lr_kat_llvm_mc_lr_w_a0_a1() {
    let want = 0x1005_a52fu32;
    let mc = llvm_mc_word("lr.w a0, (a1)").expect("llvm-mc KAT lr.w a0, (a1)");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("a0"), mem("a1", 0)], 0b010).expect("SUT KAT lr.w a0, (a1)");
    assert_eq!(sut, want);
}

#[test]
fn encode_lr_kat_llvm_mc_lr_d_x1_x2() {
    let want = 0x1001_30afu32;
    let mc = llvm_mc_word("lr.d x1, (x2)").expect("llvm-mc KAT lr.d x1, (x2)");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("x1"), mem("x2", 0)], 0b011).expect("SUT KAT lr.d x1, (x2)");
    assert_eq!(sut, want);
}

#[test]
fn encode_lr_kat_llvm_mc_zero_offset_and_x31() {
    let want = 0x1000_3fafu32;
    let mc = llvm_mc_word("lr.d x31, (zero)").expect("llvm-mc KAT lr.d x31, (zero)");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("x31"), mem("zero", 0)], 0b011).expect("SUT KAT lr.d x31, (zero)");
    assert_eq!(sut, want);

    let via_zero = llvm_mc_word("lr.w a0, 0(a1)").expect("llvm-mc 0(a1)");
    assert_eq!(via_zero, 0x1005_a52fu32);
}

/// Regression: extra operand must be rejected (llvm-mc errors).
#[test]
fn test_encode_lr_regression_extra_operand() {
    let ops = [reg("a0"), mem("a1", 0), Operand::Imm(0)];
    assert!(
        encode_lr(&ops, 0b010).is_err(),
        "lr.w a0, (a1) with a third operand must Err; got {:?}",
        encode_lr(&ops, 0b010)
    );
}

/// Regression: nonzero mem offset must be rejected (llvm-mc: optional integer offset must be 0).
#[test]
fn test_encode_lr_regression_nonzero_offset() {
    let ops = [reg("a0"), mem("a1", 8)];
    assert!(
        encode_lr(&ops, 0b010).is_err(),
        "lr.w a0, 8(a1) must Err; got {:?}",
        encode_lr(&ops, 0b010)
    );
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_lr_diff_llvm_mc(
        (mn, f3) in lr_mn(),
        rd in gpr_name(),
        rs1 in gpr_name()
    ) {
        let asm = format!("{} {}, ({})", mn, rd, rs1);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&[reg(&rd), mem(&rs1, 0)], f3)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "SUT {:08x} != llvm-mc {:08x} for {}", sut, mc, asm);
    }

    #[test]
    fn encode_lr_r_type_fields(
        (mn, f3) in lr_mn(),
        rd in 0u32..=31u32,
        rs1 in 0u32..=31u32
    ) {
        let w = sut_word(&[reg(&xn(rd)), mem(&xn(rs1), 0)], f3)
            .unwrap_or_else(|e| panic!("SUT rejected {} x{}, (x{}): {}", mn, rd, rs1, e));
        let (opc, got_f3, got_rd, got_rs1, got_rs2, got_f5, aq, rl) = unpack_lr(w);
        prop_assert_eq!(opc, OP_AMO, "opcode");
        prop_assert_eq!(got_f3, f3, "funct3");
        prop_assert_eq!(got_rd, rd, "rd");
        prop_assert_eq!(got_rs1, rs1, "rs1");
        prop_assert_eq!(got_rs2, 0u32, "rs2 must be 0 for LR");
        prop_assert_eq!(got_f5, FUNCT5_LR, "funct5");
        prop_assert_eq!(aq, 0u32, "aq");
        prop_assert_eq!(rl, 0u32, "rl");
    }

    #[test]
    fn encode_lr_abi_xn_alias(
        (_mn, f3) in lr_mn(),
        n in 0u32..=31u32,
        k in 0u32..=31u32
    ) {
        let via_x = sut_word(&[reg(&xn(n)), mem(&xn(k), 0)], f3)
            .unwrap_or_else(|e| panic!("xN rejected: {}", e));
        let via_abi = sut_word(&[reg(abi_name(n)), mem(abi_name(k), 0)], f3)
            .unwrap_or_else(|e| panic!("ABI rejected: {}", e));
        prop_assert_eq!(via_abi, via_x);
        if n == 8 {
            let via_fp = sut_word(&[reg("fp"), mem(&xn(k), 0)], f3)
                .unwrap_or_else(|e| panic!("fp rd rejected: {}", e));
            prop_assert_eq!(via_fp, via_x);
        }
        if k == 8 {
            let via_fp = sut_word(&[reg(&xn(n)), mem("fp", 0)], f3)
                .unwrap_or_else(|e| panic!("fp base rejected: {}", e));
            prop_assert_eq!(via_fp, via_x);
        }
    }

    #[test]
    fn encode_lr_neg_extra(
        (mn, f3) in lr_mn(),
        rd in gpr_name(),
        rs1 in gpr_name(),
        extra in extra_operand()
    ) {
        let asm = format!("{} {}, ({})", mn, rd, rs1);
        let ops = vec![reg(&rd), mem(&rs1, 0), extra];
        prop_assert!(
            encode_lr(&ops, f3).is_err(),
            "extra operand must Err for {} (llvm-mc rejects extra operands); got {:?}",
            asm,
            encode_lr(&ops, f3)
        );
    }

    #[test]
    fn encode_lr_neg_nonzero_offset(
        (mn, f3) in lr_mn(),
        rd in gpr_name(),
        rs1 in gpr_name(),
        off in nonzero_offset()
    ) {
        let asm = format!("{} {}, {}({})", mn, rd, off, rs1);
        let ops = [reg(&rd), mem(&rs1, off)];
        prop_assert!(
            encode_lr(&ops, f3).is_err(),
            "nonzero offset must Err for {} (llvm-mc: optional integer offset must be 0); got {:?}",
            asm,
            encode_lr(&ops, f3)
        );
    }

    #[test]
    fn encode_lr_neg_arity_fp(
        (_mn, f3) in lr_mn(),
        rd in gpr_name(),
        rs1 in gpr_name(),
        fp in fp_name()
    ) {
        prop_assert!(
            encode_lr(&[], f3).is_err(),
            "empty operand list must Err; got {:?}",
            encode_lr(&[], f3)
        );
        prop_assert!(
            encode_lr(&[reg(&rd)], f3).is_err(),
            "missing mem must Err; got {:?}",
            encode_lr(&[reg(&rd)], f3)
        );
        let fp_rd = [reg(&fp), mem(&rs1, 0)];
        prop_assert!(
            encode_lr(&fp_rd, f3).is_err(),
            "FP dest {} must Err (not a GPR); got {:?}",
            fp,
            encode_lr(&fp_rd, f3)
        );
        let fp_base = [reg(&rd), mem(&fp, 0)];
        prop_assert!(
            encode_lr(&fp_base, f3).is_err(),
            "FP base {} must Err (not a GPR); got {:?}",
            fp,
            encode_lr(&fp_base, f3)
        );
    }

    #[test]
    fn encode_lr_neg_non_mem(
        (mn, f3) in lr_mn(),
        rd in gpr_name(),
        bad in non_mem_operand()
    ) {
        let ops = [reg(&rd), bad];
        prop_assert!(
            encode_lr(&ops, f3).is_err(),
            "non-Mem slot 1 must Err for {} (llvm-mc requires (rs1)); got {:?}",
            mn,
            encode_lr(&ops, f3)
        );
    }
}
