// Oracle: differential — llvm-mc RISC-V assembler (R-type OP-FP compare word)
// Evidence: src/backend/riscv/assembler/README.md:6-7 textual assembly from codegen;
//   README.md:13-16 RV64GC includes F (single-precision) and D (double-precision);
//   README.md:309 feq/flt/fle;
//   README.md:352 R-type: [funct7 | rs2 | rs1 | funct3 | rd | opcode];
//   encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words";
//   encoder/mod.rs:4 "This covers the subset of instructions emitted by our codegen (RV64GC + Zbb).";
//   encoder/mod.rs:313 "R-type: funct7[31:25] | rs2[24:20] | rs1[19:15] | funct3[14:12] | rd[11:7] | opcode[6:0]";
//   encoder/mod.rs:377 OP_OP_FP = 0b1010011;
//   encoder/mod.rs:737-739 feq.s/flt.s/fle.s => encode_fp_cmp;
//   encoder/mod.rs:765-767 D-extension counterparts;
//   float.rs:103 "Result goes to integer register";
//   RISC-V Unprivileged ISA OP-FP compare: opcode=1010011, rd integer, rs1/rs2 FP,
//   funct3 selects the op (000 FLE, 001 FLT, 010 FEQ), funct7 encodes funct5+fmt
//   (1010000 S / 1010001 D). No rounding-mode field.
// Stronger considered:
//   - State machine: rejected — encode_fp_cmp is a pure function with no lifecycle
//   - Algebraic round-trip via in-tree decoder: rejected — no OP-FP decoder
//   - encode_r / encode_fp_arith / encode_fp_sgnj as differential sibling: rejected —
//     same-job gate (private packer / FADD-family uses rm in funct3 / FSGNJ uses FP rd)
// Weaker available: algebraic.invariant (R-type field unpack), algebraic.metamorphic
//   (GPR ABI vs xN; FP ABI vs fN), negative_error (arity / class / extra / rm-as-fourth)
// Differential: candidate=encode_fp_cmp, reference=llvm-mc -triple=riscv64 -mattr=+f,+d -show-encoding,
//   SUT-boundary=internal-helper of RISC-V assembler,
//   mapping=[Reg(rd_gpr), Reg(rs1_fp), Reg(rs2_fp)] <-> `mn rd, rs1, rs2`.

use super::{encode_fp_cmp, EncodeResult};
use crate::backend::riscv::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;
const OP_OP_FP: u32 = 0b1010011;

const FABI: [&str; 32] = [
    "ft0", "ft1", "ft2", "ft3", "ft4", "ft5", "ft6", "ft7", "fs0", "fs1", "fa0", "fa1",
    "fa2", "fa3", "fa4", "fa5", "fa6", "fa7", "fs2", "fs3", "fs4", "fs5", "fs6", "fs7",
    "fs8", "fs9", "fs10", "fs11", "ft8", "ft9", "ft10", "ft11",
];

const GABI: [&str; 32] = [
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3",
    "a4", "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11",
    "t3", "t4", "t5", "t6",
];

/// OP-FP compare mnemonics that encode through encode_fp_cmp.
const FP_CMP: [(&str, u32, u32); 6] = [
    ("feq.s", 0b1010000, 0b010),
    ("flt.s", 0b1010000, 0b001),
    ("fle.s", 0b1010000, 0b000),
    ("feq.d", 0b1010001, 0b010),
    ("flt.d", 0b1010001, 0b001),
    ("fle.d", 0b1010001, 0b000),
];

const RM: [&str; 6] = ["rne", "rtz", "rdn", "rup", "rmm", "dyn"];

fn cfg() -> ProptestConfig {
    ProptestConfig::with_cases(CASES)
}

fn fn_name(n: u32) -> String {
    format!("f{n}")
}

fn xn(n: u32) -> String {
    format!("x{n}")
}

fn fabi(n: u32) -> &'static str {
    FABI[n as usize]
}

fn gabi(n: u32) -> &'static str {
    GABI[n as usize]
}

fn reg(name: &str) -> Operand {
    Operand::Reg(name.to_string())
}

fn rm_op(s: &str) -> Operand {
    Operand::RoundingMode(s.to_string())
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

fn sut_word(ops: &[Operand], funct7: u32, funct3: u32) -> Result<u32, String> {
    match encode_fp_cmp(ops, funct7, funct3)? {
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
        .args(["-triple=riscv64", "-mattr=+f,+d", "-show-encoding"])
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

fn fp_name() -> impl Strategy<Value = String> {
    prop_oneof![
        (0u32..=31).prop_map(fn_name),
        (0u32..=31).prop_map(|n| fabi(n).to_string()),
    ]
}

fn gpr_name() -> impl Strategy<Value = String> {
    prop_oneof![
        (0u32..=31).prop_map(xn),
        (0u32..=31).prop_map(|n| gabi(n).to_string()),
        Just("fp".into()),
    ]
}

fn fp_cmp_mn() -> impl Strategy<Value = (&'static str, u32, u32)> {
    prop::sample::select(FP_CMP.to_vec())
}

fn rm_name() -> impl Strategy<Value = &'static str> {
    prop::sample::select(RM.to_vec())
}

fn extra_operand() -> impl Strategy<Value = Operand> {
    prop_oneof![
        Just(Operand::Imm(0)),
        Just(Operand::Imm(-1)),
        Just(Operand::Imm(7)),
        (0u32..=31).prop_map(|n| Operand::Reg(fn_name(n))),
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

fn bad_reg_operand() -> impl Strategy<Value = Operand> {
    // Imm(0..=31) is intentionally accepted by get_reg as a GCC bare GPR number
    // (encoder/mod.rs:398-399). Those values are not "non-register" for rd.
    // Out-of-range immediates and non-Reg tokens remain invalid.
    prop_oneof![
        Just(Operand::Imm(-1)),
        Just(Operand::Imm(32)),
        Just(Operand::Imm(256)),
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
    ]
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_fp_cmp_kat_llvm_mc_feq_s_a0_fa1_fa2() {
    let want = 0xa0c5_a553u32;
    let mc = llvm_mc_word("feq.s a0, fa1, fa2").expect("llvm-mc KAT feq.s a0, fa1, fa2");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("a0"), reg("fa1"), reg("fa2")], 0b1010000, 0b010)
        .expect("SUT KAT feq.s a0, fa1, fa2");
    assert_eq!(sut, want);
}

#[test]
fn encode_fp_cmp_kat_llvm_mc_variants() {
    let want_lt = 0xa094_12d3u32;
    assert_eq!(llvm_mc_word("flt.s t0, fs0, fs1").unwrap(), want_lt);
    assert_eq!(
        sut_word(&[reg("t0"), reg("fs0"), reg("fs1")], 0b1010000, 0b001).unwrap(),
        want_lt
    );

    let want_le = 0xa010_0053u32;
    assert_eq!(llvm_mc_word("fle.s zero, ft0, ft1").unwrap(), want_le);
    assert_eq!(
        sut_word(&[reg("zero"), reg("ft0"), reg("ft1")], 0b1010000, 0b000).unwrap(),
        want_le
    );

    let want_eq_d = 0xa2c5_a553u32;
    assert_eq!(llvm_mc_word("feq.d a0, fa1, fa2").unwrap(), want_eq_d);
    assert_eq!(
        sut_word(&[reg("a0"), reg("fa1"), reg("fa2")], 0b1010001, 0b010).unwrap(),
        want_eq_d
    );

    let want_lt_d = 0xa3ef_9fd3u32;
    assert_eq!(llvm_mc_word("flt.d t6, ft11, ft10").unwrap(), want_lt_d);
    assert_eq!(
        sut_word(&[reg("t6"), reg("ft11"), reg("ft10")], 0b1010001, 0b001).unwrap(),
        want_lt_d
    );

    let want_le_d = 0xa210_0053u32;
    assert_eq!(llvm_mc_word("fle.d x0, f0, f1").unwrap(), want_le_d);
    assert_eq!(
        sut_word(&[reg("x0"), reg("f0"), reg("f1")], 0b1010001, 0b000).unwrap(),
        want_le_d
    );

    let want_x31 = 0xa1f0_2fd3u32;
    assert_eq!(llvm_mc_word("feq.s x31, f0, f31").unwrap(), want_x31);
    assert_eq!(
        sut_word(&[reg("x31"), reg("f0"), reg("f31")], 0b1010000, 0b010).unwrap(),
        want_x31
    );

    let want_fp = 0xa0a0_2453u32;
    assert_eq!(llvm_mc_word("feq.s fp, ft0, fa0").unwrap(), want_fp);
    assert_eq!(
        sut_word(&[reg("fp"), reg("ft0"), reg("fa0")], 0b1010000, 0b010).unwrap(),
        want_fp
    );

    let want_s0 = 0xa0a4_2453u32;
    assert_eq!(llvm_mc_word("feq.s s0, f8, f10").unwrap(), want_s0);
    assert_eq!(
        sut_word(&[reg("s0"), reg("f8"), reg("f10")], 0b1010000, 0b010).unwrap(),
        want_s0
    );
}

/// Regression: extra 4th operand must be rejected (llvm-mc errors).
#[test]
fn test_encode_fp_cmp_regression_extra_operand() {
    let ops = [reg("a0"), reg("fa0"), reg("fa1"), Operand::Imm(0)];
    assert!(
        encode_fp_cmp(&ops, 0b1010000, 0b010).is_err(),
        "feq.s a0, fa0, fa1, 0 must Err; got {:?}",
        encode_fp_cmp(&ops, 0b1010000, 0b010)
    );
}

/// Regression: 4th RoundingMode must be rejected (these ops have no rm field).
#[test]
fn test_encode_fp_cmp_regression_rm_fourth() {
    let ops = [reg("a0"), reg("fa0"), reg("fa1"), rm_op("rne")];
    assert!(
        encode_fp_cmp(&ops, 0b1010000, 0b010).is_err(),
        "feq.s a0, fa0, fa1, rne must Err; got {:?}",
        encode_fp_cmp(&ops, 0b1010000, 0b010)
    );
}

proptest! {
    #![proptest_config(cfg())]

    // Target: encoder.encode_fp_cmp
    #[test]
    fn encode_fp_cmp_diff_3op_llvm_mc(
        (mn, f7, f3) in fp_cmp_mn(),
        rd in gpr_name(),
        rs1 in fp_name(),
        rs2 in fp_name()
    ) {
        let asm = format!("{} {}, {}, {}", mn, rd, rs1, rs2);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&[reg(&rd), reg(&rs1), reg(&rs2)], f7, f3)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "SUT {:08x} != llvm-mc {:08x} for {}", sut, mc, asm);
    }

    // Target: encoder.encode_fp_cmp
    #[test]
    fn encode_fp_cmp_r_type_fields(
        (_mn, f7, f3) in fp_cmp_mn(),
        rd in 0u32..=31u32,
        rs1 in 0u32..=31u32,
        rs2 in 0u32..=31u32
    ) {
        let w = sut_word(
            &[reg(&xn(rd)), reg(&fn_name(rs1)), reg(&fn_name(rs2))],
            f7,
            f3,
        )
        .unwrap_or_else(|e| panic!("SUT rejected x{rd}, f{rs1}, f{rs2}: {e}"));
        let (opc, got_f3, got_rd, got_rs1, got_rs2, got_f7) = unpack_r(w);
        prop_assert_eq!(opc, OP_OP_FP, "opcode");
        prop_assert_eq!(got_f3, f3, "funct3");
        prop_assert_eq!(got_rd, rd, "rd");
        prop_assert_eq!(got_rs1, rs1, "rs1");
        prop_assert_eq!(got_rs2, rs2, "rs2");
        prop_assert_eq!(got_f7, f7, "funct7");
    }

    // Target: encoder.encode_fp_cmp
    #[test]
    fn encode_fp_cmp_abi_xn_fn_alias(
        (_mn, f7, f3) in fp_cmp_mn(),
        n in 0u32..=31u32,
        m in 0u32..=31u32,
        p in 0u32..=31u32
    ) {
        let via_n = sut_word(&[reg(&xn(n)), reg(&fn_name(m)), reg(&fn_name(p))], f7, f3)
            .unwrap_or_else(|e| panic!("xN/fN rejected: {e}"));
        let via_abi = sut_word(&[reg(gabi(n)), reg(fabi(m)), reg(fabi(p))], f7, f3)
            .unwrap_or_else(|e| panic!("ABI rejected: {e}"));
        prop_assert_eq!(via_abi, via_n);
    }

    // Target: encoder.encode_fp_cmp
    #[test]
    fn encode_fp_cmp_diff_rs1_eq_rs2_llvm_mc(
        (mn, f7, f3) in fp_cmp_mn(),
        rd in gpr_name(),
        rs in fp_name()
    ) {
        let asm = format!("{} {}, {}, {}", mn, rd, rs, rs);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&[reg(&rd), reg(&rs), reg(&rs)], f7, f3)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "SUT {:08x} != llvm-mc {:08x} for {}", sut, mc, asm);
    }

    // Target: encoder.encode_fp_cmp
    #[test]
    fn encode_fp_cmp_neg_arity_class(
        (_mn, f7, f3) in fp_cmp_mn(),
        fp in fp_name(),
        fp2 in fp_name(),
        gpr in gpr_name(),
        bad in bad_reg_operand()
    ) {
        prop_assert!(
            encode_fp_cmp(&[], f7, f3).is_err(),
            "empty operand list must Err; got {:?}",
            encode_fp_cmp(&[], f7, f3)
        );
        prop_assert!(
            encode_fp_cmp(&[reg(&gpr)], f7, f3).is_err(),
            "one operand must Err; got {:?}",
            encode_fp_cmp(&[reg(&gpr)], f7, f3)
        );
        prop_assert!(
            encode_fp_cmp(&[reg(&gpr), reg(&fp)], f7, f3).is_err(),
            "two operands must Err; got {:?}",
            encode_fp_cmp(&[reg(&gpr), reg(&fp)], f7, f3)
        );
        let fp_rd = [reg(&fp), reg(&fp), reg(&fp2)];
        prop_assert!(
            encode_fp_cmp(&fp_rd, f7, f3).is_err(),
            "FP rd {} must Err; got {:?}",
            fp,
            encode_fp_cmp(&fp_rd, f7, f3)
        );
        let gpr_rs1 = [reg(&gpr), reg(&gpr), reg(&fp2)];
        prop_assert!(
            encode_fp_cmp(&gpr_rs1, f7, f3).is_err(),
            "GPR rs1 {} must Err; got {:?}",
            gpr,
            encode_fp_cmp(&gpr_rs1, f7, f3)
        );
        let gpr_rs2 = [reg(&gpr), reg(&fp), reg(&gpr)];
        prop_assert!(
            encode_fp_cmp(&gpr_rs2, f7, f3).is_err(),
            "GPR rs2 {} must Err; got {:?}",
            gpr,
            encode_fp_cmp(&gpr_rs2, f7, f3)
        );
        let bad_rd = [bad.clone(), reg(&fp), reg(&fp2)];
        prop_assert!(
            encode_fp_cmp(&bad_rd, f7, f3).is_err(),
            "non-reg rd must Err; got {:?}",
            encode_fp_cmp(&bad_rd, f7, f3)
        );
        // get_freg does not accept a bare Imm (unlike get_reg for rd).
        let imm_rs1 = [reg(&gpr), Operand::Imm(0), reg(&fp2)];
        prop_assert!(
            encode_fp_cmp(&imm_rs1, f7, f3).is_err(),
            "Imm rs1 must Err; got {:?}",
            encode_fp_cmp(&imm_rs1, f7, f3)
        );
        let imm_rs2 = [reg(&gpr), reg(&fp), Operand::Imm(0)];
        prop_assert!(
            encode_fp_cmp(&imm_rs2, f7, f3).is_err(),
            "Imm rs2 must Err; got {:?}",
            encode_fp_cmp(&imm_rs2, f7, f3)
        );
    }

    // Target: encoder.encode_fp_cmp
    #[test]
    fn encode_fp_cmp_neg_extra(
        (mn, f7, f3) in fp_cmp_mn(),
        rd in gpr_name(),
        rs1 in fp_name(),
        rs2 in fp_name(),
        extra in extra_operand()
    ) {
        let asm = format!("{} {}, {}, {}", mn, rd, rs1, rs2);
        let ops = vec![reg(&rd), reg(&rs1), reg(&rs2), extra];
        prop_assert!(
            encode_fp_cmp(&ops, f7, f3).is_err(),
            "4th operand must Err for {} (llvm-mc rejects extra operands); got {:?}",
            asm,
            encode_fp_cmp(&ops, f7, f3)
        );
    }

    // Target: encoder.encode_fp_cmp
    #[test]
    fn encode_fp_cmp_neg_rm_fourth(
        (mn, f7, f3) in fp_cmp_mn(),
        rd in gpr_name(),
        rs1 in fp_name(),
        rs2 in fp_name(),
        rm in rm_name()
    ) {
        let asm = format!("{} {}, {}, {}, {}", mn, rd, rs1, rs2, rm);
        let ops = [reg(&rd), reg(&rs1), reg(&rs2), rm_op(rm)];
        prop_assert!(
            encode_fp_cmp(&ops, f7, f3).is_err(),
            "4th RoundingMode must Err for {} (FEQ/FLT/FLE have no rm); got {:?}",
            asm,
            encode_fp_cmp(&ops, f7, f3)
        );
    }
}
