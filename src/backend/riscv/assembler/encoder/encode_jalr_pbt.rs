// Oracle: differential — llvm-mc RISC-V assembler (JALR I-type word)
// Evidence: src/backend/riscv/assembler/README.md:6-7 textual assembly from codegen;
//   README.md:300-301 I-type includes jalr;
//   README.md:353 I-type: imm[11:0] | rs1 | funct3 | rd | opcode;
//   encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words";
//   encoder/mod.rs:285 "I-type: imm[31:20] | rs1[19:15] | funct3[14:12] | rd[11:7] | opcode[6:0]";
//   encoder/mod.rs:330 OP_JALR = 0b1100111;
//   encoder/mod.rs:463 "jalr" => encode_jalr;
//   base.rs:93 "jalr rd, rs1, offset  OR  jalr rd, offset(rs1)  OR  jalr rs1";
//   base.rs:96 "jalr rs1 (rd = ra, offset = 0)";
//   base.rs:101 "jalr rd, rs1  (offset = 0)";
//   RISC-V Unprivileged ISA JALR: opcode=1100111, funct3=000, rd, rs1, imm[11:0].
// Stronger considered:
//   - State machine: rejected — encode_jalr is a pure function with no lifecycle
//   - Algebraic round-trip via in-tree decoder: rejected — no JALR decoder
//   - encode_i / encode_c_jalr / jr / ret / encode_jal as differential sibling: rejected —
//     same-job gate (private packer / compressed / jalr x0 pseudo / J-type)
// Weaker available: algebraic.invariant (I-type field unpack), algebraic.metamorphic
//   (1-operand == jalr ra, rs1, 0; 2-op reg == 3-op zero; 2-op mem == 3-op; ABI vs xN),
//   negative_error (oob imm / extra / FP dest-src / empty)
// Differential: candidate=encode_jalr, reference=llvm-mc -triple=riscv64 -show-encoding,
//   SUT-boundary=internal-helper of RISC-V assembler,
//   mapping=[Reg(rd), Reg(rs1), Imm(off)] <-> `jalr rd, rs1, off`;
//   [Reg(rd), Mem{base, offset}] <-> `jalr rd, offset(rs1)`;
//   [Reg(rs1)] <-> `jalr rs1` (implicit rd=ra, offset=0);
//   [Reg(rd), Reg(rs1)] <-> `jalr rd, rs1` (offset=0).

use super::{encode_jalr, EncodeResult, RelocType};
use crate::backend::riscv::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;
const OP_JALR: u32 = 0b1100111;

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
    let imm = (word as i32) >> 20;
    (opcode, rd, funct3, rs1, imm)
}

fn sut_word(ops: &[Operand]) -> Result<u32, String> {
    match encode_jalr(ops)? {
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

/// Signed 12-bit JALR immediate, with bounds pinned.
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
        "[A-Za-z_][A-Za-z0-9_]{0,11}",
    ]
}

fn reloc_kind(t: &RelocType) -> &'static str {
    match t {
        RelocType::PcrelLo12I => "PcrelLo12I",
        RelocType::Lo12I => "Lo12I",
        RelocType::TprelLo12I => "TprelLo12I",
        other => panic!("unexpected reloc {other:?}"),
    }
}

fn sut_reloc(ops: &[Operand]) -> Result<(u32, &'static str, String, i64), String> {
    match encode_jalr(ops)? {
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

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_jalr_kat_llvm_mc_x0_x1() {
    let want = 0x0000_8067u32;
    let mc = llvm_mc_word("jalr x0, x1, 0").expect("llvm-mc KAT x0,x1,0");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("x0"), reg("x1"), Operand::Imm(0)]).expect("SUT KAT x0,x1,0");
    assert_eq!(sut, want);
}

#[test]
fn encode_jalr_kat_llvm_mc_x1_x2_0() {
    let want = 0x0001_00e7u32;
    let mc = llvm_mc_word("jalr x1, x2, 0").expect("llvm-mc KAT x1,x2,0");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("x1"), reg("x2"), Operand::Imm(0)]).expect("SUT KAT x1,x2,0");
    assert_eq!(sut, want);
}

#[test]
fn encode_jalr_kat_llvm_mc_x1_x2_8() {
    let want = 0x0081_00e7u32;
    let mc = llvm_mc_word("jalr x1, x2, 8").expect("llvm-mc KAT x1,x2,8");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("x1"), reg("x2"), Operand::Imm(8)]).expect("SUT KAT x1,x2,8");
    assert_eq!(sut, want);
}

#[test]
fn encode_jalr_kat_llvm_mc_x1_x2_neg8() {
    let want = 0xff81_00e7u32;
    let mc = llvm_mc_word("jalr x1, x2, -8").expect("llvm-mc KAT x1,x2,-8");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("x1"), reg("x2"), Operand::Imm(-8)]).expect("SUT KAT x1,x2,-8");
    assert_eq!(sut, want);
}

#[test]
fn encode_jalr_kat_llvm_mc_max_min() {
    let want_max = 0x7ff1_00e7u32;
    let mc = llvm_mc_word("jalr x1, x2, 2047").expect("llvm-mc KAT max");
    assert_eq!(mc, want_max, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("x1"), reg("x2"), Operand::Imm(2047)]).expect("SUT KAT max");
    assert_eq!(sut, want_max);

    let want_min = 0x8001_00e7u32;
    let mc = llvm_mc_word("jalr x1, x2, -2048").expect("llvm-mc KAT min");
    assert_eq!(mc, want_min, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("x1"), reg("x2"), Operand::Imm(-2048)]).expect("SUT KAT min");
    assert_eq!(sut, want_min);
}

#[test]
fn encode_jalr_kat_one_operand_is_ra() {
    let want = 0x0000_80e7u32;
    let mc = llvm_mc_word("jalr x1").expect("llvm-mc KAT jalr x1");
    assert_eq!(mc, want);
    let sut = sut_word(&[reg("x1")]).expect("SUT 1-operand KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_jalr_kat_odd_imm() {
    let want = 0x0011_00e7u32;
    let mc = llvm_mc_word("jalr x1, x2, 1").expect("llvm-mc KAT odd");
    assert_eq!(mc, want);
    let sut = sut_word(&[reg("x1"), reg("x2"), Operand::Imm(1)]).expect("SUT odd KAT");
    assert_eq!(sut, want);
}

/// Regression: out-of-range JALR immediate must be rejected
/// (llvm-mc: integer in [-2048, 2047]).
#[test]
fn test_encode_jalr_regression_imm_oob() {
    let ops = [reg("x0"), reg("x1"), Operand::Imm(2048)];
    assert!(
        encode_jalr(&ops).is_err(),
        "jalr x0, x1, 2048 must Err (imm12 range); got {:?}",
        encode_jalr(&ops)
    );
}

/// Regression: extra operand must be rejected (JALR is 1/2/3-operand; llvm-mc errors).
#[test]
fn test_encode_jalr_regression_extra_operand() {
    let ops = [reg("x0"), reg("x1"), Operand::Imm(0), Operand::Imm(0)];
    assert!(
        encode_jalr(&ops).is_err(),
        "jalr x0, x1, 0 with a fourth operand must Err; got {:?}",
        encode_jalr(&ops)
    );
}

/// Regression: 1-operand mem form jalr off(rs1) equals jalr ra, rs1, off (llvm-mc).
#[test]
fn test_encode_jalr_regression_one_operand_mem() {
    let ops = [Operand::Mem {
        base: "x2".into(),
        offset: 8,
    }];
    match encode_jalr(&ops) {
        Ok(EncodeResult::Word(w)) => {
            let want = sut_word(&[reg("ra"), reg("x2"), Operand::Imm(8)])
                .expect("jalr ra, x2, 8");
            assert_eq!(w, want, "jalr 8(x2) must equal jalr ra, x2, 8");
        }
        other => panic!("expected Word for jalr 8(x2), got {:?}", other),
    }
}

/// Regression: jalr rd, %pcrel_lo(sym)(rs1) must relocate, not reject.
#[test]
fn test_encode_jalr_regression_pcrel_lo() {
    let ops = [reg("ra"), Operand::MemSymbol {
        base: "ra".into(),
        symbol: "%pcrel_lo(foo)".into(),
        modifier: String::new(),
    }];
    match encode_jalr(&ops) {
        Ok(EncodeResult::WordWithReloc { word, reloc }) => {
            let want = sut_word(&[reg("ra"), Operand::Mem {
                base: "ra".into(),
                offset: 0,
            }])
            .expect("jalr ra, 0(ra)");
            assert_eq!(word, want);
            match reloc.reloc_type {
                RelocType::PcrelLo12I => {}
                other => panic!("expected PcrelLo12I, got {:?}", other),
            }
            assert_eq!(reloc.symbol, "foo");
            assert_eq!(reloc.addend, 0);
        }
        other => panic!(
            "expected WordWithReloc for jalr ra, %pcrel_lo(foo)(ra), got {:?}",
            other
        ),
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_jalr_diff_3op_llvm_mc(
        rd in gpr_name(),
        rs1 in gpr_name(),
        off in i12_imm()
    ) {
        let asm = format!("jalr {}, {}, {}", rd, rs1, off);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&[reg(&rd), reg(&rs1), Operand::Imm(off)])
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "SUT {:08x} != llvm-mc {:08x} for {}", sut, mc, asm);
    }

    #[test]
    fn encode_jalr_isa_i_type(rd in 0u32..=31u32, rs1 in 0u32..=31u32, off in i12_imm()) {
        let w = sut_word(&[reg(&xn(rd)), reg(&xn(rs1)), Operand::Imm(off)])
            .unwrap_or_else(|e| panic!("SUT rejected x{}, x{}, {}: {}", rd, rs1, off, e));
        let (opc, got_rd, funct3, got_rs1, got_off) = unpack_i(w);
        prop_assert_eq!(opc, OP_JALR, "opcode");
        prop_assert_eq!(got_rd, rd, "rd");
        prop_assert_eq!(funct3, 0u32, "funct3");
        prop_assert_eq!(got_rs1, rs1, "rs1");
        prop_assert_eq!(got_off as i64, off, "imm");
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_jalr_one_operand_is_ra(rs1 in gpr_name()) {
        let one = sut_word(&[reg(&rs1)])
            .unwrap_or_else(|e| panic!("SUT rejected 1-operand jalr {}: {}", rs1, e));
        let three = sut_word(&[reg("ra"), reg(&rs1), Operand::Imm(0)])
            .unwrap_or_else(|e| panic!("SUT rejected jalr ra, {}, 0: {}", rs1, e));
        prop_assert_eq!(one, three, "1-operand jalr {} must equal jalr ra, {}, 0", rs1, rs1);
        let asm = format!("jalr {}", rs1);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        prop_assert_eq!(one, mc, "SUT {:08x} != llvm-mc {:08x} for {}", one, mc, asm);
    }

    #[test]
    fn encode_jalr_two_op_reg_eq_zero_imm(rd in gpr_name(), rs1 in gpr_name()) {
        let two = sut_word(&[reg(&rd), reg(&rs1)])
            .unwrap_or_else(|e| panic!("SUT rejected jalr {}, {}: {}", rd, rs1, e));
        let three = sut_word(&[reg(&rd), reg(&rs1), Operand::Imm(0)])
            .unwrap_or_else(|e| panic!("SUT rejected jalr {}, {}, 0: {}", rd, rs1, e));
        prop_assert_eq!(two, three, "jalr {}, {} must equal jalr {}, {}, 0", rd, rs1, rd, rs1);
        let asm = format!("jalr {}, {}", rd, rs1);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        prop_assert_eq!(two, mc, "SUT {:08x} != llvm-mc {:08x} for {}", two, mc, asm);
    }

    #[test]
    fn encode_jalr_two_op_mem_eq_3op(
        rd in gpr_name(),
        rs1 in gpr_name(),
        off in i12_imm()
    ) {
        let mem = Operand::Mem {
            base: rs1.clone(),
            offset: off,
        };
        let via_mem = sut_word(&[reg(&rd), mem])
            .unwrap_or_else(|e| panic!("SUT rejected jalr {}, {}({}): {}", rd, off, rs1, e));
        let via_3 = sut_word(&[reg(&rd), reg(&rs1), Operand::Imm(off)])
            .unwrap_or_else(|e| panic!("SUT rejected jalr {}, {}, {}: {}", rd, rs1, off, e));
        prop_assert_eq!(
            via_mem, via_3,
            "jalr {}, {}({}) must equal jalr {}, {}, {}",
            rd, off, rs1, rd, rs1, off
        );
        let asm = format!("jalr {}, {}({})", rd, off, rs1);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        prop_assert_eq!(via_mem, mc, "SUT {:08x} != llvm-mc {:08x} for {}", via_mem, mc, asm);
    }

    #[test]
    fn encode_jalr_abi_xn_alias(n in 0u32..=31u32, m in 0u32..=31u32, off in i12_imm()) {
        let via_x = sut_word(&[reg(&xn(n)), reg(&xn(m)), Operand::Imm(off)])
            .unwrap_or_else(|e| panic!("xN rejected: {}", e));
        let via_abi = sut_word(&[reg(abi_name(n)), reg(abi_name(m)), Operand::Imm(off)])
            .unwrap_or_else(|e| panic!("ABI rejected: {}", e));
        prop_assert_eq!(via_abi, via_x);
        if n == 8 {
            let via_fp = sut_word(&[reg("fp"), reg(&xn(m)), Operand::Imm(off)])
                .unwrap_or_else(|e| panic!("fp rejected: {}", e));
            prop_assert_eq!(via_fp, via_x);
        }
    }

    #[test]
    fn encode_jalr_neg_imm_oob(rd in gpr_name(), rs1 in gpr_name(), imm in oob_i12()) {
        prop_assume!(!(-2048..=2047).contains(&imm));
        let asm = format!("jalr {}, {}, {}", rd, rs1, imm);
        prop_assert!(
            llvm_mc_rejects(&asm),
            "reference unexpectedly accepted {}",
            asm
        );
        let ops = [reg(&rd), reg(&rs1), Operand::Imm(imm)];
        prop_assert!(
            encode_jalr(&ops).is_err(),
            "oob imm {} must Err (llvm-mc range [-2048, 2047]); got {:?}",
            imm,
            encode_jalr(&ops)
        );
    }

    #[test]
    fn encode_jalr_neg_arity_fp(
        rd in gpr_name(),
        rs1 in gpr_name(),
        off in i12_imm(),
        extra in extra_operand(),
        fp in fp_name()
    ) {
        prop_assert!(
            encode_jalr(&[]).is_err(),
            "empty operand list must Err; got {:?}",
            encode_jalr(&[])
        );
        let ops = vec![reg(&rd), reg(&rs1), Operand::Imm(off), extra];
        prop_assert!(
            encode_jalr(&ops).is_err(),
            "extra operand must Err for jalr {}, {}, {} (llvm-mc rejects extra operands); got {:?}",
            rd, rs1, off,
            encode_jalr(&ops)
        );
        let fp_rd = [reg(&fp), reg(&rd), Operand::Imm(off)];
        prop_assert!(
            encode_jalr(&fp_rd).is_err(),
            "FP dest {} must Err (not a GPR); got {:?}",
            fp,
            encode_jalr(&fp_rd)
        );
        let fp_rs1 = [reg(&rd), reg(&fp), Operand::Imm(off)];
        prop_assert!(
            encode_jalr(&fp_rs1).is_err(),
            "FP rs1 {} must Err (not a GPR); got {:?}",
            fp,
            encode_jalr(&fp_rs1)
        );
    }

    #[test]
    fn encode_jalr_one_operand_mem(rs1 in gpr_name(), off in i12_imm()) {
        let mem = Operand::Mem {
            base: rs1.clone(),
            offset: off,
        };
        let one = sut_word(&[mem])
            .unwrap_or_else(|e| panic!("SUT rejected 1-operand jalr {}({}): {}", off, rs1, e));
        let three = sut_word(&[reg("ra"), reg(&rs1), Operand::Imm(off)])
            .unwrap_or_else(|e| panic!("SUT rejected jalr ra, {}, {}: {}", rs1, off, e));
        prop_assert_eq!(
            one, three,
            "jalr {}({}) must equal jalr ra, {}, {}",
            off, rs1, rs1, off
        );
        let asm = format!("jalr {}({})", off, rs1);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        prop_assert_eq!(one, mc, "SUT {:08x} != llvm-mc {:08x} for {}", one, mc, asm);
    }

    #[test]
    fn encode_jalr_reloc_lo(rd in gpr_name(), rs1 in gpr_name(), s in ident()) {
        let zero = sut_word(&[reg(&rd), Operand::Mem {
            base: rs1.clone(),
            offset: 0,
        }])
        .unwrap_or_else(|e| panic!("SUT rejected jalr {}, 0({}): {}", rd, rs1, e));
        let ops = [reg(&rd), Operand::MemSymbol {
            base: rs1.clone(),
            symbol: format!("%pcrel_lo({})", s),
            modifier: String::new(),
        }];
        let (word, kind, got_sym, addend) = sut_reloc(&ops)
            .unwrap_or_else(|e| panic!("SUT rejected jalr {}, %pcrel_lo({})({}): {}", rd, s, rs1, e));
        prop_assert_eq!(word, zero, "reloc-form word must equal jalr {}, 0({})", rd, rs1);
        prop_assert_eq!(kind, "PcrelLo12I");
        prop_assert_eq!(got_sym, s.clone(), "reloc symbol must be the identifier");
        prop_assert_eq!(addend, 0i64);

        let ops_lo = [reg(&rd), Operand::MemSymbol {
            base: rs1.clone(),
            symbol: format!("%lo({})", s),
            modifier: String::new(),
        }];
        let (word_lo, kind_lo, got_lo, add_lo) = sut_reloc(&ops_lo)
            .unwrap_or_else(|e| panic!("SUT rejected jalr {}, %lo({})({}): {}", rd, s, rs1, e));
        prop_assert_eq!(word_lo, zero);
        prop_assert_eq!(kind_lo, "Lo12I");
        prop_assert_eq!(got_lo, s);
        prop_assert_eq!(add_lo, 0i64);
    }
}
