// Oracle: differential — llvm-mc RISC-V assembler (LUI U-type word)
// Evidence: src/backend/riscv/assembler/README.md:7 textual assembly from codegen;
//   README.md:304 U-type: lui, auipc;
//   README.md:356 U-type: imm[31:12] | rd | opcode;
//   encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words";
//   encoder/mod.rs:452 "lui" => encode_lui;
//   base.rs:12 "%hi(symbol)"; base.rs:16-18 %tprel_hi -> TprelHi20 else Hi20;
//   RISC-V Unprivileged ISA LUI: opcode=0110111, rd, imm[31:12].
// Stronger considered:
//   - State machine: rejected — encode_lui is a pure function with no lifecycle
//   - Algebraic round-trip via in-tree decoder: rejected — no LUI decoder
//   - encode_auipc / encode_c_lui / encode_u as differential sibling: rejected —
//     same-job gate (AUIPC opcode / compressed C.LUI / private shared packer)
// Weaker available: algebraic.invariant (U-type field unpack), algebraic.metamorphic
//   (ABI vs xN alias), negative_error (oob imm / extra / bad modifier / arity / FP)
// Differential: candidate=encode_lui, reference=llvm-mc -triple=riscv64 -show-encoding,
//   SUT-boundary=internal-helper of RISC-V assembler,
//   mapping=[Reg(rd), Imm(imm)] <-> `lui rd, imm`;
//   reloc-form word (imm=0) <-> `lui rd, 0` plus R_RISCV_HI20 / R_RISCV_TPREL_HI20.

use super::{encode_lui, EncodeResult, RelocType};
use crate::backend::riscv::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;
const OP_LUI: u32 = 0b0110111;

const ABI: [&str; 32] = [
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3",
    "a4", "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11",
    "t3", "t4", "t5", "t6",
];

fn cfg() -> ProptestConfig {
    ProptestConfig::with_cases(CASES)
}

fn xn(n: u32) -> String {
    format!("x{n}")
}

fn abi_name(n: u32) -> &'static str {
    ABI[n as usize]
}

fn reg(name: &str) -> Operand {
    Operand::Reg(name.to_string())
}

/// Unpack U-type per RISC-V unprivileged ISA (not a copy of encode_u).
fn unpack_u(word: u32) -> (u32, u32, u32) {
    let opcode = word & 0x7f;
    let rd = (word >> 7) & 0x1f;
    let imm20 = word >> 12;
    (opcode, rd, imm20)
}

fn sut_word(ops: &[Operand]) -> Result<u32, String> {
    match encode_lui(ops)? {
        EncodeResult::Word(w) => Ok(w),
        other => Err(format!("expected Word, got {other:?}")),
    }
}

fn reloc_kind(t: &RelocType) -> &'static str {
    match t {
        RelocType::Hi20 => "Hi20",
        RelocType::TprelHi20 => "TprelHi20",
        RelocType::PcrelHi20 => "PcrelHi20",
        RelocType::PcrelLo12I => "PcrelLo12I",
        RelocType::Lo12I => "Lo12I",
        RelocType::GotHi20 => "GotHi20",
        other => panic!("unexpected reloc {other:?}"),
    }
}

fn sut_reloc(ops: &[Operand]) -> Result<(u32, &'static str, String, i64), String> {
    match encode_lui(ops)? {
        EncodeResult::WordWithReloc { word, reloc } => {
            Ok((word, reloc_kind(&reloc.reloc_type), reloc.symbol, reloc.addend))
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
        .args(["-triple=riscv64", "-show-encoding"])
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

fn llvm_mc_rejects(asm: &str) -> bool {
    llvm_mc_word(asm).is_err()
}

fn gpr_name() -> impl Strategy<Value = String> {
    prop_oneof![
        (0u32..=31).prop_map(xn),
        (0u32..=31).prop_map(|n| abi_name(n).to_string()),
        Just("fp".into()),
    ]
}

fn imm20() -> impl Strategy<Value = i64> {
    prop_oneof![
        Just(0i64),
        Just(1i64),
        Just(4096i64),
        Just(0x80000i64),
        Just(1048575i64),
        0i64..=1048575,
    ]
}

fn ident() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("foo".into()),
        Just("bar".into()),
        Just("tls_var".into()),
        Just("_x".into()),
        Just("sym0".into()),
        "[A-Za-z_][A-Za-z0-9_]{0,11}",
    ]
}

fn extra_operand() -> impl Strategy<Value = Operand> {
    prop_oneof![
        Just(Operand::Imm(0)),
        Just(Operand::Imm(-1)),
        (0u32..=31).prop_map(|n| Operand::Reg(xn(n))),
        Just(Operand::Symbol("foo".into())),
        Just(Operand::Label("L0".into())),
        Just(Operand::SymbolOffset("foo".into(), 4)),
        Just(Operand::Mem {
            base: "sp".into(),
            offset: 8,
        }),
        Just(Operand::FenceArg("iorw".into())),
        Just(Operand::Csr("mstatus".into())),
        Just(Operand::RoundingMode("rne".into())),
    ]
}

fn oob_imm() -> impl Strategy<Value = i64> {
    prop_oneof![
        Just(-1i64),
        Just(-524288i64),
        Just(1048576i64),
        Just(0x100000i64),
        Just(i64::MIN),
        Just(i64::MAX),
        i64::MIN..=-1,
        1048576i64..=i64::MAX,
    ]
}

fn signed_addend() -> impl Strategy<Value = i64> {
    prop_oneof![
        Just(1i64),
        Just(-1i64),
        Just(4i64),
        Just(-4i64),
        Just(8i64),
        Just(-8i64),
        1i64..=4096,
        -4096i64..=-1,
    ]
}

fn fp_name() -> impl Strategy<Value = String> {
    prop_oneof![
        (0u32..=31).prop_map(|n| format!("f{n}")),
        Just("fa0".into()),
        Just("ft0".into()),
        Just("fs0".into()),
        Just("fa7".into()),
        Just("ft11".into()),
    ]
}

fn short_ops() -> impl Strategy<Value = Vec<Operand>> {
    prop_oneof![
        Just(vec![]),
        gpr_name().prop_map(|rd| vec![reg(&rd)]),
        imm20().prop_map(|i| vec![Operand::Imm(i)]),
    ]
}

fn non_imm_non_symbol() -> impl Strategy<Value = Operand> {
    prop_oneof![
        Just(Operand::Label("L0".into())),
        Just(Operand::Mem {
            base: "sp".into(),
            offset: 0,
        }),
        Just(Operand::FenceArg("iorw".into())),
        Just(Operand::Csr("mstatus".into())),
        Just(Operand::RoundingMode("rne".into())),
        Just(Operand::SymbolOffset("foo".into(), 4)),
        Just(Operand::MemSymbol {
            base: "a0".into(),
            symbol: "foo".into(),
            modifier: "lo".into(),
        }),
    ]
}

fn bad_lui_symbol() -> impl Strategy<Value = String> {
    ident().prop_flat_map(|s| {
        prop_oneof![
            Just(s.clone()),
            Just(format!("%pcrel_hi({s})")),
            Just(format!("%pcrel_lo({s})")),
            Just(format!("%lo({s})")),
            Just(format!("%got_pcrel_hi({s})")),
            Just(format!("%tprel_lo({s})")),
            Just(format!("%tprel_add({s})")),
        ]
    })
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_lui_kat_llvm_mc_x0_0() {
    let want = 0x0000_0037u32;
    let mc = llvm_mc_word("lui x0, 0").expect("llvm-mc KAT x0,0");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("x0"), Operand::Imm(0)]).expect("SUT KAT x0,0");
    assert_eq!(sut, want);
}

#[test]
fn encode_lui_kat_llvm_mc_x1_1() {
    let want = 0x0000_10b7u32;
    let mc = llvm_mc_word("lui x1, 1").expect("llvm-mc KAT x1,1");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("x1"), Operand::Imm(1)]).expect("SUT KAT x1,1");
    assert_eq!(sut, want);
}

#[test]
fn encode_lui_kat_llvm_mc_x1_max() {
    let want = 0xffff_f0b7u32;
    let mc = llvm_mc_word("lui x1, 1048575").expect("llvm-mc KAT max");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word(&[reg("x1"), Operand::Imm(1048575)]).expect("SUT KAT max");
    assert_eq!(sut, want);
}

#[test]
fn encode_lui_kat_hi_foo() {
    let want = 0x0000_00b7u32;
    let mc = llvm_mc_word("lui x1, 0").expect("llvm-mc zero-imm word");
    assert_eq!(mc, want);
    let (word, kind, sym, addend) =
        sut_reloc(&[reg("x1"), Operand::Symbol("%hi(foo)".into())]).expect("SUT %hi KAT");
    assert_eq!(word, want);
    assert_eq!(kind, "Hi20");
    assert_eq!(sym, "foo");
    assert_eq!(addend, 0);
}

#[test]
fn encode_lui_kat_tprel_hi() {
    let want = 0x0000_02b7u32; // lui t0=x5, 0
    let mc = llvm_mc_word("lui t0, 0").expect("llvm-mc t0,0");
    assert_eq!(mc, want);
    let (word, kind, sym, addend) = sut_reloc(&[
        reg("t0"),
        Operand::Symbol("%tprel_hi(x)".into()),
    ])
    .expect("SUT %tprel_hi KAT");
    assert_eq!(word, want);
    assert_eq!(kind, "TprelHi20");
    assert_eq!(sym, "x");
    assert_eq!(addend, 0);
}

/// Regression: out-of-range LUI immediate must be rejected (llvm-mc range [0, 1048575]).
#[test]
fn test_encode_lui_regression_imm_oob() {
    let ops = [reg("x0"), Operand::Imm(-1)];
    assert!(
        encode_lui(&ops).is_err(),
        "lui x0, -1 must Err (llvm-mc range [0, 1048575]); got {:?}",
        encode_lui(&ops)
    );
}

/// Regression: extra operand must be rejected (LUI is two-operand; llvm-mc errors).
#[test]
fn test_encode_lui_regression_extra_operand() {
    let ops = [reg("x0"), Operand::Imm(0), Operand::Imm(0)];
    assert!(
        encode_lui(&ops).is_err(),
        "lui x0, 0 with a third operand must Err; got {:?}",
        encode_lui(&ops)
    );
}

/// Regression: plain symbol without %hi/%tprel_hi must be rejected.
#[test]
fn test_encode_lui_regression_plain_symbol() {
    let ops = [reg("x0"), Operand::Symbol("foo".into())];
    assert!(
        encode_lui(&ops).is_err(),
        "lui x0, foo must Err (llvm-mc requires %hi/%tprel_hi); got {:?}",
        encode_lui(&ops)
    );
}

/// Regression: %hi(sym+addend) must relocate against `sym` with that addend.
#[test]
fn test_encode_lui_regression_hi_addend() {
    let ops = [reg("x0"), Operand::Symbol("%hi(foo+4)".into())];
    match encode_lui(&ops) {
        Ok(EncodeResult::WordWithReloc { reloc, .. }) => {
            assert_eq!(
                reloc.symbol, "foo",
                "%hi(foo+4) symbol must be foo, not {}",
                reloc.symbol
            );
            assert_eq!(reloc.addend, 4, "%hi(foo+4) addend must be 4, not {}", reloc.addend);
        }
        other => panic!("expected WordWithReloc for %hi(foo+4), got {:?}", other),
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_lui_diff_imm_llvm_mc(rd in gpr_name(), imm in imm20()) {
        let asm = format!("lui {}, {}", rd, imm);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&[reg(&rd), Operand::Imm(imm)])
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "SUT {:08x} != llvm-mc {:08x} for {}", sut, mc, asm);
    }

    #[test]
    fn encode_lui_isa_u_type(rd in 0u32..=31u32, imm in imm20()) {
        let w = sut_word(&[reg(&xn(rd)), Operand::Imm(imm)])
            .unwrap_or_else(|e| panic!("SUT rejected x{}, {}: {}", rd, imm, e));
        let (opc, got_rd, imm20) = unpack_u(w);
        prop_assert_eq!(opc, OP_LUI, "opcode");
        prop_assert_eq!(got_rd, rd, "rd");
        prop_assert_eq!(imm20, imm as u32, "imm20");
    }

    #[test]
    fn encode_lui_abi_xn_alias(n in 0u32..=31u32, imm in imm20()) {
        let via_x = sut_word(&[reg(&xn(n)), Operand::Imm(imm)])
            .unwrap_or_else(|e| panic!("xN rejected: {}", e));
        let via_abi = sut_word(&[reg(abi_name(n)), Operand::Imm(imm)])
            .unwrap_or_else(|e| panic!("ABI rejected: {}", e));
        prop_assert_eq!(via_abi, via_x);
        if n == 8 {
            let via_fp = sut_word(&[reg("fp"), Operand::Imm(imm)])
                .unwrap_or_else(|e| panic!("fp rejected: {}", e));
            prop_assert_eq!(via_fp, via_x);
        }
    }

    #[test]
    fn encode_lui_reloc_hi(rd in gpr_name(), sym in ident()) {
        let zero_asm = format!("lui {}, 0", rd);
        let mc = llvm_mc_word(&zero_asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected {}: {}", zero_asm, e));
        let ops = [reg(&rd), Operand::Symbol(format!("%hi({sym})"))];
        let (word, kind, got_sym, addend) = sut_reloc(&ops)
            .unwrap_or_else(|e| panic!("SUT rejected lui {}, %hi({}): {}", rd, sym, e));
        prop_assert_eq!(word, mc);
        prop_assert_eq!(kind, "Hi20");
        prop_assert_eq!(got_sym, sym);
        prop_assert_eq!(addend, 0i64);
    }

    #[test]
    fn encode_lui_reloc_tprel_hi(rd in gpr_name(), sym in ident()) {
        let zero_asm = format!("lui {}, 0", rd);
        let mc = llvm_mc_word(&zero_asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected {}: {}", zero_asm, e));
        let ops = [reg(&rd), Operand::Symbol(format!("%tprel_hi({sym})"))];
        let (word, kind, got_sym, addend) = sut_reloc(&ops)
            .unwrap_or_else(|e| panic!("SUT rejected lui {}, %tprel_hi({}): {}", rd, sym, e));
        prop_assert_eq!(word, mc);
        prop_assert_eq!(kind, "TprelHi20");
        prop_assert_eq!(got_sym, sym);
        prop_assert_eq!(addend, 0i64);
    }

    #[test]
    fn encode_lui_neg_imm_oob(rd in gpr_name(), imm in oob_imm()) {
        prop_assume!(!(0..=1048575).contains(&imm));
        let asm = format!("lui {}, {}", rd, imm);
        prop_assert!(
            llvm_mc_rejects(&asm),
            "reference unexpectedly accepted {}",
            asm
        );
        let ops = [reg(&rd), Operand::Imm(imm)];
        prop_assert!(
            encode_lui(&ops).is_err(),
            "oob imm {} must Err (llvm-mc range [0, 1048575]); got {:?}",
            imm,
            encode_lui(&ops)
        );
    }

    #[test]
    fn encode_lui_neg_extra(rd in gpr_name(), imm in imm20(), extra in extra_operand()) {
        let asm = format!("lui {}, {}", rd, imm);
        let ops = vec![reg(&rd), Operand::Imm(imm), extra];
        prop_assert!(
            encode_lui(&ops).is_err(),
            "extra operand must Err for {} (llvm-mc rejects extra operands); got {:?}",
            asm,
            encode_lui(&ops)
        );
    }

    #[test]
    fn encode_lui_neg_bad_modifier(rd in gpr_name(), s in bad_lui_symbol()) {
        let asm = format!("lui {}, {}", rd, s);
        prop_assert!(
            llvm_mc_rejects(&asm),
            "reference unexpectedly accepted {}",
            asm
        );
        let ops = [reg(&rd), Operand::Symbol(s.clone())];
        prop_assert!(
            encode_lui(&ops).is_err(),
            "modifier/symbol {} must Err (llvm-mc requires %hi/%tprel_hi or [0,1048575]); got {:?}",
            s,
            encode_lui(&ops)
        );
    }

    #[test]
    fn encode_lui_neg_arity(ops in short_ops()) {
        prop_assume!(ops.len() < 2);
        prop_assert!(
            encode_lui(&ops).is_err(),
            "arity {} must Err, got {:?}",
            ops.len(),
            encode_lui(&ops)
        );
    }

    #[test]
    fn encode_lui_neg_fp(fp in fp_name(), imm in imm20()) {
        let ops = [reg(&fp), Operand::Imm(imm)];
        prop_assert!(
            encode_lui(&ops).is_err(),
            "FP dest {} must Err (llvm-mc invalid operand); got {:?}",
            fp,
            encode_lui(&ops)
        );
    }

    #[test]
    fn encode_lui_neg_bad_operand(rd in gpr_name(), bad in non_imm_non_symbol()) {
        let ops = [reg(&rd), bad];
        prop_assert!(
            encode_lui(&ops).is_err(),
            "non-imm/non-symbol operand 1 must Err; got {:?}",
            encode_lui(&ops)
        );
    }

    #[test]
    fn encode_lui_hi_addend(rd in gpr_name(), sym in ident(), addend in signed_addend()) {
        prop_assume!(addend != 0);
        let inner = if addend >= 0 {
            format!("{sym}+{addend}")
        } else {
            format!("{sym}{addend}")
        };
        let ops = [reg(&rd), Operand::Symbol(format!("%hi({inner})"))];
        let (_word, kind, got_sym, got_add) = sut_reloc(&ops)
            .unwrap_or_else(|e| panic!("SUT rejected %hi({}): {}", inner, e));
        prop_assert_eq!(kind, "Hi20");
        prop_assert_eq!(got_sym, sym, "reloc symbol must be the identifier, not the %hi inner text");
        prop_assert_eq!(got_add, addend, "reloc addend must preserve %hi(sym+N)");
    }
}
