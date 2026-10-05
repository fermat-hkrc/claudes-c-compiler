// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md:232 NEON reduce lists umaxv, uminv, smaxv, sminv;
//   encoder/mod.rs:1-7 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:693-696 umaxv/uminv/smaxv/sminv => encode_neon_across;
//   neon.rs:437 Encode NEON across-vector instructions: UMAXV, UMINV, SMAXV, SMINV;
//   neon.rs:439 Format: 0 Q U 01110 size 11000 opcode 10 Rn Rd;
//   neon.rs:441 u_bit: 0 for signed, 1 for unsigned;
//   neon.rs:442 opcode: 5-bit opcode (bits 16-12);
//   neon.rs:446 "NEON across-vector requires 2 operands";
//   ARM ARM Advanced SIMD across lanes (UMAXV/UMINV/SMAXV/SMINV). Dest Bd/Hd/Sd matching T;
//   T in {8B,16B,4H,8H,4S}; size:Q=10:0 (2S) and size=11 reserved.
// Stronger considered:
//   - State machine: rejected — encode_neon_across is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree across-vector decoder
//   - Differential vs encode_neon_addv / encode_neon_across_long: rejected — same-job gate
//     (addv is opcode 11011 U=0; across_long is SADDLV/UADDLV widening)
// Weaker available: algebraic.metamorphic (Rd/Rn/U/opcode fields), algebraic.invariant (word layout),
//   negative_error (arity / extra / invalid T / dest mismatch / GPR dest)
// Differential: candidate=encode_neon_across, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[Reg(Vd_scalar), RegArrangement(Vn,T)] <-> `{umaxv|uminv|smaxv|sminv} Vd, Vn.T`

use super::encode_neon_across;
use super::EncodeResult;
use crate::backend::arm::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;

fn cfg() -> ProptestConfig {
    ProptestConfig::with_cases(CASES)
}

fn arr(reg: u32, t: &str) -> Operand {
    Operand::RegArrangement {
        reg: format!("v{}", reg),
        arrangement: t.to_string(),
    }
}

fn scalar(prefix: &str, n: u32) -> Operand {
    Operand::Reg(format!("{}{}", prefix, n))
}

fn gpr(prefix: &str, n: u32) -> Operand {
    Operand::Reg(format!("{}{}", prefix, n))
}

fn across_ops(rd: u32, v: &str, rn: u32, t: &str) -> Vec<Operand> {
    vec![scalar(v, rd), arr(rn, t)]
}

fn across_asm(mnem: &str, rd: u32, v: &str, rn: u32, t: &str) -> String {
    format!("{mnem} {v}{rd}, v{rn}.{t}")
}

fn sut_word(ops: &[Operand], u_bit: u32, opcode: u32) -> Result<u32, String> {
    match encode_neon_across(ops, u_bit, opcode)? {
        EncodeResult::Word(w) => Ok(w),
        other => Err(format!("expected Word, got {:?}", other)),
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
        let hex = p
            .strip_prefix("0x")
            .ok_or_else(|| format!("non-hex byte {p}"))?;
        bytes[i] = u8::from_str_radix(hex, 16).map_err(|e| e.to_string())?;
    }
    Ok(u32::from_le_bytes(bytes))
}

fn llvm_mc_word(asm: &str) -> Result<u32, String> {
    let mut child = Command::new(LLVM_MC)
        .args(["-triple=aarch64", "-show-encoding"])
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

fn reg_num() -> impl Strategy<Value = u32> {
    prop_oneof![Just(0u32), Just(1u32), Just(30u32), Just(31u32), 0u32..=31]
}

fn valid_vt() -> impl Strategy<Value = (&'static str, &'static str)> {
    prop::sample::select(vec![
        ("b", "8b"),
        ("b", "16b"),
        ("h", "4h"),
        ("h", "8h"),
        ("s", "4s"),
    ])
}

fn caller_triple() -> impl Strategy<Value = (&'static str, u32, u32)> {
    prop::sample::select(vec![
        ("umaxv", 1u32, 0b01010u32),
        ("uminv", 1, 0b11010),
        ("smaxv", 0, 0b01010),
        ("sminv", 0, 0b11010),
    ])
}

fn invalid_t() -> impl Strategy<Value = &'static str> {
    prop::sample::select(vec!["2s", "1d", "2d", "4b", "8d", "2h", "1s"])
}

fn mismatch_prefix(v: &str) -> &'static str {
    match v {
        "b" => "h",
        "h" => "s",
        _ => "b",
    }
}

/// ARM Advanced SIMD across-lanes (UMAXV/UMINV/SMAXV/SMINV):
/// 0 Q U 01110 size 11000 opcode 10 Rn Rd
/// Independent of the SUT producing `|` chain. Q/size from ARM T table
/// (2S and size=11 reserved; not taken from neon_arr_to_q_size).
fn arm_across_word(rd: u32, rn: u32, t: &str, u_bit: u32, opcode: u32) -> u32 {
    let (q, size) = match t {
        "8b" => (0u32, 0b00u32),
        "16b" => (1, 0b00),
        "4h" => (0, 0b01),
        "8h" => (1, 0b01),
        "4s" => (1, 0b10),
        _ => panic!("arm_across_word: not an across-lanes T: {t}"),
    };
    (q << 30)
        | (u_bit << 29)
        | (0b01110 << 24)
        | (size << 22)
        | (0b11000 << 17)
        | (opcode << 12)
        | (0b10 << 10)
        | (rn << 5)
        | rd
}

fn q_of(t: &str) -> u32 {
    match t {
        "16b" | "8h" | "4s" => 1,
        _ => 0,
    }
}

fn size_of(t: &str) -> u32 {
    match t {
        "8b" | "16b" => 0b00,
        "4h" | "8h" => 0b01,
        "4s" => 0b10,
        _ => 0,
    }
}

// Reference KAT gate (must pass before PBT) — llvm-mc mapping only, then SUT.
#[test]
fn encode_neon_across_kat_llvm_mc_umaxv_b0_v1_8b() {
    let want = 0x2e30a820u32;
    let mc = llvm_mc_word("umaxv b0, v1.8b").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    assert_eq!(want, arm_across_word(0, 1, "8b", 1, 0b01010));
    let sut = sut_word(&across_ops(0, "b", 1, "8b"), 1, 0b01010).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_neon_across_kat_llvm_mc_umaxv_b0_v1_16b() {
    let want = 0x6e30a820u32;
    let mc = llvm_mc_word("umaxv b0, v1.16b").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    assert_eq!(want, arm_across_word(0, 1, "16b", 1, 0b01010));
    let sut = sut_word(&across_ops(0, "b", 1, "16b"), 1, 0b01010).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_neon_across_kat_llvm_mc_umaxv_h0_v1_4h() {
    let want = 0x2e70a820u32;
    let mc = llvm_mc_word("umaxv h0, v1.4h").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    assert_eq!(want, arm_across_word(0, 1, "4h", 1, 0b01010));
    let sut = sut_word(&across_ops(0, "h", 1, "4h"), 1, 0b01010).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_neon_across_kat_llvm_mc_umaxv_h0_v1_8h() {
    let want = 0x6e70a820u32;
    let mc = llvm_mc_word("umaxv h0, v1.8h").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    assert_eq!(want, arm_across_word(0, 1, "8h", 1, 0b01010));
    let sut = sut_word(&across_ops(0, "h", 1, "8h"), 1, 0b01010).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_neon_across_kat_llvm_mc_umaxv_s0_v1_4s() {
    let want = 0x6eb0a820u32;
    let mc = llvm_mc_word("umaxv s0, v1.4s").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    assert_eq!(want, arm_across_word(0, 1, "4s", 1, 0b01010));
    let sut = sut_word(&across_ops(0, "s", 1, "4s"), 1, 0b01010).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_neon_across_kat_llvm_mc_uminv_b0_v1_8b() {
    let want = 0x2e31a820u32;
    let mc = llvm_mc_word("uminv b0, v1.8b").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    assert_eq!(want, arm_across_word(0, 1, "8b", 1, 0b11010));
    let sut = sut_word(&across_ops(0, "b", 1, "8b"), 1, 0b11010).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_neon_across_kat_llvm_mc_smaxv_b0_v1_8b() {
    let want = 0x0e30a820u32;
    let mc = llvm_mc_word("smaxv b0, v1.8b").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    assert_eq!(want, arm_across_word(0, 1, "8b", 0, 0b01010));
    let sut = sut_word(&across_ops(0, "b", 1, "8b"), 0, 0b01010).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_neon_across_kat_llvm_mc_sminv_b0_v1_8b() {
    let want = 0x0e31a820u32;
    let mc = llvm_mc_word("sminv b0, v1.8b").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    assert_eq!(want, arm_across_word(0, 1, "8b", 0, 0b11010));
    let sut = sut_word(&across_ops(0, "b", 1, "8b"), 0, 0b11010).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_neon_across_kat_llvm_mc_umaxv_b31_v31_8b() {
    let want = 0x2e30abffu32;
    let mc = llvm_mc_word("umaxv b31, v31.8b").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    assert_eq!(want, arm_across_word(31, 31, "8b", 1, 0b01010));
    let sut = sut_word(&across_ops(31, "b", 31, "8b"), 1, 0b01010).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_neon_across_kat_llvm_mc_smaxv_s0_v1_4s() {
    let want = 0x4eb0a820u32;
    let mc = llvm_mc_word("smaxv s0, v1.4s").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    assert_eq!(want, arm_across_word(0, 1, "4s", 0, 0b01010));
    let sut = sut_word(&across_ops(0, "s", 1, "4s"), 0, 0b01010).expect("SUT KAT");
    assert_eq!(sut, want);
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_across_diff_llvm_mc(
        rd in reg_num(),
        rn in reg_num(),
        (v, t) in valid_vt(),
        (mnem, u_bit, opcode) in caller_triple(),
    ) {
        let asm = across_asm(mnem, rd, v, rn, t);
        let mc = llvm_mc_word(&asm).expect(&asm);
        let sut = sut_word(&across_ops(rd, v, rn, t), u_bit, opcode).expect(&asm);
        prop_assert_eq!(sut, mc, "SUT vs llvm-mc for {}", asm);
    }

    #[test]
    fn encode_neon_across_meta_rd_rn(
        rd1 in reg_num(),
        rd2 in reg_num(),
        rn1 in reg_num(),
        rn2 in reg_num(),
        (v, t) in valid_vt(),
        (_mnem, u_bit, opcode) in caller_triple(),
    ) {
        let w11 = sut_word(&across_ops(rd1, v, rn1, t), u_bit, opcode).expect("w11");
        let w21 = sut_word(&across_ops(rd2, v, rn1, t), u_bit, opcode).expect("w21");
        let w12 = sut_word(&across_ops(rd1, v, rn2, t), u_bit, opcode).expect("w12");
        prop_assert_eq!(
            (w11 ^ w21) & !0x1Fu32,
            0,
            "changing only Rd must differ only in bits[4:0]"
        );
        prop_assert_eq!(w11 & 0x1F, rd1);
        prop_assert_eq!(w21 & 0x1F, rd2);
        prop_assert_eq!(
            (w11 ^ w12) & !(0x1Fu32 << 5),
            0,
            "changing only Rn must differ only in bits[9:5]"
        );
        prop_assert_eq!((w11 >> 5) & 0x1F, rn1);
        prop_assert_eq!((w12 >> 5) & 0x1F, rn2);
    }

    #[test]
    fn encode_neon_across_inv_layout(
        rd in reg_num(),
        rn in reg_num(),
        (v, t) in valid_vt(),
        (_mnem, u_bit, opcode) in caller_triple(),
    ) {
        let w = sut_word(&across_ops(rd, v, rn, t), u_bit, opcode).expect("layout");
        prop_assert_eq!(w, arm_across_word(rd, rn, t, u_bit, opcode), "ARM across-lanes layout");
        prop_assert_eq!(w >> 31, 0);
        prop_assert_eq!((w >> 30) & 1, q_of(t));
        prop_assert_eq!((w >> 29) & 1, u_bit, "U bit");
        prop_assert_eq!((w >> 24) & 0x1F, 0b01110);
        prop_assert_eq!((w >> 22) & 0b11, size_of(t));
        prop_assert_eq!((w >> 17) & 0x1F, 0b11000);
        prop_assert_eq!((w >> 12) & 0x1F, opcode, "opcode bits[16:12]");
        prop_assert_eq!((w >> 10) & 0b11, 0b10, "bits[11:10]");
        prop_assert_eq!((w >> 5) & 0x1F, rn);
        prop_assert_eq!(w & 0x1F, rd);
    }

    #[test]
    fn encode_neon_across_meta_u_opcode(
        rd in reg_num(),
        rn in reg_num(),
        (v, t) in valid_vt(),
        opcode1 in prop::sample::select(vec![0b01010u32, 0b11010]),
        opcode2 in prop::sample::select(vec![0b01010u32, 0b11010]),
    ) {
        let w_u0 = sut_word(&across_ops(rd, v, rn, t), 0, opcode1).expect("u0");
        let w_u1 = sut_word(&across_ops(rd, v, rn, t), 1, opcode1).expect("u1");
        prop_assert_eq!(w_u0 ^ w_u1, 1u32 << 29, "flipping U must differ only in bit 29");
        let w_op1 = sut_word(&across_ops(rd, v, rn, t), 0, opcode1).expect("op1");
        let w_op2 = sut_word(&across_ops(rd, v, rn, t), 0, opcode2).expect("op2");
        prop_assert_eq!(
            (w_op1 ^ w_op2) & !(0x1Fu32 << 12),
            0,
            "changing only opcode must differ only in bits[16:12]"
        );
        prop_assert_eq!((w_op1 >> 12) & 0x1F, opcode1);
        prop_assert_eq!((w_op2 >> 12) & 0x1F, opcode2);
    }

    #[test]
    fn encode_neon_across_neg_arity(
        n in 0usize..=1,
        rd in reg_num(),
        (v, _t) in valid_vt(),
        (_mnem, u_bit, opcode) in caller_triple(),
    ) {
        let ops: Vec<Operand> = match n {
            0 => vec![],
            _ => vec![scalar(v, rd)],
        };
        prop_assert!(
            encode_neon_across(&ops, u_bit, opcode).is_err(),
            "arity {} must Err (NEON across-vector requires 2 operands)",
            n
        );
    }

    #[test]
    fn encode_neon_across_neg_extra(
        rd in reg_num(),
        rn in reg_num(),
        extra in reg_num(),
        (v, t) in valid_vt(),
        (mnem, u_bit, opcode) in caller_triple(),
    ) {
        let asm = format!("{mnem} {v}{rd}, v{rn}.{t}, v{extra}.{t}");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted extra {}",
            asm
        );
        let ops = [scalar(v, rd), arr(rn, t), arr(extra, t)];
        prop_assert!(
            encode_neon_across(&ops, u_bit, opcode).is_err(),
            "extra operand must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_across_neg_invalid_t(
        rd in reg_num(),
        rn in reg_num(),
        t in invalid_t(),
        (mnem, u_bit, opcode) in caller_triple(),
    ) {
        let dest = match t {
            "2s" => "s",
            "1d" | "2d" => "d",
            _ => "b",
        };
        let asm = format!("{mnem} {dest}{rd}, v{rn}.{t}");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let ops = [scalar(dest, rd), arr(rn, t)];
        prop_assert!(
            encode_neon_across(&ops, u_bit, opcode).is_err(),
            "invalid T must Err (ARM reserved/unsupported; llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_across_neg_dest(
        rd in reg_num(),
        rn in reg_num(),
        (v, t) in valid_vt(),
        (mnem, u_bit, opcode) in caller_triple(),
        kind in 0u8..=4,
    ) {
        let (ops, asm) = match kind {
            0 => {
                let asm = format!("{mnem} x{rd}, v{rn}.{t}");
                let ops = vec![gpr("x", rd), arr(rn, t)];
                (ops, asm)
            }
            1 => {
                let asm = format!("{mnem} w{rd}, v{rn}.{t}");
                let ops = vec![gpr("w", rd), arr(rn, t)];
                (ops, asm)
            }
            2 => {
                let asm = format!("{mnem} sp, v0.8b");
                let ops = vec![Operand::Reg("sp".into()), arr(0, "8b")];
                (ops, asm)
            }
            3 => {
                let asm = format!("{mnem} v{rd}.{t}, v{rn}.{t}");
                let ops = vec![arr(rd, t), arr(rn, t)];
                (ops, asm)
            }
            _ => {
                let bad = mismatch_prefix(v);
                let asm = format!("{mnem} {bad}{rd}, v{rn}.{t}");
                let ops = vec![scalar(bad, rd), arr(rn, t)];
                (ops, asm)
            }
        };
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        prop_assert!(
            encode_neon_across(&ops, u_bit, opcode).is_err(),
            "invalid dest must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_across_diff_alt_spellings(
        rd in reg_num(),
        rn in reg_num(),
        (v, t) in valid_vt(),
        (mnem, u_bit, opcode) in caller_triple(),
    ) {
        let v_up = v.to_uppercase();
        let t_up = t.to_uppercase();
        let mnem_up = mnem.to_uppercase();
        let asm = format!("{mnem_up} {v_up}{rd}, V{rn}.{t_up}");
        let mc = llvm_mc_word(&asm).expect(&asm);
        let ops = [
            Operand::Reg(format!("{v_up}{rd}")),
            Operand::RegArrangement {
                reg: format!("V{rn}"),
                arrangement: t.to_string(),
            },
        ];
        let sut = sut_word(&ops, u_bit, opcode).expect(&asm);
        prop_assert_eq!(sut, mc, "uppercase prefix must match llvm-mc {}", asm);
    }
}

#[test]
fn test_encode_neon_across_regression_extra_operand() {
    let ops = [scalar("b", 0), arr(0, "8b"), arr(0, "8b")];
    assert!(
        encode_neon_across(&ops, 1, 0b01010).is_err(),
        "umaxv b0, v0.8b, v0.8b must Err (llvm-mc/gas reject a third operand)"
    );
}

#[test]
fn test_encode_neon_across_regression_reserved_2s() {
    let ops = [scalar("s", 0), arr(0, "2s")];
    assert!(
        encode_neon_across(&ops, 1, 0b01010).is_err(),
        "umaxv s0, v0.2s must Err (ARM size:Q=10:0 is reserved; llvm-mc: invalid operand)"
    );
}

#[test]
fn test_encode_neon_across_regression_gpr_dest() {
    let ops = [gpr("x", 0), arr(0, "8b")];
    assert!(
        encode_neon_across(&ops, 1, 0b01010).is_err(),
        "umaxv x0, v0.8b must Err (llvm-mc/gas reject GPR dest)"
    );
}
