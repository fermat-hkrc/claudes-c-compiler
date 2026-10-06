// Oracle: differential — llvm-mc RISC-V assembler (R-type OP-FP FMV.X.W/D word)
// Evidence: src/backend/riscv/assembler/README.md:6-7 textual assembly from codegen;
//   README.md:13-16 RV64GC includes F (single-precision) and D (double-precision);
//   README.md:308 fmv.x.w/d;
//   README.md:352 R-type: [funct7 | rs2 | rs1 | funct3 | rd | opcode];
//   encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words";
//   encoder/mod.rs:4 "This covers the subset of instructions emitted by our codegen (RV64GC + Zbb).";
//   encoder/mod.rs:333 "R-type: funct7[31:25] | rs2[24:20] | rs1[19:15] | funct3[14:12] | rd[11:7] | opcode[6:0]";
//   encoder/mod.rs:389 OP_OP_FP = 0b1010011;
//   encoder/mod.rs:759 fmv.x.w | fmv.x.s => encode_fmv_x_f(operands, 0b1110000, 0b00);
//   encoder/mod.rs:789 fmv.x.d => encode_fmv_x_f(operands, 0b1110001, 0b00);
//   RISC-V Unprivileged ISA FMV.X.W/D: opcode=1010011, rd integer, rs1 FP,
//   rs2 hardwired 00000, funct3 hardwired 000, funct7 encodes funct5+fmt
//   (1110000 S / 1110001 D). No rounding-mode field. fmv.x.s aliases fmv.x.w.
// Stronger considered:
//   - State machine: rejected — encode_fmv_x_f is a pure function with no lifecycle
//   - Algebraic round-trip via in-tree decoder: rejected — no OP-FP decoder
//   - encode_r / encode_fclass / encode_fp_unary / encode_fmv_f_x as differential sibling:
//     rejected — same-job gate (private packer / FCLASS uses funct3=001 /
//     FSQRT uses rm / FMV.W.X is the opposite direction)
// Weaker available: algebraic.invariant (R-type field unpack), algebraic.metamorphic
//   (GPR ABI vs xN; FP ABI vs fN; S vs D fmt bit; fmv.x.w vs fmv.x.s alias),
//   negative_error (arity / class / extra / rm-as-third)
// Differential: candidate=encode_fmv_x_f, reference=llvm-mc -triple=riscv64 -mattr=+f,+d -show-encoding,
//   SUT-boundary=internal-helper of RISC-V assembler,
//   mapping=[Reg(rd_gpr), Reg(rs1_fp)] <-> `mn rd, rs1`.

use super::{encode_fmv_x_f, EncodeResult};
use crate::backend::riscv::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;
const OP_OP_FP: u32 = 0b1010011;
const FMV_X_FUNCT3: u32 = 0b000;
const FMT_UNUSED: u32 = 0;

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

/// OP-FP FMV.X mnemonics that encode through encode_fmv_x_f.
const FMV_X: [(&str, u32); 3] = [
    ("fmv.x.w", 0b1110000),
    ("fmv.x.s", 0b1110000),
    ("fmv.x.d", 0b1110001),
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

fn sut_word(ops: &[Operand], funct7: u32) -> Result<u32, String> {
    match encode_fmv_x_f(ops, funct7, FMT_UNUSED)? {
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

fn fmv_x_mn() -> impl Strategy<Value = (&'static str, u32)> {
    prop::sample::select(FMV_X.to_vec())
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
    // (encoder/mod.rs:410-411). Those values are not "non-register" for rd.
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
fn encode_fmv_x_f_kat_llvm_mc_w_a0_fa1() {
    let want = 0xe005_8553u32;
    let mc = llvm_mc_word("fmv.x.w a0, fa1").expect("llvm-mc KAT fmv.x.w a0, fa1");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("a0"), reg("fa1")], 0b1110000)
        .expect("SUT KAT fmv.x.w a0, fa1");
    assert_eq!(sut, want);
}

#[test]
fn encode_fmv_x_f_kat_llvm_mc_variants() {
    let want_s_alias = 0xe005_8553u32;
    assert_eq!(llvm_mc_word("fmv.x.s a0, fa1").unwrap(), want_s_alias);
    assert_eq!(
        sut_word(&[reg("a0"), reg("fa1")], 0b1110000).unwrap(),
        want_s_alias
    );

    let want_t0 = 0xe004_02d3u32;
    assert_eq!(llvm_mc_word("fmv.x.w t0, fs0").unwrap(), want_t0);
    assert_eq!(
        sut_word(&[reg("t0"), reg("fs0")], 0b1110000).unwrap(),
        want_t0
    );

    let want_zero = 0xe000_0053u32;
    assert_eq!(llvm_mc_word("fmv.x.w zero, ft0").unwrap(), want_zero);
    assert_eq!(
        sut_word(&[reg("zero"), reg("ft0")], 0b1110000).unwrap(),
        want_zero
    );

    let want_d = 0xe205_8553u32;
    assert_eq!(llvm_mc_word("fmv.x.d a0, fa1").unwrap(), want_d);
    assert_eq!(
        sut_word(&[reg("a0"), reg("fa1")], 0b1110001).unwrap(),
        want_d
    );

    let want_d_t6 = 0xe20f_8fd3u32;
    assert_eq!(llvm_mc_word("fmv.x.d t6, ft11").unwrap(), want_d_t6);
    assert_eq!(
        sut_word(&[reg("t6"), reg("ft11")], 0b1110001).unwrap(),
        want_d_t6
    );

    let want_d_x0 = 0xe200_0053u32;
    assert_eq!(llvm_mc_word("fmv.x.d x0, f0").unwrap(), want_d_x0);
    assert_eq!(
        sut_word(&[reg("x0"), reg("f0")], 0b1110001).unwrap(),
        want_d_x0
    );

    let want_x31 = 0xe000_0fd3u32;
    assert_eq!(llvm_mc_word("fmv.x.w x31, f0").unwrap(), want_x31);
    assert_eq!(
        sut_word(&[reg("x31"), reg("f0")], 0b1110000).unwrap(),
        want_x31
    );

    let want_fp = 0xe000_0453u32;
    assert_eq!(llvm_mc_word("fmv.x.w fp, ft0").unwrap(), want_fp);
    assert_eq!(
        sut_word(&[reg("fp"), reg("ft0")], 0b1110000).unwrap(),
        want_fp
    );

    let want_s0 = 0xe004_0453u32;
    assert_eq!(llvm_mc_word("fmv.x.w s0, f8").unwrap(), want_s0);
    assert_eq!(
        sut_word(&[reg("s0"), reg("f8")], 0b1110000).unwrap(),
        want_s0
    );
}

/// Regression: extra 3rd operand must be rejected (llvm-mc errors).
#[test]
fn test_encode_fmv_x_f_regression_extra_operand() {
    let ops = [reg("a0"), reg("fa0"), Operand::Imm(0)];
    assert!(
        encode_fmv_x_f(&ops, 0b1110000, FMT_UNUSED).is_err(),
        "fmv.x.w a0, fa0, 0 must Err; got {:?}",
        encode_fmv_x_f(&ops, 0b1110000, FMT_UNUSED)
    );
}

/// Regression: 3rd RoundingMode must be rejected (FMV.X has no rm field).
#[test]
fn test_encode_fmv_x_f_regression_rm_third() {
    let ops = [reg("a0"), reg("fa0"), rm_op("rne")];
    assert!(
        encode_fmv_x_f(&ops, 0b1110000, FMT_UNUSED).is_err(),
        "fmv.x.w a0, fa0, rne must Err; got {:?}",
        encode_fmv_x_f(&ops, 0b1110000, FMT_UNUSED)
    );
}

proptest! {
    #![proptest_config(cfg())]

    // Target: encoder.encode_fmv_x_f
    #[test]
    fn encode_fmv_x_f_diff_2op_llvm_mc(
        (mn, f7) in fmv_x_mn(),
        rd in gpr_name(),
        rs1 in fp_name()
    ) {
        let asm = format!("{} {}, {}", mn, rd, rs1);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&[reg(&rd), reg(&rs1)], f7)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "SUT {:08x} != llvm-mc {:08x} for {}", sut, mc, asm);
    }

    // Target: encoder.encode_fmv_x_f
    #[test]
    fn encode_fmv_x_f_r_type_fields(
        (_mn, f7) in fmv_x_mn(),
        rd in 0u32..=31u32,
        rs1 in 0u32..=31u32
    ) {
        let w = sut_word(&[reg(&xn(rd)), reg(&fn_name(rs1))], f7)
            .unwrap_or_else(|e| panic!("SUT rejected x{rd}, f{rs1}: {e}"));
        let (opc, got_f3, got_rd, got_rs1, got_rs2, got_f7) = unpack_r(w);
        prop_assert_eq!(opc, OP_OP_FP, "opcode");
        prop_assert_eq!(got_f3, FMV_X_FUNCT3, "funct3");
        prop_assert_eq!(got_rd, rd, "rd");
        prop_assert_eq!(got_rs1, rs1, "rs1");
        prop_assert_eq!(got_rs2, 0u32, "rs2");
        prop_assert_eq!(got_f7, f7, "funct7");
    }

    // Target: encoder.encode_fmv_x_f
    #[test]
    fn encode_fmv_x_f_abi_xn_fn_alias(
        (_mn, f7) in fmv_x_mn(),
        n in 0u32..=31u32,
        m in 0u32..=31u32
    ) {
        let via_n = sut_word(&[reg(&xn(n)), reg(&fn_name(m))], f7)
            .unwrap_or_else(|e| panic!("xN/fN rejected: {e}"));
        let via_abi = sut_word(&[reg(gabi(n)), reg(fabi(m))], f7)
            .unwrap_or_else(|e| panic!("ABI rejected: {e}"));
        prop_assert_eq!(via_abi, via_n);
    }

    // Target: encoder.encode_fmv_x_f
    #[test]
    fn encode_fmv_x_f_s_vs_d_fmt(
        rd in 0u32..=31u32,
        rs1 in 0u32..=31u32
    ) {
        let ops = [reg(&xn(rd)), reg(&fn_name(rs1))];
        let ws = sut_word(&ops, 0b1110000)
            .unwrap_or_else(|e| panic!("SUT rejected fmv.x.w x{rd}, f{rs1}: {e}"));
        let wd = sut_word(&ops, 0b1110001)
            .unwrap_or_else(|e| panic!("SUT rejected fmv.x.d x{rd}, f{rs1}: {e}"));
        prop_assert_eq!(ws ^ wd, 1u32 << 25, "S vs D must differ only in funct7 bit 0");
        let (_, _, _, _, _, f7s) = unpack_r(ws);
        let (_, _, _, _, _, f7d) = unpack_r(wd);
        prop_assert_eq!(f7s, 0b1110000u32);
        prop_assert_eq!(f7d, 0b1110001u32);
    }

    // Target: encoder.encode_fmv_x_f
    #[test]
    fn encode_fmv_x_f_w_vs_s_alias(
        rd in gpr_name(),
        rs1 in fp_name()
    ) {
        let sut = sut_word(&[reg(&rd), reg(&rs1)], 0b1110000)
            .unwrap_or_else(|e| panic!("SUT rejected fmv.x.w {rd}, {rs1}: {e}"));
        let mc_w = llvm_mc_word(&format!("fmv.x.w {}, {}", rd, rs1))
            .unwrap_or_else(|e| panic!("llvm-mc rejected fmv.x.w {rd}, {rs1}: {e}"));
        let mc_s = llvm_mc_word(&format!("fmv.x.s {}, {}", rd, rs1))
            .unwrap_or_else(|e| panic!("llvm-mc rejected fmv.x.s {rd}, {rs1}: {e}"));
        prop_assert_eq!(mc_w, mc_s, "fmv.x.w vs fmv.x.s llvm-mc mismatch");
        prop_assert_eq!(sut, mc_w, "SUT {:08x} != llvm-mc {:08x}", sut, mc_w);
    }

    // Target: encoder.encode_fmv_x_f
    #[test]
    fn encode_fmv_x_f_neg_arity_class(
        (_mn, f7) in fmv_x_mn(),
        fp in fp_name(),
        gpr in gpr_name(),
        bad in bad_reg_operand()
    ) {
        prop_assert!(
            encode_fmv_x_f(&[], f7, FMT_UNUSED).is_err(),
            "empty operand list must Err; got {:?}",
            encode_fmv_x_f(&[], f7, FMT_UNUSED)
        );
        prop_assert!(
            encode_fmv_x_f(&[reg(&gpr)], f7, FMT_UNUSED).is_err(),
            "one operand must Err; got {:?}",
            encode_fmv_x_f(&[reg(&gpr)], f7, FMT_UNUSED)
        );
        let fp_rd = [reg(&fp), reg(&fp)];
        prop_assert!(
            encode_fmv_x_f(&fp_rd, f7, FMT_UNUSED).is_err(),
            "FP rd {} must Err; got {:?}",
            fp,
            encode_fmv_x_f(&fp_rd, f7, FMT_UNUSED)
        );
        let gpr_rs1 = [reg(&gpr), reg(&gpr)];
        prop_assert!(
            encode_fmv_x_f(&gpr_rs1, f7, FMT_UNUSED).is_err(),
            "GPR rs1 {} must Err; got {:?}",
            gpr,
            encode_fmv_x_f(&gpr_rs1, f7, FMT_UNUSED)
        );
        let bad_rd = [bad.clone(), reg(&fp)];
        prop_assert!(
            encode_fmv_x_f(&bad_rd, f7, FMT_UNUSED).is_err(),
            "non-reg rd must Err; got {:?}",
            encode_fmv_x_f(&bad_rd, f7, FMT_UNUSED)
        );
        // get_freg does not accept a bare Imm (unlike get_reg for rd).
        let imm_rs1 = [reg(&gpr), Operand::Imm(0)];
        prop_assert!(
            encode_fmv_x_f(&imm_rs1, f7, FMT_UNUSED).is_err(),
            "Imm rs1 must Err; got {:?}",
            encode_fmv_x_f(&imm_rs1, f7, FMT_UNUSED)
        );
    }

    // Target: encoder.encode_fmv_x_f
    #[test]
    fn encode_fmv_x_f_neg_extra(
        (mn, f7) in fmv_x_mn(),
        rd in gpr_name(),
        rs1 in fp_name(),
        extra in extra_operand()
    ) {
        let asm = format!("{} {}, {}", mn, rd, rs1);
        let ops = vec![reg(&rd), reg(&rs1), extra];
        prop_assert!(
            encode_fmv_x_f(&ops, f7, FMT_UNUSED).is_err(),
            "3rd operand must Err for {} (llvm-mc rejects extra operands); got {:?}",
            asm,
            encode_fmv_x_f(&ops, f7, FMT_UNUSED)
        );
    }

    // Target: encoder.encode_fmv_x_f
    #[test]
    fn encode_fmv_x_f_neg_rm_third(
        (mn, f7) in fmv_x_mn(),
        rd in gpr_name(),
        rs1 in fp_name(),
        rm in rm_name()
    ) {
        let asm = format!("{} {}, {}, {}", mn, rd, rs1, rm);
        let ops = [reg(&rd), reg(&rs1), rm_op(rm)];
        prop_assert!(
            encode_fmv_x_f(&ops, f7, FMT_UNUSED).is_err(),
            "3rd RoundingMode must Err for {} (FMV.X has no rm); got {:?}",
            asm,
            encode_fmv_x_f(&ops, f7, FMT_UNUSED)
        );
    }
}
