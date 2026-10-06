// Oracle: differential — llvm-mc RISC-V assembler (B-type branch word)
// Evidence: src/backend/riscv/assembler/README.md:6-7 textual assembly from codegen;
//   README.md:303 B-type: beq, bne, blt, bge, bltu, bgeu;
//   README.md:355 B-type: imm[12|10:5] | rs2 | rs1 | funct3 | imm[4:1|11] | opcode;
//   README.md:383 R_RISCV_BRANCH (B-type): 12-bit signed offset, bit-scattered;
//   encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words";
//   encoder/mod.rs:301 "B-type: imm[12|10:5] | rs2 | rs1 | funct3 | imm[4:1|11] | opcode";
//   encoder/mod.rs:333 OP_BRANCH = 0b1100011;
//   encoder/mod.rs:75 "R_RISCV_BRANCH - 12-bit PC-relative branch (B-type)";
//   encoder/mod.rs:468-473 beq/bne/blt/bge/bltu/bgeu => encode_branch_instr;
//   base.rs:143 "branch: expected offset or label as 3rd operand";
//   RISC-V Unprivileged ISA BRANCH: opcode=1100011, even offset in [-4096, 4094].
// Stronger considered:
//   - State machine: rejected — encode_branch_instr is a pure function with no lifecycle
//   - Algebraic round-trip via in-tree decoder: rejected — no B-type decoder
//   - encode_b / beqz / bnez / bgez / bltz / bgt / C.BEQZ as differential sibling: rejected —
//     same-job gate (private packer / zero-compare pseudo / swapped-operand pseudo / compressed)
// Weaker available: algebraic.invariant (B-type field unpack), algebraic.metamorphic
//   (ABI vs xN alias; Symbol vs Label vs Reg reloc), negative_error (oob/odd imm /
//   extra / FP rs / empty / SymbolOffset)
// Differential: candidate=encode_branch_instr, reference=llvm-mc -triple=riscv64 -show-encoding,
//   SUT-boundary=internal-helper of RISC-V assembler,
//   mapping=[Reg(rs1), Reg(rs2), Imm(off)] <-> `mn rs1, rs2, off`;
//   reloc-form word (imm=0) <-> `mn rs1, rs2, 0` plus R_RISCV_BRANCH.

use super::{encode_branch_instr, EncodeResult, RelocType};
use crate::backend::riscv::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;
const OP_BRANCH: u32 = 0b1100011;

const ABI: [&str; 32] = [
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3",
    "a4", "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11",
    "t3", "t4", "t5", "t6",
];

const BRANCH: [(&str, u32); 6] = [
    ("beq", 0b000),
    ("bne", 0b001),
    ("blt", 0b100),
    ("bge", 0b101),
    ("bltu", 0b110),
    ("bgeu", 0b111),
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

/// Unpack B-type per RISC-V unprivileged ISA (not a copy of encode_b).
/// Layout: imm[12|10:5] | rs2 | rs1 | funct3 | imm[4:1|11] | opcode
fn unpack_b(word: u32) -> (u32, u32, u32, u32, i32) {
    let opcode = word & 0x7f;
    let imm11 = (word >> 7) & 1;
    let bits4_1 = (word >> 8) & 0xf;
    let funct3 = (word >> 12) & 7;
    let rs1 = (word >> 15) & 0x1f;
    let rs2 = (word >> 20) & 0x1f;
    let bits10_5 = (word >> 25) & 0x3f;
    let bit12 = (word >> 31) & 1;
    let u13 = (bit12 << 12) | (imm11 << 11) | (bits10_5 << 5) | (bits4_1 << 1);
    let off = ((u13 as i32) << 19) >> 19;
    (opcode, funct3, rs1, rs2, off)
}

fn sut_word(ops: &[Operand], funct3: u32) -> Result<u32, String> {
    match encode_branch_instr(ops, funct3)? {
        EncodeResult::Word(w) => Ok(w),
        other => Err(format!("expected Word, got {other:?}")),
    }
}

fn reloc_kind(t: &RelocType) -> &'static str {
    match t {
        RelocType::Branch => "Branch",
        other => panic!("unexpected reloc {other:?}"),
    }
}

fn sut_reloc(ops: &[Operand], funct3: u32) -> Result<(u32, &'static str, String, i64), String> {
    match encode_branch_instr(ops, funct3)? {
        EncodeResult::WordWithReloc { word, reloc } => {
            Ok((word, reloc_kind(&reloc.reloc_type), reloc.symbol, reloc.addend))
        }
        other => Err(format!("expected WordWithReloc, got {other:?}")),
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

fn branch_mn() -> impl Strategy<Value = (&'static str, u32)> {
    prop::sample::select(BRANCH.to_vec())
}

fn fp_name() -> impl Strategy<Value = String> {
    prop_oneof![
        (0u32..=31).prop_map(|n| format!("f{n}")),
        (0u32..=11).prop_map(|n| format!("ft{n}")),
        (0u32..=11).prop_map(|n| format!("fs{n}")),
        (0u32..=7).prop_map(|n| format!("fa{n}")),
    ]
}

/// Even B-type offset in [-4096, 4094], bounds pinned.
fn even_b_imm() -> impl Strategy<Value = i64> {
    prop_oneof![
        Just(0i64),
        Just(2i64),
        Just(-2i64),
        Just(4i64),
        Just(-4i64),
        Just(8i64),
        Just(-8i64),
        Just(4094i64),
        Just(-4096i64),
        (-2048i32..=2047).prop_map(|h| (h as i64) * 2),
    ]
}

fn oob_or_odd_b_imm() -> impl Strategy<Value = i64> {
    prop_oneof![
        Just(1i64),
        Just(-1i64),
        Just(3i64),
        Just(-3i64),
        Just(4095i64),
        Just(-4095i64),
        Just(4096i64),
        Just(-4097i64),
        Just(-4098i64),
        Just(4098i64),
        Just(i64::MIN),
        Just(i64::MAX),
        (0i64..=4094).prop_map(|x| if x % 2 == 0 { x + 1 } else { x }),
        4096i64..=i64::MAX,
        i64::MIN..=-4097,
    ]
}

fn invalid_3rd() -> impl Strategy<Value = Operand> {
    prop_oneof![
        Just(Operand::Mem {
            base: "sp".into(),
            offset: 8,
        }),
        Just(Operand::Mem {
            base: "x1".into(),
            offset: 0,
        }),
        Just(Operand::MemSymbol {
            base: "sp".into(),
            symbol: "foo".into(),
            modifier: "%lo".into(),
        }),
        Just(Operand::FenceArg("iorw".into())),
        Just(Operand::Csr("mstatus".into())),
        Just(Operand::RoundingMode("rne".into())),
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
        Just("L0".into()),
        Just(".LBB0_1".into()),
        Just("my_label".into()),
        Just("_start".into()),
        Just("loop_top".into()),
    ]
}

fn signed_addend() -> impl Strategy<Value = i64> {
    prop_oneof![
        Just(1i64),
        Just(-1i64),
        Just(4i64),
        Just(-4i64),
        Just(8i64),
        Just(-8i64),
        Just(4096i64),
        -4096i64..=4096,
    ]
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_branch_instr_kat_llvm_mc_beq_x0_x1_0() {
    let want = 0x0010_0063u32;
    let mc = llvm_mc_word("beq x0, x1, 0").expect("llvm-mc KAT beq x0,x1,0");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("x0"), reg("x1"), Operand::Imm(0)], 0b000)
        .expect("SUT KAT beq x0,x1,0");
    assert_eq!(sut, want);
}

#[test]
fn encode_branch_instr_kat_llvm_mc_bne_x0_x1_0() {
    let want = 0x0010_1063u32;
    let mc = llvm_mc_word("bne x0, x1, 0").expect("llvm-mc KAT bne");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("x0"), reg("x1"), Operand::Imm(0)], 0b001).expect("SUT KAT bne");
    assert_eq!(sut, want);
}

#[test]
fn encode_branch_instr_kat_llvm_mc_blt_x0_x1_0() {
    let want = 0x0010_4063u32;
    let mc = llvm_mc_word("blt x0, x1, 0").expect("llvm-mc KAT blt");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("x0"), reg("x1"), Operand::Imm(0)], 0b100).expect("SUT KAT blt");
    assert_eq!(sut, want);
}

#[test]
fn encode_branch_instr_kat_llvm_mc_bge_x0_x1_0() {
    let want = 0x0010_5063u32;
    let mc = llvm_mc_word("bge x0, x1, 0").expect("llvm-mc KAT bge");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("x0"), reg("x1"), Operand::Imm(0)], 0b101).expect("SUT KAT bge");
    assert_eq!(sut, want);
}

#[test]
fn encode_branch_instr_kat_llvm_mc_bltu_x0_x1_0() {
    let want = 0x0010_6063u32;
    let mc = llvm_mc_word("bltu x0, x1, 0").expect("llvm-mc KAT bltu");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("x0"), reg("x1"), Operand::Imm(0)], 0b110).expect("SUT KAT bltu");
    assert_eq!(sut, want);
}

#[test]
fn encode_branch_instr_kat_llvm_mc_bgeu_x0_x1_0() {
    let want = 0x0010_7063u32;
    let mc = llvm_mc_word("bgeu x0, x1, 0").expect("llvm-mc KAT bgeu");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("x0"), reg("x1"), Operand::Imm(0)], 0b111).expect("SUT KAT bgeu");
    assert_eq!(sut, want);
}

#[test]
fn encode_branch_instr_kat_llvm_mc_bounds() {
    let want_pos = 0x0020_8263u32; // beq x1, x2, 4
    let mc = llvm_mc_word("beq x1, x2, 4").expect("llvm-mc KAT +4");
    assert_eq!(mc, want_pos, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("x1"), reg("x2"), Operand::Imm(4)], 0b000).expect("SUT +4");
    assert_eq!(sut, want_pos);

    let want_neg = 0xfe20_9ee3u32; // bne x1, x2, -4
    let mc = llvm_mc_word("bne x1, x2, -4").expect("llvm-mc KAT -4");
    assert_eq!(mc, want_neg, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("x1"), reg("x2"), Operand::Imm(-4)], 0b001).expect("SUT -4");
    assert_eq!(sut, want_neg);

    let want_max = 0x7e20_8fe3u32; // beq x1, x2, 4094
    let mc = llvm_mc_word("beq x1, x2, 4094").expect("llvm-mc KAT max");
    assert_eq!(mc, want_max, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("x1"), reg("x2"), Operand::Imm(4094)], 0b000).expect("SUT max");
    assert_eq!(sut, want_max);

    let want_min = 0x8020_8063u32; // beq x1, x2, -4096
    let mc = llvm_mc_word("beq x1, x2, -4096").expect("llvm-mc KAT min");
    assert_eq!(mc, want_min, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("x1"), reg("x2"), Operand::Imm(-4096)], 0b000).expect("SUT min");
    assert_eq!(sut, want_min);
}

#[test]
fn encode_branch_instr_kat_reloc_foo() {
    let want = 0x0020_8063u32; // beq x1, x2, 0
    let mc = llvm_mc_word("beq x1, x2, 0").expect("llvm-mc zero-imm word");
    assert_eq!(mc, want);
    let (word, kind, sym, addend) =
        sut_reloc(&[reg("x1"), reg("x2"), Operand::Symbol("foo".into())], 0b000)
            .expect("SUT symbol KAT");
    assert_eq!(word, want);
    assert_eq!(kind, "Branch");
    assert_eq!(sym, "foo");
    assert_eq!(addend, 0);
}

/// Regression: odd / out-of-range B-type immediate must be rejected
/// (llvm-mc: multiple of 2 in [-4096, 4094]).
#[test]
fn test_encode_branch_instr_regression_imm_oob() {
    let ops = [reg("x0"), reg("x1"), Operand::Imm(1)];
    assert!(
        encode_branch_instr(&ops, 0b000).is_err(),
        "beq x0, x1, 1 must Err (odd offset); got {:?}",
        encode_branch_instr(&ops, 0b000)
    );
}

/// Regression: extra operand must be rejected (B-type is three-operand; llvm-mc errors).
#[test]
fn test_encode_branch_instr_regression_extra_operand() {
    let ops = [reg("x0"), reg("x1"), Operand::Imm(0), Operand::Imm(0)];
    assert!(
        encode_branch_instr(&ops, 0b000).is_err(),
        "beq x0, x1, 0 with a fourth operand must Err; got {:?}",
        encode_branch_instr(&ops, 0b000)
    );
}

/// Regression: SymbolOffset must relocate against the symbol with that addend.
#[test]
fn test_encode_branch_instr_regression_symbol_offset() {
    let ops = [reg("x0"), reg("x1"), Operand::SymbolOffset("foo".into(), 4)];
    match encode_branch_instr(&ops, 0b000) {
        Ok(EncodeResult::WordWithReloc { reloc, .. }) => {
            assert_eq!(
                reloc.symbol, "foo",
                "beq x0, x1, foo+4 symbol must be foo, not {}",
                reloc.symbol
            );
            assert_eq!(
                reloc.addend, 4,
                "beq x0, x1, foo+4 addend must be 4, not {}",
                reloc.addend
            );
        }
        other => panic!("expected WordWithReloc for foo+4, got {:?}", other),
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_branch_instr_diff_imm_llvm_mc(
        (mn, funct3) in branch_mn(),
        rs1 in gpr_name(),
        rs2 in gpr_name(),
        off in even_b_imm()
    ) {
        let asm = format!("{mn} {rs1}, {rs2}, {off}");
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {asm}: {e}"));
        let sut = sut_word(&[reg(&rs1), reg(&rs2), Operand::Imm(off)], funct3)
            .unwrap_or_else(|e| panic!("SUT rejected valid {asm}: {e}"));
        prop_assert_eq!(sut, mc, "SUT {:08x} != llvm-mc {:08x} for {}", sut, mc, asm);
    }

    #[test]
    fn encode_branch_instr_isa_b_type(
        (mn, funct3) in branch_mn(),
        rs1 in 0u32..=31u32,
        rs2 in 0u32..=31u32,
        off in even_b_imm()
    ) {
        let w = sut_word(&[reg(&xn(rs1)), reg(&xn(rs2)), Operand::Imm(off)], funct3)
            .unwrap_or_else(|e| panic!("SUT rejected {mn} x{rs1}, x{rs2}, {off}: {e}"));
        let (opc, got_f3, got_rs1, got_rs2, got_off) = unpack_b(w);
        prop_assert_eq!(opc, OP_BRANCH, "opcode");
        prop_assert_eq!(got_f3, funct3, "funct3");
        prop_assert_eq!(got_rs1, rs1, "rs1");
        prop_assert_eq!(got_rs2, rs2, "rs2");
        prop_assert_eq!(got_off as i64, off, "imm");
    }

    #[test]
    fn encode_branch_instr_abi_xn_alias(
        n in 0u32..=31u32,
        m in 0u32..=31u32,
        off in even_b_imm()
    ) {
        let via_x = sut_word(&[reg(&xn(n)), reg(&xn(m)), Operand::Imm(off)], 0b000)
            .unwrap_or_else(|e| panic!("xN rejected: {e}"));
        let via_abi = sut_word(&[reg(abi_name(n)), reg(abi_name(m)), Operand::Imm(off)], 0b000)
            .unwrap_or_else(|e| panic!("ABI rejected: {e}"));
        prop_assert_eq!(via_abi, via_x);
        if n == 8 {
            let via_fp = sut_word(&[reg("fp"), reg(&xn(m)), Operand::Imm(off)], 0b000)
                .unwrap_or_else(|e| panic!("fp rejected: {e}"));
            prop_assert_eq!(via_fp, via_x);
        }
        if m == 8 {
            let via_fp = sut_word(&[reg(&xn(n)), reg("fp"), Operand::Imm(off)], 0b000)
                .unwrap_or_else(|e| panic!("fp rs2 rejected: {e}"));
            prop_assert_eq!(via_fp, via_x);
        }
    }

    #[test]
    fn encode_branch_instr_reloc_symbol(
        (mn, funct3) in branch_mn(),
        rs1 in gpr_name(),
        rs2 in gpr_name(),
        s in ident()
    ) {
        let zero_asm = format!("{mn} {rs1}, {rs2}, 0");
        let mc = llvm_mc_word(&zero_asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected {zero_asm}: {e}"));
        let ops = [reg(&rs1), reg(&rs2), Operand::Symbol(s.clone())];
        let (word, kind, got_sym, addend) = sut_reloc(&ops, funct3)
            .unwrap_or_else(|e| panic!("SUT rejected {mn} {rs1}, {rs2}, {s}: {e}"));
        prop_assert_eq!(word, mc);
        prop_assert_eq!(kind, "Branch");
        prop_assert_eq!(got_sym, s.clone());
        prop_assert_eq!(addend, 0i64);

        let ops_l = [reg(&rs1), reg(&rs2), Operand::Label(s.clone())];
        let (word_l, kind_l, got_l, add_l) = sut_reloc(&ops_l, funct3)
            .unwrap_or_else(|e| panic!("SUT rejected {mn} Label({s}): {e}"));
        prop_assert_eq!(word_l, mc);
        prop_assert_eq!(kind_l, "Branch");
        prop_assert_eq!(got_l, s.clone());
        prop_assert_eq!(add_l, 0i64);

        let ops_r = [reg(&rs1), reg(&rs2), Operand::Reg(s.clone())];
        let (word_r, kind_r, got_r, add_r) = sut_reloc(&ops_r, funct3)
            .unwrap_or_else(|e| panic!("SUT rejected {mn} Reg({s}) as label: {e}"));
        prop_assert_eq!(word_r, mc);
        prop_assert_eq!(kind_r, "Branch");
        prop_assert_eq!(got_r, s);
        prop_assert_eq!(add_r, 0i64);
    }

    #[test]
    fn encode_branch_instr_neg_imm_oob_odd(
        rs1 in gpr_name(),
        rs2 in gpr_name(),
        imm in oob_or_odd_b_imm()
    ) {
        prop_assume!(imm % 2 != 0 || !(-4096..=4094).contains(&imm));
        let asm = format!("beq {rs1}, {rs2}, {imm}");
        prop_assert!(
            llvm_mc_rejects(&asm),
            "reference unexpectedly accepted {asm}"
        );
        let ops = [reg(&rs1), reg(&rs2), Operand::Imm(imm)];
        prop_assert!(
            encode_branch_instr(&ops, 0b000).is_err(),
            "oob/odd imm {imm} must Err (llvm-mc range even [-4096, 4094]); got {:?}",
            encode_branch_instr(&ops, 0b000)
        );
    }

    #[test]
    fn encode_branch_instr_neg_extra(
        rs1 in gpr_name(),
        rs2 in gpr_name(),
        off in even_b_imm(),
        extra in extra_operand()
    ) {
        let asm = format!("beq {rs1}, {rs2}, {off}");
        let ops = vec![reg(&rs1), reg(&rs2), Operand::Imm(off), extra];
        prop_assert!(
            encode_branch_instr(&ops, 0b000).is_err(),
            "extra operand must Err for {asm} (llvm-mc rejects extra operands); got {:?}",
            encode_branch_instr(&ops, 0b000)
        );
    }

    #[test]
    fn encode_branch_instr_neg_arity_fp(
        fp in fp_name(),
        rs in gpr_name(),
        off in even_b_imm()
    ) {
        prop_assert!(
            encode_branch_instr(&[], 0b000).is_err(),
            "empty operand list must Err; got {:?}",
            encode_branch_instr(&[], 0b000)
        );
        let two = [reg(&rs), reg(&rs)];
        prop_assert!(
            encode_branch_instr(&two, 0b000).is_err(),
            "missing 3rd operand must Err; got {:?}",
            encode_branch_instr(&two, 0b000)
        );
        let fp_rs1 = [reg(&fp), reg(&rs), Operand::Imm(off)];
        prop_assert!(
            encode_branch_instr(&fp_rs1, 0b000).is_err(),
            "FP rs1 {fp} must Err (not a GPR); got {:?}",
            encode_branch_instr(&fp_rs1, 0b000)
        );
        let fp_rs2 = [reg(&rs), reg(&fp), Operand::Imm(off)];
        prop_assert!(
            encode_branch_instr(&fp_rs2, 0b000).is_err(),
            "FP rs2 {fp} must Err (not a GPR); got {:?}",
            encode_branch_instr(&fp_rs2, 0b000)
        );
    }

    #[test]
    fn encode_branch_instr_symbol_offset_addend(
        rs1 in gpr_name(),
        rs2 in gpr_name(),
        s in ident(),
        addend in signed_addend()
    ) {
        prop_assume!(addend != 0);
        let zero_asm = format!("beq {rs1}, {rs2}, 0");
        let mc = llvm_mc_word(&zero_asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected {zero_asm}: {e}"));
        let ops = [reg(&rs1), reg(&rs2), Operand::SymbolOffset(s.clone(), addend)];
        let (word, kind, got_sym, got_add) = sut_reloc(&ops, 0b000)
            .unwrap_or_else(|e| panic!("SUT rejected beq {rs1}, {rs2}, {s}+{addend}: {e}"));
        prop_assert_eq!(word, mc);
        prop_assert_eq!(kind, "Branch");
        prop_assert_eq!(got_sym, s, "reloc symbol must be the identifier");
        prop_assert_eq!(got_add, addend, "reloc addend must preserve SymbolOffset");
    }

    #[test]
    fn encode_branch_instr_neg_bad_3rd(
        rs1 in gpr_name(),
        rs2 in gpr_name(),
        third in invalid_3rd()
    ) {
        let ops = [reg(&rs1), reg(&rs2), third];
        prop_assert!(
            encode_branch_instr(&ops, 0b000).is_err(),
            "invalid 3rd operand must Err (branch: expected offset or label); got {:?}",
            encode_branch_instr(&ops, 0b000)
        );
    }
}
