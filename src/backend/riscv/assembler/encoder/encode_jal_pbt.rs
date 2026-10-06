// Oracle: differential — llvm-mc RISC-V assembler (JAL J-type word)
// Evidence: src/backend/riscv/assembler/README.md:6-7 textual assembly from codegen;
//   README.md:305 J-type: jal;
//   README.md:357 J-type: imm[20|10:1|11|19:12] | rd | opcode;
//   README.md:384 R_RISCV_JAL (J-type): 20-bit signed offset, bit-scattered;
//   encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words";
//   encoder/mod.rs:73 "R_RISCV_JAL - 20-bit PC-relative jump (J-type)";
//   encoder/mod.rs:460 "jal" => encode_jal;
//   base.rs:52 "jal rd, offset  OR  jal offset (rd = ra)";
//   RISC-V Unprivileged ISA JAL: opcode=1101111, rd, imm[20:1] (bit 0 implicit 0).
// Stronger considered:
//   - State machine: rejected — encode_jal is a pure function with no lifecycle
//   - Algebraic round-trip via in-tree decoder: rejected — no JAL decoder
//   - encode_j / encode_j_pseudo / encode_jalr / C.J as differential sibling: rejected —
//     same-job gate (private packer / jal x0 pseudo / I-type / compressed)
// Weaker available: algebraic.invariant (J-type field unpack), algebraic.metamorphic
//   (1-operand == jal ra; ABI vs xN alias), negative_error (oob/odd imm / extra /
//   SymbolOffset addend / FP dest / empty)
// Differential: candidate=encode_jal, reference=llvm-mc -triple=riscv64 -show-encoding,
//   SUT-boundary=internal-helper of RISC-V assembler,
//   mapping=[Reg(rd), Imm(off)] <-> `jal rd, off`;
//   [Imm(off)] <-> `jal off` (implicit rd=ra);
//   reloc-form word (imm=0) <-> `jal rd, 0` plus R_RISCV_JAL.

use super::{encode_jal, EncodeResult, RelocType};
use crate::backend::riscv::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;
const OP_JAL: u32 = 0b1101111;

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

/// Unpack J-type per RISC-V unprivileged ISA (not a copy of encode_j).
/// Layout: imm[20|10:1|11|19:12] | rd[11:7] | opcode[6:0]
fn unpack_j(word: u32) -> (u32, u32, i32) {
    let opcode = word & 0x7f;
    let rd = (word >> 7) & 0x1f;
    let imm20 = (word >> 31) & 1;
    let imm10_1 = (word >> 21) & 0x3ff;
    let imm11 = (word >> 20) & 1;
    let imm19_12 = (word >> 12) & 0xff;
    let u21 = (imm20 << 20) | (imm19_12 << 12) | (imm11 << 11) | (imm10_1 << 1);
    let off = ((u21 << 11) as i32) >> 11;
    (opcode, rd, off)
}

fn sut_word(ops: &[Operand]) -> Result<u32, String> {
    match encode_jal(ops)? {
        EncodeResult::Word(w) => Ok(w),
        other => Err(format!("expected Word, got {other:?}")),
    }
}

fn reloc_kind(t: &RelocType) -> &'static str {
    match t {
        RelocType::Jal => "Jal",
        other => panic!("unexpected reloc {other:?}"),
    }
}

fn sut_reloc(ops: &[Operand]) -> Result<(u32, &'static str, String, i64), String> {
    match encode_jal(ops)? {
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

/// Even JAL offset in the documented llvm-mc / ISA range, with bounds pinned.
fn even_jal_imm() -> impl Strategy<Value = i64> {
    prop_oneof![
        Just(0i64),
        Just(2i64),
        Just(-2i64),
        Just(4i64),
        Just(-4i64),
        Just(8i64),
        Just(-8i64),
        Just(2048i64),
        Just(-2048i64),
        Just(1048574i64),
        Just(-1048576i64),
        (-524288i64..=524287i64).prop_map(|k| k * 2),
    ]
}

fn ident() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("foo".into()),
        Just("bar".into()),
        Just("tls_var".into()),
        Just("_x".into()),
        Just("sym0".into()),
        "[A-Za-z_][A-Za-z0-9_]{0,11}",
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

fn oob_or_odd_jal_imm() -> impl Strategy<Value = i64> {
    prop_oneof![
        Just(1i64),
        Just(-1i64),
        Just(3i64),
        Just(-3i64),
        Just(1048575i64),
        Just(1048576i64),
        Just(-1048577i64),
        Just(-1048578i64),
        Just(i64::MIN),
        Just(i64::MAX),
        (-524288i64..=524287i64).prop_map(|k| k * 2 + 1),
        1048576i64..=i64::MAX,
        i64::MIN..=-1048577,
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
        1i64..=4096,
        -4096i64..=-1,
    ]
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

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_jal_kat_llvm_mc_x0_0() {
    let want = 0x0000_006fu32;
    let mc = llvm_mc_word("jal x0, 0").expect("llvm-mc KAT x0,0");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("x0"), Operand::Imm(0)]).expect("SUT KAT x0,0");
    assert_eq!(sut, want);
}

#[test]
fn encode_jal_kat_llvm_mc_x1_0() {
    let want = 0x0000_00efu32;
    let mc = llvm_mc_word("jal x1, 0").expect("llvm-mc KAT x1,0");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("x1"), Operand::Imm(0)]).expect("SUT KAT x1,0");
    assert_eq!(sut, want);
}

#[test]
fn encode_jal_kat_llvm_mc_x1_4() {
    let want = 0x0040_00efu32;
    let mc = llvm_mc_word("jal x1, 4").expect("llvm-mc KAT x1,4");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("x1"), Operand::Imm(4)]).expect("SUT KAT x1,4");
    assert_eq!(sut, want);
}

#[test]
fn encode_jal_kat_llvm_mc_x1_neg4() {
    let want = 0xffdf_f0efu32;
    let mc = llvm_mc_word("jal x1, -4").expect("llvm-mc KAT x1,-4");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("x1"), Operand::Imm(-4)]).expect("SUT KAT x1,-4");
    assert_eq!(sut, want);
}

#[test]
fn encode_jal_kat_llvm_mc_max_min() {
    let want_max = 0x7fff_f0efu32;
    let mc = llvm_mc_word("jal x1, 1048574").expect("llvm-mc KAT max");
    assert_eq!(mc, want_max, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("x1"), Operand::Imm(1048574)]).expect("SUT KAT max");
    assert_eq!(sut, want_max);

    let want_min = 0x8000_00efu32;
    let mc = llvm_mc_word("jal x1, -1048576").expect("llvm-mc KAT min");
    assert_eq!(mc, want_min, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("x1"), Operand::Imm(-1048576)]).expect("SUT KAT min");
    assert_eq!(sut, want_min);
}

#[test]
fn encode_jal_kat_one_operand_is_ra() {
    let want = 0x0040_00efu32;
    let mc = llvm_mc_word("jal 4").expect("llvm-mc KAT jal 4");
    assert_eq!(mc, want);
    let sut = sut_word(&[Operand::Imm(4)]).expect("SUT 1-operand KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_jal_kat_reloc_foo() {
    let want = 0x0000_00efu32;
    let mc = llvm_mc_word("jal x1, 0").expect("llvm-mc zero-imm word");
    assert_eq!(mc, want);
    let (word, kind, sym, addend) =
        sut_reloc(&[reg("x1"), Operand::Symbol("foo".into())]).expect("SUT symbol KAT");
    assert_eq!(word, want);
    assert_eq!(kind, "Jal");
    assert_eq!(sym, "foo");
    assert_eq!(addend, 0);
}

/// Regression: odd / out-of-range JAL immediate must be rejected
/// (llvm-mc: multiple of 2 in [-1048576, 1048574]).
#[test]
fn test_encode_jal_regression_imm_oob() {
    let ops = [reg("x0"), Operand::Imm(1)];
    assert!(
        encode_jal(&ops).is_err(),
        "jal x0, 1 must Err (odd offset); got {:?}",
        encode_jal(&ops)
    );
}

/// Regression: extra operand must be rejected (JAL is one- or two-operand; llvm-mc errors).
#[test]
fn test_encode_jal_regression_extra_operand() {
    let ops = [reg("x0"), Operand::Imm(0), Operand::Imm(0)];
    assert!(
        encode_jal(&ops).is_err(),
        "jal x0, 0 with a third operand must Err; got {:?}",
        encode_jal(&ops)
    );
}

/// Regression: SymbolOffset must relocate against the symbol with that addend.
#[test]
fn test_encode_jal_regression_symbol_offset() {
    let ops = [reg("x0"), Operand::SymbolOffset("foo".into(), 4)];
    match encode_jal(&ops) {
        Ok(EncodeResult::WordWithReloc { reloc, .. }) => {
            assert_eq!(
                reloc.symbol, "foo",
                "jal x0, foo+4 symbol must be foo, not {}",
                reloc.symbol
            );
            assert_eq!(
                reloc.addend, 4,
                "jal x0, foo+4 addend must be 4, not {}",
                reloc.addend
            );
        }
        other => panic!("expected WordWithReloc for foo+4, got {:?}", other),
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_jal_diff_imm_llvm_mc(rd in gpr_name(), off in even_jal_imm()) {
        let asm = format!("jal {}, {}", rd, off);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&[reg(&rd), Operand::Imm(off)])
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "SUT {:08x} != llvm-mc {:08x} for {}", sut, mc, asm);
    }

    #[test]
    fn encode_jal_one_operand_is_ra(off in even_jal_imm()) {
        let one = sut_word(&[Operand::Imm(off)])
            .unwrap_or_else(|e| panic!("SUT rejected 1-operand jal {}: {}", off, e));
        let two = sut_word(&[reg("ra"), Operand::Imm(off)])
            .unwrap_or_else(|e| panic!("SUT rejected jal ra, {}: {}", off, e));
        prop_assert_eq!(one, two, "1-operand jal {} must equal jal ra, {}", off, off);
        let asm = format!("jal {}", off);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        prop_assert_eq!(one, mc, "SUT {:08x} != llvm-mc {:08x} for {}", one, mc, asm);
    }

    #[test]
    fn encode_jal_isa_j_type(rd in 0u32..=31u32, off in even_jal_imm()) {
        let w = sut_word(&[reg(&xn(rd)), Operand::Imm(off)])
            .unwrap_or_else(|e| panic!("SUT rejected x{}, {}: {}", rd, off, e));
        let (opc, got_rd, got_off) = unpack_j(w);
        prop_assert_eq!(opc, OP_JAL, "opcode");
        prop_assert_eq!(got_rd, rd, "rd");
        prop_assert_eq!(got_off as i64, off, "imm");
    }

    #[test]
    fn encode_jal_abi_xn_alias(n in 0u32..=31u32, off in even_jal_imm()) {
        let via_x = sut_word(&[reg(&xn(n)), Operand::Imm(off)])
            .unwrap_or_else(|e| panic!("xN rejected: {}", e));
        let via_abi = sut_word(&[reg(abi_name(n)), Operand::Imm(off)])
            .unwrap_or_else(|e| panic!("ABI rejected: {}", e));
        prop_assert_eq!(via_abi, via_x);
        if n == 8 {
            let via_fp = sut_word(&[reg("fp"), Operand::Imm(off)])
                .unwrap_or_else(|e| panic!("fp rejected: {}", e));
            prop_assert_eq!(via_fp, via_x);
        }
    }

    #[test]
    fn encode_jal_reloc_symbol(rd in gpr_name(), s in ident()) {
        let zero_asm = format!("jal {}, 0", rd);
        let mc = llvm_mc_word(&zero_asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected {}: {}", zero_asm, e));
        let ops = [reg(&rd), Operand::Symbol(s.clone())];
        let (word, kind, got_sym, addend) = sut_reloc(&ops)
            .unwrap_or_else(|e| panic!("SUT rejected jal {}, {}: {}", rd, s, e));
        prop_assert_eq!(word, mc);
        prop_assert_eq!(kind, "Jal");
        prop_assert_eq!(got_sym, s.clone());
        prop_assert_eq!(addend, 0i64);

        let ops_l = [reg(&rd), Operand::Label(s.clone())];
        let (word_l, kind_l, got_l, add_l) = sut_reloc(&ops_l)
            .unwrap_or_else(|e| panic!("SUT rejected jal {}, Label({}): {}", rd, s, e));
        prop_assert_eq!(word_l, mc);
        prop_assert_eq!(kind_l, "Jal");
        prop_assert_eq!(got_l, s);
        prop_assert_eq!(add_l, 0i64);
    }

    #[test]
    fn encode_jal_neg_imm_oob_odd(rd in gpr_name(), imm in oob_or_odd_jal_imm()) {
        prop_assume!(imm % 2 != 0 || !( -1048576..=1048574 ).contains(&imm));
        let asm = format!("jal {}, {}", rd, imm);
        prop_assert!(
            llvm_mc_rejects(&asm),
            "reference unexpectedly accepted {}",
            asm
        );
        let ops = [reg(&rd), Operand::Imm(imm)];
        prop_assert!(
            encode_jal(&ops).is_err(),
            "oob/odd imm {} must Err (llvm-mc range even [-1048576, 1048574]); got {:?}",
            imm,
            encode_jal(&ops)
        );
    }

    #[test]
    fn encode_jal_neg_extra(rd in gpr_name(), off in even_jal_imm(), extra in extra_operand()) {
        let asm = format!("jal {}, {}", rd, off);
        let ops = vec![reg(&rd), Operand::Imm(off), extra];
        prop_assert!(
            encode_jal(&ops).is_err(),
            "extra operand must Err for {} (llvm-mc rejects extra operands); got {:?}",
            asm,
            encode_jal(&ops)
        );
    }

    #[test]
    fn encode_jal_symbol_offset_addend(rd in gpr_name(), s in ident(), addend in signed_addend()) {
        prop_assume!(addend != 0);
        let zero_asm = format!("jal {}, 0", rd);
        let mc = llvm_mc_word(&zero_asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected {}: {}", zero_asm, e));
        let ops = [reg(&rd), Operand::SymbolOffset(s.clone(), addend)];
        let (word, kind, got_sym, got_add) = sut_reloc(&ops)
            .unwrap_or_else(|e| panic!("SUT rejected jal {}, {}+{}: {}", rd, s, addend, e));
        prop_assert_eq!(word, mc);
        prop_assert_eq!(kind, "Jal");
        prop_assert_eq!(got_sym, s, "reloc symbol must be the identifier");
        prop_assert_eq!(got_add, addend, "reloc addend must preserve SymbolOffset");
    }

    #[test]
    fn encode_jal_one_operand_reloc(s in ident()) {
        let mc = llvm_mc_word("jal ra, 0")
            .unwrap_or_else(|e| panic!("llvm-mc rejected jal ra, 0: {}", e));
        let (word, kind, got_sym, addend) = sut_reloc(&[Operand::Symbol(s.clone())])
            .unwrap_or_else(|e| panic!("SUT rejected 1-operand jal {}: {}", s, e));
        prop_assert_eq!(word, mc);
        prop_assert_eq!(kind, "Jal");
        prop_assert_eq!(got_sym, s.clone());
        prop_assert_eq!(addend, 0i64);
        let (word_l, kind_l, got_l, add_l) = sut_reloc(&[Operand::Label(s.clone())])
            .unwrap_or_else(|e| panic!("SUT rejected 1-operand Label({}): {}", s, e));
        prop_assert_eq!(word_l, mc);
        prop_assert_eq!(kind_l, "Jal");
        prop_assert_eq!(got_l, s);
        prop_assert_eq!(add_l, 0i64);
    }

    #[test]
    fn encode_jal_neg_fp(fp in fp_name(), off in even_jal_imm()) {
        let ops = [reg(&fp), Operand::Imm(off)];
        prop_assert!(
            encode_jal(&ops).is_err(),
            "FP dest {} must Err (not a GPR); got {:?}",
            fp,
            encode_jal(&ops)
        );
    }

    #[test]
    fn encode_jal_neg_empty(_unit in Just(())) {
        prop_assert!(
            encode_jal(&[]).is_err(),
            "empty operand list must Err; got {:?}",
            encode_jal(&[])
        );
    }
}
