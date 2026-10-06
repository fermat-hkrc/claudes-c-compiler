// Oracle: differential — llvm-mc RISC-V assembler (I-type SYSTEM CSR-immediate)
// Evidence: src/backend/riscv/assembler/README.md:6-7 textual assembly from codegen;
//   README.md:105 system.rs System instructions: CSR read/write/set/clear;
//   README.md:311-312 System: ecall, ebreak, fence, fence.i, csrr/csrw/csrs/csrc
//   and their immediate variants (csrwi, csrsi, csrci);
//   README.md:353 I-type: [imm[11:0] | rs1 | funct3 | rd | opcode];
//   encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words";
//   encoder/mod.rs:4 "This covers the subset of instructions emitted by our codegen (RV64GC + Zbb).";
//   encoder/mod.rs:313 "I-type: imm[31:20] | rs1[19:15] | funct3[14:12] | rd[11:7] | opcode[6:0]";
//   encoder/mod.rs:366 OP_SYSTEM = 0b1110011;
//   encoder/mod.rs:706-708 csrrwi/csrrsi/csrrci => encode_csri with operands passed through;
//   encoder/mod.rs:386 "GCC sometimes emits bare register numbers (0-31) in inline asm";
//   RISC-V Unprivileged ISA SYSTEM CSR-immediate: opcode=1110011, csr[31:20],
//   zimm[19:15], funct3, rd, with csrrwi/csrrsi/csrrci = 101/110/111;
//   zimm is uimm5 in [0, 31]; csr is 12-bit in [0, 4095].
// Stronger considered:
//   - State machine: rejected — encode_csri is a pure function with no lifecycle
//   - Algebraic round-trip via in-tree decoder: rejected — no SYSTEM/CSR decoder
//   - encode_i / encode_csr / encode_csrw as differential sibling: rejected —
//     same-job gate (private packer / register-form sibling / pseudo with rd=x0)
// Weaker available: algebraic.invariant (I-type field unpack), algebraic.metamorphic
//   (ABI vs xN alias; Csr name vs Imm number), negative_error (extra / FP / empty /
//   oob zimm / oob csr / unknown CSR / non-Imm zimm)
// Differential: candidate=encode_csri, reference=llvm-mc -triple=riscv64 -show-encoding,
//   SUT-boundary=internal-helper of RISC-V assembler,
//   mapping=[Reg(rd), Csr(csr), Imm(zimm)] <-> `mn rd, csr, zimm`.

use super::{encode_csri, EncodeResult};
use crate::backend::riscv::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;
const OP_SYSTEM: u32 = 0b1110011;

const ABI: [&str; 32] = [
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3",
    "a4", "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11",
    "t3", "t4", "t5", "t6",
];

/// Immediate-form SYSTEM CSR mnemonics that dispatch to encode_csri, with funct3.
const CSRI_MN: [(&str, u32); 3] = [
    ("csrrwi", 0b101),
    ("csrrsi", 0b110),
    ("csrrci", 0b111),
];

/// CSRs named in csr_name_to_num that llvm-mc RV64 also accepts (no *h aliases).
const KNOWN_CSR: [(&str, u32); 24] = [
    ("fflags", 0x001),
    ("frm", 0x002),
    ("fcsr", 0x003),
    ("cycle", 0xC00),
    ("time", 0xC01),
    ("instret", 0xC02),
    ("mstatus", 0x300),
    ("misa", 0x301),
    ("mie", 0x304),
    ("mtvec", 0x305),
    ("mscratch", 0x340),
    ("mepc", 0x341),
    ("mcause", 0x342),
    ("mtval", 0x343),
    ("mip", 0x344),
    ("sstatus", 0x100),
    ("sip", 0x144),
    ("sie", 0x104),
    ("stvec", 0x105),
    ("sscratch", 0x140),
    ("sepc", 0x141),
    ("scause", 0x142),
    ("stval", 0x143),
    ("satp", 0x180),
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

fn csr_op(name: &str) -> Operand {
    Operand::Csr(name.to_string())
}

/// Unpack I-type per RISC-V unprivileged ISA (not a copy of encode_i).
/// Layout: imm[31:20] | rs1[19:15] | funct3[14:12] | rd[11:7] | opcode[6:0]
fn unpack_i(word: u32) -> (u32, u32, u32, u32, u32) {
    let opcode = word & 0x7f;
    let rd = (word >> 7) & 0x1f;
    let funct3 = (word >> 12) & 0x7;
    let rs1 = (word >> 15) & 0x1f;
    let imm12 = (word >> 20) & 0xfff;
    (opcode, funct3, rd, rs1, imm12)
}

fn sut_word(ops: &[Operand], funct3: u32) -> Result<u32, String> {
    match encode_csri(ops, funct3)? {
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

fn gpr_name() -> impl Strategy<Value = String> {
    prop_oneof![
        (0u32..=31).prop_map(xn),
        (0u32..=31).prop_map(|n| abi_name(n).to_string()),
        Just("fp".into()),
    ]
}

fn csri_mn() -> impl Strategy<Value = (&'static str, u32)> {
    prop::sample::select(CSRI_MN.to_vec())
}

fn known_csr() -> impl Strategy<Value = (&'static str, u32)> {
    prop::sample::select(KNOWN_CSR.to_vec())
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
        Just(Operand::Csr("mstatus".into())),
        Just(Operand::FenceArg("iorw".into())),
        Just(Operand::RoundingMode("rne".into())),
    ]
}

fn fp_name() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("ft0".into()),
        Just("fs0".into()),
        Just("fa0".into()),
        Just("fa7".into()),
        Just("ft11".into()),
        Just("f0".into()),
        Just("f31".into()),
    ]
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

fn unknown_csr_name() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("notacsr".into()),
        Just("mhartid".into()),
        Just("xyzzy".into()),
        Just("foo".into()),
        Just("x32".into()),
    ]
}

fn oob_zimm() -> impl Strategy<Value = i64> {
    prop_oneof![
        Just(-1i64),
        Just(-2i64),
        Just(32i64),
        Just(33i64),
        Just(63i64),
        Just(64i64),
        Just(255i64),
        Just(i64::MIN),
        Just(i64::MAX),
        32i64..=i64::MAX,
        i64::MIN..=-1,
    ]
}

fn oob_csr() -> impl Strategy<Value = i64> {
    prop_oneof![
        Just(-1i64),
        Just(-2i64),
        Just(4096i64),
        Just(4097i64),
        Just(8192i64),
        Just(i64::MIN),
        Just(i64::MAX),
        4096i64..=i64::MAX,
        i64::MIN..=-1,
    ]
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_csri_kat_llvm_mc_csrrwi_x1_mstatus_5() {
    let want = 0x3002_d0f3u32;
    let mc = llvm_mc_word("csrrwi x1, mstatus, 5").expect("llvm-mc KAT csrrwi x1, mstatus, 5");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("x1"), csr_op("mstatus"), Operand::Imm(5)], 0b101)
        .expect("SUT KAT csrrwi x1, mstatus, 5");
    assert_eq!(sut, want);
}

#[test]
fn encode_csri_kat_llvm_mc_csrrsi_csrrci() {
    let want_csrrsi = 0x1000_6573u32;
    assert_eq!(llvm_mc_word("csrrsi a0, sstatus, 0").unwrap(), want_csrrsi);
    assert_eq!(
        sut_word(&[reg("a0"), csr_op("sstatus"), Operand::Imm(0)], 0b110).unwrap(),
        want_csrrsi
    );

    let want_csrrci = 0x001f_fff3u32;
    assert_eq!(llvm_mc_word("csrrci x31, fflags, 31").unwrap(), want_csrrci);
    assert_eq!(
        sut_word(&[reg("x31"), csr_op("fflags"), Operand::Imm(31)], 0b111).unwrap(),
        want_csrrci
    );
}

/// Regression: extra operand must be rejected (llvm-mc errors).
#[test]
fn test_encode_csri_regression_extra_operand() {
    let ops = [
        reg("x1"),
        csr_op("mstatus"),
        Operand::Imm(5),
        Operand::Imm(0),
    ];
    assert!(
        encode_csri(&ops, 0b101).is_err(),
        "csrrwi x1, mstatus, 5 with a fourth operand must Err; got {:?}",
        encode_csri(&ops, 0b101)
    );
}

/// Regression: zimm outside 0..=31 must be rejected (llvm-mc range [0, 31]).
#[test]
fn test_encode_csri_regression_zimm_oob() {
    let ops32 = [reg("x1"), csr_op("mstatus"), Operand::Imm(32)];
    assert!(
        encode_csri(&ops32, 0b101).is_err(),
        "csrrwi x1, mstatus, 32 must Err (zimm not in 0..=31); got {:?}",
        encode_csri(&ops32, 0b101)
    );
    let ops_neg = [reg("x1"), csr_op("mstatus"), Operand::Imm(-1)];
    assert!(
        encode_csri(&ops_neg, 0b101).is_err(),
        "csrrwi x1, mstatus, -1 must Err (zimm not in 0..=31); got {:?}",
        encode_csri(&ops_neg, 0b101)
    );
}

/// Regression: CSR number outside 0..=4095 must be rejected (llvm-mc range [0, 4095]).
#[test]
fn test_encode_csri_regression_csr_oob() {
    let ops = [reg("x1"), Operand::Imm(4096), Operand::Imm(5)];
    assert!(
        encode_csri(&ops, 0b101).is_err(),
        "csrrwi x1, 4096, 5 must Err (csr not in 0..=4095); got {:?}",
        encode_csri(&ops, 0b101)
    );
    let ops_neg = [reg("x1"), Operand::Imm(-1), Operand::Imm(5)];
    assert!(
        encode_csri(&ops_neg, 0b101).is_err(),
        "csrrwi x1, -1, 5 must Err (csr not in 0..=4095); got {:?}",
        encode_csri(&ops_neg, 0b101)
    );
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_csri_diff_llvm_mc(
        (mn, f3) in csri_mn(),
        rd in gpr_name(),
        (csr_name, _num) in known_csr(),
        zimm in 0u32..=31u32
    ) {
        let asm = format!("{} {}, {}, {}", mn, rd, csr_name, zimm);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&[reg(&rd), csr_op(csr_name), Operand::Imm(zimm as i64)], f3)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "SUT {:08x} != llvm-mc {:08x} for {}", sut, mc, asm);
    }

    #[test]
    fn encode_csri_i_type_fields(
        (_mn, f3) in csri_mn(),
        rd in 0u32..=31u32,
        csr in 0u32..=4095u32,
        zimm in 0u32..=31u32
    ) {
        let w = sut_word(
            &[reg(&xn(rd)), Operand::Imm(csr as i64), Operand::Imm(zimm as i64)],
            f3,
        )
        .unwrap_or_else(|e| panic!("SUT rejected csr {} zimm {}: {}", csr, zimm, e));
        let (opc, got_f3, got_rd, got_rs1, got_imm) = unpack_i(w);
        prop_assert_eq!(opc, OP_SYSTEM, "opcode");
        prop_assert_eq!(got_f3, f3, "funct3");
        prop_assert_eq!(got_rd, rd, "rd");
        prop_assert_eq!(got_rs1, zimm, "zimm in rs1");
        prop_assert_eq!(got_imm, csr, "csr");
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_csri_abi_xn_alias(
        (_mn, f3) in csri_mn(),
        n in 0u32..=31u32,
        (csr_name, _num) in known_csr(),
        zimm in 0u32..=31u32
    ) {
        let via_x = sut_word(
            &[reg(&xn(n)), csr_op(csr_name), Operand::Imm(zimm as i64)],
            f3,
        )
        .unwrap_or_else(|e| panic!("xN rejected: {}", e));
        let via_abi = sut_word(
            &[reg(abi_name(n)), csr_op(csr_name), Operand::Imm(zimm as i64)],
            f3,
        )
        .unwrap_or_else(|e| panic!("ABI rejected: {}", e));
        prop_assert_eq!(via_abi, via_x);
        if n == 8 {
            let via_fp = sut_word(
                &[reg("fp"), csr_op(csr_name), Operand::Imm(zimm as i64)],
                f3,
            )
            .unwrap_or_else(|e| panic!("fp rd rejected: {}", e));
            prop_assert_eq!(via_fp, via_x);
        }
        // Imm 0..=31 is a GCC bare register number only at rd (get_reg).
        let via_imm_rd = sut_word(
            &[Operand::Imm(n as i64), csr_op(csr_name), Operand::Imm(zimm as i64)],
            f3,
        )
        .unwrap_or_else(|e| panic!("Imm 0-31 as rd rejected: {}", e));
        prop_assert_eq!(via_imm_rd, via_x);
    }

    #[test]
    fn encode_csri_name_vs_imm(
        (_mn, f3) in csri_mn(),
        rd in gpr_name(),
        (csr_name, num) in known_csr(),
        zimm in 0u32..=31u32
    ) {
        let via_csr = sut_word(
            &[reg(&rd), csr_op(csr_name), Operand::Imm(zimm as i64)],
            f3,
        )
        .unwrap_or_else(|e| panic!("Csr({}) rejected: {}", csr_name, e));
        let via_imm = sut_word(
            &[reg(&rd), Operand::Imm(num as i64), Operand::Imm(zimm as i64)],
            f3,
        )
        .unwrap_or_else(|e| panic!("Imm({}) rejected: {}", num, e));
        let via_sym = sut_word(
            &[reg(&rd), Operand::Symbol(csr_name.into()), Operand::Imm(zimm as i64)],
            f3,
        )
        .unwrap_or_else(|e| panic!("Symbol({}) rejected: {}", csr_name, e));
        let hex = format!("0x{num:x}");
        let via_hex = sut_word(
            &[reg(&rd), csr_op(&hex), Operand::Imm(zimm as i64)],
            f3,
        )
        .unwrap_or_else(|e| panic!("Csr({}) rejected: {}", hex, e));
        prop_assert_eq!(via_imm, via_csr);
        prop_assert_eq!(via_sym, via_csr);
        prop_assert_eq!(via_hex, via_csr);
        // get_csr_num: "sometimes CSR names look like regs"
        let via_reg = sut_word(
            &[reg(&rd), Operand::Reg(csr_name.into()), Operand::Imm(zimm as i64)],
            f3,
        )
        .unwrap_or_else(|e| panic!("Reg({}) as CSR rejected: {}", csr_name, e));
        prop_assert_eq!(via_reg, via_csr);
        let dec = num.to_string();
        let via_dec = sut_word(
            &[reg(&rd), csr_op(&dec), Operand::Imm(zimm as i64)],
            f3,
        )
        .unwrap_or_else(|e| panic!("Csr({}) decimal rejected: {}", dec, e));
        prop_assert_eq!(via_dec, via_csr);
    }

    #[test]
    fn encode_csri_neg_extra(
        (mn, f3) in csri_mn(),
        rd in gpr_name(),
        (csr_name, _num) in known_csr(),
        zimm in 0u32..=31u32,
        extra in extra_operand()
    ) {
        let asm = format!("{} {}, {}, {}", mn, rd, csr_name, zimm);
        let ops = vec![reg(&rd), csr_op(csr_name), Operand::Imm(zimm as i64), extra];
        prop_assert!(
            encode_csri(&ops, f3).is_err(),
            "extra operand must Err for {} (llvm-mc rejects extra operands); got {:?}",
            asm,
            encode_csri(&ops, f3)
        );
    }

    #[test]
    fn encode_csri_neg_zimm_oob(
        (_mn, f3) in csri_mn(),
        rd in gpr_name(),
        (csr_name, _num) in known_csr(),
        zimm in oob_zimm()
    ) {
        prop_assume!(!(0..=31).contains(&zimm));
        let zops = [reg(&rd), csr_op(csr_name), Operand::Imm(zimm)];
        prop_assert!(
            encode_csri(&zops, f3).is_err(),
            "zimm {} outside 0..=31 must Err (llvm-mc uimm5); got {:?}",
            zimm,
            encode_csri(&zops, f3)
        );
    }

    #[test]
    fn encode_csri_neg_csr_oob(
        (_mn, f3) in csri_mn(),
        rd in gpr_name(),
        zimm in 0u32..=31u32,
        csr_num in oob_csr()
    ) {
        prop_assume!(!(0..=4095).contains(&csr_num));
        let cops = [reg(&rd), Operand::Imm(csr_num), Operand::Imm(zimm as i64)];
        prop_assert!(
            encode_csri(&cops, f3).is_err(),
            "csr {} outside 0..=4095 must Err (llvm-mc csr[11:0]); got {:?}",
            csr_num,
            encode_csri(&cops, f3)
        );
    }

    #[test]
    fn encode_csri_neg_arity_fp_unknown(
        (_mn, f3) in csri_mn(),
        rd in gpr_name(),
        (csr_name, _num) in known_csr(),
        zimm in 0u32..=31u32,
        fp in fp_name(),
        bad in invalid_gpr_name(),
        unknown in unknown_csr_name()
    ) {
        prop_assert!(
            encode_csri(&[], f3).is_err(),
            "empty operand list must Err; got {:?}",
            encode_csri(&[], f3)
        );
        prop_assert!(
            encode_csri(&[reg(&rd)], f3).is_err(),
            "missing csr/zimm must Err; got {:?}",
            encode_csri(&[reg(&rd)], f3)
        );
        prop_assert!(
            encode_csri(&[reg(&rd), csr_op(csr_name)], f3).is_err(),
            "missing zimm must Err; got {:?}",
            encode_csri(&[reg(&rd), csr_op(csr_name)], f3)
        );
        let fp_rd = [reg(&fp), csr_op(csr_name), Operand::Imm(zimm as i64)];
        prop_assert!(
            encode_csri(&fp_rd, f3).is_err(),
            "FP dest {} must Err (not a GPR); got {:?}",
            fp,
            encode_csri(&fp_rd, f3)
        );
        let unknown_ops = [reg(&rd), csr_op(&unknown), Operand::Imm(zimm as i64)];
        prop_assert!(
            encode_csri(&unknown_ops, f3).is_err(),
            "unknown CSR {} must Err; got {:?}",
            unknown,
            encode_csri(&unknown_ops, f3)
        );
        let bad_rd = [reg(&bad), csr_op(csr_name), Operand::Imm(zimm as i64)];
        prop_assert!(
            encode_csri(&bad_rd, f3).is_err(),
            "invalid integer register {} as rd must Err; got {:?}",
            bad,
            encode_csri(&bad_rd, f3)
        );
        let non_imm_zimm = [reg(&rd), csr_op(csr_name), reg(&rd)];
        prop_assert!(
            encode_csri(&non_imm_zimm, f3).is_err(),
            "Reg as zimm must Err (expected immediate); got {:?}",
            encode_csri(&non_imm_zimm, f3)
        );
    }
}
