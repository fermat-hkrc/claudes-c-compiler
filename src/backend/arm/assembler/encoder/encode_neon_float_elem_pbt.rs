// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md:231 NEON by-element lists fmul/fmla/fmls (with lane index);
//   encoder/mod.rs:1-7 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:456-459 fmul RegLane => encode_neon_float_elem(operands, 1, 0b1001);
//   encoder/mod.rs:534-545 fmla/fmls RegLane => encode_neon_float_elem;
//   ARM ARM Advanced SIMD vector x indexed element (floating-point):
//     FMUL/FMLA/FMLS/FMULX Vd.T, Vn.T, Vm.Ts[index];
//     T in {2S,4S,2D};
//     size=10 (S) Rm v0-v31 index H:L 0..3, M=Rm[4];
//     size=11 (D) Rm v0-v31 index H 0..1, L=0, M=Rm[4];
//     FMUL U=0 opcode=1001; FMLA U=0 opcode=0001; FMLS U=0 opcode=0101;
//     FMULX U=1 opcode=1001.
// Stronger considered:
//   - State machine: rejected — encode_neon_float_elem is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree FP by-element decoder
//   - Differential vs encode_neon_elem: rejected — same-job gate (integer by-element)
//   - Differential vs encode_neon_float_three_same: rejected — vector (non-lane)
//   - Differential vs encode_fp_arith: rejected — scalar FMUL
// Weaker available: algebraic.metamorphic (Rd/Rn/U fields), algebraic.invariant (word layout),
//   negative_error (arity / extra / dest-src mismatch / GPR dest / non-V prefix / index OOB)
// Differential: candidate=encode_neon_float_elem, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[RegArrangement(Vd,T), RegArrangement(Vn,T), RegLane(Vm,Ts,idx)] + (U, opcode)
//     <-> `{fmul|fmla|fmls|fmulx} Vd.T, Vn.T, Vm.Ts[idx]`

use super::encode_neon_float_elem;
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
    match encode_neon_float_elem(ops, u_bit, opcode)? {
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

/// Dest/src T, lane elem, max index (inclusive).
fn valid_shape() -> impl Strategy<Value = (&'static str, &'static str, u32)> {
    prop::sample::select(vec![
        ("2s", "s", 3u32),
        ("4s", "s", 3),
        ("2d", "d", 1),
    ])
}

/// ARM-correct (U, opcode, mnemonic) for FP by-element.
fn insn_table() -> impl Strategy<Value = (u32, u32, &'static str)> {
    prop::sample::select(vec![
        (0u32, 0b1001u32, "fmul"),
        (0u32, 0b0001u32, "fmla"),
        (0u32, 0b0101u32, "fmls"),
        (1u32, 0b1001u32, "fmulx"),
    ])
}

fn opcode_bits() -> impl Strategy<Value = u32> {
    prop::sample::select(vec![0b0001u32, 0b0101u32, 0b1001u32])
}

fn size_of(t: &str) -> u32 {
    match t {
        "2s" | "4s" => 0b10,
        "2d" => 0b11,
        _ => 0xff,
    }
}

fn q_of(t: &str) -> u32 {
    match t {
        "4s" | "2d" => 1,
        _ => 0,
    }
}

fn index_hlm(size: u32, index: u32, rm: u32) -> (u32, u32, u32) {
    match size {
        0b10 => ((index >> 1) & 1, index & 1, (rm >> 4) & 1),
        0b11 => (index & 1, 0u32, (rm >> 4) & 1),
        _ => (0xff, 0xff, 0xff),
    }
}

// Mapping KAT gate: llvm-mc must produce the known ARM encoding before PBT.
#[test]
fn encode_neon_float_elem_kat_llvm_mc_fmul_v0_2s_v1_2s_v2_s0() {
    let want = 0x0f829020u32;
    let mc = llvm_mc_word("fmul v0.2s, v1.2s, v2.s[0]").expect("llvm-mc KAT mapping");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
}

#[test]
fn encode_neon_float_elem_kat_llvm_mc_variants() {
    let mc_fmul = llvm_mc_word("fmul v0.2s, v1.2s, v2.s[0]").expect("llvm-mc KAT fmul");
    let sut_fmul = sut_word(&[arr(0, "2s"), arr(1, "2s"), lane(2, "s", 0)], 0, 0b1001)
        .expect("SUT KAT fmul");
    assert_eq!(sut_fmul, mc_fmul, "SUT vs llvm-mc fmul v0.2s, v1.2s, v2.s[0]");
    assert_eq!(mc_fmul, 0x0f829020u32);

    let mc_4s = llvm_mc_word("fmul v0.4s, v1.4s, v2.s[3]").expect("llvm-mc KAT fmul 4s");
    let sut_4s = sut_word(&[arr(0, "4s"), arr(1, "4s"), lane(2, "s", 3)], 0, 0b1001)
        .expect("SUT KAT fmul 4s");
    assert_eq!(sut_4s, mc_4s, "SUT vs llvm-mc fmul v0.4s, v1.4s, v2.s[3]");
    assert_eq!(mc_4s, 0x4fa29820u32);

    let mc_2d = llvm_mc_word("fmul v0.2d, v1.2d, v2.d[1]").expect("llvm-mc KAT fmul 2d");
    let sut_2d = sut_word(&[arr(0, "2d"), arr(1, "2d"), lane(2, "d", 1)], 0, 0b1001)
        .expect("SUT KAT fmul 2d");
    assert_eq!(sut_2d, mc_2d, "SUT vs llvm-mc fmul v0.2d, v1.2d, v2.d[1]");
    assert_eq!(mc_2d, 0x4fc29820u32);

    let mc_fmla = llvm_mc_word("fmla v0.4s, v1.4s, v31.s[2]").expect("llvm-mc KAT fmla");
    let sut_fmla = sut_word(&[arr(0, "4s"), arr(1, "4s"), lane(31, "s", 2)], 0, 0b0001)
        .expect("SUT KAT fmla");
    assert_eq!(sut_fmla, mc_fmla, "SUT vs llvm-mc fmla v0.4s, v1.4s, v31.s[2]");
    assert_eq!(mc_fmla, 0x4f9f1820u32);

    let mc_fmls = llvm_mc_word("fmls v0.2s, v1.2s, v16.s[1]").expect("llvm-mc KAT fmls");
    let sut_fmls = sut_word(&[arr(0, "2s"), arr(1, "2s"), lane(16, "s", 1)], 0, 0b0101)
        .expect("SUT KAT fmls");
    assert_eq!(sut_fmls, mc_fmls, "SUT vs llvm-mc fmls v0.2s, v1.2s, v16.s[1]");
    assert_eq!(mc_fmls, 0x0fb05020u32);

    let mc_x = llvm_mc_word("fmulx v0.4s, v1.4s, v2.s[0]").expect("llvm-mc KAT fmulx");
    let sut_x = sut_word(&[arr(0, "4s"), arr(1, "4s"), lane(2, "s", 0)], 1, 0b1001)
        .expect("SUT KAT fmulx");
    assert_eq!(sut_x, mc_x, "SUT vs llvm-mc fmulx v0.4s, v1.4s, v2.s[0]");
    assert_eq!(mc_x, 0x6f829020u32);
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_float_elem_diff_llvm_mc(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        idx_raw in 0u32..=3,
        shape in valid_shape(),
        insn in insn_table(),
    ) {
        let (t, elem, imax) = shape;
        let (u, opcode, mnem) = insn;
        let idx = idx_raw % (imax + 1);
        let asm = format!("{mnem} v{rd}.{t}, v{rn}.{t}, v{rm}.{elem}[{idx}]");
        let mc = llvm_mc_word(&asm).expect(&asm);
        let sut = sut_word(&[arr(rd, t), arr(rn, t), lane(rm, elem, idx)], u, opcode)
            .expect(&asm);
        prop_assert_eq!(sut, mc, "SUT vs llvm-mc for {}", asm);
    }

    #[test]
    fn encode_neon_float_elem_meta_rd_rn_u(
        rd1 in reg_num(),
        rd2 in reg_num(),
        rn1 in reg_num(),
        rn2 in reg_num(),
        rm in reg_num(),
        idx_raw in 0u32..=3,
        shape in valid_shape(),
        opcode in opcode_bits(),
    ) {
        let (t, elem, imax) = shape;
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
    fn encode_neon_float_elem_inv_layout(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        idx_raw in 0u32..=3,
        shape in valid_shape(),
        u in 0u32..=1,
        opcode in opcode_bits(),
    ) {
        let (t, elem, imax) = shape;
        let idx = idx_raw % (imax + 1);
        let w = sut_word(&[arr(rd, t), arr(rn, t), lane(rm, elem, idx)], u, opcode)
            .expect("layout");
        let size = size_of(t);
        let q = q_of(t);
        let (h, l, m) = index_hlm(size, idx, rm);
        prop_assert_eq!(w >> 31, 0);
        prop_assert_eq!((w >> 30) & 1, q);
        prop_assert_eq!((w >> 29) & 1, u);
        prop_assert_eq!((w >> 24) & 0x1F, 0b01111);
        prop_assert_eq!((w >> 22) & 0b11, size, "size must be 10 (S) or 11 (D)");
        prop_assert_eq!((w >> 21) & 1, l);
        prop_assert_eq!((w >> 20) & 1, m);
        prop_assert_eq!((w >> 16) & 0x1F, rm);
        prop_assert_eq!((w >> 12) & 0xF, opcode);
        prop_assert_eq!((w >> 11) & 1, h);
        prop_assert_eq!((w >> 10) & 1, 0);
        prop_assert_eq!((w >> 5) & 0x1F, rn);
        prop_assert_eq!(w & 0x1F, rd);
    }

    #[test]
    fn encode_neon_float_elem_neg_arity(
        n in 0usize..=2,
        rd in reg_num(),
        rn in reg_num(),
        shape in valid_shape(),
    ) {
        let (t, _elem, _) = shape;
        let ops: Vec<Operand> = match n {
            0 => vec![],
            1 => vec![arr(rd, t)],
            _ => vec![arr(rd, t), arr(rn, t)],
        };
        prop_assert!(
            encode_neon_float_elem(&ops, 0, 0b1001).is_err(),
            "arity {} must Err (NEON float by-element requires 3 operands)",
            n
        );
    }

    #[test]
    fn encode_neon_float_elem_neg_extra(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        extra in reg_num(),
        idx_raw in 0u32..=3,
        shape in valid_shape(),
        insn in insn_table(),
    ) {
        let (t, elem, imax) = shape;
        let (u, opcode, mnem) = insn;
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
            encode_neon_float_elem(&ops, u, opcode).is_err(),
            "extra operand must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_float_elem_neg_mismatch_t(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        idx_raw in 0u32..=3,
        shape in valid_shape(),
        t_wrong in prop::sample::select(vec!["8b", "16b", "4h", "8h", "2s", "4s", "2d", "1d"]),
    ) {
        let (t, elem, imax) = shape;
        prop_assume!(t_wrong != t);
        let idx = idx_raw % (imax + 1);
        let asm = format!("fmul v{rd}.{t}, v{rn}.{t_wrong}, v{rm}.{elem}[{idx}]");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let ops = [arr(rd, t), arr(rn, t_wrong), lane(rm, elem, idx)];
        prop_assert!(
            encode_neon_float_elem(&ops, 0, 0b1001).is_err(),
            "mismatched source T must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_float_elem_neg_gpr_bare_nonv(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        idx in 0u32..=3,
        kind in 0u8..=5,
    ) {
        let (ops, asm) = match kind {
            0 => {
                let asm = format!("fmul x{rd}, v{rn}.4s, v{rm}.s[{idx}]");
                let ops = vec![gpr("x", rd), arr(rn, "4s"), lane(rm, "s", idx)];
                (ops, asm)
            }
            1 => {
                let asm = format!("fmul w{rd}, v{rn}.4s, v{rm}.s[{idx}]");
                let ops = vec![gpr("w", rd), arr(rn, "4s"), lane(rm, "s", idx)];
                (ops, asm)
            }
            2 => {
                let asm = format!("fmul sp, v{rn}.4s, v{rm}.s[{idx}]");
                let ops = vec![Operand::Reg("sp".into()), arr(rn, "4s"), lane(rm, "s", idx)];
                (ops, asm)
            }
            3 => {
                let asm = format!("fmul v{rd}, v{rn}, v{rm}.s[{idx}]");
                let ops = vec![gpr("v", rd), gpr("v", rn), lane(rm, "s", idx)];
                (ops, asm)
            }
            4 => {
                let asm = format!("fmul x{rd}.4s, v{rn}.4s, v{rm}.s[{idx}]");
                let ops = vec![
                    Operand::RegArrangement {
                        reg: format!("x{rd}"),
                        arrangement: "4s".to_string(),
                    },
                    arr(rn, "4s"),
                    lane(rm, "s", idx),
                ];
                (ops, asm)
            }
            _ => {
                let asm = format!("fmul s{rd}, v{rn}.4s, v{rm}.s[{idx}]");
                let ops = vec![gpr("s", rd), arr(rn, "4s"), lane(rm, "s", idx)];
                (ops, asm)
            }
        };
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        prop_assert!(
            encode_neon_float_elem(&ops, 0, 0b1001).is_err(),
            "non-arranged NEON / GPR / SP / non-V prefix must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_float_elem_neg_index_oob(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        shape in valid_shape(),
        extra in 1u32..=8,
    ) {
        let (t, elem, imax) = shape;
        let idx = imax + extra;
        let asm = format!("fmul v{rd}.{t}, v{rn}.{t}, v{rm}.{elem}[{idx}]");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let ops = [arr(rd, t), arr(rn, t), lane(rm, elem, idx)];
        prop_assert!(
            encode_neon_float_elem(&ops, 0, 0b1001).is_err(),
            "index {} out of range for .{} must Err (llvm-mc rejects {})",
            idx,
            elem,
            asm
        );
    }

    #[test]
    fn encode_neon_float_elem_neg_unsupported_t(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        idx in 0u32..=3,
        t in prop::sample::select(vec!["8b", "16b", "4h", "8h", "1d", "4b", ""]),
    ) {
        let ops = [arr(rd, t), arr(rn, t), lane(rm, "s", idx)];
        prop_assert!(
            encode_neon_float_elem(&ops, 0, 0b1001).is_err(),
            "unsupported dest arrangement {} must Err",
            t
        );
    }

    #[test]
    fn encode_neon_float_elem_meta_alt_spellings(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        idx_raw in 0u32..=3,
        shape in valid_shape(),
        insn in insn_table(),
    ) {
        let (t, elem, imax) = shape;
        let (u, opcode, _mnem) = insn;
        let idx = idx_raw % (imax + 1);
        let lower = sut_word(&[arr(rd, t), arr(rn, t), lane(rm, elem, idx)], u, opcode)
            .expect("lower");
        let upper_ops = [
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
        let upper = sut_word(&upper_ops, u, opcode).expect("upper");
        prop_assert_eq!(upper, lower, "uppercase V prefix must encode the same as v");
    }

    #[test]
    fn encode_neon_float_elem_neg_lane_elem_mismatch(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        idx_raw in 0u32..=3,
        shape in valid_shape(),
        wrong in prop::sample::select(vec!["b", "h", "s", "d"]),
    ) {
        let (t, elem, imax) = shape;
        prop_assume!(wrong != elem);
        let idx = idx_raw % (imax + 1);
        let asm = format!("fmul v{rd}.{t}, v{rn}.{t}, v{rm}.{wrong}[{idx}]");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let ops = [arr(rd, t), arr(rn, t), lane(rm, wrong, idx)];
        prop_assert!(
            encode_neon_float_elem(&ops, 0, 0b1001).is_err(),
            "lane elem_size {} must match arrangement {} (llvm-mc rejects {})",
            wrong,
            elem,
            asm
        );
    }
}

#[test]
fn test_encode_neon_float_elem_regression_size_bit23() {
    let ops = [arr(0, "2s"), arr(1, "2s"), lane(2, "s", 0)];
    let sut = sut_word(&ops, 0, 0b1001).expect("SUT");
    let mc = 0x0f829020u32;
    assert_eq!(
        sut, mc,
        "fmul v0.2s, v1.2s, v2.s[0] must set size=10 (bit 23); got {sut:#010x}"
    );
}

#[test]
fn test_encode_neon_float_elem_regression_extra_operand() {
    let ops = [
        arr(0, "4s"),
        arr(0, "4s"),
        lane(0, "s", 0),
        arr(0, "4s"),
    ];
    assert!(
        encode_neon_float_elem(&ops, 0, 0b1001).is_err(),
        "fmul v0.4s, v0.4s, v0.s[0], v0.4s must Err (llvm-mc/gas reject a fourth operand)"
    );
}

#[test]
fn test_encode_neon_float_elem_regression_mismatch_t() {
    let ops = [arr(0, "4s"), arr(0, "2s"), lane(0, "s", 0)];
    assert!(
        encode_neon_float_elem(&ops, 0, 0b1001).is_err(),
        "fmul v0.4s, v0.2s, v0.s[0] must Err (llvm-mc/gas require matching T)"
    );
}

#[test]
fn test_encode_neon_float_elem_regression_x_prefix() {
    let ops = [
        Operand::RegArrangement {
            reg: "x0".into(),
            arrangement: "4s".to_string(),
        },
        arr(0, "4s"),
        lane(0, "s", 0),
    ];
    assert!(
        encode_neon_float_elem(&ops, 0, 0b1001).is_err(),
        "fmul x0.4s, v0.4s, v0.s[0] must Err (llvm-mc/gas require Vd)"
    );
}

#[test]
fn test_encode_neon_float_elem_regression_index_oob() {
    let ops = [arr(0, "4s"), arr(0, "4s"), lane(0, "s", 4)];
    assert!(
        encode_neon_float_elem(&ops, 0, 0b1001).is_err(),
        "fmul v0.4s, v0.4s, v0.s[4] must Err (llvm-mc: vector lane must be in range [0, 3])"
    );
}

#[test]
fn test_encode_neon_float_elem_regression_lane_elem_mismatch() {
    let ops = [arr(0, "4s"), arr(0, "4s"), lane(0, "d", 0)];
    assert!(
        encode_neon_float_elem(&ops, 0, 0b1001).is_err(),
        "fmul v0.4s, v0.4s, v0.d[0] must Err (llvm-mc/gas require lane size to match T)"
    );
}
