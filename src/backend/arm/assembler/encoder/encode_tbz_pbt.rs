// Oracle: differential — llvm-mc AArch64 assembler (immediate-offset form);
//   algebraic.invariant (symbol reloc / word layout); algebraic.metamorphic (TBZ vs TBNZ);
//   negative_error (arity / extra / wrong Rt / bit OOR).
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md:220 Branches lists tbz/tbnz; README.md:267 TstBr14 ELF 279;
//   README.md:458 TBZ/TBNZ deferred as branch relocations;
//   encoder/mod.rs:3 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:455-456 "tbz" => encode_tbz(operands, false) / "tbnz" => encode_tbz(operands, true);
//   compare_branch.rs:261 TBZ/TBNZ: b5 011011 op b40 imm14 Rt;
//   ARM ARM Test and branch (immediate): b5 011011 op b40 imm14 Rt,
//     bit = b5:b40 in 0..63 (W requires b5=0 so 0..31), offset/4 ±32 KiB.
// Stronger considered:
//   - State machine: rejected — encode_tbz is a pure function with no lifecycle
//   - Algebraic round-trip via in-tree decoder: rejected — no TBZ decoder
//   - encode_cbz / encode_cond_branch as differential siblings: rejected — different jobs
//     (compare-and-branch CondBr19 / B.cond)
// Weaker available: algebraic.invariant (opcode/reloc fields), algebraic.metamorphic
//   (bit-24 XOR vs TBNZ), negative_error (empty/extra/SP/FP/bit-OOR)
// Differential: candidate=encode_tbz, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[Reg(rt), Imm(bit), Imm(imm)] <-> asm text `tbz/tbnz rt, #bit, #imm`;
//   [Reg(rt), Imm(bit), Symbol(s)|Label(s)|SymbolOffset(s,a)] <-> `tbz/tbnz rt, #bit, s{+a}`

use super::encode_tbz;
use super::{EncodeResult, RelocType};
use crate::backend::arm::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;
/// ARM ARM TBZ/TBNZ signed PC offset: ±32 KiB, multiple of 4.
const IMM_MIN: i64 = -32_768; // -2^15
const IMM_MAX: i64 = 32_764; // 2^15 - 4
const TBZ_B5_0: u32 = 0x3600_0000;
const TBNZ_B5_0: u32 = 0x3700_0000;
const TBZ_B5_1: u32 = 0xb600_0000;
const TBNZ_B5_1: u32 = 0xb700_0000;

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

fn mnemonic(is_nz: bool) -> &'static str {
    if is_nz {
        "tbnz"
    } else {
        "tbz"
    }
}

/// n=0..30 -> xN; 31 -> xzr; 32 -> lr.
fn x_name(n: u32) -> String {
    match n {
        31 => "xzr".to_string(),
        32 => "lr".to_string(),
        n => format!("x{}", n.min(30)),
    }
}

/// n=0..30 -> wN; 31 -> wzr.
fn w_name(n: u32) -> String {
    match n {
        31 => "wzr".to_string(),
        n => format!("w{}", n.min(30)),
    }
}

fn gpr_and_bit() -> impl Strategy<Value = (String, i64)> {
    prop_oneof![
        (0u32..=32, 0i64..=63).prop_map(|(n, b)| (x_name(n), b)),
        (0u32..=31, 0i64..=31).prop_map(|(n, b)| (w_name(n), b)),
        Just(("x0".to_string(), 0i64)),
        Just(("x0".to_string(), 31i64)),
        Just(("x0".to_string(), 32i64)),
        Just(("x0".to_string(), 63i64)),
        Just(("w0".to_string(), 0i64)),
        Just(("w0".to_string(), 31i64)),
        Just(("xzr".to_string(), 32i64)),
        Just(("wzr".to_string(), 0i64)),
        Just(("lr".to_string(), 0i64)),
        Just(("X0".to_string(), 0i64)),
        Just(("W0".to_string(), 31i64)),
        Just(("x31".to_string(), 0i64)),
        Just(("w31".to_string(), 0i64)),
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
        (-(1i64 << 12)..(1i64 << 12)).prop_map(|k| k * 4),
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

fn wrong_reg_name(which: u32, n: u32) -> String {
    let n = n.min(31);
    match which {
        0 => "sp".to_string(),
        1 => "wsp".to_string(),
        2 => format!("d{}", n),
        3 => format!("s{}", n),
        4 => format!("q{}", n),
        5 => format!("v{}", n),
        6 => format!("h{}", n),
        7 => format!("b{}", n),
        _ => match n {
            0 => "x32".to_string(),
            1 => "w32".to_string(),
            2 => "foo".to_string(),
            3 => "".to_string(),
            4 => "r0".to_string(),
            5 => "x".to_string(),
            6 => "x-1".to_string(),
            _ => "x99".to_string(),
        },
    }
}

fn is_tstbr14(t: &RelocType) -> bool {
    matches!(t, RelocType::TstBr14)
}

fn expected_word(n: u32, bit: i64, is_nz: bool) -> u32 {
    let b = bit as u32;
    let b5 = (b >> 5) & 1;
    let b40 = b & 0x1f;
    let op = if is_nz { 1u32 } else { 0u32 };
    (b5 << 31) | (0b011011 << 25) | (op << 24) | (b40 << 19) | n
}

fn reloc_base(bit: i64, is_nz: bool) -> u32 {
    match (bit >= 32, is_nz) {
        (false, false) => TBZ_B5_0,
        (false, true) => TBNZ_B5_0,
        (true, false) => TBZ_B5_1,
        (true, true) => TBNZ_B5_1,
    }
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_tbz_kat_llvm_mc_tbz_x0_bit0_imm0() {
    let want = 0x3600_0000u32;
    let mc = llvm_mc_word("tbz x0, #0, #0").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [
        Operand::Reg("x0".into()),
        Operand::Imm(0),
        Operand::Imm(0),
    ];
    match encode_tbz(&ops, false) {
        Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
        other => panic!(
            "SUT KAT: expected Word({:#010x}) for tbz x0, #0, #0, got {:?}",
            want, other
        ),
    }
}

#[test]
fn encode_tbz_kat_llvm_mc_tbz_w0_bit0_imm0() {
    let want = 0x3600_0000u32;
    let mc = llvm_mc_word("tbz w0, #0, #0").expect("llvm-mc KAT w0");
    assert_eq!(mc, want, "llvm-mc KAT w0 mapping broken");
    let ops = [
        Operand::Reg("w0".into()),
        Operand::Imm(0),
        Operand::Imm(0),
    ];
    match encode_tbz(&ops, false) {
        Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
        other => panic!(
            "SUT KAT: expected Word({:#010x}) for tbz w0, #0, #0, got {:?}",
            want, other
        ),
    }
}

#[test]
fn encode_tbz_kat_llvm_mc_tbnz_x0_bit32_imm4() {
    let want = 0xb700_0020u32;
    let mc = llvm_mc_word("tbnz x0, #32, #4").expect("llvm-mc KAT tbnz #32 #4");
    assert_eq!(mc, want, "llvm-mc KAT tbnz #32 #4 mapping broken");
    let ops = [
        Operand::Reg("x0".into()),
        Operand::Imm(32),
        Operand::Imm(4),
    ];
    match encode_tbz(&ops, true) {
        Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
        other => panic!(
            "SUT KAT: expected Word({:#010x}) for tbnz x0, #32, #4, got {:?}",
            want, other
        ),
    }
}

#[test]
fn encode_tbz_kat_symbol_foo() {
    let ops = [
        Operand::Reg("x0".into()),
        Operand::Imm(0),
        Operand::Symbol("foo".into()),
    ];
    match encode_tbz(&ops, false) {
        Ok(EncodeResult::WordWithReloc { word, reloc }) => {
            assert_eq!(word, TBZ_B5_0);
            assert!(is_tstbr14(&reloc.reloc_type));
            assert_eq!(reloc.reloc_type.elf_type(), 279);
            assert_eq!(reloc.symbol, "foo");
            assert_eq!(reloc.addend, 0);
        }
        other => panic!(
            "expected WordWithReloc TstBr14 for tbz x0, #0, foo, got {:?}",
            other
        ),
    }
}

proptest! {
    #![proptest_config(cfg())]

    // Oracle: differential
    // Target: encoder.compare_branch.encode_tbz
    #[test]
    fn encode_tbz_diff_imm_llvm_mc(
        (rt, bit) in gpr_and_bit(),
        is_nz in any::<bool>(),
        imm in aligned_imm(),
    ) {
        let asm = format!("{} {}, #{}, #{}", mnemonic(is_nz), rt, bit, imm);
        let ops = [
            Operand::Reg(rt.clone()),
            Operand::Imm(bit),
            Operand::Imm(imm),
        ];
        let sut = match encode_tbz(&ops, is_nz) {
            Ok(EncodeResult::Word(w)) => w,
            other => {
                return Err(TestCaseError::fail(format!(
                    "SUT rejected valid {}: {:?}",
                    asm, other
                )));
            }
        };
        let mc = llvm_mc_word(&asm)
            .map_err(|e| TestCaseError::fail(format!(
                "llvm-mc rejected valid {}: {}",
                asm, e
            )))?;
        prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
    }

    // Oracle: algebraic.invariant
    // Target: encoder.compare_branch.encode_tbz
    #[test]
    fn encode_tbz_symbol_reloc(
        n in 0u32..=31,
        bit in 0i64..=63,
        is_nz in any::<bool>(),
        suffix in 0u32..=1000,
        addend in prop_oneof![
            Just(0i64), Just(-1i64), Just(4i64), Just(8i64),
            Just(-8i64), Just(-4i64), -4096i64..=4096i64,
        ],
    ) {
        let rt = if n == 31 { "xzr".to_string() } else { format!("x{}", n) };
        let sym = format!("labl{}", suffix);
        let base = expected_word(n, bit, is_nz);
        let check = |ops: &[Operand], expect_addend: i64, tag: &str| {
            match encode_tbz(ops, is_nz) {
                Ok(EncodeResult::WordWithReloc { word, reloc }) => {
                    prop_assert_eq!(word, base, "{} word", tag);
                    prop_assert!(
                        is_tstbr14(&reloc.reloc_type),
                        "{} expected TstBr14, got {:?}",
                        tag, reloc.reloc_type
                    );
                    prop_assert_eq!(
                        reloc.reloc_type.elf_type(),
                        279u32,
                        "{} ELF type", tag
                    );
                    prop_assert_eq!(&reloc.symbol, &sym, "{} symbol", tag);
                    prop_assert_eq!(reloc.addend, expect_addend, "{} addend", tag);
                    prop_assert_eq!(word & 0x0007_ffe0, 0, "{} imm14 must be 0", tag);
                    Ok(())
                }
                other => Err(TestCaseError::fail(format!(
                    "{} expected WordWithReloc, got {:?}",
                    tag, other
                ))),
            }
        };
        check(
            &[
                Operand::Reg(rt.clone()),
                Operand::Imm(bit),
                Operand::Symbol(sym.clone()),
            ],
            0,
            "Symbol",
        )?;
        check(
            &[
                Operand::Reg(rt.clone()),
                Operand::Imm(bit),
                Operand::Label(sym.clone()),
            ],
            0,
            "Label",
        )?;
        check(
            &[
                Operand::Reg(rt),
                Operand::Imm(bit),
                Operand::SymbolOffset(sym.clone(), addend),
            ],
            addend,
            "SymbolOffset",
        )?;
    }

    // Oracle: algebraic.invariant
    // Target: encoder.compare_branch.encode_tbz
    #[test]
    fn encode_tbz_word_layout(
        n in 0u32..=31,
        bit in 0i64..=63,
        is_nz in any::<bool>(),
    ) {
        let rt = if n == 31 { "xzr".to_string() } else { format!("x{}", n) };
        match encode_tbz(
            &[
                Operand::Reg(rt),
                Operand::Imm(bit),
                Operand::Symbol("L".into()),
            ],
            is_nz,
        ) {
            Ok(EncodeResult::WordWithReloc { word, reloc }) => {
                let b5 = ((bit as u32) >> 5) & 1;
                let b40 = (bit as u32) & 0x1f;
                prop_assert_eq!((word >> 25) & 0x3f, 0b011011u32, "bits[30:25]");
                prop_assert_eq!(word >> 31, b5, "b5");
                prop_assert_eq!((word >> 24) & 1, if is_nz { 1u32 } else { 0u32 }, "op");
                prop_assert_eq!((word >> 19) & 0x1f, b40, "b40");
                prop_assert_eq!(word & 0x1f, n, "Rt");
                prop_assert_eq!(word & 0x0007_ffe0, 0, "imm14");
                prop_assert!(is_tstbr14(&reloc.reloc_type));
                prop_assert_eq!(word, expected_word(n, bit, is_nz));
                let _ = reloc_base(bit, is_nz);
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
    // Target: encoder.compare_branch.encode_tbz
    #[test]
    fn encode_tbz_meta_tbz_vs_tbnz(
        n in 0u32..=31,
        bit in 0i64..=63,
        suffix in 0u32..=1000,
        addend in -4096i64..=4096i64,
    ) {
        let rt = if n == 31 { "xzr".to_string() } else { format!("x{}", n) };
        let sym = format!("labl{}", suffix);
        let ops = [
            Operand::Reg(rt),
            Operand::Imm(bit),
            Operand::SymbolOffset(sym.clone(), addend),
        ];
        let tbnz = encode_tbz(&ops, true).map_err(|e| TestCaseError::fail(e))?;
        let tbz = encode_tbz(&ops, false).map_err(|e| TestCaseError::fail(e))?;
        match (tbnz, tbz) {
            (
                EncodeResult::WordWithReloc { word: w_nz, reloc: r_nz },
                EncodeResult::WordWithReloc { word: w_z, reloc: r_z },
            ) => {
                prop_assert_eq!(
                    w_nz ^ w_z,
                    1u32 << 24,
                    "TBNZ XOR TBZ must be bit 24 (tbnz={:#010x} tbz={:#010x})",
                    w_nz, w_z
                );
                prop_assert!(is_tstbr14(&r_nz.reloc_type), "TBNZ reloc TstBr14");
                prop_assert!(is_tstbr14(&r_z.reloc_type), "TBZ reloc TstBr14");
                prop_assert_eq!(&r_nz.symbol, &sym);
                prop_assert_eq!(&r_z.symbol, &sym);
                prop_assert_eq!(r_nz.addend, addend);
                prop_assert_eq!(r_z.addend, addend);
            }
            other => {
                return Err(TestCaseError::fail(format!(
                    "expected WordWithReloc pair, got {:?}",
                    other
                )));
            }
        }
    }

    // Oracle: negative_error
    // Target: encoder.compare_branch.encode_tbz
    #[test]
    fn encode_tbz_neg_arity(is_nz in any::<bool>()) {
        prop_assert!(
            encode_tbz(&[], is_nz).is_err(),
            "bare {} must Err (llvm-mc: too few operands)",
            mnemonic(is_nz)
        );
        prop_assert!(
            encode_tbz(&[Operand::Reg("x0".into())], is_nz).is_err(),
            "{} x0 must Err (llvm-mc: too few operands)",
            mnemonic(is_nz)
        );
        prop_assert!(
            encode_tbz(&[Operand::Reg("x0".into()), Operand::Imm(0)], is_nz).is_err(),
            "{} x0, #0 must Err (llvm-mc: too few operands)",
            mnemonic(is_nz)
        );
    }

    // Oracle: negative_error
    // Target: encoder.compare_branch.encode_tbz
    #[test]
    fn encode_tbz_neg_extra_operand(
        n in 0u32..=30,
        bit in 0i64..=63,
        is_nz in any::<bool>(),
        suffix in 0u32..=1000,
        which in 0u32..=3,
    ) {
        let extra = extra_operand(which);
        let ops = [
            Operand::Reg(format!("x{}", n)),
            Operand::Imm(bit),
            Operand::Symbol(format!("labl{}", suffix)),
            extra,
        ];
        prop_assert!(
            encode_tbz(&ops, is_nz).is_err(),
            "{} x{}, #{}, label, extra (which={}) must Err (llvm-mc: invalid operand)",
            mnemonic(is_nz), n, bit, which
        );
    }

    // Oracle: negative_error
    // Target: encoder.compare_branch.encode_tbz
    #[test]
    fn encode_tbz_neg_wrong_reg(
        which in 0u32..=1,
        is_nz in any::<bool>(),
    ) {
        let name = if which == 0 { "sp".to_string() } else { "wsp".to_string() };
        let ops = [
            Operand::Reg(name.clone()),
            Operand::Imm(0),
            Operand::Symbol("L".into()),
        ];
        prop_assert!(
            encode_tbz(&ops, is_nz).is_err(),
            "{} {} , #0, L must Err (llvm-mc rejects SP/WSP)",
            mnemonic(is_nz), name
        );
    }

    // Oracle: negative_error
    // Target: encoder.compare_branch.encode_tbz
    #[test]
    fn encode_tbz_neg_fp_reg(
        which in 2u32..=7,
        n in 0u32..=31,
        is_nz in any::<bool>(),
    ) {
        let name = wrong_reg_name(which, n);
        let ops = [
            Operand::Reg(name.clone()),
            Operand::Imm(0),
            Operand::Symbol("L".into()),
        ];
        prop_assert!(
            encode_tbz(&ops, is_nz).is_err(),
            "{} {} , #0, L must Err (llvm-mc rejects FP/SIMD Rt)",
            mnemonic(is_nz), name
        );
    }

    // Oracle: negative_error (coverage sweep: unparsable names)
    // Target: encoder.compare_branch.encode_tbz
    #[test]
    fn encode_tbz_neg_invalid_name(
        which in 0u32..=7,
        is_nz in any::<bool>(),
    ) {
        let name = wrong_reg_name(8, which);
        let ops = [
            Operand::Reg(name.clone()),
            Operand::Imm(0),
            Operand::Symbol("L".into()),
        ];
        prop_assert!(
            encode_tbz(&ops, is_nz).is_err(),
            "{} {} , #0, L must Err (unparsable register)",
            mnemonic(is_nz), name
        );
    }

    // Oracle: algebraic.metamorphic (coverage sweep: Rt isolation)
    // Target: encoder.compare_branch.encode_tbz
    #[test]
    fn encode_tbz_meta_rt_isolation(
        n in 0u32..=30,
        bit in 0i64..=63,
        is_nz in any::<bool>(),
    ) {
        let a = [
            Operand::Reg(format!("x{}", n)),
            Operand::Imm(bit),
            Operand::Symbol("L".into()),
        ];
        let b = [
            Operand::Reg(format!("x{}", n + 1)),
            Operand::Imm(bit),
            Operand::Symbol("L".into()),
        ];
        match (encode_tbz(&a, is_nz), encode_tbz(&b, is_nz)) {
            (
                Ok(EncodeResult::WordWithReloc { word: wa, .. }),
                Ok(EncodeResult::WordWithReloc { word: wb, .. }),
            ) => {
                prop_assert_eq!(
                    wa ^ wb,
                    n ^ (n + 1),
                    "Rt {{n,n+1}} must differ only in bits[4:0] (wa={:#010x} wb={:#010x})",
                    wa, wb
                );
            }
            other => {
                return Err(TestCaseError::fail(format!(
                    "expected WordWithReloc pair, got {:?}",
                    other
                )));
            }
        }
    }

    // Oracle: negative_error (coverage sweep: get_symbol other-kind arm)
    // Target: encoder.compare_branch.encode_tbz
    #[test]
    fn encode_tbz_neg_bad_label_kind(which in 0u32..=5, is_nz in any::<bool>()) {
        let bad = match which {
            0 => Operand::Mem { base: "x0".into(), offset: 0 },
            1 => Operand::Shift { kind: "lsl".into(), amount: 0 },
            2 => Operand::Extend { kind: "sxtw".into(), amount: 0 },
            3 => Operand::RegArrangement { reg: "v0".into(), arrangement: "16b".into() },
            4 => Operand::Expr("foo+bar".into()),
            _ => Operand::RegList(vec![]),
        };
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Imm(0),
            bad,
        ];
        prop_assert!(
            encode_tbz(&ops, is_nz).is_err(),
            "TBZ label slot does not take Mem/Shift/Extend/RegArrangement/Expr/RegList (which={})",
            which
        );
    }

    // Oracle: negative_error
    // Target: encoder.compare_branch.encode_tbz
    #[test]
    fn encode_tbz_neg_bit_oor(
        n in 0u32..=31,
        is_64 in any::<bool>(),
        is_nz in any::<bool>(),
        which in 0u32..=6,
    ) {
        let rt = if is_64 {
            if n == 31 { "xzr".to_string() } else { format!("x{}", n) }
        } else if n == 31 {
            "wzr".to_string()
        } else {
            format!("w{}", n)
        };
        let bit = if is_64 {
            match which {
                0 => -1i64,
                1 => -8i64,
                2 => 64i64,
                3 => 65i64,
                4 => 127i64,
                5 => i64::MIN,
                _ => i64::MAX,
            }
        } else {
            match which {
                0 => -1i64,
                1 => -8i64,
                2 => 32i64,
                3 => 33i64,
                4 => 63i64,
                5 => 64i64,
                _ => i64::MIN,
            }
        };
        let ops = [
            Operand::Reg(rt.clone()),
            Operand::Imm(bit),
            Operand::Symbol("L".into()),
        ];
        prop_assert!(
            encode_tbz(&ops, is_nz).is_err(),
            "{} {}, #{}, L is out of bit range and must Err",
            mnemonic(is_nz), rt, bit
        );
    }
}

#[test]
fn test_encode_tbz_regression_imm_offset() {
    let ops = [
        Operand::Reg("x0".into()),
        Operand::Imm(0),
        Operand::Imm(IMM_MIN),
    ];
    match encode_tbz(&ops, false) {
        Ok(EncodeResult::Word(w)) => {
            let mc = llvm_mc_word("tbz x0, #0, #-32768").expect("llvm-mc");
            assert_eq!(w, mc, "tbz x0, #0, #-32768 must match llvm-mc");
        }
        other => panic!(
            "tbz x0, #0, #-32768 must encode as Word matching llvm-mc, got {:?}",
            other
        ),
    }
}

#[test]
fn test_encode_tbz_regression_extra_operand() {
    let ops = [
        Operand::Reg("x0".into()),
        Operand::Imm(0),
        Operand::Symbol("labl0".into()),
        Operand::Reg("x1".into()),
    ];
    assert!(
        encode_tbz(&ops, false).is_err(),
        "tbz x0, #0, labl0, x1 must Err; llvm-mc rejects a fourth operand"
    );
}

#[test]
fn test_encode_tbz_regression_sp_as_zr() {
    let ops = [
        Operand::Reg("sp".into()),
        Operand::Imm(0),
        Operand::Symbol("L".into()),
    ];
    assert!(
        encode_tbz(&ops, false).is_err(),
        "tbz sp, #0, L must Err; llvm-mc rejects SP"
    );
}

#[test]
fn test_encode_tbz_regression_fp_reg() {
    let ops = [
        Operand::Reg("d0".into()),
        Operand::Imm(0),
        Operand::Symbol("L".into()),
    ];
    assert!(
        encode_tbz(&ops, false).is_err(),
        "tbz d0, #0, L must Err; llvm-mc rejects FP/SIMD Rt"
    );
}

#[test]
fn test_encode_tbz_regression_bit_oor() {
    let ops = [
        Operand::Reg("w0".into()),
        Operand::Imm(32),
        Operand::Symbol("L".into()),
    ];
    assert!(
        encode_tbz(&ops, false).is_err(),
        "tbz w0, #32, L must Err; llvm-mc requires bit in [0, 31] for W"
    );
}
