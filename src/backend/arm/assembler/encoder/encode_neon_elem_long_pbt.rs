// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md:230 NEON widen/long lists smull/umull/smlal/umlal/smlsl/umlsl (+ 2);
//   README.md:231 NEON by-element lists smull/umull/smlal/umlal (with lane index);
//   encoder/mod.rs:1-7 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:318-329 smull/umull RegLane => encode_neon_elem_long;
//   encoder/mod.rs:749-775 sqdmlal/sqdmlsl/sqdmull (+2);
//   encoder/mod.rs:837-890 umlal/smlal/umlsl/smlsl/umull2/smull2;
//   neon.rs:230 Encode NEON vector-by-element long instructions: SMULL/UMULL/SMLAL/UMLAL/SMLSL/UMLSL (elem);
//   neon.rs:232 Format: 0 Q U 01111 size L M Rm opcode H 0 Rn Rd;
//   ARM ARM Advanced SIMD vector x indexed element (long/widening):
//     SMULL/UMULL/SMLAL/UMLAL/SMLSL/UMLSL/SQDMULL/SQDMLAL/SQDMLSL
//     Vd.{4S,2D}, Vn.{4H,8H,2S,4S}, Vm.{H,S}[index];
//     size=00/11 reserved; size=01 Rm v0-v15 index H:L:M 0..7;
//     size=10 Rm v0-v31 (M=Rm[4]) index H:L 0..3.
// Stronger considered:
//   - State machine: rejected — encode_neon_elem_long is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree by-element-long decoder
//   - Differential vs encode_neon_elem: rejected — same-job gate (non-widening MUL/MLA/MLS)
//   - Differential vs encode_neon_three_diff: rejected — vector (non-lane) long form
//   - Differential vs encode_neon_float_elem: rejected — FP by-element
// Weaker available: algebraic.metamorphic (Rd/Rn/U fields), algebraic.invariant (word layout),
//   negative_error (arity / extra / dest mismatch / H-Rm v16-v31 / GPR dest / non-V prefix)
// Differential: candidate=encode_neon_elem_long, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[RegArrangement(Vd,Ta), RegArrangement(Vn,Tb), RegLane(Vm,elem,idx)] + (U, opcode, is_high)
//     <-> `{smull|umull|smlal|umlal|smlsl|umlsl|sqdmull|sqdmlal|sqdmlsl}[+2] Vd.Ta, Vn.Tb, Vm.elem[idx]`

use super::encode_neon_elem_long;
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

fn sut_word(
    ops: &[Operand],
    u_bit: u32,
    opcode: u32,
    is_high: bool,
) -> Result<u32, String> {
    match encode_neon_elem_long(ops, u_bit, opcode, is_high)? {
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
    prop_oneof![Just(0u32), Just(1u32), Just(15u32), Just(16u32), Just(30u32), Just(31u32), 0u32..=31]
}

/// Source arrangement, dest arrangement, lane elem, max index, max Rm, is_high.
fn valid_shape() -> impl Strategy<Value = (&'static str, &'static str, &'static str, u32, u32, bool)> {
    prop::sample::select(vec![
        ("4h", "4s", "h", 7u32, 15u32, false),
        ("8h", "4s", "h", 7, 15, true),
        ("2s", "2d", "s", 3, 31, false),
        ("4s", "2d", "s", 3, 31, true),
    ])
}

/// ARM-correct (U, opcode, base mnemonic) for long by-element.
fn insn_table() -> impl Strategy<Value = (u32, u32, &'static str)> {
    prop::sample::select(vec![
        (0u32, 0b1010u32, "smull"),
        (1u32, 0b1010u32, "umull"),
        (0u32, 0b0010u32, "smlal"),
        (1u32, 0b0010u32, "umlal"),
        (0u32, 0b0110u32, "smlsl"),
        (1u32, 0b0110u32, "umlsl"),
        (0u32, 0b1011u32, "sqdmull"),
        (0u32, 0b0011u32, "sqdmlal"),
        (0u32, 0b0111u32, "sqdmlsl"),
    ])
}

fn opcode_bits() -> impl Strategy<Value = u32> {
    prop::sample::select(vec![
        0b0010u32, 0b0011u32, 0b0110u32, 0b0111u32, 0b1010u32, 0b1011u32,
    ])
}

fn size_of(tb: &str) -> u32 {
    match tb {
        "4h" | "8h" => 0b01,
        "2s" | "4s" => 0b10,
        _ => 0xff,
    }
}

fn q_of(is_high: bool) -> u32 {
    if is_high {
        1
    } else {
        0
    }
}

fn index_hlm(size: u32, index: u32, rm: u32) -> (u32, u32, u32) {
    match size {
        0b01 => ((index >> 2) & 1, (index >> 1) & 1, index & 1),
        0b10 => ((index >> 1) & 1, index & 1, (rm >> 4) & 1),
        _ => (0xff, 0xff, 0xff),
    }
}

fn mnem_of(base: &str, is_high: bool) -> String {
    if is_high {
        format!("{base}2")
    } else {
        base.to_string()
    }
}

fn mandated_ta(tb: &str) -> &'static str {
    match tb {
        "4h" | "8h" => "4s",
        "2s" | "4s" => "2d",
        _ => "",
    }
}

// Reference KAT gate (must pass before PBT).
#[test]
fn encode_neon_elem_long_kat_llvm_mc_smull_v0_4s_v1_4h_v2_h2() {
    let want = 0x0f62a020u32;
    let mc = llvm_mc_word("smull v0.4s, v1.4h, v2.h[2]").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [arr(0, "4s"), arr(1, "4h"), lane(2, "h", 2)];
    let sut = sut_word(&ops, 0, 0b1010, false).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_neon_elem_long_kat_llvm_mc_smull2_v0_4s_v1_8h_v2_h7() {
    let want = 0x4f72a820u32;
    let mc = llvm_mc_word("smull2 v0.4s, v1.8h, v2.h[7]").expect("llvm-mc KAT smull2");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken for smull2");
    let ops = [arr(0, "4s"), arr(1, "8h"), lane(2, "h", 7)];
    let sut = sut_word(&ops, 0, 0b1010, true).expect("SUT KAT smull2");
    assert_eq!(sut, want);
}

#[test]
fn encode_neon_elem_long_kat_llvm_mc_smull_v0_2d_v1_2s_v31_s3() {
    let want = 0x0fbfa820u32;
    let mc = llvm_mc_word("smull v0.2d, v1.2s, v31.s[3]").expect("llvm-mc KAT .s v31");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken for .s v31");
    let ops = [arr(0, "2d"), arr(1, "2s"), lane(31, "s", 3)];
    let sut = sut_word(&ops, 0, 0b1010, false).expect("SUT KAT .s v31");
    assert_eq!(sut, want);
}

#[test]
fn encode_neon_elem_long_kat_llvm_mc_umlal_sqdmull_bounds() {
    let mc_u = llvm_mc_word("umlal v0.4s, v1.4h, v0.h[0]").expect("llvm-mc KAT umlal");
    let sut_u = sut_word(&[arr(0, "4s"), arr(1, "4h"), lane(0, "h", 0)], 1, 0b0010, false)
        .expect("SUT KAT umlal");
    assert_eq!(sut_u, mc_u, "SUT vs llvm-mc umlal v0.4s, v1.4h, v0.h[0]");

    let mc_q = llvm_mc_word("sqdmull v31.2d, v30.2s, v15.s[1]").expect("llvm-mc KAT sqdmull");
    let sut_q = sut_word(&[arr(31, "2d"), arr(30, "2s"), lane(15, "s", 1)], 0, 0b1011, false)
        .expect("SUT KAT sqdmull");
    assert_eq!(sut_q, mc_q, "SUT vs llvm-mc sqdmull v31.2d, v30.2s, v15.s[1]");
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_elem_long_diff_llvm_mc(
        rd in reg_num(),
        rn in reg_num(),
        rm_raw in reg_num(),
        idx_raw in 0u32..=7,
        shape in valid_shape(),
        insn in insn_table(),
    ) {
        let (tb, ta, elem, imax, rmmax, is_high) = shape;
        let (u, opcode, base) = insn;
        let rm = rm_raw % (rmmax + 1);
        let idx = idx_raw % (imax + 1);
        let mnem = mnem_of(base, is_high);
        let asm = format!("{mnem} v{rd}.{ta}, v{rn}.{tb}, v{rm}.{elem}[{idx}]");
        let mc = llvm_mc_word(&asm).expect(&asm);
        let sut = sut_word(&[arr(rd, ta), arr(rn, tb), lane(rm, elem, idx)], u, opcode, is_high)
            .expect(&asm);
        prop_assert_eq!(sut, mc, "SUT vs llvm-mc for {}", asm);
    }

    #[test]
    fn encode_neon_elem_long_meta_rd_rn_u(
        rd1 in reg_num(),
        rd2 in reg_num(),
        rn1 in reg_num(),
        rn2 in reg_num(),
        rm_raw in reg_num(),
        idx_raw in 0u32..=7,
        shape in valid_shape(),
        opcode in opcode_bits(),
    ) {
        let (tb, ta, elem, imax, rmmax, is_high) = shape;
        let rm = rm_raw % (rmmax + 1);
        let idx = idx_raw % (imax + 1);
        let ops = |rd: u32, rn: u32| [arr(rd, ta), arr(rn, tb), lane(rm, elem, idx)];
        let w11 = sut_word(&ops(rd1, rn1), 0, opcode, is_high).expect("w11");
        let w21 = sut_word(&ops(rd2, rn1), 0, opcode, is_high).expect("w21");
        let w12 = sut_word(&ops(rd1, rn2), 0, opcode, is_high).expect("w12");
        let w_u1 = sut_word(&ops(rd1, rn1), 1, opcode, is_high).expect("w_u1");
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
    fn encode_neon_elem_long_inv_layout(
        rd in reg_num(),
        rn in reg_num(),
        rm_raw in reg_num(),
        idx_raw in 0u32..=7,
        shape in valid_shape(),
        u in 0u32..=1,
        opcode in opcode_bits(),
    ) {
        let (tb, ta, elem, imax, rmmax, is_high) = shape;
        let rm = rm_raw % (rmmax + 1);
        let idx = idx_raw % (imax + 1);
        let w = sut_word(&[arr(rd, ta), arr(rn, tb), lane(rm, elem, idx)], u, opcode, is_high)
            .expect("layout");
        let size = size_of(tb);
        let q = q_of(is_high);
        let (h, l, m) = index_hlm(size, idx, rm);
        let rm_lo = if size == 0b01 { rm & 0xF } else { rm & 0xF };
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
    fn encode_neon_elem_long_neg_arity(
        n in 0usize..=2,
        rd in reg_num(),
        rn in reg_num(),
        shape in valid_shape(),
    ) {
        let (tb, ta, _elem, _, _, is_high) = shape;
        let ops: Vec<Operand> = match n {
            0 => vec![],
            1 => vec![arr(rd, ta)],
            _ => vec![arr(rd, ta), arr(rn, tb)],
        };
        prop_assert!(
            encode_neon_elem_long(&ops, 0, 0b1010, is_high).is_err(),
            "arity {} must Err (NEON elem-long requires 3 operands)",
            n
        );
    }

    #[test]
    fn encode_neon_elem_long_neg_extra(
        rd in reg_num(),
        rn in reg_num(),
        rm_raw in reg_num(),
        extra in reg_num(),
        idx_raw in 0u32..=7,
        shape in valid_shape(),
        insn in insn_table(),
    ) {
        let (tb, ta, elem, imax, rmmax, is_high) = shape;
        let (u, opcode, base) = insn;
        let rm = rm_raw % (rmmax + 1);
        let idx = idx_raw % (imax + 1);
        let mnem = mnem_of(base, is_high);
        let asm = format!("{mnem} v{rd}.{ta}, v{rn}.{tb}, v{rm}.{elem}[{idx}], v{extra}.{ta}");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted extra {}",
            asm
        );
        let ops = [
            arr(rd, ta),
            arr(rn, tb),
            lane(rm, elem, idx),
            arr(extra, ta),
        ];
        prop_assert!(
            encode_neon_elem_long(&ops, u, opcode, is_high).is_err(),
            "extra operand must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_elem_long_neg_mismatch_ta(
        rd in reg_num(),
        rn in reg_num(),
        rm_raw in reg_num(),
        idx_raw in 0u32..=7,
        shape in valid_shape(),
        ta_wrong in prop::sample::select(vec!["8b", "16b", "4h", "8h", "2s", "4s", "2d", "1d"]),
    ) {
        let (tb, ta, elem, imax, rmmax, is_high) = shape;
        prop_assume!(ta_wrong != ta);
        let rm = rm_raw % (rmmax + 1);
        let idx = idx_raw % (imax + 1);
        let mnem = mnem_of("smull", is_high);
        let asm = format!("{mnem} v{rd}.{ta_wrong}, v{rn}.{tb}, v{rm}.{elem}[{idx}]");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let ops = [arr(rd, ta_wrong), arr(rn, tb), lane(rm, elem, idx)];
        prop_assert!(
            encode_neon_elem_long(&ops, 0, 0b1010, is_high).is_err(),
            "mismatched dest Ta must Err (llvm-mc rejects {}; mandated Ta={})",
            asm,
            mandated_ta(tb)
        );
    }

    #[test]
    fn encode_neon_elem_long_neg_h_rm_hi(
        rd in reg_num(),
        rn in reg_num(),
        rm in 16u32..=31,
        idx in 0u32..=7,
        tb in prop::sample::select(vec!["4h", "8h"]),
    ) {
        let is_high = tb == "8h";
        let mnem = mnem_of("smull", is_high);
        let asm = format!("{mnem} v{rd}.4s, v{rn}.{tb}, v{rm}.h[{idx}]");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let ops = [arr(rd, "4s"), arr(rn, tb), lane(rm, "h", idx)];
        prop_assert!(
            encode_neon_elem_long(&ops, 0, 0b1010, is_high).is_err(),
            "H-lane Rm v16-v31 must Err (ARM size=01 Rm v0-v15; llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_elem_long_neg_gpr_bare_nonv(
        rd in reg_num(),
        rn in reg_num(),
        rm in 0u32..=15,
        idx in 0u32..=7,
        kind in 0u8..=6,
    ) {
        let (ops, asm) = match kind {
            0 => {
                let asm = format!("smull x{rd}, v{rn}.4h, v{rm}.h[{idx}]");
                let ops = vec![gpr("x", rd), arr(rn, "4h"), lane(rm, "h", idx)];
                (ops, asm)
            }
            1 => {
                let asm = format!("smull w{rd}, v{rn}.4h, v{rm}.h[{idx}]");
                let ops = vec![gpr("w", rd), arr(rn, "4h"), lane(rm, "h", idx)];
                (ops, asm)
            }
            2 => {
                let asm = format!("smull sp, v{rn}.4h, v{rm}.h[{idx}]");
                let ops = vec![Operand::Reg("sp".into()), arr(rn, "4h"), lane(rm, "h", idx)];
                (ops, asm)
            }
            3 => {
                let asm = format!("smull v{rd}, v{rn}, v{rm}.h[{idx}]");
                let ops = vec![gpr("v", rd), gpr("v", rn), lane(rm, "h", idx)];
                (ops, asm)
            }
            4 => {
                let asm = format!("smull x{rd}.4s, v{rn}.4h, v{rm}.h[{idx}]");
                let ops = vec![
                    Operand::RegArrangement {
                        reg: format!("x{rd}"),
                        arrangement: "4s".to_string(),
                    },
                    arr(rn, "4h"),
                    lane(rm, "h", idx),
                ];
                (ops, asm)
            }
            5 => {
                let asm = format!("smull v{rd}.4s, v{rn}.4h, v{rm}.4h");
                let ops = vec![arr(rd, "4s"), arr(rn, "4h"), arr(rm, "4h")];
                (ops, asm)
            }
            _ => {
                let asm = format!("smull s{rd}, v{rn}.4h, v{rm}.h[{idx}]");
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
            encode_neon_elem_long(&ops, 0, 0b1010, false).is_err(),
            "non-arranged NEON / GPR / SP / non-V prefix / non-lane third must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_elem_long_neg_index_oob(
        rd in reg_num(),
        rn in reg_num(),
        rm_raw in reg_num(),
        shape in valid_shape(),
        extra in 1u32..=8,
    ) {
        let (tb, ta, elem, imax, rmmax, is_high) = shape;
        let rm = rm_raw % (rmmax + 1);
        let idx = imax + extra;
        let mnem = mnem_of("smull", is_high);
        let asm = format!("{mnem} v{rd}.{ta}, v{rn}.{tb}, v{rm}.{elem}[{idx}]");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let ops = [arr(rd, ta), arr(rn, tb), lane(rm, elem, idx)];
        prop_assert!(
            encode_neon_elem_long(&ops, 0, 0b1010, is_high).is_err(),
            "index {} out of range for .{} must Err (llvm-mc rejects {})",
            idx,
            elem,
            asm
        );
    }

    #[test]
    fn encode_neon_elem_long_neg_unsupported_src(
        rd in reg_num(),
        rn in reg_num(),
        rm in 0u32..=15,
        idx in 0u32..=3,
        tb in prop::sample::select(vec!["8b", "16b", "2d", "1d", "4b", ""]),
    ) {
        let ops = [arr(rd, "4s"), arr(rn, tb), lane(rm, "h", idx)];
        prop_assert!(
            encode_neon_elem_long(&ops, 0, 0b1010, false).is_err(),
            "unsupported source arrangement {} must Err",
            tb
        );
    }

    #[test]
    fn encode_neon_elem_long_neg_lane_elem_mismatch(
        rd in reg_num(),
        rn in reg_num(),
        rm_raw in reg_num(),
        idx_raw in 0u32..=3,
        shape in valid_shape(),
        wrong in prop::sample::select(vec!["b", "h", "s", "d"]),
    ) {
        let (tb, ta, elem, imax, rmmax, is_high) = shape;
        prop_assume!(wrong != elem);
        let rm = rm_raw % (rmmax + 1);
        let idx = idx_raw % (imax + 1);
        let mnem = mnem_of("smull", is_high);
        let asm = format!("{mnem} v{rd}.{ta}, v{rn}.{tb}, v{rm}.{wrong}[{idx}]");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let ops = [arr(rd, ta), arr(rn, tb), lane(rm, wrong, idx)];
        prop_assert!(
            encode_neon_elem_long(&ops, 0, 0b1010, is_high).is_err(),
            "lane elem_size {} must match source {} (llvm-mc rejects {})",
            wrong,
            elem,
            asm
        );
    }

    #[test]
    fn encode_neon_elem_long_diff_alt_spellings(
        rd in reg_num(),
        rn in reg_num(),
        rm_raw in reg_num(),
        idx_raw in 0u32..=7,
        shape in valid_shape(),
        insn in insn_table(),
    ) {
        let (tb, ta, elem, imax, rmmax, is_high) = shape;
        let (u, opcode, base) = insn;
        let rm = rm_raw % (rmmax + 1);
        let idx = idx_raw % (imax + 1);
        let mnem = mnem_of(base, is_high);
        let asm = format!("{mnem} V{rd}.{ta}, V{rn}.{tb}, V{rm}.{elem}[{idx}]");
        let mc = llvm_mc_word(&asm).expect(&asm);
        let ops = [
            Operand::RegArrangement {
                reg: format!("V{rd}"),
                arrangement: ta.to_string(),
            },
            Operand::RegArrangement {
                reg: format!("V{rn}"),
                arrangement: tb.to_string(),
            },
            Operand::RegLane {
                reg: format!("V{rm}"),
                elem_size: elem.to_string(),
                index: idx,
            },
        ];
        let sut = sut_word(&ops, u, opcode, is_high).expect(&asm);
        prop_assert_eq!(sut, mc, "uppercase V prefix must match llvm-mc {}", asm);
    }
}

#[test]
fn test_encode_neon_elem_long_regression_extra_operand() {
    let ops = [
        arr(0, "4s"),
        arr(0, "4h"),
        lane(0, "h", 0),
        arr(0, "4s"),
    ];
    assert!(
        encode_neon_elem_long(&ops, 0, 0b1010, false).is_err(),
        "smull v0.4s, v0.4h, v0.h[0], v0.4s must Err (llvm-mc/gas reject a fourth operand)"
    );
}

#[test]
fn test_encode_neon_elem_long_regression_mismatch_ta() {
    let ops = [arr(0, "8b"), arr(0, "4h"), lane(0, "h", 0)];
    assert!(
        encode_neon_elem_long(&ops, 0, 0b1010, false).is_err(),
        "smull v0.8b, v0.4h, v0.h[0] must Err (llvm-mc/gas require dest .4s for .4h source)"
    );
}

#[test]
fn test_encode_neon_elem_long_regression_h_rm_hi() {
    let ops = [arr(0, "4s"), arr(0, "4h"), lane(16, "h", 0)];
    assert!(
        encode_neon_elem_long(&ops, 0, 0b1010, false).is_err(),
        "smull v0.4s, v0.4h, v16.h[0] must Err (ARM size=01 Rm v0-v15; llvm-mc rejects)"
    );
}

#[test]
fn test_encode_neon_elem_long_regression_gpr_dest() {
    let ops = [gpr("x", 0), arr(0, "4h"), lane(0, "h", 0)];
    assert!(
        encode_neon_elem_long(&ops, 0, 0b1010, false).is_err(),
        "smull x0, v0.4h, v0.h[0] must Err (llvm-mc/gas require Vd.4s)"
    );
}

#[test]
fn test_encode_neon_elem_long_regression_lane_elem_mismatch() {
    let ops = [arr(0, "2d"), arr(0, "2s"), lane(0, "b", 0)];
    assert!(
        encode_neon_elem_long(&ops, 0, 0b1010, false).is_err(),
        "smull v0.2d, v0.2s, v0.b[0] must Err (llvm-mc/gas require Vm.s[index] for .2s source)"
    );
}
