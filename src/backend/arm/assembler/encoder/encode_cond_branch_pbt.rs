// Oracle: differential — llvm-mc AArch64 assembler (immediate-offset form);
//   algebraic.invariant (symbol reloc / word layout); algebraic.metamorphic (cs==hs, cc==lo, invert);
//   negative_error (arity / extra / unknown cond).
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md:220 Branches lists b.eq/beq, ... (all 16 conditions); README.md:267 CondBr19 ELF 280;
//   README.md:458 B.cond deferred as branch relocations;
//   encoder/mod.rs:3 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:351-353 b.{cond} => encode_cond_branch(cond, operands);
//   compare_branch.rs:200 B.cond: 01010100 imm19 0 cond;
//   ARM ARM Conditional branch (immediate): 01010100 imm19 0 cond,
//     offset/4 signed 19-bit (±1 MiB, multiple of 4).
// Stronger considered:
//   - State machine: rejected — encode_cond_branch is a pure function with no lifecycle
//   - Algebraic round-trip via in-tree decoder: rejected — no B.cond decoder
//   - encode_branch / encode_cbz / encode_tbz as differential siblings: rejected — different jobs
//     (unconditional B/Jump26, compare-and-branch CondBr19, test-and-branch TstBr14)
// Weaker available: algebraic.invariant (opcode/reloc fields), algebraic.metamorphic
//   (cs==hs, cc==lo, invert XOR 1, cond isolation), negative_error (empty/extra/unknown cond)
// Differential: candidate=encode_cond_branch, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[Imm(imm)] <-> asm text `b.{cond} #imm`;
//   [Symbol(s)|Label(s)|SymbolOffset(s,a)] <-> `b.{cond} s{+a}`

use super::encode_cond_branch;
use super::{EncodeResult, RelocType};
use crate::backend::arm::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;
/// ARM ARM B.cond signed PC offset: ±1 MiB, multiple of 4.
const IMM_MIN: i64 = -1_048_576; // -2^20
const IMM_MAX: i64 = 1_048_572; // 2^20 - 4
const BCOND_OP: u32 = 0x54 << 24; // 01010100 imm19 0 cond

const CONDS: &[&str] = &[
    "eq", "ne", "cs", "hs", "cc", "lo", "mi", "pl", "vs", "vc", "hi", "ls", "ge", "lt",
    "gt", "le", "al", "nv",
];

fn cfg() -> ProptestConfig {
    ProptestConfig::with_cases(CASES)
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

fn cond_strat() -> impl Strategy<Value = String> {
    prop_oneof![
        prop::sample::select(CONDS.iter().map(|s| s.to_string()).collect::<Vec<_>>()),
        prop::sample::select(CONDS.iter().map(|s| s.to_ascii_uppercase()).collect::<Vec<_>>()),
        Just("eq".to_string()),
        Just("ne".to_string()),
        Just("al".to_string()),
        Just("nv".to_string()),
        Just("hs".to_string()),
        Just("cs".to_string()),
        Just("lo".to_string()),
        Just("cc".to_string()),
        Just("EQ".to_string()),
        Just("Hs".to_string()),
    ]
}

fn aligned_imm() -> impl Strategy<Value = i64> {
    prop_oneof![
        Just(IMM_MIN),
        Just(IMM_MIN + 4),
        Just(-8i64),
        Just(-4i64),
        Just(0i64),
        Just(4i64),
        Just(8i64),
        Just(IMM_MAX - 4),
        Just(IMM_MAX),
        (-(1i64 << 16)..(1i64 << 16)).prop_map(|k| k * 4),
    ]
}

fn addend_strat() -> impl Strategy<Value = i64> {
    prop_oneof![
        Just(0i64),
        Just(-1i64),
        Just(4i64),
        Just(8i64),
        Just(-8i64),
        Just(-4i64),
        -4096i64..=4096i64,
    ]
}

fn extra_operand(which: u32) -> Operand {
    match which {
        0 => Operand::Reg("x1".into()),
        1 => Operand::Imm(0),
        2 => Operand::Symbol("bar".into()),
        _ => Operand::Mem {
            base: "x1".into(),
            offset: 0,
        },
    }
}

fn is_condbr19(t: &RelocType) -> bool {
    matches!(t, RelocType::CondBr19)
}

fn unknown_cond() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("xx".to_string()),
        Just("eqq".to_string()),
        Just("neq".to_string()),
        Just("e".to_string()),
        Just("zzz".to_string()),
        Just("n".to_string()),
        Just("cond".to_string()),
        Just("eq ".to_string()),
        Just(" eq".to_string()),
        Just("1".to_string()),
        Just("foo".to_string()),
        Just("alx".to_string()),
        Just("nvv".to_string()),
        Just("hs1".to_string()),
        Just("".to_string()),
    ]
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_cond_branch_kat_llvm_mc_beq_imm0() {
    let want = 0x5400_0000u32;
    let mc = llvm_mc_word("b.eq #0").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [Operand::Imm(0)];
    match encode_cond_branch("eq", &ops) {
        Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
        other => panic!(
            "SUT KAT: expected Word({:#010x}) for b.eq #0, got {:?}",
            want, other
        ),
    }
}

#[test]
fn encode_cond_branch_kat_llvm_mc_beq_imm4() {
    let want = 0x5400_0020u32;
    let mc = llvm_mc_word("b.eq #4").expect("llvm-mc KAT #4");
    assert_eq!(mc, want, "llvm-mc KAT #4 mapping broken");
    let ops = [Operand::Imm(4)];
    match encode_cond_branch("eq", &ops) {
        Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
        other => panic!(
            "SUT KAT: expected Word({:#010x}) for b.eq #4, got {:?}",
            want, other
        ),
    }
}

#[test]
fn encode_cond_branch_kat_llvm_mc_bne_nv_al_hs() {
    assert_eq!(llvm_mc_word("b.ne #0").unwrap(), 0x5400_0001);
    assert_eq!(llvm_mc_word("b.nv #0").unwrap(), 0x5400_000f);
    assert_eq!(llvm_mc_word("b.al #0").unwrap(), 0x5400_000e);
    assert_eq!(llvm_mc_word("b.hs #0").unwrap(), 0x5400_0002);
    assert_eq!(llvm_mc_word("b.cs #0").unwrap(), 0x5400_0002);
    assert_eq!(llvm_mc_word("b.lo #0").unwrap(), 0x5400_0003);
    assert_eq!(llvm_mc_word("b.cc #0").unwrap(), 0x5400_0003);
}

#[test]
fn encode_cond_branch_kat_symbol_foo() {
    let ops = [Operand::Symbol("foo".into())];
    match encode_cond_branch("eq", &ops) {
        Ok(EncodeResult::WordWithReloc { word, reloc }) => {
            assert_eq!(word, 0x5400_0000);
            assert!(is_condbr19(&reloc.reloc_type));
            assert_eq!(reloc.reloc_type.elf_type(), 280);
            assert_eq!(reloc.symbol, "foo");
            assert_eq!(reloc.addend, 0);
        }
        other => panic!(
            "expected WordWithReloc CondBr19 for b.eq foo, got {:?}",
            other
        ),
    }
}

proptest! {
    #![proptest_config(cfg())]

    // Oracle: differential
    // Target: encoder.compare_branch.encode_cond_branch
    #[test]
    fn encode_cond_branch_diff_imm_llvm_mc(
        cond in cond_strat(),
        imm in aligned_imm(),
    ) {
        let asm = format!("b.{} #{}", cond, imm);
        let ops = [Operand::Imm(imm)];
        let sut = match encode_cond_branch(&cond, &ops) {
            Ok(EncodeResult::Word(w)) => w,
            other => {
                return Err(TestCaseError::fail(format!(
                    "SUT rejected valid B.cond {}: {:?}",
                    asm, other
                )));
            }
        };
        let mc = llvm_mc_word(&asm)
            .map_err(|e| TestCaseError::fail(format!(
                "llvm-mc rejected valid B.cond {}: {}",
                asm, e
            )))?;
        prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
    }

    // Oracle: differential
    // Target: encoder.compare_branch.encode_cond_branch
    #[test]
    fn encode_cond_branch_diff_reloc_eq_imm0(
        cond in cond_strat(),
        suffix in 0u32..=1000,
    ) {
        let asm0 = format!("b.{} #0", cond);
        let mc = llvm_mc_word(&asm0)
            .map_err(|e| TestCaseError::fail(format!(
                "llvm-mc rejected {}: {}",
                asm0, e
            )))?;
        let sym = format!("labl{}", suffix);
        match encode_cond_branch(&cond, &[Operand::Symbol(sym)]) {
            Ok(EncodeResult::WordWithReloc { word, reloc }) => {
                prop_assert_eq!(word, mc, "reloc word must equal llvm-mc b.{} #0", cond);
                prop_assert!(is_condbr19(&reloc.reloc_type));
            }
            other => {
                return Err(TestCaseError::fail(format!(
                    "expected WordWithReloc for b.{} label, got {:?}",
                    cond, other
                )));
            }
        }
    }

    // Oracle: algebraic.invariant
    // Target: encoder.compare_branch.encode_cond_branch
    #[test]
    fn encode_cond_branch_symbol_reloc(
        cond in cond_strat(),
        suffix in 0u32..=1000,
        addend in addend_strat(),
    ) {
        let sym = format!("labl{}", suffix);
        let check = |ops: &[Operand], expect_addend: i64, tag: &str| {
            match encode_cond_branch(&cond, ops) {
                Ok(EncodeResult::WordWithReloc { word, reloc }) => {
                    prop_assert_eq!(word & 0xffff_ffe0, BCOND_OP, "{} opcode", tag);
                    prop_assert!(
                        is_condbr19(&reloc.reloc_type),
                        "{} expected CondBr19, got {:?}",
                        tag, reloc.reloc_type
                    );
                    prop_assert_eq!(
                        reloc.reloc_type.elf_type(),
                        280u32,
                        "{} ELF type", tag
                    );
                    prop_assert_eq!(&reloc.symbol, &sym, "{} symbol", tag);
                    prop_assert_eq!(reloc.addend, expect_addend, "{} addend", tag);
                    prop_assert_eq!((word >> 5) & 0x7ffff, 0, "{} imm19 must be 0", tag);
                    Ok(())
                }
                other => Err(TestCaseError::fail(format!(
                    "{} expected WordWithReloc, got {:?}",
                    tag, other
                ))),
            }
        };
        check(&[Operand::Symbol(sym.clone())], 0, "Symbol")?;
        check(&[Operand::Label(sym.clone())], 0, "Label")?;
        check(
            &[Operand::SymbolOffset(sym.clone(), addend)],
            addend,
            "SymbolOffset",
        )?;
    }

    // Oracle: algebraic.invariant
    // Target: encoder.compare_branch.encode_cond_branch
    #[test]
    fn encode_cond_branch_word_layout(
        cond in cond_strat(),
        suffix in 0u32..=1000,
    ) {
        let sym = format!("labl{}", suffix);
        match encode_cond_branch(&cond, &[Operand::Symbol(sym)]) {
            Ok(EncodeResult::WordWithReloc { word, reloc }) => {
                prop_assert_eq!(word >> 24, 0x54u32, "bits[31:24]=01010100");
                prop_assert_eq!((word >> 4) & 1, 0u32, "bit 4 must be 0");
                prop_assert_eq!((word >> 5) & 0x7ffff, 0u32, "imm19 reloc form");
                prop_assert!((word & 0xf) <= 15, "cond in 0..15");
                prop_assert!(is_condbr19(&reloc.reloc_type));
            }
            other => {
                return Err(TestCaseError::fail(format!(
                    "expected WordWithReloc, got {:?}",
                    other
                )));
            }
        }
    }

    // Oracle: algebraic.metamorphic
    // Target: encoder.compare_branch.encode_cond_branch
    #[test]
    fn encode_cond_branch_meta_cond(
        suffix in 0u32..=1000,
        addend in addend_strat(),
    ) {
        let sym = format!("labl{}", suffix);
        let ops = [Operand::SymbolOffset(sym.clone(), addend)];
        let word_of = |c: &str| -> Result<u32, TestCaseError> {
            match encode_cond_branch(c, &ops) {
                Ok(EncodeResult::WordWithReloc { word, reloc }) => {
                    prop_assert!(is_condbr19(&reloc.reloc_type));
                    prop_assert_eq!(&reloc.symbol, &sym);
                    prop_assert_eq!(reloc.addend, addend);
                    Ok(word)
                }
                other => Err(TestCaseError::fail(format!(
                    "expected WordWithReloc for {}, got {:?}",
                    c, other
                ))),
            }
        };
        let w_cs = word_of("cs")?;
        let w_hs = word_of("hs")?;
        prop_assert_eq!(w_cs, w_hs, "cs == hs");
        let w_cc = word_of("cc")?;
        let w_lo = word_of("lo")?;
        prop_assert_eq!(w_cc, w_lo, "cc == lo");
        let pairs = [
            ("eq", "ne"),
            ("cs", "cc"),
            ("mi", "pl"),
            ("vs", "vc"),
            ("hi", "ls"),
            ("ge", "lt"),
            ("gt", "le"),
            ("al", "nv"),
        ];
        for (a, b) in pairs {
            let wa = word_of(a)?;
            let wb = word_of(b)?;
            prop_assert_eq!(
                wa ^ wb,
                1u32,
                "{} XOR {} must be 1 ({:#010x} vs {:#010x})",
                a, b, wa, wb
            );
        }
        let w_eq = word_of("eq")?;
        let w_gt = word_of("gt")?;
        prop_assert_eq!(
            (w_eq ^ w_gt) & !0xfu32,
            0u32,
            "cond change must only touch bits[3:0]"
        );
    }

    // Oracle: negative_error
    // Target: encoder.compare_branch.encode_cond_branch
    #[test]
    fn encode_cond_branch_neg_arity(cond in cond_strat()) {
        prop_assert!(
            encode_cond_branch(&cond, &[]).is_err(),
            "bare b.{} must Err (llvm-mc: too few operands)",
            cond
        );
    }

    // Oracle: negative_error
    // Target: encoder.compare_branch.encode_cond_branch
    #[test]
    fn encode_cond_branch_neg_extra_operand(
        cond in cond_strat(),
        suffix in 0u32..=1000,
        which in 0u32..=3,
    ) {
        let extra = extra_operand(which);
        let ops = [
            Operand::Symbol(format!("labl{}", suffix)),
            extra,
        ];
        prop_assert!(
            encode_cond_branch(&cond, &ops).is_err(),
            "b.{} label, extra (which={}) must Err (llvm-mc: invalid operand)",
            cond, which
        );
    }

    // Oracle: negative_error
    // Target: encoder.compare_branch.encode_cond_branch
    #[test]
    fn encode_cond_branch_neg_unknown_cond(c in unknown_cond()) {
        let lower = c.to_ascii_lowercase();
        prop_assume!(!CONDS.contains(&lower.as_str()));
        prop_assert!(
            encode_cond_branch(&c, &[Operand::Symbol("L".into())]).is_err(),
            "unknown condition {:?} must Err",
            c
        );
    }

    // Oracle: negative_error (coverage sweep: operand kinds gas/llvm-mc reject)
    // Target: encoder.compare_branch.encode_cond_branch
    #[test]
    fn encode_cond_branch_neg_bad_operand(
        cond in cond_strat(),
        which in 0u32..=4,
    ) {
        let bad = match which {
            0 => Operand::Mem { base: "x0".into(), offset: 0 },
            1 => Operand::Shift { kind: "lsl".into(), amount: 0 },
            2 => Operand::Extend { kind: "sxtw".into(), amount: 0 },
            3 => Operand::RegArrangement { reg: "v0".into(), arrangement: "16b".into() },
            _ => Operand::Expr("foo+bar".into()),
        };
        prop_assert!(
            encode_cond_branch(&cond, &[bad]).is_err(),
            "b.{} Mem/Shift/Extend/RegArrangement/Expr (which={}) must Err",
            cond, which
        );
    }

    // Oracle: negative_error (coverage sweep: :lo12: modifier)
    // Target: encoder.compare_branch.encode_cond_branch
    #[test]
    fn encode_cond_branch_neg_modifier(
        cond in cond_strat(),
        which in 0u32..=1,
    ) {
        let bad = if which == 0 {
            Operand::Modifier { kind: "lo12".into(), symbol: "foo".into() }
        } else {
            Operand::ModifierOffset { kind: "lo12".into(), symbol: "foo".into(), offset: 8 }
        };
        prop_assert!(
            encode_cond_branch(&cond, &[bad]).is_err(),
            "b.{} :lo12:foo must Err (which={}); llvm-mc/gas reject modifiers",
            cond, which
        );
    }

    // Oracle: algebraic.invariant (coverage sweep: parser-misclassified symbols)
    // Target: encoder.compare_branch.encode_cond_branch
    #[test]
    fn encode_cond_branch_symbol_misclassified(
        cond in cond_strat(),
        which in 0u32..=2,
        name in prop::sample::select(vec!["eq", "ne", "lt", "gt", "sy", "ish", "st", "ld"]),
    ) {
        let op = match which {
            0 => Operand::Reg(name.to_string()),
            1 => Operand::Cond(name.to_string()),
            _ => Operand::Barrier(name.to_string()),
        };
        match encode_cond_branch(&cond, &[op]) {
            Ok(EncodeResult::WordWithReloc { word, reloc }) => {
                prop_assert_eq!(word >> 24, 0x54u32);
                prop_assert!(is_condbr19(&reloc.reloc_type));
                prop_assert_eq!(&reloc.symbol, name);
                prop_assert_eq!(reloc.addend, 0);
            }
            other => {
                return Err(TestCaseError::fail(format!(
                    "parser-misclassified {} as B.cond target must be a CondBr19 reloc, got {:?}",
                    name, other
                )));
            }
        }
    }
}

#[test]
fn test_encode_cond_branch_regression_imm_offset() {
    let ops = [Operand::Imm(0)];
    match encode_cond_branch("eq", &ops) {
        Ok(EncodeResult::Word(w)) => {
            let mc = llvm_mc_word("b.eq #0").expect("llvm-mc");
            assert_eq!(w, mc, "b.eq #0 must match llvm-mc");
        }
        other => panic!(
            "b.eq #0 must encode as Word matching llvm-mc, got {:?}",
            other
        ),
    }
}

#[test]
fn test_encode_cond_branch_regression_extra_operand() {
    let ops = [Operand::Symbol("labl0".into()), Operand::Reg("x0".into())];
    assert!(
        encode_cond_branch("eq", &ops).is_err(),
        "b.eq labl0, x0 must Err; llvm-mc rejects a second operand"
    );
}

#[test]
fn test_encode_cond_branch_regression_modifier() {
    let ops = [Operand::Modifier {
        kind: "lo12".into(),
        symbol: "foo".into(),
    }];
    assert!(
        encode_cond_branch("eq", &ops).is_err(),
        "b.eq :lo12:foo must Err; B.cond does not take :lo12: modifiers"
    );
}
