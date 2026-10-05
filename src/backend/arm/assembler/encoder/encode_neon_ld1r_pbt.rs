// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md "accepts the same textual assembly that GCC's gas would consume";
//   README.md:235 ld1r/ld2r/ld3r/ld4r (with post-index);
//   encoder/mod.rs:1-7 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:732 ld1r => encode_neon_ld1r(operands);
//   ARM AdvSIMD load/store single structure (replicate): 0 Q 001101 L R=1 S=0 Rm opcode=110 size Rn Rt;
//   gas aarch64-linux-gnu-as agrees with llvm-mc on KAT vectors.
// Stronger considered:
//   - State machine: rejected — encode_neon_ld1r is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree LD1R decoder
//   - Differential vs encode_neon_ldnr: rejected — same-job gate (LD2R/LD3R/LD4R different mnemonic/dispatch)
//   - Differential vs encode_neon_ld_st_single / encode_neon_ld_st_multi: rejected — different ARM class
// Weaker available: algebraic.metamorphic (Rt/Rn/Q), algebraic.invariant (ARM fields), negative_error
// Differential: candidate=encode_neon_ld1r, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[RegList({Vt.T}), Mem{Xn|SP}|MemPostIndex] <-> `ld1r {Vt.T}, [Xn|SP{], #imm}]`

use super::encode_neon_ld1r;
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

fn xreg(n: u32) -> String {
    if n == 31 {
        "sp".to_string()
    } else {
        format!("x{}", n)
    }
}

fn valid_t() -> impl Strategy<Value = &'static str> {
    prop::sample::select(vec!["8b", "16b", "4h", "8h", "2s", "4s", "1d", "2d"])
}

fn reg_num() -> impl Strategy<Value = u32> {
    prop_oneof![Just(0u32), Just(1u32), Just(30u32), Just(31u32), 0u32..=31]
}

fn q_size(t: &str) -> (u32, u32) {
    match t {
        "8b" => (0, 0b00),
        "16b" => (1, 0b00),
        "4h" => (0, 0b01),
        "8h" => (1, 0b01),
        "2s" => (0, 0b10),
        "4s" => (1, 0b10),
        "1d" => (0, 0b11),
        "2d" => (1, 0b11),
        _ => (0, 0),
    }
}

fn esize_bytes(t: &str) -> i64 {
    match t {
        "8b" | "16b" => 1,
        "4h" | "8h" => 2,
        "2s" | "4s" => 4,
        "1d" | "2d" => 8,
        _ => 1,
    }
}

fn wide_pair(t: &str) -> Option<(&'static str, &'static str)> {
    match t {
        "8b" => Some(("8b", "16b")),
        "4h" => Some(("4h", "8h")),
        "2s" => Some(("2s", "4s")),
        "1d" => Some(("1d", "2d")),
        _ => None,
    }
}

fn neon_arr(reg: u32, arr: &str) -> Operand {
    Operand::RegArrangement {
        reg: vreg(reg),
        arrangement: arr.to_string(),
    }
}

fn reg_list(rt: u32, t: &str) -> Operand {
    Operand::RegList(vec![neon_arr(rt, t)])
}

fn list_asm(rt: u32, t: &str) -> String {
    format!("{{{}.{}}}", vreg(rt), t)
}

fn mem0(rn: u32) -> Operand {
    Operand::Mem {
        base: xreg(rn),
        offset: 0,
    }
}

fn mem_post(rn: u32, imm: i64) -> Operand {
    Operand::MemPostIndex {
        base: xreg(rn),
        offset: imm,
    }
}

fn sut_word(ops: &[Operand]) -> Result<u32, String> {
    match encode_neon_ld1r(ops)? {
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

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_neon_ld1r_kat_llvm_mc() {
    let want = 0x0d40c020u32;
    let mc = llvm_mc_word("ld1r {v0.8b}, [x1]").expect("llvm-mc KAT ld1r");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken for ld1r");
    let sut = sut_word(&[reg_list(0, "8b"), mem0(1)]).expect("SUT KAT ld1r");
    assert_eq!(sut, want);

    let want16 = 0x4d40c020u32;
    let mc16 = llvm_mc_word("ld1r {v0.16b}, [x1]").expect("llvm-mc KAT 16b");
    assert_eq!(mc16, want16, "llvm-mc KAT mapping broken for ld1r 16b");
    let sut16 = sut_word(&[reg_list(0, "16b"), mem0(1)]).expect("SUT KAT 16b");
    assert_eq!(sut16, want16);

    let want_h = 0x0d40c420u32;
    let mc_h = llvm_mc_word("ld1r {v0.4h}, [x1]").expect("llvm-mc KAT 4h");
    assert_eq!(mc_h, want_h);

    let want_d = 0x4d40cc20u32;
    let mc_d = llvm_mc_word("ld1r {v0.2d}, [x1]").expect("llvm-mc KAT 2d");
    assert_eq!(mc_d, want_d);

    let want_post = 0x0ddfc020u32;
    let mc_post = llvm_mc_word("ld1r {v0.8b}, [x1], #1").expect("llvm-mc KAT post");
    assert_eq!(mc_post, want_post, "llvm-mc KAT mapping broken for post-index");
    let sut_post = sut_word(&[reg_list(0, "8b"), mem_post(1, 1)]).expect("SUT KAT post");
    assert_eq!(sut_post, want_post);

    let want_sp = 0x0d40c3e0u32;
    let mc_sp = llvm_mc_word("ld1r {v0.8b}, [sp]").expect("llvm-mc KAT sp");
    assert_eq!(mc_sp, want_sp);
    let sut_sp = sut_word(&[reg_list(0, "8b"), mem0(31)]).expect("SUT KAT sp");
    assert_eq!(sut_sp, want_sp);

    let want_wrap = 0x4d40cfdfu32;
    let mc_wrap = llvm_mc_word("ld1r {v31.2d}, [x30]").expect("llvm-mc KAT v31");
    assert_eq!(mc_wrap, want_wrap);

    let want_regpost = 0x0dc2c020u32;
    let mc_rp = llvm_mc_word("ld1r {v0.8b}, [x1], x2").expect("llvm-mc KAT reg post");
    assert_eq!(mc_rp, want_regpost, "llvm-mc KAT mapping broken for register post-index");
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_ld1r_diff_no_offset_llvm_mc(
        t in valid_t(),
        rt in reg_num(),
        rn in reg_num(),
    ) {
        let asm = format!("ld1r {}, [{}]", list_asm(rt, t), xreg(rn));
        let ops = [reg_list(rt, t), mem0(rn)];
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&ops)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for {}", asm);
    }

    #[test]
    fn encode_neon_ld1r_diff_post_imm_llvm_mc(
        t in valid_t(),
        rt in reg_num(),
        rn in reg_num(),
    ) {
        let imm = esize_bytes(t);
        let asm = format!("ld1r {}, [{}], #{}", list_asm(rt, t), xreg(rn), imm);
        let ops = [reg_list(rt, t), mem_post(rn, imm)];
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&ops)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for {}", asm);
    }

    #[test]
    fn encode_neon_ld1r_arm_fields(
        t in valid_t(),
        rt in reg_num(),
        rn in reg_num(),
        post in any::<bool>(),
    ) {
        let (ops, l, rm) = if post {
            ( [reg_list(rt, t), mem_post(rn, esize_bytes(t))], 1u32, 0b11111u32 )
        } else {
            ( [reg_list(rt, t), mem0(rn)], 0u32, 0u32 )
        };
        let w = sut_word(&ops)
            .unwrap_or_else(|e| panic!("SUT rejected valid ld1r: {}", e));
        let (q, size) = q_size(t);
        prop_assert_eq!(w >> 31, 0, "bit31 must be 0");
        prop_assert_eq!((w >> 30) & 1, q, "Q bit");
        prop_assert_eq!((w >> 24) & 0b111111, 0b001101, "bits[29:24]=001101");
        prop_assert_eq!((w >> 23) & 1, l, "L bit");
        prop_assert_eq!((w >> 22) & 1, 1, "replicate bit22=1");
        prop_assert_eq!((w >> 21) & 1, 0, "S=0 for LD1R");
        prop_assert_eq!((w >> 16) & 0b11111, rm, "Rm");
        prop_assert_eq!((w >> 13) & 0b111, 0b110, "opcode=110");
        prop_assert_eq!((w >> 12) & 1, 0, "bit12=0 for replicate");
        prop_assert_eq!((w >> 10) & 0b11, size, "size");
        prop_assert_eq!((w >> 5) & 0b11111, rn, "Rn");
        prop_assert_eq!(w & 0b11111, rt, "Rt");
    }

    #[test]
    fn encode_neon_ld1r_metamorphic_rt_rn_q(
        t in prop::sample::select(vec!["8b", "4h", "2s", "1d"]),
        rt in 0u32..=30,
        rn in 0u32..=30,
    ) {
        let w = sut_word(&[reg_list(rt, t), mem0(rn)])
            .unwrap_or_else(|e| panic!("SUT rejected: {}", e));
        let w_rt = sut_word(&[reg_list(rt + 1, t), mem0(rn)])
            .unwrap_or_else(|e| panic!("SUT rejected rt+1: {}", e));
        let w_rn = sut_word(&[reg_list(rt, t), mem0(rn + 1)])
            .unwrap_or_else(|e| panic!("SUT rejected rn+1: {}", e));
        prop_assert_eq!(w_rt & 0x1f, rt + 1, "Rt+1 must increment bits[4:0]");
        prop_assert_eq!(w_rt & !0x1f, w & !0x1f, "Rt+1 must not change other fields");
        prop_assert_eq!((w_rn >> 5) & 0x1f, rn + 1, "Rn+1 must increment bits[9:5]");
        prop_assert_eq!(w_rn & !(0x1f << 5), w & !(0x1f << 5), "Rn+1 must not change other fields");
        if let Some((narrow, wide)) = wide_pair(t) {
            let w_n = sut_word(&[reg_list(rt, narrow), mem0(rn)])
                .unwrap_or_else(|e| panic!("SUT rejected narrow: {}", e));
            let w_w = sut_word(&[reg_list(rt, wide), mem0(rn)])
                .unwrap_or_else(|e| panic!("SUT rejected wide: {}", e));
            prop_assert_eq!(w_w ^ w_n, 1u32 << 30, "narrow vs wide must flip only Q");
        }
    }

    #[test]
    fn encode_neon_ld1r_neg_arity_kinds(
        kind in 0u32..=5,
        rt in reg_num(),
        rn in 0u32..=30,
    ) {
        let ops: Vec<Operand> = match kind {
            0 => vec![],
            1 => vec![reg_list(rt, "8b")],
            2 => vec![Operand::Reg(vreg(rt)), mem0(rn)],
            3 => vec![neon_arr(rt, "8b"), mem0(rn)],
            4 => vec![mem0(rn), reg_list(rt, "8b")],
            _ => vec![reg_list(rt, "8b"), Operand::Imm(0)],
        };
        prop_assert!(
            encode_neon_ld1r(&ops).is_err(),
            "ld1r kind={} must Err",
            kind
        );
    }

    #[test]
    fn encode_neon_ld1r_neg_count_arr(
        count in 2u32..=5,
        t_idx in 0u32..=7,
        rt in reg_num(),
        rn in 0u32..=30,
    ) {
        let bad_t = ["3s", "8s", "1s", "b", "h", "2h", "", "32b"][t_idx as usize];
        let ops_arr = [reg_list(rt, bad_t), mem0(rn)];
        prop_assert!(
            encode_neon_ld1r(&ops_arr).is_err(),
            "ld1r unsupported T={} must Err",
            bad_t
        );
        let regs: Vec<Operand> = (0..count).map(|i| neon_arr((rt + i) % 32, "8b")).collect();
        let ops_cnt = [Operand::RegList(regs), mem0(rn)];
        prop_assert!(
            encode_neon_ld1r(&ops_cnt).is_err(),
            "ld1r with {} regs must Err",
            count
        );
    }

    #[test]
    fn encode_neon_ld1r_neg_extra(
        t in valid_t(),
        rt in reg_num(),
        rn in 0u32..=30,
        extra_kind in 0u32..=3,
    ) {
        let extra = match extra_kind {
            0 => Operand::Cond("eq".into()),
            1 => Operand::Shift { kind: "lsl".into(), amount: 0 },
            2 => neon_arr(0, "8b"),
            _ => Operand::Label("1f".into()),
        };
        let ops = vec![reg_list(rt, t), mem0(rn), extra];
        prop_assert!(
            encode_neon_ld1r(&ops).is_err(),
            "ld1r extra operand must Err (llvm-mc/gas reject a surplus operand)"
        );
    }

    #[test]
    fn encode_neon_ld1r_neg_invalid_base(
        t in valid_t(),
        rt in reg_num(),
        base in prop::sample::select(vec![
            "w0", "wzr", "wsp", "xzr", "x31", "d0", "s0", "v0", "q0",
        ]),
    ) {
        let ops = [
            reg_list(rt, t),
            Operand::Mem {
                base: base.to_string(),
                offset: 0,
            },
        ];
        prop_assert!(
            encode_neon_ld1r(&ops).is_err(),
            "ld1r base [{}] must Err (llvm-mc requires Xn|SP)",
            base
        );
    }

    #[test]
    fn encode_neon_ld1r_diff_post_reg_llvm_mc(
        t in valid_t(),
        rt in reg_num(),
        rn in 0u32..=30,
        rm in 0u32..=30,
    ) {
        let asm = format!(
            "ld1r {}, [{}], x{}",
            list_asm(rt, t),
            xreg(rn),
            rm
        );
        let ops = [
            reg_list(rt, t),
            mem0(rn),
            Operand::Reg(format!("x{}", rm)),
        ];
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&ops)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for {}", asm);
    }

    #[test]
    fn encode_neon_ld1r_neg_bad_post_imm(
        t in valid_t(),
        rt in reg_num(),
        rn in 0u32..=30,
        imm in prop::sample::select(vec![-1i64, 0, 3, 5, 7, 64, 256]),
    ) {
        let legal = esize_bytes(t);
        prop_assume!(imm != legal);
        let ops = [reg_list(rt, t), mem_post(rn, imm)];
        prop_assert!(
            encode_neon_ld1r(&ops).is_err(),
            "ld1r post-index #{} (legal #{}) must Err",
            imm,
            legal
        );
    }

    #[test]
    fn encode_neon_ld1r_diff_alt_spellings(
        t in valid_t(),
        rt in 0u32..=30,
        rn in 0u32..=30,
    ) {
        let t_up = t.to_uppercase();
        let asm = format!("ld1r {{V{}.{}}}, [X{}]", rt, t_up, rn);
        let ops = [
            Operand::RegList(vec![Operand::RegArrangement {
                reg: format!("V{}", rt),
                arrangement: t.to_string(),
            }]),
            Operand::Mem {
                base: format!("X{}", rn),
                offset: 0,
            },
        ];
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&ops)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for {}", asm);
    }

    #[test]
    fn encode_neon_ld1r_neg_invalid_name(
        t in valid_t(),
        name in prop::sample::select(vec!["foo", "x32", "v32", "r0", "x", "v", ""]),
        slot in 0u32..=1,
    ) {
        let mut ops = vec![reg_list(0, t), mem0(1)];
        if slot == 0 {
            ops[0] = Operand::RegList(vec![Operand::RegArrangement {
                reg: name.to_string(),
                arrangement: t.to_string(),
            }]);
        } else {
            ops[1] = Operand::Mem {
                base: name.to_string(),
                offset: 0,
            };
        }
        prop_assert!(
            encode_neon_ld1r(&ops).is_err(),
            "ld1r invalid name '{}' slot {} must Err",
            name,
            slot
        );
    }

    #[test]
    fn encode_neon_ld1r_neg_mem_offset(
        t in valid_t(),
        rt in reg_num(),
        rn in 0u32..=30,
        off in prop::sample::select(vec![-1i64, 1, 4, 8, 16]),
    ) {
        let ops = [
            reg_list(rt, t),
            Operand::Mem {
                base: xreg(rn),
                offset: off,
            },
        ];
        prop_assert!(
            encode_neon_ld1r(&ops).is_err(),
            "ld1r [x{}, #{}] must Err (only [Xn] or post-index)",
            rn,
            off
        );
    }
}

/// Deterministic regression: extra surplus operand ignored.
#[test]
fn test_encode_neon_ld1r_regression_extra_operand() {
    let ops = [
        reg_list(0, "8b"),
        mem0(0),
        Operand::Cond("eq".into()),
    ];
    assert!(
        encode_neon_ld1r(&ops).is_err(),
        "ld1r {{v0.8b}}, [x0], eq must Err"
    );
}

/// Deterministic regression: W register as base accepted.
#[test]
fn test_encode_neon_ld1r_regression_w_base() {
    let ops = [
        reg_list(0, "8b"),
        Operand::Mem {
            base: "w0".into(),
            offset: 0,
        },
    ];
    assert!(
        encode_neon_ld1r(&ops).is_err(),
        "ld1r {{v0.8b}}, [w0] must Err"
    );
}

/// Deterministic regression: XZR as base accepted (encodes as SP).
#[test]
fn test_encode_neon_ld1r_regression_xzr_base() {
    let ops = [
        reg_list(0, "8b"),
        Operand::Mem {
            base: "xzr".into(),
            offset: 0,
        },
    ];
    assert!(
        encode_neon_ld1r(&ops).is_err(),
        "ld1r {{v0.8b}}, [xzr] must Err"
    );
}

/// Deterministic regression: FP register as base accepted.
#[test]
fn test_encode_neon_ld1r_regression_fp_base() {
    let ops = [
        reg_list(0, "8b"),
        Operand::Mem {
            base: "d0".into(),
            offset: 0,
        },
    ];
    assert!(
        encode_neon_ld1r(&ops).is_err(),
        "ld1r {{v0.8b}}, [d0] must Err"
    );
}

/// Deterministic regression: register post-index encodes as no-offset.
#[test]
fn test_encode_neon_ld1r_regression_reg_post() {
    let want = 0x0dc0c000u32;
    let ops = [
        reg_list(0, "8b"),
        mem0(0),
        Operand::Reg("x0".into()),
    ];
    let sut = sut_word(&ops).expect("SUT ld1r reg post");
    assert_eq!(
        sut, want,
        "ld1r {{v0.8b}}, [x0], x0 must be 0x0dc0c000"
    );
}

/// Deterministic regression: illegal post-index immediate accepted.
#[test]
fn test_encode_neon_ld1r_regression_bad_post_imm() {
    let ops = [reg_list(0, "8b"), mem_post(0, -1)];
    assert!(
        encode_neon_ld1r(&ops).is_err(),
        "ld1r {{v0.8b}}, [x0], #-1 must Err"
    );
}
