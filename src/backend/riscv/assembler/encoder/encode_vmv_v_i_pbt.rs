// Oracle: differential — llvm-mc RISC-V assembler (RVV vmv.v.i)
// Evidence: vector.rs:171-176 "vmv.v.i vd, simm5: OPIVI, funct6=010111, vm=1, vs2=0";
//   encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words";
//   encoder/mod.rs:1008 "vmv.v.i" => encode_vmv_v_i(operands) (operands passed through);
//   assembler/README.md:14 V (vector) standard extension; assembler/README.md:109 vector.rs RVV;
//   RISC-V V 1.0 vmv.v.i: opcode=1010111, funct3=011 (OPIVI), vd[11:7], simm5[19:15] signed 5-bit,
//   vs2[24:20]=00000, vm[25]=1, funct6[31:26]=010111.
// Stronger considered:
//   - State machine: rejected — encode_vmv_v_i is a pure function with no lifecycle
//   - Algebraic round-trip via in-tree decoder: rejected — no vmv.v.i decoder
//   - encode_vmv_v_v / encode_vmv_v_x / encode_v_arith_vi as differential siblings:
//     rejected — same-job gate (OPIVV funct3=000 / OPIVX funct3=100 /
//     3-operand OPIVI with vs2 in bits[24:20])
// Weaker available: algebraic.invariant (field unpack, simm5 two's complement),
//   algebraic.metamorphic (vd/simm5 isolation), negative_error (arity / bad regs / extra / v0.t / oob)
// Differential: candidate=encode_vmv_v_i,
//   reference=llvm-mc -triple=riscv64 -mattr=+v -show-encoding,
//   SUT-boundary=internal-helper of RISC-V assembler,
//   mapping=[Reg(vd), Imm(simm5)] <-> `vmv.v.i vd, simm5`.

use super::{encode_vmv_v_i, EncodeResult};
use crate::backend::riscv::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;
const OP_V: u32 = 0b1010111;
const FUNCT6_VMV: u32 = 0b010111;
const FUNCT3_OPIVI: u32 = 0b011;

fn cfg() -> ProptestConfig {
    ProptestConfig::with_cases(CASES)
}

fn vn(n: u32) -> String {
    format!("v{n}")
}

fn xn(n: u32) -> String {
    format!("x{n}")
}

fn vreg(n: u32) -> Operand {
    Operand::Reg(vn(n))
}

fn ops2(vd: u32, simm: i64) -> Vec<Operand> {
    vec![vreg(vd), Operand::Imm(simm)]
}

fn sut_word(ops: &[Operand]) -> Result<u32, String> {
    match encode_vmv_v_i(ops)? {
        EncodeResult::Word(w) => Ok(w),
        other => Err(format!("expected Word, got {other:?}")),
    }
}

/// Unpack OPIVI vmv.v.i per RISC-V V 1.0 (not a copy of encode_vmv_v_i).
fn unpack_vmv_v_i(word: u32) -> (u32, u32, u32, u32, u32, u32, u32) {
    let opcode = word & 0x7f;
    let vd = (word >> 7) & 0x1f;
    let funct3 = (word >> 12) & 0x7;
    let simm5 = (word >> 15) & 0x1f;
    let vs2 = (word >> 20) & 0x1f;
    let vm = (word >> 25) & 1;
    let funct6 = word >> 26;
    (opcode, vd, funct3, simm5, vs2, vm, funct6)
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
        let hex = p
            .strip_prefix("0x")
            .ok_or_else(|| format!("non-hex byte {p}"))?;
        bytes[i] = u8::from_str_radix(hex, 16).map_err(|e| e.to_string())?;
    }
    Ok(u32::from_le_bytes(bytes))
}

fn llvm_mc_word(asm: &str) -> Result<u32, String> {
    let mut child = Command::new(LLVM_MC)
        .args(["-triple=riscv64", "-mattr=+v", "-show-encoding"])
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

fn vreg_n() -> impl Strategy<Value = u32> {
    0u32..=31
}

fn simm5() -> impl Strategy<Value = i64> {
    prop_oneof![
        Just(-16i64),
        Just(-1i64),
        Just(0i64),
        Just(15i64),
        -16i64..=15,
    ]
}

fn extra_operand() -> impl Strategy<Value = Operand> {
    prop_oneof![
        Just(Operand::Imm(0)),
        Just(Operand::Imm(-1)),
        Just(Operand::Imm(208)),
        (0u32..=31).prop_map(|n| Operand::Reg(xn(n))),
        (0u32..=31).prop_map(|n| Operand::Reg(vn(n))),
        Just(Operand::Symbol("foo".into())),
        Just(Operand::Symbol("v0.t".into())),
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

fn bad_vreg() -> impl Strategy<Value = Operand> {
    prop_oneof![
        (0u32..=31).prop_map(|n| Operand::Reg(xn(n))),
        Just(Operand::Reg("a0".into())),
        Just(Operand::Reg("fa0".into())),
        Just(Operand::Reg("ft0".into())),
        Just(Operand::Reg("f0".into())),
        Just(Operand::Reg("zero".into())),
        Just(Operand::Reg("sp".into())),
        Just(Operand::Reg("v32".into())),
        Just(Operand::Imm(0)),
        Just(Operand::Imm(31)),
        Just(Operand::Symbol("foo".into())),
        Just(Operand::Label("L0".into())),
        Just(Operand::Mem {
            base: "sp".into(),
            offset: 0,
        }),
        Just(Operand::FenceArg("iorw".into())),
        Just(Operand::Csr("mstatus".into())),
        Just(Operand::RoundingMode("rne".into())),
    ]
}

fn bad_imm() -> impl Strategy<Value = Operand> {
    prop_oneof![
        (0u32..=31).prop_map(|n| Operand::Reg(vn(n))),
        (0u32..=31).prop_map(|n| Operand::Reg(xn(n))),
        Just(Operand::Reg("a0".into())),
        Just(Operand::Reg("fa0".into())),
        Just(Operand::Symbol("foo".into())),
        Just(Operand::Label("L0".into())),
        Just(Operand::Mem {
            base: "sp".into(),
            offset: 0,
        }),
        Just(Operand::FenceArg("iorw".into())),
        Just(Operand::Csr("mstatus".into())),
        Just(Operand::RoundingMode("rne".into())),
        Just(Operand::SymbolOffset("foo".into(), 4)),
    ]
}

fn short_ops() -> impl Strategy<Value = Vec<Operand>> {
    prop_oneof![Just(vec![]), vreg_n().prop_map(|vd| vec![vreg(vd)]),]
}

fn signed_oob() -> impl Strategy<Value = i64> {
    prop_oneof![
        Just(16i64),
        Just(-17i64),
        Just(31i64),
        Just(-32i64),
        Just(i64::MAX),
        Just(i64::MIN),
        16i64..=1024,
        -1024i64..=-17,
    ]
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_vmv_v_i_kat_llvm_mc_v0_0() {
    let want = 0x5e003057u32;
    let mc = llvm_mc_word("vmv.v.i v0, 0").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT drifted: {mc:#x} != {want:#x}");
    let sut = sut_word(&ops2(0, 0)).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_vmv_v_i_kat_llvm_mc_v1_2() {
    let want = 0x5e0130d7u32;
    let mc = llvm_mc_word("vmv.v.i v1, 2").expect("llvm-mc KAT");
    assert_eq!(mc, want);
    let sut = sut_word(&ops2(1, 2)).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_vmv_v_i_kat_llvm_mc_v31_15() {
    let want = 0x5e07bfd7u32;
    let mc = llvm_mc_word("vmv.v.i v31, 15").expect("llvm-mc KAT");
    assert_eq!(mc, want);
    let sut = sut_word(&ops2(31, 15)).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_vmv_v_i_kat_llvm_mc_v1_m1() {
    let want = 0x5e0fb0d7u32;
    let mc = llvm_mc_word("vmv.v.i v1, -1").expect("llvm-mc KAT");
    assert_eq!(mc, want);
    let sut = sut_word(&ops2(1, -1)).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_vmv_v_i_kat_llvm_mc_v1_m16() {
    let want = 0x5e0830d7u32;
    let mc = llvm_mc_word("vmv.v.i v1, -16").expect("llvm-mc KAT");
    assert_eq!(mc, want);
    let sut = sut_word(&ops2(1, -16)).expect("SUT KAT");
    assert_eq!(sut, want);
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_vmv_v_i_diff_llvm_mc(vd in vreg_n(), simm in simm5()) {
        let asm = format!("vmv.v.i v{vd}, {simm}");
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {asm}: {e}"));
        let sut = sut_word(&ops2(vd, simm))
            .unwrap_or_else(|e| panic!("SUT rejected valid {asm}: {e}"));
        prop_assert_eq!(sut, mc, "SUT {:#010x} != llvm-mc {:#010x} for {}", sut, mc, asm);
    }

    #[test]
    fn encode_vmv_v_i_format_fields(vd in vreg_n(), simm in simm5()) {
        let w = sut_word(&ops2(vd, simm))
            .unwrap_or_else(|e| panic!("SUT rejected: {e}"));
        let (opcode, got_vd, funct3, got_simm5, vs2, vm, funct6) = unpack_vmv_v_i(w);
        prop_assert_eq!(opcode, OP_V, "opcode");
        prop_assert_eq!(got_vd, vd, "vd");
        prop_assert_eq!(funct3, FUNCT3_OPIVI, "funct3 must be 011 (OPIVI)");
        prop_assert_eq!(got_simm5, (simm as u32) & 0x1F, "simm5");
        prop_assert_eq!(vs2, 0, "vs2 must be 0 for vmv.v.i");
        prop_assert_eq!(vm, 1, "vm must be 1 (unmasked)");
        prop_assert_eq!(funct6, FUNCT6_VMV, "funct6 must be 010111");
    }

    #[test]
    fn encode_vmv_v_i_field_isolation(
        vd_a in vreg_n(),
        vd_b in vreg_n(),
        simm_a in simm5(),
        simm_b in simm5(),
    ) {
        let wa = sut_word(&ops2(vd_a, simm_a))
            .unwrap_or_else(|e| panic!("a rejected: {e}"));
        let wb = sut_word(&ops2(vd_b, simm_a))
            .unwrap_or_else(|e| panic!("b rejected: {e}"));
        let vd_mask = 0x1fu32 << 7;
        prop_assert_eq!(wa & !vd_mask, wb & !vd_mask, "non-vd bits independent of vd");
        prop_assert_eq!((wa >> 7) & 0x1f, vd_a);
        prop_assert_eq!((wb >> 7) & 0x1f, vd_b);

        let wc = sut_word(&ops2(vd_a, simm_b))
            .unwrap_or_else(|e| panic!("c rejected: {e}"));
        let simm_mask = 0x1fu32 << 15;
        prop_assert_eq!(wa & !simm_mask, wc & !simm_mask, "non-simm5 bits independent of simm5");
        prop_assert_eq!((wc >> 15) & 0x1f, (simm_b as u32) & 0x1F);
    }

    #[test]
    fn encode_vmv_v_i_simm5_twos_complement(vd in vreg_n(), simm in simm5()) {
        let w = sut_word(&ops2(vd, simm))
            .unwrap_or_else(|e| panic!("SUT rejected: {e}"));
        prop_assert_eq!(
            (w >> 15) & 0x1f,
            (simm as u32) & 0x1F,
            "simm5 two's complement bits[19:15] for {}",
            simm
        );
    }

    #[test]
    fn encode_vmv_v_i_neg_arity_bad_regs(
        ops in short_ops(),
        bad_v in bad_vreg(),
        bad_i in bad_imm(),
        pos in 0usize..=1,
    ) {
        prop_assert!(
            encode_vmv_v_i(&ops).is_err(),
            "arity {} must Err (llvm-mc too few operands); got {:?}",
            ops.len(),
            encode_vmv_v_i(&ops)
        );
        if pos == 0 {
            let mut bad_ops = ops2(0, 0);
            bad_ops[0] = bad_v.clone();
            prop_assert!(
                encode_vmv_v_i(&bad_ops).is_err(),
                "non-vector operand {:?} at vd must Err (llvm-mc invalid operand); got {:?}",
                bad_v,
                encode_vmv_v_i(&bad_ops)
            );
        } else {
            let mut bad_ops = ops2(0, 0);
            bad_ops[1] = bad_i.clone();
            prop_assert!(
                encode_vmv_v_i(&bad_ops).is_err(),
                "non-Imm operand {:?} at simm5 must Err (llvm-mc invalid operand); got {:?}",
                bad_i,
                encode_vmv_v_i(&bad_ops)
            );
        }
    }

    #[test]
    fn encode_vmv_v_i_neg_extra(
        vd in vreg_n(),
        simm in simm5(),
        extra in extra_operand(),
    ) {
        let mut ops = ops2(vd, simm);
        ops.push(extra.clone());
        prop_assert!(
            encode_vmv_v_i(&ops).is_err(),
            "extra operand {:?} must Err for vmv.v.i (llvm-mc rejects extra); got {:?}",
            extra,
            encode_vmv_v_i(&ops)
        );
    }

    #[test]
    fn encode_vmv_v_i_neg_mask_v0t(vd in vreg_n(), simm in simm5()) {
        let mut ops = ops2(vd, simm);
        ops.push(Operand::Symbol("v0.t".into()));
        prop_assert!(
            encode_vmv_v_i(&ops).is_err(),
            "trailing v0.t must Err for vmv.v.i (llvm-mc rejects mask on vmv.v.i); got {:?}",
            encode_vmv_v_i(&ops)
        );
    }

    #[test]
    fn encode_vmv_v_i_neg_imm_oob(vd in vreg_n(), imm in signed_oob()) {
        let ops = ops2(vd, imm);
        prop_assert!(
            encode_vmv_v_i(&ops).is_err(),
            "vmv.v.i v{vd}, {imm} must Err (llvm-mc immediate out of range [-16, 15]); got {:?}",
            encode_vmv_v_i(&ops)
        );
    }
}

/// Regression: extra operand after a complete vd, simm5 vmv.v.i must Err.
#[test]
fn test_encode_vmv_v_i_regression_extra_operand() {
    let mut ops = ops2(0, 0);
    ops.push(Operand::Imm(0));
    let got = encode_vmv_v_i(&ops);
    assert!(
        got.is_err(),
        "vmv.v.i v0, 0, 0 must Err (llvm-mc invalid operand); got {got:?}"
    );
}

/// Regression: trailing v0.t on vmv.v.i must Err (unmasked-only).
#[test]
fn test_encode_vmv_v_i_regression_mask_v0t() {
    let mut ops = ops2(1, 2);
    ops.push(Operand::Symbol("v0.t".into()));
    let got = encode_vmv_v_i(&ops);
    assert!(
        got.is_err(),
        "vmv.v.i v1, 2, v0.t must Err (llvm-mc invalid operand); got {got:?}"
    );
}

/// Regression: signed simm5 16 is out of [-16, 15] and must Err.
#[test]
fn test_encode_vmv_v_i_regression_simm_oob_16() {
    let got = encode_vmv_v_i(&ops2(1, 16));
    assert!(
        got.is_err(),
        "vmv.v.i v1, 16 must Err (llvm-mc range [-16, 15]); got {got:?}"
    );
}

/// Regression: signed simm5 -17 is out of [-16, 15] and must Err.
#[test]
fn test_encode_vmv_v_i_regression_simm_oob_m17() {
    let got = encode_vmv_v_i(&ops2(1, -17));
    assert!(
        got.is_err(),
        "vmv.v.i v1, -17 must Err (llvm-mc range [-16, 15]); got {got:?}"
    );
}
