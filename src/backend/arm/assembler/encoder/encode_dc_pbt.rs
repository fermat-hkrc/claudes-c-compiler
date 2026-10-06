// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md:239 System table lists dc;
//   encoder/mod.rs:3 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:983 "dc" => encode_dc(operands, raw_operands);
//   ARM ARM DC is SYS (Xt required, 64-bit GPR / XZR / LR):
//     CIVAC = SYS #3, C7, C14, #1, Xt = 0xD50B7E20 | Rt;
//     CVAC  = SYS #3, C7, C10, #1, Xt = 0xD50B7A20 | Rt;
//     CVAP  = SYS #3, C7, C12, #1, Xt = 0xD50B7C20 | Rt (FEAT_CCPP);
//     CVAU  = SYS #3, C7, C11, #1, Xt = 0xD50B7B20 | Rt;
//     IVAC  = SYS #0, C7, C6,  #1, Xt = 0xD5087620 | Rt;
//     ZVA   = SYS #3, C7, C4,  #1, Xt = 0xD50B7420 | Rt;
//   llvm-mc -triple=aarch64 -show-encoding (LLVM 15.0.6), -mattr=+ccpp for CVAP;
//   gas: "comma expected between operands at operand 2",
//        "unexpected characters following instruction",
//        "operand mismatch" / "must be an integer register".
// Stronger considered:
//   - State machine: rejected — encode_dc is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree DC decoder
//   - Differential vs encode_ic / encode_tlbi / encode_at / encode_sys: rejected —
//     same-job gate (different SYS encodings and operand grammars)
// Weaker available: algebraic.metamorphic (Rt isolation; case/whitespace),
//   algebraic.invariant (ARM SYS layout),
//   negative_error (unknown op / missing Xt / extra operand / wrong class)
// Differential: candidate=encode_dc, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=(Symbol(op), Reg(xt), raw) <-> `dc <op>, <Xt>`

use super::encode_dc;
use super::EncodeResult;
use crate::backend::arm::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;

/// ARM ARM SYS template with L=0 (bit 21). Independent of the SUT match table.
const SYS_TEMPLATE: u32 = 0xd5080000;
const SYS_HI: u32 = 0b11010101000; // bits[31:21]

const DC_OPS: &[&str] = &["civac", "cvac", "cvap", "cvau", "ivac", "zva"];

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

fn arm_dc_word(op: &str, rt: u32) -> u32 {
    match op {
        "civac" => arm_sys(3, 7, 14, 1, rt),
        "cvac" => arm_sys(3, 7, 10, 1, rt),
        "cvap" => arm_sys(3, 7, 12, 1, rt),
        "cvau" => arm_sys(3, 7, 11, 1, rt),
        "ivac" => arm_sys(0, 7, 6, 1, rt),
        "zva" => arm_sys(3, 7, 4, 1, rt),
        other => panic!("not an ARM DC op: {other}"),
    }
}

fn dc_ops(op: &str, xt: &str) -> (Vec<Operand>, String) {
    let raw = format!("{op}, {xt}");
    (
        vec![
            Operand::Symbol(op.to_string()),
            Operand::Reg(xt.to_string()),
        ],
        raw,
    )
}

fn sut_word(operands: &[Operand], raw: &str) -> Result<u32, String> {
    match encode_dc(operands, raw)? {
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

fn llvm_mc_word(asm: &str, ccpp: bool) -> Result<u32, String> {
    let mut args = vec!["-triple=aarch64", "-show-encoding"];
    if ccpp {
        args.push("-mattr=+ccpp");
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
        "dc".to_string()
    } else {
        format!("dc {raw}")
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

fn dc_op() -> impl Strategy<Value = String> {
    prop::sample::select(DC_OPS.iter().map(|s| s.to_string()).collect::<Vec<_>>())
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

fn valid_pair() -> impl Strategy<Value = (String, String, String)> {
    (dc_op(), xt_token(), ws_pad(), ws_pad(), ws_pad(), ws_pad()).prop_flat_map(
        |(op, xt, p1, p2, p3, p4)| {
            let op_s = ascii_case_variant(&op);
            let reg_s = ascii_case_variant(&xt);
            (Just(op), Just(xt), op_s, reg_s, Just(p1), Just(p2), Just(p3), Just(p4)).prop_map(
                |(canon_op, canon_xt, op, xt, p1, p2, p3, p4)| {
                    (canon_op, canon_xt, format!("{p1}{op}{p2},{p3}{xt}{p4}"))
                },
            )
        },
    )
}

fn is_exact_dc_op(s: &str) -> bool {
    matches!(
        s.trim().to_ascii_lowercase().as_str(),
        "civac" | "cvac" | "cvap" | "cvau" | "ivac" | "zva"
    )
}

/// ARM DC ops llvm-mc accepts on the default CPU that this SUT does not implement.
/// Excluded from the unknown-op generator so the llvm-mc-rejects premise holds.
fn is_default_llvm_extra_dc(s: &str) -> bool {
    matches!(
        s.trim().to_ascii_lowercase().as_str(),
        "cisw" | "csw" | "isw"
    )
}

fn unknown_op() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("".to_string()),
        Just("foo".to_string()),
        Just("gzva".to_string()),
        Just("gva".to_string()),
        Just("civacs".to_string()),
        Just("mycvac".to_string()),
        Just("zvafoo".to_string()),
        Just("ivacs".to_string()),
        Just("cvacx".to_string()),
        Just("cvapx".to_string()),
        Just("cvaux".to_string()),
        Just("iallu".to_string()),
        Just("ialluis".to_string()),
        Just("ivau".to_string()),
        Just("sys".to_string()),
        Just("nop".to_string()),
        Just("tlbi".to_string()),
        Just("#0".to_string()),
        Just("x0".to_string()),
        Just("xzr".to_string()),
        Just("ish".to_string()),
        Just("sy".to_string()),
        Just(",".to_string()),
        Just(", x0".to_string()),
        Just(".".to_string()),
        Just("civac x0".to_string()),
        Just("cvadp".to_string()),
        Just("cigvac".to_string()),
        Just("cgvac".to_string()),
        prop::string::string_regex("[A-Za-z]{1,12}").unwrap(),
        prop::string::string_regex("[A-Za-z0-9_#]{1,16}").unwrap(),
    ]
    .prop_filter(
        "not an exact implemented DC op or default-llvm extra DC op",
        |s| {
            let op = s
                .split(',')
                .next()
                .unwrap_or("")
                .trim()
                .to_ascii_lowercase();
            !is_exact_dc_op(&op) && !is_default_llvm_extra_dc(&op)
        },
    )
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

fn contains_implemented_dc_substr(s: &str) -> bool {
    let t = s.trim().to_ascii_lowercase();
    t.contains("civac")
        || t.contains("cvac")
        || t.contains("cvap")
        || t.contains("cvau")
        || t.contains("ivac")
        || t.contains("zva")
}

fn unknown_op_no_substr() -> impl Strategy<Value = String> {
    unknown_op().prop_filter("must not substring-match an implemented DC op", |s| {
        !contains_implemented_dc_substr(s)
    })
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

fn extra_token() -> impl Strategy<Value = String> {
    prop_oneof![
        xt_token(),
        wrong_reg(),
        Just("fp".to_string()),
        Just("#0".to_string()),
        Just("x32".to_string()),
    ]
}

fn xt_from_raw(raw: &str) -> String {
    raw.splitn(2, ',')
        .nth(1)
        .unwrap_or("")
        .trim()
        .to_string()
}

fn op_from_raw(raw: &str) -> String {
    raw.splitn(2, ',')
        .next()
        .unwrap_or("")
        .trim()
        .to_string()
}

#[test]
fn encode_dc_kat_llvm_mc_civac_x0() {
    let mc = llvm_mc_word("dc civac, x0", true).expect("llvm-mc dc civac, x0");
    assert_eq!(mc, 0xd50b7e20, "llvm-mc KAT mapping broken for dc civac, x0");
    assert_eq!(mc, arm_dc_word("civac", 0));
    let (ops, raw) = dc_ops("civac", "x0");
    let sut = sut_word(&ops, &raw).expect("SUT KAT civac x0");
    assert_eq!(sut, mc);
}

#[test]
fn encode_dc_kat_llvm_mc_cvac_x0() {
    let mc = llvm_mc_word("dc cvac, x0", true).expect("llvm-mc dc cvac, x0");
    assert_eq!(mc, 0xd50b7a20, "llvm-mc KAT mapping broken for dc cvac, x0");
    assert_eq!(mc, arm_dc_word("cvac", 0));
    let (ops, raw) = dc_ops("cvac", "x0");
    let sut = sut_word(&ops, &raw).expect("SUT KAT cvac x0");
    assert_eq!(sut, mc);
}

#[test]
fn encode_dc_kat_llvm_mc_cvap_x0() {
    let mc = llvm_mc_word("dc cvap, x0", true).expect("llvm-mc dc cvap, x0");
    assert_eq!(mc, 0xd50b7c20, "llvm-mc KAT mapping broken for dc cvap, x0");
    assert_eq!(mc, arm_dc_word("cvap", 0));
    let (ops, raw) = dc_ops("cvap", "x0");
    let sut = sut_word(&ops, &raw).expect("SUT KAT cvap x0");
    assert_eq!(sut, mc);
}

#[test]
fn encode_dc_kat_llvm_mc_cvau_x0() {
    let mc = llvm_mc_word("dc cvau, x0", true).expect("llvm-mc dc cvau, x0");
    assert_eq!(mc, 0xd50b7b20, "llvm-mc KAT mapping broken for dc cvau, x0");
    assert_eq!(mc, arm_dc_word("cvau", 0));
    let (ops, raw) = dc_ops("cvau", "x0");
    let sut = sut_word(&ops, &raw).expect("SUT KAT cvau x0");
    assert_eq!(sut, mc);
}

#[test]
fn encode_dc_kat_llvm_mc_ivac_x0() {
    let mc = llvm_mc_word("dc ivac, x0", true).expect("llvm-mc dc ivac, x0");
    assert_eq!(mc, 0xd5087620, "llvm-mc KAT mapping broken for dc ivac, x0");
    assert_eq!(mc, arm_dc_word("ivac", 0));
    let (ops, raw) = dc_ops("ivac", "x0");
    let sut = sut_word(&ops, &raw).expect("SUT KAT ivac x0");
    assert_eq!(sut, mc);
}

#[test]
fn encode_dc_kat_llvm_mc_zva_x0() {
    let mc = llvm_mc_word("dc zva, x0", true).expect("llvm-mc dc zva, x0");
    assert_eq!(mc, 0xd50b7420, "llvm-mc KAT mapping broken for dc zva, x0");
    assert_eq!(mc, arm_dc_word("zva", 0));
    let (ops, raw) = dc_ops("zva", "x0");
    let sut = sut_word(&ops, &raw).expect("SUT KAT zva x0");
    assert_eq!(sut, mc);
}

#[test]
fn encode_dc_kat_llvm_mc_civac_xzr() {
    let mc = llvm_mc_word("dc civac, xzr", true).expect("llvm-mc dc civac, xzr");
    assert_eq!(mc, 0xd50b7e3f, "llvm-mc KAT mapping broken for dc civac, xzr");
    assert_eq!(mc, arm_dc_word("civac", 31));
    let (ops, raw) = dc_ops("civac", "xzr");
    let sut = sut_word(&ops, &raw).expect("SUT KAT civac xzr");
    assert_eq!(sut, mc);
}

#[test]
fn test_encode_dc_regression_missing_xt() {
    encode_dc(&[Operand::Symbol("civac".into())], "civac").expect_err(
        "DC CIVAC requires Xt (llvm-mc/gas reject a missing register)",
    );
}

#[test]
fn test_encode_dc_regression_extra_operand() {
    encode_dc(
        &[
            Operand::Symbol("civac".into()),
            Operand::Reg("x0".into()),
            Operand::Reg("x1".into()),
        ],
        "civac, x0, x1",
    )
    .expect_err(
        "DC must reject a third operand (llvm-mc: unexpected characters following instruction)",
    );
}

#[test]
fn test_encode_dc_regression_w0() {
    encode_dc(
        &[
            Operand::Symbol("civac".into()),
            Operand::Reg("w0".into()),
        ],
        "civac, w0",
    )
    .expect_err(
        "DC Xt must be a 64-bit GPR (llvm-mc: invalid operand / gas: operand mismatch)",
    );
}

#[test]
fn test_encode_dc_regression_sp() {
    encode_dc(
        &[
            Operand::Symbol("civac".into()),
            Operand::Reg("sp".into()),
        ],
        "civac, sp",
    )
    .expect_err("DC Xt must not be SP (gas: operand 2 must be an integer register)");
}

#[test]
fn test_encode_dc_regression_gzva() {
    encode_dc(
        &[
            Operand::Symbol("gzva".into()),
            Operand::Reg("x0".into()),
        ],
        "gzva, x0",
    )
    .expect_err(
        "DC GZVA is not ZVA (llvm-mc/gas reject gzva without MTE; substring must not match)",
    );
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_dc_diff_valid(pair in valid_pair()) {
        let (canon_op, canon_xt, raw) = pair;
        let op_cased = op_from_raw(&raw);
        let xt_cased = xt_from_raw(&raw);
        let ops = vec![
            Operand::Symbol(op_cased),
            Operand::Reg(xt_cased),
        ];
        let asm = llvm_asm(&raw);
        let mc = llvm_mc_word(&asm, true).expect(&asm);
        let rt = xt_num(&canon_xt);
        let expect = arm_dc_word(&canon_op, rt);
        prop_assert_eq!(mc, expect, "llvm-mc vs ARM ARM for {}", asm);
        let sut = sut_word(&ops, &raw).expect(&asm);
        prop_assert_eq!(sut, mc, "SUT vs llvm-mc for {}", asm);
    }

    #[test]
    fn encode_dc_inv_arm_layout(op in dc_op(), t in 0u32..=31) {
        let xt = format!("x{t}");
        let (ops, raw) = dc_ops(&op, &xt);
        let w = sut_word(&ops, &raw).expect("valid DC must encode");
        prop_assert_eq!(w, arm_dc_word(&op, t), "ARM SYS word for {:?}", raw);
        prop_assert_eq!(w >> 21, SYS_HI, "bits[31:21] SYS");
        let (op1, crm) = match op.as_str() {
            "civac" => (3u32, 14u32),
            "cvac" => (3, 10),
            "cvap" => (3, 12),
            "cvau" => (3, 11),
            "ivac" => (0, 6),
            "zva" => (3, 4),
            _ => unreachable!(),
        };
        prop_assert_eq!((w >> 16) & 7, op1, "op1");
        prop_assert_eq!((w >> 12) & 0xf, 7, "CRn=7");
        prop_assert_eq!((w >> 8) & 0xf, crm, "CRm");
        prop_assert_eq!((w >> 5) & 7, 1, "op2=1");
        prop_assert_eq!(w & 0x1f, t, "Rt");
    }

    #[test]
    fn encode_dc_meta_rt_isolation(op in dc_op(), t in 0u32..=31) {
        let (ops0, raw0) = dc_ops(&op, "x0");
        let (opst, rawt) = dc_ops(&op, &format!("x{t}"));
        let w0 = sut_word(&ops0, &raw0).expect("x0");
        let wt = sut_word(&opst, &rawt).expect("xt");
        prop_assert_eq!(wt ^ w0, t, "encodings must differ only in Rt");
        prop_assert_eq!(wt & !0x1f, w0 & !0x1f, "high bits independent of Rt");
        let (civac, _) = dc_ops("civac", "x0");
        let (cvac, _) = dc_ops("cvac", "x0");
        let w_ci = sut_word(&civac, "civac, x0").expect("civac");
        let w_c = sut_word(&cvac, "cvac, x0").expect("cvac");
        prop_assert_ne!(w_ci, w_c, "CIVAC and CVAC must encode distinctly");
    }

    #[test]
    fn encode_dc_meta_case_ws(pair in valid_pair()) {
        let (canon_op, canon_xt, raw) = pair;
        let op_cased = op_from_raw(&raw);
        let xt_cased = xt_from_raw(&raw);
        let ops = vec![
            Operand::Symbol(op_cased),
            Operand::Reg(xt_cased),
        ];
        let (ops0, raw0) = dc_ops(&canon_op, &canon_xt);
        let w = sut_word(&ops, &raw).expect("case/ws variant must encode");
        let w0 = sut_word(&ops0, &raw0).expect("canonical must encode");
        prop_assert_eq!(w, w0, "case/whitespace must be behavior-preserving for {:?}", raw);
    }

    #[test]
    fn encode_dc_neg_unknown_op(s in unknown_op()) {
        let raw = if s.trim().is_empty() {
            s.clone()
        } else {
            format!("{s}, x0")
        };
        let asm = llvm_asm(&raw);
        prop_assert!(
            llvm_mc_word(&asm, false).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let ops = vec![
            Operand::Symbol(op_from_raw(&raw)),
            Operand::Reg("x0".into()),
        ];
        let err = encode_dc(&ops, &raw).expect_err(&format!(
            "unknown DC op {:?} must Err (llvm-mc rejects {})",
            s, asm
        ));
        prop_assert!(
            err.contains("unsupported dc variant") || err.contains("invalid register"),
            "error must name unsupported dc variant or invalid register, got {err:?} for {s:?}"
        );
    }

    #[test]
    fn encode_dc_neg_missing_xt(op in dc_op()) {
        let raw = op.clone();
        let asm = llvm_asm(&raw);
        prop_assert!(
            llvm_mc_word(&asm, true).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let ops = vec![Operand::Symbol(op.clone())];
        let _err = encode_dc(&ops, &raw).expect_err(&format!(
            "DC {} without Xt must Err (llvm-mc rejects {}); SUT raw {:?}",
            op, asm, raw
        ));
    }

    #[test]
    fn encode_dc_neg_extra_operand(op in dc_op(), xt in xt_token(), extra in extra_token()) {
        let raw = format!("{op}, {xt}, {extra}");
        let asm = llvm_asm(&raw);
        prop_assert!(
            llvm_mc_word(&asm, true).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let ops = vec![
            Operand::Symbol(op),
            Operand::Reg(xt),
            Operand::Reg(extra),
        ];
        let _err = encode_dc(&ops, &raw).expect_err(&format!(
            "DC with extra operand must Err (llvm-mc rejects {}); SUT raw {:?}",
            asm, raw
        ));
    }

    #[test]
    fn encode_dc_neg_wrong_reg_class(op in dc_op(), bad in wrong_reg()) {
        let raw = format!("{op}, {bad}");
        let asm = llvm_asm(&raw);
        prop_assert!(
            llvm_mc_word(&asm, true).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let ops = vec![
            Operand::Symbol(op),
            Operand::Reg(bad),
        ];
        let _err = encode_dc(&ops, &raw).expect_err(&format!(
            "DC with non-X register must Err (llvm-mc rejects {}); SUT raw {:?}",
            asm, raw
        ));
    }

    #[test]
    fn encode_dc_neg_invalid_reg(op in dc_op(), xt in invalid_xt()) {
        let raw = format!("{op}, {xt}");
        let asm = llvm_asm(&raw);
        prop_assert!(
            llvm_mc_word(&asm, true).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let err = encode_dc(
            &[
                Operand::Symbol(op),
                Operand::Reg(xt.clone()),
            ],
            &raw,
        )
        .expect_err(&format!(
            "malformed DC Xt must Err (llvm-mc rejects {}); SUT raw {:?}",
            asm, raw
        ));
        prop_assert!(
            err.contains("invalid register") || err.contains("unsupported dc variant"),
            "error must name invalid register, got {err:?} for {raw:?}"
        );
    }

    #[test]
    fn encode_dc_neg_unknown_nonsubstr(s in unknown_op_no_substr()) {
        let raw = if s.trim().is_empty() {
            s.clone()
        } else {
            format!("{s}, x0")
        };
        let asm = llvm_asm(&raw);
        prop_assert!(
            llvm_mc_word(&asm, false).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let ops = vec![
            Operand::Symbol(op_from_raw(&raw)),
            Operand::Reg("x0".into()),
        ];
        let err = encode_dc(&ops, &raw).expect_err(&format!(
            "unknown DC op without substring must Err (llvm-mc rejects {})",
            asm
        ));
        prop_assert!(
            err.contains("unsupported dc variant") || err.contains("invalid register"),
            "error must name unsupported dc variant or invalid register, got {err:?} for {s:?}"
        );
    }
}
