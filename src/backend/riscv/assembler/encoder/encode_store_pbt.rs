// Oracle: differential — llvm-mc RISC-V assembler (S-type STORE word)
// Evidence: src/backend/riscv/assembler/README.md:6-7 textual assembly from codegen;
//   README.md:302 S-type includes sb/sh/sw/sd;
//   README.md:354 S-type: imm[11:5]| rs2 | rs1 | funct3 | imm[4:0] | opcode;
//   README.md:387 R_RISCV_PCREL_LO12_I/S: patches load/store lower 12 bits;
//   encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words";
//   encoder/mod.rs:297 "S-type: imm[11:5] | rs2[24:20] | rs1[19:15] | funct3[14:12] | imm[4:0] | opcode[6:0]";
//   encoder/mod.rs:339 OP_STORE = 0b0100011;
//   encoder/mod.rs:71 "R_RISCV_PCREL_LO12_S - for SW/SD (low 12 bits of PC-relative, S-type)";
//   encoder/mod.rs:77 "R_RISCV_LO12_S - for SW/SD (absolute low 12 bits, S-type)";
//   encoder/mod.rs:488-492 sb/sh/sw/sd => encode_store;
//   base.rs:219 "store: expected memory operand";
//   RISC-V Unprivileged ISA STORE: opcode=0100011, funct3 by width, rs1, rs2, imm[11:5]|imm[4:0].
// Stronger considered:
//   - State machine: rejected — encode_store is a pure function with no lifecycle
//   - Algebraic round-trip via in-tree decoder: rejected — no S-type store decoder
//   - encode_s / encode_float_store / C.SW / encode_load as differential sibling: rejected —
//     same-job gate (private packer / FP store / compressed / loads)
// Weaker available: algebraic.invariant (S-type field unpack), algebraic.metamorphic
//   (ABI vs xN alias), negative_error (oob imm / extra / FP rs2-base / empty / hi-modifier)
// Differential: candidate=encode_store, reference=llvm-mc -triple=riscv64 -show-encoding,
//   SUT-boundary=internal-helper of RISC-V assembler,
//   mapping=[Reg(rs2), Mem{base, offset}] <-> `mn rs2, offset(rs1)`;
//   reloc-form word (imm=0) <-> `mn rs2, 0(rs1)` plus R_RISCV_LO12_S / PCREL_LO12_S / TPREL_LO12_S.

use super::{encode_store, EncodeResult, RelocType};
use crate::backend::riscv::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;
const OP_STORE: u32 = 0b0100011;

const ABI: [&str; 32] = [
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3",
    "a4", "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11",
    "t3", "t4", "t5", "t6",
];

const STORE: [(&str, u32); 4] = [
    ("sb", 0b000),
    ("sh", 0b001),
    ("sw", 0b010),
    ("sd", 0b011),
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

fn mem(base: &str, offset: i64) -> Operand {
    Operand::Mem {
        base: base.to_string(),
        offset,
    }
}

/// Unpack S-type per RISC-V unprivileged ISA (not a copy of encode_s).
/// Layout: imm[11:5] | rs2[24:20] | rs1[19:15] | funct3[14:12] | imm[4:0] | opcode[6:0]
fn unpack_s(word: u32) -> (u32, u32, u32, u32, i32) {
    let opcode = word & 0x7f;
    let imm_4_0 = (word >> 7) & 0x1f;
    let funct3 = (word >> 12) & 0x7;
    let rs1 = (word >> 15) & 0x1f;
    let rs2 = (word >> 20) & 0x1f;
    let imm_11_5 = (word >> 25) & 0x7f;
    let imm12 = (imm_11_5 << 5) | imm_4_0;
    let imm = ((imm12 as i32) << 20) >> 20;
    (opcode, funct3, rs1, rs2, imm)
}

fn sut_word(ops: &[Operand], funct3: u32) -> Result<u32, String> {
    match encode_store(ops, funct3)? {
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

fn store_mn() -> impl Strategy<Value = (&'static str, u32)> {
    prop::sample::select(STORE.to_vec())
}

/// Signed 12-bit store immediate, with bounds pinned.
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
        RelocType::PcrelLo12S => "PcrelLo12S",
        RelocType::Lo12S => "Lo12S",
        RelocType::TprelLo12S => "TprelLo12S",
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
    match encode_store(ops, funct3)? {
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

fn bad_2nd() -> impl Strategy<Value = Operand> {
    prop_oneof![
        Just(Operand::Imm(0)),
        Just(Operand::Imm(8)),
        Just(Operand::Csr("mstatus".into())),
        Just(Operand::FenceArg("iorw".into())),
        Just(Operand::RoundingMode("rne".into())),
        Just(Operand::Symbol("foo".into())),
        Just(Operand::Label("L0".into())),
        Just(Operand::SymbolOffset("foo".into(), 4)),
    ]
}

fn hi_modifier() -> impl Strategy<Value = &'static str> {
    prop_oneof![Just("%hi"), Just("%pcrel_hi"), Just("%tprel_hi"),]
}

fn other_modifier_form(s: &str) -> impl Strategy<Value = String> {
    let s = s.to_string();
    prop_oneof![
        Just(format!("%got_pcrel_hi({s})")),
        Just(format!("%tls_ie_pcrel_hi({s})")),
        Just(format!("%tls_gd_pcrel_hi({s})")),
        Just(format!("%tprel_add({s})")),
        Just(s.clone()),
    ]
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_store_kat_llvm_mc_sd_x1_x2() {
    let want = 0x0011_3023u32;
    let mc = llvm_mc_word("sd x1, 0(x2)").expect("llvm-mc KAT sd x1, 0(x2)");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("x1"), mem("x2", 0)], 0b011).expect("SUT KAT sd x1, 0(x2)");
    assert_eq!(sut, want);
}

#[test]
fn encode_store_kat_llvm_mc_sb_x0_x1() {
    let want = 0x0000_8023u32;
    let mc = llvm_mc_word("sb x0, 0(x1)").expect("llvm-mc KAT sb x0, 0(x1)");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("x0"), mem("x1", 0)], 0b000).expect("SUT KAT sb");
    assert_eq!(sut, want);
}

#[test]
fn encode_store_kat_llvm_mc_sw_off8() {
    let want = 0x0011_2423u32;
    let mc = llvm_mc_word("sw x1, 8(x2)").expect("llvm-mc KAT sw x1, 8(x2)");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("x1"), mem("x2", 8)], 0b010).expect("SUT KAT sw");
    assert_eq!(sut, want);
}

#[test]
fn encode_store_kat_llvm_mc_sd_neg8() {
    let want = 0xfe11_3c23u32;
    let mc = llvm_mc_word("sd x1, -8(x2)").expect("llvm-mc KAT sd x1, -8(x2)");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("x1"), mem("x2", -8)], 0b011).expect("SUT KAT sd -8");
    assert_eq!(sut, want);
}

#[test]
fn encode_store_kat_llvm_mc_max_min() {
    let want_max = 0x7e11_3fa3u32;
    let mc = llvm_mc_word("sd x1, 2047(x2)").expect("llvm-mc KAT max");
    assert_eq!(mc, want_max, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("x1"), mem("x2", 2047)], 0b011).expect("SUT KAT max");
    assert_eq!(sut, want_max);

    let want_min = 0x8011_3023u32;
    let mc = llvm_mc_word("sd x1, -2048(x2)").expect("llvm-mc KAT min");
    assert_eq!(mc, want_min, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("x1"), mem("x2", -2048)], 0b011).expect("SUT KAT min");
    assert_eq!(sut, want_min);
}

#[test]
fn encode_store_kat_llvm_mc_sh() {
    let want_sh = 0xfe11_1fa3u32;
    assert_eq!(llvm_mc_word("sh x1, -1(x2)").unwrap(), want_sh);
    assert_eq!(sut_word(&[reg("x1"), mem("x2", -1)], 0b001).unwrap(), want_sh);
}

/// Regression: out-of-range store immediate must be rejected
/// (llvm-mc: integer in [-2048, 2047]).
#[test]
fn test_encode_store_regression_imm_oob() {
    let ops = [reg("x1"), mem("x2", 2048)];
    assert!(
        encode_store(&ops, 0b011).is_err(),
        "sd x1, 2048(x2) must Err (imm12 range); got {:?}",
        encode_store(&ops, 0b011)
    );
}

/// Regression: extra operand must be rejected (llvm-mc errors).
#[test]
fn test_encode_store_regression_extra_operand() {
    let ops = [reg("x1"), mem("x2", 0), Operand::Imm(0)];
    assert!(
        encode_store(&ops, 0b011).is_err(),
        "sd x1, 0(x2) with a third operand must Err; got {:?}",
        encode_store(&ops, 0b011)
    );
}

/// Regression: %hi on a store mem operand must be rejected
/// (llvm-mc only allows %lo/%pcrel_lo/%tprel_lo).
#[test]
fn test_encode_store_regression_hi_modifier() {
    let ops = [reg("x1"), Operand::MemSymbol {
        base: "x2".into(),
        symbol: "%hi(foo)".into(),
        modifier: String::new(),
    }];
    assert!(
        encode_store(&ops, 0b011).is_err(),
        "sd x1, %hi(foo)(x2) must Err; got {:?}",
        encode_store(&ops, 0b011)
    );
}

/// Regression: GOT/TLS/plain MemSymbol modifiers must be rejected
/// (llvm-mc only allows %lo/%pcrel_lo/%tprel_lo).
#[test]
fn test_encode_store_regression_other_modifier() {
    let ops = [reg("x1"), Operand::MemSymbol {
        base: "x2".into(),
        symbol: "%got_pcrel_hi(foo)".into(),
        modifier: String::new(),
    }];
    assert!(
        encode_store(&ops, 0b011).is_err(),
        "sd x1, %got_pcrel_hi(foo)(x2) must Err; got {:?}",
        encode_store(&ops, 0b011)
    );
}

/// Regression: store %lo must emit R_RISCV_LO12_S, not the I-type twin.
#[test]
fn test_encode_store_regression_lo_is_s_type() {
    let ops = [reg("x1"), Operand::MemSymbol {
        base: "x2".into(),
        symbol: "%lo(foo)".into(),
        modifier: String::new(),
    }];
    match encode_store(&ops, 0b011) {
        Ok(EncodeResult::WordWithReloc { reloc, .. }) => {
            assert!(
                matches!(reloc.reloc_type, RelocType::Lo12S),
                "sd x1, %lo(foo)(x2) must use Lo12S; got {:?}",
                reloc.reloc_type
            );
        }
        other => panic!("expected WordWithReloc Lo12S; got {other:?}"),
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_store_diff_imm_llvm_mc(
        (mn, f3) in store_mn(),
        rs2 in gpr_name(),
        rs1 in gpr_name(),
        off in i12_imm()
    ) {
        let asm = format!("{} {}, {}({})", mn, rs2, off, rs1);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&[reg(&rs2), mem(&rs1, off)], f3)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "SUT {:08x} != llvm-mc {:08x} for {}", sut, mc, asm);
    }

    #[test]
    fn encode_store_s_type_fields(
        (mn, f3) in store_mn(),
        rs2 in 0u32..=31u32,
        rs1 in 0u32..=31u32,
        off in i12_imm()
    ) {
        let w = sut_word(&[reg(&xn(rs2)), mem(&xn(rs1), off)], f3)
            .unwrap_or_else(|e| panic!("SUT rejected {} x{}, {}(x{}): {}", mn, rs2, off, rs1, e));
        let (opc, got_f3, got_rs1, got_rs2, got_off) = unpack_s(w);
        prop_assert_eq!(opc, OP_STORE, "opcode");
        prop_assert_eq!(got_f3, f3, "funct3");
        prop_assert_eq!(got_rs1, rs1, "rs1");
        prop_assert_eq!(got_rs2, rs2, "rs2");
        prop_assert_eq!(got_off as i64, off, "imm");
    }

    #[test]
    fn encode_store_abi_xn_alias(
        (_mn, f3) in store_mn(),
        n in 0u32..=31u32,
        m in 0u32..=31u32,
        off in i12_imm()
    ) {
        let via_x = sut_word(&[reg(&xn(n)), mem(&xn(m), off)], f3)
            .unwrap_or_else(|e| panic!("xN rejected: {}", e));
        let via_abi = sut_word(&[reg(abi_name(n)), mem(abi_name(m), off)], f3)
            .unwrap_or_else(|e| panic!("ABI rejected: {}", e));
        prop_assert_eq!(via_abi, via_x);
        if n == 8 {
            let via_fp = sut_word(&[reg("fp"), mem(&xn(m), off)], f3)
                .unwrap_or_else(|e| panic!("fp rejected: {}", e));
            prop_assert_eq!(via_fp, via_x);
        }
        if m == 8 {
            let via_fp_b = sut_word(&[reg(&xn(n)), mem("fp", off)], f3)
                .unwrap_or_else(|e| panic!("fp base rejected: {}", e));
            prop_assert_eq!(via_fp_b, via_x);
        }
    }

    #[test]
    fn encode_store_reloc_lo(
        (_mn, f3) in store_mn(),
        rs2 in gpr_name(),
        rs1 in gpr_name(),
        s in ident()
    ) {
        let zero = sut_word(&[reg(&rs2), mem(&rs1, 0)], f3)
            .unwrap_or_else(|e| panic!("SUT rejected store {}, 0({}): {}", rs2, rs1, e));

        let ops_pcrel = [reg(&rs2), Operand::MemSymbol {
            base: rs1.clone(),
            symbol: format!("%pcrel_lo({})", s),
            modifier: String::new(),
        }];
        let (word, kind, got_sym, addend) = sut_reloc(&ops_pcrel, f3)
            .unwrap_or_else(|e| panic!("SUT rejected %pcrel_lo({})({}): {}", s, rs1, e));
        prop_assert_eq!(word, zero, "reloc-form word must equal store {}, 0({})", rs2, rs1);
        prop_assert_eq!(kind, "PcrelLo12S");
        prop_assert_eq!(got_sym, s.clone(), "reloc symbol must be the identifier");
        prop_assert_eq!(addend, 0i64);

        let ops_lo = [reg(&rs2), Operand::MemSymbol {
            base: rs1.clone(),
            symbol: format!("%lo({})", s),
            modifier: String::new(),
        }];
        let (word_lo, kind_lo, got_lo, add_lo) = sut_reloc(&ops_lo, f3)
            .unwrap_or_else(|e| panic!("SUT rejected %lo({})({}): {}", s, rs1, e));
        prop_assert_eq!(word_lo, zero);
        prop_assert_eq!(kind_lo, "Lo12S");
        prop_assert_eq!(got_lo, s.clone());
        prop_assert_eq!(add_lo, 0i64);

        let ops_tp = [reg(&rs2), Operand::MemSymbol {
            base: rs1.clone(),
            symbol: format!("%tprel_lo({})", s),
            modifier: String::new(),
        }];
        let (word_tp, kind_tp, got_tp, add_tp) = sut_reloc(&ops_tp, f3)
            .unwrap_or_else(|e| panic!("SUT rejected %tprel_lo({})({}): {}", s, rs1, e));
        prop_assert_eq!(word_tp, zero);
        prop_assert_eq!(kind_tp, "TprelLo12S");
        prop_assert_eq!(got_tp, s);
        prop_assert_eq!(add_tp, 0i64);
    }

    #[test]
    fn encode_store_neg_imm_oob(
        (mn, f3) in store_mn(),
        rs2 in gpr_name(),
        rs1 in gpr_name(),
        imm in oob_i12()
    ) {
        prop_assume!(!(-2048..=2047).contains(&imm));
        let asm = format!("{} {}, {}({})", mn, rs2, imm, rs1);
        prop_assert!(
            llvm_mc_rejects(&asm),
            "reference unexpectedly accepted {}",
            asm
        );
        let ops = [reg(&rs2), mem(&rs1, imm)];
        prop_assert!(
            encode_store(&ops, f3).is_err(),
            "oob imm {} must Err (llvm-mc range [-2048, 2047]); got {:?}",
            imm,
            encode_store(&ops, f3)
        );
    }

    #[test]
    fn encode_store_neg_extra(
        (mn, f3) in store_mn(),
        rs2 in gpr_name(),
        rs1 in gpr_name(),
        off in i12_imm(),
        extra in extra_operand()
    ) {
        let asm = format!("{} {}, {}({})", mn, rs2, off, rs1);
        let ops = vec![reg(&rs2), mem(&rs1, off), extra];
        prop_assert!(
            encode_store(&ops, f3).is_err(),
            "extra operand must Err for {} (llvm-mc rejects extra operands); got {:?}",
            asm,
            encode_store(&ops, f3)
        );
    }

    #[test]
    fn encode_store_neg_arity_fp(
        (_mn, f3) in store_mn(),
        rs2 in gpr_name(),
        rs1 in gpr_name(),
        off in i12_imm(),
        fp in fp_name(),
        bad in bad_2nd()
    ) {
        prop_assert!(
            encode_store(&[], f3).is_err(),
            "empty operand list must Err; got {:?}",
            encode_store(&[], f3)
        );
        prop_assert!(
            encode_store(&[reg(&rs2)], f3).is_err(),
            "missing memory operand must Err; got {:?}",
            encode_store(&[reg(&rs2)], f3)
        );
        let fp_rs2 = [reg(&fp), mem(&rs1, off)];
        prop_assert!(
            encode_store(&fp_rs2, f3).is_err(),
            "FP source {} must Err (not a GPR); got {:?}",
            fp,
            encode_store(&fp_rs2, f3)
        );
        let fp_base = [reg(&rs2), mem(&fp, off)];
        prop_assert!(
            encode_store(&fp_base, f3).is_err(),
            "FP base {} must Err (not a GPR); got {:?}",
            fp,
            encode_store(&fp_base, f3)
        );
        let bad_ops = [reg(&rs2), bad];
        prop_assert!(
            encode_store(&bad_ops, f3).is_err(),
            "non-memory 2nd operand must Err; got {:?}",
            encode_store(&bad_ops, f3)
        );
    }

    #[test]
    fn encode_store_neg_hi_modifier(
        (mn, f3) in store_mn(),
        rs2 in gpr_name(),
        rs1 in gpr_name(),
        s in ident(),
        hi in hi_modifier()
    ) {
        let inner = format!("{}({})", hi, s);
        let asm = format!("{} {}, {}({})", mn, rs2, inner, rs1);
        prop_assert!(
            llvm_mc_rejects(&asm),
            "reference unexpectedly accepted {}",
            asm
        );
        let ops = [reg(&rs2), Operand::MemSymbol {
            base: rs1.clone(),
            symbol: inner.clone(),
            modifier: String::new(),
        }];
        prop_assert!(
            encode_store(&ops, f3).is_err(),
            "hi-type modifier {} must Err on store (llvm-mc only allows %lo/%pcrel_lo/%tprel_lo); got {:?}",
            inner,
            encode_store(&ops, f3)
        );
    }

    #[test]
    fn encode_store_neg_other_modifier(
        (mn, f3) in store_mn(),
        rs2 in gpr_name(),
        rs1 in gpr_name(),
        form in ident().prop_flat_map(|s| other_modifier_form(&s))
    ) {
        let asm = format!("{} {}, {}({})", mn, rs2, form, rs1);
        prop_assert!(
            llvm_mc_rejects(&asm),
            "reference unexpectedly accepted {}",
            asm
        );
        let ops = [reg(&rs2), Operand::MemSymbol {
            base: rs1.clone(),
            symbol: form.clone(),
            modifier: String::new(),
        }];
        prop_assert!(
            encode_store(&ops, f3).is_err(),
            "non-lo modifier {} must Err on store; got {:?}",
            form,
            encode_store(&ops, f3)
        );
    }
}
