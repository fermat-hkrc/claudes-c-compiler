// Oracle: differential — llvm-mc RISC-V assembler (I-type OP-IMM-32 / addiw)
// Evidence: src/backend/riscv/assembler/README.md:6-7 textual assembly from codegen;
//   README.md:301 I-type includes addiw;
//   README.md:353 I-type: imm[11:0] | rs1 | funct3 | rd | opcode;
//   encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words";
//   encoder/mod.rs:298 "I-type: imm[31:20] | rs1[19:15] | funct3[14:12] | rd[11:7] | opcode[6:0]";
//   encoder/mod.rs:348 OP_OP_IMM_32 = 0b0011011;
//   encoder/mod.rs:75 "R_RISCV_PCREL_LO12_I - for ADDI/LW/LD (low 12 bits of PC-relative, I-type)";
//   encoder/mod.rs:81 "R_RISCV_LO12_I - for ADDI/LW/LD (absolute low 12 bits, I-type)";
//   encoder/mod.rs:563 "addiw" => encode_alu_imm_w(operands, 0b000);
//   RISC-V Unprivileged ISA OP-IMM-32: opcode=0011011, funct3=000 ADDIW, rd, rs1, imm[11:0].
// Stronger considered:
//   - State machine: rejected — encode_alu_imm_w is a pure function with no lifecycle
//   - Algebraic round-trip via in-tree decoder: rejected — no OP-IMM-32 decoder
//   - encode_i / encode_alu_imm / encode_shift_imm_w / C.ADDIW as differential sibling: rejected —
//     same-job gate (private packer / OP-IMM 64-bit / shamt I-type / compressed)
// Weaker available: algebraic.invariant (I-type field unpack), algebraic.metamorphic
//   (ABI vs xN alias; Imm 0-31 vs xN), negative_error (oob imm / extra / FP rd-rs1 / empty / symbol)
// Differential: candidate=encode_alu_imm_w, reference=llvm-mc -triple=riscv64 -show-encoding,
//   SUT-boundary=internal-helper of RISC-V assembler,
//   mapping=[Reg(rd), Reg(rs1), Imm(imm)] <-> `addiw rd, rs1, imm`;
//   reloc-form word (imm=0) <-> `addiw rd, rs1, 0` plus R_RISCV_LO12_I / PCREL_LO12_I / TPREL_LO12_I.

use super::{encode_alu_imm_w, EncodeResult, RelocType};
use crate::backend::riscv::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;
const OP_OP_IMM_32: u32 = 0b0011011;
const FUNCT3_ADDIW: u32 = 0b000;

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

/// Unpack I-type per RISC-V unprivileged ISA (not a copy of encode_i).
/// Layout: imm[11:0] | rs1[19:15] | funct3[14:12] | rd[11:7] | opcode[6:0]
fn unpack_i(word: u32) -> (u32, u32, u32, u32, i32) {
    let opcode = word & 0x7f;
    let rd = (word >> 7) & 0x1f;
    let funct3 = (word >> 12) & 0x7;
    let rs1 = (word >> 15) & 0x1f;
    let imm12 = (word >> 20) & 0xfff;
    let imm = ((imm12 as i32) << 20) >> 20;
    (opcode, funct3, rd, rs1, imm)
}

fn sut_word(ops: &[Operand], funct3: u32) -> Result<u32, String> {
    match encode_alu_imm_w(ops, funct3)? {
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

fn llvm_mc_rejects(asm: &str) -> bool {
    llvm_mc_word(asm).is_err()
}

fn gpr_name() -> impl Strategy<Value = String> {
    prop_oneof![
        (0u32..=31).prop_map(xn),
        (0u32..=31).prop_map(|n| abi_name(n).to_string()),
        Just("fp".into()),
    ]
}

fn i12_imm() -> impl Strategy<Value = i64> {
    prop_oneof![
        Just(0i64),
        Just(1i64),
        Just(-1i64),
        Just(2i64),
        Just(-2i64),
        Just(8i64),
        Just(-8i64),
        Just(2047i64),
        Just(-2048i64),
        -2048i64..=2047i64,
    ]
}

fn oob_i12() -> impl Strategy<Value = i64> {
    prop_oneof![
        Just(2048i64),
        Just(-2049i64),
        Just(2049i64),
        Just(-2050i64),
        Just(4095i64),
        Just(4096i64),
        Just(-4096i64),
        Just(i64::MIN),
        Just(i64::MAX),
        2048i64..=i64::MAX,
        i64::MIN..=-2049,
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
        Just(Operand::FenceArg("iorw".into())),
        Just(Operand::Csr("mstatus".into())),
        Just(Operand::RoundingMode("rne".into())),
    ]
}

fn ident() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("foo".into()),
        Just("bar".into()),
        Just("tls_var".into()),
        Just("_x".into()),
        Just("sym0".into()),
        Just("Lsym".into()),
    ]
}

fn reloc_kind(t: &RelocType) -> &'static str {
    match t {
        RelocType::PcrelLo12I => "PcrelLo12I",
        RelocType::Lo12I => "Lo12I",
        RelocType::TprelLo12I => "TprelLo12I",
        RelocType::PcrelHi20 => "PcrelHi20",
        RelocType::Hi20 => "Hi20",
        RelocType::TprelHi20 => "TprelHi20",
        RelocType::PcrelLo12S => "PcrelLo12S",
        RelocType::Lo12S => "Lo12S",
        RelocType::TprelLo12S => "TprelLo12S",
        RelocType::GotHi20 => "GotHi20",
        RelocType::TlsGotHi20 => "TlsGotHi20",
        RelocType::TlsGdHi20 => "TlsGdHi20",
        RelocType::TprelAdd => "TprelAdd",
        other => panic!("unexpected reloc {other:?}"),
    }
}

fn sut_reloc(ops: &[Operand], funct3: u32) -> Result<(u32, &'static str, String, i64), String> {
    match encode_alu_imm_w(ops, funct3)? {
        EncodeResult::WordWithReloc { word, reloc } => {
            Ok((word, reloc_kind(&reloc.reloc_type), reloc.symbol, reloc.addend))
        }
        other => Err(format!("expected WordWithReloc, got {other:?}")),
    }
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

fn bad_3rd() -> impl Strategy<Value = Operand> {
    prop_oneof![
        Just(Operand::Csr("mstatus".into())),
        Just(Operand::FenceArg("iorw".into())),
        Just(Operand::RoundingMode("rne".into())),
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
    ]
}

fn lo_modifier() -> impl Strategy<Value = &'static str> {
    prop_oneof![Just("%lo"), Just("%pcrel_lo"), Just("%tprel_lo"),]
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

fn expected_lo_kind(m: &str) -> &'static str {
    match m {
        "%lo" => "Lo12I",
        "%pcrel_lo" => "PcrelLo12I",
        "%tprel_lo" => "TprelLo12I",
        other => panic!("unexpected lo modifier {other}"),
    }
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_alu_imm_w_kat_llvm_mc_addiw_x1_x2_1() {
    let want = 0x0011_009bu32;
    let mc = llvm_mc_word("addiw x1, x2, 1").expect("llvm-mc KAT addiw x1, x2, 1");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("x1"), reg("x2"), Operand::Imm(1)], FUNCT3_ADDIW)
        .expect("SUT KAT addiw x1, x2, 1");
    assert_eq!(sut, want);
}

#[test]
fn encode_alu_imm_w_kat_llvm_mc_addiw_neg1() {
    let want = 0xfff1_009bu32;
    let mc = llvm_mc_word("addiw x1, x2, -1").expect("llvm-mc KAT addiw -1");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("x1"), reg("x2"), Operand::Imm(-1)], FUNCT3_ADDIW)
        .expect("SUT KAT addiw -1");
    assert_eq!(sut, want);
}

#[test]
fn encode_alu_imm_w_kat_llvm_mc_max_min() {
    let want_max = 0x7ff1_009bu32;
    let mc = llvm_mc_word("addiw x1, x2, 2047").expect("llvm-mc KAT max");
    assert_eq!(mc, want_max, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("x1"), reg("x2"), Operand::Imm(2047)], FUNCT3_ADDIW)
        .expect("SUT KAT max");
    assert_eq!(sut, want_max);

    let want_min = 0x8001_009bu32;
    let mc = llvm_mc_word("addiw x1, x2, -2048").expect("llvm-mc KAT min");
    assert_eq!(mc, want_min, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("x1"), reg("x2"), Operand::Imm(-2048)], FUNCT3_ADDIW)
        .expect("SUT KAT min");
    assert_eq!(sut, want_min);
}

#[test]
fn encode_alu_imm_w_kat_llvm_mc_addiw_zero() {
    let want = 0x0001_009bu32;
    let mc = llvm_mc_word("addiw x1, x2, 0").expect("llvm-mc KAT addiw 0");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("x1"), reg("x2"), Operand::Imm(0)], FUNCT3_ADDIW)
        .expect("SUT KAT addiw 0");
    assert_eq!(sut, want);
    let sext = llvm_mc_word("sext.w x1, x2").expect("llvm-mc KAT sext.w");
    assert_eq!(sext, want);
}

/// Regression: out-of-range OP-IMM-32 immediate must be rejected
/// (llvm-mc: integer in [-2048, 2047]).
#[test]
fn test_encode_alu_imm_w_regression_imm_oob() {
    let ops = [reg("x1"), reg("x2"), Operand::Imm(2048)];
    assert!(
        encode_alu_imm_w(&ops, FUNCT3_ADDIW).is_err(),
        "addiw x1, x2, 2048 must Err (imm12 range); got {:?}",
        encode_alu_imm_w(&ops, FUNCT3_ADDIW)
    );
}

/// Regression: extra operand must be rejected (llvm-mc errors).
#[test]
fn test_encode_alu_imm_w_regression_extra_operand() {
    let ops = [reg("x1"), reg("x2"), Operand::Imm(0), Operand::Imm(0)];
    assert!(
        encode_alu_imm_w(&ops, FUNCT3_ADDIW).is_err(),
        "addiw x1, x2, 0 with a fourth operand must Err; got {:?}",
        encode_alu_imm_w(&ops, FUNCT3_ADDIW)
    );
}

/// Regression: %lo on an OP-IMM-32 immediate must produce Lo12I reloc
/// (llvm-mc accepts addiw rd, rs1, %lo(foo) with fixup_riscv_lo12_i).
#[test]
fn test_encode_alu_imm_w_regression_lo_reloc() {
    let ops = [reg("x1"), reg("x2"), Operand::Symbol("%lo(foo)".into())];
    match encode_alu_imm_w(&ops, FUNCT3_ADDIW) {
        Ok(EncodeResult::WordWithReloc { word, reloc }) => {
            let zero = sut_word(&[reg("x1"), reg("x2"), Operand::Imm(0)], FUNCT3_ADDIW)
                .expect("addiw imm0");
            assert_eq!(word, zero);
            assert!(
                matches!(reloc.reloc_type, RelocType::Lo12I),
                "expected Lo12I, got {:?}",
                reloc.reloc_type
            );
            assert_eq!(reloc.symbol, "foo");
            assert_eq!(reloc.addend, 0);
        }
        other => panic!(
            "addiw x1, x2, %lo(foo) must be WordWithReloc Lo12I; got {:?}",
            other
        ),
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_alu_imm_w_diff_imm_llvm_mc(
        rd in gpr_name(),
        rs1 in gpr_name(),
        imm in i12_imm()
    ) {
        let asm = format!("addiw {}, {}, {}", rd, rs1, imm);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&[reg(&rd), reg(&rs1), Operand::Imm(imm)], FUNCT3_ADDIW)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "SUT {:08x} != llvm-mc {:08x} for {}", sut, mc, asm);
    }

    #[test]
    fn encode_alu_imm_w_i_type_fields(
        rd in 0u32..=31u32,
        rs1 in 0u32..=31u32,
        imm in i12_imm()
    ) {
        let w = sut_word(&[reg(&xn(rd)), reg(&xn(rs1)), Operand::Imm(imm)], FUNCT3_ADDIW)
            .unwrap_or_else(|e| panic!("SUT rejected addiw x{}, x{}, {}: {}", rd, rs1, imm, e));
        let (opc, got_f3, got_rd, got_rs1, got_imm) = unpack_i(w);
        prop_assert_eq!(opc, OP_OP_IMM_32, "opcode");
        prop_assert_eq!(got_f3, FUNCT3_ADDIW, "funct3");
        prop_assert_eq!(got_rd, rd, "rd");
        prop_assert_eq!(got_rs1, rs1, "rs1");
        prop_assert_eq!(got_imm as i64, imm, "imm");
    }

    #[test]
    fn encode_alu_imm_w_abi_xn_alias(
        n in 0u32..=31u32,
        m in 0u32..=31u32,
        imm in i12_imm()
    ) {
        let via_x = sut_word(&[reg(&xn(n)), reg(&xn(m)), Operand::Imm(imm)], FUNCT3_ADDIW)
            .unwrap_or_else(|e| panic!("xN rejected: {}", e));
        let via_abi = sut_word(&[reg(abi_name(n)), reg(abi_name(m)), Operand::Imm(imm)], FUNCT3_ADDIW)
            .unwrap_or_else(|e| panic!("ABI rejected: {}", e));
        prop_assert_eq!(via_abi, via_x);
        if n == 8 {
            let via_fp = sut_word(&[reg("fp"), reg(&xn(m)), Operand::Imm(imm)], FUNCT3_ADDIW)
                .unwrap_or_else(|e| panic!("fp rejected: {}", e));
            prop_assert_eq!(via_fp, via_x);
        }
        if m == 8 {
            let via_fp_s = sut_word(&[reg(&xn(n)), reg("fp"), Operand::Imm(imm)], FUNCT3_ADDIW)
                .unwrap_or_else(|e| panic!("fp rs1 rejected: {}", e));
            prop_assert_eq!(via_fp_s, via_x);
        }
    }

    #[test]
    fn encode_alu_imm_w_imm_as_reg(
        n in 0u32..=31u32,
        m in 0u32..=31u32,
        imm in i12_imm()
    ) {
        let via_x = sut_word(&[reg(&xn(n)), reg(&xn(m)), Operand::Imm(imm)], FUNCT3_ADDIW)
            .unwrap_or_else(|e| panic!("xN rejected: {}", e));
        let via_imm = sut_word(
            &[Operand::Imm(n as i64), Operand::Imm(m as i64), Operand::Imm(imm)],
            FUNCT3_ADDIW,
        )
        .unwrap_or_else(|e| panic!("Imm-as-reg rejected: {}", e));
        prop_assert_eq!(via_imm, via_x);
    }

    #[test]
    fn encode_alu_imm_w_neg_imm_oob(
        rd in gpr_name(),
        rs1 in gpr_name(),
        imm in oob_i12()
    ) {
        prop_assume!(!(-2048..=2047).contains(&imm));
        let asm = format!("addiw {}, {}, {}", rd, rs1, imm);
        prop_assert!(
            llvm_mc_rejects(&asm),
            "reference unexpectedly accepted {}",
            asm
        );
        let ops = [reg(&rd), reg(&rs1), Operand::Imm(imm)];
        prop_assert!(
            encode_alu_imm_w(&ops, FUNCT3_ADDIW).is_err(),
            "oob imm {} must Err (llvm-mc range [-2048, 2047]); got {:?}",
            imm,
            encode_alu_imm_w(&ops, FUNCT3_ADDIW)
        );
    }

    #[test]
    fn encode_alu_imm_w_neg_extra(
        rd in gpr_name(),
        rs1 in gpr_name(),
        imm in i12_imm(),
        extra in extra_operand()
    ) {
        let asm = format!("addiw {}, {}, {}", rd, rs1, imm);
        let ops = vec![reg(&rd), reg(&rs1), Operand::Imm(imm), extra];
        prop_assert!(
            encode_alu_imm_w(&ops, FUNCT3_ADDIW).is_err(),
            "extra operand must Err for {} (llvm-mc rejects extra operands); got {:?}",
            asm,
            encode_alu_imm_w(&ops, FUNCT3_ADDIW)
        );
    }

    #[test]
    fn encode_alu_imm_w_neg_arity_fp(
        rd in gpr_name(),
        rs1 in gpr_name(),
        imm in i12_imm(),
        fp in fp_name(),
        bad in bad_3rd()
    ) {
        prop_assert!(
            encode_alu_imm_w(&[], FUNCT3_ADDIW).is_err(),
            "empty operand list must Err; got {:?}",
            encode_alu_imm_w(&[], FUNCT3_ADDIW)
        );
        prop_assert!(
            encode_alu_imm_w(&[reg(&rd)], FUNCT3_ADDIW).is_err(),
            "missing rs1/imm must Err; got {:?}",
            encode_alu_imm_w(&[reg(&rd)], FUNCT3_ADDIW)
        );
        prop_assert!(
            encode_alu_imm_w(&[reg(&rd), reg(&rs1)], FUNCT3_ADDIW).is_err(),
            "missing immediate must Err; got {:?}",
            encode_alu_imm_w(&[reg(&rd), reg(&rs1)], FUNCT3_ADDIW)
        );
        let fp_rd = [reg(&fp), reg(&rs1), Operand::Imm(imm)];
        prop_assert!(
            encode_alu_imm_w(&fp_rd, FUNCT3_ADDIW).is_err(),
            "FP dest {} must Err (not a GPR); got {:?}",
            fp,
            encode_alu_imm_w(&fp_rd, FUNCT3_ADDIW)
        );
        let fp_rs1 = [reg(&rd), reg(&fp), Operand::Imm(imm)];
        prop_assert!(
            encode_alu_imm_w(&fp_rs1, FUNCT3_ADDIW).is_err(),
            "FP rs1 {} must Err (not a GPR); got {:?}",
            fp,
            encode_alu_imm_w(&fp_rs1, FUNCT3_ADDIW)
        );
        let bad_ops = [reg(&rd), reg(&rs1), bad];
        prop_assert!(
            encode_alu_imm_w(&bad_ops, FUNCT3_ADDIW).is_err(),
            "non-imm 3rd operand must Err; got {:?}",
            encode_alu_imm_w(&bad_ops, FUNCT3_ADDIW)
        );
    }

    #[test]
    fn encode_alu_imm_w_reloc_lo(
        rd in gpr_name(),
        rs1 in gpr_name(),
        s in ident(),
        m in lo_modifier()
    ) {
        let zero = sut_word(&[reg(&rd), reg(&rs1), Operand::Imm(0)], FUNCT3_ADDIW)
            .unwrap_or_else(|e| panic!("SUT rejected addiw-imm0 {}, {}: {}", rd, rs1, e));
        let inner = format!("{}({})", m, s);
        let ops = [reg(&rd), reg(&rs1), Operand::Symbol(inner.clone())];
        let (word, kind, got_sym, addend) = sut_reloc(&ops, FUNCT3_ADDIW)
            .unwrap_or_else(|e| panic!("SUT rejected {}: {}", inner, e));
        prop_assert_eq!(word, zero, "reloc-form word must equal addiw {}, {}, 0", rd, rs1);
        prop_assert_eq!(kind, expected_lo_kind(m));
        prop_assert_eq!(got_sym, s, "reloc symbol must be the identifier");
        prop_assert_eq!(addend, 0i64);
    }

    #[test]
    fn encode_alu_imm_w_neg_invalid_name(
        rd in gpr_name(),
        rs1 in gpr_name(),
        imm in i12_imm(),
        bad in invalid_gpr_name(),
        which in 0u32..=1u32
    ) {
        let mut ops = vec![reg(&rd), reg(&rs1), Operand::Imm(imm)];
        ops[which as usize] = reg(&bad);
        prop_assert!(
            encode_alu_imm_w(&ops, FUNCT3_ADDIW).is_err(),
            "invalid integer register {} at operand {} must Err; got {:?}",
            bad,
            which,
            encode_alu_imm_w(&ops, FUNCT3_ADDIW)
        );
    }
}
