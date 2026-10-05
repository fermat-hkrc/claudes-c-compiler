// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md:239 System table lists dsb;
//   encoder/mod.rs:3 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:967 "dsb" => encode_dsb(operands);
//   parser.rs:50 Barrier option for dmb/dsb: ish, ishld, ishst, sy, etc.;
//   system.rs:45 "unknown dsb option: {}";
//   system.rs:47 "DSB: 0xD503309F | (option << 8)";
//   ARM ARM DSB: 1101 0101 0000 0011 0011 CRm 100 11111;
//   named CRm: sy=1111 st=1110 ld=1101 ish=1011 ishst=1010 ishld=1001
//              nsh=0111 nshst=0110 nshld=0101 osh=0011 oshst=0010 oshld=0001;
//   assembler form also accepts #imm with imm in 0..=15 as CRm.
// Stronger considered:
//   - State machine: rejected — encode_dsb is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree DSB decoder
//   - Differential vs encode_dmb: rejected — same-job gate (DSB vs DMB opcodes)
// Weaker available: algebraic.metamorphic (Barrier vs Symbol), algebraic.invariant (ARM layout),
//   negative_error (unknown name / extra / empty / wrong kind / imm out of 0..=15)
// Differential: candidate=encode_dsb, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[Barrier(name)|Symbol(name)|Imm(crm)] <-> `dsb <option>|#<imm>`

use super::encode_dsb;
use super::EncodeResult;
use crate::backend::arm::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;

/// ARM ARM DSB with CRm=0: 1101 0101 0000 0011 0011 0000 1001 1111
const DSB_TEMPLATE: u32 = 0xd503309f;

const NAMED: &[&str] = &[
    "sy", "st", "ld", "ish", "ishst", "ishld", "nsh", "nshst", "nshld", "osh", "oshst", "oshld",
];

fn cfg() -> ProptestConfig {
    ProptestConfig::with_cases(CASES)
}

/// ARM ARM named-option → CRm nibble. Independent of the SUT match table.
fn arm_crm(name: &str) -> Option<u32> {
    Some(match name.to_ascii_lowercase().as_str() {
        "sy" => 0b1111,
        "st" => 0b1110,
        "ld" => 0b1101,
        "ish" => 0b1011,
        "ishst" => 0b1010,
        "ishld" => 0b1001,
        "nsh" => 0b0111,
        "nshst" => 0b0110,
        "nshld" => 0b0101,
        "osh" => 0b0011,
        "oshst" => 0b0010,
        "oshld" => 0b0001,
        _ => return None,
    })
}

fn is_named(s: &str) -> bool {
    arm_crm(s).is_some()
}

fn sut_word(ops: &[Operand]) -> Result<u32, String> {
    match encode_dsb(ops)? {
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

fn named_option() -> impl Strategy<Value = &'static str> {
    prop::sample::select(NAMED.to_vec())
}

fn case_fold(name: &'static str) -> impl Strategy<Value = String> {
    prop_oneof![
        Just(name.to_string()),
        Just(name.to_ascii_uppercase()),
        Just({
            let mut s = name.to_string();
            if let Some(c) = s.get_mut(0..1) {
                c.make_ascii_uppercase();
            }
            s
        }),
        Just({
            name.chars()
                .enumerate()
                .map(|(i, c)| {
                    if i % 2 == 0 {
                        c.to_ascii_uppercase()
                    } else {
                        c.to_ascii_lowercase()
                    }
                })
                .collect::<String>()
        }),
    ]
}

fn unknown_name() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("foo".to_string()),
        Just("syy".to_string()),
        Just("ishh".to_string()),
        Just("sys".to_string()),
        Just("ishldd".to_string()),
        Just("oshldst".to_string()),
        Just("barrier".to_string()),
        Just("x".to_string()),
        Just("sy ".to_string()),
        Just(" sy".to_string()),
        Just("".to_string()),
        Just("#sy".to_string()),
        Just("ish-ld".to_string()),
        Just("ISH LD".to_string()),
        Just("nshld2".to_string()),
        prop::string::string_regex("[a-zA-Z]{1,8}")
            .unwrap()
            .prop_filter("not a named dsb option", |s| !is_named(s)),
    ]
}

fn extra_operand() -> impl Strategy<Value = Operand> {
    prop_oneof![
        (0u32..=31).prop_map(|n| Operand::Reg(format!("x{n}"))),
        (0u32..=31).prop_map(|n| Operand::Reg(format!("w{n}"))),
        Just(Operand::Reg("sp".into())),
        Just(Operand::Reg("xzr".into())),
        (-4i64..=32).prop_map(Operand::Imm),
        named_option().prop_map(|n| Operand::Barrier(n.to_string())),
        Just(Operand::Cond("eq".into())),
        Just(Operand::Label(".L0".into())),
    ]
}

fn wrong_kind_operand() -> impl Strategy<Value = Operand> {
    prop_oneof![
        prop_oneof![
            Just(-1i64),
            Just(16i64),
            Just(17i64),
            Just(255i64),
            Just(256i64),
            Just(i64::MIN),
            Just(i64::MAX),
            (-4096i64..=-2),
            (16i64..=1024),
        ]
        .prop_map(Operand::Imm),
        (0u32..=31).prop_map(|n| Operand::Reg(format!("x{n}"))),
        (0u32..=31).prop_map(|n| Operand::Reg(format!("w{n}"))),
        Just(Operand::Reg("sp".into())),
        Just(Operand::Reg("xzr".into())),
        Just(Operand::Reg("d0".into())),
        Just(Operand::Mem {
            base: "x0".into(),
            offset: 0,
        }),
        Just(Operand::Cond("eq".into())),
        Just(Operand::Shift {
            kind: "lsl".into(),
            amount: 0,
        }),
        Just(Operand::Label(".LBB0".into())),
        Just(Operand::Extend {
            kind: "sxtw".into(),
            amount: 0,
        }),
    ]
}

fn asm_for_wrong(op: &Operand) -> Option<String> {
    match op {
        Operand::Imm(n) => Some(format!("dsb #{n}")),
        Operand::Reg(r) => Some(format!("dsb {r}")),
        Operand::Mem { base, offset } => Some(format!("dsb [{base}, #{offset}]")),
        Operand::Cond(c) => Some(format!("dsb {c}")),
        Operand::Shift { kind, amount } => Some(format!("dsb {kind} #{amount}")),
        Operand::Label(l) => Some(format!("dsb {l}")),
        Operand::Extend { kind, amount } => Some(format!("dsb {kind} #{amount}")),
        _ => None,
    }
}

#[test]
fn encode_dsb_kat_llvm_mc_sy() {
    let mc = llvm_mc_word("dsb sy").expect("llvm-mc dsb sy");
    assert_eq!(mc, 0xd5033f9f, "llvm-mc KAT mapping broken for dsb sy");
    let sut = sut_word(&[Operand::Barrier("sy".into())]).expect("SUT KAT sy");
    assert_eq!(sut, mc);
}

#[test]
fn encode_dsb_kat_llvm_mc_ish() {
    let mc = llvm_mc_word("dsb ish").expect("llvm-mc dsb ish");
    assert_eq!(mc, 0xd5033b9f, "llvm-mc KAT mapping broken for dsb ish");
    let sut = sut_word(&[Operand::Barrier("ish".into())]).expect("SUT KAT ish");
    assert_eq!(sut, mc);
}

#[test]
fn encode_dsb_kat_llvm_mc_imm0() {
    let mc = llvm_mc_word("dsb #0").expect("llvm-mc dsb #0");
    assert_eq!(mc, 0xd503309f, "llvm-mc KAT mapping broken for dsb #0");
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_dsb_diff_named(
        name in named_option(),
        cased in named_option().prop_flat_map(case_fold),
        as_symbol in any::<bool>(),
    ) {
        // Keep `name` drawn so every enumerator is visited; the assembly uses `cased`
        // which is an independent case-fold of some named option (possibly different).
        let _ = name;
        prop_assume!(is_named(&cased));
        let asm = format!("dsb {cased}");
        let mc = llvm_mc_word(&asm).expect(&asm);
        let op = if as_symbol {
            Operand::Symbol(cased.clone())
        } else {
            Operand::Barrier(cased.clone())
        };
        let sut = sut_word(&[op]).expect(&asm);
        prop_assert_eq!(sut, mc, "SUT vs llvm-mc for {}", asm);
    }

    #[test]
    fn encode_dsb_diff_imm(crm in 0u32..=15) {
        let asm = format!("dsb #{crm}");
        let mc = llvm_mc_word(&asm).expect(&asm);
        let sut = sut_word(&[Operand::Imm(crm as i64)]).expect(&asm);
        prop_assert_eq!(sut, mc, "SUT vs llvm-mc for {}", asm);
    }

    #[test]
    fn encode_dsb_meta_barrier_eq_symbol(
        name in named_option(),
        cased in named_option().prop_flat_map(case_fold),
    ) {
        let _ = name;
        prop_assume!(is_named(&cased));
        let b = sut_word(&[Operand::Barrier(cased.clone())]).expect("barrier");
        let s = sut_word(&[Operand::Symbol(cased.clone())]).expect("symbol");
        prop_assert_eq!(b, s, "Barrier vs Symbol for {}", cased);
    }

    #[test]
    fn encode_dsb_inv_arm_layout(
        name in named_option(),
        other in named_option(),
    ) {
        let crm = arm_crm(name).expect("named");
        let w = sut_word(&[Operand::Barrier(name.to_string())]).expect("layout");
        prop_assert_eq!(w, DSB_TEMPLATE | (crm << 8), "ARM DSB word for {}", name);
        prop_assert_eq!((w >> 12) & 0xfffff, 0xd5033, "bits[31:12] DMB/DSB group");
        prop_assert_eq!((w >> 8) & 0xf, crm, "CRm nibble");
        prop_assert_eq!((w >> 5) & 0x7, 0b100, "op2=100 distinguishes DSB from DMB");
        prop_assert_eq!(w & 0x1f, 0b11111, "Rt=11111");
        if name != other {
            let w2 = sut_word(&[Operand::Barrier(other.to_string())]).expect("other");
            prop_assert_eq!(
                (w ^ w2) & !0xf00u32,
                0,
                "different named options must differ only in bits[11:8]"
            );
        }
    }

    #[test]
    fn encode_dsb_neg_unknown(
        s in unknown_name(),
        as_symbol in any::<bool>(),
    ) {
        prop_assume!(!is_named(&s));
        let op = if as_symbol {
            Operand::Symbol(s.clone())
        } else {
            Operand::Barrier(s.clone())
        };
        let err = encode_dsb(&[op]).expect_err(&format!("unknown {s} must Err"));
        prop_assert!(
            err.contains("unknown dsb option"),
            "error must mention unknown dsb option, got {err}"
        );
    }

    #[test]
    fn encode_dsb_neg_extra(
        name in named_option(),
        extra in extra_operand(),
    ) {
        let extra_asm = match &extra {
            Operand::Reg(r) => r.clone(),
            Operand::Imm(n) => format!("#{n}"),
            Operand::Barrier(b) => b.clone(),
            Operand::Cond(c) => c.clone(),
            Operand::Label(l) => l.clone(),
            _ => "x0".to_string(),
        };
        let asm = format!("dsb {name}, {extra_asm}");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted extra {}",
            asm
        );
        let ops = [Operand::Barrier(name.to_string()), extra];
        prop_assert!(
            encode_dsb(&ops).is_err(),
            "extra operand must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_dsb_neg_empty(_n in 0u32..8) {
        prop_assert!(
            llvm_mc_word("dsb").is_err(),
            "llvm-mc unexpectedly accepted omitted-operand dsb"
        );
        prop_assert!(
            encode_dsb(&[]).is_err(),
            "empty operands must Err (gas/llvm-mc reject omitted dsb option)"
        );
    }

    #[test]
    fn encode_dsb_neg_wrong_kind_imm_oob(op in wrong_kind_operand()) {
        if let Some(asm) = asm_for_wrong(&op) {
            prop_assert!(
                llvm_mc_word(&asm).is_err(),
                "llvm-mc unexpectedly accepted {}",
                asm
            );
        }
        prop_assert!(
            encode_dsb(&[op.clone()]).is_err(),
            "non-named / out-of-range operand must Err, got {:?}",
            encode_dsb(&[op])
        );
    }
}

#[test]
fn test_encode_dsb_regression_imm_crm0() {
    let mc = llvm_mc_word("dsb #0").expect("llvm-mc dsb #0");
    assert_eq!(mc, 0xd503309f);
    let sut = sut_word(&[Operand::Imm(0)]).expect("dsb #0 must encode");
    assert_eq!(sut, mc, "dsb #0 must encode CRm=0, not SY");
}

#[test]
fn test_encode_dsb_regression_extra_sy_x0() {
    assert!(
        llvm_mc_word("dsb sy, x0").is_err(),
        "llvm-mc must reject extra operand"
    );
    let ops = [
        Operand::Barrier("sy".into()),
        Operand::Reg("x0".into()),
    ];
    assert!(
        encode_dsb(&ops).is_err(),
        "dsb sy, x0 must Err (gas/llvm-mc reject extra operands)"
    );
}

#[test]
fn test_encode_dsb_regression_empty() {
    assert!(llvm_mc_word("dsb").is_err(), "llvm-mc must reject omitted option");
    assert!(
        encode_dsb(&[]).is_err(),
        "dsb with no operand must Err (gas/llvm-mc reject omitted option)"
    );
}

#[test]
fn test_encode_dsb_regression_imm_neg1() {
    assert!(
        llvm_mc_word("dsb #-1").is_err(),
        "llvm-mc must reject dsb #-1"
    );
    assert!(
        encode_dsb(&[Operand::Imm(-1)]).is_err(),
        "dsb #-1 must Err (imm out of 0..=15)"
    );
}
