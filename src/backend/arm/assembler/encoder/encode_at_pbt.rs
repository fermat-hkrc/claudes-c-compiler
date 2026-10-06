// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   encoder/mod.rs:3 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:998 "at" => encode_at(operands, raw_operands);
//   ARM ARM AT is SYS (Xt required, 64-bit GPR / XZR / LR):
//     S1E1R = SYS #0, C7, C8, #0, Xt = 0xD5087800 | Rt;
//     S1E1W = SYS #0, C7, C8, #1, Xt = 0xD5087820 | Rt;
//     S1E0R = SYS #0, C7, C8, #2, Xt = 0xD5087840 | Rt;
//     S1E0W = SYS #0, C7, C8, #3, Xt = 0xD5087860 | Rt;
//   llvm-mc -triple=aarch64 -show-encoding (LLVM 15.0.6);
//   gas: "comma expected between operands at operand 2",
//        "unexpected characters following instruction",
//        "operand mismatch" / "must be an integer register".
// Stronger considered:
//   - State machine: rejected — encode_at is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree AT decoder
//   - Differential vs encode_ic / encode_dc / encode_tlbi / encode_sys: rejected —
//     same-job gate (different SYS encodings and operand grammars)
// Weaker available: algebraic.metamorphic (Rt isolation; case/whitespace),
//   algebraic.invariant (ARM SYS layout),
//   negative_error (unknown op / missing Xt / extra operand / wrong class)
// Differential: candidate=encode_at, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=raw_operands <-> `at <op>, <Xt>`

use super::encode_at;
use super::EncodeResult;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;

/// ARM ARM SYS template with L=0 (bit 21). Independent of the SUT match table.
const SYS_TEMPLATE: u32 = 0xd5080000;
const SYS_HI: u32 = 0b11010101000; // bits[31:21]

const AT_OPS: &[&str] = &["s1e1r", "s1e1w", "s1e0r", "s1e0w"];
/// ARM AT ops llvm-mc accepts on the default CPU without extra features.
const ARM_AT_OPS: &[&str] = &[
    "s1e1r", "s1e1w", "s1e0r", "s1e0w",
    "s1e2r", "s1e2w", "s1e3r", "s1e3w",
    "s12e1r", "s12e1w", "s12e0r", "s12e0w",
];

fn cfg() -> ProptestConfig {
    ProptestConfig::with_cases(CASES)
}

/// ARM ARM SYS #<op1>, Cn, Cm, #<op2>, Xt. Not the SUT base constants.
fn arm_sys(op1: u32, crn: u32, crm: u32, op2: u32, rt: u32) -> u32 {
    SYS_TEMPLATE
        | ((op1 & 7) << 16)
        | ((crn & 0xf) << 12)
        | ((crm & 0xf) << 8)
        | ((op2 & 7) << 5)
        | (rt & 0x1f)
}

fn op2_of(op: &str) -> u32 {
    match op {
        "s1e1r" => 0,
        "s1e1w" => 1,
        "s1e0r" => 2,
        "s1e0w" => 3,
        other => panic!("not an implemented AT op: {other}"),
    }
}

fn arm_at_word(op: &str, rt: u32) -> u32 {
    arm_sys(0, 7, 8, op2_of(op), rt)
}

fn sut_word(raw: &str) -> Result<u32, String> {
    match encode_at(&[], raw)? {
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

fn llvm_asm(raw: &str) -> String {
    if raw.trim().is_empty() {
        "at".to_string()
    } else {
        format!("at {raw}")
    }
}

fn ascii_case_variant(s: &str) -> impl Strategy<Value = String> {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    if n == 0 {
        return Just(String::new()).boxed();
    }
    prop::collection::vec(any::<bool>(), n..=n)
        .prop_map(move |mask| {
            chars
                .iter()
                .zip(mask.iter())
                .map(|(ch, up)| {
                    if *up {
                        ch.to_ascii_uppercase()
                    } else {
                        ch.to_ascii_lowercase()
                    }
                })
                .collect()
        })
        .boxed()
}

fn ws_pad() -> impl Strategy<Value = String> {
    prop::collection::vec(prop::sample::select(vec![' ', '\t']), 0..=4)
        .prop_map(|v| v.into_iter().collect())
}

fn at_op() -> impl Strategy<Value = String> {
    prop::sample::select(AT_OPS.iter().map(|s| s.to_string()).collect::<Vec<_>>())
}

fn xt_token() -> impl Strategy<Value = String> {
    prop_oneof![
        (0u32..=31).prop_map(|n| format!("x{n}")),
        Just("xzr".to_string()),
        Just("lr".to_string()),
    ]
}

fn xt_num(name: &str) -> u32 {
    match name.to_ascii_lowercase().as_str() {
        "xzr" | "x31" => 31,
        "lr" | "x30" => 30,
        s if s.starts_with('x') => s[1..].parse().unwrap_or(0),
        _ => 0,
    }
}

fn valid_raw() -> impl Strategy<Value = String> {
    (at_op(), xt_token(), ws_pad(), ws_pad(), ws_pad(), ws_pad()).prop_flat_map(
        |(op, xt, p1, p2, p3, p4)| {
            let op_s = ascii_case_variant(&op);
            let reg_s = ascii_case_variant(&xt);
            (op_s, reg_s).prop_map(move |(op, reg)| format!("{p1}{op}{p2},{p3}{reg}{p4}"))
        },
    )
}

fn canonical(raw: &str) -> String {
    let parts: Vec<&str> = raw.splitn(2, ',').collect();
    let op = parts[0].trim().to_ascii_lowercase();
    if parts.len() > 1 {
        let reg = parts[1].trim().to_ascii_lowercase();
        format!("{op}, {reg}")
    } else {
        op
    }
}

fn unknown_op() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("".to_string()),
        Just("foo".to_string()),
        Just("s1e1".to_string()),
        Just("s1e1rr".to_string()),
        Just("s1e1x".to_string()),
        Just("s1e".to_string()),
        Just("iale1r".to_string()),
        Just("ialluis".to_string()),
        Just("ivau".to_string()),
        Just("civac".to_string()),
        Just("cvac".to_string()),
        Just("tlbi".to_string()),
        Just("sys".to_string()),
        Just("nop".to_string()),
        Just("ish".to_string()),
        Just("sy".to_string()),
        Just("#0".to_string()),
        Just("x0".to_string()),
        Just("xzr".to_string()),
        Just("s1e1r x0".to_string()),
        Just(",".to_string()),
        Just(", x0".to_string()),
        Just(".".to_string()),
        Just("vmalle1".to_string()),
        Just("s1e1r,s1e1w".to_string()),
        prop::string::string_regex("[A-Za-z]{1,12}").unwrap(),
        prop::string::string_regex("[A-Za-z0-9_#]{1,16}").unwrap(),
    ]
    .prop_filter("not a valid AT op name after trim+casefold", |s| {
        let op = s
            .split(',')
            .next()
            .unwrap_or("")
            .trim()
            .to_ascii_lowercase();
        !matches!(
            op.as_str(),
            "s1e1r"
                | "s1e1w"
                | "s1e0r"
                | "s1e0w"
                | "s1e2r"
                | "s1e2w"
                | "s1e3r"
                | "s1e3w"
                | "s12e1r"
                | "s12e1w"
                | "s12e0r"
                | "s12e0w"
                | "s1e1rp"
                | "s1e1wp"
                | "s1e2rp"
                | "s1e2wp"
        )
    })
}

fn wrong_reg() -> impl Strategy<Value = String> {
    prop_oneof![
        (0u32..=30).prop_map(|n| format!("w{n}")),
        Just("wzr".to_string()),
        Just("wsp".to_string()),
        Just("sp".to_string()),
        Just("w31".to_string()),
        (0u32..=31).prop_map(|n| format!("d{n}")),
        (0u32..=31).prop_map(|n| format!("s{n}")),
        (0u32..=31).prop_map(|n| format!("q{n}")),
        (0u32..=31).prop_map(|n| format!("v{n}")),
        (0u32..=31).prop_map(|n| format!("h{n}")),
        (0u32..=31).prop_map(|n| format!("b{n}")),
    ]
}

fn invalid_xt() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("x32".to_string()),
        Just("x33".to_string()),
        Just("#0".to_string()),
        Just("".to_string()),
        Just("foo".to_string()),
        Just("31".to_string()),
        Just("x".to_string()),
        (32u32..=99).prop_map(|n| format!("x{n}")),
    ]
}

fn extra_xt() -> impl Strategy<Value = String> {
    prop_oneof![
        xt_token(),
        wrong_reg(),
        Just("fp".to_string()),
        Just("#0".to_string()),
        Just("x32".to_string()),
    ]
}

#[test]
fn encode_at_kat_llvm_mc_s1e1r_x0() {
    let mc = llvm_mc_word("at s1e1r, x0").expect("llvm-mc at s1e1r, x0");
    assert_eq!(mc, 0xd5087800, "llvm-mc KAT mapping broken for at s1e1r, x0");
    assert_eq!(mc, arm_at_word("s1e1r", 0));
    let sut = sut_word("s1e1r, x0").expect("SUT KAT s1e1r x0");
    assert_eq!(sut, mc);
}

#[test]
fn encode_at_kat_llvm_mc_s1e1w_x0() {
    let mc = llvm_mc_word("at s1e1w, x0").expect("llvm-mc at s1e1w, x0");
    assert_eq!(mc, 0xd5087820, "llvm-mc KAT mapping broken for at s1e1w, x0");
    assert_eq!(mc, arm_at_word("s1e1w", 0));
    let sut = sut_word("s1e1w, x0").expect("SUT KAT s1e1w x0");
    assert_eq!(sut, mc);
}

#[test]
fn encode_at_kat_llvm_mc_s1e0r_x0() {
    let mc = llvm_mc_word("at s1e0r, x0").expect("llvm-mc at s1e0r, x0");
    assert_eq!(mc, 0xd5087840, "llvm-mc KAT mapping broken for at s1e0r, x0");
    assert_eq!(mc, arm_at_word("s1e0r", 0));
    let sut = sut_word("s1e0r, x0").expect("SUT KAT s1e0r x0");
    assert_eq!(sut, mc);
}

#[test]
fn encode_at_kat_llvm_mc_s1e0w_x0() {
    let mc = llvm_mc_word("at s1e0w, x0").expect("llvm-mc at s1e0w, x0");
    assert_eq!(mc, 0xd5087860, "llvm-mc KAT mapping broken for at s1e0w, x0");
    assert_eq!(mc, arm_at_word("s1e0w", 0));
    let sut = sut_word("s1e0w, x0").expect("SUT KAT s1e0w x0");
    assert_eq!(sut, mc);
}

#[test]
fn test_encode_at_regression_s1e2r() {
    encode_at(&[], "s1e2r, x0").expect(
        "AT S1E2R is a valid ARM op (llvm-mc/gas assemble at s1e2r, x0)",
    );
}

#[test]
fn test_encode_at_regression_missing_xt() {
    encode_at(&[], "s1e1r").expect_err(
        "AT requires Xt (llvm-mc: specified at op requires a register)",
    );
}

#[test]
fn test_encode_at_regression_w0() {
    encode_at(&[], "s1e1r, w0").expect_err(
        "AT Xt must be a 64-bit GPR (llvm-mc: invalid operand for instruction)",
    );
}

#[test]
fn encode_at_kat_llvm_mc_s1e1r_xzr() {
    let mc = llvm_mc_word("at s1e1r, xzr").expect("llvm-mc at s1e1r, xzr");
    assert_eq!(mc, 0xd508781f, "llvm-mc KAT mapping broken for at s1e1r, xzr");
    assert_eq!(mc, arm_at_word("s1e1r", 31));
    let sut = sut_word("s1e1r, xzr").expect("SUT KAT s1e1r xzr");
    assert_eq!(sut, mc);
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_at_diff_valid(raw in valid_raw()) {
        let asm = llvm_asm(&raw);
        let mc = llvm_mc_word(&asm).expect(&asm);
        let canon = canonical(&raw);
        let parts: Vec<&str> = canon.splitn(2, ',').collect();
        let op = parts[0];
        let rt = xt_num(parts[1].trim());
        let expect = arm_at_word(op, rt);
        prop_assert_eq!(mc, expect, "llvm-mc vs ARM ARM for {}", asm);
        let sut = sut_word(&raw).expect(&asm);
        prop_assert_eq!(sut, mc, "SUT vs llvm-mc for {}", asm);
    }

    #[test]
    fn encode_at_diff_arm_ops(
        op in prop::sample::select(ARM_AT_OPS.iter().map(|s| s.to_string()).collect::<Vec<_>>()),
        t in 0u32..=31
    ) {
        let raw = format!("{op}, x{t}");
        let asm = llvm_asm(&raw);
        let mc = llvm_mc_word(&asm).expect(&asm);
        let sut = sut_word(&raw).expect(&format!(
            "ARM AT op {:?} must encode (llvm-mc accepts {})",
            op, asm
        ));
        prop_assert_eq!(sut, mc, "SUT vs llvm-mc for {}", asm);
    }

    #[test]
    fn encode_at_inv_arm_layout(op in at_op(), t in 0u32..=31) {
        let raw = format!("{op}, x{t}");
        let w = sut_word(&raw).expect("AT with Xt must encode");
        prop_assert_eq!(w, arm_at_word(&op, t), "ARM SYS word for {:?}", raw);
        prop_assert_eq!(w >> 21, SYS_HI, "bits[31:21] SYS");
        prop_assert_eq!((w >> 16) & 7, 0, "AT op1=0");
        prop_assert_eq!((w >> 12) & 0xf, 7, "CRn=7");
        prop_assert_eq!((w >> 8) & 0xf, 8, "CRm=8");
        prop_assert_eq!((w >> 5) & 7, op2_of(&op), "op2");
        prop_assert_eq!(w & 0x1f, t, "Rt");
    }

    #[test]
    fn encode_at_meta_rt_isolation(op in at_op(), t in 0u32..=31) {
        let w0 = sut_word(&format!("{op}, x0")).expect("at op x0");
        let wt = sut_word(&format!("{op}, x{t}")).expect("at op xt");
        prop_assert_eq!(wt ^ w0, t, "AT encodings must differ only in Rt");
        prop_assert_eq!(wt & !0x1f, w0 & !0x1f, "AT high bits independent of Rt");

        let r = sut_word("s1e1r, x0").expect("s1e1r");
        let w = sut_word("s1e1w, x0").expect("s1e1w");
        let r0 = sut_word("s1e0r, x0").expect("s1e0r");
        let w0op = sut_word("s1e0w, x0").expect("s1e0w");
        prop_assert_eq!(w ^ r, 0x20, "S1E1W vs S1E1R differ only in op2 bit 5");
        prop_assert_eq!(r0 ^ r, 0x40, "S1E0R vs S1E1R differ only in op2 bit 6");
        prop_assert_eq!(w0op ^ r, 0x60, "S1E0W vs S1E1R differ in op2 bits[6:5]");
        prop_assert_ne!(r, w);
        prop_assert_ne!(r, r0);
        prop_assert_ne!(r, w0op);
    }

    #[test]
    fn encode_at_meta_case_ws(raw in valid_raw()) {
        let w = sut_word(&raw).expect("case/ws variant must encode");
        let w0 = sut_word(&canonical(&raw)).expect("canonical must encode");
        prop_assert_eq!(w, w0, "case/whitespace must be behavior-preserving for {:?}", raw);
    }

    #[test]
    fn encode_at_neg_unknown_op(s in unknown_op()) {
        let asm = llvm_asm(&s);
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let err = encode_at(&[], &s).expect_err(&format!(
            "unknown AT op {:?} must Err (llvm-mc rejects {})",
            s, asm
        ));
        prop_assert!(
            err.contains("unsupported at operation") || err.contains("invalid register"),
            "error must name unsupported at operation or invalid register, got {err:?} for {s:?}"
        );
    }

    #[test]
    fn encode_at_neg_missing_reg(raw in (at_op(), ws_pad(), ws_pad()).prop_flat_map(|(op, pre, post)| {
        ascii_case_variant(&op).prop_map(move |op| format!("{pre}{op}{post}"))
    })) {
        let asm = llvm_asm(&raw);
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let _err = encode_at(&[], &raw).expect_err(&format!(
            "AT without register must Err (llvm-mc rejects {}); SUT raw {:?}",
            asm, raw
        ));
    }

    #[test]
    fn encode_at_neg_wrong_reg_class(op in at_op(), bad in wrong_reg()) {
        let raw = format!("{op}, {bad}");
        let asm = llvm_asm(&raw);
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let _err = encode_at(&[], &raw).expect_err(&format!(
            "AT with non-X register must Err (llvm-mc rejects {}); SUT raw {:?}",
            asm, raw
        ));
    }

    #[test]
    fn encode_at_neg_invalid_reg(op in at_op(), xt in invalid_xt()) {
        let raw = format!("{op}, {xt}");
        let asm = llvm_asm(&raw);
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let err = encode_at(&[], &raw).expect_err(&format!(
            "malformed AT Xt must Err (llvm-mc rejects {}); SUT raw {:?}",
            asm, raw
        ));
        prop_assert!(
            err.contains("invalid register") || err.contains("unsupported at operation"),
            "error must name invalid register, got {err:?} for {raw:?}"
        );
    }

    #[test]
    fn encode_at_neg_extra_operand(op in at_op(), extra in extra_xt()) {
        let raw = format!("{op}, x0, {extra}");
        let asm = llvm_asm(&raw);
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let _err = encode_at(&[], &raw).expect_err(&format!(
            "AT with extra operand must Err (llvm-mc rejects {}); SUT raw {:?}",
            asm, raw
        ));
    }
}
