// Oracle: differential — llvm-mc RISC-V assembler (R-type OP-FP FCVT.fp word)
// Evidence: src/backend/riscv/assembler/README.md:6-7 textual assembly from codegen;
//   README.md:13-16 RV64GC includes F (single-precision) and D (double-precision);
//   README.md:308 fcvt (all int/float conversions);
//   README.md:352 R-type: [funct7 | rs2 | rs1 | funct3 | rd | opcode];
//   encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words";
//   encoder/mod.rs:4 "This covers the subset of instructions emitted by our codegen (RV64GC + Zbb).";
//   encoder/mod.rs:331 "R-type: funct7[31:25] | rs2[24:20] | rs1[19:15] | funct3[14:12] | rd[11:7] | opcode[6:0]";
//   encoder/mod.rs:389 OP_OP_FP = 0b1010011;
//   encoder/mod.rs:485 "Parse a rounding mode to 3-bit encoding";
//   encoder/mod.rs:785 "fcvt.s.d" => encode_fcvt_fp(operands, 0b0100000, 0b00001);
//   encoder/mod.rs:786 "fcvt.d.s" => encode_fcvt_fp(operands, 0b0100001, 0b00000);
//   parser.rs:41 "Rounding mode: rne, rtz, rdn, rup, rmm, dyn";
//   float.rs:147 "Float to float conversion (e.g., fcvt.s.d, fcvt.d.s)";
//   RISC-V Unprivileged ISA FCVT.S.D / FCVT.D.S: opcode=1010011, rm in bits 14:12,
//   rd/rs1 FP, rs2 encodes source format (00001 D / 00000 S), funct7 0100000 (S) / 0100001 (D).
// Stronger considered:
//   - State machine: rejected — encode_fcvt_fp is a pure function with no lifecycle
//   - Algebraic round-trip via in-tree decoder: rejected — no OP-FP decoder
//   - encode_r / encode_fp_unary / encode_fcvt_int as differential sibling: rejected —
//     same-job gate (private packer / FSQRT unary / float-to-int)
// Weaker available: algebraic.invariant (R-type field unpack), algebraic.metamorphic
//   (FP ABI vs fN; 2-op == explicit dyn), negative_error (arity / GPR / extra / non-rm 3rd)
// Differential: candidate=encode_fcvt_fp, reference=llvm-mc -triple=riscv64 -mattr=+f,+d -show-encoding,
//   SUT-boundary=internal-helper of RISC-V assembler,
//   mapping=[Reg(rd FP), Reg(rs1 FP)] <-> `mn rd, rs1`;
//   optional RoundingMode <-> `, rm`.

use super::{encode_fcvt_fp, EncodeResult};
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

/// OP-FP float-to-float mnemonics that encode through encode_fcvt_fp.
const FCVT_FP: [(&str, u32, u32); 2] = [
    ("fcvt.s.d", 0b0100000, 0b00001),
    ("fcvt.d.s", 0b0100001, 0b00000),
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

fn sut_word(ops: &[Operand], funct7: u32, rs2: u32) -> Result<u32, String> {
    match encode_fcvt_fp(ops, funct7, rs2)? {
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

fn fcvt_fp_mn() -> impl Strategy<Value = (&'static str, u32, u32)> {
    prop::sample::select(FCVT_FP.to_vec())
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
fn encode_fcvt_fp_kat_llvm_mc_fcvt_s_d_fa0_fa1() {
    let want = 0x4015_f553u32;
    let mc = llvm_mc_word("fcvt.s.d fa0, fa1").expect("llvm-mc KAT fcvt.s.d fa0, fa1");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("fa0"), reg("fa1")], 0b0100000, 0b00001)
        .expect("SUT KAT fcvt.s.d fa0, fa1");
    assert_eq!(sut, want);
}

#[test]
fn encode_fcvt_fp_kat_llvm_mc_fcvt_s_d_ft0_ft1() {
    let want = 0x4010_f053u32;
    let mc = llvm_mc_word("fcvt.s.d ft0, ft1").expect("llvm-mc KAT fcvt.s.d ft0, ft1");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("ft0"), reg("ft1")], 0b0100000, 0b00001).expect("SUT KAT fcvt.s.d");
    assert_eq!(sut, want);
}

#[test]
fn encode_fcvt_fp_kat_llvm_mc_fcvt_s_d_fs0_fs1() {
    let want = 0x4014_f453u32;
    let mc = llvm_mc_word("fcvt.s.d fs0, fs1").expect("llvm-mc KAT fcvt.s.d fs0, fs1");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("fs0"), reg("fs1")], 0b0100000, 0b00001).expect("SUT KAT fcvt.s.d fs0");
    assert_eq!(sut, want);
}

#[test]
fn encode_fcvt_fp_kat_llvm_mc_fcvt_s_d_ft11_ft0() {
    let want = 0x4010_7fd3u32;
    let mc = llvm_mc_word("fcvt.s.d ft11, ft0").expect("llvm-mc KAT fcvt.s.d ft11, ft0");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("ft11"), reg("ft0")], 0b0100000, 0b00001).expect("SUT KAT fcvt.s.d ft11");
    assert_eq!(sut, want);
}

#[test]
fn encode_fcvt_fp_kat_llvm_mc_rm_and_d() {
    let want_rne = 0x4015_8553u32;
    assert_eq!(llvm_mc_word("fcvt.s.d fa0, fa1, rne").unwrap(), want_rne);
    assert_eq!(
        sut_word(&[reg("fa0"), reg("fa1"), rm_op("rne")], 0b0100000, 0b00001).unwrap(),
        want_rne
    );

    let want_rtz = 0x4015_9553u32;
    assert_eq!(llvm_mc_word("fcvt.s.d fa0, fa1, rtz").unwrap(), want_rtz);
    assert_eq!(
        sut_word(&[reg("fa0"), reg("fa1"), rm_op("rtz")], 0b0100000, 0b00001).unwrap(),
        want_rtz
    );

    let want_f0 = 0x4010_f053u32;
    assert_eq!(llvm_mc_word("fcvt.s.d f0, f1").unwrap(), want_f0);
    assert_eq!(
        sut_word(&[reg("f0"), reg("f1")], 0b0100000, 0b00001).unwrap(),
        want_f0
    );

    // llvm-mc hardwires rm=RNE for FCVT.D.S; pin the reference encoding only.
    let want_d_s = 0x4205_8553u32;
    assert_eq!(llvm_mc_word("fcvt.d.s fa0, fa1").unwrap(), want_d_s);
    let want_d_s_f0 = 0x4200_8053u32;
    assert_eq!(llvm_mc_word("fcvt.d.s f0, f1").unwrap(), want_d_s_f0);
}

/// Regression: extra 4th operand must be rejected (llvm-mc errors).
#[test]
fn test_encode_fcvt_fp_regression_extra_operand() {
    let ops = [
        reg("f0"),
        reg("f0"),
        rm_op("rne"),
        Operand::Imm(0),
    ];
    assert!(
        encode_fcvt_fp(&ops, 0b0100000, 0b00001).is_err(),
        "fcvt.s.d f0, f0, rne with a 4th operand must Err; got {:?}",
        encode_fcvt_fp(&ops, 0b0100000, 0b00001)
    );
}

/// Regression: 3rd non-RoundingMode operand must be rejected.
#[test]
fn test_encode_fcvt_fp_regression_non_rm_third() {
    let ops = [reg("f0"), reg("f0"), Operand::Imm(0)];
    assert!(
        encode_fcvt_fp(&ops, 0b0100000, 0b00001).is_err(),
        "fcvt.s.d f0, f0, 0 must Err (3rd operand is not a rounding mode); got {:?}",
        encode_fcvt_fp(&ops, 0b0100000, 0b00001)
    );
}

/// Regression: FCVT.D.S 2-op must match llvm-mc (rm=RNE, not DYN).
#[test]
fn test_encode_fcvt_fp_regression_fcvt_d_s_default_rm() {
    let mc = llvm_mc_word("fcvt.d.s fa0, fa1").expect("llvm-mc fcvt.d.s fa0, fa1");
    let sut = sut_word(&[reg("fa0"), reg("fa1")], 0b0100001, 0b00000)
        .expect("SUT fcvt.d.s fa0, fa1");
    assert_eq!(
        sut, mc,
        "fcvt.d.s fa0, fa1 must match llvm-mc (RNE); SUT {:08x} llvm-mc {:08x}",
        sut, mc
    );
}

proptest! {
    #![proptest_config(cfg())]

    // Target: encoder.encode_fcvt_fp
    #[test]
    fn encode_fcvt_fp_diff_2op_llvm_mc(
        (mn, f7, rs2) in fcvt_fp_mn(),
        rd in fp_name(),
        rs1 in fp_name()
    ) {
        let asm = format!("{} {}, {}", mn, rd, rs1);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&[reg(&rd), reg(&rs1)], f7, rs2)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "SUT {:08x} != llvm-mc {:08x} for {}", sut, mc, asm);
    }

    // Target: encoder.encode_fcvt_fp
    #[test]
    fn encode_fcvt_fp_diff_rm_llvm_mc(
        rd in fp_name(),
        rs1 in fp_name(),
        (rm, _) in rm_pair()
    ) {
        let asm = format!("fcvt.s.d {}, {}, {}", rd, rs1, rm);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&[reg(&rd), reg(&rs1), rm_op(rm)], 0b0100000, 0b00001)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "SUT {:08x} != llvm-mc {:08x} for {}", sut, mc, asm);
    }

    // Target: encoder.encode_fcvt_fp
    #[test]
    fn encode_fcvt_fp_r_type_fields(
        (_mn, f7, rs2) in fcvt_fp_mn(),
        rd in 0u32..=31u32,
        rs1 in 0u32..=31u32,
        (rm_name, rm) in rm_pair()
    ) {
        let w = sut_word(
            &[reg(&fn_name(rd)), reg(&fn_name(rs1)), rm_op(rm_name)],
            f7,
            rs2,
        )
        .unwrap_or_else(|e| panic!("SUT rejected f{rd}, f{rs1}, {rm_name}: {e}"));
        let (opc, got_rm, got_rd, got_rs1, got_rs2, got_f7) = unpack_r(w);
        prop_assert_eq!(opc, OP_OP_FP, "opcode");
        prop_assert_eq!(got_rm, rm, "rm/funct3");
        prop_assert_eq!(got_rd, rd, "rd");
        prop_assert_eq!(got_rs1, rs1, "rs1");
        prop_assert_eq!(got_rs2, rs2, "rs2");
        prop_assert_eq!(got_f7, f7, "funct7");
    }

    // Target: encoder.encode_fcvt_fp
    #[test]
    fn encode_fcvt_fp_abi_fn_alias(
        (_mn, f7, rs2) in fcvt_fp_mn(),
        n in 0u32..=31u32,
        m in 0u32..=31u32
    ) {
        let via_n = sut_word(&[reg(&fn_name(n)), reg(&fn_name(m))], f7, rs2)
            .unwrap_or_else(|e| panic!("fN rejected: {e}"));
        let via_abi = sut_word(&[reg(fabi(n)), reg(fabi(m))], f7, rs2)
            .unwrap_or_else(|e| panic!("FABI rejected: {e}"));
        prop_assert_eq!(via_abi, via_n);
    }

    // Target: encoder.encode_fcvt_fp
    #[test]
    fn encode_fcvt_fp_rm_default_dyn(
        (_mn, f7, rs2) in fcvt_fp_mn(),
        rd in fp_name(),
        rs1 in fp_name()
    ) {
        let two = sut_word(&[reg(&rd), reg(&rs1)], f7, rs2)
            .unwrap_or_else(|e| panic!("2-op rejected: {e}"));
        let dyn3 = sut_word(&[reg(&rd), reg(&rs1), rm_op("dyn")], f7, rs2)
            .unwrap_or_else(|e| panic!("dyn rejected: {e}"));
        prop_assert_eq!(two, dyn3);
        let (opc, rm, _, _, got_rs2, _) = unpack_r(two);
        prop_assert_eq!(opc, OP_OP_FP);
        prop_assert_eq!(rm, 0b111u32, "omitted rm must be DYN");
        prop_assert_eq!(got_rs2, rs2);
    }

    // Target: encoder.encode_fcvt_fp
    #[test]
    fn encode_fcvt_fp_neg_arity_gpr(
        (_mn, f7, rs2) in fcvt_fp_mn(),
        fp in fp_name(),
        gpr in gpr_name(),
        bad in bad_reg_operand()
    ) {
        prop_assert!(
            encode_fcvt_fp(&[], f7, rs2).is_err(),
            "empty operand list must Err; got {:?}",
            encode_fcvt_fp(&[], f7, rs2)
        );
        prop_assert!(
            encode_fcvt_fp(&[reg(&fp)], f7, rs2).is_err(),
            "one operand must Err; got {:?}",
            encode_fcvt_fp(&[reg(&fp)], f7, rs2)
        );
        let gpr_rd = [reg(&gpr), reg(&fp)];
        prop_assert!(
            encode_fcvt_fp(&gpr_rd, f7, rs2).is_err(),
            "GPR rd {} must Err; got {:?}",
            gpr,
            encode_fcvt_fp(&gpr_rd, f7, rs2)
        );
        let gpr_rs1 = [reg(&fp), reg(&gpr)];
        prop_assert!(
            encode_fcvt_fp(&gpr_rs1, f7, rs2).is_err(),
            "GPR rs1 {} must Err; got {:?}",
            gpr,
            encode_fcvt_fp(&gpr_rs1, f7, rs2)
        );
        let bad_rd = [bad.clone(), reg(&fp)];
        prop_assert!(
            encode_fcvt_fp(&bad_rd, f7, rs2).is_err(),
            "non-reg rd must Err; got {:?}",
            encode_fcvt_fp(&bad_rd, f7, rs2)
        );
    }

    // Target: encoder.encode_fcvt_fp
    #[test]
    fn encode_fcvt_fp_neg_extra(
        (mn, f7, rs2) in fcvt_fp_mn(),
        rd in fp_name(),
        rs1 in fp_name(),
        extra in extra_operand()
    ) {
        let asm = format!("{} {}, {}, rne", mn, rd, rs1);
        let ops = vec![reg(&rd), reg(&rs1), rm_op("rne"), extra];
        prop_assert!(
            encode_fcvt_fp(&ops, f7, rs2).is_err(),
            "4th operand must Err for {} (llvm-mc rejects extra operands); got {:?}",
            asm,
            encode_fcvt_fp(&ops, f7, rs2)
        );
    }

    // Target: encoder.encode_fcvt_fp
    #[test]
    fn encode_fcvt_fp_neg_non_rm_third(
        (mn, f7, rs2) in fcvt_fp_mn(),
        rd in fp_name(),
        rs1 in fp_name(),
        extra in non_rm_operand()
    ) {
        let asm = format!("{} {}, {}", mn, rd, rs1);
        let ops = [reg(&rd), reg(&rs1), extra];
        prop_assert!(
            encode_fcvt_fp(&ops, f7, rs2).is_err(),
            "3rd non-RoundingMode operand must Err for {} (optional rm only); got {:?}",
            asm,
            encode_fcvt_fp(&ops, f7, rs2)
        );
    }
}
