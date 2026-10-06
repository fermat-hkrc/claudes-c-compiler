// Oracle: differential — llvm-mc RISC-V assembler (R-type OP-FP FMV.W.X/D word)
// Evidence: src/backend/riscv/assembler/README.md:6-7 textual assembly from codegen;
//   README.md:13-16 RV64GC includes F (single-precision) and D (double-precision);
//   README.md:308 fmv.w.x/d.x;
//   README.md:352 R-type: [funct7 | rs2 | rs1 | funct3 | rd | opcode];
//   encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words";
//   encoder/mod.rs:4 "This covers the subset of instructions emitted by our codegen (RV64GC + Zbb).";
//   encoder/mod.rs:334 "R-type: funct7[31:25] | rs2[24:20] | rs1[19:15] | funct3[14:12] | rd[11:7] | opcode[6:0]";
//   encoder/mod.rs:397 OP_OP_FP = 0b1010011;
//   encoder/mod.rs:762 fmv.w.x | fmv.s.x => encode_fmv_f_x(operands, 0b1111000, 0b00);
//   encoder/mod.rs:792 fmv.d.x => encode_fmv_f_x(operands, 0b1111001, 0b00);
//   RISC-V Unprivileged ISA FMV.W.X/D: opcode=1010011, rd FP, rs1 integer,
//   rs2 hardwired 00000, funct3 hardwired 000, funct7 encodes funct5+fmt
//   (1111000 S / 1111001 D). No rounding-mode field. fmv.s.x aliases fmv.w.x.
// Stronger considered:
//   - State machine: rejected — encode_fmv_f_x is a pure function with no lifecycle
//   - Algebraic round-trip via in-tree decoder: rejected — no OP-FP decoder
//   - encode_r / encode_fclass / encode_fp_unary / encode_fmv_x_f as differential sibling:
//     rejected — same-job gate (private packer / FCLASS uses funct3=001 /
//     FSQRT uses rm / FMV.X.W is the opposite direction)
// Weaker available: algebraic.invariant (R-type field unpack), algebraic.metamorphic
//   (FP ABI vs fN; GPR ABI vs xN; S vs D fmt bit; fmv.w.x vs fmv.s.x alias),
//   negative_error (arity / class / extra / rm-as-third)
// Differential: candidate=encode_fmv_f_x, reference=llvm-mc -triple=riscv64 -mattr=+f,+d -show-encoding,
//   SUT-boundary=internal-helper of RISC-V assembler,
//   mapping=[Reg(rd_fp), Reg(rs1_gpr)] <-> `mn rd, rs1`.

use super::{encode_fmv_f_x, EncodeResult};
use crate::backend::riscv::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;
const OP_OP_FP: u32 = 0b1010011;
const FMV_FX_FUNCT3: u32 = 0b000;
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

/// OP-FP FMV.W.X / FMV.D.X mnemonics that encode through encode_fmv_f_x.
const FMV_FX: [(&str, u32); 3] = [
    ("fmv.w.x", 0b1111000),
    ("fmv.s.x", 0b1111000),
    ("fmv.d.x", 0b1111001),
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
    match encode_fmv_f_x(ops, funct7, FMT_UNUSED)? {
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

fn fmv_fx_mn() -> impl Strategy<Value = (&'static str, u32)> {
    prop::sample::select(FMV_FX.to_vec())
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
    // get_freg does not accept Imm (unlike get_reg for GPR). All of these are
    // non-float-register tokens for rd.
    prop_oneof![
        Just(Operand::Imm(-1)),
        Just(Operand::Imm(0)),
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
fn encode_fmv_f_x_kat_llvm_mc_w_fa0_a1() {
    let want = 0xf005_8553u32;
    let mc = llvm_mc_word("fmv.w.x fa0, a1").expect("llvm-mc KAT fmv.w.x fa0, a1");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("fa0"), reg("a1")], 0b1111000)
        .expect("SUT KAT fmv.w.x fa0, a1");
    assert_eq!(sut, want);
}

#[test]
fn encode_fmv_f_x_kat_llvm_mc_variants() {
    let want_s_alias = 0xf005_8553u32;
    assert_eq!(llvm_mc_word("fmv.s.x fa0, a1").unwrap(), want_s_alias);
    assert_eq!(
        sut_word(&[reg("fa0"), reg("a1")], 0b1111000).unwrap(),
        want_s_alias
    );

    let want_t0 = 0xf002_8053u32;
    assert_eq!(llvm_mc_word("fmv.w.x ft0, t0").unwrap(), want_t0);
    assert_eq!(
        sut_word(&[reg("ft0"), reg("t0")], 0b1111000).unwrap(),
        want_t0
    );

    let want_zero = 0xf000_0053u32;
    assert_eq!(llvm_mc_word("fmv.w.x ft0, zero").unwrap(), want_zero);
    assert_eq!(
        sut_word(&[reg("ft0"), reg("zero")], 0b1111000).unwrap(),
        want_zero
    );

    let want_d = 0xf205_8553u32;
    assert_eq!(llvm_mc_word("fmv.d.x fa0, a1").unwrap(), want_d);
    assert_eq!(
        sut_word(&[reg("fa0"), reg("a1")], 0b1111001).unwrap(),
        want_d
    );

    let want_d_t6 = 0xf20f_8fd3u32;
    assert_eq!(llvm_mc_word("fmv.d.x ft11, t6").unwrap(), want_d_t6);
    assert_eq!(
        sut_word(&[reg("ft11"), reg("t6")], 0b1111001).unwrap(),
        want_d_t6
    );

    let want_d_x0 = 0xf200_0053u32;
    assert_eq!(llvm_mc_word("fmv.d.x f0, x0").unwrap(), want_d_x0);
    assert_eq!(
        sut_word(&[reg("f0"), reg("x0")], 0b1111001).unwrap(),
        want_d_x0
    );

    let want_f31 = 0xf000_0fd3u32;
    assert_eq!(llvm_mc_word("fmv.w.x f31, x0").unwrap(), want_f31);
    assert_eq!(
        sut_word(&[reg("f31"), reg("x0")], 0b1111000).unwrap(),
        want_f31
    );

    let want_fp = 0xf004_0053u32;
    assert_eq!(llvm_mc_word("fmv.w.x ft0, fp").unwrap(), want_fp);
    assert_eq!(
        sut_word(&[reg("ft0"), reg("fp")], 0b1111000).unwrap(),
        want_fp
    );

    let want_s0 = 0xf004_0453u32;
    assert_eq!(llvm_mc_word("fmv.w.x f8, s0").unwrap(), want_s0);
    assert_eq!(
        sut_word(&[reg("f8"), reg("s0")], 0b1111000).unwrap(),
        want_s0
    );
}

/// Regression: extra 3rd operand must be rejected (llvm-mc errors).
#[test]
fn test_encode_fmv_f_x_regression_extra_operand() {
    let ops = [reg("fa0"), reg("a0"), Operand::Imm(0)];
    assert!(
        encode_fmv_f_x(&ops, 0b1111000, FMT_UNUSED).is_err(),
        "fmv.w.x fa0, a0, 0 must Err; got {:?}",
        encode_fmv_f_x(&ops, 0b1111000, FMT_UNUSED)
    );
}

/// Regression: 3rd RoundingMode must be rejected (FMV.W.X has no rm field).
#[test]
fn test_encode_fmv_f_x_regression_rm_third() {
    let ops = [reg("fa0"), reg("a0"), rm_op("rne")];
    assert!(
        encode_fmv_f_x(&ops, 0b1111000, FMT_UNUSED).is_err(),
        "fmv.w.x fa0, a0, rne must Err; got {:?}",
        encode_fmv_f_x(&ops, 0b1111000, FMT_UNUSED)
    );
}

proptest! {
    #![proptest_config(cfg())]

    // Target: encoder.encode_fmv_f_x
    #[test]
    fn encode_fmv_f_x_diff_2op_llvm_mc(
        (mn, f7) in fmv_fx_mn(),
        rd in fp_name(),
        rs1 in gpr_name()
    ) {
        let asm = format!("{} {}, {}", mn, rd, rs1);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&[reg(&rd), reg(&rs1)], f7)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "SUT {:08x} != llvm-mc {:08x} for {}", sut, mc, asm);
    }

    // Target: encoder.encode_fmv_f_x
    #[test]
    fn encode_fmv_f_x_r_type_fields(
        (_mn, f7) in fmv_fx_mn(),
        rd in 0u32..=31u32,
        rs1 in 0u32..=31u32
    ) {
        let w = sut_word(&[reg(&fn_name(rd)), reg(&xn(rs1))], f7)
            .unwrap_or_else(|e| panic!("SUT rejected f{rd}, x{rs1}: {e}"));
        let (opc, got_f3, got_rd, got_rs1, got_rs2, got_f7) = unpack_r(w);
        prop_assert_eq!(opc, OP_OP_FP, "opcode");
        prop_assert_eq!(got_f3, FMV_FX_FUNCT3, "funct3");
        prop_assert_eq!(got_rd, rd, "rd");
        prop_assert_eq!(got_rs1, rs1, "rs1");
        prop_assert_eq!(got_rs2, 0u32, "rs2");
        prop_assert_eq!(got_f7, f7, "funct7");
    }

    // Target: encoder.encode_fmv_f_x
    #[test]
    fn encode_fmv_f_x_abi_fn_xn_alias(
        (_mn, f7) in fmv_fx_mn(),
        n in 0u32..=31u32,
        m in 0u32..=31u32
    ) {
        let via_n = sut_word(&[reg(&fn_name(n)), reg(&xn(m))], f7)
            .unwrap_or_else(|e| panic!("fN/xN rejected: {e}"));
        let via_abi = sut_word(&[reg(fabi(n)), reg(gabi(m))], f7)
            .unwrap_or_else(|e| panic!("ABI rejected: {e}"));
        prop_assert_eq!(via_abi, via_n);
    }

    // Target: encoder.encode_fmv_f_x
    #[test]
    fn encode_fmv_f_x_s_vs_d_fmt(
        rd in 0u32..=31u32,
        rs1 in 0u32..=31u32
    ) {
        let ops = [reg(&fn_name(rd)), reg(&xn(rs1))];
        let ws = sut_word(&ops, 0b1111000)
            .unwrap_or_else(|e| panic!("SUT rejected fmv.w.x f{rd}, x{rs1}: {e}"));
        let wd = sut_word(&ops, 0b1111001)
            .unwrap_or_else(|e| panic!("SUT rejected fmv.d.x f{rd}, x{rs1}: {e}"));
        prop_assert_eq!(ws ^ wd, 1u32 << 25, "S vs D must differ only in funct7 bit 0");
        let (_, _, _, _, _, f7s) = unpack_r(ws);
        let (_, _, _, _, _, f7d) = unpack_r(wd);
        prop_assert_eq!(f7s, 0b1111000u32);
        prop_assert_eq!(f7d, 0b1111001u32);
    }

    // Target: encoder.encode_fmv_f_x
    #[test]
    fn encode_fmv_f_x_w_vs_s_alias(
        rd in fp_name(),
        rs1 in gpr_name()
    ) {
        let sut = sut_word(&[reg(&rd), reg(&rs1)], 0b1111000)
            .unwrap_or_else(|e| panic!("SUT rejected fmv.w.x {rd}, {rs1}: {e}"));
        let mc_w = llvm_mc_word(&format!("fmv.w.x {}, {}", rd, rs1))
            .unwrap_or_else(|e| panic!("llvm-mc rejected fmv.w.x {rd}, {rs1}: {e}"));
        let mc_s = llvm_mc_word(&format!("fmv.s.x {}, {}", rd, rs1))
            .unwrap_or_else(|e| panic!("llvm-mc rejected fmv.s.x {rd}, {rs1}: {e}"));
        prop_assert_eq!(mc_w, mc_s, "fmv.w.x vs fmv.s.x llvm-mc mismatch");
        prop_assert_eq!(sut, mc_w, "SUT {:08x} != llvm-mc {:08x}", sut, mc_w);
    }

    // Target: encoder.encode_fmv_f_x
    #[test]
    fn encode_fmv_f_x_neg_arity_class(
        (_mn, f7) in fmv_fx_mn(),
        fp in fp_name(),
        gpr in gpr_name(),
        bad in bad_reg_operand()
    ) {
        prop_assert!(
            encode_fmv_f_x(&[], f7, FMT_UNUSED).is_err(),
            "empty operand list must Err; got {:?}",
            encode_fmv_f_x(&[], f7, FMT_UNUSED)
        );
        prop_assert!(
            encode_fmv_f_x(&[reg(&fp)], f7, FMT_UNUSED).is_err(),
            "one operand must Err; got {:?}",
            encode_fmv_f_x(&[reg(&fp)], f7, FMT_UNUSED)
        );
        let gpr_rd = [reg(&gpr), reg(&gpr)];
        prop_assert!(
            encode_fmv_f_x(&gpr_rd, f7, FMT_UNUSED).is_err(),
            "GPR rd {} must Err; got {:?}",
            gpr,
            encode_fmv_f_x(&gpr_rd, f7, FMT_UNUSED)
        );
        let fp_rs1 = [reg(&fp), reg(&fp)];
        prop_assert!(
            encode_fmv_f_x(&fp_rs1, f7, FMT_UNUSED).is_err(),
            "FP rs1 {} must Err; got {:?}",
            fp,
            encode_fmv_f_x(&fp_rs1, f7, FMT_UNUSED)
        );
        let bad_rd = [bad.clone(), reg(&gpr)];
        prop_assert!(
            encode_fmv_f_x(&bad_rd, f7, FMT_UNUSED).is_err(),
            "non-fp rd must Err; got {:?}",
            encode_fmv_f_x(&bad_rd, f7, FMT_UNUSED)
        );
        // get_freg does not accept a bare Imm for rd (unlike get_reg for GPR).
        let imm_rd = [Operand::Imm(0), reg(&gpr)];
        prop_assert!(
            encode_fmv_f_x(&imm_rd, f7, FMT_UNUSED).is_err(),
            "Imm rd must Err; got {:?}",
            encode_fmv_f_x(&imm_rd, f7, FMT_UNUSED)
        );
    }

    // Target: encoder.encode_fmv_f_x
    #[test]
    fn encode_fmv_f_x_neg_extra(
        (mn, f7) in fmv_fx_mn(),
        rd in fp_name(),
        rs1 in gpr_name(),
        extra in extra_operand()
    ) {
        let asm = format!("{} {}, {}", mn, rd, rs1);
        let ops = vec![reg(&rd), reg(&rs1), extra];
        prop_assert!(
            encode_fmv_f_x(&ops, f7, FMT_UNUSED).is_err(),
            "3rd operand must Err for {} (llvm-mc rejects extra operands); got {:?}",
            asm,
            encode_fmv_f_x(&ops, f7, FMT_UNUSED)
        );
    }

    // Target: encoder.encode_fmv_f_x
    #[test]
    fn encode_fmv_f_x_neg_rm_third(
        (mn, f7) in fmv_fx_mn(),
        rd in fp_name(),
        rs1 in gpr_name(),
        rm in rm_name()
    ) {
        let asm = format!("{} {}, {}, {}", mn, rd, rs1, rm);
        let ops = [reg(&rd), reg(&rs1), rm_op(rm)];
        prop_assert!(
            encode_fmv_f_x(&ops, f7, FMT_UNUSED).is_err(),
            "3rd RoundingMode must Err for {} (FMV.W.X has no rm); got {:?}",
            asm,
            encode_fmv_f_x(&ops, f7, FMT_UNUSED)
        );
    }
}
