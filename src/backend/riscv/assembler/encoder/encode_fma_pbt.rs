// Oracle: differential — llvm-mc RISC-V assembler (R4-type FMA word)
// Evidence: src/backend/riscv/assembler/README.md:6-7 textual assembly from codegen;
//   README.md:13-16 RV64GC includes F (single-precision) and D (double-precision);
//   README.md:309 fmadd/fmsub/fnmadd/fnmsub;
//   encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words";
//   encoder/mod.rs:4 "This covers the subset of instructions emitted by our codegen (RV64GC + Zbb).";
//   encoder/mod.rs:400-403 OP_FMADD/OP_FMSUB/OP_FNMSUB/OP_FNMADD;
//   encoder/mod.rs:492 "Parse a rounding mode to 3-bit encoding";
//   encoder/mod.rs:797-804 fmadd/fmsub/fnmsub/fnmadd .s/.d => encode_fma;
//   float.rs:189 "R4-type: rs3[31:27] | fmt[26:25] | rs2[24:20] | rs1[19:15] | rm[14:12] | rd[11:7] | opcode[6:0]";
//   parser.rs:41 "Rounding mode: rne, rtz, rdn, rup, rmm, dyn";
//   RISC-V Unprivileged ISA R4-type FMA: opcode in {1000011,1000111,1001011,1001111},
//   fmt in bits 26:25 (00 S / 01 D), rs3 in bits 31:27, rm in bits 14:12, all four
//   registers FP. Omitted rm is DYN (111).
// Stronger considered:
//   - State machine: rejected — encode_fma is a pure function with no lifecycle
//   - Algebraic round-trip via in-tree decoder: rejected — no R4-type decoder
//   - encode_r / encode_fp_arith as differential sibling: rejected —
//     same-job gate (R-type packer / 3-operand OP-FP, not R4 FMA)
// Weaker available: algebraic.invariant (R4 field unpack), algebraic.metamorphic
//   (FP ABI vs fN; 4-op == explicit dyn), negative_error (arity / GPR / extra / non-rm 5th)
// Differential: candidate=encode_fma, reference=llvm-mc -triple=riscv64 -mattr=+f,+d -show-encoding,
//   SUT-boundary=internal-helper of RISC-V assembler,
//   mapping=[Reg(rd), Reg(rs1), Reg(rs2), Reg(rs3)] <-> `mn rd, rs1, rs2, rs3`;
//   optional RoundingMode <-> `, rm`.

use super::{encode_fma, EncodeResult};
use crate::backend::riscv::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;

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

/// FMA mnemonics that encode through encode_fma, with opcode and fmt.
const FMA: [(&str, u32, u32); 8] = [
    ("fmadd.s", 0b1000011, 0b00),
    ("fmsub.s", 0b1000111, 0b00),
    ("fnmsub.s", 0b1001011, 0b00),
    ("fnmadd.s", 0b1001111, 0b00),
    ("fmadd.d", 0b1000011, 0b01),
    ("fmsub.d", 0b1000111, 0b01),
    ("fnmsub.d", 0b1001011, 0b01),
    ("fnmadd.d", 0b1001111, 0b01),
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

/// Unpack R4-type per RISC-V unprivileged ISA (not a copy of encode_fma).
/// Layout: rs3[31:27] | fmt[26:25] | rs2[24:20] | rs1[19:15] | rm[14:12] | rd[11:7] | opcode[6:0]
fn unpack_r4(word: u32) -> (u32, u32, u32, u32, u32, u32, u32) {
    let opcode = word & 0x7f;
    let rd = (word >> 7) & 0x1f;
    let rm = (word >> 12) & 0x7;
    let rs1 = (word >> 15) & 0x1f;
    let rs2 = (word >> 20) & 0x1f;
    let fmt = (word >> 25) & 0x3;
    let rs3 = (word >> 27) & 0x1f;
    (opcode, rd, rm, rs1, rs2, fmt, rs3)
}

fn sut_word(ops: &[Operand], opcode: u32, fmt: u32) -> Result<u32, String> {
    match encode_fma(ops, opcode, fmt)? {
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

fn fma_mn() -> impl Strategy<Value = (&'static str, u32, u32)> {
    prop::sample::select(FMA.to_vec())
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
fn encode_fma_kat_llvm_mc_fmadd_s_fa0_fa1_fa2_fa3() {
    let want = 0x68c5_f543u32;
    let mc = llvm_mc_word("fmadd.s fa0, fa1, fa2, fa3").expect("llvm-mc KAT fmadd.s fa0, fa1, fa2, fa3");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("fa0"), reg("fa1"), reg("fa2"), reg("fa3")], 0b1000011, 0b00)
        .expect("SUT KAT fmadd.s fa0, fa1, fa2, fa3");
    assert_eq!(sut, want);
}

#[test]
fn encode_fma_kat_llvm_mc_variants() {
    let want_ft = 0x1820_f043u32;
    assert_eq!(llvm_mc_word("fmadd.s ft0, ft1, ft2, ft3").unwrap(), want_ft);
    assert_eq!(
        sut_word(&[reg("ft0"), reg("ft1"), reg("ft2"), reg("ft3")], 0b1000011, 0b00).unwrap(),
        want_ft
    );

    let want_fmsub = 0x9924_f447u32;
    assert_eq!(llvm_mc_word("fmsub.s fs0, fs1, fs2, fs3").unwrap(), want_fmsub);
    assert_eq!(
        sut_word(&[reg("fs0"), reg("fs1"), reg("fs2"), reg("fs3")], 0b1000111, 0b00).unwrap(),
        want_fmsub
    );

    let want_fnmsub = 0x68c5_f54bu32;
    assert_eq!(llvm_mc_word("fnmsub.s fa0, fa1, fa2, fa3").unwrap(), want_fnmsub);
    assert_eq!(
        sut_word(&[reg("fa0"), reg("fa1"), reg("fa2"), reg("fa3")], 0b1001011, 0b00).unwrap(),
        want_fnmsub
    );

    let want_fnmadd = 0x68c5_f54fu32;
    assert_eq!(llvm_mc_word("fnmadd.s fa0, fa1, fa2, fa3").unwrap(), want_fnmadd);
    assert_eq!(
        sut_word(&[reg("fa0"), reg("fa1"), reg("fa2"), reg("fa3")], 0b1001111, 0b00).unwrap(),
        want_fnmadd
    );

    let want_d = 0x6ac5_f543u32;
    assert_eq!(llvm_mc_word("fmadd.d fa0, fa1, fa2, fa3").unwrap(), want_d);
    assert_eq!(
        sut_word(&[reg("fa0"), reg("fa1"), reg("fa2"), reg("fa3")], 0b1000011, 0b01).unwrap(),
        want_d
    );

    let want_rne = 0x68c5_8543u32;
    assert_eq!(llvm_mc_word("fmadd.s fa0, fa1, fa2, fa3, rne").unwrap(), want_rne);
    assert_eq!(
        sut_word(
            &[reg("fa0"), reg("fa1"), reg("fa2"), reg("fa3"), rm_op("rne")],
            0b1000011,
            0b00,
        )
        .unwrap(),
        want_rne
    );

    let want_rtz = 0x68c5_9543u32;
    assert_eq!(llvm_mc_word("fmadd.s fa0, fa1, fa2, fa3, rtz").unwrap(), want_rtz);
    assert_eq!(
        sut_word(
            &[reg("fa0"), reg("fa1"), reg("fa2"), reg("fa3"), rm_op("rtz")],
            0b1000011,
            0b00,
        )
        .unwrap(),
        want_rtz
    );

    let want_f0 = 0x1820_f043u32;
    assert_eq!(llvm_mc_word("fmadd.s f0, f1, f2, f3").unwrap(), want_f0);
    assert_eq!(
        sut_word(&[reg("f0"), reg("f1"), reg("f2"), reg("f3")], 0b1000011, 0b00).unwrap(),
        want_f0
    );

    let want_same = 0x0000_7043u32;
    assert_eq!(llvm_mc_word("fmadd.s f0, f0, f0, f0").unwrap(), want_same);
    assert_eq!(
        sut_word(&[reg("f0"), reg("f0"), reg("f0"), reg("f0")], 0b1000011, 0b00).unwrap(),
        want_same
    );

    let want_max = 0xf9ff_ffc3u32;
    assert_eq!(llvm_mc_word("fmadd.s f31, f31, f31, f31").unwrap(), want_max);
    assert_eq!(
        sut_word(&[reg("f31"), reg("f31"), reg("f31"), reg("f31")], 0b1000011, 0b00).unwrap(),
        want_max
    );
}

/// Regression: extra 6th operand must be rejected (llvm-mc errors).
#[test]
fn test_encode_fma_regression_extra_operand() {
    let ops = [
        reg("f0"),
        reg("f0"),
        reg("f0"),
        reg("f0"),
        rm_op("rne"),
        Operand::Imm(0),
    ];
    assert!(
        encode_fma(&ops, 0b1000011, 0b00).is_err(),
        "fmadd.s f0, f0, f0, f0, rne with a 6th operand must Err; got {:?}",
        encode_fma(&ops, 0b1000011, 0b00)
    );
}

/// Regression: 5th non-RoundingMode operand must be rejected.
#[test]
fn test_encode_fma_regression_non_rm_fifth() {
    let ops = [reg("f0"), reg("f0"), reg("f0"), reg("f0"), Operand::Imm(0)];
    assert!(
        encode_fma(&ops, 0b1000011, 0b00).is_err(),
        "fmadd.s f0, f0, f0, f0, 0 must Err (5th operand is not a rounding mode); got {:?}",
        encode_fma(&ops, 0b1000011, 0b00)
    );
}

proptest! {
    #![proptest_config(cfg())]

    // Target: encoder.encode_fma
    #[test]
    fn encode_fma_diff_4op_llvm_mc(
        (mn, opc, fmt) in fma_mn(),
        rd in fp_name(),
        rs1 in fp_name(),
        rs2 in fp_name(),
        rs3 in fp_name()
    ) {
        let asm = format!("{} {}, {}, {}, {}", mn, rd, rs1, rs2, rs3);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&[reg(&rd), reg(&rs1), reg(&rs2), reg(&rs3)], opc, fmt)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "SUT {:08x} != llvm-mc {:08x} for {}", sut, mc, asm);
    }

    // Target: encoder.encode_fma
    #[test]
    fn encode_fma_diff_rm_llvm_mc(
        (mn, opc, fmt) in fma_mn(),
        rd in fp_name(),
        rs1 in fp_name(),
        rs2 in fp_name(),
        rs3 in fp_name(),
        (rm, _) in rm_pair()
    ) {
        let asm = format!("{} {}, {}, {}, {}, {}", mn, rd, rs1, rs2, rs3, rm);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&[reg(&rd), reg(&rs1), reg(&rs2), reg(&rs3), rm_op(rm)], opc, fmt)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "SUT {:08x} != llvm-mc {:08x} for {}", sut, mc, asm);
    }

    // Target: encoder.encode_fma
    #[test]
    fn encode_fma_r4_type_fields(
        (_mn, opc, fmt) in fma_mn(),
        rd in 0u32..=31u32,
        rs1 in 0u32..=31u32,
        rs2 in 0u32..=31u32,
        rs3 in 0u32..=31u32,
        (rm_name, rm) in rm_pair()
    ) {
        let w = sut_word(
            &[
                reg(&fn_name(rd)),
                reg(&fn_name(rs1)),
                reg(&fn_name(rs2)),
                reg(&fn_name(rs3)),
                rm_op(rm_name),
            ],
            opc,
            fmt,
        )
        .unwrap_or_else(|e| panic!("SUT rejected f{rd}, f{rs1}, f{rs2}, f{rs3}, {rm_name}: {e}"));
        let (got_opc, got_rd, got_rm, got_rs1, got_rs2, got_fmt, got_rs3) = unpack_r4(w);
        prop_assert_eq!(got_opc, opc, "opcode");
        prop_assert_eq!(got_rd, rd, "rd");
        prop_assert_eq!(got_rm, rm, "rm");
        prop_assert_eq!(got_rs1, rs1, "rs1");
        prop_assert_eq!(got_rs2, rs2, "rs2");
        prop_assert_eq!(got_fmt, fmt, "fmt");
        prop_assert_eq!(got_rs3, rs3, "rs3");
    }

    // Target: encoder.encode_fma
    #[test]
    fn encode_fma_abi_fn_alias(
        (_mn, opc, fmt) in fma_mn(),
        n in 0u32..=31u32,
        m in 0u32..=31u32,
        p in 0u32..=31u32,
        q in 0u32..=31u32
    ) {
        let via_n = sut_word(
            &[reg(&fn_name(n)), reg(&fn_name(m)), reg(&fn_name(p)), reg(&fn_name(q))],
            opc,
            fmt,
        )
        .unwrap_or_else(|e| panic!("fN rejected: {e}"));
        let via_abi = sut_word(
            &[reg(fabi(n)), reg(fabi(m)), reg(fabi(p)), reg(fabi(q))],
            opc,
            fmt,
        )
        .unwrap_or_else(|e| panic!("FABI rejected: {e}"));
        prop_assert_eq!(via_abi, via_n);
    }

    // Target: encoder.encode_fma
    #[test]
    fn encode_fma_rm_default_dyn(
        (_mn, opc, fmt) in fma_mn(),
        rd in fp_name(),
        rs1 in fp_name(),
        rs2 in fp_name(),
        rs3 in fp_name()
    ) {
        let four = sut_word(&[reg(&rd), reg(&rs1), reg(&rs2), reg(&rs3)], opc, fmt)
            .unwrap_or_else(|e| panic!("4-op rejected: {e}"));
        let dyn5 = sut_word(&[reg(&rd), reg(&rs1), reg(&rs2), reg(&rs3), rm_op("dyn")], opc, fmt)
            .unwrap_or_else(|e| panic!("dyn rejected: {e}"));
        prop_assert_eq!(four, dyn5);
        let (got_opc, _, rm, _, _, _, _) = unpack_r4(four);
        prop_assert_eq!(got_opc, opc);
        prop_assert_eq!(rm, 0b111u32, "omitted rm must be DYN");
    }

    // Target: encoder.encode_fma
    #[test]
    fn encode_fma_neg_arity_gpr(
        (_mn, opc, fmt) in fma_mn(),
        fp in fp_name(),
        fp2 in fp_name(),
        fp3 in fp_name(),
        gpr in gpr_name(),
        bad in bad_reg_operand()
    ) {
        prop_assert!(
            encode_fma(&[], opc, fmt).is_err(),
            "empty operand list must Err; got {:?}",
            encode_fma(&[], opc, fmt)
        );
        prop_assert!(
            encode_fma(&[reg(&fp)], opc, fmt).is_err(),
            "one operand must Err; got {:?}",
            encode_fma(&[reg(&fp)], opc, fmt)
        );
        prop_assert!(
            encode_fma(&[reg(&fp), reg(&fp2)], opc, fmt).is_err(),
            "two operands must Err; got {:?}",
            encode_fma(&[reg(&fp), reg(&fp2)], opc, fmt)
        );
        prop_assert!(
            encode_fma(&[reg(&fp), reg(&fp2), reg(&fp3)], opc, fmt).is_err(),
            "three operands must Err; got {:?}",
            encode_fma(&[reg(&fp), reg(&fp2), reg(&fp3)], opc, fmt)
        );
        let gpr_rd = [reg(&gpr), reg(&fp), reg(&fp2), reg(&fp3)];
        prop_assert!(
            encode_fma(&gpr_rd, opc, fmt).is_err(),
            "GPR rd {} must Err; got {:?}",
            gpr,
            encode_fma(&gpr_rd, opc, fmt)
        );
        let gpr_rs1 = [reg(&fp), reg(&gpr), reg(&fp2), reg(&fp3)];
        prop_assert!(
            encode_fma(&gpr_rs1, opc, fmt).is_err(),
            "GPR rs1 {} must Err; got {:?}",
            gpr,
            encode_fma(&gpr_rs1, opc, fmt)
        );
        let gpr_rs2 = [reg(&fp), reg(&fp2), reg(&gpr), reg(&fp3)];
        prop_assert!(
            encode_fma(&gpr_rs2, opc, fmt).is_err(),
            "GPR rs2 {} must Err; got {:?}",
            gpr,
            encode_fma(&gpr_rs2, opc, fmt)
        );
        let gpr_rs3 = [reg(&fp), reg(&fp2), reg(&fp3), reg(&gpr)];
        prop_assert!(
            encode_fma(&gpr_rs3, opc, fmt).is_err(),
            "GPR rs3 {} must Err; got {:?}",
            gpr,
            encode_fma(&gpr_rs3, opc, fmt)
        );
        let bad_rd = [bad.clone(), reg(&fp), reg(&fp2), reg(&fp3)];
        prop_assert!(
            encode_fma(&bad_rd, opc, fmt).is_err(),
            "non-reg rd must Err; got {:?}",
            encode_fma(&bad_rd, opc, fmt)
        );
    }

    // Target: encoder.encode_fma
    #[test]
    fn encode_fma_neg_extra(
        (mn, opc, fmt) in fma_mn(),
        rd in fp_name(),
        rs1 in fp_name(),
        rs2 in fp_name(),
        rs3 in fp_name(),
        extra in extra_operand()
    ) {
        let asm = format!("{} {}, {}, {}, {}, rne", mn, rd, rs1, rs2, rs3);
        let ops = vec![reg(&rd), reg(&rs1), reg(&rs2), reg(&rs3), rm_op("rne"), extra];
        prop_assert!(
            encode_fma(&ops, opc, fmt).is_err(),
            "6th operand must Err for {} (llvm-mc rejects extra operands); got {:?}",
            asm,
            encode_fma(&ops, opc, fmt)
        );
    }

    // Target: encoder.encode_fma
    #[test]
    fn encode_fma_neg_non_rm_fifth(
        (mn, opc, fmt) in fma_mn(),
        rd in fp_name(),
        rs1 in fp_name(),
        rs2 in fp_name(),
        rs3 in fp_name(),
        extra in non_rm_operand()
    ) {
        let asm = format!("{} {}, {}, {}, {}", mn, rd, rs1, rs2, rs3);
        let ops = [reg(&rd), reg(&rs1), reg(&rs2), reg(&rs3), extra];
        prop_assert!(
            encode_fma(&ops, opc, fmt).is_err(),
            "5th non-RoundingMode operand must Err for {} (optional rm only); got {:?}",
            asm,
            encode_fma(&ops, opc, fmt)
        );
    }
}
