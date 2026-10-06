// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md:239 System table lists tlbi;
//   encoder/mod.rs:3 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:990 "tlbi" => encode_tlbi(operands, raw_operands);
//   ARM ARM TLBI is SYS with CRn=8:
//     VMALLE1IS = SYS #0, C8, C3, #0 (no Xt);
//     VAE1IS    = SYS #0, C8, C3, #1, Xt;
//     VALE1IS   = SYS #0, C8, C3, #5, Xt;
//     ALLE2IS   = SYS #4, C8, C3, #0 (no Xt);
//     RVAE1IS   = SYS #0, C8, C2, #1, Xt (FEAT_TLBIRANGE);
//   llvm-mc -triple=aarch64 -show-encoding (LLVM 15.0.6);
//   range ops: -mattr=+tlb-rmi;
//   gas/llvm-mc: "specified tlbi op requires a register",
//        "specified tlbi op does not use a register",
//        "invalid operand for instruction".
// Stronger considered:
//   - State machine: rejected — encode_tlbi is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree TLBI decoder
//   - Differential vs encode_ic / encode_dc / encode_at / encode_sys: rejected —
//     same-job gate (different SYS encodings and operand grammars)
// Weaker available: algebraic.metamorphic (Rt isolation; case/whitespace),
//   algebraic.invariant (ARM SYS layout),
//   negative_error (unknown op / missing Xt / extra Xt / wrong class)
// Differential: candidate=encode_tlbi, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=raw_operands <-> `tlbi <op>` / `tlbi <op>, <Xt>`

use super::encode_tlbi;
use super::EncodeResult;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;

/// ARM ARM SYS template with L=0. Independent of the SUT match table.
const SYS_TEMPLATE: u32 = 0xd5080000;
const SYS_HI: u32 = 0b11010101000; // bits[31:21]

const NO_XT: &[&str] = &[
    "vmalle1is", "vmalle1", "alle1is", "alle1", "alle2is", "vmalls12e1is", "vmalls12e1",
];
const XT_REQUIRED: &[&str] = &[
    "vale1is", "vale1", "vale2is", "vale2", "vaae1is", "vaae1", "vaale1is", "vaale1",
    "vae1is", "vae1", "vae2is", "vae2", "aside1is", "aside1",
    "ipas2e1is", "ipas2e1", "ipas2le1is", "ipas2le1",
];
const RANGE_XT: &[&str] = &[
    "rvae1is", "rvale1is", "rvaae1is", "rvaale1is", "rvae1", "rvale1", "rvaae1", "rvaale1",
    "rvae1os", "rvale1os", "rvaae1os", "rvaale1os",
    "ripas2e1is", "ripas2e1", "ripas2e1os", "ripas2le1is", "ripas2le1", "ripas2le1os",
];
const ARM_DEFAULT_NO_XT: &[&str] = &[
    "vmalle1is", "vmalle1", "alle1is", "alle1", "alle2is", "alle2", "alle3is", "alle3",
    "vmalls12e1is", "vmalls12e1",
];
const ARM_DEFAULT_XT: &[&str] = &[
    "vale1is", "vale1", "vale2is", "vale2", "vaae1is", "vaae1", "vaale1is", "vaale1",
    "vae1is", "vae1", "vae2is", "vae2", "vae3is", "vae3", "vale3is", "vale3",
    "aside1is", "aside1", "ipas2e1is", "ipas2e1", "ipas2le1is", "ipas2le1",
];

fn cfg() -> ProptestConfig {
    ProptestConfig::with_cases(CASES)
}

fn arm_sys(op1: u32, crn: u32, crm: u32, op2: u32, rt: u32) -> u32 {
    SYS_TEMPLATE
        | ((op1 & 7) << 16)
        | ((crn & 0xf) << 12)
        | ((crm & 0xf) << 8)
        | ((op2 & 7) << 5)
        | (rt & 0x1f)
}

fn arm_fields(op: &str) -> (u32, u32, u32, bool, bool) {
    match op {
        "vmalle1is" => (0, 3, 0, false, false),
        "vmalle1" => (0, 7, 0, false, false),
        "alle1is" => (4, 3, 4, false, false),
        "alle1" => (4, 7, 4, false, false),
        "alle2is" => (4, 3, 0, false, false),
        "alle2" => (4, 7, 0, false, false),
        "alle3is" => (6, 3, 0, false, false),
        "alle3" => (6, 7, 0, false, false),
        "vmalls12e1is" => (4, 3, 6, false, false),
        "vmalls12e1" => (4, 7, 6, false, false),
        "vale1is" => (0, 3, 5, true, false),
        "vale1" => (0, 7, 5, true, false),
        "vale2is" => (4, 3, 5, true, false),
        "vale2" => (4, 7, 5, true, false),
        "vale3is" => (6, 3, 5, true, false),
        "vale3" => (6, 7, 5, true, false),
        "vaae1is" => (0, 3, 3, true, false),
        "vaae1" => (0, 7, 3, true, false),
        "vaale1is" => (0, 3, 7, true, false),
        "vaale1" => (0, 7, 7, true, false),
        "vae1is" => (0, 3, 1, true, false),
        "vae1" => (0, 7, 1, true, false),
        "vae2is" => (4, 3, 1, true, false),
        "vae2" => (4, 7, 1, true, false),
        "vae3is" => (6, 3, 1, true, false),
        "vae3" => (6, 7, 1, true, false),
        "aside1is" => (0, 3, 2, true, false),
        "aside1" => (0, 7, 2, true, false),
        "ipas2e1is" => (4, 0, 1, true, false),
        "ipas2e1" => (4, 4, 1, true, false),
        "ipas2le1is" => (4, 0, 5, true, false),
        "ipas2le1" => (4, 4, 5, true, false),
        "rvae1is" => (0, 2, 1, true, true),
        "rvale1is" => (0, 2, 5, true, true),
        "rvaae1is" => (0, 2, 3, true, true),
        "rvaale1is" => (0, 2, 7, true, true),
        "rvae1" => (0, 6, 1, true, true),
        "rvale1" => (0, 6, 5, true, true),
        "rvaae1" => (0, 6, 3, true, true),
        "rvaale1" => (0, 6, 7, true, true),
        "rvae1os" => (0, 5, 1, true, true),
        "rvale1os" => (0, 5, 5, true, true),
        "rvaae1os" => (0, 5, 3, true, true),
        "rvaale1os" => (0, 5, 7, true, true),
        "ripas2e1is" => (4, 0, 2, true, true),
        "ripas2e1" => (4, 4, 2, true, true),
        "ripas2e1os" => (4, 4, 3, true, true),
        "ripas2le1is" => (4, 0, 6, true, true),
        "ripas2le1" => (4, 4, 6, true, true),
        "ripas2le1os" => (4, 4, 7, true, true),
        other => panic!("not an ARM TLBI op: {other}"),
    }
}

fn arm_tlbi_word(op: &str, rt: u32) -> u32 {
    let (op1, crm, op2, needs_xt, _) = arm_fields(op);
    let r = if needs_xt { rt } else { 31 };
    arm_sys(op1, 8, crm, op2, r)
}

fn is_range(op: &str) -> bool {
    arm_fields(op).4
}

fn sut_word(raw: &str) -> Result<u32, String> {
    match encode_tlbi(&[], raw)? {
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

fn llvm_mc_word_attr(asm: &str, mattr: Option<&str>) -> Result<u32, String> {
    let mut args: Vec<&str> = vec!["-triple=aarch64", "-show-encoding"];
    if let Some(a) = mattr {
        args.push("-mattr");
        args.push(a);
    }
    let mut child = Command::new(LLVM_MC)
        .args(&args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("spawn llvm-mc: {e}"))?;
    {
        let mut stdin = child.stdin.take().ok_or("llvm-mc stdin")?;
        stdin.write_all(asm.as_bytes()).map_err(|e| format!("write llvm-mc: {e}"))?;
        stdin.write_all(b"\n").map_err(|e| format!("write llvm-mc: {e}"))?;
    }
    let out = child.wait_with_output().map_err(|e| format!("wait llvm-mc: {e}"))?;
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    if !out.status.success() || stderr.contains("error:") {
        return Err(format!("llvm-mc error: {stderr}"));
    }
    parse_llvm_encoding(&stdout)
}

fn llvm_mc_word(asm: &str) -> Result<u32, String> {
    llvm_mc_word_attr(asm, None)
}

fn llvm_mc_for_op(op: &str, asm: &str) -> Result<u32, String> {
    if is_range(op) {
        llvm_mc_word_attr(asm, Some("+tlb-rmi"))
    } else {
        llvm_mc_word(asm)
    }
}

fn llvm_asm(raw: &str) -> String {
    if raw.trim().is_empty() {
        "tlbi".to_string()
    } else {
        format!("tlbi {raw}")
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
fn xt_op() -> impl Strategy<Value = String> {
    prop::sample::select(XT_REQUIRED.iter().map(|s| s.to_string()).collect::<Vec<_>>())
}
fn range_op() -> impl Strategy<Value = String> {
    prop::sample::select(RANGE_XT.iter().map(|s| s.to_string()).collect::<Vec<_>>())
}
fn implemented_xt_op() -> impl Strategy<Value = String> {
    prop_oneof![xt_op(), range_op()]
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
    prop_oneof![
        (no_xt_op(), ws_pad(), ws_pad()).prop_flat_map(|(op, p1, p2)| {
            ascii_case_variant(&op).prop_map(move |op| format!("{p1}{op}{p2}"))
        }),
        (implemented_xt_op(), xt_token(), ws_pad(), ws_pad(), ws_pad(), ws_pad()).prop_flat_map(
            |(op, xt, p1, p2, p3, p4)| {
                let op_s = ascii_case_variant(&op);
                let reg_s = ascii_case_variant(&xt);
                (op_s, reg_s).prop_map(move |(op, reg)| format!("{p1}{op}{p2},{p3}{reg}{p4}"))
            },
        ),
    ]
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
fn op_of(raw: &str) -> String {
    raw.splitn(2, ',').next().unwrap_or("").trim().to_ascii_lowercase()
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

fn unknown_op() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("".to_string()),
        Just("foo".to_string()),
        Just("tlbi".to_string()),
        Just("sys".to_string()),
        Just("ialluis".to_string()),
        Just("civac".to_string()),
        Just("s1e1r".to_string()),
        Just("nop".to_string()),
        Just("#0".to_string()),
        Just("x0".to_string()),
        Just("xzr".to_string()),
        Just(",".to_string()),
        Just(", x0".to_string()),
        Just(".".to_string()),
        Just("vae1is x0".to_string()),
        prop::string::string_regex("[A-Za-z]{1,12}").unwrap(),
        prop::string::string_regex("[A-Za-z0-9_#]{1,16}").unwrap(),
    ]
    .prop_filter("not a valid TLBI op name after trim+casefold", |s| {
        let op = s.split(',').next().unwrap_or("").trim().to_ascii_lowercase();
        !matches!(
            op.as_str(),
            "vmalle1is" | "vmalle1" | "alle1is" | "alle1" | "alle2is" | "alle2"
                | "alle3is" | "alle3" | "vmalls12e1is" | "vmalls12e1"
                | "vale1is" | "vale1" | "vale2is" | "vale2" | "vale3is" | "vale3"
                | "vaae1is" | "vaae1" | "vaale1is" | "vaale1"
                | "vae1is" | "vae1" | "vae2is" | "vae2" | "vae3is" | "vae3"
                | "aside1is" | "aside1"
                | "ipas2e1is" | "ipas2e1" | "ipas2le1is" | "ipas2le1"
                | "rvae1is" | "rvale1is" | "rvaae1is" | "rvaale1is"
                | "rvae1" | "rvale1" | "rvaae1" | "rvaale1"
                | "rvae1os" | "rvale1os" | "rvaae1os" | "rvaale1os"
                | "ripas2e1is" | "ripas2e1" | "ripas2e1os"
                | "ripas2le1is" | "ripas2le1" | "ripas2le1os"
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

#[test]
fn encode_tlbi_kat_llvm_mc_vmalle1is() {
    let mc = llvm_mc_word("tlbi vmalle1is").expect("llvm-mc tlbi vmalle1is");
    assert_eq!(mc, 0xd508831f, "llvm-mc KAT mapping broken for tlbi vmalle1is");
    assert_eq!(mc, arm_tlbi_word("vmalle1is", 31));
    let sut = sut_word("vmalle1is").expect("SUT KAT vmalle1is");
    assert_eq!(sut, mc);
}
#[test]
fn encode_tlbi_kat_llvm_mc_vae1is_x0() {
    let mc = llvm_mc_word("tlbi vae1is, x0").expect("llvm-mc tlbi vae1is, x0");
    assert_eq!(mc, 0xd5088320, "llvm-mc KAT mapping broken for tlbi vae1is, x0");
    assert_eq!(mc, arm_tlbi_word("vae1is", 0));
    let sut = sut_word("vae1is, x0").expect("SUT KAT vae1is x0");
    assert_eq!(sut, mc);
}
#[test]
fn encode_tlbi_kat_llvm_mc_vale1is_x0() {
    let mc = llvm_mc_word("tlbi vale1is, x0").expect("llvm-mc tlbi vale1is, x0");
    assert_eq!(mc, 0xd50883a0, "llvm-mc KAT mapping broken for tlbi vale1is, x0");
    assert_eq!(mc, arm_tlbi_word("vale1is", 0));
    let sut = sut_word("vale1is, x0").expect("SUT KAT vale1is x0");
    assert_eq!(sut, mc);
}
#[test]
fn encode_tlbi_kat_llvm_mc_alle2is() {
    let mc = llvm_mc_word("tlbi alle2is").expect("llvm-mc tlbi alle2is");
    assert_eq!(mc, 0xd50c831f, "llvm-mc KAT mapping broken for tlbi alle2is");
    assert_eq!(mc, arm_tlbi_word("alle2is", 31));
    let sut = sut_word("alle2is").expect("SUT KAT alle2is");
    assert_eq!(sut, mc);
}
#[test]
fn encode_tlbi_kat_llvm_mc_rvae1is_x0() {
    let mc = llvm_mc_word_attr("tlbi rvae1is, x0", Some("+tlb-rmi")).expect("llvm-mc tlbi rvae1is, x0");
    assert_eq!(mc, 0xd5088220, "llvm-mc KAT mapping broken for tlbi rvae1is, x0");
    assert_eq!(mc, arm_tlbi_word("rvae1is", 0));
    let sut = sut_word("rvae1is, x0").expect("SUT KAT rvae1is x0");
    assert_eq!(sut, mc);
}
#[test]
fn encode_tlbi_kat_llvm_mc_vae1is_xzr() {
    let mc = llvm_mc_word("tlbi vae1is, xzr").expect("llvm-mc tlbi vae1is, xzr");
    assert_eq!(mc, 0xd508833f, "llvm-mc KAT mapping broken for tlbi vae1is, xzr");
    assert_eq!(mc, arm_tlbi_word("vae1is", 31));
    let sut = sut_word("vae1is, xzr").expect("SUT KAT vae1is xzr");
    assert_eq!(sut, mc);
}
#[test]
fn test_encode_tlbi_regression_missing_xt() {
    encode_tlbi(&[], "vae1is").expect_err(
        "TLBI VAE1IS requires Xt (llvm-mc: specified tlbi op requires a register)",
    );
}
#[test]
fn test_encode_tlbi_regression_extra_xt() {
    encode_tlbi(&[], "vmalle1is, x0").expect_err(
        "TLBI VMALLE1IS takes no Xt (llvm-mc: specified tlbi op does not use a register)",
    );
}
#[test]
fn test_encode_tlbi_regression_w0() {
    encode_tlbi(&[], "vae1is, w0").expect_err(
        "TLBI Xt must be a 64-bit GPR (llvm-mc: invalid operand for instruction)",
    );
}
#[test]
fn test_encode_tlbi_regression_alle2() {
    encode_tlbi(&[], "alle2").expect(
        "TLBI ALLE2 is a valid ARM op (llvm-mc/gas assemble tlbi alle2)",
    );
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_tlbi_diff_valid(raw in valid_raw()) {
        let op = op_of(&raw);
        let asm = llvm_asm(&raw);
        let mc = llvm_mc_for_op(&op, &asm).expect(&asm);
        let canon = canonical(&raw);
        let parts: Vec<&str> = canon.splitn(2, ',').collect();
        let rt = if parts.len() > 1 { xt_num(parts[1].trim()) } else { 31 };
        let expect = arm_tlbi_word(&op, rt);
        prop_assert_eq!(mc, expect, "llvm-mc vs ARM ARM for {}", asm);
        let sut = sut_word(&raw).expect(&asm);
        prop_assert_eq!(sut, mc, "SUT vs llvm-mc for {}", asm);
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_tlbi_inv_arm_layout(
        op in prop_oneof![no_xt_op(), implemented_xt_op()],
        t in 0u32..=31
    ) {
        let needs_xt = arm_fields(&op).3;
        let raw = if needs_xt {
            format!("{op}, x{t}")
        } else {
            op.clone()
        };
        let w = sut_word(&raw).expect("implemented TLBI must encode");
        let expect = arm_tlbi_word(&op, t);
        prop_assert_eq!(w, expect, "ARM SYS word for {:?}", raw);
        prop_assert_eq!(w >> 21, SYS_HI, "bits[31:21] SYS");
        let (op1, crm, op2, _, _) = arm_fields(&op);
        prop_assert_eq!((w >> 16) & 7, op1, "op1");
        prop_assert_eq!((w >> 12) & 0xf, 8, "CRn=8");
        prop_assert_eq!((w >> 8) & 0xf, crm, "CRm");
        prop_assert_eq!((w >> 5) & 7, op2, "op2");
        if needs_xt {
            prop_assert_eq!(w & 0x1f, t, "Rt");
        } else {
            prop_assert_eq!(w & 0x1f, 31, "no-Xt Rt=XZR");
        }
    }

    #[test]
    fn encode_tlbi_meta_rt_isolation(op in implemented_xt_op(), t in 0u32..=31) {
        let w0 = sut_word(&format!("{op}, x0")).expect("tlbi op x0");
        let wt = sut_word(&format!("{op}, x{t}")).expect("tlbi op xt");
        prop_assert_eq!(wt ^ w0, t, "TLBI encodings must differ only in Rt");
        prop_assert_eq!(wt & !0x1f, w0 & !0x1f, "TLBI high bits independent of Rt");
    }

    #[test]
    fn encode_tlbi_meta_case_ws(raw in valid_raw()) {
        let w = sut_word(&raw).expect("case/ws variant must encode");
        let w0 = sut_word(&canonical(&raw)).expect("canonical must encode");
        prop_assert_eq!(w, w0, "case/whitespace must be behavior-preserving for {:?}", raw);
    }

    #[test]
    fn encode_tlbi_diff_arm_ops(
        kind in 0u8..=1,
        i in 0usize..16,
        t in 0u32..=31
    ) {
        let (op, needs_xt) = if kind == 0 {
            let ops = ARM_DEFAULT_NO_XT;
            (ops[i % ops.len()].to_string(), false)
        } else {
            let ops = ARM_DEFAULT_XT;
            (ops[i % ops.len()].to_string(), true)
        };
        let raw = if needs_xt {
            format!("{op}, x{t}")
        } else {
            op.clone()
        };
        let asm = llvm_asm(&raw);
        let mc = llvm_mc_word(&asm).expect(&asm);
        let sut = sut_word(&raw).expect(&format!(
            "ARM TLBI op {:?} must encode (llvm-mc accepts {})",
            op, asm
        ));
        prop_assert_eq!(sut, mc, "SUT vs llvm-mc for {}", asm);
    }

    #[test]
    fn encode_tlbi_neg_missing_xt(raw in (xt_op(), ws_pad(), ws_pad()).prop_flat_map(|(op, pre, post)| {
        ascii_case_variant(&op).prop_map(move |op| format!("{pre}{op}{post}"))
    })) {
        let asm = llvm_asm(&raw);
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let _err = encode_tlbi(&[], &raw).expect_err(&format!(
            "Xt-required TLBI without register must Err (llvm-mc rejects {}); SUT raw {:?}",
            asm, raw
        ));
    }

    #[test]
    fn encode_tlbi_neg_extra_xt(op in no_xt_op(), xt in xt_token()) {
        let raw = format!("{op}, {xt}");
        let asm = llvm_asm(&raw);
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let _err = encode_tlbi(&[], &raw).expect_err(&format!(
            "no-Xt TLBI with register must Err (llvm-mc rejects {}); SUT raw {:?}",
            asm, raw
        ));
    }

    #[test]
    fn encode_tlbi_neg_invalid_reg(op in xt_op(), xt in invalid_xt()) {
        let raw = format!("{op}, {xt}");
        let asm = llvm_asm(&raw);
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let err = encode_tlbi(&[], &raw).expect_err(&format!(
            "malformed TLBI Xt must Err (llvm-mc rejects {}); SUT raw {:?}",
            asm, raw
        ));
        prop_assert!(
            err.contains("invalid register") || err.contains("unsupported tlbi operation"),
            "error must name invalid register, got {err:?} for {raw:?}"
        );
    }

    #[test]
    fn encode_tlbi_neg_unknown_op(s in unknown_op()) {
        let asm = llvm_asm(&s);
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let err = encode_tlbi(&[], &s).expect_err(&format!(
            "unknown TLBI op {:?} must Err (llvm-mc rejects {})",
            s, asm
        ));
        prop_assert!(
            err.contains("unsupported tlbi operation") || err.contains("invalid register"),
            "error must name unsupported tlbi operation or invalid register, got {err:?} for {s:?}"
        );
    }

    #[test]
    fn encode_tlbi_neg_wrong_reg_class(op in xt_op(), bad in wrong_reg()) {
        let raw = format!("{op}, {bad}");
        let asm = llvm_asm(&raw);
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let _err = encode_tlbi(&[], &raw).expect_err(&format!(
            "TLBI with non-X register must Err (llvm-mc rejects {}); SUT raw {:?}",
            asm, raw
        ));
    }
}
