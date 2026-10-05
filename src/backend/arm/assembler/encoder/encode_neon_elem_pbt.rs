// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md:231 NEON by-element lists mul/mla/mls/sqdmulh/sqrdmulh (with lane index);
//   encoder/mod.rs:1-7 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:307-310 mul RegLane => encode_neon_elem(operands, 0, 0b1000);
//   encoder/mod.rs:782-787 sqdmulh/sqrdmulh RegLane => encode_neon_elem;
//   encoder/mod.rs:793-796 mla/mls RegLane => encode_neon_elem;
//   neon.rs:1590 MUL/MLA/MLS by element: 0 Q U 01111 size L M Rm opcode H 0 Rn Rd;
//   ARM ARM Advanced SIMD vector x indexed element (non-long):
//     MUL/MLA/MLS/SQDMULH/SQRDMULH Vd.T, Vn.T, Vm.Ts[index];
//     T in {4H,8H,2S,4S}; size=00/11 reserved;
//     size=01 Rm v0-v15 index H:L:M 0..7;
//     size=10 Rm v0-v31 (M=Rm[4]) index H:L 0..3;
//     MUL U=0 opcode=1000; MLA U=1 opcode=0000; MLS U=1 opcode=0100;
//     SQDMULH U=0 opcode=1100; SQRDMULH U=0 opcode=1101.
// Stronger considered:
//   - State machine: rejected — encode_neon_elem is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree by-element decoder
//   - Differential vs encode_neon_elem_long: rejected — same-job gate (widening SMULL/…)
//   - Differential vs encode_neon_mul / encode_neon_mla / encode_neon_mls: rejected — vector (non-lane)
//   - Differential vs encode_neon_float_elem: rejected — FP by-element
// Weaker available: algebraic.metamorphic (Rd/Rn/U fields), algebraic.invariant (word layout),
//   negative_error (arity / extra / dest-src mismatch / H-Rm v16-v31 / GPR dest / non-V prefix)
// Differential: candidate=encode_neon_elem, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[RegArrangement(Vd,T), RegArrangement(Vn,T), RegLane(Vm,Ts,idx)] + (U, opcode)
//     <-> `{mul|mla|mls|sqdmulh|sqrdmulh} Vd.T, Vn.T, Vm.Ts[idx]`

use super::encode_neon_elem;
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

fn lane(reg: u32, elem: &str, index: u32) -> Operand {
    Operand::RegLane {
        reg: format!("v{}", reg),
        elem_size: elem.to_string(),
        index,
    }
}

fn gpr(prefix: &str, n: u32) -> Operand {
    Operand::Reg(format!("{}{}", prefix, n))
}

fn sut_word(ops: &[Operand], u_bit: u32, opcode: u32) -> Result<u32, String> {
    match encode_neon_elem(ops, u_bit, opcode)? {
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
    prop_oneof![
        Just(0u32),
        Just(1u32),
        Just(15u32),
        Just(16u32),
        Just(30u32),
        Just(31u32),
        0u32..=31
    ]
}

/// Dest/src T, lane elem, max index, max Rm.
fn valid_shape() -> impl Strategy<Value = (&'static str, &'static str, u32, u32)> {
    prop::sample::select(vec![
        ("4h", "h", 7u32, 15u32),
        ("8h", "h", 7, 15),
        ("2s", "s", 3, 31),
        ("4s", "s", 3, 31),
    ])
}

/// ARM-correct (U, opcode, mnemonic) for non-long by-element.
fn insn_table() -> impl Strategy<Value = (u32, u32, &'static str)> {
    prop::sample::select(vec![
        (0u32, 0b1000u32, "mul"),
        (1u32, 0b0000u32, "mla"),
        (1u32, 0b0100u32, "mls"),
        (0u32, 0b1100u32, "sqdmulh"),
        (0u32, 0b1101u32, "sqrdmulh"),
    ])
}

fn opcode_bits() -> impl Strategy<Value = u32> {
    prop::sample::select(vec![0b0000u32, 0b0100u32, 0b1000u32, 0b1100u32, 0b1101u32])
}

fn size_of(t: &str) -> u32 {
    match t {
        "4h" | "8h" => 0b01,
        "2s" | "4s" => 0b10,
        _ => 0xff,
    }
}

fn q_of(t: &str) -> u32 {
    match t {
        "8h" | "4s" => 1,
        _ => 0,
    }
}

fn index_hlm(size: u32, index: u32, rm: u32) -> (u32, u32, u32) {
    match size {
        0b01 => ((index >> 2) & 1, (index >> 1) & 1, index & 1),
        0b10 => ((index >> 1) & 1, index & 1, (rm >> 4) & 1),
        _ => (0xff, 0xff, 0xff),
    }
}

// Reference KAT gate (must pass before PBT).
#[test]
fn encode_neon_elem_kat_llvm_mc_mul_v0_4h_v1_4h_v2_h2() {
    let want = 0x0f628020u32;
    let mc = llvm_mc_word("mul v0.4h, v1.4h, v2.h[2]").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [arr(0, "4h"), arr(1, "4h"), lane(2, "h", 2)];
    let sut = sut_word(&ops, 0, 0b1000).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_neon_elem_kat_llvm_mc_mla_mls_sqdmulh_bounds() {
    let mc_mla = llvm_mc_word("mla v0.8h, v1.8h, v15.h[7]").expect("llvm-mc KAT mla");
    let sut_mla = sut_word(&[arr(0, "8h"), arr(1, "8h"), lane(15, "h", 7)], 1, 0b0000)
        .expect("SUT KAT mla");
    assert_eq!(sut_mla, mc_mla, "SUT vs llvm-mc mla v0.8h, v1.8h, v15.h[7]");
    assert_eq!(mc_mla, 0x6f7f0820u32);

    let mc_mls = llvm_mc_word("mls v0.2s, v1.2s, v31.s[3]").expect("llvm-mc KAT mls");
    let sut_mls = sut_word(&[arr(0, "2s"), arr(1, "2s"), lane(31, "s", 3)], 1, 0b0100)
        .expect("SUT KAT mls");
    assert_eq!(sut_mls, mc_mls, "SUT vs llvm-mc mls v0.2s, v1.2s, v31.s[3]");
    assert_eq!(mc_mls, 0x2fbf4820u32);

    let mc_q = llvm_mc_word("sqdmulh v0.4s, v1.4s, v2.s[1]").expect("llvm-mc KAT sqdmulh");
    let sut_q = sut_word(&[arr(0, "4s"), arr(1, "4s"), lane(2, "s", 1)], 0, 0b1100)
        .expect("SUT KAT sqdmulh");
    assert_eq!(sut_q, mc_q, "SUT vs llvm-mc sqdmulh v0.4s, v1.4s, v2.s[1]");
    assert_eq!(mc_q, 0x4fa2c020u32);

    let mc_r = llvm_mc_word("sqrdmulh v31.4h, v30.4h, v0.h[0]").expect("llvm-mc KAT sqrdmulh");
    let sut_r = sut_word(&[arr(31, "4h"), arr(30, "4h"), lane(0, "h", 0)], 0, 0b1101)
        .expect("SUT KAT sqrdmulh");
    assert_eq!(
        sut_r, mc_r,
        "SUT vs llvm-mc sqrdmulh v31.4h, v30.4h, v0.h[0]"
    );
    assert_eq!(mc_r, 0x0f40d3dfu32);
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_elem_diff_llvm_mc(
        rd in reg_num(),
        rn in reg_num(),
        rm_raw in reg_num(),
        idx_raw in 0u32..=7,
        shape in valid_shape(),
        insn in insn_table(),
    ) {
        let (t, elem, imax, rmmax) = shape;
        let (u, opcode, mnem) = insn;
        let rm = rm_raw % (rmmax + 1);
        let idx = idx_raw % (imax + 1);
        let asm = format!("{mnem} v{rd}.{t}, v{rn}.{t}, v{rm}.{elem}[{idx}]");
        let mc = llvm_mc_word(&asm).expect(&asm);
        let sut = sut_word(&[arr(rd, t), arr(rn, t), lane(rm, elem, idx)], u, opcode)
            .expect(&asm);
        prop_assert_eq!(sut, mc, "SUT vs llvm-mc for {}", asm);
    }

    #[test]
    fn encode_neon_elem_meta_rd_rn_u(
        rd1 in reg_num(),
        rd2 in reg_num(),
        rn1 in reg_num(),
        rn2 in reg_num(),
        rm_raw in reg_num(),
        idx_raw in 0u32..=7,
        shape in valid_shape(),
        opcode in opcode_bits(),
    ) {
        let (t, elem, imax, rmmax) = shape;
        let rm = rm_raw % (rmmax + 1);
        let idx = idx_raw % (imax + 1);
        let ops = |rd: u32, rn: u32| [arr(rd, t), arr(rn, t), lane(rm, elem, idx)];
        let w11 = sut_word(&ops(rd1, rn1), 0, opcode).expect("w11");
        let w21 = sut_word(&ops(rd2, rn1), 0, opcode).expect("w21");
        let w12 = sut_word(&ops(rd1, rn2), 0, opcode).expect("w12");
        let w_u1 = sut_word(&ops(rd1, rn1), 1, opcode).expect("w_u1");
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
        prop_assert_eq!(w11 ^ w_u1, 1u32 << 29, "U bit must be bit 29");
    }

    #[test]
    fn encode_neon_elem_inv_layout(
        rd in reg_num(),
        rn in reg_num(),
        rm_raw in reg_num(),
        idx_raw in 0u32..=7,
        shape in valid_shape(),
        u in 0u32..=1,
        opcode in opcode_bits(),
    ) {
        let (t, elem, imax, rmmax) = shape;
        let rm = rm_raw % (rmmax + 1);
        let idx = idx_raw % (imax + 1);
        let w = sut_word(&[arr(rd, t), arr(rn, t), lane(rm, elem, idx)], u, opcode)
            .expect("layout");
        let size = size_of(t);
        let q = q_of(t);
        let (h, l, m) = index_hlm(size, idx, rm);
        let rm_lo = rm & 0xF;
        prop_assert_eq!(w >> 31, 0);
        prop_assert_eq!((w >> 30) & 1, q);
        prop_assert_eq!((w >> 29) & 1, u);
        prop_assert_eq!((w >> 24) & 0x1F, 0b01111);
        prop_assert_eq!((w >> 22) & 0b11, size);
        prop_assert_eq!((w >> 21) & 1, l);
        prop_assert_eq!((w >> 20) & 1, m);
        prop_assert_eq!((w >> 16) & 0xF, rm_lo);
        prop_assert_eq!((w >> 12) & 0xF, opcode);
        prop_assert_eq!((w >> 11) & 1, h);
        prop_assert_eq!((w >> 10) & 1, 0);
        prop_assert_eq!((w >> 5) & 0x1F, rn);
        prop_assert_eq!(w & 0x1F, rd);
    }

    #[test]
    fn encode_neon_elem_neg_arity(
        n in 0usize..=2,
        rd in reg_num(),
        rn in reg_num(),
        shape in valid_shape(),
    ) {
        let (t, _elem, _, _) = shape;
        let ops: Vec<Operand> = match n {
            0 => vec![],
            1 => vec![arr(rd, t)],
            _ => vec![arr(rd, t), arr(rn, t)],
        };
        prop_assert!(
            encode_neon_elem(&ops, 0, 0b1000).is_err(),
            "arity {} must Err (NEON by-element requires 3 operands)",
            n
        );
    }

    #[test]
    fn encode_neon_elem_neg_extra(
        rd in reg_num(),
        rn in reg_num(),
        rm_raw in reg_num(),
        extra in reg_num(),
        idx_raw in 0u32..=7,
        shape in valid_shape(),
        insn in insn_table(),
    ) {
        let (t, elem, imax, rmmax) = shape;
        let (u, opcode, mnem) = insn;
        let rm = rm_raw % (rmmax + 1);
        let idx = idx_raw % (imax + 1);
        let asm = format!("{mnem} v{rd}.{t}, v{rn}.{t}, v{rm}.{elem}[{idx}], v{extra}.{t}");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted extra {}",
            asm
        );
        let ops = [
            arr(rd, t),
            arr(rn, t),
            lane(rm, elem, idx),
            arr(extra, t),
        ];
        prop_assert!(
            encode_neon_elem(&ops, u, opcode).is_err(),
            "extra operand must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_elem_neg_mismatch_t(
        rd in reg_num(),
        rn in reg_num(),
        rm_raw in reg_num(),
        idx_raw in 0u32..=7,
        shape in valid_shape(),
        t_wrong in prop::sample::select(vec!["8b", "16b", "4h", "8h", "2s", "4s", "2d", "1d"]),
    ) {
        let (t, elem, imax, rmmax) = shape;
        prop_assume!(t_wrong != t);
        let rm = rm_raw % (rmmax + 1);
        let idx = idx_raw % (imax + 1);
        let asm = format!("mul v{rd}.{t}, v{rn}.{t_wrong}, v{rm}.{elem}[{idx}]");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let ops = [arr(rd, t), arr(rn, t_wrong), lane(rm, elem, idx)];
        prop_assert!(
            encode_neon_elem(&ops, 0, 0b1000).is_err(),
            "mismatched source T must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_elem_neg_h_rm_hi(
        rd in reg_num(),
        rn in reg_num(),
        rm in 16u32..=31,
        idx in 0u32..=7,
        t in prop::sample::select(vec!["4h", "8h"]),
    ) {
        let asm = format!("mul v{rd}.{t}, v{rn}.{t}, v{rm}.h[{idx}]");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let ops = [arr(rd, t), arr(rn, t), lane(rm, "h", idx)];
        prop_assert!(
            encode_neon_elem(&ops, 0, 0b1000).is_err(),
            "H-lane Rm v16-v31 must Err (ARM size=01 Rm v0-v15; llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_elem_neg_gpr_bare_nonv(
        rd in reg_num(),
        rn in reg_num(),
        rm in 0u32..=15,
        idx in 0u32..=7,
        kind in 0u8..=6,
    ) {
        let (ops, asm) = match kind {
            0 => {
                let asm = format!("mul x{rd}, v{rn}.4h, v{rm}.h[{idx}]");
                let ops = vec![gpr("x", rd), arr(rn, "4h"), lane(rm, "h", idx)];
                (ops, asm)
            }
            1 => {
                let asm = format!("mul w{rd}, v{rn}.4h, v{rm}.h[{idx}]");
                let ops = vec![gpr("w", rd), arr(rn, "4h"), lane(rm, "h", idx)];
                (ops, asm)
            }
            2 => {
                let asm = format!("mul sp, v{rn}.4h, v{rm}.h[{idx}]");
                let ops = vec![Operand::Reg("sp".into()), arr(rn, "4h"), lane(rm, "h", idx)];
                (ops, asm)
            }
            3 => {
                let asm = format!("mul v{rd}, v{rn}, v{rm}.h[{idx}]");
                let ops = vec![gpr("v", rd), gpr("v", rn), lane(rm, "h", idx)];
                (ops, asm)
            }
            4 => {
                let asm = format!("mul x{rd}.4h, v{rn}.4h, v{rm}.h[{idx}]");
                let ops = vec![
                    Operand::RegArrangement {
                        reg: format!("x{rd}"),
                        arrangement: "4h".to_string(),
                    },
                    arr(rn, "4h"),
                    lane(rm, "h", idx),
                ];
                (ops, asm)
            }
            5 => {
                let asm = format!("mul v{rd}.4h, v{rn}.4h, v{rm}.4h");
                let ops = vec![arr(rd, "4h"), arr(rn, "4h"), arr(rm, "4h")];
                (ops, asm)
            }
            _ => {
                let asm = format!("mul s{rd}, v{rn}.4h, v{rm}.h[{idx}]");
                let ops = vec![gpr("s", rd), arr(rn, "4h"), lane(rm, "h", idx)];
                (ops, asm)
            }
        };
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        prop_assert!(
            encode_neon_elem(&ops, 0, 0b1000).is_err(),
            "non-arranged NEON / GPR / SP / non-V prefix / non-lane third must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_elem_neg_index_oob(
        rd in reg_num(),
        rn in reg_num(),
        rm_raw in reg_num(),
        shape in valid_shape(),
        extra in 1u32..=8,
    ) {
        let (t, elem, imax, rmmax) = shape;
        let rm = rm_raw % (rmmax + 1);
        let idx = imax + extra;
        let asm = format!("mul v{rd}.{t}, v{rn}.{t}, v{rm}.{elem}[{idx}]");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let ops = [arr(rd, t), arr(rn, t), lane(rm, elem, idx)];
        prop_assert!(
            encode_neon_elem(&ops, 0, 0b1000).is_err(),
            "index {} out of range for .{} must Err (llvm-mc rejects {})",
            idx,
            elem,
            asm
        );
    }

    #[test]
    fn encode_neon_elem_neg_unsupported_t(
        rd in reg_num(),
        rn in reg_num(),
        rm in 0u32..=15,
        idx in 0u32..=3,
        t in prop::sample::select(vec!["8b", "16b", "2d", "1d", "4b", ""]),
    ) {
        let ops = [arr(rd, t), arr(rn, t), lane(rm, "h", idx)];
        prop_assert!(
            encode_neon_elem(&ops, 0, 0b1000).is_err(),
            "unsupported dest arrangement {} must Err",
            t
        );
    }

    #[test]
    fn encode_neon_elem_neg_lane_elem_mismatch(
        rd in reg_num(),
        rn in reg_num(),
        rm_raw in reg_num(),
        idx_raw in 0u32..=3,
        shape in valid_shape(),
        wrong in prop::sample::select(vec!["b", "h", "s", "d"]),
    ) {
        let (t, elem, imax, rmmax) = shape;
        prop_assume!(wrong != elem);
        let rm = rm_raw % (rmmax + 1);
        let idx = idx_raw % (imax + 1);
        let asm = format!("mul v{rd}.{t}, v{rn}.{t}, v{rm}.{wrong}[{idx}]");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let ops = [arr(rd, t), arr(rn, t), lane(rm, wrong, idx)];
        prop_assert!(
            encode_neon_elem(&ops, 0, 0b1000).is_err(),
            "lane elem_size {} must match arrangement {} (llvm-mc rejects {})",
            wrong,
            elem,
            asm
        );
    }

    #[test]
    fn encode_neon_elem_diff_alt_spellings(
        rd in reg_num(),
        rn in reg_num(),
        rm_raw in reg_num(),
        idx_raw in 0u32..=7,
        shape in valid_shape(),
        insn in insn_table(),
    ) {
        let (t, elem, imax, rmmax) = shape;
        let (u, opcode, mnem) = insn;
        let rm = rm_raw % (rmmax + 1);
        let idx = idx_raw % (imax + 1);
        let asm = format!("{mnem} V{rd}.{t}, V{rn}.{t}, V{rm}.{elem}[{idx}]");
        let mc = llvm_mc_word(&asm).expect(&asm);
        let ops = [
            Operand::RegArrangement {
                reg: format!("V{rd}"),
                arrangement: t.to_string(),
            },
            Operand::RegArrangement {
                reg: format!("V{rn}"),
                arrangement: t.to_string(),
            },
            Operand::RegLane {
                reg: format!("V{rm}"),
                elem_size: elem.to_string(),
                index: idx,
            },
        ];
        let sut = sut_word(&ops, u, opcode).expect(&asm);
        prop_assert_eq!(sut, mc, "uppercase V prefix must match llvm-mc {}", asm);
    }
}

#[test]
fn test_encode_neon_elem_regression_extra_operand() {
    let ops = [
        arr(0, "4h"),
        arr(0, "4h"),
        lane(0, "h", 0),
        arr(0, "4h"),
    ];
    assert!(
        encode_neon_elem(&ops, 0, 0b1000).is_err(),
        "mul v0.4h, v0.4h, v0.h[0], v0.4h must Err (llvm-mc/gas reject a fourth operand)"
    );
}

#[test]
fn test_encode_neon_elem_regression_mismatch_t() {
    let ops = [arr(0, "4h"), arr(0, "8b"), lane(0, "h", 0)];
    assert!(
        encode_neon_elem(&ops, 0, 0b1000).is_err(),
        "mul v0.4h, v0.8b, v0.h[0] must Err (llvm-mc/gas require matching T)"
    );
}

#[test]
fn test_encode_neon_elem_regression_h_rm_hi() {
    let ops = [arr(0, "4h"), arr(0, "4h"), lane(16, "h", 0)];
    assert!(
        encode_neon_elem(&ops, 0, 0b1000).is_err(),
        "mul v0.4h, v0.4h, v16.h[0] must Err (ARM size=01 Rm v0-v15; llvm-mc rejects)"
    );
}

#[test]
fn test_encode_neon_elem_regression_x_prefix() {
    let ops = [
        Operand::RegArrangement {
            reg: "x0".into(),
            arrangement: "4h".to_string(),
        },
        arr(0, "4h"),
        lane(0, "h", 0),
    ];
    assert!(
        encode_neon_elem(&ops, 0, 0b1000).is_err(),
        "mul x0.4h, v0.4h, v0.h[0] must Err (llvm-mc/gas require Vd)"
    );
}

#[test]
fn test_encode_neon_elem_regression_index_oob() {
    let ops = [arr(0, "4h"), arr(0, "4h"), lane(0, "h", 8)];
    assert!(
        encode_neon_elem(&ops, 0, 0b1000).is_err(),
        "mul v0.4h, v0.4h, v0.h[8] must Err (llvm-mc: vector lane must be in range [0, 7])"
    );
}

#[test]
fn test_encode_neon_elem_regression_lane_elem_mismatch() {
    let ops = [arr(0, "4h"), arr(0, "4h"), lane(0, "b", 0)];
    assert!(
        encode_neon_elem(&ops, 0, 0b1000).is_err(),
        "mul v0.4h, v0.4h, v0.b[0] must Err (llvm-mc/gas require Vm.h[index] for .4h)"
    );
}
