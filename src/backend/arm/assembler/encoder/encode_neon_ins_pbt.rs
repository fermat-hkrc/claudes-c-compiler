// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md "accepts the same textual assembly that GCC's gas would consume";
//   README.md:234 NEON insert/move lists ins (element/GPR);
//   encoder/mod.rs:1-7 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:679 "ins" => encode_neon_ins;
//   neon.rs:549 Encode NEON INS (insert element from GP register): INS Vd.Ts[index], Xn;
//   neon.rs:565 INS Vd.Ts[i], Xn: 0 1 0 01110 000 imm5 0 0011 1 Rn Rd;
//   neon.rs:595 INS Vd.Ts[dst], Vn.Ts[src]: 0 1 1 01110 000 imm5 0 imm4 1 Rn Rd;
//   ARM ARM Advanced SIMD INS (general) / INS (element). Ts in {B,H,S,D};
//   GPR source Wn for Ts in {B,H,S}, Xn for Ts=D; index b[0-15] h[0-7] s[0-3] d[0-1].
// Stronger considered:
//   - State machine: rejected — encode_neon_ins is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree INS decoder (UMOV is a different opcode)
//   - Differential vs encode_neon_dup / encode_neon_umov: rejected — same-job gate
//     (DUP 000011/000001, UMOV 001111 vs INS 000111 / element bit29=1)
// Weaker available: algebraic.metamorphic (Rd/Rn fields), algebraic.invariant (word layout),
//   negative_error (arity / extra / index / GPR width / size mismatch / SP / FP-as-GPR)
// Differential: candidate=encode_neon_ins, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[RegLane(Vd,Ts,i), Reg(Wn|Xn)] <-> `ins Vd.Ts[i], Wn|Xn`
//          and [RegLane(Vd,Ts,i), RegLane(Vn,Ts,j)] <-> `ins Vd.Ts[i], Vn.Ts[j]`

use super::encode_neon_ins;
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

fn gpr_ops(rd: u32, ts: &str, i: u32, rn: u32) -> Vec<Operand> {
    vec![lane(rd, ts, i), Operand::Reg(gpr_name(ts, rn))]
}

fn elem_ops(rd: u32, dts: &str, di: u32, rn: u32, sts: &str, si: u32) -> Vec<Operand> {
    vec![lane(rd, dts, di), lane(rn, sts, si)]
}

fn ins_gpr_asm(rd: u32, ts: &str, i: u32, rn: u32) -> String {
    format!("ins v{}.{}[{}], {}", rd, ts, i, gpr_name(ts, rn))
}

fn ins_elem_asm(rd: u32, ts: &str, di: u32, rn: u32, si: u32) -> String {
    format!("ins v{}.{}[{}], v{}.{}[{}]", rd, ts, di, rn, ts, si)
}

fn sut_word(ops: &[Operand]) -> Result<u32, String> {
    match encode_neon_ins(ops)? {
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

fn ts_and_two_idx() -> impl Strategy<Value = (&'static str, u32, u32)> {
    ts_size().prop_flat_map(|ts| {
        let m = imax(ts);
        let idx = prop_oneof![Just(0u32), Just(m), 0u32..=m];
        (Just(ts), idx.clone(), idx)
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

fn imm4(ts: &str, i: u32) -> u32 {
    match ts {
        "b" => i & 0xF,
        "h" => (i & 0x7) << 1,
        "s" => (i & 0x3) << 2,
        "d" => (i & 0x1) << 3,
        _ => 0,
    }
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_neon_ins_kat_llvm_mc() {
    let want = 0x4e011c20u32;
    let mc = llvm_mc_word("ins v0.b[0], w1").expect("llvm-mc KAT ins b gpr");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken for ins v0.b[0], w1");
    assert_eq!(
        sut_word(&gpr_ops(0, "b", 0, 1)).expect("SUT KAT ins b gpr"),
        want
    );

    let want_h = 0x4e021c20u32;
    let mc_h = llvm_mc_word("ins v0.h[0], w1").expect("llvm-mc KAT ins h gpr");
    assert_eq!(mc_h, want_h);
    assert_eq!(sut_word(&gpr_ops(0, "h", 0, 1)).expect("SUT KAT h"), want_h);

    let want_s = 0x4e041c20u32;
    let mc_s = llvm_mc_word("ins v0.s[0], w1").expect("llvm-mc KAT ins s gpr");
    assert_eq!(mc_s, want_s);
    assert_eq!(sut_word(&gpr_ops(0, "s", 0, 1)).expect("SUT KAT s"), want_s);

    let want_d = 0x4e081c20u32;
    let mc_d = llvm_mc_word("ins v0.d[0], x1").expect("llvm-mc KAT ins d gpr");
    assert_eq!(mc_d, want_d);
    assert_eq!(sut_word(&gpr_ops(0, "d", 0, 1)).expect("SUT KAT d"), want_d);

    let want_b15 = 0x4e1f1c20u32;
    let mc_b15 = llvm_mc_word("ins v0.b[15], w1").expect("llvm-mc KAT ins b[15]");
    assert_eq!(mc_b15, want_b15);
    assert_eq!(
        sut_word(&gpr_ops(0, "b", 15, 1)).expect("SUT KAT b[15]"),
        want_b15
    );

    let want_d1 = 0x4e181fdfu32;
    let mc_d1 = llvm_mc_word("ins v31.d[1], x30").expect("llvm-mc KAT ins v31.d[1], x30");
    assert_eq!(mc_d1, want_d1);
    assert_eq!(
        sut_word(&gpr_ops(31, "d", 1, 30)).expect("SUT KAT v31.d[1] x30"),
        want_d1
    );

    let want_wzr = 0x4e011fe0u32;
    let mc_wzr = llvm_mc_word("ins v0.b[0], wzr").expect("llvm-mc KAT wzr");
    assert_eq!(mc_wzr, want_wzr);
    assert_eq!(
        sut_word(&gpr_ops(0, "b", 0, 31)).expect("SUT KAT wzr"),
        want_wzr
    );

    let want_elem = 0x6e010420u32;
    let mc_elem = llvm_mc_word("ins v0.b[0], v1.b[0]").expect("llvm-mc KAT elem");
    assert_eq!(mc_elem, want_elem);
    assert_eq!(
        sut_word(&elem_ops(0, "b", 0, 1, "b", 0)).expect("SUT KAT elem"),
        want_elem
    );

    let want_elem_h = 0x6e0e1440u32;
    let mc_elem_h = llvm_mc_word("ins v0.h[3], v2.h[1]").expect("llvm-mc KAT elem h");
    assert_eq!(mc_elem_h, want_elem_h);
    assert_eq!(
        sut_word(&elem_ops(0, "h", 3, 2, "h", 1)).expect("SUT KAT elem h"),
        want_elem_h
    );

    let want_elem_d = 0x6e180480u32;
    let mc_elem_d = llvm_mc_word("ins v0.d[1], v4.d[0]").expect("llvm-mc KAT elem d");
    assert_eq!(mc_elem_d, want_elem_d);
    assert_eq!(
        sut_word(&elem_ops(0, "d", 1, 4, "d", 0)).expect("SUT KAT elem d"),
        want_elem_d
    );
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_ins_diff_llvm_mc_gpr(
        rd in reg_num(),
        rn in reg_num(),
        (ts, i) in ts_and_idx(),
    ) {
        let asm = ins_gpr_asm(rd, ts, i, rn);
        let ops = gpr_ops(rd, ts, i, rn);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&ops)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for {}", asm);
    }

    #[test]
    fn encode_neon_ins_diff_llvm_mc_elem(
        rd in reg_num(),
        rn in reg_num(),
        (ts, di, si) in ts_and_two_idx(),
    ) {
        let asm = ins_elem_asm(rd, ts, di, rn, si);
        let ops = elem_ops(rd, ts, di, rn, ts, si);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&ops)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for {}", asm);
    }

    #[test]
    fn encode_neon_ins_metamorphic_rd_rn(
        rd1 in reg_num(),
        rd2 in reg_num(),
        rn1 in reg_num(),
        rn2 in reg_num(),
        (ts, i) in ts_and_idx(),
    ) {
        let w11 = sut_word(&gpr_ops(rd1, ts, i, rn1))
            .unwrap_or_else(|e| panic!("SUT rejected rd1/rn1: {}", e));
        let w21 = sut_word(&gpr_ops(rd2, ts, i, rn1))
            .unwrap_or_else(|e| panic!("SUT rejected rd2/rn1: {}", e));
        let w12 = sut_word(&gpr_ops(rd1, ts, i, rn2))
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
    fn encode_neon_ins_invariant_arm_fields(
        rd in reg_num(),
        rn in reg_num(),
        (ts, i, si) in ts_and_two_idx(),
    ) {
        let wg = sut_word(&gpr_ops(rd, ts, i, rn))
            .unwrap_or_else(|e| panic!("SUT rejected GPR form: {}", e));
        prop_assert_eq!((wg >> 31) & 1, 0u32, "bit 31 must be 0");
        prop_assert_eq!((wg >> 30) & 1, 1u32, "Q/bit30 = 1 for INS general");
        prop_assert_eq!((wg >> 29) & 1, 0u32, "bit 29 = 0 for INS general");
        prop_assert_eq!((wg >> 21) & 0xFF, 0b01110000u32, "bits[28:21]=01110000");
        prop_assert_eq!((wg >> 16) & 0x1F, imm5(ts, i), "imm5");
        prop_assert_eq!((wg >> 10) & 0x3F, 0b000111u32, "bits[15:10]=000111");
        prop_assert_eq!((wg >> 5) & 0x1F, rn, "Rn");
        prop_assert_eq!(wg & 0x1F, rd, "Rd");

        let we = sut_word(&elem_ops(rd, ts, i, rn, ts, si))
            .unwrap_or_else(|e| panic!("SUT rejected elem form: {}", e));
        prop_assert_eq!((we >> 31) & 1, 0u32, "elem bit 31 must be 0");
        prop_assert_eq!((we >> 30) & 1, 1u32, "elem bit 30 = 1");
        prop_assert_eq!((we >> 29) & 1, 1u32, "bit 29 = 1 for INS element");
        prop_assert_eq!((we >> 21) & 0xFF, 0b01110000u32, "elem bits[28:21]");
        prop_assert_eq!((we >> 16) & 0x1F, imm5(ts, i), "elem imm5");
        prop_assert_eq!((we >> 15) & 1, 0u32, "bit 15 = 0");
        prop_assert_eq!((we >> 11) & 0xF, imm4(ts, si), "imm4");
        prop_assert_eq!((we >> 10) & 1, 1u32, "bit 10 = 1");
        prop_assert_eq!((we >> 5) & 0x1F, rn, "elem Rn");
        prop_assert_eq!(we & 0x1F, rd, "elem Rd");
    }

    #[test]
    fn encode_neon_ins_neg_extra_operand(
        rd in reg_num(),
        rn in reg_num(),
        extra in reg_num(),
        (ts, i) in ts_and_idx(),
    ) {
        let asm = format!("{}, {}", ins_gpr_asm(rd, ts, i, rn), gpr_name(ts, extra));
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted 3-operand {}",
            asm
        );
        let mut ops = gpr_ops(rd, ts, i, rn);
        ops.push(Operand::Reg(gpr_name(ts, extra)));
        prop_assert!(
            encode_neon_ins(&ops).is_err(),
            "3 operands must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_ins_neg_index_oor(
        rd in reg_num(),
        rn in reg_num(),
        ts in ts_size(),
        over in 1u32..=8,
    ) {
        let i = imax(ts) + over;
        let asm = ins_gpr_asm(rd, ts, i, rn);
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted OOR {}",
            asm
        );
        prop_assert!(
            encode_neon_ins(&gpr_ops(rd, ts, i, rn)).is_err(),
            "index {} for .{} must Err (llvm-mc range [0, {}]; asm {})",
            i,
            ts,
            imax(ts),
            asm
        );
        let asm_e = format!("ins v{}.{}[0], v{}.{}[{}]", rd, ts, rn, ts, i);
        prop_assert!(
            llvm_mc_word(&asm_e).is_err(),
            "llvm-mc unexpectedly accepted OOR src {}",
            asm_e
        );
        prop_assert!(
            encode_neon_ins(&elem_ops(rd, ts, 0, rn, ts, i)).is_err(),
            "src index {} for .{} must Err (asm {})",
            i,
            ts,
            asm_e
        );
    }

    #[test]
    fn encode_neon_ins_neg_wrong_width_gpr(
        rd in reg_num(),
        rn in 0u32..=30,
        (ts, i) in ts_and_idx(),
    ) {
        let bad = wrong_gpr_name(ts, rn);
        let asm = format!("ins v{}.{}[{}], {}", rd, ts, i, bad);
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted wrong-width {}",
            asm
        );
        let ops = vec![lane(rd, ts, i), Operand::Reg(bad.clone())];
        prop_assert!(
            encode_neon_ins(&ops).is_err(),
            "wrong GPR width must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_ins_neg_mismatch_sp_fp(
        rd in reg_num(),
        rn in reg_num(),
        (ts, i) in ts_and_idx(),
        (ts2, j) in ts_and_idx(),
        kind in 0u8..=3,
        fp_prefix in prop::sample::select(vec!["d", "s", "q", "v", "h", "b"]),
    ) {
        let (ops, asm) = match kind {
            0 => {
                prop_assume!(ts != ts2);
                (
                    elem_ops(rd, ts, i, rn, ts2, j),
                    format!("ins v{}.{}[{}], v{}.{}[{}]", rd, ts, i, rn, ts2, j),
                )
            }
            1 => {
                (
                    vec![lane(rd, ts, i), Operand::Reg("sp".into())],
                    format!("ins v{}.{}[{}], sp", rd, ts, i),
                )
            }
            2 => {
                (
                    vec![lane(rd, ts, i), Operand::Reg("wsp".into())],
                    format!("ins v{}.{}[{}], wsp", rd, ts, i),
                )
            }
            _ => {
                let src = format!("{}{}", fp_prefix, rn);
                (
                    vec![lane(rd, ts, i), Operand::Reg(src.clone())],
                    format!("ins v{}.{}[{}], {}", rd, ts, i, src),
                )
            }
        };
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        prop_assert!(
            encode_neon_ins(&ops).is_err(),
            "invalid kind={} must Err (llvm-mc rejects {})",
            kind,
            asm
        );
    }

    #[test]
    fn encode_neon_ins_neg_arity(
        n in 0usize..=1,
        rd in reg_num(),
        rn in reg_num(),
        (ts, i) in ts_and_idx(),
        bad in prop_oneof![
            Just("v32".to_string()),
            Just("foo".to_string()),
            Just("".to_string()),
            Just("v".to_string()),
        ],
    ) {
        let all = gpr_ops(rd, ts, i, rn);
        let ops: Vec<Operand> = all.iter().take(n).cloned().collect();
        let asm = if n == 0 {
            "ins".to_string()
        } else {
            format!("ins v{}.{}[{}]", rd, ts, i)
        };
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted arity-{} {}",
            n,
            asm
        );
        prop_assert!(
            encode_neon_ins(&ops).is_err(),
            "arity {} must Err (llvm-mc rejects {})",
            n,
            asm
        );

        let mut bad_ops = gpr_ops(rd, ts, i, rn);
        bad_ops[0] = Operand::RegLane {
            reg: bad.clone(),
            elem_size: ts.to_string(),
            index: i,
        };
        prop_assert!(
            encode_neon_ins(&bad_ops).is_err(),
            "invalid dest name {} must Err",
            bad
        );
    }

    #[test]
    fn encode_neon_ins_diff_alt_spellings(
        rd in reg_num(),
        rn in 0u32..=30,
        (ts, i) in ts_and_idx(),
    ) {
        let gpr = if ts == "d" {
            format!("X{}", rn)
        } else {
            format!("W{}", rn)
        };
        let asm = format!("ins V{}.{}[{}], {}", rd, ts, i, gpr);
        let ops = vec![
            Operand::RegLane {
                reg: format!("V{}", rd),
                elem_size: ts.to_string(),
                index: i,
            },
            Operand::Reg(gpr),
        ];
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected alt-spelling {}: {}", asm, e));
        let sut = sut_word(&ops)
            .unwrap_or_else(|e| panic!("SUT rejected alt-spelling {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for alt-spelling {}", asm);
    }
}

/// Deterministic regression: extra operand ignored (from neg_extra_operand).
#[test]
fn test_encode_neon_ins_regression_extra_operand() {
    let mut ops = gpr_ops(0, "b", 0, 1);
    ops.push(Operand::Reg("w2".into()));
    assert!(
        encode_neon_ins(&ops).is_err(),
        "ins v0.b[0], w1, w2 must Err (llvm-mc rejects a third operand)"
    );
}

/// Deterministic regression: out-of-range lane masked (from neg_index_oor).
#[test]
fn test_encode_neon_ins_regression_index_oor() {
    assert!(
        encode_neon_ins(&gpr_ops(0, "b", 16, 1)).is_err(),
        "ins v0.b[16], w1 must Err (llvm-mc range for .b is [0, 15])"
    );
    assert!(
        encode_neon_ins(&gpr_ops(0, "h", 8, 1)).is_err(),
        "ins v0.h[8], w1 must Err (llvm-mc range for .h is [0, 7])"
    );
    assert!(
        encode_neon_ins(&gpr_ops(0, "s", 4, 1)).is_err(),
        "ins v0.s[4], w1 must Err (llvm-mc range for .s is [0, 3])"
    );
    assert!(
        encode_neon_ins(&gpr_ops(0, "d", 2, 1)).is_err(),
        "ins v0.d[2], x1 must Err (llvm-mc range for .d is [0, 1])"
    );
}

/// Deterministic regression: wrong GPR width accepted (from neg_wrong_width_gpr).
#[test]
fn test_encode_neon_ins_regression_wrong_width_gpr() {
    let ops_x_for_b = vec![lane(0, "b", 0), Operand::Reg("x1".into())];
    assert!(
        encode_neon_ins(&ops_x_for_b).is_err(),
        "ins v0.b[0], x1 must Err (llvm-mc requires Wn for Ts=B)"
    );
    let ops_w_for_d = vec![lane(0, "d", 0), Operand::Reg("w1".into())];
    assert!(
        encode_neon_ins(&ops_w_for_d).is_err(),
        "ins v0.d[0], w1 must Err (llvm-mc requires Xn for Ts=D)"
    );
}

/// Deterministic regression: mismatched element sizes (from neg_mismatch_sp_fp).
#[test]
fn test_encode_neon_ins_regression_size_mismatch() {
    let ops = elem_ops(0, "b", 0, 1, "h", 0);
    assert!(
        encode_neon_ins(&ops).is_err(),
        "ins v0.b[0], v1.h[0] must Err (llvm-mc requires matching Ts)"
    );
}

/// Deterministic regression: SP encoded as XZR (from neg_mismatch_sp_fp).
#[test]
fn test_encode_neon_ins_regression_sp_as_zr() {
    let ops = vec![lane(0, "b", 0), Operand::Reg("sp".into())];
    assert!(
        encode_neon_ins(&ops).is_err(),
        "ins v0.b[0], sp must Err (llvm-mc rejects SP as INS GPR source)"
    );
}

/// Deterministic regression: FP register as GPR source (from neg_mismatch_sp_fp).
#[test]
fn test_encode_neon_ins_regression_fp_as_gpr() {
    let ops = vec![lane(0, "b", 0), Operand::Reg("d1".into())];
    assert!(
        encode_neon_ins(&ops).is_err(),
        "ins v0.b[0], d1 must Err (llvm-mc rejects FP as INS GPR source)"
    );
}
