// Oracle: differential — llvm-mc AArch64 assembler (zero-page ADRP word)
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md:221 Address table lists adrp;
//   README.md:248/256/263 WordWithReloc for adrp / AdrpPage21 ELF 275 / AdrGotPage21 ELF 311;
//   encoder/mod.rs:3 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:524 "adrp" => encode_adrp;
//   ARM ARM ADRP: 1 immlo[1:0] 10000 immhi[18:0] Rd (op=1); Rd is Xd (X31=XZR, not SP).
// Stronger considered:
//   - State machine: rejected — encode_adrp is a pure function with no lifecycle
//   - Algebraic round-trip via in-tree decoder: rejected — no ADRP decoder
//   - linker reloc::encode_adrp as differential sibling: rejected — different job (patches imm)
//   - encode_adr as differential sibling: rejected — same-job gate (op=0 / AdrPrelLo21)
// Weaker available: algebraic.invariant (ARM field unpack, AdrpPage21 / AdrGotPage21),
//   algebraic.metamorphic (Rd vs symbol isolation), negative_error (W/SP/FP/arity/extra/:lo12:/Imm)
// Differential: candidate=encode_adrp, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=reloc-form word (imm=0) <-> `adrp Xd, #0` encoding (independent zero-page encoder);
//   GNU as rejects `#imm`, so Imm is not in the SUT domain.

use super::encode_adrp;
use super::{EncodeResult, RelocType};
use crate::backend::arm::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;
const ADRP_OP: u32 = 1;
const ADRP_OPC: u32 = 0b10000;
const ADRP_BASE: u32 = (1u32 << 31) | (0b10000 << 24); // 0x9000_0000, imm=0

fn cfg() -> ProptestConfig {
    ProptestConfig::with_cases(CASES)
}

fn xreg(n: u32) -> String {
    if n == 31 {
        "xzr".into()
    } else {
        format!("x{n}")
    }
}

fn wreg(n: u32) -> String {
    if n == 31 {
        "wzr".into()
    } else {
        format!("w{n}")
    }
}

/// Unpack ADRP fields per ARM ARM (not a copy of the SUT packer).
fn unpack_adrp(word: u32) -> (u32 /*rd*/, i64 /*imm21*/, u32 /*op*/, u32 /*opc*/) {
    let rd = word & 0x1f;
    let immlo = (word >> 29) & 0x3;
    let immhi = (word >> 5) & 0x7ffff;
    let imm21 = (immhi << 2) | immlo;
    let imm = if (imm21 & (1 << 20)) != 0 {
        (imm21 as i64) | !0x1f_ffffi64
    } else {
        imm21 as i64
    };
    let op = (word >> 31) & 1;
    let opc = (word >> 24) & 0x1f;
    (rd, imm, op, opc)
}

fn sut_reloc(ops: &[Operand]) -> Result<(u32, RelocType, String, i64), String> {
    match encode_adrp(ops)? {
        EncodeResult::WordWithReloc { word, reloc } => {
            Ok((word, reloc.reloc_type, reloc.symbol, reloc.addend))
        }
        other => Err(format!("expected WordWithReloc, got {other:?}")),
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

fn addend_edge() -> impl Strategy<Value = i64> {
    prop_oneof![
        Just(0i64),
        Just(-1i64),
        Just(1i64),
        Just(8i64),
        Just(-8i64),
        Just(4096i64),
        Just(-4096i64),
        Just(i64::MIN),
        Just(i64::MAX),
        -4096i64..=4096,
    ]
}

fn extra_operand() -> impl Strategy<Value = Operand> {
    prop_oneof![
        (0u32..=31).prop_map(|n| Operand::Reg(format!("x{n}"))),
        Just(Operand::Imm(0)),
        Just(Operand::Imm(-1)),
        Just(Operand::Shift {
            kind: "lsl".into(),
            amount: 0,
        }),
        Just(Operand::Label("L0".into())),
        Just(Operand::Symbol("foo".into())),
        Just(Operand::Cond("eq".into())),
    ]
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_adrp_kat_llvm_mc_x0_imm0() {
    let want = 0x9000_0000u32;
    let mc = llvm_mc_word("adrp x0, #0").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [Operand::Reg("x0".into()), Operand::Symbol("foo".into())];
    let (word, ty, sym, addend) = sut_reloc(&ops).expect("SUT KAT");
    assert_eq!(word, want);
    match ty {
        RelocType::AdrpPage21 => {}
        other => panic!("KAT expected AdrpPage21, got {other:?}"),
    }
    assert_eq!(sym, "foo");
    assert_eq!(addend, 0);
}

#[test]
fn encode_adrp_kat_llvm_mc_xzr_imm0() {
    let want = 0x9000_001fu32;
    let mc = llvm_mc_word("adrp xzr, #0").expect("llvm-mc KAT xzr");
    assert_eq!(mc, want, "llvm-mc KAT xzr mapping broken");
    let ops = [Operand::Reg("xzr".into()), Operand::Symbol("foo".into())];
    let (word, _, _, _) = sut_reloc(&ops).expect("SUT KAT xzr");
    assert_eq!(word, want);
}

#[test]
fn encode_adrp_kat_got_x0() {
    let want = 0x9000_0000u32;
    let ops = [
        Operand::Reg("x0".into()),
        Operand::Modifier {
            kind: "got".into(),
            symbol: "foo".into(),
        },
    ];
    let (word, ty, sym, addend) = sut_reloc(&ops).expect("SUT GOT KAT");
    assert_eq!(word, want);
    match ty {
        RelocType::AdrGotPage21 => {}
        other => panic!("GOT KAT expected AdrGotPage21, got {other:?}"),
    }
    assert_eq!(sym, "foo");
    assert_eq!(addend, 0);
}

proptest! {
    #![proptest_config(cfg())]

    // Oracle: differential
    // Target: encoder.load_store.encode_adrp
    #[test]
    fn encode_adrp_diff_symbol_word_llvm_mc(
        rd in 0u32..=31,
        suffix in 0u32..=1000,
    ) {
        let rd_n = xreg(rd);
        let sym = format!("s{suffix}");
        let ops = [Operand::Reg(rd_n.clone()), Operand::Symbol(sym)];
        let (sut, _, _, _) = sut_reloc(&ops)
            .unwrap_or_else(|e| panic!("SUT rejected valid ADRP {}: {}", rd_n, e));
        let asm = format!("adrp {}, #0", rd_n);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "SUT reloc-form word vs llvm-mc adrp {}, #0", rd_n);
    }

    // Oracle: algebraic.invariant
    // Target: encoder.load_store.encode_adrp
    #[test]
    fn encode_adrp_arm_fields(
        rd in 0u32..=31,
        kind in 0u32..=3,
        suffix in 0u32..=1000,
        addend in addend_edge(),
    ) {
        let rd_n = xreg(rd);
        let sym = format!("s{suffix}");
        let op1 = match kind {
            0 => Operand::Symbol(sym),
            1 => Operand::Label(sym),
            2 => Operand::SymbolOffset(sym, addend),
            _ => Operand::Modifier {
                kind: "got".into(),
                symbol: sym,
            },
        };
        let ops = [Operand::Reg(rd_n), op1];
        let (word, _, _, _) = sut_reloc(&ops)
            .unwrap_or_else(|e| panic!("SUT rejected in-range ADRP: {}", e));
        let (got_rd, got_imm, op, opc) = unpack_adrp(word);
        prop_assert_eq!(op, ADRP_OP, "ADRP op bit 31 must be 1 (not ADR), word={:#010x}", word);
        prop_assert_eq!(opc, ADRP_OPC, "ADRP bits [28:24] must be 10000, word={:#010x}", word);
        prop_assert_eq!(got_rd, rd, "Rd field mismatch word={:#010x}", word);
        prop_assert_eq!(got_imm, 0, "reloc-form imm fields must be 0, word={:#010x}", word);
    }

    // Oracle: algebraic.metamorphic
    // Target: encoder.load_store.encode_adrp
    #[test]
    fn encode_adrp_meta_rd_symbol(
        rd in 0u32..=31,
        rd2 in 0u32..=31,
        suffix1 in 0u32..=1000,
        suffix2 in 0u32..=1000,
    ) {
        let s1 = format!("s{suffix1}");
        let s2 = format!("s{suffix2}");
        let w = |r: u32, op1: Operand| {
            sut_reloc(&[Operand::Reg(xreg(r)), op1])
                .map(|(word, _, _, _)| word)
        };
        let w_rd_s1 = w(rd, Operand::Symbol(s1.clone()))
            .unwrap_or_else(|e| panic!("encode rd,s1: {e}"));
        let w_rd_s2 = w(rd, Operand::Symbol(s2))
            .unwrap_or_else(|e| panic!("encode rd,s2: {e}"));
        let w_rd2_s1 = w(rd2, Operand::Symbol(s1.clone()))
            .unwrap_or_else(|e| panic!("encode rd2,s1: {e}"));
        let w_rd_got = w(
            rd,
            Operand::Modifier {
                kind: "got".into(),
                symbol: s1,
            },
        )
        .unwrap_or_else(|e| panic!("encode rd,got: {e}"));
        prop_assert_eq!(
            (w_rd_s1 ^ w_rd_s2) & 0x1f,
            0,
            "changing symbol must not change Rd (w1={:#010x} w2={:#010x})",
            w_rd_s1,
            w_rd_s2
        );
        prop_assert_eq!(
            (w_rd_s1 ^ w_rd2_s1) & !0x1fu32,
            0,
            "changing Rd must not change opcode/imm fields (w1={:#010x} w2={:#010x})",
            w_rd_s1,
            w_rd2_s1
        );
        prop_assert_eq!(
            w_rd_s1, w_rd_got,
            "GOT vs page reloc must not change the instruction word (page={:#010x} got={:#010x})",
            w_rd_s1,
            w_rd_got
        );
    }

    // Oracle: algebraic.invariant
    // Target: encoder.load_store.encode_adrp
    #[test]
    fn encode_adrp_reloc_page21(
        rd in 0u32..=31,
        suffix in 0u32..=1000,
        addend in addend_edge(),
    ) {
        let sym = format!("labl{suffix}");
        let rd_n = xreg(rd);
        let expected_word = ADRP_BASE | rd;

        let check = |ops: &[Operand], expect_addend: i64, tag: &str| {
            match encode_adrp(ops) {
                Ok(EncodeResult::WordWithReloc { word, reloc }) => {
                    prop_assert_eq!(word, expected_word, "{} word", tag);
                    match reloc.reloc_type {
                        RelocType::AdrpPage21 => {}
                        other => {
                            return Err(TestCaseError::fail(format!(
                                "{tag} expected AdrpPage21, got {other:?}"
                            )));
                        }
                    }
                    prop_assert_eq!(&reloc.symbol, &sym, "{} symbol", tag);
                    prop_assert_eq!(reloc.addend, expect_addend, "{} addend", tag);
                    Ok(())
                }
                other => Err(TestCaseError::fail(format!(
                    "{tag} expected WordWithReloc, got {other:?}"
                ))),
            }
        };

        check(
            &[Operand::Reg(rd_n.clone()), Operand::Symbol(sym.clone())],
            0,
            "Symbol",
        )?;
        check(
            &[Operand::Reg(rd_n.clone()), Operand::Label(sym.clone())],
            0,
            "Label",
        )?;
        check(
            &[
                Operand::Reg(rd_n),
                Operand::SymbolOffset(sym.clone(), addend),
            ],
            addend,
            "SymbolOffset",
        )?;
    }

    // Oracle: algebraic.invariant
    // Target: encoder.load_store.encode_adrp
    #[test]
    fn encode_adrp_reloc_got(
        rd in 0u32..=31,
        suffix in 0u32..=1000,
        addend in addend_edge(),
    ) {
        let sym = format!("g{suffix}");
        let rd_n = xreg(rd);
        let expected_word = ADRP_BASE | rd;

        let check = |ops: &[Operand], expect_addend: i64, tag: &str| {
            match encode_adrp(ops) {
                Ok(EncodeResult::WordWithReloc { word, reloc }) => {
                    prop_assert_eq!(word, expected_word, "{} word", tag);
                    match reloc.reloc_type {
                        RelocType::AdrGotPage21 => {}
                        other => {
                            return Err(TestCaseError::fail(format!(
                                "{tag} expected AdrGotPage21, got {other:?}"
                            )));
                        }
                    }
                    prop_assert_eq!(&reloc.symbol, &sym, "{} symbol", tag);
                    prop_assert_eq!(reloc.addend, expect_addend, "{} addend", tag);
                    Ok(())
                }
                other => Err(TestCaseError::fail(format!(
                    "{tag} expected WordWithReloc AdrGotPage21, got {other:?}"
                ))),
            }
        };

        check(
            &[
                Operand::Reg(rd_n.clone()),
                Operand::Modifier {
                    kind: "got".into(),
                    symbol: sym.clone(),
                },
            ],
            0,
            "Modifier-got",
        )?;
        check(
            &[
                Operand::Reg(rd_n),
                Operand::ModifierOffset {
                    kind: "got".into(),
                    symbol: sym.clone(),
                    offset: addend,
                },
            ],
            addend,
            "ModifierOffset-got",
        )?;
    }

    // Oracle: negative_error
    // Target: encoder.load_store.encode_adrp
    #[test]
    fn encode_adrp_neg_w_sp_fp(
        kind in 0u32..=2,
        n in 0u32..=31,
        suffix in 0u32..=1000,
        fp_prefix in prop::sample::select(vec!["d", "s", "q", "v", "h", "b"]),
        use_wsp in any::<bool>(),
    ) {
        let dest = match kind {
            0 => {
                if use_wsp {
                    "wsp".to_string()
                } else {
                    wreg(n)
                }
            }
            1 => "sp".to_string(),
            _ => format!("{fp_prefix}{n}"),
        };
        let ops = [
            Operand::Reg(dest.clone()),
            Operand::Symbol(format!("s{suffix}")),
        ];
        prop_assert!(
            encode_adrp(&ops).is_err(),
            "ADRP takes Xd only; {} must Err (gas/llvm-mc reject it)",
            dest
        );
    }

    // Oracle: negative_error
    // Target: encoder.load_store.encode_adrp
    #[test]
    fn encode_adrp_neg_bad_operands(
        kind in 0u32..=6,
        rd in 0u32..=30,
        extra in extra_operand(),
        imm in prop_oneof![Just(0i64), Just(4096i64), Just(-4096i64), Just(1i64)],
    ) {
        let x = xreg(rd);
        let ops: Vec<Operand> = match kind {
            0 => vec![],
            1 => vec![Operand::Reg(x.clone())],
            2 => vec![
                Operand::Reg(x.clone()),
                Operand::Symbol("foo".into()),
                extra,
            ],
            3 => vec![
                Operand::Reg(x.clone()),
                Operand::Modifier {
                    kind: "lo12".into(),
                    symbol: "foo".into(),
                },
            ],
            4 => vec![
                Operand::Reg(x.clone()),
                Operand::Modifier {
                    kind: "got_lo12".into(),
                    symbol: "foo".into(),
                },
            ],
            5 => vec![Operand::Reg(x.clone()), Operand::Imm(imm)],
            _ => vec![
                Operand::Reg(x.clone()),
                Operand::Mem {
                    base: "x1".into(),
                    offset: 0,
                },
            ],
        };
        prop_assert!(
            encode_adrp(&ops).is_err(),
            "invalid ADRP arity/kind={} must Err, got {:?}",
            kind,
            encode_adrp(&ops)
        );
    }

    // Oracle: algebraic.invariant
    // Target: encoder.load_store.encode_adrp
    #[test]
    fn encode_adrp_symbol_misclassified(
        rd in 0u32..=31,
        which in 0u32..=2,
        name in prop::sample::select(vec!["s1", "v0", "d1", "cc", "lt", "le", "st", "ld"]),
    ) {
        let second = match which {
            0 => Operand::Reg(name.to_string()),
            1 => Operand::Cond(name.to_string()),
            _ => Operand::Barrier(name.to_string()),
        };
        let ops = [Operand::Reg(xreg(rd)), second];
        match encode_adrp(&ops) {
            Ok(EncodeResult::WordWithReloc { word, reloc }) => {
                prop_assert_eq!(word, ADRP_BASE | rd);
                match reloc.reloc_type {
                    RelocType::AdrpPage21 => {}
                    other => {
                        return Err(TestCaseError::fail(format!(
                            "expected AdrpPage21, got {other:?}"
                        )));
                    }
                }
                prop_assert_eq!(&reloc.symbol, name);
                prop_assert_eq!(reloc.addend, 0);
            }
            other => {
                return Err(TestCaseError::fail(format!(
                    "parser-misclassified {name} as operand 1 must be a symbol reloc, got {other:?}"
                )));
            }
        }
    }

    // Oracle: differential (coverage sweep: parse_reg_num aliases lr / uppercase / x31)
    // Target: encoder.load_store.encode_adrp
    #[test]
    fn encode_adrp_diff_alt_spellings(
        which in 0u32..=2,
        n in 0u32..=30,
    ) {
        let (name, rd) = match which {
            0 => ("lr".to_string(), 30u32),
            1 => (format!("X{n}"), n),
            _ => ("x31".to_string(), 31u32),
        };
        let ops = [Operand::Reg(name.clone()), Operand::Symbol("foo".into())];
        let (sut, ty, _, _) = sut_reloc(&ops)
            .unwrap_or_else(|e| panic!("SUT rejected alias {}: {}", name, e));
        prop_assert_eq!(sut, ADRP_BASE | rd, "alias {} word", name);
        match ty {
            RelocType::AdrpPage21 => {}
            other => {
                return Err(TestCaseError::fail(format!(
                    "alias {} expected AdrpPage21, got {:?}",
                    name, other
                )));
            }
        }
    }
}

#[test]
fn test_encode_adrp_regression_w_reg() {
    let ops = [Operand::Reg("w0".into()), Operand::Symbol("foo".into())];
    assert!(
        encode_adrp(&ops).is_err(),
        "adrp w0, foo must Err; ADRP takes Xd only (llvm-mc/gas reject it)"
    );
}

#[test]
fn test_encode_adrp_regression_sp() {
    let ops = [Operand::Reg("sp".into()), Operand::Symbol("foo".into())];
    assert!(
        encode_adrp(&ops).is_err(),
        "adrp sp, foo must Err; ADRP Rd is Xd (X31=XZR, not SP)"
    );
}

#[test]
fn test_encode_adrp_regression_fp() {
    let ops = [Operand::Reg("d0".into()), Operand::Symbol("foo".into())];
    assert!(
        encode_adrp(&ops).is_err(),
        "adrp d0, foo must Err; ADRP takes Xd only (llvm-mc/gas reject FP/SIMD)"
    );
}

#[test]
fn test_encode_adrp_regression_extra_operand() {
    let ops = [
        Operand::Reg("x0".into()),
        Operand::Symbol("foo".into()),
        Operand::Reg("x0".into()),
    ];
    assert!(
        encode_adrp(&ops).is_err(),
        "adrp x0, foo, x0 must Err; extra operand is rejected by gas/llvm-mc"
    );
}

#[test]
fn test_encode_adrp_regression_got_modifier_offset() {
    let ops = [
        Operand::Reg("x0".into()),
        Operand::ModifierOffset {
            kind: "got".into(),
            symbol: "g0".into(),
            offset: 0,
        },
    ];
    match encode_adrp(&ops) {
        Ok(EncodeResult::WordWithReloc { word, reloc }) => {
            assert_eq!(word, 0x9000_0000);
            match reloc.reloc_type {
                RelocType::AdrGotPage21 => {}
                other => panic!("expected AdrGotPage21, got {other:?}"),
            }
            assert_eq!(reloc.symbol, "g0");
            assert_eq!(reloc.addend, 0);
        }
        other => panic!(
            "adrp x0, :got:g0+0 must encode AdrGotPage21 (gas/llvm-mc accept it), got {other:?}"
        ),
    }
}
