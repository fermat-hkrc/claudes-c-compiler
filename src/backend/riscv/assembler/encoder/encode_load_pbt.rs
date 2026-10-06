// Oracle: differential — llvm-mc RISC-V assembler (I-type LOAD word)
// Evidence: src/backend/riscv/assembler/README.md:6-7 textual assembly from codegen;
//   README.md:300-301 I-type includes lb/lh/lw/ld/lbu/lhu/lwu;
//   README.md:353 I-type: imm[11:0] | rs1 | funct3 | rd | opcode;
//   README.md:387 R_RISCV_PCREL_LO12_I/S: patches load/store lower 12 bits;
//   encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words";
//   encoder/mod.rs:289 "I-type: imm[31:20] | rs1[19:15] | funct3[14:12] | rd[11:7] | opcode[6:0]";
//   encoder/mod.rs:336 OP_LOAD = 0b0000011;
//   encoder/mod.rs:67 "R_RISCV_PCREL_LO12_I - for ADDI/LW/LD (low 12 bits of PC-relative, I-type)";
//   encoder/mod.rs:73 "R_RISCV_LO12_I - for ADDI/LW/LD (absolute low 12 bits, I-type)";
//   encoder/mod.rs:478-484 lb/lh/lw/ld/lbu/lhu/lwu => encode_load;
//   base.rs:157 "Use Lo12I for load-type relocations";
//   base.rs:173-175 bare symbol ld rd, symbol → auipc + ld;
//   base.rs:190 "load: expected memory operand";
//   RISC-V Unprivileged ISA LOAD: opcode=0000011, funct3 by width, rd, rs1, imm[11:0].
// Stronger considered:
//   - State machine: rejected — encode_load is a pure function with no lifecycle
//   - Algebraic round-trip via in-tree decoder: rejected — no I-type load decoder
//   - encode_i / encode_float_load / C.LW / encode_store as differential sibling: rejected —
//     same-job gate (private packer / FP load / compressed / stores)
// Weaker available: algebraic.invariant (I-type field unpack), algebraic.metamorphic
//   (ABI vs xN alias; Symbol vs Label expansion), negative_error (oob imm / extra /
//   FP dest-base / empty / SymbolOffset)
// Differential: candidate=encode_load, reference=llvm-mc -triple=riscv64 -show-encoding,
//   SUT-boundary=internal-helper of RISC-V assembler,
//   mapping=[Reg(rd), Mem{base, offset}] <-> `mn rd, offset(rs1)`;
//   reloc-form word (imm=0) <-> `mn rd, 0(rs1)` plus R_RISCV_LO12_I / PCREL_LO12_I / TPREL_LO12_I;
//   bare Symbol/Label <-> `mn rd, s` expanding to auipc+load.

use super::{encode_load, EncodeResult, RelocType};
use crate::backend::riscv::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;
const OP_LOAD: u32 = 0b0000011;

const ABI: [&str; 32] = [
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3",
    "a4", "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11",
    "t3", "t4", "t5", "t6",
];

const LOAD: [(&str, u32); 7] = [
    ("lb", 0b000),
    ("lh", 0b001),
    ("lw", 0b010),
    ("ld", 0b011),
    ("lbu", 0b100),
    ("lhu", 0b101),
    ("lwu", 0b110),
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
    match encode_load(ops, funct3)? {
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

fn load_mn() -> impl Strategy<Value = (&'static str, u32)> {
    prop::sample::select(LOAD.to_vec())
}

/// Signed 12-bit load immediate, with bounds pinned.
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
        other => panic!("unexpected reloc {other:?}"),
    }
}

fn sut_reloc(ops: &[Operand], funct3: u32) -> Result<(u32, &'static str, String, i64), String> {
    match encode_load(ops, funct3)? {
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
    ]
}

fn hi_modifier() -> impl Strategy<Value = &'static str> {
    prop_oneof![
        Just("%hi"),
        Just("%pcrel_hi"),
        Just("%tprel_hi"),
    ]
}

fn signed_addend() -> impl Strategy<Value = i64> {
    prop_oneof![
        Just(1i64),
        Just(-1i64),
        Just(4i64),
        Just(-4i64),
        Just(8i64),
        Just(2047i64),
        Just(-2048i64),
        1i64..=4096i64,
        -4096i64..=-1i64,
    ]
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_load_kat_llvm_mc_ld_x1_x2() {
    let want = 0x0001_3083u32;
    let mc = llvm_mc_word("ld x1, 0(x2)").expect("llvm-mc KAT ld x1, 0(x2)");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("x1"), mem("x2", 0)], 0b011).expect("SUT KAT ld x1, 0(x2)");
    assert_eq!(sut, want);
}

#[test]
fn encode_load_kat_llvm_mc_lb_x0_x1() {
    let want = 0x0000_8003u32;
    let mc = llvm_mc_word("lb x0, 0(x1)").expect("llvm-mc KAT lb x0, 0(x1)");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("x0"), mem("x1", 0)], 0b000).expect("SUT KAT lb");
    assert_eq!(sut, want);
}

#[test]
fn encode_load_kat_llvm_mc_lw_off8() {
    let want = 0x0081_2083u32;
    let mc = llvm_mc_word("lw x1, 8(x2)").expect("llvm-mc KAT lw x1, 8(x2)");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("x1"), mem("x2", 8)], 0b010).expect("SUT KAT lw");
    assert_eq!(sut, want);
}

#[test]
fn encode_load_kat_llvm_mc_ld_neg8() {
    let want = 0xff81_3083u32;
    let mc = llvm_mc_word("ld x1, -8(x2)").expect("llvm-mc KAT ld x1, -8(x2)");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("x1"), mem("x2", -8)], 0b011).expect("SUT KAT ld -8");
    assert_eq!(sut, want);
}

#[test]
fn encode_load_kat_llvm_mc_max_min() {
    let want_max = 0x7ff1_3083u32;
    let mc = llvm_mc_word("ld x1, 2047(x2)").expect("llvm-mc KAT max");
    assert_eq!(mc, want_max, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("x1"), mem("x2", 2047)], 0b011).expect("SUT KAT max");
    assert_eq!(sut, want_max);

    let want_min = 0x8001_3083u32;
    let mc = llvm_mc_word("ld x1, -2048(x2)").expect("llvm-mc KAT min");
    assert_eq!(mc, want_min, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("x1"), mem("x2", -2048)], 0b011).expect("SUT KAT min");
    assert_eq!(sut, want_min);
}

#[test]
fn encode_load_kat_llvm_mc_lbu_lhu_lwu_lh() {
    let want_lbu = 0x0011_4083u32;
    assert_eq!(llvm_mc_word("lbu x1, 1(x2)").unwrap(), want_lbu);
    assert_eq!(sut_word(&[reg("x1"), mem("x2", 1)], 0b100).unwrap(), want_lbu);

    let want_lhu = 0x0021_5083u32;
    assert_eq!(llvm_mc_word("lhu x1, 2(x2)").unwrap(), want_lhu);
    assert_eq!(sut_word(&[reg("x1"), mem("x2", 2)], 0b101).unwrap(), want_lhu);

    let want_lwu = 0x0041_6083u32;
    assert_eq!(llvm_mc_word("lwu x1, 4(x2)").unwrap(), want_lwu);
    assert_eq!(sut_word(&[reg("x1"), mem("x2", 4)], 0b110).unwrap(), want_lwu);

    let want_lh = 0xfff1_1083u32;
    assert_eq!(llvm_mc_word("lh x1, -1(x2)").unwrap(), want_lh);
    assert_eq!(sut_word(&[reg("x1"), mem("x2", -1)], 0b001).unwrap(), want_lh);
}

#[test]
fn encode_load_kat_bare_symbol_auipc_ld() {
    let auipc = llvm_mc_word("auipc x1, 0").expect("llvm-mc auipc x1, 0");
    let ld = llvm_mc_word("ld x1, 0(x1)").expect("llvm-mc ld x1, 0(x1)");
    match encode_load(&[reg("x1"), Operand::Symbol("foo".into())], 0b011) {
        Ok(EncodeResult::WordsWithRelocs(v)) => {
            assert_eq!(v.len(), 2, "bare symbol must expand to two words");
            assert_eq!(v[0].0, auipc);
            match &v[0].1 {
                Some(r) => {
                    assert!(matches!(r.reloc_type, RelocType::PcrelHi20));
                    assert_eq!(r.symbol, "foo");
                    assert_eq!(r.addend, 0);
                }
                None => panic!("expected PcrelHi20 on auipc"),
            }
            assert_eq!(v[1].0, ld);
            match &v[1].1 {
                Some(r) => {
                    assert!(matches!(r.reloc_type, RelocType::PcrelLo12I));
                    assert_eq!(r.symbol, "foo");
                    assert_eq!(r.addend, 0);
                }
                None => panic!("expected PcrelLo12I on ld"),
            }
        }
        other => panic!("expected WordsWithRelocs, got {other:?}"),
    }
}

/// Regression: out-of-range load immediate must be rejected
/// (llvm-mc: integer in [-2048, 2047]).
#[test]
fn test_encode_load_regression_imm_oob() {
    let ops = [reg("x1"), mem("x2", 2048)];
    assert!(
        encode_load(&ops, 0b011).is_err(),
        "ld x1, 2048(x2) must Err (imm12 range); got {:?}",
        encode_load(&ops, 0b011)
    );
}

/// Regression: extra operand must be rejected (llvm-mc errors).
#[test]
fn test_encode_load_regression_hi_modifier() {
    let ops = [reg("x1"), Operand::MemSymbol {
        base: "x2".into(),
        symbol: "%hi(foo)".into(),
        modifier: String::new(),
    }];
    assert!(
        encode_load(&ops, 0b011).is_err(),
        "ld x1, %hi(foo)(x2) must Err (llvm-mc only allows %lo/%pcrel_lo/%tprel_lo); got {:?}",
        encode_load(&ops, 0b011)
    );
}

/// Regression: ld rd, symbol+addend must expand like the bare-symbol pseudo with addend preserved.
#[test]
fn test_encode_load_regression_symbol_offset() {
    let ops = [reg("x1"), Operand::SymbolOffset("foo".into(), 4)];
    match encode_load(&ops, 0b011) {
        Ok(EncodeResult::WordsWithRelocs(v)) => {
            assert_eq!(v.len(), 2);
            match &v[0].1 {
                Some(r) => {
                    assert!(matches!(r.reloc_type, RelocType::PcrelHi20));
                    assert_eq!(r.symbol, "foo");
                    assert_eq!(r.addend, 4, "addend must be preserved on AUIPC");
                }
                None => panic!("expected PcrelHi20"),
            }
        }
        other => panic!("ld x1, foo+4 must expand to WordsWithRelocs; got {:?}", other),
    }
}

/// Regression: extra operand must be rejected (llvm-mc errors).
#[test]
fn test_encode_load_regression_extra_operand() {
    let ops = [reg("x1"), mem("x2", 0), Operand::Imm(0)];
    assert!(
        encode_load(&ops, 0b011).is_err(),
        "ld x1, 0(x2) with a third operand must Err; got {:?}",
        encode_load(&ops, 0b011)
    );
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_load_diff_imm_llvm_mc(
        (mn, f3) in load_mn(),
        rd in gpr_name(),
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
    fn encode_load_i_type_fields(
        (mn, f3) in load_mn(),
        rd in 0u32..=31u32,
        rs1 in 0u32..=31u32,
        off in i12_imm()
    ) {
        let w = sut_word(&[reg(&xn(rd)), mem(&xn(rs1), off)], f3)
            .unwrap_or_else(|e| panic!("SUT rejected {} x{}, {}(x{}): {}", mn, rd, off, rs1, e));
        let (opc, got_rd, got_f3, got_rs1, got_off) = unpack_i(w);
        prop_assert_eq!(opc, OP_LOAD, "opcode");
        prop_assert_eq!(got_rd, rd, "rd");
        prop_assert_eq!(got_f3, f3, "funct3");
        prop_assert_eq!(got_rs1, rs1, "rs1");
        prop_assert_eq!(got_off as i64, off, "imm");
    }

    #[test]
    fn encode_load_abi_xn_alias(
        (_mn, f3) in load_mn(),
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
    fn encode_load_reloc_lo(
        (_mn, f3) in load_mn(),
        rd in gpr_name(),
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
    fn encode_load_bare_symbol_pseudo(
        (mn, f3) in load_mn(),
        rd in gpr_name(),
        s in ident()
    ) {
        let auipc = llvm_mc_word(&format!("auipc {}, 0", rd))
            .unwrap_or_else(|e| panic!("llvm-mc rejected auipc {}, 0: {}", rd, e));
        let load0 = llvm_mc_word(&format!("{} {}, 0({})", mn, rd, rd))
            .unwrap_or_else(|e| panic!("llvm-mc rejected {} {}, 0({}): {}", mn, rd, rd, e));

        match encode_load(&[reg(&rd), Operand::Symbol(s.clone())], f3) {
            Ok(EncodeResult::WordsWithRelocs(v)) => {
                prop_assert_eq!(v.len(), 2usize, "bare symbol must expand to two words");
                prop_assert_eq!(v[0].0, auipc, "auipc word");
                match &v[0].1 {
                    Some(r) => {
                        prop_assert_eq!(reloc_kind(&r.reloc_type), "PcrelHi20");
                        prop_assert_eq!(&r.symbol, &s);
                        prop_assert_eq!(r.addend, 0i64);
                    }
                    None => prop_assert!(false, "expected PcrelHi20 on auipc"),
                }
                prop_assert_eq!(v[1].0, load0, "load word");
                match &v[1].1 {
                    Some(r) => {
                        prop_assert_eq!(reloc_kind(&r.reloc_type), "PcrelLo12I");
                        prop_assert_eq!(&r.symbol, &s);
                        prop_assert_eq!(r.addend, 0i64);
                    }
                    None => prop_assert!(false, "expected PcrelLo12I on load"),
                }
            }
            other => {
                return Err(proptest::test_runner::TestCaseError::fail(format!(
                    "expected WordsWithRelocs for {} {}, {}, got {:?}",
                    mn, rd, s, other
                )));
            }
        }

        match encode_load(&[reg(&rd), Operand::Label(s.clone())], f3) {
            Ok(EncodeResult::WordsWithRelocs(v)) => {
                prop_assert_eq!(v.len(), 2usize);
                prop_assert_eq!(v[0].0, auipc);
                prop_assert_eq!(v[1].0, load0);
                match &v[0].1 {
                    Some(r) => {
                        prop_assert_eq!(reloc_kind(&r.reloc_type), "PcrelHi20");
                        prop_assert_eq!(&r.symbol, &s);
                    }
                    None => prop_assert!(false, "expected PcrelHi20 on auipc (Label)"),
                }
                match &v[1].1 {
                    Some(r) => {
                        prop_assert_eq!(reloc_kind(&r.reloc_type), "PcrelLo12I");
                        prop_assert_eq!(&r.symbol, &s);
                    }
                    None => prop_assert!(false, "expected PcrelLo12I on load (Label)"),
                }
            }
            other => {
                return Err(proptest::test_runner::TestCaseError::fail(format!(
                    "expected WordsWithRelocs for Label {}, got {:?}",
                    s, other
                )));
            }
        }
    }

    #[test]
    fn encode_load_neg_imm_oob(
        (mn, f3) in load_mn(),
        rd in gpr_name(),
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
            encode_load(&ops, f3).is_err(),
            "oob imm {} must Err (llvm-mc range [-2048, 2047]); got {:?}",
            imm,
            encode_load(&ops, f3)
        );
    }

    #[test]
    fn encode_load_neg_extra(
        (mn, f3) in load_mn(),
        rd in gpr_name(),
        rs1 in gpr_name(),
        off in i12_imm(),
        extra in extra_operand()
    ) {
        let asm = format!("{} {}, {}({})", mn, rd, off, rs1);
        let ops = vec![reg(&rd), mem(&rs1, off), extra];
        prop_assert!(
            encode_load(&ops, f3).is_err(),
            "extra operand must Err for {} (llvm-mc rejects extra operands); got {:?}",
            asm,
            encode_load(&ops, f3)
        );
    }

    #[test]
    fn encode_load_neg_arity_fp(
        (_mn, f3) in load_mn(),
        rd in gpr_name(),
        rs1 in gpr_name(),
        off in i12_imm(),
        fp in fp_name(),
        bad in bad_2nd()
    ) {
        prop_assert!(
            encode_load(&[], f3).is_err(),
            "empty operand list must Err; got {:?}",
            encode_load(&[], f3)
        );
        prop_assert!(
            encode_load(&[reg(&rd)], f3).is_err(),
            "missing memory operand must Err; got {:?}",
            encode_load(&[reg(&rd)], f3)
        );
        let fp_rd = [reg(&fp), mem(&rs1, off)];
        prop_assert!(
            encode_load(&fp_rd, f3).is_err(),
            "FP dest {} must Err (not a GPR); got {:?}",
            fp,
            encode_load(&fp_rd, f3)
        );
        let fp_base = [reg(&rd), mem(&fp, off)];
        prop_assert!(
            encode_load(&fp_base, f3).is_err(),
            "FP base {} must Err (not a GPR); got {:?}",
            fp,
            encode_load(&fp_base, f3)
        );
        let bad_ops = [reg(&rd), bad];
        prop_assert!(
            encode_load(&bad_ops, f3).is_err(),
            "non-memory 2nd operand must Err; got {:?}",
            encode_load(&bad_ops, f3)
        );
    }

    #[test]
    fn encode_load_neg_hi_modifier(
        (mn, f3) in load_mn(),
        rd in gpr_name(),
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
            encode_load(&ops, f3).is_err(),
            "hi-type modifier {} must Err on load (llvm-mc only allows %lo/%pcrel_lo/%tprel_lo); got {:?}",
            inner,
            encode_load(&ops, f3)
        );
    }

    #[test]
    fn encode_load_symbol_offset_addend(
        (mn, f3) in load_mn(),
        rd in gpr_name(),
        s in ident(),
        addend in signed_addend()
    ) {
        prop_assume!(addend != 0);
        let auipc = llvm_mc_word(&format!("auipc {}, 0", rd))
            .unwrap_or_else(|e| panic!("llvm-mc rejected auipc {}, 0: {}", rd, e));
        let load0 = llvm_mc_word(&format!("{} {}, 0({})", mn, rd, rd))
            .unwrap_or_else(|e| panic!("llvm-mc rejected {} {}, 0({}): {}", mn, rd, rd, e));
        let ops = [reg(&rd), Operand::SymbolOffset(s.clone(), addend)];
        match encode_load(&ops, f3) {
            Ok(EncodeResult::WordsWithRelocs(v)) => {
                prop_assert_eq!(v.len(), 2usize, "symbol+addend must expand to two words");
                prop_assert_eq!(v[0].0, auipc, "auipc word");
                match &v[0].1 {
                    Some(r) => {
                        prop_assert_eq!(reloc_kind(&r.reloc_type), "PcrelHi20");
                        prop_assert_eq!(&r.symbol, &s, "reloc symbol must be the identifier");
                        prop_assert_eq!(r.addend, addend, "reloc addend must preserve SymbolOffset");
                    }
                    None => prop_assert!(false, "expected PcrelHi20 on auipc"),
                }
                prop_assert_eq!(v[1].0, load0, "load word");
                match &v[1].1 {
                    Some(r) => {
                        prop_assert_eq!(reloc_kind(&r.reloc_type), "PcrelLo12I");
                        prop_assert_eq!(&r.symbol, &s);
                    }
                    None => prop_assert!(false, "expected PcrelLo12I on load"),
                }
            }
            other => {
                return Err(proptest::test_runner::TestCaseError::fail(format!(
                    "expected WordsWithRelocs for {} {}, {}+{}, got {:?}",
                    mn, rd, s, addend, other
                )));
            }
        }
    }
}
