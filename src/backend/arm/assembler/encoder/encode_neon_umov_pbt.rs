// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md "accepts the same textual assembly that GCC's gas would consume";
//   README.md:234 NEON insert/move lists umov;
//   encoder/mod.rs:1-7 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:679 "umov" => encode_neon_umov;
//   neon.rs:460 Encode NEON UMOV: move element to GP register;
//   neon.rs:482 UMOV Rd, Vn.Ts[index]: 0 Q 0 01110 000 imm5 0 0111 1 Rn Rd;
//   ARM ARM Advanced SIMD copy (UMOV). Ts in {B,H,S} with Wd and Q=0; Ts=D with Xd and Q=1;
//   index b[0-15] h[0-7] s[0-3] d[0-1].
// Stronger considered:
//   - State machine: rejected — encode_neon_umov is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree UMOV decoder (INS/DUP are different opcodes)
//   - Differential vs encode_neon_dup / encode_neon_ins / encode_mov: rejected — same-job gate
//     (DUP 000011/000001, INS 000111, MOV is a multi-form alias encoder vs UMOV 001111)
// Weaker available: algebraic.metamorphic (Rd/Rn fields), algebraic.invariant (word layout),
//   negative_error (arity / extra / index / dest width / SP / FP-as-GPR)
// Differential: candidate=encode_neon_umov, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[Reg(Wd|Xd), RegLane(Vn,Ts,i)] <-> `umov Wd|Xd, Vn.Ts[i]`

use super::encode_neon_umov;
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

fn vreg(n: u32) -> String {
    format!("v{}", n)
}

fn imax(ts: &str) -> u32 {
    match ts {
        "b" => 15,
        "h" => 7,
        "s" => 3,
        "d" => 1,
        _ => 0,
    }
}

fn gpr_name(ts: &str, n: u32) -> String {
    if ts == "d" {
        if n == 31 {
            "xzr".to_string()
        } else {
            format!("x{}", n)
        }
    } else if n == 31 {
        "wzr".to_string()
    } else {
        format!("w{}", n)
    }
}

fn wrong_gpr_name(ts: &str, n: u32) -> String {
    if ts == "d" {
        format!("w{}", n)
    } else {
        format!("x{}", n)
    }
}

fn lane(reg: u32, ts: &str, index: u32) -> Operand {
    Operand::RegLane {
        reg: vreg(reg),
        elem_size: ts.to_string(),
        index,
    }
}

fn umov_ops(rd: u32, ts: &str, i: u32, rn: u32) -> Vec<Operand> {
    vec![Operand::Reg(gpr_name(ts, rd)), lane(rn, ts, i)]
}

fn umov_asm(rd: u32, ts: &str, i: u32, rn: u32) -> String {
    format!("umov {}, v{}.{}[{}]", gpr_name(ts, rd), rn, ts, i)
}

fn sut_word(ops: &[Operand]) -> Result<u32, String> {
    match encode_neon_umov(ops)? {
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

fn ts_size() -> impl Strategy<Value = &'static str> {
    prop::sample::select(vec!["b", "h", "s", "d"])
}

fn ts_and_idx() -> impl Strategy<Value = (&'static str, u32)> {
    ts_size().prop_flat_map(|ts| {
        let m = imax(ts);
        (Just(ts), prop_oneof![Just(0u32), Just(m), 0u32..=m])
    })
}

fn imm5(ts: &str, i: u32) -> u32 {
    match ts {
        "b" => ((i & 0xF) << 1) | 0b00001,
        "h" => ((i & 0x7) << 2) | 0b00010,
        "s" => ((i & 0x3) << 3) | 0b00100,
        "d" => ((i & 0x1) << 4) | 0b01000,
        _ => 0,
    }
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_neon_umov_kat_llvm_mc() {
    let want = 0x0e013c00u32;
    let mc = llvm_mc_word("umov w0, v0.b[0]").expect("llvm-mc KAT umov b");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken for umov w0, v0.b[0]");
    assert_eq!(
        sut_word(&umov_ops(0, "b", 0, 0)).expect("SUT KAT umov b"),
        want
    );

    let want_h = 0x0e023c00u32;
    let mc_h = llvm_mc_word("umov w0, v0.h[0]").expect("llvm-mc KAT umov h");
    assert_eq!(mc_h, want_h);
    assert_eq!(sut_word(&umov_ops(0, "h", 0, 0)).expect("SUT KAT h"), want_h);

    let want_s = 0x0e043c00u32;
    let mc_s = llvm_mc_word("umov w0, v0.s[0]").expect("llvm-mc KAT umov s");
    assert_eq!(mc_s, want_s);
    assert_eq!(sut_word(&umov_ops(0, "s", 0, 0)).expect("SUT KAT s"), want_s);

    let want_d = 0x4e083c00u32;
    let mc_d = llvm_mc_word("umov x0, v0.d[0]").expect("llvm-mc KAT umov d");
    assert_eq!(mc_d, want_d);
    assert_eq!(sut_word(&umov_ops(0, "d", 0, 0)).expect("SUT KAT d"), want_d);

    let want_b15 = 0x0e1f3c00u32;
    let mc_b15 = llvm_mc_word("umov w0, v0.b[15]").expect("llvm-mc KAT umov b[15]");
    assert_eq!(mc_b15, want_b15);
    assert_eq!(
        sut_word(&umov_ops(0, "b", 15, 0)).expect("SUT KAT b[15]"),
        want_b15
    );

    let want_d1 = 0x4e183fffu32;
    let mc_d1 = llvm_mc_word("umov x31, v31.d[1]").expect("llvm-mc KAT umov x31, v31.d[1]");
    assert_eq!(mc_d1, want_d1);
    assert_eq!(
        sut_word(&umov_ops(31, "d", 1, 31)).expect("SUT KAT xzr v31.d[1]"),
        want_d1
    );

    let want_wzr = 0x0e013c1fu32;
    let mc_wzr = llvm_mc_word("umov wzr, v0.b[0]").expect("llvm-mc KAT wzr");
    assert_eq!(mc_wzr, want_wzr);
    assert_eq!(
        sut_word(&umov_ops(31, "b", 0, 0)).expect("SUT KAT wzr"),
        want_wzr
    );

    let want_h7 = 0x0e1e3c41u32;
    let mc_h7 = llvm_mc_word("umov w1, v2.h[7]").expect("llvm-mc KAT h[7]");
    assert_eq!(mc_h7, want_h7);
    assert_eq!(
        sut_word(&umov_ops(1, "h", 7, 2)).expect("SUT KAT h[7]"),
        want_h7
    );

    let want_s3 = 0x0e1c3c83u32;
    let mc_s3 = llvm_mc_word("umov w3, v4.s[3]").expect("llvm-mc KAT s[3]");
    assert_eq!(mc_s3, want_s3);
    assert_eq!(
        sut_word(&umov_ops(3, "s", 3, 4)).expect("SUT KAT s[3]"),
        want_s3
    );

    let want_xd1 = 0x4e183cc5u32;
    let mc_xd1 = llvm_mc_word("umov x5, v6.d[1]").expect("llvm-mc KAT x5 v6.d[1]");
    assert_eq!(mc_xd1, want_xd1);
    assert_eq!(
        sut_word(&umov_ops(5, "d", 1, 6)).expect("SUT KAT x5 v6.d[1]"),
        want_xd1
    );
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_umov_diff_llvm_mc(
        rd in reg_num(),
        rn in reg_num(),
        (ts, i) in ts_and_idx(),
    ) {
        let asm = umov_asm(rd, ts, i, rn);
        let ops = umov_ops(rd, ts, i, rn);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&ops)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for {}", asm);
    }

    #[test]
    fn encode_neon_umov_metamorphic_rd_rn(
        rd1 in reg_num(),
        rd2 in reg_num(),
        rn1 in reg_num(),
        rn2 in reg_num(),
        (ts, i) in ts_and_idx(),
    ) {
        let w11 = sut_word(&umov_ops(rd1, ts, i, rn1))
            .unwrap_or_else(|e| panic!("SUT rejected rd1/rn1: {}", e));
        let w21 = sut_word(&umov_ops(rd2, ts, i, rn1))
            .unwrap_or_else(|e| panic!("SUT rejected rd2/rn1: {}", e));
        let w12 = sut_word(&umov_ops(rd1, ts, i, rn2))
            .unwrap_or_else(|e| panic!("SUT rejected rd1/rn2: {}", e));
        prop_assert_eq!(
            (w11 ^ w21) & !0x1Fu32,
            0u32,
            "changing only Rd must differ only in bits[4:0]"
        );
        prop_assert_eq!(w21 & 0x1F, rd2, "Rd field");
        prop_assert_eq!(
            (w11 ^ w12) & !(0x1Fu32 << 5),
            0u32,
            "changing only Rn must differ only in bits[9:5]"
        );
        prop_assert_eq!((w12 >> 5) & 0x1F, rn2, "Rn field");
    }

    #[test]
    fn encode_neon_umov_invariant_arm_fields(
        rd in reg_num(),
        rn in reg_num(),
        (ts, i) in ts_and_idx(),
    ) {
        let w = sut_word(&umov_ops(rd, ts, i, rn))
            .unwrap_or_else(|e| panic!("SUT rejected: {}", e));
        let q = if ts == "d" { 1u32 } else { 0 };
        prop_assert_eq!((w >> 31) & 1, 0u32, "bit 31 must be 0");
        prop_assert_eq!((w >> 30) & 1, q, "Q/bit30 = 1 iff Ts=D");
        prop_assert_eq!((w >> 29) & 1, 0u32, "bit 29 = 0");
        prop_assert_eq!((w >> 21) & 0xFF, 0b01110000u32, "bits[28:21]=01110000");
        prop_assert_eq!((w >> 16) & 0x1F, imm5(ts, i), "imm5");
        prop_assert_eq!((w >> 10) & 0x3F, 0b001111u32, "bits[15:10]=001111");
        prop_assert_eq!((w >> 5) & 0x1F, rn, "Rn");
        prop_assert_eq!(w & 0x1F, rd, "Rd");
    }

    #[test]
    fn encode_neon_umov_neg_extra_operand(
        rd in reg_num(),
        rn in reg_num(),
        extra in reg_num(),
        (ts, i) in ts_and_idx(),
    ) {
        let asm = format!("{}, {}", umov_asm(rd, ts, i, rn), gpr_name(ts, extra));
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted 3-operand {}",
            asm
        );
        let mut ops = umov_ops(rd, ts, i, rn);
        ops.push(Operand::Reg(gpr_name(ts, extra)));
        prop_assert!(
            encode_neon_umov(&ops).is_err(),
            "3 operands must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_umov_neg_index_oor(
        rd in reg_num(),
        rn in reg_num(),
        ts in ts_size(),
        over in 1u32..=8,
    ) {
        let i = imax(ts) + over;
        let asm = umov_asm(rd, ts, i, rn);
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted OOR {}",
            asm
        );
        prop_assert!(
            encode_neon_umov(&umov_ops(rd, ts, i, rn)).is_err(),
            "index {} for .{} must Err (llvm-mc range [0, {}]; asm {})",
            i,
            ts,
            imax(ts),
            asm
        );
    }

    #[test]
    fn encode_neon_umov_neg_wrong_width(
        rd in 0u32..=30,
        rn in reg_num(),
        (ts, i) in ts_and_idx(),
    ) {
        let bad = wrong_gpr_name(ts, rd);
        let asm = format!("umov {}, v{}.{}[{}]", bad, rn, ts, i);
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted wrong-width {}",
            asm
        );
        let ops = vec![Operand::Reg(bad.clone()), lane(rn, ts, i)];
        prop_assert!(
            encode_neon_umov(&ops).is_err(),
            "wrong GPR width must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_umov_neg_arity_sp_fp(
        n in 0usize..=1,
        rd in reg_num(),
        rn in reg_num(),
        (ts, i) in ts_and_idx(),
        kind in 0u8..=4,
        fp_prefix in prop::sample::select(vec!["d", "s", "q", "v", "h", "b"]),
        bad in prop_oneof![
            Just("x32".to_string()),
            Just("foo".to_string()),
            Just("".to_string()),
            Just("w".to_string()),
        ],
    ) {
        let all = umov_ops(rd, ts, i, rn);
        let arity_ops: Vec<Operand> = all.iter().take(n).cloned().collect();
        let arity_asm = if n == 0 {
            "umov".to_string()
        } else {
            format!("umov {}", gpr_name(ts, rd))
        };
        prop_assert!(
            llvm_mc_word(&arity_asm).is_err(),
            "llvm-mc unexpectedly accepted arity-{} {}",
            n,
            arity_asm
        );
        prop_assert!(
            encode_neon_umov(&arity_ops).is_err(),
            "arity {} must Err (llvm-mc rejects {})",
            n,
            arity_asm
        );

        let (ops, asm) = match kind {
            0 => (
                vec![Operand::Reg("sp".into()), lane(rn, ts, i)],
                format!("umov sp, v{}.{}[{}]", rn, ts, i),
            ),
            1 => (
                vec![Operand::Reg("wsp".into()), lane(rn, ts, i)],
                format!("umov wsp, v{}.{}[{}]", rn, ts, i),
            ),
            2 => {
                let dest = format!("{}{}", fp_prefix, rd);
                (
                    vec![Operand::Reg(dest.clone()), lane(rn, ts, i)],
                    format!("umov {}, v{}.{}[{}]", dest, rn, ts, i),
                )
            }
            3 => {
                let mut bad_ops = umov_ops(rd, ts, i, rn);
                bad_ops[0] = Operand::Reg(bad.clone());
                (
                    bad_ops,
                    format!("umov {}, v{}.{}[{}]", bad, rn, ts, i),
                )
            }
            _ => {
                let mut bad_ops = umov_ops(rd, ts, i, rn);
                bad_ops[1] = Operand::RegLane {
                    reg: bad.clone(),
                    elem_size: ts.to_string(),
                    index: i,
                };
                (
                    bad_ops,
                    format!("umov {}, {}.{}[{}]", gpr_name(ts, rd), bad, ts, i),
                )
            }
        };
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        prop_assert!(
            encode_neon_umov(&ops).is_err(),
            "invalid kind={} must Err (llvm-mc rejects {})",
            kind,
            asm
        );
    }

    #[test]
    fn encode_neon_umov_diff_alt_spellings(
        rd in 0u32..=30,
        rn in reg_num(),
        (ts, i) in ts_and_idx(),
    ) {
        let gpr = if ts == "d" {
            format!("X{}", rd)
        } else {
            format!("W{}", rd)
        };
        let asm = format!("umov {}, V{}.{}[{}]", gpr, rn, ts, i);
        let ops = vec![
            Operand::Reg(gpr),
            Operand::RegLane {
                reg: format!("V{}", rn),
                elem_size: ts.to_string(),
                index: i,
            },
        ];
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected alt-spelling {}: {}", asm, e));
        let sut = sut_word(&ops)
            .unwrap_or_else(|e| panic!("SUT rejected alt-spelling {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for alt-spelling {}", asm);
    }

    #[test]
    fn encode_neon_umov_neg_unsupported_elem_size(
        rd in reg_num(),
        rn in reg_num(),
        bad_ts in prop::sample::select(vec!["q", "8b", "16b", "4h", "", "x"]),
        i in 0u32..=15,
    ) {
        let asm = format!("umov {}, v{}.{}[{}]", gpr_name("b", rd), rn, bad_ts, i);
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted unsupported Ts {}",
            asm
        );
        let ops = vec![
            Operand::Reg(gpr_name("b", rd)),
            Operand::RegLane {
                reg: vreg(rn),
                elem_size: bad_ts.to_string(),
                index: i,
            },
        ];
        prop_assert!(
            encode_neon_umov(&ops).is_err(),
            "unsupported elem_size {} must Err (llvm-mc rejects {})",
            bad_ts,
            asm
        );
    }

    #[test]
    fn encode_neon_umov_neg_non_lane_src(
        rd in reg_num(),
        rn in reg_num(),
        (ts, i) in ts_and_idx(),
        kind in 0u8..=2,
    ) {
        let dest = Operand::Reg(gpr_name(ts, rd));
        let (ops, asm) = match kind {
            0 => (
                vec![dest, Operand::Reg(format!("v{}", rn))],
                format!("umov {}, v{}", gpr_name(ts, rd), rn),
            ),
            1 => (
                vec![
                    dest,
                    Operand::RegArrangement {
                        reg: vreg(rn),
                        arrangement: "8b".into(),
                    },
                ],
                format!("umov {}, v{}.8b", gpr_name(ts, rd), rn),
            ),
            _ => (
                vec![dest, Operand::Imm(i as i64)],
                format!("umov {}, #{}", gpr_name(ts, rd), i),
            ),
        };
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted non-lane {}",
            asm
        );
        prop_assert!(
            encode_neon_umov(&ops).is_err(),
            "non-RegLane src kind={} must Err (llvm-mc rejects {})",
            kind,
            asm
        );
    }
}

/// Deterministic regression: extra operand ignored (from neg_extra_operand).
#[test]
fn test_encode_neon_umov_regression_extra_operand() {
    let mut ops = umov_ops(0, "b", 0, 0);
    ops.push(Operand::Reg("w0".into()));
    assert!(
        encode_neon_umov(&ops).is_err(),
        "umov w0, v0.b[0], w0 must Err (llvm-mc rejects a third operand)"
    );
}

/// Deterministic regression: out-of-range lane masked (from neg_index_oor).
#[test]
fn test_encode_neon_umov_regression_index_oor() {
    assert!(
        encode_neon_umov(&umov_ops(0, "b", 16, 0)).is_err(),
        "umov w0, v0.b[16] must Err (llvm-mc range for .b is [0, 15])"
    );
    assert!(
        encode_neon_umov(&umov_ops(0, "h", 8, 0)).is_err(),
        "umov w0, v0.h[8] must Err (llvm-mc range for .h is [0, 7])"
    );
    assert!(
        encode_neon_umov(&umov_ops(0, "s", 4, 0)).is_err(),
        "umov w0, v0.s[4] must Err (llvm-mc range for .s is [0, 3])"
    );
    assert!(
        encode_neon_umov(&umov_ops(0, "d", 2, 0)).is_err(),
        "umov x0, v0.d[2] must Err (llvm-mc range for .d is [0, 1])"
    );
}

/// Deterministic regression: wrong dest GPR width accepted (from neg_wrong_width).
#[test]
fn test_encode_neon_umov_regression_wrong_width() {
    let ops_x_for_b = vec![Operand::Reg("x0".into()), lane(0, "b", 0)];
    assert!(
        encode_neon_umov(&ops_x_for_b).is_err(),
        "umov x0, v0.b[0] must Err (llvm-mc requires Wd for Ts=B)"
    );
    let ops_w_for_d = vec![Operand::Reg("w0".into()), lane(0, "d", 0)];
    assert!(
        encode_neon_umov(&ops_w_for_d).is_err(),
        "umov w0, v0.d[0] must Err (llvm-mc requires Xd for Ts=D)"
    );
}

/// Deterministic regression: SP encoded as XZR (from neg_arity_sp_fp).
#[test]
fn test_encode_neon_umov_regression_sp_as_zr() {
    let ops = vec![Operand::Reg("sp".into()), lane(0, "b", 0)];
    assert!(
        encode_neon_umov(&ops).is_err(),
        "umov sp, v0.b[0] must Err (llvm-mc rejects SP as UMOV dest)"
    );
}

/// Deterministic regression: WSP encoded as WZR (from neg_arity_sp_fp).
#[test]
fn test_encode_neon_umov_regression_wsp_as_zr() {
    let ops = vec![Operand::Reg("wsp".into()), lane(0, "b", 0)];
    assert!(
        encode_neon_umov(&ops).is_err(),
        "umov wsp, v0.b[0] must Err (llvm-mc rejects WSP as UMOV dest)"
    );
}

/// Deterministic regression: FP register as GPR dest (from neg_arity_sp_fp).
#[test]
fn test_encode_neon_umov_regression_fp_as_gpr() {
    let ops = vec![Operand::Reg("d0".into()), lane(0, "b", 0)];
    assert!(
        encode_neon_umov(&ops).is_err(),
        "umov d0, v0.b[0] must Err (llvm-mc rejects FP as UMOV dest)"
    );
}
