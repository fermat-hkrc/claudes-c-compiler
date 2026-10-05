// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md:227 NEON compare-zero lists cmgt/cmge/cmeq/cmle/cmlt #0;
//   encoder/mod.rs:1-7 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:566-584 cmeq/cmge/cmgt Imm(0) => encode_neon_cmp_zero;
//   encoder/mod.rs:665-666 cmlt/cmle => encode_neon_cmp_zero;
//   neon.rs:186 Encode NEON compare-to-zero: CMEQ Vd, Vn, #0, CMGE Vd, Vn, #0, etc.;
//   neon.rs:188 Format: 0 Q U 01110 size 10000 opcode 10 Rn Rd;
//   ARM ARM Advanced SIMD two-register miscellaneous integer compare-with-zero
//     (CMEQ/CMGE/CMGT/CMLE/CMLT Vd.T, Vn.T, #0; T in {8B,16B,4H,8H,2S,4S,2D};
//      size:Q=11:0 / .1d reserved).
// Stronger considered:
//   - State machine: rejected — encode_neon_cmp_zero is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree integer compare-zero decoder
//   - Differential vs encode_neon_float_cmp_zero: rejected — same-job gate (FP size=1sz)
//   - Differential vs encode_neon_three_same: rejected — register-register CMEQ/CMGE/CMGT
//   - Differential vs encode_neon_two_misc: rejected — different two-misc opcodes
// Weaker available: algebraic.metamorphic (Rd/Rn/U fields), algebraic.invariant (word layout),
//   negative_error (arity / extra / reserved 1d / mismatched T / GPR dest / non-V prefix)
// Differential: candidate=encode_neon_cmp_zero, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[RegArrangement(Vd,T), RegArrangement(Vn,T)] + (U, opcode)
//     <-> `{cmeq|cmge|cmgt|cmle|cmlt} Vd.T, Vn.T, #0`

use super::encode_neon_cmp_zero;
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

fn gpr(prefix: &str, n: u32) -> Operand {
    Operand::Reg(format!("{}{}", prefix, n))
}

fn sut_word(ops: &[Operand], u_bit: u32, opcode: u32) -> Result<u32, String> {
    match encode_neon_cmp_zero(ops, u_bit, opcode)? {
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

fn valid_t() -> impl Strategy<Value = &'static str> {
    prop::sample::select(vec!["8b", "16b", "4h", "8h", "2s", "4s", "2d"])
}

/// ARM-correct (U, opcode, mnemonic) for integer compare-to-zero.
fn insn_table() -> impl Strategy<Value = (u32, u32, &'static str)> {
    prop::sample::select(vec![
        (0u32, 0b01001u32, "cmeq"),
        (1u32, 0b01000u32, "cmge"),
        (0u32, 0b01000u32, "cmgt"),
        (1u32, 0b01001u32, "cmle"),
        (0u32, 0b01010u32, "cmlt"),
    ])
}

fn q_size_of(t: &str) -> (u32, u32) {
    match t {
        "8b" => (0, 0b00),
        "16b" => (1, 0b00),
        "4h" => (0, 0b01),
        "8h" => (1, 0b01),
        "2s" => (0, 0b10),
        "4s" => (1, 0b10),
        "2d" => (1, 0b11),
        _ => (0xff, 0xff),
    }
}

fn opcode_bits() -> impl Strategy<Value = u32> {
    prop::sample::select(vec![0b01000u32, 0b01001u32, 0b01010u32])
}

// Reference KAT gate (must pass before PBT).
#[test]
fn encode_neon_cmp_zero_kat_llvm_mc_cmeq_v0_8b_v1_8b() {
    let want = 0x0e209820u32;
    let mc = llvm_mc_word("cmeq v0.8b, v1.8b, #0").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [arr(0, "8b"), arr(1, "8b")];
    let sut = sut_word(&ops, 0, 0b01001).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_neon_cmp_zero_kat_llvm_mc_cmeq_v0_16b_v1_16b() {
    let want = 0x4e209820u32;
    let mc = llvm_mc_word("cmeq v0.16b, v1.16b, #0").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [arr(0, "16b"), arr(1, "16b")];
    let sut = sut_word(&ops, 0, 0b01001).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_neon_cmp_zero_kat_llvm_mc_cmge_v0_4s_v1_4s() {
    let want = 0x6ea08820u32;
    let mc = llvm_mc_word("cmge v0.4s, v1.4s, #0").expect("llvm-mc KAT cmge");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken for cmge");
    let ops = [arr(0, "4s"), arr(1, "4s")];
    let sut = sut_word(&ops, 1, 0b01000).expect("SUT KAT cmge");
    assert_eq!(sut, want);
}

#[test]
fn encode_neon_cmp_zero_kat_llvm_mc_cmgt_cmlt_cmle_bounds() {
    let want_gt = 0x4ee08820u32;
    let mc_gt = llvm_mc_word("cmgt v0.2d, v1.2d, #0").expect("llvm-mc KAT cmgt");
    assert_eq!(mc_gt, want_gt, "llvm-mc KAT mapping broken for cmgt .2d");
    let sut_gt = sut_word(&[arr(0, "2d"), arr(1, "2d")], 0, 0b01000).expect("SUT KAT cmgt");
    assert_eq!(sut_gt, want_gt);

    let want_le = 0x6e609820u32;
    let mc_le = llvm_mc_word("cmle v0.8h, v1.8h, #0").expect("llvm-mc KAT cmle");
    assert_eq!(mc_le, want_le, "llvm-mc KAT mapping broken for cmle");
    let sut_le = sut_word(&[arr(0, "8h"), arr(1, "8h")], 1, 0b01001).expect("SUT KAT cmle");
    assert_eq!(sut_le, want_le);

    let want_lt = 0x0e60a820u32;
    let mc_lt = llvm_mc_word("cmlt v0.4h, v1.4h, #0").expect("llvm-mc KAT cmlt");
    assert_eq!(mc_lt, want_lt, "llvm-mc KAT mapping broken for cmlt");
    let sut_lt = sut_word(&[arr(0, "4h"), arr(1, "4h")], 0, 0b01010).expect("SUT KAT cmlt");
    assert_eq!(sut_lt, want_lt);

    let want_31 = 0x0e209bffu32;
    let mc_31 = llvm_mc_word("cmeq v31.8b, v31.8b, #0").expect("llvm-mc KAT v31");
    assert_eq!(mc_31, want_31, "llvm-mc KAT mapping broken for v31");
    let sut_31 = sut_word(&[arr(31, "8b"), arr(31, "8b")], 0, 0b01001).expect("SUT KAT v31");
    assert_eq!(sut_31, want_31);
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_cmp_zero_diff_llvm_mc(
        rd in reg_num(),
        rn in reg_num(),
        t in valid_t(),
        insn in insn_table(),
    ) {
        let (u, opcode, mnem) = insn;
        let asm = format!("{mnem} v{rd}.{t}, v{rn}.{t}, #0");
        let mc = llvm_mc_word(&asm).expect(&asm);
        let sut = sut_word(&[arr(rd, t), arr(rn, t)], u, opcode).expect(&asm);
        prop_assert_eq!(sut, mc, "SUT vs llvm-mc for {}", asm);
    }

    #[test]
    fn encode_neon_cmp_zero_meta_rd_rn_u(
        rd1 in reg_num(),
        rd2 in reg_num(),
        rn1 in reg_num(),
        rn2 in reg_num(),
        t in valid_t(),
        opcode in opcode_bits(),
    ) {
        let w11 = sut_word(&[arr(rd1, t), arr(rn1, t)], 0, opcode).expect("w11");
        let w21 = sut_word(&[arr(rd2, t), arr(rn1, t)], 0, opcode).expect("w21");
        let w12 = sut_word(&[arr(rd1, t), arr(rn2, t)], 0, opcode).expect("w12");
        let w_u1 = sut_word(&[arr(rd1, t), arr(rn1, t)], 1, opcode).expect("w_u1");
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
    fn encode_neon_cmp_zero_inv_layout(
        rd in reg_num(),
        rn in reg_num(),
        t in valid_t(),
        u in 0u32..=1,
        opcode in opcode_bits(),
    ) {
        let w = sut_word(&[arr(rd, t), arr(rn, t)], u, opcode).expect("layout");
        let (q, size) = q_size_of(t);
        prop_assert_eq!(w >> 31, 0);
        prop_assert_eq!((w >> 30) & 1, q);
        prop_assert_eq!((w >> 29) & 1, u);
        prop_assert_eq!((w >> 24) & 0x1F, 0b01110);
        prop_assert_eq!((w >> 22) & 0b11, size);
        prop_assert_eq!((w >> 17) & 0x1F, 0b10000);
        prop_assert_eq!((w >> 12) & 0x1F, opcode);
        prop_assert_eq!((w >> 10) & 0b11, 0b10);
        prop_assert_eq!((w >> 5) & 0x1F, rn);
        prop_assert_eq!(w & 0x1F, rd);
    }

    #[test]
    fn encode_neon_cmp_zero_neg_arity(n in 0usize..=1, rd in reg_num(), t in valid_t()) {
        let ops: Vec<Operand> = match n {
            0 => vec![],
            _ => vec![arr(rd, t)],
        };
        prop_assert!(
            encode_neon_cmp_zero(&ops, 0, 0b01001).is_err(),
            "arity {} must Err (NEON compare-zero requires at least 2 operands)",
            n
        );
    }

    #[test]
    fn encode_neon_cmp_zero_neg_extra(
        rd in reg_num(),
        rn in reg_num(),
        extra in reg_num(),
        t in valid_t(),
        insn in insn_table(),
    ) {
        let (u, opcode, mnem) = insn;
        let asm = format!("{mnem} v{rd}.{t}, v{rn}.{t}, #0, v{extra}.{t}");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted extra {}",
            asm
        );
        let ops = [arr(rd, t), arr(rn, t), Operand::Imm(0), arr(extra, t)];
        prop_assert!(
            encode_neon_cmp_zero(&ops, u, opcode).is_err(),
            "extra operand must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_cmp_zero_neg_mismatch_t(
        rd in reg_num(),
        rn in reg_num(),
        td in valid_t(),
        tn in valid_t(),
    ) {
        prop_assume!(td != tn);
        let asm = format!("cmeq v{rd}.{td}, v{rn}.{tn}, #0");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let ops = [arr(rd, td), arr(rn, tn)];
        prop_assert!(
            encode_neon_cmp_zero(&ops, 0, 0b01001).is_err(),
            "mismatched T must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_cmp_zero_neg_reserved_1d(
        rd in reg_num(),
        rn in reg_num(),
        u in 0u32..=1,
        opcode in opcode_bits(),
    ) {
        let asm = format!("cmeq v{rd}.1d, v{rn}.1d, #0");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let ops = [arr(rd, "1d"), arr(rn, "1d")];
        prop_assert!(
            encode_neon_cmp_zero(&ops, u, opcode).is_err(),
            "reserved .1d must Err (ARM size:Q=11:0 reserved; llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_cmp_zero_neg_gpr_bare_nonv(
        rd in reg_num(),
        rn in reg_num(),
        t in valid_t(),
        kind in 0u8..=6,
    ) {
        // Scalar `cmeq Dd, Dn, #0` is a different ARM encoding (bits[31:30]=01);
        // llvm-mc accepts it, so it is not in this invalid-vector domain.
        let (ops, asm) = match kind {
            0 => {
                let asm = format!("cmeq x{rd}, x{rn}, #0");
                let ops = vec![gpr("x", rd), gpr("x", rn)];
                (ops, asm)
            }
            1 => {
                let asm = format!("cmeq w{rd}, v{rn}.{t}, #0");
                let ops = vec![gpr("w", rd), arr(rn, t)];
                (ops, asm)
            }
            2 => {
                let asm = "cmeq sp, v0.8b, #0".to_string();
                let ops = vec![Operand::Reg("sp".into()), arr(0, "8b")];
                (ops, asm)
            }
            3 => {
                let asm = format!("cmeq v{rd}, v{rn}, #0");
                let ops = vec![gpr("v", rd), gpr("v", rn)];
                (ops, asm)
            }
            4 => {
                let asm = format!("cmeq x{rd}.{t}, v{rn}.{t}, #0");
                let ops = vec![
                    Operand::RegArrangement {
                        reg: format!("x{rd}"),
                        arrangement: t.to_string(),
                    },
                    arr(rn, t),
                ];
                (ops, asm)
            }
            5 => {
                let asm = format!("cmeq v{rd}.{t}, x{rn}, #0");
                let ops = vec![arr(rd, t), gpr("x", rn)];
                (ops, asm)
            }
            _ => {
                let asm = format!("cmeq s{rd}, v{rn}.{t}, #0");
                let ops = vec![gpr("s", rd), arr(rn, t)];
                (ops, asm)
            }
        };
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        prop_assert!(
            encode_neon_cmp_zero(&ops, 0, 0b01001).is_err(),
            "non-arranged NEON / GPR / SP / non-V prefix must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_cmp_zero_neg_invalid_t(
        rd in reg_num(),
        rn in reg_num(),
        t in prop::sample::select(vec!["4b", "8d", "2h", "1s", "32b", ""]),
    ) {
        let asm = format!("cmeq v{rd}.{t}, v{rn}.{t}, #0");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let ops = [arr(rd, t), arr(rn, t)];
        prop_assert!(
            encode_neon_cmp_zero(&ops, 0, 0b01001).is_err(),
            "invalid T must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_cmp_zero_neg_nonreg(
        rd in reg_num(),
        t in valid_t(),
        kind in 0u8..=2,
    ) {
        let ops: Vec<Operand> = match kind {
            0 => vec![Operand::Imm(0), arr(rd, t)],
            1 => vec![arr(rd, t), Operand::Imm(0)],
            _ => vec![
                Operand::Mem {
                    base: "x0".into(),
                    offset: 0,
                },
                arr(rd, t),
            ],
        };
        prop_assert!(
            encode_neon_cmp_zero(&ops, 0, 0b01001).is_err(),
            "non-register dest/src must Err (kind {})",
            kind
        );
    }

    #[test]
    fn encode_neon_cmp_zero_diff_alt_spellings(
        rd in reg_num(),
        rn in reg_num(),
        t in valid_t(),
        insn in insn_table(),
    ) {
        let (u, opcode, mnem) = insn;
        let asm = format!("{mnem} V{rd}.{t}, V{rn}.{t}, #0");
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
        ];
        let sut = sut_word(&ops, u, opcode).expect(&asm);
        prop_assert_eq!(sut, mc, "uppercase V prefix must match llvm-mc {}", asm);
    }
}

#[test]
fn test_encode_neon_cmp_zero_regression_extra_operand() {
    let ops = [arr(0, "8b"), arr(0, "8b"), Operand::Imm(0), arr(0, "8b")];
    assert!(
        encode_neon_cmp_zero(&ops, 0, 0b01001).is_err(),
        "cmeq v0.8b, v0.8b, #0, v0.8b must Err (llvm-mc/gas reject a fourth operand)"
    );
}

#[test]
fn test_encode_neon_cmp_zero_regression_mismatch_t() {
    let ops = [arr(0, "16b"), arr(0, "8b")];
    assert!(
        encode_neon_cmp_zero(&ops, 0, 0b01001).is_err(),
        "cmeq v0.16b, v0.8b, #0 must Err (llvm-mc/gas require matching T)"
    );
}

#[test]
fn test_encode_neon_cmp_zero_regression_reserved_1d() {
    let ops = [arr(0, "1d"), arr(0, "1d")];
    assert!(
        encode_neon_cmp_zero(&ops, 0, 0b01000).is_err(),
        "cmeq v0.1d, v0.1d, #0 must Err (ARM size:Q=11:0 reserved; llvm-mc rejects)"
    );
}

#[test]
fn test_encode_neon_cmp_zero_regression_non_v_prefix() {
    let ops = [
        Operand::RegArrangement {
            reg: "x0".into(),
            arrangement: "8b".into(),
        },
        arr(0, "8b"),
    ];
    assert!(
        encode_neon_cmp_zero(&ops, 0, 0b01001).is_err(),
        "cmeq x0.8b, v0.8b, #0 must Err (llvm-mc/gas require Vd.T)"
    );
}

#[test]
fn test_encode_neon_cmp_zero_regression_gpr_src() {
    let ops = [arr(0, "8b"), gpr("x", 0)];
    assert!(
        encode_neon_cmp_zero(&ops, 0, 0b01001).is_err(),
        "cmeq v0.8b, x0, #0 must Err (llvm-mc/gas require Vn.T)"
    );
}
