// Oracle: differential — llvm-mc RISC-V assembler (C.LUI CI-type halfword)
// Evidence: src/backend/riscv/assembler/README.md:13 C (compressed 16-bit);
//   README.md:108 compressed.rs RVC 16-bit instructions;
//   encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words";
//   encoder/mod.rs:101 EncodeResult::Half; encoder/mod.rs:915 "c.lui" => encode_c_lui;
//   compressed.rs:5 "c.lui rd, nzimm";
//   compress.rs:30 rd != {x0, x2}; compress.rs:41 signed 6-bit -32..31 excluding 0;
//   RISC-V Unprivileged ISA C.LUI CI-type: [15:13]=011, [12]=nzimm[17], [11:7]=rd,
//   [6:2]=nzimm[16:12], [1:0]=01.
// Stronger considered:
//   - State machine: rejected — encode_c_lui is a pure function with no lifecycle
//   - Algebraic round-trip via in-tree decoder: rejected — no C.LUI decoder
//   - encode_lui / try_compress_rv64 as differential sibling: rejected —
//     same-job gate (32-bit LUI / post-encode compress pass, not c.lui mnemonic)
// Weaker available: algebraic.invariant (CI-type field unpack), algebraic.metamorphic
//   (ABI vs xN alias), negative_error (x0/x2, nzimm=0, oob imm, extra, arity/FP)
// Differential: candidate=encode_c_lui, reference=llvm-mc -triple=riscv64 -mattr=+c
//   -show-encoding, SUT-boundary=internal-helper of RISC-V assembler,
//   mapping=[Reg(rd), Imm(imm)] <-> `c.lui rd, imm` with imm in
//   [1, 31] ∪ [0xfffe0, 0xfffff] (llvm-mc 20-bit LUI-style form of signed 6-bit nzimm).

use super::{encode_c_lui, EncodeResult};
use crate::backend::riscv::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;

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

/// Unpack C.LUI CI-type per RISC-V unprivileged ISA (not a copy of encode_c_lui).
fn unpack_c_lui(half: u16) -> (u16, u16, u32, i32) {
    let op = half & 0b11;
    let funct3 = (half >> 13) & 0b111;
    let rd = ((half >> 7) & 0x1f) as u32;
    let bit17 = ((half >> 12) & 1) as i32;
    let bits16_12 = ((half >> 2) & 0x1f) as i32;
    let mut nzimm = bits16_12 | (bit17 << 5);
    if bit17 != 0 {
        nzimm |= !0x3f; // sign-extend 6-bit
    }
    (op, funct3, rd, nzimm)
}

fn sut_half(ops: &[Operand]) -> Result<u16, String> {
    match encode_c_lui(ops)? {
        EncodeResult::Half(h) => Ok(h),
        other => Err(format!("expected Half, got {other:?}")),
    }
}

fn parse_llvm_encoding(stdout: &str) -> Result<u16, String> {
    let marker = "encoding: [";
    let start = stdout
        .find(marker)
        .ok_or_else(|| format!("no encoding in stdout: {stdout}"))?;
    let rest = &stdout[start + marker.len()..];
    let end = rest
        .find(']')
        .ok_or_else(|| format!("no closing bracket: {stdout}"))?;
    let inner = &rest[..end];
    let parts: Vec<&str> = inner.split(',').collect();
    if parts.len() != 2 {
        return Err(format!("expected 2 bytes, got {inner}"));
    }
    let mut bytes = [0u8; 2];
    for (i, p) in parts.iter().enumerate() {
        let p = p.trim();
        let hex = p
            .strip_prefix("0x")
            .ok_or_else(|| format!("non-hex byte {p}"))?;
        bytes[i] = u8::from_str_radix(hex, 16).map_err(|e| e.to_string())?;
    }
    Ok(u16::from_le_bytes(bytes))
}

fn llvm_mc_half(asm: &str) -> Result<u16, String> {
    let mut child = Command::new(LLVM_MC)
        .args(["-triple=riscv64", "-mattr=+c", "-show-encoding"])
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
    llvm_mc_half(asm).is_err()
}

fn valid_rd() -> impl Strategy<Value = u32> {
    (1u32..=31).prop_filter("rd != x2", |n| *n != 2)
}

fn gpr_name_valid() -> impl Strategy<Value = String> {
    valid_rd().prop_flat_map(|n| {
        prop_oneof![
            Just(xn(n)),
            Just(abi_name(n).to_string()),
            Just(if n == 8 { "fp".into() } else { xn(n) }),
        ]
    })
}

/// llvm-mc C.LUI immediate: [1, 31] ∪ [0xfffe0, 0xfffff].
fn clui_imm20() -> impl Strategy<Value = i64> {
    prop_oneof![
        Just(1i64),
        Just(31i64),
        Just(0xfffe0_i64),
        Just(0xfffff_i64),
        1i64..=31,
        0xfffe0_i64..=0xfffff_i64,
    ]
}

/// Signed 6-bit nzimm excluding 0, as used by the ISA / compressor.
fn simm6_nz() -> impl Strategy<Value = i32> {
    prop_oneof![
        Just(1i32),
        Just(31i32),
        Just(-1i32),
        Just(-32i32),
        (1i32..=31),
        (-32i32..=-1),
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

fn oob_imm() -> impl Strategy<Value = i64> {
    prop_oneof![
        Just(32i64),
        Just(33i64),
        Just(-33i64),
        Just(1048543i64),
        Just(1048576i64),
        Just(i64::MIN),
        Just(i64::MAX),
        Just(0x100000i64),
        32i64..=1048543,
        1048576i64..=i64::MAX,
        i64::MIN..=-33,
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

fn short_ops() -> impl Strategy<Value = Vec<Operand>> {
    prop_oneof![
        Just(vec![]),
        gpr_name_valid().prop_map(|rd| vec![reg(&rd)]),
        clui_imm20().prop_map(|i| vec![Operand::Imm(i)]),
    ]
}

fn rd_x0_x2() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("x0".into()),
        Just("zero".into()),
        Just("x2".into()),
        Just("sp".into()),
    ]
}

fn is_clui_imm20(imm: i64) -> bool {
    (1..=31).contains(&imm) || (0xfffe0..=0xfffff).contains(&imm)
}

fn is_simm6_nz(imm: i64) -> bool {
    let v = imm as i32;
    v as i64 == imm && v != 0 && (-32..=31).contains(&v)
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_c_lui_kat_llvm_mc_x1_1() {
    let want = 0x6085u16;
    let mc = llvm_mc_half("c.lui x1, 1").expect("llvm-mc KAT x1,1");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_half(&[reg("x1"), Operand::Imm(1)]).expect("SUT KAT x1,1");
    assert_eq!(sut, want);
}

#[test]
fn encode_c_lui_kat_llvm_mc_x1_31() {
    let want = 0x60fdu16;
    let mc = llvm_mc_half("c.lui x1, 31").expect("llvm-mc KAT x1,31");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_half(&[reg("x1"), Operand::Imm(31)]).expect("SUT KAT x1,31");
    assert_eq!(sut, want);
}

#[test]
fn encode_c_lui_kat_llvm_mc_x1_n32() {
    let want = 0x7081u16;
    let mc = llvm_mc_half("c.lui x1, 1048544").expect("llvm-mc KAT 0xfffe0");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_half(&[reg("x1"), Operand::Imm(1048544)]).expect("SUT KAT 0xfffe0");
    assert_eq!(sut, want);
}

#[test]
fn encode_c_lui_kat_llvm_mc_x1_n1() {
    let want = 0x70fdu16;
    let mc = llvm_mc_half("c.lui x1, 1048575").expect("llvm-mc KAT 0xfffff");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_half(&[reg("x1"), Operand::Imm(1048575)]).expect("SUT KAT 0xfffff");
    assert_eq!(sut, want);
}

#[test]
fn encode_c_lui_kat_llvm_mc_a0_1() {
    let want = 0x6505u16;
    let mc = llvm_mc_half("c.lui a0, 1").expect("llvm-mc KAT a0,1");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_half(&[reg("a0"), Operand::Imm(1)]).expect("SUT KAT a0,1");
    assert_eq!(sut, want);
}

#[test]
fn encode_c_lui_kat_llvm_mc_x31_1() {
    let want = 0x6f85u16;
    let mc = llvm_mc_half("c.lui x31, 1").expect("llvm-mc KAT x31,1");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_half(&[reg("x31"), Operand::Imm(1)]).expect("SUT KAT x31,1");
    assert_eq!(sut, want);
}

/// Regression: extra operand must be rejected (C.LUI is two-operand; llvm-mc errors).
#[test]
fn test_encode_c_lui_regression_extra_operand() {
    let ops = [reg("x3"), Operand::Imm(1), Operand::Imm(0)];
    assert!(
        encode_c_lui(&ops).is_err(),
        "c.lui x3, 1 with a third operand must Err; got {:?}",
        encode_c_lui(&ops)
    );
}

/// Regression: out-of-range C.LUI immediate must be rejected (32 wraps to -32 today).
#[test]
fn test_encode_c_lui_regression_imm_oob() {
    let ops = [reg("x3"), Operand::Imm(32)];
    assert!(
        encode_c_lui(&ops).is_err(),
        "c.lui x3, 32 must Err (outside simm6 and llvm-mc [1,31]∪[0xfffe0,0xfffff]); got {:?}",
        encode_c_lui(&ops)
    );
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_c_lui_diff_llvm_mc(rd in gpr_name_valid(), imm in clui_imm20()) {
        let asm = format!("c.lui {}, {}", rd, imm);
        let mc = llvm_mc_half(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_half(&[reg(&rd), Operand::Imm(imm)])
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "SUT {:04x} != llvm-mc {:04x} for {}", sut, mc, asm);
    }

    #[test]
    fn encode_c_lui_ci_type_fields(rd in valid_rd(), nzimm in simm6_nz()) {
        prop_assume!(nzimm != 0);
        let h = sut_half(&[reg(&xn(rd)), Operand::Imm(nzimm as i64)])
            .unwrap_or_else(|e| panic!("SUT rejected x{}, {}: {}", rd, nzimm, e));
        let (op, funct3, got_rd, got_nz) = unpack_c_lui(h);
        prop_assert_eq!(op, 0b01, "op");
        prop_assert_eq!(funct3, 0b011, "funct3");
        prop_assert_eq!(got_rd, rd, "rd");
        prop_assert_eq!(got_nz, nzimm, "nzimm");
    }

    #[test]
    fn encode_c_lui_abi_xn_alias(n in valid_rd(), imm in clui_imm20()) {
        let via_x = sut_half(&[reg(&xn(n)), Operand::Imm(imm)])
            .unwrap_or_else(|e| panic!("xN rejected: {}", e));
        let via_abi = sut_half(&[reg(abi_name(n)), Operand::Imm(imm)])
            .unwrap_or_else(|e| panic!("ABI rejected: {}", e));
        prop_assert_eq!(via_abi, via_x);
        if n == 8 {
            let via_fp = sut_half(&[reg("fp"), Operand::Imm(imm)])
                .unwrap_or_else(|e| panic!("fp rejected: {}", e));
            prop_assert_eq!(via_fp, via_x);
        }
    }

    #[test]
    fn encode_c_lui_signed_vs_uimm20(rd in valid_rd(), nzimm in -32i32..=-1) {
        let uimm20 = (nzimm as i64) & 0xfffff;
        let via_signed = sut_half(&[reg(&xn(rd)), Operand::Imm(nzimm as i64)])
            .unwrap_or_else(|e| panic!("signed {} rejected: {}", nzimm, e));
        let via_uimm = sut_half(&[reg(&xn(rd)), Operand::Imm(uimm20)])
            .unwrap_or_else(|e| panic!("uimm20 {} rejected: {}", uimm20, e));
        prop_assert_eq!(via_signed, via_uimm, "signed {} vs uimm20 {}", nzimm, uimm20);
        let asm = format!("c.lui x{}, {}", rd, uimm20);
        let mc = llvm_mc_half(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected {}: {}", asm, e));
        prop_assert_eq!(via_signed, mc, "SUT signed {:04x} != llvm-mc {:04x} for {}", via_signed, mc, asm);
    }

    #[test]
    fn encode_c_lui_neg_rd_x0_x2(rd in rd_x0_x2(), imm in 1i64..=31) {
        let ops = [reg(&rd), Operand::Imm(imm)];
        prop_assert!(
            encode_c_lui(&ops).is_err(),
            "c.lui {}, {} must Err (rd cannot be x0 or x2); got {:?}",
            rd,
            imm,
            encode_c_lui(&ops)
        );
    }

    #[test]
    fn encode_c_lui_neg_nzimm_zero(rd in gpr_name_valid()) {
        let asm = format!("c.lui {}, 0", rd);
        prop_assert!(llvm_mc_rejects(&asm), "reference unexpectedly accepted {}", asm);
        let ops = [reg(&rd), Operand::Imm(0)];
        prop_assert!(
            encode_c_lui(&ops).is_err(),
            "c.lui {}, 0 must Err (nzimm must not be zero); got {:?}",
            rd,
            encode_c_lui(&ops)
        );
    }

    #[test]
    fn encode_c_lui_neg_imm_oob(rd in gpr_name_valid(), imm in oob_imm()) {
        prop_assume!(!is_clui_imm20(imm));
        prop_assume!(!is_simm6_nz(imm));
        let asm = format!("c.lui {}, {}", rd, imm);
        prop_assert!(
            llvm_mc_rejects(&asm),
            "reference unexpectedly accepted {}",
            asm
        );
        let ops = [reg(&rd), Operand::Imm(imm)];
        prop_assert!(
            encode_c_lui(&ops).is_err(),
            "oob imm {} must Err (llvm-mc range [1,31]∪[0xfffe0,0xfffff]; ISA simm6); got {:?}",
            imm,
            encode_c_lui(&ops)
        );
    }

    #[test]
    fn encode_c_lui_neg_extra(rd in gpr_name_valid(), imm in 1i64..=31, extra in extra_operand()) {
        let asm = format!("c.lui {}, {}", rd, imm);
        let ops = vec![reg(&rd), Operand::Imm(imm), extra];
        prop_assert!(
            encode_c_lui(&ops).is_err(),
            "extra operand must Err for {} (llvm-mc rejects extra operands); got {:?}",
            asm,
            encode_c_lui(&ops)
        );
    }

    #[test]
    fn encode_c_lui_neg_arity_fp(ops in short_ops(), fp in fp_name(), imm in 1i64..=31) {
        prop_assume!(ops.len() < 2);
        prop_assert!(
            encode_c_lui(&ops).is_err(),
            "arity {} must Err, got {:?}",
            ops.len(),
            encode_c_lui(&ops)
        );
        let fp_ops = [reg(&fp), Operand::Imm(imm)];
        prop_assert!(
            encode_c_lui(&fp_ops).is_err(),
            "FP dest {} must Err (llvm-mc invalid operand); got {:?}",
            fp,
            encode_c_lui(&fp_ops)
        );
    }
}
