// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md:234 NEON insert/move lists mvni;
//   encoder/mod.rs:1-7 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:961 "mvni" => encode_neon_mvni;
//   neon.rs:1331 Encode NEON MVNI Vd.T, #imm (move bitwise NOT immediate to vector);
//   neon.rs:1367 MVNI: 0 Q 1 0 1111 00 abc cmode 01 defgh Rd  (op=1);
//   ARM ARM Advanced SIMD modified immediate (MVNI).
// Stronger considered:
//   - State machine: rejected — encode_neon_mvni is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree MVNI decoder
//   - Differential vs encode_neon_movi: rejected — same-job gate
//     (MOVI inverted-immediate dual, op=0, extra 8B/16B/2D forms)
// Weaker available: algebraic.metamorphic (Rd field), algebraic.invariant (word layout),
//   negative_error (arity / extra / invalid T / imm OOR / illegal shift)
// Differential: candidate=encode_neon_mvni, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[RegArrangement(Vd,T), Imm(imm), Shift?] <-> `mvni Vd.T, #imm {, lsl/msl #n}`

use super::encode_neon_mvni;
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

fn shift_op(kind: &str, amount: u32) -> Operand {
    Operand::Shift {
        kind: kind.to_string(),
        amount,
    }
}

fn sut_word(ops: &[Operand]) -> Result<u32, String> {
    match encode_neon_mvni(ops)? {
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

fn q_bit(t: &str) -> u32 {
    match t {
        "8h" | "4s" => 1,
        _ => 0,
    }
}

fn cmode_for(t: &str, kind: Option<&str>, amount: u32) -> u32 {
    match (t, kind, amount) {
        ("4h" | "8h", Some("lsl"), 8) => 0b1010,
        ("4h" | "8h", _, _) => 0b1000,
        ("2s" | "4s", Some("lsl"), 8) => 0b0010,
        ("2s" | "4s", Some("lsl"), 16) => 0b0100,
        ("2s" | "4s", Some("lsl"), 24) => 0b0110,
        ("2s" | "4s", Some("msl"), 8) => 0b1100,
        ("2s" | "4s", Some("msl"), 16) => 0b1101,
        ("2s" | "4s", _, _) => 0b0000,
        _ => 0,
    }
}

#[derive(Clone, Debug)]
struct MvniValid {
    rd: u32,
    t: &'static str,
    imm8: u32,
    shift: Option<(&'static str, u32)>,
}

impl MvniValid {
    fn ops(&self) -> Vec<Operand> {
        let mut ops = vec![arr(self.rd, self.t), Operand::Imm(self.imm8 as i64)];
        if let Some((kind, amt)) = self.shift {
            ops.push(shift_op(kind, amt));
        }
        ops
    }

    fn asm(&self) -> String {
        match self.shift {
            Some((kind, amt)) => {
                format!(
                    "mvni v{}.{t}, #{}, {kind} #{amt}",
                    self.rd,
                    self.imm8,
                    t = self.t
                )
            }
            None => format!("mvni v{}.{t}, #{}", self.rd, self.imm8, t = self.t),
        }
    }
}

fn reg_num() -> impl Strategy<Value = u32> {
    prop_oneof![Just(0u32), Just(1u32), Just(30u32), Just(31u32), 0u32..=31]
}

fn imm8_edge() -> impl Strategy<Value = u32> {
    prop_oneof![
        Just(0u32),
        Just(1u32),
        Just(0x1Fu32),
        Just(0x20u32),
        Just(0xAAu32),
        Just(0xFFu32),
        0u32..=255
    ]
}

fn valid_mvni() -> impl Strategy<Value = MvniValid> {
    let half = (reg_num(), prop::sample::select(vec!["4h", "8h"]), imm8_edge()).prop_map(
        |(rd, t, imm8)| MvniValid {
            rd,
            t,
            imm8,
            shift: None,
        },
    );
    let single = (
        reg_num(),
        prop::sample::select(vec!["2s", "4s"]),
        imm8_edge(),
        prop::sample::select(vec![0u32, 8, 16, 24]),
    )
        .prop_map(|(rd, t, imm8, sh)| MvniValid {
            rd,
            t,
            imm8,
            shift: if sh == 0 {
                None
            } else {
                Some(("lsl", sh))
            },
        });
    prop_oneof![half, single]
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_neon_mvni_kat_llvm_mc() {
    let cases: &[(&str, u32, Vec<Operand>)] = &[
        (
            "mvni v0.4s, #0",
            0x6f000400,
            vec![arr(0, "4s"), Operand::Imm(0)],
        ),
        (
            "mvni v0.2s, #0",
            0x2f000400,
            vec![arr(0, "2s"), Operand::Imm(0)],
        ),
        (
            "mvni v0.4s, #255",
            0x6f0707e0,
            vec![arr(0, "4s"), Operand::Imm(255)],
        ),
        (
            "mvni v31.4s, #0xaa",
            0x6f05055f,
            vec![arr(31, "4s"), Operand::Imm(0xaa)],
        ),
        (
            "mvni v0.4s, #1, lsl #8",
            0x6f002420,
            vec![arr(0, "4s"), Operand::Imm(1), shift_op("lsl", 8)],
        ),
        (
            "mvni v0.4s, #1, lsl #16",
            0x6f004420,
            vec![arr(0, "4s"), Operand::Imm(1), shift_op("lsl", 16)],
        ),
        (
            "mvni v0.4s, #1, lsl #24",
            0x6f006420,
            vec![arr(0, "4s"), Operand::Imm(1), shift_op("lsl", 24)],
        ),
        (
            "mvni v0.2s, #1",
            0x2f000420,
            vec![arr(0, "2s"), Operand::Imm(1)],
        ),
        (
            "mvni v0.8h, #1",
            0x6f008420,
            vec![arr(0, "8h"), Operand::Imm(1)],
        ),
        (
            "mvni v0.4h, #1",
            0x2f008420,
            vec![arr(0, "4h"), Operand::Imm(1)],
        ),
        (
            "mvni v0.2s, #0, msl #8",
            0x2f00c400,
            vec![arr(0, "2s"), Operand::Imm(0), shift_op("msl", 8)],
        ),
        (
            "mvni v0.4s, #1, msl #16",
            0x6f00d420,
            vec![arr(0, "4s"), Operand::Imm(1), shift_op("msl", 16)],
        ),
    ];
    for (asm, want, ops) in cases {
        let mc = llvm_mc_word(asm).unwrap_or_else(|e| panic!("llvm-mc KAT {asm}: {e}"));
        assert_eq!(mc, *want, "llvm-mc KAT mapping broken for {asm}");
        let sut = sut_word(ops).unwrap_or_else(|e| panic!("SUT KAT {asm}: {e}"));
        assert_eq!(sut, *want, "SUT KAT {asm}");
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_mvni_diff_llvm_mc(v in valid_mvni()) {
        let asm = v.asm();
        let ops = v.ops();
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&ops)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for {}", asm);
    }

    #[test]
    fn encode_neon_mvni_diff_h_lsl8(
        rd in reg_num(),
        t in prop::sample::select(vec!["4h", "8h"]),
        imm8 in imm8_edge(),
    ) {
        let asm = format!("mvni v{rd}.{t}, #{imm8}, lsl #8");
        let ops = vec![arr(rd, t), Operand::Imm(imm8 as i64), shift_op("lsl", 8)];
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&ops)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for {}", asm);
    }

    #[test]
    fn encode_neon_mvni_diff_s_msl(
        rd in reg_num(),
        t in prop::sample::select(vec!["2s", "4s"]),
        imm8 in imm8_edge(),
        n in prop::sample::select(vec![8u32, 16]),
    ) {
        let asm = format!("mvni v{rd}.{t}, #{imm8}, msl #{n}");
        let ops = vec![arr(rd, t), Operand::Imm(imm8 as i64), shift_op("msl", n)];
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&ops)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for {}", asm);
    }

    #[test]
    fn encode_neon_mvni_metamorphic_rd(v in valid_mvni(), rd2 in reg_num()) {
        let mut v2 = v.clone();
        v2.rd = rd2;
        let w1 = sut_word(&v.ops())
            .unwrap_or_else(|e| panic!("SUT rejected rd1 {}: {}", v.asm(), e));
        let w2 = sut_word(&v2.ops())
            .unwrap_or_else(|e| panic!("SUT rejected rd2 {}: {}", v2.asm(), e));
        prop_assert_eq!(
            (w1 ^ w2) & !0x1Fu32,
            0u32,
            "changing only Rd must differ only in bits[4:0] ({} vs {})",
            v.asm(),
            v2.asm()
        );
        prop_assert_eq!(w2 & 0x1F, rd2, "Rd field");
    }

    #[test]
    fn encode_neon_mvni_invariant_arm_fields(v in valid_mvni()) {
        let w = sut_word(&v.ops())
            .unwrap_or_else(|e| panic!("SUT rejected {}: {}", v.asm(), e));
        let (kind, amt) = match v.shift {
            Some((k, a)) => (Some(k), a),
            None => (None, 0),
        };
        let cmode = cmode_for(v.t, kind, amt);
        let abc = (v.imm8 >> 5) & 0x7;
        let defgh = v.imm8 & 0x1F;
        prop_assert_eq!(w >> 31, 0u32, "bit31=0 {}", v.asm());
        prop_assert_eq!((w >> 30) & 1, q_bit(v.t), "Q {}", v.asm());
        prop_assert_eq!((w >> 29) & 1, 1u32, "op {}", v.asm());
        prop_assert_eq!((w >> 24) & 0x1F, 0b01111u32, "bits[28:24] {}", v.asm());
        prop_assert_eq!((w >> 23) & 1, 0u32, "bit23 {}", v.asm());
        prop_assert_eq!((w >> 19) & 0xF, 0u32, "bits[22:19] {}", v.asm());
        prop_assert_eq!((w >> 16) & 0x7, abc, "abc {}", v.asm());
        prop_assert_eq!((w >> 12) & 0xF, cmode, "cmode {}", v.asm());
        prop_assert_eq!((w >> 11) & 1, 0u32, "o2 {}", v.asm());
        prop_assert_eq!((w >> 10) & 1, 1u32, "bit10 {}", v.asm());
        prop_assert_eq!((w >> 5) & 0x1F, defgh, "defgh {}", v.asm());
        prop_assert_eq!(w & 0x1F, v.rd, "Rd {}", v.asm());
    }

    #[test]
    fn encode_neon_mvni_neg_arity(
        n in 0usize..=1,
        rd in reg_num(),
        t in prop::sample::select(vec!["4h", "8h", "2s", "4s"]),
        imm8 in imm8_edge(),
    ) {
        let all = vec![arr(rd, t), Operand::Imm(imm8 as i64)];
        let arity_ops: Vec<Operand> = all.iter().take(n).cloned().collect();
        let arity_asm = if n == 0 {
            "mvni".to_string()
        } else {
            format!("mvni v{rd}.{t}")
        };
        prop_assert!(
            llvm_mc_word(&arity_asm).is_err(),
            "llvm-mc unexpectedly accepted arity-{} {}",
            n,
            arity_asm
        );
        prop_assert!(
            encode_neon_mvni(&arity_ops).is_err(),
            "arity {} must Err (llvm-mc rejects {})",
            n,
            arity_asm
        );
    }

    #[test]
    fn encode_neon_mvni_neg_extra_and_illegal_shift(
        rd in reg_num(),
        extra in reg_num(),
        t in prop::sample::select(vec!["4h", "8h", "2s", "4s"]),
        imm8 in imm8_edge(),
        kind in 0u8..=3,
    ) {
        let (ops, asm) = match kind {
            0 => {
                let asm = format!("mvni v{rd}.{t}, #{imm8}, v{extra}.{t}");
                let ops = vec![
                    arr(rd, t),
                    Operand::Imm(imm8 as i64),
                    arr(extra, t),
                ];
                (ops, asm)
            }
            1 => {
                let amt = 32u32;
                let asm = format!("mvni v{rd}.{t}, #{imm8}, lsl #{amt}");
                let ops = vec![
                    arr(rd, t),
                    Operand::Imm(imm8 as i64),
                    shift_op("lsl", amt),
                ];
                (ops, asm)
            }
            2 => {
                let asm = format!("mvni v{rd}.{t}, #{imm8}, lsr #8");
                let ops = vec![
                    arr(rd, t),
                    Operand::Imm(imm8 as i64),
                    shift_op("lsr", 8),
                ];
                (ops, asm)
            }
            _ => {
                let asm = format!("mvni v{rd}.{t}, #{imm8}, lsl #0, #{extra}");
                let ops = vec![
                    arr(rd, t),
                    Operand::Imm(imm8 as i64),
                    shift_op("lsl", 0),
                    Operand::Imm(extra as i64),
                ];
                (ops, asm)
            }
        };
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted extra/illegal {}",
            asm
        );
        prop_assert!(
            encode_neon_mvni(&ops).is_err(),
            "extra/illegal shift must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_mvni_neg_imm_oor_invalid_t(
        rd in reg_num(),
        kind in 0u8..=3,
        t_ok in prop::sample::select(vec!["4h", "8h", "2s", "4s"]),
        t_bad in prop::sample::select(vec!["8b", "16b", "2d", "1d", "2h", "8d", "b", "q", "1s"]),
        over in 1i64..=256,
        neg in 1i64..=256,
    ) {
        let (ops, asm) = match kind {
            0 => {
                let imm = 255 + over;
                let asm = format!("mvni v{rd}.{t_ok}, #{imm}");
                let ops = vec![arr(rd, t_ok), Operand::Imm(imm)];
                (ops, asm)
            }
            1 => {
                let imm = -neg;
                let asm = format!("mvni v{rd}.{t_ok}, #{imm}");
                let ops = vec![arr(rd, t_ok), Operand::Imm(imm)];
                (ops, asm)
            }
            2 => {
                let asm = format!("mvni v{rd}.{t_bad}, #0");
                let ops = vec![arr(rd, t_bad), Operand::Imm(0)];
                (ops, asm)
            }
            _ => {
                let imm = 256i64;
                let asm = format!("mvni v{rd}.{t_ok}, #{imm}");
                let ops = vec![arr(rd, t_ok), Operand::Imm(imm)];
                (ops, asm)
            }
        };
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        prop_assert!(
            encode_neon_mvni(&ops).is_err(),
            "OOR imm / invalid T must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_mvni_neg_documented_errors(
        rd in reg_num(),
        kind in 0u8..=2,
        t_bad in prop::sample::select(vec!["8b", "16b", "2d", "1d", "2h", "8d", "b", "q", "1s"]),
        t_s in prop::sample::select(vec!["2s", "4s"]),
        imm8 in imm8_edge(),
        bad_lsl in prop::sample::select(vec![1u32, 4, 7, 9, 32, 48]),
        bad_msl in prop::sample::select(vec![0u32, 1, 4, 24, 32]),
    ) {
        let (ops, asm) = match kind {
            0 => {
                let asm = format!("mvni v{rd}.{t_bad}, #0");
                let ops = vec![arr(rd, t_bad), Operand::Imm(0)];
                (ops, asm)
            }
            1 => {
                let asm = format!("mvni v{rd}.{t_s}, #{imm8}, lsl #{bad_lsl}");
                let ops = vec![
                    arr(rd, t_s),
                    Operand::Imm(imm8 as i64),
                    shift_op("lsl", bad_lsl),
                ];
                (ops, asm)
            }
            _ => {
                let asm = format!("mvni v{rd}.{t_s}, #{imm8}, msl #{bad_msl}");
                let ops = vec![
                    arr(rd, t_s),
                    Operand::Imm(imm8 as i64),
                    shift_op("msl", bad_msl),
                ];
                (ops, asm)
            }
        };
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        prop_assert!(
            encode_neon_mvni(&ops).is_err(),
            "documented error path must Err (llvm-mc rejects {})",
            asm
        );
    }
}

#[test]
fn test_encode_neon_mvni_regression_h_lsl8() {
    let ops = vec![arr(0, "4h"), Operand::Imm(0), shift_op("lsl", 8)];
    let sut = sut_word(&ops).expect("valid mvni v0.4h, #0, lsl #8");
    let mc = llvm_mc_word("mvni v0.4h, #0, lsl #8").expect("llvm-mc");
    assert_eq!(
        sut, mc,
        "mvni v0.4h, #0, lsl #8 must match llvm-mc (cmode=1010, 0x2f00a400)"
    );
    assert_eq!(mc, 0x2f00a400);
}

#[test]
fn test_encode_neon_mvni_regression_extra_operand() {
    let ops = vec![arr(0, "4s"), Operand::Imm(0), arr(0, "4s")];
    assert!(
        encode_neon_mvni(&ops).is_err(),
        "mvni v0.4s, #0, v0.4s must Err (gas/llvm-mc reject a third non-shift operand)"
    );
}

#[test]
fn test_encode_neon_mvni_regression_imm_oor() {
    let ops = vec![arr(0, "4s"), Operand::Imm(256)];
    assert!(
        encode_neon_mvni(&ops).is_err(),
        "mvni v0.4s, #256 must Err (llvm-mc: immediate must be in range [0, 255])"
    );
}

#[test]
fn test_encode_neon_mvni_regression_illegal_h_shift() {
    let ops = vec![arr(0, "4h"), Operand::Imm(0), shift_op("lsl", 32)];
    assert!(
        encode_neon_mvni(&ops).is_err(),
        "mvni v0.4h, #0, lsl #32 must Err (llvm-mc rejects illegal H LSL amount)"
    );
}

#[test]
fn test_encode_neon_mvni_regression_lsr() {
    let ops = vec![arr(0, "4s"), Operand::Imm(0), shift_op("lsr", 8)];
    assert!(
        encode_neon_mvni(&ops).is_err(),
        "mvni v0.4s, #0, lsr #8 must Err (llvm-mc rejects LSR on MVNI)"
    );
}
