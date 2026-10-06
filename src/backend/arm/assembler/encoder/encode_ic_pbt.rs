// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md:239 System table lists ic;
//   encoder/mod.rs:3 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:983 "ic" => encode_ic(raw_operands);
//   ARM ARM IC is SYS:
//     IALLUIS = SYS #0, C7, C1, #0 = 0xD508711F (no Xt);
//     IALLU   = SYS #0, C7, C5, #0 = 0xD508751F (no Xt);
//     IVAU    = SYS #3, C7, C5, #1, Xt = 0xD50B7520 | Rt (Xt required, 64-bit GPR);
//   llvm-mc -triple=aarch64 -show-encoding (LLVM 15.0.6);
//   gas: "extraneous register at operand 2", "missing register at operand 2",
//        "unknown or missing operation name", "operand mismatch".
// Stronger considered:
//   - State machine: rejected — encode_ic is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree IC decoder
//   - Differential vs encode_dc / encode_tlbi / encode_at / encode_sys: rejected —
//     same-job gate (different SYS encodings and operand grammars)
// Weaker available: algebraic.metamorphic (IVAU Rt isolation; IALLUIS vs IALLU CRm;
//   case/whitespace), algebraic.invariant (ARM SYS layout),
//   negative_error (unknown op / extra Xt on IALLU* / missing Xt on IVAU / wrong class)
// Differential: candidate=encode_ic, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=raw_operands <-> `ic <op>` / `ic ivau, <Xt>`

use super::encode_ic;
use super::EncodeResult;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;

/// ARM ARM SYS template with L=0 (bit 21). Independent of the SUT match table.
const SYS_TEMPLATE: u32 = 0xd5080000;
const SYS_HI: u32 = 0b11010101000; // bits[31:21]

const NO_XT: &[&str] = &["ialluis", "iallu"];

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

fn arm_ic_word(op: &str, rt: u32) -> u32 {
    match op {
        "ialluis" => arm_sys(0, 7, 1, 0, 31),
        "iallu" => arm_sys(0, 7, 5, 0, 31),
        "ivau" => arm_sys(3, 7, 5, 1, rt),
        other => panic!("not an ARM IC op: {other}"),
    }
}

fn sut_word(raw: &str) -> Result<u32, String> {
    match encode_ic(raw)? {
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
        "ic".to_string()
    } else {
        format!("ic {raw}")
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

fn no_xt_op() -> impl Strategy<Value = String> {
    prop::sample::select(NO_XT.iter().map(|s| s.to_string()).collect::<Vec<_>>())
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
    let no_xt = (no_xt_op(), ws_pad(), ws_pad()).prop_flat_map(|(op, pre, post)| {
        ascii_case_variant(&op).prop_map(move |cased| format!("{pre}{cased}{post}"))
    });
    let ivau = (ws_pad(), ws_pad(), ws_pad(), ws_pad(), xt_token()).prop_flat_map(
        |(p1, p2, p3, p4, xt)| {
            let op_s = ascii_case_variant("ivau");
            let reg_s = ascii_case_variant(&xt);
            (op_s, reg_s).prop_map(move |(op, reg)| format!("{p1}{op}{p2},{p3}{reg}{p4}"))
        },
    );
    prop_oneof![no_xt, ivau]
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
        Just("ialluu".to_string()),
        Just("iallui".to_string()),
        Just("ialluiss".to_string()),
        Just("ivauu".to_string()),
        Just("ivauis".to_string()),
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
        Just("iall".to_string()),
        Just("ivau ialluis".to_string()),
        Just("ialluis x0".to_string()),
        Just("ivau x0".to_string()),
        Just(",".to_string()),
        Just(", x0".to_string()),
        Just(".".to_string()),
        Just("ivalleis".to_string()),
        Just("ivallis".to_string()),
        prop::string::string_regex("[A-Za-z]{1,12}").unwrap(),
        prop::string::string_regex("[A-Za-z0-9_#]{1,16}").unwrap(),
    ]
    .prop_filter("not a valid IC op name after trim+casefold", |s| {
        let op = s
            .split(',')
            .next()
            .unwrap_or("")
            .trim()
            .to_ascii_lowercase();
        !matches!(op.as_str(), "ialluis" | "iallu" | "ivau")
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
        Just("x0, x1".to_string()),
        Just("foo".to_string()),
        Just("31".to_string()),
        Just("x".to_string()),
        Just("xzr, x0".to_string()),
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
fn encode_ic_kat_llvm_mc_ialluis() {
    let mc = llvm_mc_word("ic ialluis").expect("llvm-mc ic ialluis");
    assert_eq!(mc, 0xd508711f, "llvm-mc KAT mapping broken for ic ialluis");
    assert_eq!(mc, arm_ic_word("ialluis", 31));
    let sut = sut_word("ialluis").expect("SUT KAT ialluis");
    assert_eq!(sut, mc);
}

#[test]
fn encode_ic_kat_llvm_mc_iallu() {
    let mc = llvm_mc_word("ic iallu").expect("llvm-mc ic iallu");
    assert_eq!(mc, 0xd508751f, "llvm-mc KAT mapping broken for ic iallu");
    assert_eq!(mc, arm_ic_word("iallu", 31));
    let sut = sut_word("iallu").expect("SUT KAT iallu");
    assert_eq!(sut, mc);
}

#[test]
fn encode_ic_kat_llvm_mc_ivau_x0() {
    let mc = llvm_mc_word("ic ivau, x0").expect("llvm-mc ic ivau, x0");
    assert_eq!(mc, 0xd50b7520, "llvm-mc KAT mapping broken for ic ivau, x0");
    assert_eq!(mc, arm_ic_word("ivau", 0));
    let sut = sut_word("ivau, x0").expect("SUT KAT ivau x0");
    assert_eq!(sut, mc);
}

#[test]
fn encode_ic_kat_llvm_mc_ivau_xzr() {
    let mc = llvm_mc_word("ic ivau, xzr").expect("llvm-mc ic ivau, xzr");
    assert_eq!(mc, 0xd50b753f, "llvm-mc KAT mapping broken for ic ivau, xzr");
    assert_eq!(mc, arm_ic_word("ivau", 31));
    let sut = sut_word("ivau, xzr").expect("SUT KAT ivau xzr");
    assert_eq!(sut, mc);
}

#[test]
fn test_encode_ic_regression_ialluis_x0() {
    encode_ic("ialluis, x0").expect_err(
        "IALLUIS must not accept a register (llvm-mc: specified ic op does not use a register)",
    );
}

#[test]
fn test_encode_ic_regression_ivau_missing() {
    encode_ic("ivau").expect_err(
        "IVAU requires Xt (llvm-mc: specified ic op requires a register)",
    );
}

#[test]
fn test_encode_ic_regression_ivau_w0() {
    encode_ic("ivau, w0").expect_err(
        "IVAU Xt must be a 64-bit GPR (llvm-mc: invalid operand for instruction)",
    );
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_ic_diff_valid(raw in valid_raw()) {
        let asm = llvm_asm(&raw);
        let mc = llvm_mc_word(&asm).expect(&asm);
        let canon = canonical(&raw);
        let parts: Vec<&str> = canon.splitn(2, ',').collect();
        let op = parts[0];
        let rt = if parts.len() > 1 {
            xt_num(parts[1].trim())
        } else {
            31
        };
        let expect = arm_ic_word(op, rt);
        prop_assert_eq!(mc, expect, "llvm-mc vs ARM ARM for {}", asm);
        let sut = sut_word(&raw).expect(&asm);
        prop_assert_eq!(sut, mc, "SUT vs llvm-mc for {}", asm);
    }

    #[test]
    fn encode_ic_inv_arm_layout(
        op in no_xt_op(),
        t in 0u32..=31
    ) {
        let w_no = sut_word(&op).expect("IALLUIS/IALLU must encode");
        prop_assert_eq!(w_no, arm_ic_word(&op, 31), "ARM SYS word for {:?}", op);
        prop_assert_eq!(w_no >> 21, SYS_HI, "bits[31:21] SYS");
        prop_assert_eq!((w_no >> 16) & 7, 0, "IALLU* op1=0");
        prop_assert_eq!((w_no >> 12) & 0xf, 7, "CRn=7");
        let crm = if op == "ialluis" { 1 } else { 5 };
        prop_assert_eq!((w_no >> 8) & 0xf, crm, "CRm");
        prop_assert_eq!((w_no >> 5) & 7, 0, "op2=0");
        prop_assert_eq!(w_no & 0x1f, 31, "Rt=XZR");

        let raw = format!("ivau, x{t}");
        let w = sut_word(&raw).expect("IVAU with Xt must encode");
        prop_assert_eq!(w, arm_ic_word("ivau", t), "ARM SYS word for {:?}", raw);
        prop_assert_eq!(w >> 21, SYS_HI, "bits[31:21] SYS");
        prop_assert_eq!((w >> 16) & 7, 3, "IVAU op1=3");
        prop_assert_eq!((w >> 12) & 0xf, 7, "CRn=7");
        prop_assert_eq!((w >> 8) & 0xf, 5, "CRm=5");
        prop_assert_eq!((w >> 5) & 7, 1, "op2=1");
        prop_assert_eq!(w & 0x1f, t, "Rt");
    }

    #[test]
    fn encode_ic_meta_rt_isolation(t in 0u32..=31) {
        let w0 = sut_word("ivau, x0").expect("ivau x0");
        let wt = sut_word(&format!("ivau, x{t}")).expect("ivau xt");
        prop_assert_eq!(wt ^ w0, t, "IVAU encodings must differ only in Rt");
        prop_assert_eq!(wt & !0x1f, w0 & !0x1f, "IVAU high bits independent of Rt");

        let w_is = sut_word("ialluis").expect("ialluis");
        let w_u = sut_word("iallu").expect("iallu");
        prop_assert_eq!(w_u ^ w_is, 0x400, "IALLU vs IALLUIS must differ only in CRm bit 10");
        prop_assert_eq!(w_is & 0x1f, 31);
        prop_assert_eq!(w_u & 0x1f, 31);
        prop_assert_ne!(w_is, w_u);
        prop_assert_ne!(w0, w_is);
    }

    #[test]
    fn encode_ic_meta_case_ws(raw in valid_raw()) {
        let w = sut_word(&raw).expect("case/ws variant must encode");
        let w0 = sut_word(&canonical(&raw)).expect("canonical must encode");
        prop_assert_eq!(w, w0, "case/whitespace must be behavior-preserving for {:?}", raw);
    }

    #[test]
    fn encode_ic_neg_unknown_op(s in unknown_op()) {
        let asm = llvm_asm(&s);
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let err = encode_ic(&s).expect_err(&format!(
            "unknown IC op {:?} must Err (llvm-mc rejects {})",
            s, asm
        ));
        prop_assert!(
            err.contains("unsupported ic operation") || err.contains("invalid register"),
            "error must name unsupported ic operation or invalid register, got {err:?} for {s:?}"
        );
    }

    #[test]
    fn encode_ic_neg_iallu_with_reg(op in no_xt_op(), xt in extra_xt()) {
        let raw = format!("{op}, {xt}");
        let asm = llvm_asm(&raw);
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let _err = encode_ic(&raw).expect_err(&format!(
            "IALLU* with register must Err (llvm-mc rejects {}); SUT raw {:?}",
            asm, raw
        ));
    }

    #[test]
    fn encode_ic_neg_ivau_missing_reg(raw in (ws_pad(), ws_pad()).prop_flat_map(|(pre, post)| {
        ascii_case_variant("ivau").prop_map(move |op| format!("{pre}{op}{post}"))
    })) {
        let asm = llvm_asm(&raw);
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let _err = encode_ic(&raw).expect_err(&format!(
            "IVAU without register must Err (llvm-mc rejects {}); SUT raw {:?}",
            asm, raw
        ));
    }

    #[test]
    fn encode_ic_neg_wrong_reg_class(bad in wrong_reg()) {
        let raw = format!("ivau, {bad}");
        let asm = llvm_asm(&raw);
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let _err = encode_ic(&raw).expect_err(&format!(
            "IVAU with non-X register must Err (llvm-mc rejects {}); SUT raw {:?}",
            asm, raw
        ));
    }

    #[test]
    fn encode_ic_neg_invalid_reg(xt in invalid_xt()) {
        let raw = format!("ivau, {xt}");
        let asm = llvm_asm(&raw);
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let err = encode_ic(&raw).expect_err(&format!(
            "malformed IVAU Xt must Err (llvm-mc rejects {}); SUT raw {:?}",
            asm, raw
        ));
        prop_assert!(
            err.contains("invalid register") || err.contains("unsupported ic operation"),
            "error must name invalid register, got {err:?} for {raw:?}"
        );
    }
}
