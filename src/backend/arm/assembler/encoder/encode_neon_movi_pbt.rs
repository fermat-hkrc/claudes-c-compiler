// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md:234 NEON insert/move lists movi;
//   encoder/mod.rs:1-7 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:687 "movi" => encode_neon_movi;
//   neon.rs:623 Encode NEON MOVI (move immediate to vector);
//   neon.rs:634 Encoding: 0 Q 00 1111 00000 abc 1110 01 defgh Rd;
//   ARM ARM Advanced SIMD modified immediate (MOVI).
// Stronger considered:
//   - State machine: rejected — encode_neon_movi is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree MOVI decoder
//   - Differential vs encode_neon_mvni: rejected — same-job gate
//     (MVNI inverted immediate, op=1 on 2S/4S/4H/8H, no 8B/16B/2D form)
// Weaker available: algebraic.metamorphic (Rd field), algebraic.invariant (word layout),
//   negative_error (arity / extra / invalid T / imm OOR / illegal shift)
// Differential: candidate=encode_neon_movi, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[RegArrangement(Vd,T), Imm(imm), Shift?] <-> `movi Vd.T, #imm {, lsl/msl #n}`

use super::encode_neon_movi;
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
    match encode_neon_movi(ops)? {
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

fn expand_imm8_to_imm64(imm8: u32) -> i64 {
    let mut v = 0u64;
    for i in 0..8 {
        if (imm8 >> i) & 1 == 1 {
            v |= 0xFFu64 << (i * 8);
        }
    }
    v as i64
}

fn q_bit(t: &str) -> u32 {
    match t {
        "16b" | "8h" | "4s" | "2d" => 1,
        _ => 0,
    }
}

fn op_bit(t: &str) -> u32 {
    if t == "2d" {
        1
    } else {
        0
    }
}

fn cmode_for(t: &str, kind: Option<&str>, amount: u32) -> u32 {
    match (t, kind, amount) {
        ("8b" | "16b" | "2d", _, _) => 0b1110,
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
struct MoviValid {
    rd: u32,
    t: &'static str,
    imm8: u32,
    shift: Option<(&'static str, u32)>,
}

impl MoviValid {
    fn imm_i64(&self) -> i64 {
        if self.t == "2d" {
            expand_imm8_to_imm64(self.imm8)
        } else {
            self.imm8 as i64
        }
    }

    fn ops(&self) -> Vec<Operand> {
        let mut ops = vec![arr(self.rd, self.t), Operand::Imm(self.imm_i64())];
        if let Some((kind, amt)) = self.shift {
            ops.push(shift_op(kind, amt));
        }
        ops
    }

    fn asm(&self) -> String {
        let imm_txt = if self.t == "2d" {
            format!("#0x{:016x}", self.imm_i64() as u64)
        } else {
            format!("#{}", self.imm8)
        };
        match self.shift {
            Some((kind, amt)) => {
                format!("movi v{}.{t}, {imm_txt}, {kind} #{amt}", self.rd, t = self.t)
            }
            None => format!("movi v{}.{t}, {imm_txt}", self.rd, t = self.t),
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

fn valid_movi() -> impl Strategy<Value = MoviValid> {
    let byte = (reg_num(), prop::sample::select(vec!["8b", "16b"]), imm8_edge()).prop_map(
        |(rd, t, imm8)| MoviValid {
            rd,
            t,
            imm8,
            shift: None,
        },
    );
    let half = (reg_num(), prop::sample::select(vec!["4h", "8h"]), imm8_edge()).prop_map(
        |(rd, t, imm8)| MoviValid {
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
        .prop_map(|(rd, t, imm8, sh)| MoviValid {
            rd,
            t,
            imm8,
            shift: if sh == 0 {
                None
            } else {
                Some(("lsl", sh))
            },
        });
    let double = (reg_num(), imm8_edge()).prop_map(|(rd, imm8)| MoviValid {
        rd,
        t: "2d",
        imm8,
        shift: None,
    });
    prop_oneof![byte, half, single, double]
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_neon_movi_kat_llvm_mc() {
    let cases: &[(&str, u32, Vec<Operand>)] = &[
        (
            "movi v0.16b, #0",
            0x4f00e400,
            vec![arr(0, "16b"), Operand::Imm(0)],
        ),
        (
            "movi v0.8b, #0",
            0x0f00e400,
            vec![arr(0, "8b"), Operand::Imm(0)],
        ),
        (
            "movi v0.16b, #255",
            0x4f07e7e0,
            vec![arr(0, "16b"), Operand::Imm(255)],
        ),
        (
            "movi v31.16b, #0xaa",
            0x4f05e55f,
            vec![arr(31, "16b"), Operand::Imm(0xaa)],
        ),
        (
            "movi v0.4s, #0",
            0x4f000400,
            vec![arr(0, "4s"), Operand::Imm(0)],
        ),
        (
            "movi v0.4s, #255",
            0x4f0707e0,
            vec![arr(0, "4s"), Operand::Imm(255)],
        ),
        (
            "movi v0.4s, #1, lsl #8",
            0x4f002420,
            vec![arr(0, "4s"), Operand::Imm(1), shift_op("lsl", 8)],
        ),
        (
            "movi v0.4s, #1, lsl #16",
            0x4f004420,
            vec![arr(0, "4s"), Operand::Imm(1), shift_op("lsl", 16)],
        ),
        (
            "movi v0.4s, #1, lsl #24",
            0x4f006420,
            vec![arr(0, "4s"), Operand::Imm(1), shift_op("lsl", 24)],
        ),
        (
            "movi v0.2s, #1",
            0x0f000420,
            vec![arr(0, "2s"), Operand::Imm(1)],
        ),
        (
            "movi v0.8h, #1",
            0x4f008420,
            vec![arr(0, "8h"), Operand::Imm(1)],
        ),
        (
            "movi v0.4h, #1",
            0x0f008420,
            vec![arr(0, "4h"), Operand::Imm(1)],
        ),
        (
            "movi v0.2d, #0",
            0x6f00e400,
            vec![arr(0, "2d"), Operand::Imm(0)],
        ),
        (
            "movi v0.2d, #-1",
            0x6f07e7e0,
            vec![arr(0, "2d"), Operand::Imm(-1)],
        ),
        (
            "movi v0.2d, #0xff00000000000000",
            0x6f04e400,
            vec![arr(0, "2d"), Operand::Imm(0xff00000000000000u64 as i64)],
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
    fn encode_neon_movi_diff_llvm_mc(v in valid_movi()) {
        let asm = v.asm();
        let ops = v.ops();
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&ops)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for {}", asm);
    }

    #[test]
    fn encode_neon_movi_diff_h_lsl8(
        rd in reg_num(),
        t in prop::sample::select(vec!["4h", "8h"]),
        imm8 in imm8_edge(),
    ) {
        let asm = format!("movi v{rd}.{t}, #{imm8}, lsl #8");
        let ops = vec![arr(rd, t), Operand::Imm(imm8 as i64), shift_op("lsl", 8)];
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&ops)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for {}", asm);
    }

    #[test]
    fn encode_neon_movi_diff_s_msl(
        rd in reg_num(),
        t in prop::sample::select(vec!["2s", "4s"]),
        imm8 in imm8_edge(),
        n in prop::sample::select(vec![8u32, 16]),
    ) {
        let asm = format!("movi v{rd}.{t}, #{imm8}, msl #{n}");
        let ops = vec![arr(rd, t), Operand::Imm(imm8 as i64), shift_op("msl", n)];
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&ops)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for {}", asm);
    }

    #[test]
    fn encode_neon_movi_metamorphic_rd(v in valid_movi(), rd2 in reg_num()) {
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
    fn encode_neon_movi_invariant_arm_fields(v in valid_movi()) {
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
        prop_assert_eq!((w >> 29) & 1, op_bit(v.t), "op {}", v.asm());
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
    fn encode_neon_movi_neg_arity(
        n in 0usize..=1,
        rd in reg_num(),
        t in prop::sample::select(vec!["16b", "8b", "4s", "8h", "2d"]),
        imm8 in imm8_edge(),
    ) {
        let all = vec![arr(rd, t), Operand::Imm(imm8 as i64)];
        let arity_ops: Vec<Operand> = all.iter().take(n).cloned().collect();
        let arity_asm = if n == 0 {
            "movi".to_string()
        } else {
            format!("movi v{rd}.{t}")
        };
        prop_assert!(
            llvm_mc_word(&arity_asm).is_err(),
            "llvm-mc unexpectedly accepted arity-{} {}",
            n,
            arity_asm
        );
        prop_assert!(
            encode_neon_movi(&arity_ops).is_err(),
            "arity {} must Err (llvm-mc rejects {})",
            n,
            arity_asm
        );
    }

    #[test]
    fn encode_neon_movi_neg_extra_and_illegal_shift(
        rd in reg_num(),
        extra in reg_num(),
        t in prop::sample::select(vec!["8b", "16b", "4h", "8h", "2s", "4s", "2d"]),
        imm8 in imm8_edge(),
        kind in 0u8..=3,
    ) {
        let base_imm = if t == "2d" {
            expand_imm8_to_imm64(imm8)
        } else {
            imm8 as i64
        };
        let (ops, asm) = match kind {
            0 => {
                let asm = if t == "2d" {
                    format!(
                        "movi v{rd}.{t}, #0x{:016x}, v{extra}.{t}",
                        base_imm as u64
                    )
                } else {
                    format!("movi v{rd}.{t}, #{imm8}, v{extra}.{t}")
                };
                let ops = vec![
                    arr(rd, t),
                    Operand::Imm(base_imm),
                    arr(extra, t),
                ];
                (ops, asm)
            }
            1 => {
                let amt = if matches!(t, "8b" | "16b" | "2d") { 8u32 } else { 32u32 };
                let asm = if t == "2d" {
                    format!(
                        "movi v{rd}.{t}, #0x{:016x}, lsl #{amt}",
                        base_imm as u64
                    )
                } else {
                    format!("movi v{rd}.{t}, #{imm8}, lsl #{amt}")
                };
                let ops = vec![
                    arr(rd, t),
                    Operand::Imm(base_imm),
                    shift_op("lsl", amt),
                ];
                (ops, asm)
            }
            2 => {
                let asm = if t == "2d" {
                    format!(
                        "movi v{rd}.{t}, #0x{:016x}, lsr #8",
                        base_imm as u64
                    )
                } else {
                    format!("movi v{rd}.{t}, #{imm8}, lsr #8")
                };
                let ops = vec![
                    arr(rd, t),
                    Operand::Imm(base_imm),
                    shift_op("lsr", 8),
                ];
                (ops, asm)
            }
            _ => {
                let asm = format!("movi v{rd}.{t}, #{imm8}, lsl #0, #{extra}");
                let ops = vec![
                    arr(rd, t),
                    Operand::Imm(base_imm),
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
            encode_neon_movi(&ops).is_err(),
            "extra/illegal shift must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_movi_neg_imm_oor_invalid_t(
        rd in reg_num(),
        kind in 0u8..=3,
        t_ok in prop::sample::select(vec!["8b", "16b", "4h", "8h", "2s", "4s"]),
        t_bad in prop::sample::select(vec!["1d", "2h", "8d", "b", "q", "1s"]),
        over in 1i64..=256,
        neg in 1i64..=256,
        bad_byte in 1u8..=0xFEu8,
        byte_idx in 0u32..=7,
    ) {
        let (ops, asm) = match kind {
            0 => {
                let imm = 255 + over;
                let asm = format!("movi v{rd}.{t_ok}, #{imm}");
                let ops = vec![arr(rd, t_ok), Operand::Imm(imm)];
                (ops, asm)
            }
            1 => {
                let imm = -neg;
                let asm = format!("movi v{rd}.{t_ok}, #{imm}");
                let ops = vec![arr(rd, t_ok), Operand::Imm(imm)];
                (ops, asm)
            }
            2 => {
                let asm = format!("movi v{rd}.{t_bad}, #0");
                let ops = vec![arr(rd, t_bad), Operand::Imm(0)];
                (ops, asm)
            }
            _ => {
                let mut imm64 = 0u64;
                imm64 |= (bad_byte as u64) << (byte_idx * 8);
                let asm = format!("movi v{rd}.2d, #0x{imm64:016x}");
                let ops = vec![arr(rd, "2d"), Operand::Imm(imm64 as i64)];
                (ops, asm)
            }
        };
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        prop_assert!(
            encode_neon_movi(&ops).is_err(),
            "OOR imm / invalid T / bad 2d byte must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_movi_neg_documented_errors(
        rd in reg_num(),
        kind in 0u8..=2,
        t_bad in prop::sample::select(vec!["1d", "2h", "8d", "b", "q", "1s"]),
        t_s in prop::sample::select(vec!["2s", "4s"]),
        imm8 in imm8_edge(),
        bad_amt in prop::sample::select(vec![1u32, 4, 7, 9, 32, 48]),
        bad_byte in 1u8..=0xFEu8,
        byte_idx in 0u32..=7,
    ) {
        let (ops, asm) = match kind {
            0 => {
                let asm = format!("movi v{rd}.{t_bad}, #0");
                let ops = vec![arr(rd, t_bad), Operand::Imm(0)];
                (ops, asm)
            }
            1 => {
                let asm = format!("movi v{rd}.{t_s}, #{imm8}, lsl #{bad_amt}");
                let ops = vec![
                    arr(rd, t_s),
                    Operand::Imm(imm8 as i64),
                    shift_op("lsl", bad_amt),
                ];
                (ops, asm)
            }
            _ => {
                let mut imm64 = 0u64;
                imm64 |= (bad_byte as u64) << (byte_idx * 8);
                let asm = format!("movi v{rd}.2d, #0x{imm64:016x}");
                let ops = vec![arr(rd, "2d"), Operand::Imm(imm64 as i64)];
                (ops, asm)
            }
        };
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        prop_assert!(
            encode_neon_movi(&ops).is_err(),
            "documented error path must Err (llvm-mc rejects {})",
            asm
        );
    }
}

#[test]
fn test_encode_neon_movi_regression_h_lsl8() {
    let ops = vec![arr(0, "4h"), Operand::Imm(0), shift_op("lsl", 8)];
    let sut = sut_word(&ops).expect("valid movi v0.4h, #0, lsl #8");
    let mc = llvm_mc_word("movi v0.4h, #0, lsl #8").expect("llvm-mc");
    assert_eq!(
        sut, mc,
        "movi v0.4h, #0, lsl #8 must match llvm-mc (cmode=1010, 0x0f00a400)"
    );
    assert_eq!(mc, 0x0f00a400);
}

#[test]
fn test_encode_neon_movi_regression_s_msl() {
    let ops = vec![arr(0, "2s"), Operand::Imm(0), shift_op("msl", 8)];
    let sut = sut_word(&ops).expect("valid movi v0.2s, #0, msl #8");
    let mc = llvm_mc_word("movi v0.2s, #0, msl #8").expect("llvm-mc");
    assert_eq!(
        sut, mc,
        "movi v0.2s, #0, msl #8 must match llvm-mc (cmode=1100, 0x0f00c400)"
    );
    assert_eq!(mc, 0x0f00c400);
}

#[test]
fn test_encode_neon_movi_regression_extra_operand() {
    let ops = vec![arr(0, "8b"), Operand::Imm(0), arr(0, "8b")];
    assert!(
        encode_neon_movi(&ops).is_err(),
        "movi v0.8b, #0, v0.8b must Err (gas/llvm-mc reject a third non-shift operand)"
    );
}

#[test]
fn test_encode_neon_movi_regression_imm_oor() {
    let ops = vec![arr(0, "8b"), Operand::Imm(256)];
    assert!(
        encode_neon_movi(&ops).is_err(),
        "movi v0.8b, #256 must Err (llvm-mc: immediate must be in range [0, 255])"
    );
}
