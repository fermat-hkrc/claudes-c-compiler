// Oracle: differential — llvm-mc RISC-V assembler (C.ADD CR-type halfword)
// Evidence: src/backend/riscv/assembler/README.md:13 C (compressed 16-bit);
//   README.md:108 compressed.rs RVC 16-bit instructions;
//   encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words";
//   encoder/mod.rs:107 EncodeResult::Half; encoder/mod.rs:927 "c.add" => encode_c_add;
//   compressed.rs:42 "c.add rd, rs2";
//   compress.rs:202 C.ADD: add rd, rd, rs2; compress.rs:200 rd != 0 && rs2 != 0;
//   RISC-V Unprivileged ISA C.ADD CR-type: [15:12]=1001, [11:7]=rd, [6:2]=rs2,
//   [1:0]=10. rs2≠x0 (rs2=x0 is C.JALR if rd≠0, C.EBREAK if rd=0);
//   rd=x0 with rs2≠x0 is HINT.
// Stronger considered:
//   - State machine: rejected — encode_c_add is a pure function with no lifecycle
//   - Algebraic round-trip via in-tree decoder: rejected — no C.ADD decoder
//   - try_compress_rv64 as differential sibling: rejected —
//     same-job gate (post-encode compress of 32-bit ADD, not c.add mnemonic)
// Weaker available: algebraic.invariant (CR-type field unpack), algebraic.metamorphic
//   (ABI vs xN alias, field isolation), negative_error (rs2=x0, extra, arity/FP)
// Differential: candidate=encode_c_add, reference=llvm-mc -triple=riscv64 -mattr=+c
//   -show-encoding, SUT-boundary=internal-helper of RISC-V assembler,
//   mapping=[Reg(rd), Reg(rs2)] <-> `c.add rd, rs2` with rs2 ≠ x0.

use super::{encode_c_add, EncodeResult};
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

/// Unpack C.ADD CR-type per RISC-V unprivileged ISA (not a copy of encode_c_add).
fn unpack_c_add(half: u16) -> (u16, u16, u32, u32) {
    let op = half & 0b11;
    let funct4 = (half >> 12) & 0b1111;
    let rd = ((half >> 7) & 0x1f) as u32;
    let rs2 = ((half >> 2) & 0x1f) as u32;
    (op, funct4, rd, rs2)
}

fn sut_half(ops: &[Operand]) -> Result<u16, String> {
    match encode_c_add(ops)? {
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

fn gpr() -> impl Strategy<Value = u32> {
    0u32..=31
}

fn gpr_nz() -> impl Strategy<Value = u32> {
    1u32..=31
}

fn gpr_name() -> impl Strategy<Value = String> {
    gpr().prop_flat_map(|n| {
        prop_oneof![
            Just(xn(n)),
            Just(abi_name(n).to_string()),
            Just(if n == 8 { "fp".into() } else { xn(n) }),
        ]
    })
}

fn gpr_nz_name() -> impl Strategy<Value = String> {
    gpr_nz().prop_flat_map(|n| {
        prop_oneof![
            Just(xn(n)),
            Just(abi_name(n).to_string()),
            Just(if n == 8 { "fp".into() } else { xn(n) }),
        ]
    })
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
        gpr_name().prop_map(|rd| vec![reg(&rd)]),
        gpr_nz().prop_map(|n| vec![Operand::Imm(n as i64)]),
    ]
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_c_add_kat_llvm_mc_t1_t0() {
    let want = 0x9316u16;
    let mc = llvm_mc_half("c.add t1, t0").expect("llvm-mc KAT t1,t0");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_half(&[reg("t1"), reg("t0")]).expect("SUT KAT t1,t0");
    assert_eq!(sut, want);
}

#[test]
fn encode_c_add_kat_llvm_mc_x1_x2() {
    let want = 0x908au16;
    let mc = llvm_mc_half("c.add x1, x2").expect("llvm-mc KAT x1,x2");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_half(&[reg("x1"), reg("x2")]).expect("SUT KAT x1,x2");
    assert_eq!(sut, want);
}

#[test]
fn encode_c_add_kat_llvm_mc_a0_a1() {
    let want = 0x952eu16;
    let mc = llvm_mc_half("c.add a0, a1").expect("llvm-mc KAT a0,a1");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_half(&[reg("a0"), reg("a1")]).expect("SUT KAT a0,a1");
    assert_eq!(sut, want);
}

#[test]
fn encode_c_add_kat_llvm_mc_x31_x31() {
    let want = 0x9ffeu16;
    let mc = llvm_mc_half("c.add x31, x31").expect("llvm-mc KAT x31,x31");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_half(&[reg("x31"), reg("x31")]).expect("SUT KAT x31,x31");
    assert_eq!(sut, want);
}

#[test]
fn encode_c_add_kat_llvm_mc_x1_x1() {
    let want = 0x9086u16;
    let mc = llvm_mc_half("c.add x1, x1").expect("llvm-mc KAT x1,x1");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_half(&[reg("x1"), reg("x1")]).expect("SUT KAT x1,x1");
    assert_eq!(sut, want);
}

#[test]
fn encode_c_add_kat_llvm_mc_sp_ra() {
    let want = 0x9106u16;
    let mc = llvm_mc_half("c.add sp, ra").expect("llvm-mc KAT sp,ra");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_half(&[reg("sp"), reg("ra")]).expect("SUT KAT sp,ra");
    assert_eq!(sut, want);
}

/// Regression: extra operand must be rejected (C.ADD is two-operand; llvm-mc errors).
#[test]
fn test_encode_c_add_regression_extra_operand() {
    let ops = [reg("x1"), reg("x2"), Operand::Imm(0)];
    assert!(
        encode_c_add(&ops).is_err(),
        "c.add x1, x2 with a third operand must Err; got {:?}",
        encode_c_add(&ops)
    );
}

/// Regression: rs2=x0 must be rejected (that encoding is C.JALR / C.EBREAK, not C.ADD).
#[test]
fn test_encode_c_add_regression_rs2_x0() {
    let ops = [reg("x1"), reg("x0")];
    assert!(
        encode_c_add(&ops).is_err(),
        "c.add x1, x0 must Err (rs2=x0 is C.JALR); got {:?}",
        encode_c_add(&ops)
    );
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_c_add_diff_llvm_mc(rd in gpr_name(), rs2 in gpr_nz_name()) {
        let asm = format!("c.add {}, {}", rd, rs2);
        let mc = llvm_mc_half(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_half(&[reg(&rd), reg(&rs2)])
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "SUT {:04x} != llvm-mc {:04x} for {}", sut, mc, asm);
    }

    #[test]
    fn encode_c_add_cr_type_fields(rd in gpr(), rs2 in gpr_nz()) {
        let h = sut_half(&[reg(&xn(rd)), reg(&xn(rs2))])
            .unwrap_or_else(|e| panic!("SUT rejected x{}, x{}: {}", rd, rs2, e));
        let (op, funct4, got_rd, got_rs2) = unpack_c_add(h);
        prop_assert_eq!(op, 0b10, "op");
        prop_assert_eq!(funct4, 0b1001, "funct4");
        prop_assert_eq!(got_rd, rd, "rd");
        prop_assert_eq!(got_rs2, rs2, "rs2");
    }

    #[test]
    fn encode_c_add_abi_xn_alias(n in gpr(), m in gpr_nz()) {
        let via_x = sut_half(&[reg(&xn(n)), reg(&xn(m))])
            .unwrap_or_else(|e| panic!("xN rejected: {}", e));
        let via_abi = sut_half(&[reg(abi_name(n)), reg(abi_name(m))])
            .unwrap_or_else(|e| panic!("ABI rejected: {}", e));
        prop_assert_eq!(via_abi, via_x);
        if n == 8 {
            let via_fp = sut_half(&[reg("fp"), reg(&xn(m))])
                .unwrap_or_else(|e| panic!("fp rd rejected: {}", e));
            prop_assert_eq!(via_fp, via_x);
        }
        if m == 8 {
            let via_fp = sut_half(&[reg(&xn(n)), reg("fp")])
                .unwrap_or_else(|e| panic!("fp rs2 rejected: {}", e));
            prop_assert_eq!(via_fp, via_x);
        }
    }

    #[test]
    fn encode_c_add_field_isolation(rd in gpr(), rs2_a in gpr_nz(), rs2_b in gpr_nz(), rd_a in gpr(), rd_b in gpr(), rs2 in gpr_nz()) {
        let ha = sut_half(&[reg(&xn(rd)), reg(&xn(rs2_a))])
            .unwrap_or_else(|e| panic!("rs2_a rejected: {}", e));
        let hb = sut_half(&[reg(&xn(rd)), reg(&xn(rs2_b))])
            .unwrap_or_else(|e| panic!("rs2_b rejected: {}", e));
        prop_assert_eq!((ha >> 7) & 0x1f, (hb >> 7) & 0x1f, "rd field must be independent of rs2");
        let ha2 = sut_half(&[reg(&xn(rd_a)), reg(&xn(rs2))])
            .unwrap_or_else(|e| panic!("rd_a rejected: {}", e));
        let hb2 = sut_half(&[reg(&xn(rd_b)), reg(&xn(rs2))])
            .unwrap_or_else(|e| panic!("rd_b rejected: {}", e));
        let rs2_mask = !0x0f80u16;
        prop_assert_eq!(ha2 & rs2_mask, hb2 & rs2_mask, "rs2/op/funct4 bits must be independent of rd");
    }

    #[test]
    fn encode_c_add_neg_rs2_x0(rd in gpr_name()) {
        let asm = format!("c.add {}, x0", rd);
        prop_assert!(
            llvm_mc_rejects(&asm),
            "reference unexpectedly accepted {}",
            asm
        );
        let ops_x0 = [reg(&rd), reg("x0")];
        prop_assert!(
            encode_c_add(&ops_x0).is_err(),
            "rs2=x0 must Err (llvm-mc rejects; encoding is C.JALR/C.EBREAK); got {:?}",
            encode_c_add(&ops_x0)
        );
        let ops_zero = [reg(&rd), reg("zero")];
        prop_assert!(
            encode_c_add(&ops_zero).is_err(),
            "rs2=zero must Err (llvm-mc rejects; encoding is C.JALR/C.EBREAK); got {:?}",
            encode_c_add(&ops_zero)
        );
    }

    #[test]
    fn encode_c_add_neg_extra(rd in gpr_name(), rs2 in gpr_nz_name(), extra in extra_operand()) {
        let asm = format!("c.add {}, {}", rd, rs2);
        let ops = vec![reg(&rd), reg(&rs2), extra];
        prop_assert!(
            encode_c_add(&ops).is_err(),
            "extra operand must Err for {} (llvm-mc rejects extra operands); got {:?}",
            asm,
            encode_c_add(&ops)
        );
    }

    #[test]
    fn encode_c_add_neg_arity_fp(ops in short_ops(), fp in fp_name(), gpr in gpr_nz_name()) {
        prop_assume!(ops.len() < 2);
        prop_assert!(
            encode_c_add(&ops).is_err(),
            "arity {} must Err, got {:?}",
            ops.len(),
            encode_c_add(&ops)
        );
        let fp_rd = [reg(&fp), reg(&gpr)];
        prop_assert!(
            encode_c_add(&fp_rd).is_err(),
            "FP dest {} must Err (llvm-mc invalid operand); got {:?}",
            fp,
            encode_c_add(&fp_rd)
        );
        let fp_rs2 = [reg(&gpr), reg(&fp)];
        prop_assert!(
            encode_c_add(&fp_rs2).is_err(),
            "FP src {} must Err (llvm-mc invalid operand); got {:?}",
            fp,
            encode_c_add(&fp_rs2)
        );
    }
}
