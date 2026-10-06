// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   encoder/mod.rs:3 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:997 "sys" => encode_sys(raw_operands);
//   system.rs:447 "Encode `sys #op1, Cn, Cm, #op2, Xt` instruction.";
//   ARM ARM SYS #<op1>, <Cn>, <Cm>, #<op2>{, <Xt>} =
//     0xD5080000 | (op1<<16) | (CRn<<12) | (CRm<<8) | (op2<<5) | Rt
//     op1,op2 in [0,7]; Cn,Cm in C0–C15; Xt 64-bit GPR (XZR/LR/FP); Xt optional → XZR;
//   llvm-mc -triple=aarch64 -show-encoding (LLVM 15.0.6);
//   gas: "too few operands" / "comma expected",
//        "immediate value out of range 0 to 7",
//        "C0 - C15 expected",
//        "unexpected characters following instruction",
//        "operand mismatch" / "must be an integer register".
// Stronger considered:
//   - State machine: rejected — encode_sys is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree SYS decoder
//   - Differential vs encode_ic / encode_dc / encode_tlbi / encode_at: rejected —
//     same-job gate (named aliases of fixed SYS encodings, different operand grammar)
// Weaker available: algebraic.metamorphic (Rt isolation; omitted Xt ≡ xzr; case/ws),
//   algebraic.invariant (ARM SYS layout),
//   negative_error (too few / oob fields / extra operand / wrong class)
// Differential: candidate=encode_sys, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=raw_operands <-> `sys <raw>`

use super::encode_sys;
use super::EncodeResult;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;

/// ARM ARM SYS template with L=0 (bit 21). Independent of the SUT body.
const SYS_TEMPLATE: u32 = 0xd5080000;
const SYS_HI: u32 = 0b11010101000; // bits[31:21]

fn cfg() -> ProptestConfig {
    ProptestConfig::with_cases(CASES)
}

/// ARM ARM SYS #<op1>, Cn, Cm, #<op2>, Xt. Inputs are already in-range; no mask.
fn arm_sys(op1: u32, crn: u32, crm: u32, op2: u32, rt: u32) -> u32 {
    SYS_TEMPLATE | (op1 << 16) | (crn << 12) | (crm << 8) | (op2 << 5) | rt
}

fn sut_word(raw: &str) -> Result<u32, String> {
    match encode_sys(raw)? {
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
        "sys".to_string()
    } else {
        format!("sys {raw}")
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
                    if !ch.is_ascii_alphabetic() {
                        *ch
                    } else if *up {
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

fn hash_prefix() -> impl Strategy<Value = String> {
    prop_oneof![Just("#".to_string()), Just("".to_string())]
}

/// Xt token including omitted (None). fp is x29; llvm-mc/gas accept it.
fn xt_choice() -> impl Strategy<Value = Option<String>> {
    prop_oneof![
        (0u32..=31).prop_map(|n| Some(format!("x{n}"))),
        Just(Some("xzr".to_string())),
        Just(Some("lr".to_string())),
        Just(Some("fp".to_string())),
        Just(None),
    ]
}

fn xt_num(name: &str) -> u32 {
    match name.to_ascii_lowercase().as_str() {
        "xzr" | "x31" => 31,
        "lr" | "x30" => 30,
        "fp" | "x29" => 29,
        s if s.starts_with('x') => s[1..].parse().unwrap_or(0),
        _ => 0,
    }
}

fn canonical_raw(op1: u32, crn: u32, crm: u32, op2: u32, xt: Option<&str>) -> String {
    match xt {
        Some(xt) => format!("#{op1}, c{crn}, c{crm}, #{op2}, {xt}"),
        None => format!("#{op1}, c{crn}, c{crm}, #{op2}"),
    }
}

/// Valid SYS operand string plus decoded fields and Rt.
fn valid_case() -> impl Strategy<Value = (u32, u32, u32, u32, u32, String, String)> {
    (
        0u32..=7,
        0u32..=15,
        0u32..=15,
        0u32..=7,
        xt_choice(),
        hash_prefix(),
        hash_prefix(),
        ws_pad(),
    )
        .prop_flat_map(|(op1, crn, crm, op2, xt, h1, h2, pre)| {
            let cn = format!("c{crn}");
            let cm = format!("c{crm}");
            let xt_owned = xt.clone();
            let xt_s = xt.clone().unwrap_or_default();
            let xt_cased = if xt.is_some() {
                ascii_case_variant(&xt_s).boxed()
            } else {
                Just(String::new()).boxed()
            };
            (
                ascii_case_variant(&cn),
                ascii_case_variant(&cm),
                xt_cased,
                ws_pad(),
                ws_pad(),
                Just(pre),
            )
                .prop_map(move |(cn, cm, xt_c, mid, post, pre)| {
                    let rt = match &xt_owned {
                        Some(name) => xt_num(name),
                        None => 31,
                    };
                    let canon = canonical_raw(op1, crn, crm, op2, xt_owned.as_deref());
                    let raw = if xt_owned.is_some() {
                        format!(
                            "{pre}{h1}{op1},{mid}{cn},{mid}{cm},{mid}{h2}{op2},{mid}{xt_c}{post}"
                        )
                    } else {
                        format!("{pre}{h1}{op1},{mid}{cn},{mid}{cm},{mid}{h2}{op2}{post}")
                    };
                    (op1, crn, crm, op2, rt, canon, raw)
                })
        })
}

fn too_few_raw() -> impl Strategy<Value = String> {
    let token = prop_oneof![
        Just("#0".to_string()),
        Just("c0".to_string()),
        Just("c7".to_string()),
        Just("x0".to_string()),
        Just("foo".to_string()),
        Just("#3".to_string()),
        Just("c15".to_string()),
    ];
    prop_oneof![
        Just("".to_string()),
        token.clone(),
        (token.clone(), token.clone()).prop_map(|(a, b)| format!("{a}, {b}")),
        (token.clone(), token.clone(), token).prop_map(|(a, b, c)| format!("{a}, {b}, {c}")),
    ]
}

fn oob_raw() -> impl Strategy<Value = String> {
    prop_oneof![
        (8u32..=255).prop_map(|op1| format!("#{op1}, c0, c0, #0, x0")),
        (16u32..=255).prop_map(|crn| format!("#0, c{crn}, c0, #0, x0")),
        (16u32..=255).prop_map(|crm| format!("#0, c0, c{crm}, #0, x0")),
        (8u32..=255).prop_map(|op2| format!("#0, c0, c0, #{op2}, x0")),
        Just("#8, c0, c0, #0, x0".to_string()),
        Just("#0, c16, c0, #0, x0".to_string()),
        Just("#0, c0, c16, #0, x0".to_string()),
        Just("#0, c0, c0, #8, x0".to_string()),
        Just("#7, c15, c15, #8, x0".to_string()),
        Just("#8, c15, c15, #7, x0".to_string()),
    ]
}

fn extra_token() -> impl Strategy<Value = String> {
    prop_oneof![
        (0u32..=31).prop_map(|n| format!("x{n}")),
        Just("#0".to_string()),
        Just("foo".to_string()),
        Just("xzr".to_string()),
        Just("c0".to_string()),
        Just("#1".to_string()),
    ]
}

fn invalid_xt() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("x32".to_string()),
        Just("x33".to_string()),
        Just("foo".to_string()),
        Just("".to_string()),
        Just("x".to_string()),
        Just("#0".to_string()),
        Just("31".to_string()),
        (32u32..=99).prop_map(|n| format!("x{n}")),
    ]
}

fn non_numeric_raw() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("foo, c0, c0, #0, x0".to_string()),
        Just("#0, xx, c0, #0, x0".to_string()),
        Just("#0, c0, yy, #0, x0".to_string()),
        Just("#0, c0, c0, bar, x0".to_string()),
        Just("#0, c, c0, #0, x0".to_string()),
        Just("#0, c0, c, #0, x0".to_string()),
        Just("#, c0, c0, #0, x0".to_string()),
        Just("#0, c0, c0, #, x0".to_string()),
    ]
}

fn wrong_reg() -> impl Strategy<Value = String> {
    prop_oneof![
        (0u32..=31).prop_map(|n| format!("w{n}")),
        Just("wzr".to_string()),
        Just("wsp".to_string()),
        Just("sp".to_string()),
        (0u32..=31).prop_map(|n| format!("d{n}")),
        (0u32..=31).prop_map(|n| format!("s{n}")),
        (0u32..=31).prop_map(|n| format!("q{n}")),
        (0u32..=31).prop_map(|n| format!("v{n}")),
        (0u32..=31).prop_map(|n| format!("h{n}")),
        (0u32..=31).prop_map(|n| format!("b{n}")),
    ]
}

#[test]
fn encode_sys_kat_llvm_mc_x0_c0() {
    let mc = llvm_mc_word("sys #0, c0, c0, #0, x0").expect("llvm-mc sys #0, c0, c0, #0, x0");
    assert_eq!(mc, 0xd5080000, "llvm-mc KAT mapping broken for sys #0,c0,c0,#0,x0");
    assert_eq!(mc, arm_sys(0, 0, 0, 0, 0));
    let sut = sut_word("#0, c0, c0, #0, x0").expect("SUT KAT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_sys_kat_llvm_mc_civac_alias() {
    let mc = llvm_mc_word("sys #3, c7, c14, #1, x0").expect("llvm-mc sys civac-alias");
    assert_eq!(mc, 0xd50b7e20, "llvm-mc KAT mapping broken for sys #3,c7,c14,#1,x0");
    assert_eq!(mc, arm_sys(3, 7, 14, 1, 0));
    let sut = sut_word("#3, c7, c14, #1, x0").expect("SUT KAT civac-alias");
    assert_eq!(sut, mc);
}

#[test]
fn encode_sys_kat_llvm_mc_ialluis_alias() {
    let mc = llvm_mc_word("sys #0, c7, c1, #0").expect("llvm-mc sys ialluis-alias");
    assert_eq!(mc, 0xd508711f, "llvm-mc KAT mapping broken for sys #0,c7,c1,#0");
    assert_eq!(mc, arm_sys(0, 7, 1, 0, 31));
    let sut = sut_word("#0, c7, c1, #0").expect("SUT KAT omitted Xt");
    assert_eq!(sut, mc);
}

#[test]
fn encode_sys_kat_llvm_mc_max_fields() {
    let mc = llvm_mc_word("sys #7, c15, c15, #7, x0").expect("llvm-mc sys max");
    assert_eq!(mc, 0xd50fffe0, "llvm-mc KAT mapping broken for sys #7,c15,c15,#7,x0");
    assert_eq!(mc, arm_sys(7, 15, 15, 7, 0));
    let sut = sut_word("#7, c15, c15, #7, x0").expect("SUT KAT max");
    assert_eq!(sut, mc);
}

#[test]
fn encode_sys_kat_llvm_mc_xzr() {
    let mc = llvm_mc_word("sys #3, c7, c14, #1, xzr").expect("llvm-mc sys xzr");
    assert_eq!(mc, 0xd50b7e3f, "llvm-mc KAT mapping broken for sys xzr");
    assert_eq!(mc, arm_sys(3, 7, 14, 1, 31));
    let sut = sut_word("#3, c7, c14, #1, xzr").expect("SUT KAT xzr");
    assert_eq!(sut, mc);
}

#[test]
fn test_encode_sys_regression_fp_alias() {
    let mc = llvm_mc_word("sys #0, c0, c0, #0, fp").expect("llvm-mc accepts fp as x29");
    assert_eq!(mc, arm_sys(0, 0, 0, 0, 29));
    let sut = sut_word("#0, c0, c0, #0, fp").expect(
        "SYS Xt alias fp must encode as x29 (llvm-mc/gas accept fp)",
    );
    assert_eq!(sut, mc);
}

#[test]
fn test_encode_sys_regression_extra_operand() {
    encode_sys("#0, c0, c0, #0, x0, x0").expect_err(
        "SYS must reject a sixth operand (llvm-mc: invalid operand / gas: unexpected characters)",
    );
}

#[test]
fn test_encode_sys_regression_oob_op1() {
    encode_sys("#8, c0, c0, #0, x0").expect_err(
        "SYS op1=8 is out of [0,7] (llvm-mc/gas reject; must not mask to 0)",
    );
}

#[test]
fn test_encode_sys_regression_w0() {
    encode_sys("#0, c0, c0, #0, w0").expect_err(
        "SYS Xt must be a 64-bit GPR (llvm-mc: invalid operand / gas: operand mismatch)",
    );
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_sys_diff_valid(case in valid_case()) {
        let (op1, crn, crm, op2, rt, canon, raw) = case;
        let asm = llvm_asm(&raw);
        let mc = llvm_mc_word(&asm).expect(&asm);
        let expect = arm_sys(op1, crn, crm, op2, rt);
        prop_assert_eq!(mc, expect, "llvm-mc vs ARM ARM for {}", asm);
        let sut = sut_word(&raw).unwrap_or_else(|e| panic!("SUT vs llvm-mc for {asm}: {e}"));
        prop_assert_eq!(sut, mc, "SUT vs llvm-mc for {} (canon {})", asm, canon);
    }

    #[test]
    fn encode_sys_inv_arm_layout(op1 in 0u32..=7, crn in 0u32..=15, crm in 0u32..=15, op2 in 0u32..=7, t in 0u32..=31) {
        let raw = canonical_raw(op1, crn, crm, op2, Some(&format!("x{t}")));
        let w = sut_word(&raw).expect("valid SYS must encode");
        prop_assert_eq!(w, arm_sys(op1, crn, crm, op2, t), "ARM SYS word for {:?}", raw);
        prop_assert_eq!(w >> 21, SYS_HI, "bits[31:21] SYS");
        prop_assert_eq!((w >> 16) & 7, op1, "op1");
        prop_assert_eq!((w >> 12) & 0xf, crn, "CRn");
        prop_assert_eq!((w >> 8) & 0xf, crm, "CRm");
        prop_assert_eq!((w >> 5) & 7, op2, "op2");
        prop_assert_eq!(w & 0x1f, t, "Rt");
    }

    #[test]
    fn encode_sys_meta_rt_isolation(op1 in 0u32..=7, crn in 0u32..=15, crm in 0u32..=15, op2 in 0u32..=7, t in 0u32..=31) {
        let raw0 = canonical_raw(op1, crn, crm, op2, Some("x0"));
        let rawt = canonical_raw(op1, crn, crm, op2, Some(&format!("x{t}")));
        let raw_omit = canonical_raw(op1, crn, crm, op2, None);
        let raw_xzr = canonical_raw(op1, crn, crm, op2, Some("xzr"));
        let raw_x31 = canonical_raw(op1, crn, crm, op2, Some("x31"));
        let w0 = sut_word(&raw0).expect("x0");
        let wt = sut_word(&rawt).expect("xt");
        let wo = sut_word(&raw_omit).expect("omitted");
        let wz = sut_word(&raw_xzr).expect("xzr");
        let w31 = sut_word(&raw_x31).expect("x31");
        prop_assert_eq!(wt ^ w0, t, "encodings must differ only in Rt");
        prop_assert_eq!(wt & !0x1f, w0 & !0x1f, "high bits independent of Rt");
        prop_assert_eq!(wo, wz, "omitted Xt must encode as xzr");
        prop_assert_eq!(wz, w31, "xzr must encode as x31");
    }

    #[test]
    fn encode_sys_meta_case_ws(case in valid_case()) {
        let (_op1, _crn, _crm, _op2, _rt, canon, raw) = case;
        match (sut_word(&raw), sut_word(&canon)) {
            (Ok(w), Ok(w0)) => {
                prop_assert_eq!(w, w0, "case/whitespace must be behavior-preserving for {:?}", raw);
            }
            (Err(_), Err(_)) => {}
            (a, b) => {
                prop_assert!(
                    false,
                    "case/whitespace changed success vs error for {:?}: {:?} vs {:?}",
                    raw,
                    a,
                    b
                );
            }
        }
    }

    #[test]
    fn encode_sys_neg_too_few(raw in too_few_raw()) {
        let asm = llvm_asm(&raw);
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let _err = encode_sys(&raw).expect_err(&format!(
            "SYS with fewer than 4 operands must Err (llvm-mc rejects {}); SUT raw {:?}",
            asm, raw
        ));
    }

    #[test]
    fn encode_sys_neg_oob_fields(raw in oob_raw()) {
        let asm = llvm_asm(&raw);
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let _err = encode_sys(&raw).expect_err(&format!(
            "SYS with out-of-range op1/Cn/Cm/op2 must Err (llvm-mc rejects {}); SUT raw {:?}",
            asm, raw
        ));
    }

    #[test]
    fn encode_sys_neg_extra_operand(
        op1 in 0u32..=7,
        crn in 0u32..=15,
        crm in 0u32..=15,
        op2 in 0u32..=7,
        t in 0u32..=31,
        extra in extra_token()
    ) {
        let raw = format!("#{op1}, c{crn}, c{crm}, #{op2}, x{t}, {extra}");
        let asm = llvm_asm(&raw);
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let _err = encode_sys(&raw).expect_err(&format!(
            "SYS with extra operand must Err (llvm-mc rejects {}); SUT raw {:?}",
            asm, raw
        ));
    }

    #[test]
    fn encode_sys_neg_wrong_reg_class(
        op1 in 0u32..=7,
        crn in 0u32..=15,
        crm in 0u32..=15,
        op2 in 0u32..=7,
        bad in wrong_reg()
    ) {
        let raw = format!("#{op1}, c{crn}, c{crm}, #{op2}, {bad}");
        let asm = llvm_asm(&raw);
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let _err = encode_sys(&raw).expect_err(&format!(
            "SYS with non-X register must Err (llvm-mc rejects {}); SUT raw {:?}",
            asm, raw
        ));
    }

    #[test]
    fn encode_sys_neg_invalid_reg(xt in invalid_xt()) {
        let raw = format!("#0, c0, c0, #0, {xt}");
        let asm = llvm_asm(&raw);
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let err = encode_sys(&raw).expect_err(&format!(
            "malformed SYS Xt must Err (llvm-mc rejects {}); SUT raw {:?}",
            asm, raw
        ));
        prop_assert!(
            err.contains("invalid register"),
            "error must name invalid register, got {err:?} for {raw:?}"
        );
    }

    #[test]
    fn encode_sys_neg_non_numeric(raw in non_numeric_raw()) {
        let asm = llvm_asm(&raw);
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let err = encode_sys(&raw).expect_err(&format!(
            "non-numeric SYS field must Err (llvm-mc rejects {}); SUT raw {:?}",
            asm, raw
        ));
        prop_assert!(
            err.contains("invalid op1")
                || err.contains("invalid CRn")
                || err.contains("invalid CRm")
                || err.contains("invalid op2"),
            "error must name the bad field, got {err:?} for {raw:?}"
        );
    }
}
