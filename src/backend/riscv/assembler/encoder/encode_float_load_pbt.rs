// Oracle: differential — llvm-mc RISC-V assembler (I-type LOAD-FP word)
// Evidence: src/backend/riscv/assembler/README.md:6-7 textual assembly from codegen;
//   README.md:13-16 RV64GC includes F (single-precision) and D (double-precision);
//   README.md:309 flw/fld/fsw/fsd;
//   README.md:353 I-type: imm[11:0] | rs1 | funct3 | rd | opcode;
//   README.md:387 R_RISCV_PCREL_LO12_I/S: patches load/store lower 12 bits;
//   encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words";
//   encoder/mod.rs:313 "I-type: imm[31:20] | rs1[19:15] | funct3[14:12] | rd[11:7] | opcode[6:0]";
//   encoder/mod.rs:371 OP_LOAD_FP = 0b0000111;
//   encoder/mod.rs:67 "R_RISCV_PCREL_LO12_I - for ADDI/LW/LD (low 12 bits of PC-relative, I-type)";
//   encoder/mod.rs:73 "R_RISCV_LO12_I - for ADDI/LW/LD (absolute low 12 bits, I-type)";
//   encoder/mod.rs:713 "flw" => encode_float_load(operands, 0b010);
//   encoder/mod.rs:741 "fld" => encode_float_load(operands, 0b011);
//   float.rs:29 "float load: expected memory operand";
//   RISC-V Unprivileged ISA LOAD-FP: opcode=0000111, funct3=010 (FLW) / 011 (FLD),
//   rd is an FP register, rs1 is an integer base, imm[11:0] signed 12-bit.
// Stronger considered:
//   - State machine: rejected — encode_float_load is a pure function with no lifecycle
//   - Algebraic round-trip via in-tree decoder: rejected — no I-type LOAD-FP decoder
//   - encode_i / encode_load / C.FLW / encode_float_store as differential sibling: rejected —
//     same-job gate (private packer / integer load / compressed / stores)
// Weaker available: algebraic.invariant (I-type field unpack), algebraic.metamorphic
//   (FP ABI vs fN; GPR ABI vs xN including fp=s0), negative_error (oob imm / extra /
//   GPR dest / FP base / empty / hi-type modifiers)
// Differential: candidate=encode_float_load, reference=llvm-mc -triple=riscv64 -mattr=+f,+d -show-encoding,
//   SUT-boundary=internal-helper of RISC-V assembler,
//   mapping=[Reg(rd), Mem{base, offset}] <-> `mn rd, offset(rs1)`;
//   reloc-form word (imm=0) <-> `mn rd, 0(rs1)` plus R_RISCV_LO12_I / PCREL_LO12_I / TPREL_LO12_I.

use super::{encode_float_load, EncodeResult, RelocType};
use crate::backend::riscv::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;
const OP_LOAD_FP: u32 = 0b0000111;

const GABI: [&str; 32] = [
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3",
    "a4", "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11",
    "t3", "t4", "t5", "t6",
];

const FABI: [&str; 32] = [
    "ft0", "ft1", "ft2", "ft3", "ft4", "ft5", "ft6", "ft7", "fs0", "fs1", "fa0", "fa1",
    "fa2", "fa3", "fa4", "fa5", "fa6", "fa7", "fs2", "fs3", "fs4", "fs5", "fs6", "fs7",
    "fs8", "fs9", "fs10", "fs11", "ft8", "ft9", "ft10", "ft11",
];

const FLOAT_LOAD: [(&str, u32); 2] = [("flw", 0b010), ("fld", 0b011)];

fn cfg() -> ProptestConfig {
    ProptestConfig::with_cases(CASES)
}

fn xn(n: u32) -> String {
    format!("x{n}")
}

fn fn_name(n: u32) -> String {
    format!("f{n}")
}

fn gabi(n: u32) -> &'static str {
    GABI[n as usize]
}

fn fabi(n: u32) -> &'static str {
    FABI[n as usize]
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

/// Unpack I-type per RISC-V unprivileged ISA (not a copy of encode_i).
/// Layout: imm[11:0] | rs1[19:15] | funct3[14:12] | rd[11:7] | opcode[6:0]
fn unpack_i(word: u32) -> (u32, u32, u32, u32, i32) {
    let opcode = word & 0x7f;
    let rd = (word >> 7) & 0x1f;
    let funct3 = (word >> 12) & 0x7;
    let rs1 = (word >> 15) & 0x1f;
    let imm = (word as i32) >> 20;
    (opcode, rd, funct3, rs1, imm)
}

fn sut_word(ops: &[Operand], funct3: u32) -> Result<u32, String> {
    match encode_float_load(ops, funct3)? {
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

fn llvm_mc_rejects(asm: &str) -> bool {
    llvm_mc_word(asm).is_err()
}

fn gpr_name() -> impl Strategy<Value = String> {
    prop_oneof![
        (0u32..=31).prop_map(xn),
        (0u32..=31).prop_map(|n| gabi(n).to_string()),
        Just("fp".into()),
    ]
}

fn fp_name() -> impl Strategy<Value = String> {
    prop_oneof![
        (0u32..=31).prop_map(fn_name),
        (0u32..=31).prop_map(|n| fabi(n).to_string()),
    ]
}

fn float_load_mn() -> impl Strategy<Value = (&'static str, u32)> {
    prop::sample::select(FLOAT_LOAD.to_vec())
}

/// Signed 12-bit load immediate, with bounds pinned.
fn i12_imm() -> impl Strategy<Value = i64> {
    prop_oneof![
        Just(0i64),
        Just(1i64),
        Just(-1i64),
        Just(2i64),
        Just(-2i64),
        Just(4i64),
        Just(-4i64),
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
        other => panic!("unexpected reloc {other:?}"),
    }
}

fn sut_reloc(ops: &[Operand], funct3: u32) -> Result<(u32, &'static str, String, i64), String> {
    match encode_float_load(ops, funct3)? {
        EncodeResult::WordWithReloc { word, reloc } => {
            Ok((word, reloc_kind(&reloc.reloc_type), reloc.symbol, reloc.addend))
        }
        other => Err(format!("expected WordWithReloc, got {other:?}")),
    }
}

fn bad_2nd() -> impl Strategy<Value = Operand> {
    prop_oneof![
        Just(Operand::Imm(0)),
        Just(Operand::Imm(8)),
        Just(Operand::Csr("mstatus".into())),
        Just(Operand::FenceArg("iorw".into())),
        Just(Operand::RoundingMode("rne".into())),
        Just(Operand::Symbol("foo".into())),
        Just(Operand::Label("L0".into())),
    ]
}

fn hi_modifier() -> impl Strategy<Value = &'static str> {
    prop_oneof![Just("%hi"), Just("%pcrel_hi"), Just("%tprel_hi")]
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_float_load_kat_llvm_mc_flw_fa0_x1() {
    let want = 0x0000_a507u32;
    let mc = llvm_mc_word("flw fa0, 0(x1)").expect("llvm-mc KAT flw fa0, 0(x1)");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("fa0"), mem("x1", 0)], 0b010).expect("SUT KAT flw fa0, 0(x1)");
    assert_eq!(sut, want);
}

#[test]
fn encode_float_load_kat_llvm_mc_fld_fa0_sp8() {
    let want = 0x0081_3507u32;
    let mc = llvm_mc_word("fld fa0, 8(sp)").expect("llvm-mc KAT fld fa0, 8(sp)");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("fa0"), mem("sp", 8)], 0b011).expect("SUT KAT fld");
    assert_eq!(sut, want);
}

#[test]
fn encode_float_load_kat_llvm_mc_max_min() {
    let want_max = 0x7ff0_a507u32;
    let mc = llvm_mc_word("flw fa0, 2047(x1)").expect("llvm-mc KAT max");
    assert_eq!(mc, want_max, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("fa0"), mem("x1", 2047)], 0b010).expect("SUT KAT max");
    assert_eq!(sut, want_max);

    let want_min = 0x8000_a507u32;
    let mc = llvm_mc_word("flw fa0, -2048(x1)").expect("llvm-mc KAT min");
    assert_eq!(mc, want_min, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("fa0"), mem("x1", -2048)], 0b010).expect("SUT KAT min");
    assert_eq!(sut, want_min);
}

#[test]
fn encode_float_load_kat_llvm_mc_neg8_ft11() {
    let want_neg8 = 0xff84_3007u32;
    assert_eq!(llvm_mc_word("fld ft0, -8(s0)").unwrap(), want_neg8);
    assert_eq!(sut_word(&[reg("ft0"), mem("s0", -8)], 0b011).unwrap(), want_neg8);

    let want_ft11 = 0xffff_af87u32;
    assert_eq!(llvm_mc_word("flw ft11, -1(t6)").unwrap(), want_ft11);
    assert_eq!(sut_word(&[reg("ft11"), mem("t6", -1)], 0b010).unwrap(), want_ft11);
}

/// Regression: out-of-range LOAD-FP immediate must be rejected
/// (llvm-mc: integer in [-2048, 2047]).
#[test]
fn test_encode_float_load_regression_imm_oob() {
    let ops = [reg("fa0"), mem("x1", 2048)];
    assert!(
        encode_float_load(&ops, 0b010).is_err(),
        "flw fa0, 2048(x1) must Err (imm12 range); got {:?}",
        encode_float_load(&ops, 0b010)
    );
}

/// Regression: extra operand must be rejected (llvm-mc errors).
#[test]
fn test_encode_float_load_regression_extra_operand() {
    let ops = [reg("fa0"), mem("x1", 0), Operand::Imm(0)];
    assert!(
        encode_float_load(&ops, 0b010).is_err(),
        "flw fa0, 0(x1) with a third operand must Err; got {:?}",
        encode_float_load(&ops, 0b010)
    );
}

/// Regression: hi-type modifier must be rejected (llvm-mc only allows %lo/%pcrel_lo/%tprel_lo).
#[test]
fn test_encode_float_load_regression_hi_modifier() {
    let ops = [reg("fa0"), Operand::MemSymbol {
        base: "x1".into(),
        symbol: "%hi(foo)".into(),
        modifier: String::new(),
    }];
    assert!(
        encode_float_load(&ops, 0b010).is_err(),
        "flw fa0, %hi(foo)(x1) must Err (llvm-mc only allows %lo/%pcrel_lo/%tprel_lo); got {:?}",
        encode_float_load(&ops, 0b010)
    );
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_float_load_diff_imm_llvm_mc(
        (mn, f3) in float_load_mn(),
        rd in fp_name(),
        rs1 in gpr_name(),
        off in i12_imm()
    ) {
        let asm = format!("{} {}, {}({})", mn, rd, off, rs1);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&[reg(&rd), mem(&rs1, off)], f3)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "SUT {:08x} != llvm-mc {:08x} for {}", sut, mc, asm);
    }

    #[test]
    fn encode_float_load_i_type_fields(
        (mn, f3) in float_load_mn(),
        rd in 0u32..=31u32,
        rs1 in 0u32..=31u32,
        off in i12_imm()
    ) {
        let w = sut_word(&[reg(&fn_name(rd)), mem(&xn(rs1), off)], f3)
            .unwrap_or_else(|e| panic!("SUT rejected {} f{}, {}(x{}): {}", mn, rd, off, rs1, e));
        let (opc, got_rd, got_f3, got_rs1, got_off) = unpack_i(w);
        prop_assert_eq!(opc, OP_LOAD_FP, "opcode");
        prop_assert_eq!(got_rd, rd, "rd");
        prop_assert_eq!(got_f3, f3, "funct3");
        prop_assert_eq!(got_rs1, rs1, "rs1");
        prop_assert_eq!(got_off as i64, off, "imm");
    }

    #[test]
    fn encode_float_load_abi_fn_alias(
        (_mn, f3) in float_load_mn(),
        n in 0u32..=31u32,
        m in 0u32..=31u32,
        off in i12_imm()
    ) {
        let via_n = sut_word(&[reg(&fn_name(n)), mem(&xn(m), off)], f3)
            .unwrap_or_else(|e| panic!("fN/xN rejected: {}", e));
        let via_abi = sut_word(&[reg(fabi(n)), mem(gabi(m), off)], f3)
            .unwrap_or_else(|e| panic!("FABI/GABI rejected: {}", e));
        prop_assert_eq!(via_abi, via_n);
        if m == 8 {
            let via_fp_b = sut_word(&[reg(&fn_name(n)), mem("fp", off)], f3)
                .unwrap_or_else(|e| panic!("fp base rejected: {}", e));
            prop_assert_eq!(via_fp_b, via_n);
        }
    }

    #[test]
    fn encode_float_load_reloc_lo(
        (_mn, f3) in float_load_mn(),
        rd in fp_name(),
        rs1 in gpr_name(),
        s in ident()
    ) {
        let zero = sut_word(&[reg(&rd), mem(&rs1, 0)], f3)
            .unwrap_or_else(|e| panic!("SUT rejected load {}, 0({}): {}", rd, rs1, e));

        let ops_pcrel = [reg(&rd), Operand::MemSymbol {
            base: rs1.clone(),
            symbol: format!("%pcrel_lo({})", s),
            modifier: String::new(),
        }];
        let (word, kind, got_sym, addend) = sut_reloc(&ops_pcrel, f3)
            .unwrap_or_else(|e| panic!("SUT rejected %pcrel_lo({})({}): {}", s, rs1, e));
        prop_assert_eq!(word, zero, "reloc-form word must equal load {}, 0({})", rd, rs1);
        prop_assert_eq!(kind, "PcrelLo12I");
        prop_assert_eq!(got_sym, s.clone(), "reloc symbol must be the identifier");
        prop_assert_eq!(addend, 0i64);

        let ops_lo = [reg(&rd), Operand::MemSymbol {
            base: rs1.clone(),
            symbol: format!("%lo({})", s),
            modifier: String::new(),
        }];
        let (word_lo, kind_lo, got_lo, add_lo) = sut_reloc(&ops_lo, f3)
            .unwrap_or_else(|e| panic!("SUT rejected %lo({})({}): {}", s, rs1, e));
        prop_assert_eq!(word_lo, zero);
        prop_assert_eq!(kind_lo, "Lo12I");
        prop_assert_eq!(got_lo, s.clone());
        prop_assert_eq!(add_lo, 0i64);

        let ops_tp = [reg(&rd), Operand::MemSymbol {
            base: rs1.clone(),
            symbol: format!("%tprel_lo({})", s),
            modifier: String::new(),
        }];
        let (word_tp, kind_tp, got_tp, add_tp) = sut_reloc(&ops_tp, f3)
            .unwrap_or_else(|e| panic!("SUT rejected %tprel_lo({})({}): {}", s, rs1, e));
        prop_assert_eq!(word_tp, zero);
        prop_assert_eq!(kind_tp, "TprelLo12I");
        prop_assert_eq!(got_tp, s);
        prop_assert_eq!(add_tp, 0i64);
    }

    #[test]
    fn encode_float_load_neg_imm_oob(
        (mn, f3) in float_load_mn(),
        rd in fp_name(),
        rs1 in gpr_name(),
        imm in oob_i12()
    ) {
        prop_assume!(!(-2048..=2047).contains(&imm));
        let asm = format!("{} {}, {}({})", mn, rd, imm, rs1);
        prop_assert!(
            llvm_mc_rejects(&asm),
            "reference unexpectedly accepted {}",
            asm
        );
        let ops = [reg(&rd), mem(&rs1, imm)];
        prop_assert!(
            encode_float_load(&ops, f3).is_err(),
            "oob imm {} must Err (llvm-mc range [-2048, 2047]); got {:?}",
            imm,
            encode_float_load(&ops, f3)
        );
    }

    #[test]
    fn encode_float_load_neg_extra(
        (mn, f3) in float_load_mn(),
        rd in fp_name(),
        rs1 in gpr_name(),
        off in i12_imm(),
        extra in extra_operand()
    ) {
        let asm = format!("{} {}, {}({})", mn, rd, off, rs1);
        let ops = vec![reg(&rd), mem(&rs1, off), extra];
        prop_assert!(
            encode_float_load(&ops, f3).is_err(),
            "extra operand must Err for {} (llvm-mc rejects extra operands); got {:?}",
            asm,
            encode_float_load(&ops, f3)
        );
    }

    #[test]
    fn encode_float_load_neg_arity_gpr(
        (_mn, f3) in float_load_mn(),
        rd in fp_name(),
        gpr in gpr_name(),
        fp in fp_name(),
        off in i12_imm(),
        bad in bad_2nd()
    ) {
        prop_assert!(
            encode_float_load(&[], f3).is_err(),
            "empty operand list must Err; got {:?}",
            encode_float_load(&[], f3)
        );
        prop_assert!(
            encode_float_load(&[reg(&rd)], f3).is_err(),
            "missing memory operand must Err; got {:?}",
            encode_float_load(&[reg(&rd)], f3)
        );
        let gpr_rd = [reg(&gpr), mem(&gpr, off)];
        prop_assert!(
            encode_float_load(&gpr_rd, f3).is_err(),
            "GPR dest {} must Err (not an FP register); got {:?}",
            gpr,
            encode_float_load(&gpr_rd, f3)
        );
        let fp_base = [reg(&rd), mem(&fp, off)];
        prop_assert!(
            encode_float_load(&fp_base, f3).is_err(),
            "FP base {} must Err (not a GPR); got {:?}",
            fp,
            encode_float_load(&fp_base, f3)
        );
        let bad_ops = [reg(&rd), bad];
        prop_assert!(
            encode_float_load(&bad_ops, f3).is_err(),
            "non-memory 2nd operand must Err; got {:?}",
            encode_float_load(&bad_ops, f3)
        );
    }

    #[test]
    fn encode_float_load_neg_hi_modifier(
        (mn, f3) in float_load_mn(),
        rd in fp_name(),
        rs1 in gpr_name(),
        s in ident(),
        hi in hi_modifier()
    ) {
        let inner = format!("{}({})", hi, s);
        let asm = format!("{} {}, {}({})", mn, rd, inner, rs1);
        prop_assert!(
            llvm_mc_rejects(&asm),
            "reference unexpectedly accepted {}",
            asm
        );
        let ops = [reg(&rd), Operand::MemSymbol {
            base: rs1.clone(),
            symbol: inner.clone(),
            modifier: String::new(),
        }];
        prop_assert!(
            encode_float_load(&ops, f3).is_err(),
            "hi-type modifier {} must Err on float load (llvm-mc only allows %lo/%pcrel_lo/%tprel_lo); got {:?}",
            inner,
            encode_float_load(&ops, f3)
        );
    }
}
