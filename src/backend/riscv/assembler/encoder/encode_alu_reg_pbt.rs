// Oracle: differential — llvm-mc RISC-V assembler (R-type OP word)
// Evidence: src/backend/riscv/assembler/README.md:6-7 textual assembly from codegen;
//   README.md:297 R-type includes add/sub/sll/slt/sltu/xor/srl/sra/or/and/mul/div;
//   README.md:352 R-type: [funct7 | rs2 | rs1 | funct3 | rd | opcode];
//   encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words";
//   encoder/mod.rs:4 "This covers the subset of instructions emitted by our codegen (RV64GC + Zbb).";
//   encoder/mod.rs:291 "R-type: funct7[31:25] | rs2[24:20] | rs1[19:15] | funct3[14:12] | rd[11:7] | opcode[6:0]";
//   encoder/mod.rs:345 OP_OP = 0b0110011;
//   encoder/mod.rs:512-631 add/sub/sll/slt/sltu/xor/srl/sra/or/and and M/Zbb => encode_alu_reg;
//   encoder/mod.rs:376 "GCC sometimes emits bare register numbers (0-31) in inline asm";
//   RISC-V Unprivileged ISA OP: opcode=0110011, funct3/funct7 by op, rd, rs1, rs2.
// Stronger considered:
//   - State machine: rejected — encode_alu_reg is a pure function with no lifecycle
//   - Algebraic round-trip via in-tree decoder: rejected — no OP/R-type decoder
//   - encode_r / encode_alu_reg_w / C.ADD as differential sibling: rejected —
//     same-job gate (private packer / OP-32 / compressed)
// Weaker available: algebraic.invariant (R-type field unpack), algebraic.metamorphic
//   (ABI vs xN alias; Imm 0-31 vs xN), negative_error (extra / FP / empty / oob Imm)
// Differential: candidate=encode_alu_reg, reference=llvm-mc -triple=riscv64 -mattr=+m,+zbb -show-encoding,
//   SUT-boundary=internal-helper of RISC-V assembler,
//   mapping=[Reg(rd), Reg(rs1), Reg(rs2)] <-> `mn rd, rs1, rs2`.

use super::{encode_alu_reg, EncodeResult};
use crate::backend::riscv::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;
const OP_OP: u32 = 0b0110011;

const ABI: [&str; 32] = [
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3",
    "a4", "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11",
    "t3", "t4", "t5", "t6",
];

/// OP (opcode 0110011) mnemonics that dispatch to encode_alu_reg, with (funct3, funct7).
const OP_MN: [(&str, u32, u32); 27] = [
    ("add", 0b000, 0b0000000),
    ("sub", 0b000, 0b0100000),
    ("sll", 0b001, 0b0000000),
    ("slt", 0b010, 0b0000000),
    ("sltu", 0b011, 0b0000000),
    ("xor", 0b100, 0b0000000),
    ("srl", 0b101, 0b0000000),
    ("sra", 0b101, 0b0100000),
    ("or", 0b110, 0b0000000),
    ("and", 0b111, 0b0000000),
    ("mul", 0b000, 0b0000001),
    ("mulh", 0b001, 0b0000001),
    ("mulhsu", 0b010, 0b0000001),
    ("mulhu", 0b011, 0b0000001),
    ("div", 0b100, 0b0000001),
    ("divu", 0b101, 0b0000001),
    ("rem", 0b110, 0b0000001),
    ("remu", 0b111, 0b0000001),
    ("andn", 0b111, 0b0100000),
    ("orn", 0b110, 0b0100000),
    ("xnor", 0b100, 0b0100000),
    ("max", 0b110, 0b0000101),
    ("maxu", 0b111, 0b0000101),
    ("min", 0b100, 0b0000101),
    ("minu", 0b101, 0b0000101),
    ("rol", 0b001, 0b0110000),
    ("ror", 0b101, 0b0110000),
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

/// Unpack R-type per RISC-V unprivileged ISA (not a copy of encode_r).
/// Layout: funct7[31:25] | rs2[24:20] | rs1[19:15] | funct3[14:12] | rd[11:7] | opcode[6:0]
fn unpack_r(word: u32) -> (u32, u32, u32, u32, u32, u32) {
    let opcode = word & 0x7f;
    let rd = (word >> 7) & 0x1f;
    let funct3 = (word >> 12) & 0x7;
    let rs1 = (word >> 15) & 0x1f;
    let rs2 = (word >> 20) & 0x1f;
    let funct7 = (word >> 25) & 0x7f;
    (opcode, funct3, rd, rs1, rs2, funct7)
}

fn sut_word(ops: &[Operand], funct3: u32, funct7: u32) -> Result<u32, String> {
    match encode_alu_reg(ops, funct3, funct7)? {
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
        .args(["-triple=riscv64", "-mattr=+m,+zbb", "-show-encoding"])
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

fn op_mn() -> impl Strategy<Value = (&'static str, u32, u32)> {
    prop::sample::select(OP_MN.to_vec())
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

fn bad_reg_operand() -> impl Strategy<Value = Operand> {
    prop_oneof![
        Just(Operand::Csr("mstatus".into())),
        Just(Operand::FenceArg("iorw".into())),
        Just(Operand::RoundingMode("rne".into())),
        Just(Operand::Label("L0".into())),
        Just(Operand::Symbol("foo".into())),
        Just(Operand::SymbolOffset("foo".into(), 4)),
        Just(Operand::Mem {
            base: "sp".into(),
            offset: 8,
        }),
        Just(Operand::MemSymbol {
            base: "sp".into(),
            symbol: "%lo(foo)".into(),
            modifier: String::new(),
        }),
    ]
}

fn invalid_gpr_name() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("x32".into()),
        Just("x33".into()),
        Just("x99".into()),
        Just("foo".into()),
        Just("v0".into()),
        Just("v31".into()),
        Just("xzr".into()),
        Just("w0".into()),
    ]
}

fn oob_imm_reg() -> impl Strategy<Value = i64> {
    prop_oneof![
        Just(-1i64),
        Just(-2i64),
        Just(32i64),
        Just(33i64),
        Just(63i64),
        Just(64i64),
        Just(255i64),
        Just(i64::MIN),
        Just(i64::MAX),
        32i64..=i64::MAX,
        i64::MIN..=-1,
    ]
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_alu_reg_kat_llvm_mc_add_x1_x2_x3() {
    let want = 0x0031_00b3u32;
    let mc = llvm_mc_word("add x1, x2, x3").expect("llvm-mc KAT add x1, x2, x3");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("x1"), reg("x2"), reg("x3")], 0b000, 0b0000000)
        .expect("SUT KAT add x1, x2, x3");
    assert_eq!(sut, want);
}

#[test]
fn encode_alu_reg_kat_llvm_mc_sub() {
    let want = 0x4031_00b3u32;
    let mc = llvm_mc_word("sub x1, x2, x3").expect("llvm-mc KAT sub");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("x1"), reg("x2"), reg("x3")], 0b000, 0b0100000).expect("SUT KAT sub");
    assert_eq!(sut, want);
}

#[test]
fn encode_alu_reg_kat_llvm_mc_and_xor_or() {
    let want_and = 0x0031_70b3u32;
    assert_eq!(llvm_mc_word("and x1, x2, x3").unwrap(), want_and);
    assert_eq!(
        sut_word(&[reg("x1"), reg("x2"), reg("x3")], 0b111, 0b0000000).unwrap(),
        want_and
    );

    let want_xor = 0x0031_40b3u32;
    assert_eq!(llvm_mc_word("xor x1, x2, x3").unwrap(), want_xor);
    assert_eq!(
        sut_word(&[reg("x1"), reg("x2"), reg("x3")], 0b100, 0b0000000).unwrap(),
        want_xor
    );

    let want_or = 0x0031_60b3u32;
    assert_eq!(llvm_mc_word("or x1, x2, x3").unwrap(), want_or);
    assert_eq!(
        sut_word(&[reg("x1"), reg("x2"), reg("x3")], 0b110, 0b0000000).unwrap(),
        want_or
    );
}

#[test]
fn encode_alu_reg_kat_llvm_mc_shifts_slt() {
    let want_sll = 0x0031_10b3u32;
    assert_eq!(llvm_mc_word("sll x1, x2, x3").unwrap(), want_sll);
    assert_eq!(
        sut_word(&[reg("x1"), reg("x2"), reg("x3")], 0b001, 0b0000000).unwrap(),
        want_sll
    );

    let want_srl = 0x0031_50b3u32;
    assert_eq!(llvm_mc_word("srl x1, x2, x3").unwrap(), want_srl);
    assert_eq!(
        sut_word(&[reg("x1"), reg("x2"), reg("x3")], 0b101, 0b0000000).unwrap(),
        want_srl
    );

    let want_sra = 0x4031_50b3u32;
    assert_eq!(llvm_mc_word("sra x1, x2, x3").unwrap(), want_sra);
    assert_eq!(
        sut_word(&[reg("x1"), reg("x2"), reg("x3")], 0b101, 0b0100000).unwrap(),
        want_sra
    );

    let want_slt = 0x0031_20b3u32;
    assert_eq!(llvm_mc_word("slt x1, x2, x3").unwrap(), want_slt);
    assert_eq!(
        sut_word(&[reg("x1"), reg("x2"), reg("x3")], 0b010, 0b0000000).unwrap(),
        want_slt
    );

    let want_sltu = 0x0031_30b3u32;
    assert_eq!(llvm_mc_word("sltu x1, x2, x3").unwrap(), want_sltu);
    assert_eq!(
        sut_word(&[reg("x1"), reg("x2"), reg("x3")], 0b011, 0b0000000).unwrap(),
        want_sltu
    );
}

#[test]
fn encode_alu_reg_kat_llvm_mc_mul_zero_zbb() {
    let want_mul = 0x0231_00b3u32;
    assert_eq!(llvm_mc_word("mul x1, x2, x3").unwrap(), want_mul);
    assert_eq!(
        sut_word(&[reg("x1"), reg("x2"), reg("x3")], 0b000, 0b0000001).unwrap(),
        want_mul
    );

    let want_zero = 0x0000_0033u32;
    assert_eq!(llvm_mc_word("add x0, x0, x0").unwrap(), want_zero);
    assert_eq!(
        sut_word(&[reg("x0"), reg("x0"), reg("x0")], 0b000, 0b0000000).unwrap(),
        want_zero
    );

    let want_andn = 0x4031_70b3u32;
    assert_eq!(llvm_mc_word("andn x1, x2, x3").unwrap(), want_andn);
    assert_eq!(
        sut_word(&[reg("x1"), reg("x2"), reg("x3")], 0b111, 0b0100000).unwrap(),
        want_andn
    );

    let want_rol = 0x6031_10b3u32;
    assert_eq!(llvm_mc_word("rol x1, x2, x3").unwrap(), want_rol);
    assert_eq!(
        sut_word(&[reg("x1"), reg("x2"), reg("x3")], 0b001, 0b0110000).unwrap(),
        want_rol
    );
}

/// Regression: extra operand must be rejected (llvm-mc errors).
#[test]
fn test_encode_alu_reg_regression_extra_operand() {
    let ops = [reg("x1"), reg("x2"), reg("x3"), Operand::Imm(0)];
    assert!(
        encode_alu_reg(&ops, 0b000, 0b0000000).is_err(),
        "add x1, x2, x3 with a fourth operand must Err; got {:?}",
        encode_alu_reg(&ops, 0b000, 0b0000000)
    );
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_alu_reg_diff_llvm_mc(
        (mn, f3, f7) in op_mn(),
        rd in gpr_name(),
        rs1 in gpr_name(),
        rs2 in gpr_name()
    ) {
        let asm = format!("{} {}, {}, {}", mn, rd, rs1, rs2);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&[reg(&rd), reg(&rs1), reg(&rs2)], f3, f7)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "SUT {:08x} != llvm-mc {:08x} for {}", sut, mc, asm);
    }

    #[test]
    fn encode_alu_reg_r_type_fields(
        (mn, f3, f7) in op_mn(),
        rd in 0u32..=31u32,
        rs1 in 0u32..=31u32,
        rs2 in 0u32..=31u32
    ) {
        let w = sut_word(&[reg(&xn(rd)), reg(&xn(rs1)), reg(&xn(rs2))], f3, f7)
            .unwrap_or_else(|e| panic!("SUT rejected {} x{}, x{}, x{}: {}", mn, rd, rs1, rs2, e));
        let (opc, got_f3, got_rd, got_rs1, got_rs2, got_f7) = unpack_r(w);
        prop_assert_eq!(opc, OP_OP, "opcode");
        prop_assert_eq!(got_f3, f3, "funct3");
        prop_assert_eq!(got_rd, rd, "rd");
        prop_assert_eq!(got_rs1, rs1, "rs1");
        prop_assert_eq!(got_rs2, rs2, "rs2");
        prop_assert_eq!(got_f7, f7, "funct7");
    }

    #[test]
    fn encode_alu_reg_abi_xn_alias(
        (_mn, f3, f7) in op_mn(),
        n in 0u32..=31u32,
        m in 0u32..=31u32,
        k in 0u32..=31u32
    ) {
        let via_x = sut_word(&[reg(&xn(n)), reg(&xn(m)), reg(&xn(k))], f3, f7)
            .unwrap_or_else(|e| panic!("xN rejected: {}", e));
        let via_abi = sut_word(&[reg(abi_name(n)), reg(abi_name(m)), reg(abi_name(k))], f3, f7)
            .unwrap_or_else(|e| panic!("ABI rejected: {}", e));
        prop_assert_eq!(via_abi, via_x);
        if n == 8 {
            let via_fp = sut_word(&[reg("fp"), reg(&xn(m)), reg(&xn(k))], f3, f7)
                .unwrap_or_else(|e| panic!("fp rd rejected: {}", e));
            prop_assert_eq!(via_fp, via_x);
        }
        if m == 8 {
            let via_fp = sut_word(&[reg(&xn(n)), reg("fp"), reg(&xn(k))], f3, f7)
                .unwrap_or_else(|e| panic!("fp rs1 rejected: {}", e));
            prop_assert_eq!(via_fp, via_x);
        }
        if k == 8 {
            let via_fp = sut_word(&[reg(&xn(n)), reg(&xn(m)), reg("fp")], f3, f7)
                .unwrap_or_else(|e| panic!("fp rs2 rejected: {}", e));
            prop_assert_eq!(via_fp, via_x);
        }
    }

    #[test]
    fn encode_alu_reg_imm_as_reg(
        (_mn, f3, f7) in op_mn(),
        n in 0u32..=31u32,
        m in 0u32..=31u32,
        k in 0u32..=31u32
    ) {
        let via_x = sut_word(&[reg(&xn(n)), reg(&xn(m)), reg(&xn(k))], f3, f7)
            .unwrap_or_else(|e| panic!("xN rejected: {}", e));
        let via_imm = sut_word(
            &[Operand::Imm(n as i64), Operand::Imm(m as i64), Operand::Imm(k as i64)],
            f3,
            f7,
        )
        .unwrap_or_else(|e| panic!("Imm 0-31 rejected: {}", e));
        prop_assert_eq!(via_imm, via_x);
    }

    #[test]
    fn encode_alu_reg_neg_extra(
        (mn, f3, f7) in op_mn(),
        rd in gpr_name(),
        rs1 in gpr_name(),
        rs2 in gpr_name(),
        extra in extra_operand()
    ) {
        let asm = format!("{} {}, {}, {}", mn, rd, rs1, rs2);
        let ops = vec![reg(&rd), reg(&rs1), reg(&rs2), extra];
        prop_assert!(
            encode_alu_reg(&ops, f3, f7).is_err(),
            "extra operand must Err for {} (llvm-mc rejects extra operands); got {:?}",
            asm,
            encode_alu_reg(&ops, f3, f7)
        );
    }

    #[test]
    fn encode_alu_reg_neg_arity_fp(
        (_mn, f3, f7) in op_mn(),
        rd in gpr_name(),
        rs1 in gpr_name(),
        rs2 in gpr_name(),
        fp in fp_name(),
        bad in bad_reg_operand()
    ) {
        prop_assert!(
            encode_alu_reg(&[], f3, f7).is_err(),
            "empty operand list must Err; got {:?}",
            encode_alu_reg(&[], f3, f7)
        );
        prop_assert!(
            encode_alu_reg(&[reg(&rd)], f3, f7).is_err(),
            "missing rs1/rs2 must Err; got {:?}",
            encode_alu_reg(&[reg(&rd)], f3, f7)
        );
        prop_assert!(
            encode_alu_reg(&[reg(&rd), reg(&rs1)], f3, f7).is_err(),
            "missing rs2 must Err; got {:?}",
            encode_alu_reg(&[reg(&rd), reg(&rs1)], f3, f7)
        );
        let fp_rd = [reg(&fp), reg(&rs1), reg(&rs2)];
        prop_assert!(
            encode_alu_reg(&fp_rd, f3, f7).is_err(),
            "FP dest {} must Err (not a GPR); got {:?}",
            fp,
            encode_alu_reg(&fp_rd, f3, f7)
        );
        let fp_rs1 = [reg(&rd), reg(&fp), reg(&rs2)];
        prop_assert!(
            encode_alu_reg(&fp_rs1, f3, f7).is_err(),
            "FP rs1 {} must Err (not a GPR); got {:?}",
            fp,
            encode_alu_reg(&fp_rs1, f3, f7)
        );
        let fp_rs2 = [reg(&rd), reg(&rs1), reg(&fp)];
        prop_assert!(
            encode_alu_reg(&fp_rs2, f3, f7).is_err(),
            "FP rs2 {} must Err (not a GPR); got {:?}",
            fp,
            encode_alu_reg(&fp_rs2, f3, f7)
        );
        let bad_ops = [reg(&rd), reg(&rs1), bad];
        prop_assert!(
            encode_alu_reg(&bad_ops, f3, f7).is_err(),
            "non-register 3rd operand must Err; got {:?}",
            encode_alu_reg(&bad_ops, f3, f7)
        );
    }

    #[test]
    fn encode_alu_reg_neg_oob_imm(
        (_mn, f3, f7) in op_mn(),
        rd in gpr_name(),
        rs1 in gpr_name(),
        imm in oob_imm_reg()
    ) {
        prop_assume!(!(0..=31).contains(&imm));
        let ops = [reg(&rd), reg(&rs1), Operand::Imm(imm)];
        prop_assert!(
            encode_alu_reg(&ops, f3, f7).is_err(),
            "Imm {} outside 0..=31 must Err (get_reg GCC bare-reg window); got {:?}",
            imm,
            encode_alu_reg(&ops, f3, f7)
        );
    }

    #[test]
    fn encode_alu_reg_neg_invalid_name(
        (_mn, f3, f7) in op_mn(),
        rd in gpr_name(),
        rs1 in gpr_name(),
        rs2 in gpr_name(),
        bad in invalid_gpr_name(),
        which in 0u32..=2u32
    ) {
        let mut ops = vec![reg(&rd), reg(&rs1), reg(&rs2)];
        ops[which as usize] = reg(&bad);
        prop_assert!(
            encode_alu_reg(&ops, f3, f7).is_err(),
            "invalid integer register {} at operand {} must Err; got {:?}",
            bad,
            which,
            encode_alu_reg(&ops, f3, f7)
        );
    }
}
