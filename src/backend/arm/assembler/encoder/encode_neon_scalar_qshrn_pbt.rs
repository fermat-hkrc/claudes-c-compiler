// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md:237 NEON scalar lists sqshrn (scalar);
//   encoder/mod.rs:1-7 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:725-728 "sqshrn" => encode_neon_scalar_qshrn when dest is Operand::Reg;
//   neon.rs:1834 NEON scalar SQSHRN: sqshrn Hd,Sn,#shift / sqshrn Sd,Dn,#shift;
//   ARM ARM Advanced SIMD scalar shift by immediate (asisdshf):
//     01 U 11111 immh:immb opcode Rn Rd;
//     SQSHRN U=0 opcode=100101; SQRSHRN U=0 opcode=100111;
//     UQSHRN U=1 opcode=100101; UQRSHRN U=1 opcode=100111;
//     dest/src B<-H / H<-S / S<-D; shift in 1..=dest_esize.
// Stronger considered:
//   - State machine: rejected — encode_neon_scalar_qshrn is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree scalar SQSHRN decoder
//   - Differential vs encode_neon_qshrn: rejected — same-job gate (vector Vd.Tb vs scalar)
// Weaker available: algebraic.metamorphic (Rd/Rn/U/rounding/immhb fields), algebraic.invariant (word layout),
//   negative_error (arity / extra / wrong register class / shift OOB / non-register)
// Differential: candidate=encode_neon_scalar_qshrn, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[Reg(<Vd><d>), Reg(<Vn><n>), Imm(shift)] + (U, is_rounding)
//     <-> `sqshrn|sqrshrn|uqshrn|uqrshrn <Vd><d>, <Vn><n>, #<shift>`

use super::encode_neon_scalar_qshrn;
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

fn mnemonic(u: u32, round: bool) -> &'static str {
    match (u, round) {
        (0, false) => "sqshrn",
        (0, true) => "sqrshrn",
        (1, false) => "uqshrn",
        (1, true) => "uqrshrn",
        _ => "sqshrn",
    }
}

fn opcode_bits(round: bool) -> u32 {
    if round {
        0b100111
    } else {
        0b100101
    }
}

fn sreg(pfx: &str, n: u32) -> Operand {
    Operand::Reg(format!("{pfx}{n}"))
}

fn ops_ok(vd: &str, vn: &str, rd: u32, rn: u32, shift: i64) -> Vec<Operand> {
    vec![sreg(vd, rd), sreg(vn, rn), Operand::Imm(shift)]
}

fn asm_ok(vd: &str, vn: &str, rd: u32, rn: u32, shift: i64, u: u32, round: bool) -> String {
    format!("{} {vd}{rd}, {vn}{rn}, #{shift}", mnemonic(u, round))
}

fn sut_word(ops: &[Operand], u: u32, round: bool) -> Result<u32, String> {
    match encode_neon_scalar_qshrn(ops, u, round)? {
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

fn dest_src_esize() -> impl Strategy<Value = (&'static str, &'static str, u32)> {
    prop::sample::select(vec![("b", "h", 8u32), ("h", "s", 16), ("s", "d", 32)])
}

fn valid_case() -> impl Strategy<Value = (&'static str, &'static str, u32, i64)> {
    dest_src_esize().prop_flat_map(|(vd, vn, es)| {
        let sh = prop_oneof![Just(1i64), Just(es as i64), 1i64..=(es as i64)];
        sh.prop_map(move |shift| (vd, vn, es, shift))
    })
}

fn u_bit() -> impl Strategy<Value = u32> {
    0u32..=1
}

fn src_pfx() -> impl Strategy<Value = &'static str> {
    prop::sample::select(vec![
        "b", "h", "s", "d", "x", "w", "q", "v", "sp", "xzr", "wsp", "wzr", "lr",
    ])
}

fn dest_pfx() -> impl Strategy<Value = &'static str> {
    prop::sample::select(vec!["b", "h", "s"])
}

fn named_operand(pfx: &str, n: u32) -> (Operand, String) {
    match pfx {
        "sp" | "wsp" | "xzr" | "wzr" | "lr" => (Operand::Reg(pfx.to_string()), pfx.to_string()),
        _ => {
            let name = format!("{pfx}{n}");
            (Operand::Reg(name.clone()), name)
        }
    }
}

fn mandated_src(vd: &str) -> &'static str {
    match vd {
        "b" => "h",
        "h" => "s",
        "s" => "d",
        _ => "h",
    }
}

fn dest_esize(vd: &str) -> u32 {
    match vd {
        "b" => 8,
        "h" => 16,
        "s" => 32,
        _ => 8,
    }
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_neon_scalar_qshrn_kat_llvm_mc() {
    let kat: &[(&str, u32)] = &[
        ("sqshrn h0, s1, #1", 0x5f1f9420),
        ("sqshrn h0, s1, #16", 0x5f109420),
        ("sqshrn b0, h1, #1", 0x5f0f9420),
        ("sqshrn b0, h1, #8", 0x5f089420),
        ("sqshrn s0, d1, #1", 0x5f3f9420),
        ("sqshrn s0, d1, #32", 0x5f209420),
        ("sqshrn b31, h31, #8", 0x5f0897ff),
        ("sqrshrn h0, s1, #1", 0x5f1f9c20),
        ("uqshrn h0, s1, #1", 0x7f1f9420),
        ("uqrshrn h0, s1, #1", 0x7f1f9c20),
        ("SQSHRN H0, S1, #1", 0x5f1f9420),
        ("sqshrn s15, d16, #17", 0x5f2f960f),
    ];
    for &(asm, want) in kat {
        let mc = llvm_mc_word(asm).unwrap_or_else(|e| panic!("llvm-mc KAT {asm}: {e}"));
        assert_eq!(mc, want, "llvm-mc KAT mapping broken for {asm}");
    }
}

proptest! {
    #![proptest_config(cfg())]

    // Oracle: differential — llvm-mc AArch64 assembler
    #[test]
    fn encode_neon_scalar_qshrn_diff_llvm_mc(
        rd in reg_num(),
        rn in reg_num(),
        case in valid_case(),
        u in u_bit(),
        round in any::<bool>(),
    ) {
        let (vd, vn, _es, shift) = case;
        let asm = asm_ok(vd, vn, rd, rn, shift, u, round);
        let ops = ops_ok(vd, vn, rd, rn, shift);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&ops, u, round)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for {}", asm);
    }

    // Oracle: algebraic.metamorphic — Rd/Rn/U/rounding/immhb field isolation
    #[test]
    fn encode_neon_scalar_qshrn_meta_rd_rn_u_round_immhb(
        rd1 in reg_num(),
        rd2 in reg_num(),
        rn1 in reg_num(),
        rn2 in reg_num(),
        u1 in u_bit(),
        u2 in u_bit(),
        round1 in any::<bool>(),
        round2 in any::<bool>(),
        case1 in valid_case(),
        case2 in valid_case(),
    ) {
        let (vd1, vn1, _es1, sh1) = case1;
        let (vd2, vn2, _es2, sh2) = case2;
        let w111 = sut_word(&ops_ok(vd1, vn1, rd1, rn1, sh1), u1, round1)
            .unwrap_or_else(|e| panic!("SUT rejected base: {}", e));
        let w_rd = sut_word(&ops_ok(vd1, vn1, rd2, rn1, sh1), u1, round1)
            .unwrap_or_else(|e| panic!("SUT rejected rd2: {}", e));
        let w_rn = sut_word(&ops_ok(vd1, vn1, rd1, rn2, sh1), u1, round1)
            .unwrap_or_else(|e| panic!("SUT rejected rn2: {}", e));
        let w_u = sut_word(&ops_ok(vd1, vn1, rd1, rn1, sh1), u2, round1)
            .unwrap_or_else(|e| panic!("SUT rejected u2: {}", e));
        let w_r = sut_word(&ops_ok(vd1, vn1, rd1, rn1, sh1), u1, round2)
            .unwrap_or_else(|e| panic!("SUT rejected round2: {}", e));
        let w_imm = sut_word(&ops_ok(vd2, vn2, rd1, rn1, sh2), u1, round1)
            .unwrap_or_else(|e| panic!("SUT rejected dest/shift: {}", e));
        prop_assert_eq!(
            (w111 ^ w_rd) & !0x1Fu32,
            0u32,
            "changing only Rd must differ only in bits[4:0]"
        );
        prop_assert_eq!(w_rd & 0x1F, rd2, "Rd field");
        prop_assert_eq!(
            (w111 ^ w_rn) & !(0x1Fu32 << 5),
            0u32,
            "changing only Rn must differ only in bits[9:5]"
        );
        prop_assert_eq!((w_rn >> 5) & 0x1F, rn2, "Rn field");
        prop_assert_eq!(
            (w111 ^ w_u) & !(1u32 << 29),
            0u32,
            "changing only U must differ only in bit 29"
        );
        prop_assert_eq!((w_u >> 29) & 1, u2, "U field");
        prop_assert_eq!(
            (w111 ^ w_r) & !(0x3Fu32 << 10),
            0u32,
            "changing only rounding must differ only in bits[15:10]"
        );
        prop_assert_eq!((w_r >> 10) & 0x3F, opcode_bits(round2), "opcode field");
        prop_assert_eq!(
            (w111 ^ w_imm) & !(0xFFu32 << 16),
            0u32,
            "changing only dest/shift must differ only in bits[23:16]"
        );
        let immhb = (dest_esize(vd2) * 2) - (sh2 as u32);
        prop_assert_eq!((w_imm >> 16) & 0xFF, immhb, "immh:immb field");
    }

    // Oracle: algebraic.invariant — ARM asisdshf field layout
    #[test]
    fn encode_neon_scalar_qshrn_invariant_arm_fields(
        rd in reg_num(),
        rn in reg_num(),
        case in valid_case(),
        u in u_bit(),
        round in any::<bool>(),
    ) {
        let (vd, vn, es, shift) = case;
        let w = sut_word(&ops_ok(vd, vn, rd, rn, shift), u, round)
            .unwrap_or_else(|e| panic!("SUT rejected: {}", e));
        let immhb = (es * 2) - (shift as u32);
        prop_assert_eq!((w >> 30) & 3, 0b01u32, "bits[31:30]=01");
        prop_assert_eq!((w >> 29) & 1, u, "U");
        prop_assert_eq!((w >> 24) & 0x1F, 0b11111u32, "bits[28:24]=11111 (ARM asisdshf)");
        prop_assert_eq!((w >> 16) & 0xFF, immhb, "immh:immb = 2*esize - shift");
        prop_assert_eq!((w >> 10) & 0x3F, opcode_bits(round), "opcode");
        prop_assert_eq!((w >> 5) & 0x1F, rn, "Rn");
        prop_assert_eq!(w & 0x1F, rd, "Rd");
    }

    // Oracle: negative_error — arity 0..=2
    #[test]
    fn encode_neon_scalar_qshrn_neg_arity(
        n in 0usize..=2,
        rd in reg_num(),
        rn in reg_num(),
        case in valid_case(),
        u in u_bit(),
        round in any::<bool>(),
    ) {
        let (vd, vn, _es, shift) = case;
        let all = ops_ok(vd, vn, rd, rn, shift);
        let arity_ops: Vec<Operand> = all.iter().take(n).cloned().collect();
        let mn = mnemonic(u, round);
        let arity_asm = match n {
            0 => mn.to_string(),
            1 => format!("{mn} {vd}{rd}"),
            _ => format!("{mn} {vd}{rd}, {vn}{rn}"),
        };
        prop_assert!(
            llvm_mc_word(&arity_asm).is_err(),
            "llvm-mc unexpectedly accepted arity-{} {}",
            n,
            arity_asm
        );
        prop_assert!(
            encode_neon_scalar_qshrn(&arity_ops, u, round).is_err(),
            "arity {} must Err (llvm-mc rejects {})",
            n,
            arity_asm
        );
    }

    // Oracle: negative_error — fourth operand
    #[test]
    fn encode_neon_scalar_qshrn_neg_extra_operand(
        rd in reg_num(),
        rn in reg_num(),
        extra in reg_num(),
        case in valid_case(),
        u in u_bit(),
        round in any::<bool>(),
    ) {
        let (vd, vn, _es, shift) = case;
        let asm = format!("{}, {vd}{extra}", asm_ok(vd, vn, rd, rn, shift, u, round));
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted 4-operand {}",
            asm
        );
        let mut ops = ops_ok(vd, vn, rd, rn, shift);
        ops.push(sreg(vd, extra));
        prop_assert!(
            encode_neon_scalar_qshrn(&ops, u, round).is_err(),
            "4 operands must Err (llvm-mc rejects {})",
            asm
        );
    }

    // Oracle: negative_error — dest/src class mismatch or non-mandated pair
    #[test]
    fn encode_neon_scalar_qshrn_neg_wrong_reg_class(
        rd in reg_num(),
        rn in reg_num(),
        vd in dest_pfx(),
        src in src_pfx(),
        u in u_bit(),
        round in any::<bool>(),
    ) {
        let want_src = mandated_src(vd);
        prop_assume!(src != want_src);
        let es = dest_esize(vd);
        let shift = es as i64;
        let (src_op, src_name) = named_operand(src, rn);
        let ops = vec![sreg(vd, rd), src_op, Operand::Imm(shift)];
        let mn = mnemonic(u, round);
        let asm = format!("{mn} {vd}{rd}, {src_name}, #{shift}");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        prop_assert!(
            encode_neon_scalar_qshrn(&ops, u, round).is_err(),
            "wrong class src={} must Err (llvm-mc rejects {})",
            src,
            asm
        );
    }

    // Oracle: negative_error — shift 0 / esize+1 / negative / large
    #[test]
    fn encode_neon_scalar_qshrn_neg_shift_oob(
        rd in reg_num(),
        rn in reg_num(),
        pair in dest_src_esize(),
        kind in 0u8..=4,
        u in u_bit(),
        round in any::<bool>(),
    ) {
        let (vd, vn, es) = pair;
        let shift: i64 = match kind {
            0 => 0,
            1 => (es as i64) + 1,
            2 => -1,
            3 => 64,
            _ => 255,
        };
        let mn = mnemonic(u, round);
        let asm = format!("{mn} {vd}{rd}, {vn}{rn}, #{shift}");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted OOB shift {}",
            asm
        );
        let ops = ops_ok(vd, vn, rd, rn, shift);
        prop_assert!(
            encode_neon_scalar_qshrn(&ops, u, round).is_err(),
            "shift {} must Err for esize {} (llvm-mc rejects {})",
            shift,
            es,
            asm
        );
    }

    // Oracle: differential — uppercase prefix / mnemonic vs llvm-mc
    #[test]
    fn encode_neon_scalar_qshrn_diff_alt_spellings(
        rd in reg_num(),
        rn in reg_num(),
        case in valid_case(),
        u in u_bit(),
        round in any::<bool>(),
    ) {
        let (vd, vn, _es, shift) = case;
        let up_d = vd.to_ascii_uppercase();
        let up_n = vn.to_ascii_uppercase();
        let mn = mnemonic(u, round).to_ascii_uppercase();
        let asm = format!("{mn} {up_d}{rd}, {up_n}{rn}, #{shift}");
        let ops = vec![
            Operand::Reg(format!("{up_d}{rd}")),
            Operand::Reg(format!("{up_n}{rn}")),
            Operand::Imm(shift),
        ];
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected alt-spelling {}: {}", asm, e));
        let sut = sut_word(&ops, u, round)
            .unwrap_or_else(|e| panic!("SUT rejected alt-spelling {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for alt-spelling {}", asm);
    }

    // Oracle: negative_error — dest prefix x/w/q/v/d (documented unsupported dest)
    #[test]
    fn encode_neon_scalar_qshrn_neg_unsupported_dest(
        rd in reg_num(),
        rn in reg_num(),
        dest_pfx in prop::sample::select(vec!["x", "w", "q", "v", "d"]),
        src_pfx in prop::sample::select(vec!["h", "s", "d"]),
        u in u_bit(),
        round in any::<bool>(),
    ) {
        let mn = mnemonic(u, round);
        let ops = vec![sreg(dest_pfx, rd), sreg(src_pfx, rn), Operand::Imm(1)];
        let asm = format!("{mn} {dest_pfx}{rd}, {src_pfx}{rn}, #1");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        prop_assert!(
            encode_neon_scalar_qshrn(&ops, u, round).is_err(),
            "dest prefix {} must Err (llvm-mc rejects {})",
            dest_pfx,
            asm
        );
    }

    // Oracle: negative_error — Imm/Mem/Label in dest or src slot
    #[test]
    fn encode_neon_scalar_qshrn_neg_nonreg(
        rd in reg_num(),
        rn in reg_num(),
        case in valid_case(),
        u in u_bit(),
        round in any::<bool>(),
        kind in 0u8..=2,
        slot in 0usize..=1,
    ) {
        let (vd, vn, _es, shift) = case;
        let mn = mnemonic(u, round);
        let bad = match kind {
            0 => Operand::Imm(0),
            1 => Operand::Mem {
                base: format!("x{}", rd),
                offset: 0,
            },
            _ => Operand::Label("L0".into()),
        };
        let mut ops = ops_ok(vd, vn, rd, rn, shift);
        ops[slot] = bad;
        let asm = match (kind, slot) {
            (0, 0) => format!("{mn} #0, {vn}{rn}, #{shift}"),
            (1, 0) => format!("{mn} [x{rd}], {vn}{rn}, #{shift}"),
            (2, 0) => format!("{mn} L0, {vn}{rn}, #{shift}"),
            (0, 1) => format!("{mn} {vd}{rd}, #0, #{shift}"),
            (1, 1) => format!("{mn} {vd}{rd}, [x{rd}], #{shift}"),
            _ => format!("{mn} {vd}{rd}, L0, #{shift}"),
        };
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        prop_assert!(
            encode_neon_scalar_qshrn(&ops, u, round).is_err(),
            "non-register operand slot={} kind={} must Err",
            slot,
            kind
        );
    }
}

/// Deterministic regression: scalar SQSHRN packed as vector Q=1 (from diff_llvm_mc).
#[test]
fn test_encode_neon_scalar_qshrn_regression_asisdshf_bit28() {
    let ops = ops_ok("b", "h", 0, 0, 1);
    let sut = sut_word(&ops, 0, false).expect("valid sqshrn b0, h0, #1");
    assert_eq!(
        sut, 0x5f0f9400,
        "sqshrn b0, h0, #1 must match llvm-mc/ARM asisdshf 0x5f0f9400, not vector-Q=1 0x4f0f9400"
    );
}

/// Deterministic regression: extra operand ignored (from neg_extra_operand).
#[test]
fn test_encode_neon_scalar_qshrn_regression_extra_operand() {
    let mut ops = ops_ok("b", "h", 0, 0, 1);
    ops.push(sreg("b", 0));
    assert!(
        encode_neon_scalar_qshrn(&ops, 0, false).is_err(),
        "sqshrn b0, h0, #1, b0 must Err (gas/llvm-mc reject a fourth operand)"
    );
}

/// Deterministic regression: dest/src class mismatch encoded (from neg_wrong_reg_class).
#[test]
fn test_encode_neon_scalar_qshrn_regression_wrong_reg_class() {
    let ops = ops_ok("b", "s", 0, 0, 8);
    assert!(
        encode_neon_scalar_qshrn(&ops, 0, false).is_err(),
        "sqshrn b0, s0, #8 must Err (ARM/gas/llvm-mc require B<-H / H<-S / S<-D)"
    );
}
