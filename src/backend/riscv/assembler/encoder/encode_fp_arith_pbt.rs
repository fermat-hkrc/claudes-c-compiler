// Oracle: differential — llvm-mc RISC-V assembler (R-type OP-FP word)
// Evidence: src/backend/riscv/assembler/README.md:6-7 textual assembly from codegen;
//   README.md:13-16 RV64GC includes F (single-precision) and D (double-precision);
//   README.md:307-308 fadd/fsub/fmul/fdiv;
//   README.md:352 R-type: [funct7 | rs2 | rs1 | funct3 | rd | opcode];
//   encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words";
//   encoder/mod.rs:4 "This covers the subset of instructions emitted by our codegen (RV64GC + Zbb).";
//   encoder/mod.rs:313 "R-type: funct7[31:25] | rs2[24:20] | rs1[19:15] | funct3[14:12] | rd[11:7] | opcode[6:0]";
//   encoder/mod.rs:377 OP_OP_FP = 0b1010011;
//   encoder/mod.rs:469 "Parse a rounding mode to 3-bit encoding";
//   encoder/mod.rs:719-722 fadd.s/fsub.s/fmul.s/fdiv.s => encode_fp_arith;
//   encoder/mod.rs:747-750 fadd.d/fsub.d/fmul.d/fdiv.d => encode_fp_arith_d which calls encode_fp_arith;
//   float.rs:65 "Check for optional rounding mode";
//   parser.rs:41 "Rounding mode: rne, rtz, rdn, rup, rmm, dyn";
//   RISC-V Unprivileged ISA OP-FP: opcode=1010011, rm in bits 14:12, rd/rs1/rs2 FP regs,
//   funct7 distinguishes FADD/FSUB/FMUL/FDIV and S/D (fmt in bits 26:25).
// Stronger considered:
//   - State machine: rejected — encode_fp_arith is a pure function with no lifecycle
//   - Algebraic round-trip via in-tree decoder: rejected — no OP-FP decoder
//   - encode_r / encode_fp_arith_d / encode_fp_sgnj as differential sibling: rejected —
//     same-job gate (private packer / thin wrapper of this symbol / funct3 not rm)
// Weaker available: algebraic.invariant (R-type field unpack), algebraic.metamorphic
//   (FP ABI vs fN; 3-op == explicit dyn), negative_error (arity / GPR / extra / non-rm 4th)
// Differential: candidate=encode_fp_arith, reference=llvm-mc -triple=riscv64 -mattr=+f,+d -show-encoding,
//   SUT-boundary=internal-helper of RISC-V assembler,
//   mapping=[Reg(rd), Reg(rs1), Reg(rs2)] <-> `mn rd, rs1, rs2`;
//   optional RoundingMode <-> `, rm`.

use super::{encode_fp_arith, EncodeResult};
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

/// OP-FP arithmetic mnemonics that encode through encode_fp_arith, with funct7.
const FP_ARITH: [(&str, u32); 8] = [
    ("fadd.s", 0b0000000),
    ("fsub.s", 0b0000100),
    ("fmul.s", 0b0001000),
    ("fdiv.s", 0b0001100),
    ("fadd.d", 0b0000001),
    ("fsub.d", 0b0000101),
    ("fmul.d", 0b0001001),
    ("fdiv.d", 0b0001101),
];

const RM: [(&str, u32); 6] = [
    ("rne", 0b000),
    ("rtz", 0b001),
    ("rdn", 0b010),
    ("rup", 0b011),
    ("rmm", 0b100),
    ("dyn", 0b111),
];

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

fn sut_word(ops: &[Operand], funct7: u32) -> Result<u32, String> {
    match encode_fp_arith(ops, funct7)? {
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

fn fp_arith_mn() -> impl Strategy<Value = (&'static str, u32)> {
    prop::sample::select(FP_ARITH.to_vec())
}

fn rm_pair() -> impl Strategy<Value = (&'static str, u32)> {
    prop::sample::select(RM.to_vec())
}

fn extra_operand() -> impl Strategy<Value = Operand> {
    prop_oneof![
        Just(Operand::Imm(0)),
        Just(Operand::Imm(-1)),
        (0u32..=31).prop_map(|n| Operand::Reg(xn(n))),
        (0u32..=31).prop_map(|n| Operand::Reg(fn_name(n))),
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

fn non_rm_operand() -> impl Strategy<Value = Operand> {
    prop_oneof![
        Just(Operand::Imm(0)),
        Just(Operand::Imm(-1)),
        Just(Operand::Imm(7)),
        (0u32..=31).prop_map(|n| Operand::Reg(xn(n))),
        (0u32..=31).prop_map(|n| Operand::Reg(fn_name(n))),
        Just(Operand::Symbol("foo".into())),
        Just(Operand::Label("L0".into())),
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
        Just(Operand::FenceArg("iorw".into())),
        Just(Operand::Csr("mstatus".into())),
    ]
}

fn bad_reg_operand() -> impl Strategy<Value = Operand> {
    prop_oneof![
        Just(Operand::Imm(0)),
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
fn encode_fp_arith_kat_llvm_mc_fadd_s_fa0_fa1_fa2() {
    let want = 0x00c5_f553u32;
    let mc = llvm_mc_word("fadd.s fa0, fa1, fa2").expect("llvm-mc KAT fadd.s fa0, fa1, fa2");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("fa0"), reg("fa1"), reg("fa2")], 0b0000000)
        .expect("SUT KAT fadd.s fa0, fa1, fa2");
    assert_eq!(sut, want);
}

#[test]
fn encode_fp_arith_kat_llvm_mc_fsub_s_ft0_ft1_ft2() {
    let want = 0x0820_f053u32;
    let mc = llvm_mc_word("fsub.s ft0, ft1, ft2").expect("llvm-mc KAT fsub.s");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("ft0"), reg("ft1"), reg("ft2")], 0b0000100).expect("SUT KAT fsub.s");
    assert_eq!(sut, want);
}

#[test]
fn encode_fp_arith_kat_llvm_mc_fmul_s_fs0_fs1_fs2() {
    let want = 0x1124_f453u32;
    let mc = llvm_mc_word("fmul.s fs0, fs1, fs2").expect("llvm-mc KAT fmul.s");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("fs0"), reg("fs1"), reg("fs2")], 0b0001000).expect("SUT KAT fmul.s");
    assert_eq!(sut, want);
}

#[test]
fn encode_fp_arith_kat_llvm_mc_fdiv_s_ft11_ft0_fa0() {
    let want = 0x18a0_7fd3u32;
    let mc = llvm_mc_word("fdiv.s ft11, ft0, fa0").expect("llvm-mc KAT fdiv.s");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("ft11"), reg("ft0"), reg("fa0")], 0b0001100).expect("SUT KAT fdiv.s");
    assert_eq!(sut, want);
}

#[test]
fn encode_fp_arith_kat_llvm_mc_rm_and_d() {
    let want_rne = 0x00c5_8553u32;
    assert_eq!(llvm_mc_word("fadd.s fa0, fa1, fa2, rne").unwrap(), want_rne);
    assert_eq!(
        sut_word(&[reg("fa0"), reg("fa1"), reg("fa2"), rm_op("rne")], 0b0000000).unwrap(),
        want_rne
    );

    let want_rtz = 0x00c5_9553u32;
    assert_eq!(llvm_mc_word("fadd.s fa0, fa1, fa2, rtz").unwrap(), want_rtz);
    assert_eq!(
        sut_word(&[reg("fa0"), reg("fa1"), reg("fa2"), rm_op("rtz")], 0b0000000).unwrap(),
        want_rtz
    );

    let want_d = 0x02c5_f553u32;
    assert_eq!(llvm_mc_word("fadd.d fa0, fa1, fa2").unwrap(), want_d);
    assert_eq!(
        sut_word(&[reg("fa0"), reg("fa1"), reg("fa2")], 0b0000001).unwrap(),
        want_d
    );

    let want_f0 = 0x0020_f053u32;
    assert_eq!(llvm_mc_word("fadd.s f0, f1, f2").unwrap(), want_f0);
    assert_eq!(
        sut_word(&[reg("f0"), reg("f1"), reg("f2")], 0b0000000).unwrap(),
        want_f0
    );
}

/// Regression: extra 5th operand must be rejected (llvm-mc errors).
#[test]
fn test_encode_fp_arith_regression_extra_operand() {
    let ops = [
        reg("f0"),
        reg("f0"),
        reg("f0"),
        rm_op("rne"),
        Operand::Imm(0),
    ];
    assert!(
        encode_fp_arith(&ops, 0b0000000).is_err(),
        "fadd.s f0, f0, f0, rne with a 5th operand must Err; got {:?}",
        encode_fp_arith(&ops, 0b0000000)
    );
}

/// Regression: 4th non-RoundingMode operand must be rejected.
#[test]
fn test_encode_fp_arith_regression_non_rm_fourth() {
    let ops = [reg("f0"), reg("f0"), reg("f0"), Operand::Imm(0)];
    assert!(
        encode_fp_arith(&ops, 0b0000000).is_err(),
        "fadd.s f0, f0, f0, 0 must Err (4th operand is not a rounding mode); got {:?}",
        encode_fp_arith(&ops, 0b0000000)
    );
}

proptest! {
    #![proptest_config(cfg())]

    // Target: encoder.encode_fp_arith
    #[test]
    fn encode_fp_arith_diff_3op_llvm_mc(
        (mn, f7) in fp_arith_mn(),
        rd in fp_name(),
        rs1 in fp_name(),
        rs2 in fp_name()
    ) {
        let asm = format!("{} {}, {}, {}", mn, rd, rs1, rs2);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&[reg(&rd), reg(&rs1), reg(&rs2)], f7)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "SUT {:08x} != llvm-mc {:08x} for {}", sut, mc, asm);
    }

    // Target: encoder.encode_fp_arith
    #[test]
    fn encode_fp_arith_diff_rm_llvm_mc(
        (mn, f7) in fp_arith_mn(),
        rd in fp_name(),
        rs1 in fp_name(),
        rs2 in fp_name(),
        (rm, _) in rm_pair()
    ) {
        let asm = format!("{} {}, {}, {}, {}", mn, rd, rs1, rs2, rm);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&[reg(&rd), reg(&rs1), reg(&rs2), rm_op(rm)], f7)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "SUT {:08x} != llvm-mc {:08x} for {}", sut, mc, asm);
    }

    // Target: encoder.encode_fp_arith
    #[test]
    fn encode_fp_arith_r_type_fields(
        (_mn, f7) in fp_arith_mn(),
        rd in 0u32..=31u32,
        rs1 in 0u32..=31u32,
        rs2 in 0u32..=31u32,
        (rm_name, rm) in rm_pair()
    ) {
        let w = sut_word(
            &[reg(&fn_name(rd)), reg(&fn_name(rs1)), reg(&fn_name(rs2)), rm_op(rm_name)],
            f7,
        )
        .unwrap_or_else(|e| panic!("SUT rejected f{rd}, f{rs1}, f{rs2}, {rm_name}: {e}"));
        let (opc, got_rm, got_rd, got_rs1, got_rs2, got_f7) = unpack_r(w);
        prop_assert_eq!(opc, OP_OP_FP, "opcode");
        prop_assert_eq!(got_rm, rm, "rm/funct3");
        prop_assert_eq!(got_rd, rd, "rd");
        prop_assert_eq!(got_rs1, rs1, "rs1");
        prop_assert_eq!(got_rs2, rs2, "rs2");
        prop_assert_eq!(got_f7, f7, "funct7");
    }

    // Target: encoder.encode_fp_arith
    #[test]
    fn encode_fp_arith_abi_fn_alias(
        (_mn, f7) in fp_arith_mn(),
        n in 0u32..=31u32,
        m in 0u32..=31u32,
        p in 0u32..=31u32
    ) {
        let via_n = sut_word(&[reg(&fn_name(n)), reg(&fn_name(m)), reg(&fn_name(p))], f7)
            .unwrap_or_else(|e| panic!("fN rejected: {e}"));
        let via_abi = sut_word(&[reg(fabi(n)), reg(fabi(m)), reg(fabi(p))], f7)
            .unwrap_or_else(|e| panic!("FABI rejected: {e}"));
        prop_assert_eq!(via_abi, via_n);
    }

    // Target: encoder.encode_fp_arith
    #[test]
    fn encode_fp_arith_rm_default_dyn(
        (_mn, f7) in fp_arith_mn(),
        rd in fp_name(),
        rs1 in fp_name(),
        rs2 in fp_name()
    ) {
        let three = sut_word(&[reg(&rd), reg(&rs1), reg(&rs2)], f7)
            .unwrap_or_else(|e| panic!("3-op rejected: {e}"));
        let dyn4 = sut_word(&[reg(&rd), reg(&rs1), reg(&rs2), rm_op("dyn")], f7)
            .unwrap_or_else(|e| panic!("dyn rejected: {e}"));
        prop_assert_eq!(three, dyn4);
        let (opc, rm, _, _, _, _) = unpack_r(three);
        prop_assert_eq!(opc, OP_OP_FP);
        prop_assert_eq!(rm, 0b111u32, "omitted rm must be DYN");
    }

    // Target: encoder.encode_fp_arith
    #[test]
    fn encode_fp_arith_neg_arity_gpr(
        (_mn, f7) in fp_arith_mn(),
        fp in fp_name(),
        fp2 in fp_name(),
        gpr in gpr_name(),
        bad in bad_reg_operand()
    ) {
        prop_assert!(
            encode_fp_arith(&[], f7).is_err(),
            "empty operand list must Err; got {:?}",
            encode_fp_arith(&[], f7)
        );
        prop_assert!(
            encode_fp_arith(&[reg(&fp)], f7).is_err(),
            "one operand must Err; got {:?}",
            encode_fp_arith(&[reg(&fp)], f7)
        );
        prop_assert!(
            encode_fp_arith(&[reg(&fp), reg(&fp2)], f7).is_err(),
            "two operands must Err; got {:?}",
            encode_fp_arith(&[reg(&fp), reg(&fp2)], f7)
        );
        let gpr_rd = [reg(&gpr), reg(&fp), reg(&fp2)];
        prop_assert!(
            encode_fp_arith(&gpr_rd, f7).is_err(),
            "GPR rd {} must Err; got {:?}",
            gpr,
            encode_fp_arith(&gpr_rd, f7)
        );
        let gpr_rs1 = [reg(&fp), reg(&gpr), reg(&fp2)];
        prop_assert!(
            encode_fp_arith(&gpr_rs1, f7).is_err(),
            "GPR rs1 {} must Err; got {:?}",
            gpr,
            encode_fp_arith(&gpr_rs1, f7)
        );
        let gpr_rs2 = [reg(&fp), reg(&fp2), reg(&gpr)];
        prop_assert!(
            encode_fp_arith(&gpr_rs2, f7).is_err(),
            "GPR rs2 {} must Err; got {:?}",
            gpr,
            encode_fp_arith(&gpr_rs2, f7)
        );
        let bad_rd = [bad.clone(), reg(&fp), reg(&fp2)];
        prop_assert!(
            encode_fp_arith(&bad_rd, f7).is_err(),
            "non-reg rd must Err; got {:?}",
            encode_fp_arith(&bad_rd, f7)
        );
    }

    // Target: encoder.encode_fp_arith
    #[test]
    fn encode_fp_arith_neg_extra(
        (mn, f7) in fp_arith_mn(),
        rd in fp_name(),
        rs1 in fp_name(),
        rs2 in fp_name(),
        extra in extra_operand()
    ) {
        let asm = format!("{} {}, {}, {}, rne", mn, rd, rs1, rs2);
        let ops = vec![reg(&rd), reg(&rs1), reg(&rs2), rm_op("rne"), extra];
        prop_assert!(
            encode_fp_arith(&ops, f7).is_err(),
            "5th operand must Err for {} (llvm-mc rejects extra operands); got {:?}",
            asm,
            encode_fp_arith(&ops, f7)
        );
    }

    // Target: encoder.encode_fp_arith
    #[test]
    fn encode_fp_arith_neg_non_rm_fourth(
        (mn, f7) in fp_arith_mn(),
        rd in fp_name(),
        rs1 in fp_name(),
        rs2 in fp_name(),
        extra in non_rm_operand()
    ) {
        let asm = format!("{} {}, {}, {}", mn, rd, rs1, rs2);
        let ops = [reg(&rd), reg(&rs1), reg(&rs2), extra];
        prop_assert!(
            encode_fp_arith(&ops, f7).is_err(),
            "4th non-RoundingMode operand must Err for {} (optional rm only); got {:?}",
            asm,
            encode_fp_arith(&ops, f7)
        );
    }
}
