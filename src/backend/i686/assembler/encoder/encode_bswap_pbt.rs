// Oracle: differential — llvm-mc i686 assembler (BSWAP r32)
// Evidence: gp_integer.rs:945 encode_bswap; mod.rs:262 "bswapl" | "bswap" => encode_bswap;
//   Intel SDM Vol.2 BSWAP — Opcode 0F C8+rd, r32 only (16-bit form undefined);
//   registers.rs:4 reg_num aliases r16/r8/xmm/mm/st to the same 3-bit numbers.
// Stronger considered:
//   - State machine: rejected — pure encoding, no lifecycle
//   - Algebraic round-trip: rejected — no in-tree i686 decoder for bswap
// Differential: candidate=encode_bswap (via InstructionEncoder::encode bswapl/bswap),
//   reference=llvm-mc -triple=i686 -show-encoding (LLVM 15),
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=AT&T operand bytes <-> llvm-mc encoding bytes.
// Also: algebraic.invariant (0F C8+n), algebraic.metamorphic (bswap≡bswapl),
//   negative_error (arity / non-reg / wrong width / non-GP).

use super::InstructionEncoder;
use crate::backend::x86::assembler::parser::{
    Displacement, ImmediateValue, Instruction, MemoryOperand, Operand, Register,
};
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;

fn cfg() -> ProptestConfig {
    ProptestConfig::with_cases(CASES)
}

const GP32: &[&str] = &["eax", "ecx", "edx", "ebx", "esp", "ebp", "esi", "edi"];
const BAD_WIDTH: &[&str] = &[
    "ax", "cx", "dx", "bx", "sp", "bp", "si", "di",
    "al", "cl", "dl", "bl", "ah", "ch", "dh", "bh",
];
const MNEMS: &[&str] = &["bswapl", "bswap"];
// Names that reg_num aliases onto GP numbers (registers.rs) — must still be rejected for BSWAP.
const NON_GP_ALIASED: &[&str] = &[
    "xmm0", "xmm1", "xmm7", "mm0", "mm3", "st", "st(0)", "st(1)", "ymm0", "ymm7",
];
// Names reg_num does not know — SUT correctly returns Err("bad register").
const NON_GP_UNKNOWN: &[&str] = &[
    "es", "cs", "ss", "ds", "fs", "gs", "cr0", "cr2", "cr3", "cr4",
];

fn reg_num(name: &str) -> u8 {
    match name {
        "eax" | "ax" | "al" | "xmm0" | "mm0" | "st" | "st(0)" => 0,
        "ecx" | "cx" | "cl" | "xmm1" | "mm1" | "st(1)" => 1,
        "edx" | "dx" | "dl" | "xmm2" | "mm2" | "st(2)" => 2,
        "ebx" | "bx" | "bl" | "xmm3" | "mm3" | "st(3)" => 3,
        "esp" | "sp" | "ah" | "xmm4" | "mm4" | "st(4)" => 4,
        "ebp" | "bp" | "ch" | "xmm5" | "mm5" | "st(5)" => 5,
        "esi" | "si" | "dh" | "xmm6" | "mm6" | "st(6)" => 6,
        "edi" | "di" | "bh" | "xmm7" | "mm7" | "st(7)" => 7,
        _ => panic!("bad reg {name}"),
    }
}

fn sut_encode(mnemonic: &str, ops: Vec<Operand>) -> Result<Vec<u8>, String> {
    let mut enc = InstructionEncoder::new();
    enc.encode(&Instruction {
        prefix: None,
        mnemonic: mnemonic.to_string(),
        operands: ops,
    })?;
    Ok(enc.bytes)
}

fn parse_llvm_bytes(stdout: &str) -> Result<Vec<u8>, String> {
    let marker = "encoding: [";
    let start = stdout
        .find(marker)
        .ok_or_else(|| format!("no encoding in llvm-mc output: {stdout}"))?
        + marker.len();
    let end = stdout[start..]
        .find(']')
        .ok_or_else(|| format!("no closing bracket in: {stdout}"))?
        + start;
    let body = &stdout[start..end];
    let mut bytes = Vec::new();
    for part in body.split(',') {
        let t = part.trim();
        if t.is_empty() {
            continue;
        }
        let v = u8::from_str_radix(t.trim_start_matches("0x"), 16)
            .map_err(|e| format!("bad byte {t}: {e}"))?;
        bytes.push(v);
    }
    if bytes.is_empty() {
        return Err(format!("empty encoding: {stdout}"));
    }
    Ok(bytes)
}

fn llvm_mc_bytes(asm: &str) -> Result<Vec<u8>, String> {
    let mut child = Command::new(LLVM_MC)
        .args(["-triple=i686", "-show-encoding"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("spawn llvm-mc: {e}"))?;
    {
        let mut stdin = child.stdin.take().ok_or("llvm-mc stdin")?;
        writeln!(stdin, "{asm}").map_err(|e| format!("write llvm-mc: {e}"))?;
    }
    let out = child
        .wait_with_output()
        .map_err(|e| format!("wait llvm-mc: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "llvm-mc error: {}",
            String::from_utf8_lossy(&out.stderr)
        ));
    }
    parse_llvm_bytes(&String::from_utf8_lossy(&out.stdout))
}

fn mem_base(base: &str) -> MemoryOperand {
    MemoryOperand {
        segment: None,
        displacement: Displacement::None,
        base: Some(Register::new(base)),
        index: None,
        scale: None,
    }
}

// --- KAT gate (reference/differential prerequisite) ---

#[test]
fn encode_bswap_kat_llvm_mc_eax() {
    let mc = llvm_mc_bytes("bswapl %eax").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x0f, 0xc8], "llvm-mc KAT mapping broken");
    let sut = sut_encode("bswapl", vec![Operand::Register(Register::new("eax"))])
        .expect("SUT KAT");
    assert_eq!(sut, mc, "SUT KAT bswapl %eax");
}

#[test]
fn encode_bswap_kat_llvm_mc_ebx() {
    let mc = llvm_mc_bytes("bswapl %ebx").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x0f, 0xcb]);
    let sut = sut_encode("bswapl", vec![Operand::Register(Register::new("ebx"))])
        .expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_bswap_kat_llvm_mc_edi_unsuffixed() {
    let mc = llvm_mc_bytes("bswap %edi").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x0f, 0xcf]);
    let sut = sut_encode("bswap", vec![Operand::Register(Register::new("edi"))]).expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_bswap_kat_llvm_mc_esp() {
    let mc = llvm_mc_bytes("bswapl %esp").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x0f, 0xcc]);
    let sut = sut_encode("bswapl", vec![Operand::Register(Register::new("esp"))])
        .expect("SUT");
    assert_eq!(sut, mc);
}

proptest! {
    #![proptest_config(cfg())]

    // Oracle: differential — bswapl/bswap r32 vs llvm-mc
    #[test]
    fn encode_bswap_diff_r32_llvm_mc(
        r in prop::sample::select(GP32),
        m in prop::sample::select(MNEMS),
    ) {
        let asm = format!("{m} %{r}");
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let sut = sut_encode(m, vec![Operand::Register(Register::new(r))])
            .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(sut, mc, "diff vs llvm-mc for {}", asm);
    }

    // Oracle: algebraic.invariant — exactly 0F C8+n
    #[test]
    fn encode_bswap_invariant_opcode(r in prop::sample::select(GP32)) {
        let expected = vec![0x0Fu8, 0xC8 + reg_num(r)];
        let sut = sut_encode("bswapl", vec![Operand::Register(Register::new(r))])
            .unwrap_or_else(|e| panic!("SUT rejected bswapl %{r}: {e}"));
        prop_assert_eq!(sut, expected, "invariant 0F C8+n for {}", r);
    }

    // Oracle: algebraic.metamorphic — bswap ≡ bswapl
    #[test]
    fn encode_bswap_meta_bswap_eq_bswapl(r in prop::sample::select(GP32)) {
        let a = sut_encode("bswap", vec![Operand::Register(Register::new(r))])
            .unwrap_or_else(|e| panic!("bswap %{r}: {e}"));
        let b = sut_encode("bswapl", vec![Operand::Register(Register::new(r))])
            .unwrap_or_else(|e| panic!("bswapl %{r}: {e}"));
        prop_assert_eq!(a, b, "bswap ≡ bswapl for {}", r);
    }

    // Oracle: negative_error — arity ≠ 1
    #[test]
    fn encode_bswap_neg_arity(
        n in 0usize..5,
        r in prop::sample::select(GP32),
    ) {
        prop_assume!(n != 1);
        let ops: Vec<Operand> = (0..n)
            .map(|_| Operand::Register(Register::new(r)))
            .collect();
        let err = sut_encode("bswapl", ops).expect_err("arity ≠ 1 must Err");
        prop_assert!(
            err.contains("bswap requires 1 operand") || err.contains("operand"),
            "unexpected err text: {err}"
        );
    }

    // Oracle: negative_error — non-register operand
    #[test]
    fn encode_bswap_neg_non_register(kind in 0u8..3) {
        let op = match kind {
            0 => Operand::Memory(mem_base("eax")),
            1 => Operand::Immediate(ImmediateValue::Integer(0)),
            _ => Operand::Label("sym".into()),
        };
        let err = sut_encode("bswapl", vec![op]).expect_err("non-reg must Err");
        prop_assert!(
            err.contains("register") || err.contains("bswap"),
            "unexpected err: {err}"
        );
    }

    // Oracle: negative_error — r16/r8 must be rejected (Intel BSWAP r32-only)
    #[test]
    fn encode_bswap_neg_wrong_width(
        r in prop::sample::select(BAD_WIDTH),
        m in prop::sample::select(MNEMS),
    ) {
        let asm = format!("{m} %{r}");
        // llvm-mc must reject (establishes independent contract)
        let mc = llvm_mc_bytes(&asm);
        prop_assert!(mc.is_err(), "expected llvm-mc to reject `{asm}`");
        match sut_encode(m, vec![Operand::Register(Register::new(r))]) {
            Err(_) => {}
            Ok(bytes) => {
                return Err(TestCaseError::fail(format!(
                    "SUT accepted invalid-width `{asm}` → {bytes:02x?}; \
                     BSWAP is r32-only (Intel SDM; llvm-mc rejects)"
                )));
            }
        }
    }

    // Oracle: negative_error — non-GP names aliased by reg_num must be rejected
    #[test]
    fn encode_bswap_neg_non_gp(r in prop::sample::select(NON_GP_ALIASED)) {
        let asm = format!("bswapl %{r}");
        let mc = llvm_mc_bytes(&asm);
        prop_assert!(mc.is_err(), "expected llvm-mc to reject `{asm}`");
        match sut_encode("bswapl", vec![Operand::Register(Register::new(r))]) {
            Err(_) => {}
            Ok(bytes) => {
                return Err(TestCaseError::fail(format!(
                    "SUT accepted non-GP `{asm}` → {bytes:02x?}; \
                     BSWAP requires GP r32 (Intel SDM; reg_num must not alias xmm/mm/st/ymm)"
                )));
            }
        }
    }

    // Strengthen: sreg/cr (unknown to reg_num) already Err — keep green as regression of that path
    #[test]
    fn encode_bswap_neg_unknown_reg(r in prop::sample::select(NON_GP_UNKNOWN)) {
        let err = sut_encode("bswapl", vec![Operand::Register(Register::new(r))])
            .expect_err("sreg/cr must Err via bad register");
        prop_assert!(
            err.contains("bad register") || err.contains("register"),
            "unexpected err for %{r}: {err}"
        );
    }
}

/// Deterministic regression: r16 must be rejected (BSWAP is r32-only).
#[test]
fn test_encode_bswap_regression_rejects_ax() {
    let result = sut_encode("bswapl", vec![Operand::Register(Register::new("ax"))]);
    assert!(
        result.is_err(),
        "bswapl %ax must Err (r32 only); got Ok({result:?})"
    );
}

/// Deterministic regression: r8 must be rejected (BSWAP is r32-only).
#[test]
fn test_encode_bswap_regression_rejects_al() {
    let result = sut_encode("bswapl", vec![Operand::Register(Register::new("al"))]);
    assert!(
        result.is_err(),
        "bswapl %al must Err (r32 only); got Ok({result:?})"
    );
}

/// Deterministic regression: xmm must not alias to GP BSWAP encoding.
#[test]
fn test_encode_bswap_regression_rejects_xmm0() {
    let result = sut_encode("bswapl", vec![Operand::Register(Register::new("xmm0"))]);
    assert!(
        result.is_err(),
        "bswapl %xmm0 must Err (GP r32 only); got Ok({result:?})"
    );
}
