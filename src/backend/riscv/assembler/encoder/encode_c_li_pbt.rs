// Oracle: differential — llvm-mc RISC-V assembler (C.LI CI-type halfword)
// Evidence: src/backend/riscv/assembler/README.md:13 C (compressed 16-bit);
//   README.md:108 compressed.rs RVC 16-bit instructions;
//   encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words";
//   encoder/mod.rs:101 EncodeResult::Half; encoder/mod.rs:918 "c.li" => encode_c_li;
//   compressed.rs:17 "c.li rd, imm";
//   compress.rs:97 C.LI: addi rd, x0, imm; compress.rs:99 signed 6-bit -32..=31;
//   RISC-V Unprivileged ISA C.LI CI-type: [15:13]=010, [12]=imm[5], [11:7]=rd,
//   [6:2]=imm[4:0], [1:0]=01.
// Stronger considered:
//   - State machine: rejected — encode_c_li is a pure function with no lifecycle
//   - Algebraic round-trip via in-tree decoder: rejected — no C.LI decoder
//   - try_compress_rv64 as differential sibling: rejected —
//     same-job gate (post-encode compress of 32-bit ADDI, not c.li mnemonic)
// Weaker available: algebraic.invariant (CI-type field unpack), algebraic.metamorphic
//   (ABI vs xN alias, field isolation), negative_error (oob imm, extra, arity/FP)
// Differential: candidate=encode_c_li, reference=llvm-mc -triple=riscv64 -mattr=+c
//   -show-encoding, SUT-boundary=internal-helper of RISC-V assembler,
//   mapping=[Reg(rd), Imm(imm)] <-> `c.li rd, imm` with imm in [-32, 31].

use super::{encode_c_li, EncodeResult};
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

/// Unpack C.LI CI-type per RISC-V unprivileged ISA (not a copy of encode_c_li).
fn unpack_c_li(half: u16) -> (u16, u16, u32, i32) {
    let op = half & 0b11;
    let funct3 = (half >> 13) & 0b111;
    let rd = ((half >> 7) & 0x1f) as u32;
    let bit5 = ((half >> 12) & 1) as i32;
    let bits4_0 = ((half >> 2) & 0x1f) as i32;
    let mut imm = bits4_0 | (bit5 << 5);
    if bit5 != 0 {
        imm |= !0x3f; // sign-extend 6-bit
    }
    (op, funct3, rd, imm)
}

fn sut_half(ops: &[Operand]) -> Result<u16, String> {
    match encode_c_li(ops)? {
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

fn gpr_name() -> impl Strategy<Value = String> {
    gpr().prop_flat_map(|n| {
        prop_oneof![
            Just(xn(n)),
            Just(abi_name(n).to_string()),
            Just(if n == 8 { "fp".into() } else { xn(n) }),
        ]
    })
}

/// Signed 6-bit immediate, as used by the ISA / llvm-mc C.LI.
fn simm6() -> impl Strategy<Value = i64> {
    prop_oneof![
        Just(0i64),
        Just(1i64),
        Just(31i64),
        Just(-1i64),
        Just(-32i64),
        -32i64..=31,
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
        Just(-34i64),
        Just(i64::MIN),
        Just(i64::MAX),
        Just(0x100000i64),
        Just(1048575i64),
        32i64..=i64::MAX,
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
        gpr_name().prop_map(|rd| vec![reg(&rd)]),
        simm6().prop_map(|i| vec![Operand::Imm(i)]),
    ]
}

fn is_simm6(imm: i64) -> bool {
    (-32..=31).contains(&imm)
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_c_li_kat_llvm_mc_x1_0() {
    let want = 0x4081u16;
    let mc = llvm_mc_half("c.li x1, 0").expect("llvm-mc KAT x1,0");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_half(&[reg("x1"), Operand::Imm(0)]).expect("SUT KAT x1,0");
    assert_eq!(sut, want);
}

#[test]
fn encode_c_li_kat_llvm_mc_x1_1() {
    let want = 0x4085u16;
    let mc = llvm_mc_half("c.li x1, 1").expect("llvm-mc KAT x1,1");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_half(&[reg("x1"), Operand::Imm(1)]).expect("SUT KAT x1,1");
    assert_eq!(sut, want);
}

#[test]
fn encode_c_li_kat_llvm_mc_x1_31() {
    let want = 0x40fdu16;
    let mc = llvm_mc_half("c.li x1, 31").expect("llvm-mc KAT x1,31");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_half(&[reg("x1"), Operand::Imm(31)]).expect("SUT KAT x1,31");
    assert_eq!(sut, want);
}

#[test]
fn encode_c_li_kat_llvm_mc_x1_n1() {
    let want = 0x50fdu16;
    let mc = llvm_mc_half("c.li x1, -1").expect("llvm-mc KAT x1,-1");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_half(&[reg("x1"), Operand::Imm(-1)]).expect("SUT KAT x1,-1");
    assert_eq!(sut, want);
}

#[test]
fn encode_c_li_kat_llvm_mc_x1_n32() {
    let want = 0x5081u16;
    let mc = llvm_mc_half("c.li x1, -32").expect("llvm-mc KAT x1,-32");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_half(&[reg("x1"), Operand::Imm(-32)]).expect("SUT KAT x1,-32");
    assert_eq!(sut, want);
}

#[test]
fn encode_c_li_kat_llvm_mc_a0_5() {
    let want = 0x4515u16;
    let mc = llvm_mc_half("c.li a0, 5").expect("llvm-mc KAT a0,5");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_half(&[reg("a0"), Operand::Imm(5)]).expect("SUT KAT a0,5");
    assert_eq!(sut, want);
}

/// Regression: extra operand must be rejected (C.LI is two-operand; llvm-mc errors).
#[test]
fn test_encode_c_li_regression_extra_operand() {
    let ops = [reg("x3"), Operand::Imm(1), Operand::Imm(0)];
    assert!(
        encode_c_li(&ops).is_err(),
        "c.li x3, 1 with a third operand must Err; got {:?}",
        encode_c_li(&ops)
    );
}

/// Regression: out-of-range C.LI immediate must be rejected (32 wraps to -32 today).
#[test]
fn test_encode_c_li_regression_imm_oob() {
    let ops = [reg("x3"), Operand::Imm(32)];
    assert!(
        encode_c_li(&ops).is_err(),
        "c.li x3, 32 must Err (outside simm6 [-32, 31]); got {:?}",
        encode_c_li(&ops)
    );
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_c_li_diff_llvm_mc(rd in gpr_name(), imm in simm6()) {
        let asm = format!("c.li {}, {}", rd, imm);
        let mc = llvm_mc_half(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_half(&[reg(&rd), Operand::Imm(imm)])
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "SUT {:04x} != llvm-mc {:04x} for {}", sut, mc, asm);
    }

    #[test]
    fn encode_c_li_ci_type_fields(rd in gpr(), imm in -32i32..=31) {
        let h = sut_half(&[reg(&xn(rd)), Operand::Imm(imm as i64)])
            .unwrap_or_else(|e| panic!("SUT rejected x{}, {}: {}", rd, imm, e));
        let (op, funct3, got_rd, got_imm) = unpack_c_li(h);
        prop_assert_eq!(op, 0b01, "op");
        prop_assert_eq!(funct3, 0b010, "funct3");
        prop_assert_eq!(got_rd, rd, "rd");
        prop_assert_eq!(got_imm, imm, "imm");
    }

    #[test]
    fn encode_c_li_abi_xn_alias(n in gpr(), imm in simm6()) {
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
    fn encode_c_li_imm_isolation(rd in gpr(), imm_a in simm6(), imm_b in simm6(), rd_a in gpr(), rd_b in gpr(), imm in simm6()) {
        let ha = sut_half(&[reg(&xn(rd)), Operand::Imm(imm_a)])
            .unwrap_or_else(|e| panic!("imm_a rejected: {}", e));
        let hb = sut_half(&[reg(&xn(rd)), Operand::Imm(imm_b)])
            .unwrap_or_else(|e| panic!("imm_b rejected: {}", e));
        prop_assert_eq!((ha >> 7) & 0x1f, (hb >> 7) & 0x1f, "rd field must be independent of imm");
        let ha2 = sut_half(&[reg(&xn(rd_a)), Operand::Imm(imm)])
            .unwrap_or_else(|e| panic!("rd_a rejected: {}", e));
        let hb2 = sut_half(&[reg(&xn(rd_b)), Operand::Imm(imm)])
            .unwrap_or_else(|e| panic!("rd_b rejected: {}", e));
        let imm_mask = !0x0f80u16;
        prop_assert_eq!(ha2 & imm_mask, hb2 & imm_mask, "imm/op/funct3 bits must be independent of rd");
    }

    #[test]
    fn encode_c_li_neg_imm_oob(rd in gpr_name(), imm in oob_imm()) {
        prop_assume!(!is_simm6(imm));
        let asm = format!("c.li {}, {}", rd, imm);
        prop_assert!(
            llvm_mc_rejects(&asm),
            "reference unexpectedly accepted {}",
            asm
        );
        let ops = [reg(&rd), Operand::Imm(imm)];
        prop_assert!(
            encode_c_li(&ops).is_err(),
            "oob imm {} must Err (llvm-mc range [-32, 31]); got {:?}",
            imm,
            encode_c_li(&ops)
        );
    }

    #[test]
    fn encode_c_li_neg_extra(rd in gpr_name(), imm in simm6(), extra in extra_operand()) {
        let asm = format!("c.li {}, {}", rd, imm);
        let ops = vec![reg(&rd), Operand::Imm(imm), extra];
        prop_assert!(
            encode_c_li(&ops).is_err(),
            "extra operand must Err for {} (llvm-mc rejects extra operands); got {:?}",
            asm,
            encode_c_li(&ops)
        );
    }

    #[test]
    fn encode_c_li_neg_arity_fp(ops in short_ops(), fp in fp_name(), imm in simm6()) {
        prop_assume!(ops.len() < 2);
        prop_assert!(
            encode_c_li(&ops).is_err(),
            "arity {} must Err, got {:?}",
            ops.len(),
            encode_c_li(&ops)
        );
        let fp_ops = [reg(&fp), Operand::Imm(imm)];
        prop_assert!(
            encode_c_li(&fp_ops).is_err(),
            "FP dest {} must Err (llvm-mc invalid operand); got {:?}",
            fp,
            encode_c_li(&fp_ops)
        );
    }
}
