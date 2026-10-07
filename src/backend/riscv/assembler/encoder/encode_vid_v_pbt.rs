// Oracle: differential — llvm-mc RISC-V assembler (RVV vid.v)
// Evidence: vector.rs:180 "vid.v vd: OPMVV, funct6=010100, vm=1, vs2=00000, rs1=10001";
//   encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words";
//   encoder/mod.rs:1015 "vid.v" => encode_vid_v(operands) (operands passed through);
//   encoder/mod.rs:962 "TODO: masked variants (v0.t) are not yet supported; vm is hardcoded to 1";
//   assembler/README.md:14 V (vector) standard extension; assembler/README.md:109 vector.rs RVV;
//   RISC-V V 1.0 vid.v: opcode=1010111, funct3=010 (OPMVV), vd[11:7], vs1[19:15]=10001,
//   vs2[24:20]=00000, vm[25], funct6[31:26]=010100.
// Stronger considered:
//   - State machine: rejected — encode_vid_v is a pure function with no lifecycle
//   - Algebraic round-trip via in-tree decoder: rejected — no vid.v decoder
//   - encode_vmv_v_v / encode_v_arith_vv as differential siblings:
//     rejected — same-job gate (OPIVV funct3=000 / 3-operand OPIVV vs OPMVV unary vs1=10001)
// Weaker available: algebraic.invariant (field unpack), algebraic.metamorphic
//   (vd isolation), negative_error (arity / bad regs / extra)
// Differential: candidate=encode_vid_v,
//   reference=llvm-mc -triple=riscv64 -mattr=+v -show-encoding,
//   SUT-boundary=internal-helper of RISC-V assembler,
//   mapping=[Reg(vd)] <-> `vid.v vd`; [Reg(vd), Symbol("v0.t")] <-> `vid.v vd, v0.t`.

use super::{encode_vid_v, EncodeResult};
use crate::backend::riscv::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;
const OP_V: u32 = 0b1010111;
const FUNCT6_VID: u32 = 0b010100;
const VS1_VID: u32 = 0b10001;
const FUNCT3_OPMVV: u32 = 0b010;

fn cfg() -> ProptestConfig {
    ProptestConfig::with_cases(CASES)
}

fn vn(n: u32) -> String {
    format!("v{n}")
}

fn vreg(n: u32) -> Operand {
    Operand::Reg(vn(n))
}

fn ops1(vd: u32) -> Vec<Operand> {
    vec![vreg(vd)]
}

fn ops_mask(vd: u32) -> Vec<Operand> {
    vec![vreg(vd), Operand::Symbol("v0.t".into())]
}

fn sut_word(ops: &[Operand]) -> Result<u32, String> {
    match encode_vid_v(ops)? {
        EncodeResult::Word(w) => Ok(w),
        other => Err(format!("expected Word, got {other:?}")),
    }
}

/// Unpack OPMVV vid.v per RISC-V V 1.0 (not a copy of encode_vid_v).
fn unpack_vid_v(word: u32) -> (u32, u32, u32, u32, u32, u32, u32) {
    let opcode = word & 0x7f;
    let vd = (word >> 7) & 0x1f;
    let funct3 = (word >> 12) & 0x7;
    let vs1 = (word >> 15) & 0x1f;
    let vs2 = (word >> 20) & 0x1f;
    let vm = (word >> 25) & 1;
    let funct6 = word >> 26;
    (opcode, vd, funct3, vs1, vs2, vm, funct6)
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

fn extra_operand() -> impl Strategy<Value = Operand> {
    prop_oneof![
        Just(Operand::Imm(0)),
        Just(Operand::Imm(-1)),
        Just(Operand::Imm(208)),
        (0u32..=31).prop_map(|n| Operand::Reg(format!("x{n}"))),
        (0u32..=31).prop_map(|n| Operand::Reg(vn(n))),
        Just(Operand::Reg("v0".into())),
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

fn bad_vreg() -> impl Strategy<Value = Operand> {
    prop_oneof![
        (0u32..=31).prop_map(|n| Operand::Reg(format!("x{n}"))),
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

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_vid_v_kat_llvm_mc_v0() {
    let want = 0x5208a057u32;
    let mc = llvm_mc_word("vid.v v0").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT drifted: {mc:#x} != {want:#x}");
    let sut = sut_word(&ops1(0)).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_vid_v_kat_llvm_mc_v1() {
    let want = 0x5208a0d7u32;
    let mc = llvm_mc_word("vid.v v1").expect("llvm-mc KAT");
    assert_eq!(mc, want);
    let sut = sut_word(&ops1(1)).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_vid_v_kat_llvm_mc_v31() {
    let want = 0x5208afd7u32;
    let mc = llvm_mc_word("vid.v v31").expect("llvm-mc KAT");
    assert_eq!(mc, want);
    let sut = sut_word(&ops1(31)).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_vid_v_kat_llvm_mc_v1_v0t() {
    let want = 0x5008a0d7u32;
    let mc = llvm_mc_word("vid.v v1, v0.t").expect("llvm-mc masked KAT");
    assert_eq!(mc, want, "llvm-mc masked KAT drifted: {mc:#x} != {want:#x}");
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_vid_v_diff_llvm_mc(vd in vreg_n()) {
        let asm = format!("vid.v v{vd}");
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {asm}: {e}"));
        let sut = sut_word(&ops1(vd))
            .unwrap_or_else(|e| panic!("SUT rejected valid {asm}: {e}"));
        prop_assert_eq!(sut, mc, "SUT {:#010x} != llvm-mc {:#010x} for {}", sut, mc, asm);
    }

    #[test]
    fn encode_vid_v_format_fields(vd in vreg_n()) {
        let w = sut_word(&ops1(vd))
            .unwrap_or_else(|e| panic!("SUT rejected: {e}"));
        let (opcode, got_vd, funct3, vs1, vs2, vm, funct6) = unpack_vid_v(w);
        prop_assert_eq!(opcode, OP_V, "opcode");
        prop_assert_eq!(got_vd, vd, "vd");
        prop_assert_eq!(funct3, FUNCT3_OPMVV, "funct3 must be 010 (OPMVV)");
        prop_assert_eq!(vs1, VS1_VID, "vs1 must be 10001");
        prop_assert_eq!(vs2, 0, "vs2 must be 0 for vid.v");
        prop_assert_eq!(vm, 1, "vm must be 1 (unmasked)");
        prop_assert_eq!(funct6, FUNCT6_VID, "funct6 must be 010100");
    }

    #[test]
    fn encode_vid_v_field_isolation(vd_a in vreg_n(), vd_b in vreg_n()) {
        let wa = sut_word(&ops1(vd_a))
            .unwrap_or_else(|e| panic!("a rejected: {e}"));
        let wb = sut_word(&ops1(vd_b))
            .unwrap_or_else(|e| panic!("b rejected: {e}"));
        let vd_mask = 0x1fu32 << 7;
        prop_assert_eq!(wa & !vd_mask, wb & !vd_mask, "non-vd bits independent of vd");
        prop_assert_eq!((wa >> 7) & 0x1f, vd_a);
        prop_assert_eq!((wb >> 7) & 0x1f, vd_b);
    }

    #[test]
    fn encode_vid_v_neg_arity_bad_regs(bad in bad_vreg()) {
        prop_assert!(
            encode_vid_v(&[]).is_err(),
            "arity 0 must Err (llvm-mc too few operands); got {:?}",
            encode_vid_v(&[])
        );
        let bad_ops = vec![bad.clone()];
        prop_assert!(
            encode_vid_v(&bad_ops).is_err(),
            "non-vector operand {:?} must Err (llvm-mc invalid operand); got {:?}",
            bad,
            encode_vid_v(&bad_ops)
        );
    }

    #[test]
    fn encode_vid_v_neg_extra(vd in vreg_n(), extra in extra_operand()) {
        let mut ops = ops1(vd);
        ops.push(extra.clone());
        prop_assert!(
            encode_vid_v(&ops).is_err(),
            "extra operand {:?} must Err for vid.v (llvm-mc rejects extra except v0.t); got {:?}",
            extra,
            encode_vid_v(&ops)
        );
    }

    #[test]
    fn encode_vid_v_mask_v0t_diff_llvm_mc(vd in vreg_n()) {
        let asm = format!("vid.v v{vd}, v0.t");
        let mc = llvm_mc_word(&asm);
        let sut = sut_word(&ops_mask(vd));
        match (mc, sut) {
            (Ok(mc_w), Ok(sut_w)) => {
                prop_assert_eq!(
                    sut_w,
                    mc_w,
                    "SUT {:#010x} != llvm-mc {:#010x} for {}",
                    sut_w,
                    mc_w,
                    asm
                );
            }
            (Err(_), Err(_)) => {}
            (Ok(mc_w), Err(e)) => {
                prop_assert!(
                    false,
                    "SUT rejected valid {asm} (llvm-mc {:#010x}): {e}",
                    mc_w
                );
            }
            (Err(mc_e), Ok(sut_w)) => {
                prop_assert!(
                    false,
                    "SUT encoded {asm} as {:#010x} but llvm-mc rejected: {mc_e}",
                    sut_w
                );
            }
        }
    }
}

/// Regression: extra operand after a complete vd vid.v must Err.
#[test]
fn test_encode_vid_v_regression_extra_operand() {
    let mut ops = ops1(0);
    ops.push(Operand::Imm(0));
    let got = encode_vid_v(&ops);
    assert!(
        got.is_err(),
        "vid.v v0, 0 must Err (llvm-mc invalid operand); got {got:?}"
    );
}

/// Regression: vid.v v0, v0.t must Err (destination overlaps mask).
#[test]
fn test_encode_vid_v_regression_mask_v0_overlap() {
    let got = encode_vid_v(&ops_mask(0));
    assert!(
        got.is_err(),
        "vid.v v0, v0.t must Err (llvm-mc: destination overlaps mask); got {got:?}"
    );
}

/// Regression: vid.v v1, v0.t must encode vm=0 matching llvm-mc 0x5008a0d7.
#[test]
fn test_encode_vid_v_regression_mask_v0t() {
    let want = 0x5008a0d7u32;
    let got = sut_word(&ops_mask(1));
    assert_eq!(
        got,
        Ok(want),
        "vid.v v1, v0.t must encode vm=0 (llvm-mc {want:#010x}); got {got:?}"
    );
}
