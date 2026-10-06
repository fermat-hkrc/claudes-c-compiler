// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md:239 System table lists bti;
//   encoder/mod.rs:3 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:972 "bti" => encode_bti(raw_operands);
//   ARM ARM BTI is HINT with CRm=0b0100, op2=0bxx0:
//     omitted → HINT #32 = 0xD503241F;
//     c       → HINT #34 = 0xD503245F (op2 bit1 / word bit6);
//     j       → HINT #36 = 0xD503249F (op2 bit2 / word bit7);
//     jc      → HINT #38 = 0xD50324DF;
//   llvm-mc -triple=aarch64 -show-encoding (LLVM 15.0.6);
//   gas: "unknown option to BTI at operand 1", "unexpected characters following instruction".
// Stronger considered:
//   - State machine: rejected — encode_bti is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree BTI decoder
//   - Differential vs encode_hint / NOP/YIELD/WFE/WFI/SEV/SEVL: rejected — same-job gate
//     (those mnemonics are HINT aliases with a different operand grammar)
// Weaker available: algebraic.metamorphic (j/c flag-bit independence; case/whitespace),
//   algebraic.invariant (ARM BTI layout),
//   negative_error (unknown target / extra operand)
// Differential: candidate=encode_bti, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=raw_operands <-> `bti` / `bti <target>`

use super::encode_bti;
use super::EncodeResult;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;

/// ARM ARM BTI / HINT #32 template: CRm=0100, op2=000, Rt=11111.
const BTI_TEMPLATE: u32 = 0xd503241f;
const BTI_HI: u32 = 0xd5032; // bits[31:12]
const BTI_CRM: u32 = 0b0100;
const J_BIT: u32 = 1 << 7; // op2[2]
const C_BIT: u32 = 1 << 6; // op2[1]

const VALID: &[&str] = &["", "c", "j", "jc"];

fn cfg() -> ProptestConfig {
    ProptestConfig::with_cases(CASES)
}

/// ARM ARM BTI word from independent j/c flags. Not the SUT match table.
fn arm_bti_word(target: &str) -> u32 {
    let t = target.trim().to_ascii_lowercase();
    let j = matches!(t.as_str(), "j" | "jc") as u32;
    let c = matches!(t.as_str(), "c" | "jc") as u32;
    BTI_TEMPLATE | (j << 7) | (c << 6)
}

fn is_valid_target(s: &str) -> bool {
    matches!(s.trim().to_ascii_lowercase().as_str(), "" | "c" | "j" | "jc")
}

fn sut_word(raw: &str) -> Result<u32, String> {
    match encode_bti(raw)? {
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
        "bti".to_string()
    } else {
        format!("bti {raw}")
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

fn valid_raw() -> impl Strategy<Value = String> {
    let target = prop::sample::select(VALID.iter().map(|s| s.to_string()).collect::<Vec<_>>());
    (target, ws_pad(), ws_pad()).prop_flat_map(|(t, pre, post)| {
        ascii_case_variant(&t).prop_map(move |cased| format!("{pre}{cased}{post}"))
    })
}

fn extra_token() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("x0".to_string()),
        Just("x1".to_string()),
        Just("xzr".to_string()),
        Just("w0".to_string()),
        Just("#0".to_string()),
        Just("#32".to_string()),
        Just("#127".to_string()),
        Just("sy".to_string()),
        Just("c".to_string()),
        Just("j".to_string()),
        Just("jc".to_string()),
        Just("foo".to_string()),
        Just("sp".to_string()),
        Just("x0, x1".to_string()),
        "[a-z]{1,8}".prop_map(|s| s),
    ]
    .prop_filter("non-empty extra", |s| !s.trim().is_empty())
}

fn unknown_target() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("cj".to_string()),
        Just("cs".to_string()),
        Just("js".to_string()),
        Just("cc".to_string()),
        Just("jj".to_string()),
        Just("jcj".to_string()),
        Just("j c".to_string()),
        Just("c j".to_string()),
        Just("c  j".to_string()),
        Just("#32".to_string()),
        Just("#34".to_string()),
        Just("#36".to_string()),
        Just("#38".to_string()),
        Just("#0".to_string()),
        Just("32".to_string()),
        Just("{c}".to_string()),
        Just("{j}".to_string()),
        Just("{jc}".to_string()),
        Just("c,".to_string()),
        Just("j,".to_string()),
        Just("jc,".to_string()),
        Just(",".to_string()),
        Just("x0".to_string()),
        Just("xzr".to_string()),
        Just("sy".to_string()),
        Just("nop".to_string()),
        Just("hint".to_string()),
        Just("bti".to_string()),
        Just("csync".to_string()),
        Just(".".to_string()),
        Just("c x0".to_string()),
        Just("j x0".to_string()),
        Just("jc x0".to_string()),
        Just("c  x0".to_string()),
        Just("j\tc".to_string()),
        Just("jc,".to_string()),
        Just("c,".to_string()),
        Just("#32, x0".to_string()),
        Just("cccccccccccccccc".to_string()),
        Just("jjjj".to_string()),
        "[A-Za-z]{1,12}".prop_map(|s| s),
        "[A-Za-z0-9_#{}]{1,16}".prop_map(|s| s),
    ]
    .prop_filter("not a valid BTI target after trim+casefold", |s| {
        !is_valid_target(s)
    })
}

#[test]
fn encode_bti_kat_llvm_mc_omitted() {
    let mc = llvm_mc_word("bti").expect("llvm-mc bti");
    assert_eq!(mc, 0xd503241f, "llvm-mc KAT mapping broken for bti");
    let sut = sut_word("").expect("SUT KAT omitted");
    assert_eq!(sut, mc);
}

#[test]
fn encode_bti_kat_llvm_mc_c() {
    let mc = llvm_mc_word("bti c").expect("llvm-mc bti c");
    assert_eq!(mc, 0xd503245f, "llvm-mc KAT mapping broken for bti c");
    let sut = sut_word("c").expect("SUT KAT c");
    assert_eq!(sut, mc);
}

#[test]
fn encode_bti_kat_llvm_mc_j() {
    let mc = llvm_mc_word("bti j").expect("llvm-mc bti j");
    assert_eq!(mc, 0xd503249f, "llvm-mc KAT mapping broken for bti j");
    let sut = sut_word("j").expect("SUT KAT j");
    assert_eq!(sut, mc);
}

#[test]
fn encode_bti_kat_llvm_mc_jc() {
    let mc = llvm_mc_word("bti jc").expect("llvm-mc bti jc");
    assert_eq!(mc, 0xd50324df, "llvm-mc KAT mapping broken for bti jc");
    let sut = sut_word("jc").expect("SUT KAT jc");
    assert_eq!(sut, mc);
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_bti_diff_targets(raw in valid_raw()) {
        let asm = llvm_asm(&raw);
        let mc = llvm_mc_word(&asm).expect(&asm);
        let expect = arm_bti_word(&raw);
        prop_assert_eq!(mc, expect, "llvm-mc vs ARM ARM for {}", asm);
        let sut = sut_word(&raw).expect(&asm);
        prop_assert_eq!(sut, mc, "SUT vs llvm-mc for {}", asm);
    }

    #[test]
    fn encode_bti_inv_arm_layout(target in prop::sample::select(VALID.iter().map(|s| s.to_string()).collect::<Vec<_>>())) {
        let w = sut_word(&target).expect("valid BTI target must encode");
        let expect = arm_bti_word(&target);
        prop_assert_eq!(w, expect, "ARM BTI word for {:?}", target);
        prop_assert_eq!(w >> 12, BTI_HI, "bits[31:12] HINT/BTI group");
        prop_assert_eq!((w >> 8) & 0xF, BTI_CRM, "CRm bits[11:8]=0100");
        prop_assert_eq!((w >> 5) & 1, 0, "op2[0] / bit5 must be 0");
        prop_assert_eq!(w & 0x1F, 0b11111, "Rt bits[4:0]=11111");
        let j = matches!(target.as_str(), "j" | "jc") as u32;
        let c = matches!(target.as_str(), "c" | "jc") as u32;
        prop_assert_eq!((w >> 7) & 1, j, "j flag bit7");
        prop_assert_eq!((w >> 6) & 1, c, "c flag bit6");
    }

    #[test]
    fn encode_bti_meta_jc_bits(_n in 0u32..8) {
        let w0 = sut_word("").expect("omitted");
        let wc = sut_word("c").expect("c");
        let wj = sut_word("j").expect("j");
        let wjc = sut_word("jc").expect("jc");
        prop_assert_eq!(wj ^ w0, J_BIT, "j must set only bit7 vs omitted");
        prop_assert_eq!(wc ^ w0, C_BIT, "c must set only bit6 vs omitted");
        prop_assert_eq!(wjc, wj ^ wc ^ w0, "jc must be XOR of j and c flags onto omitted");
        let set = [w0, wc, wj, wjc];
        for i in 0..4 {
            for k in (i + 1)..4 {
                prop_assert_ne!(set[i], set[k], "four BTI targets must encode distinctly");
            }
        }
    }

    #[test]
    fn encode_bti_meta_case_ws(raw in valid_raw()) {
        let canonical = raw.trim().to_ascii_lowercase();
        let w = sut_word(&raw).expect("case/ws variant must encode");
        let w0 = sut_word(&canonical).expect("canonical must encode");
        prop_assert_eq!(w, w0, "case/whitespace must be behavior-preserving for {:?}", raw);
        prop_assert_eq!(w, arm_bti_word(&canonical));
    }

    #[test]
    fn encode_bti_neg_unknown(s in unknown_target()) {
        prop_assume!(!is_valid_target(&s));
        let asm = llvm_asm(&s);
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let err = encode_bti(&s).expect_err(&format!(
            "unknown BTI target {:?} must Err (llvm-mc rejects {})",
            s, asm
        ));
        prop_assert!(
            err.contains("unsupported bti target"),
            "error must name unsupported bti target, got {err:?} for {s:?}"
        );
    }

    #[test]
    fn encode_bti_neg_extra(
        target in prop::sample::select(VALID.iter().map(|s| s.to_string()).collect::<Vec<_>>()),
        extra in extra_token(),
        comma in any::<bool>()
    ) {
        // Comma form (`bti c, x0`) and space form (`bti c x0`) are both rejected
        // by gas/llvm-mc. Empty target + space form is `bti x0` (unknown operand),
        // which is the same rejection class.
        let (raw, asm) = if comma {
            if target.is_empty() {
                (format!(", {extra}"), format!("bti, {extra}"))
            } else {
                (format!("{target}, {extra}"), format!("bti {target}, {extra}"))
            }
        } else if target.is_empty() {
            (extra.clone(), format!("bti {extra}"))
        } else {
            (format!("{target} {extra}"), format!("bti {target} {extra}"))
        };
        prop_assume!(!is_valid_target(&raw));
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted extra {}",
            asm
        );
        let err = encode_bti(&raw).expect_err(&format!(
            "extra operand must Err (llvm-mc rejects {}); SUT raw {:?}",
            asm, raw
        ));
        prop_assert!(
            err.contains("unsupported bti target"),
            "error must name unsupported bti target, got {err:?} for {raw:?}"
        );
    }
}
