// Oracle: differential — llvm-mc i686 assembler (OUT E6/E7/EE/EF + 66h)
// Evidence: system.rs:38 "Encode OUT instruction: outb/outw/outl";
//   encoder/mod.rs:360 "outb"|"outw"|"outl" => encode_out;
//   Intel SDM OUT: E6 ib / E7 ib / EE / EF; 66h operand-size for AX;
//   x86 sibling system.rs:4-49 documents AT&T outb %al,%dx / $imm8 / (%dx);
//   llvm-mc -triple=i686 -show-encoding.
// Stronger considered:
//   - State machine: rejected — pure encoding, no lifecycle
//   - Algebraic round-trip: rejected — no in-tree i686 decoder for OUT
// Differential: candidate=encode_out (via InstructionEncoder::encode),
//   reference=llvm-mc -triple=i686 -show-encoding (LLVM 15),
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=AT&T OUT operands <-> llvm-mc encoding bytes.
// Also: algebraic.invariant (fixed opcodes), algebraic.metamorphic (outw=66|outl),
//   negative_error (arity, wrong regs, imm out of range).

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

const MNEMONICS: &[&str] = &["outb", "outw", "outl"];

/// Canonical data register for each OUT size (Intel fixed).
fn data_reg(mnemonic: &str) -> &'static str {
    match mnemonic {
        "outb" => "al",
        "outw" => "ax",
        "outl" => "eax",
        _ => "al",
    }
}

fn intel_dx_bytes(mnemonic: &str) -> Vec<u8> {
    match mnemonic {
        "outb" => vec![0xEE],
        "outw" => vec![0x66, 0xEF],
        "outl" => vec![0xEF],
        _ => vec![],
    }
}

fn intel_imm_bytes(mnemonic: &str, imm: u8) -> Vec<u8> {
    match mnemonic {
        "outb" => vec![0xE6, imm],
        "outw" => vec![0x66, 0xE7, imm],
        "outl" => vec![0xE7, imm],
        _ => vec![],
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

fn reg(name: &str) -> Operand {
    Operand::Register(Register::new(name))
}

fn imm(v: i64) -> Operand {
    Operand::Immediate(ImmediateValue::Integer(v))
}

fn mem_dx() -> Operand {
    Operand::Memory(MemoryOperand {
        segment: None,
        displacement: Displacement::None,
        base: Some(Register::new("dx")),
        index: None,
        scale: None,
    })
}

// ─── KAT gate (reference oracle prerequisite) ───────────────────────────────

#[test]
fn encode_out_kat_llvm_mc_outb_dx() {
    let mc = llvm_mc_bytes("outb %al, %dx").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0xee]);
    let sut = sut_encode("outb", vec![reg("al"), reg("dx")]).expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_out_kat_llvm_mc_outw_dx() {
    let mc = llvm_mc_bytes("outw %ax, %dx").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x66, 0xef]);
    let sut = sut_encode("outw", vec![reg("ax"), reg("dx")]).expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_out_kat_llvm_mc_outl_dx() {
    let mc = llvm_mc_bytes("outl %eax, %dx").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0xef]);
    let sut = sut_encode("outl", vec![reg("eax"), reg("dx")]).expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_out_kat_llvm_mc_outb_imm80() {
    let mc = llvm_mc_bytes("outb %al, $0x80").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0xe6, 0x80]);
    let sut = sut_encode("outb", vec![reg("al"), imm(0x80)]).expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_out_kat_llvm_mc_outw_imm() {
    let mc = llvm_mc_bytes("outw %ax, $0x42").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x66, 0xe7, 0x42]);
    let sut = sut_encode("outw", vec![reg("ax"), imm(0x42)]).expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_out_kat_llvm_mc_outl_dx_mem() {
    let mc = llvm_mc_bytes("outl %eax, (%dx)").expect("llvm-mc KAT (%dx)");
    assert_eq!(mc, vec![0xef]);
    let sut = sut_encode("outl", vec![reg("eax"), mem_dx()]);
    // Expect agreement; failure is SUT missing (%dx) form.
    match sut {
        Ok(bytes) => assert_eq!(bytes, mc, "outl %eax, (%dx) must match llvm-mc"),
        Err(e) => panic!("SUT erred on valid AT&T (%dx) form: {e}"),
    }
}

proptest! {
    #![proptest_config(cfg())]

    // Oracle: differential — DX port form vs llvm-mc
    #[test]
    fn encode_out_diff_dx_port(mnemonic in prop::sample::select(MNEMONICS)) {
        let data = data_reg(mnemonic);
        let asm = format!("{mnemonic} %{data}, %dx");
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let sut = sut_encode(mnemonic, vec![reg(data), reg("dx")])
            .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(sut, mc, "DX-port diff for `{}`", asm);
    }

    // Oracle: differential — imm8 port vs llvm-mc (accepted imm domain)
    #[test]
    fn encode_out_diff_imm8_port(
        mnemonic in prop::sample::select(MNEMONICS),
        imm_v in prop_oneof![
            Just(0i64),
            Just(1i64),
            Just(0x7fi64),
            Just(0x80i64),
            Just(0xffi64),
            Just(-1i64),
            Just(-128i64),
            (0i64..=255i64),
            (-128i64..=-1i64),
        ],
    ) {
        let data = data_reg(mnemonic);
        let asm = format!("{mnemonic} %{data}, ${imm_v}");
        let mc = match llvm_mc_bytes(&asm) {
            Ok(b) => b,
            Err(e) => {
                // If llvm rejects, SUT must also reject (not silently truncate).
                let sut = sut_encode(mnemonic, vec![reg(data), imm(imm_v)]);
                prop_assert!(
                    sut.is_err(),
                    "llvm-mc rejected `{asm}` ({e}) but SUT returned Ok({sut:?})"
                );
                return Ok(());
            }
        };
        let sut = sut_encode(mnemonic, vec![reg(data), imm(imm_v)])
            .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(sut, mc, "imm8 diff for `{}`", asm);
    }

    // Oracle: algebraic.invariant — fixed Intel opcodes for DX form
    #[test]
    fn encode_out_invariant_opcodes(mnemonic in prop::sample::select(MNEMONICS)) {
        let data = data_reg(mnemonic);
        let bytes = sut_encode(mnemonic, vec![reg(data), reg("dx")])
            .expect("canonical DX form must encode");
        prop_assert_eq!(
            bytes,
            intel_dx_bytes(mnemonic),
            "opcode invariant for {}",
            mnemonic
        );
    }

    // Oracle: algebraic.invariant — imm8 opcode table
    #[test]
    fn encode_out_invariant_imm_opcodes(
        mnemonic in prop::sample::select(MNEMONICS),
        port in any::<u8>(),
    ) {
        let data = data_reg(mnemonic);
        let bytes = sut_encode(mnemonic, vec![reg(data), imm(port as i64)])
            .expect("imm8 form must encode");
        prop_assert_eq!(
            bytes,
            intel_imm_bytes(mnemonic, port),
            "imm opcode invariant for {} port={}",
            mnemonic,
            port
        );
    }

    // Oracle: algebraic.metamorphic — outw bytes == [0x66] ++ outl bytes (same ops shape)
    #[test]
    fn encode_out_meta_size_prefix(
        use_imm in any::<bool>(),
        port in any::<u8>(),
    ) {
        let (ops_l, ops_w) = if use_imm {
            (
                vec![reg("eax"), imm(port as i64)],
                vec![reg("ax"), imm(port as i64)],
            )
        } else {
            (vec![reg("eax"), reg("dx")], vec![reg("ax"), reg("dx")])
        };
        let outl = sut_encode("outl", ops_l).expect("outl");
        let outw = sut_encode("outw", ops_w).expect("outw");
        let mut expected = vec![0x66u8];
        expected.extend_from_slice(&outl);
        prop_assert_eq!(outw, expected, "outw must be 66-prefixed outl");
    }

    // Oracle: negative_error — wrong arity (not 0 or 2)
    #[test]
    fn encode_out_neg_arity(
        mnemonic in prop::sample::select(MNEMONICS),
        n in 1usize..6,
    ) {
        prop_assume!(n != 2);
        let data = data_reg(mnemonic);
        let mut ops = Vec::new();
        for i in 0..n {
            if i == 0 {
                ops.push(reg(data));
            } else if i == 1 {
                ops.push(reg("dx"));
            } else {
                ops.push(imm(i as i64));
            }
        }
        let err = sut_encode(mnemonic, ops).expect_err("arity must Err");
        prop_assert!(
            err.contains("requires 0 or 2") || err.contains("requires"),
            "unexpected err: {err}"
        );
    }

    // Oracle: negative_error — non-canonical data/port registers must Err
    // (Intel fixed AL/AX/EAX + DX; llvm-mc and gas reject others)
    #[test]
    fn encode_out_neg_wrong_registers(
        mnemonic in prop::sample::select(MNEMONICS),
        src in prop::sample::select(&[
            "al", "ax", "eax", "bl", "bx", "ebx", "cl", "cx", "ecx",
            "dl", "dx", "edx", "sil", "si", "esi", "dil", "di", "edi",
            "spl", "sp", "esp", "bpl", "bp", "ebp",
        ]),
        dst in prop::sample::select(&[
            "al", "ax", "eax", "bl", "bx", "ebx", "cl", "cx", "ecx",
            "dl", "dx", "edx", "si", "esi", "di", "edi", "sp", "esp", "bp", "ebp",
        ]),
    ) {
        let data = data_reg(mnemonic);
        // Skip the single valid pair for this mnemonic.
        prop_assume!(!(src == data && dst == "dx"));
        let asm = format!("{mnemonic} %{src}, %{dst}");
        // llvm-mc is the independent rejection oracle when it can parse the regs.
        let llvm = llvm_mc_bytes(&asm);
        let sut = sut_encode(mnemonic, vec![reg(src), reg(dst)]);
        match (llvm, sut) {
            (Err(_), Err(_)) => {} // both reject — good
            (Ok(mc), Ok(sut_b)) => {
                // Both accept: only OK if encodings agree AND pair is architecturally valid.
                // Valid pair already excluded by assume; any other accept is a bug if llvm somehow accepts.
                prop_assert_eq!(sut_b, mc, "unexpected dual-accept for `{}`", asm);
            }
            (Err(le), Ok(bytes)) => {
                return Err(TestCaseError::fail(format!(
                    "SUT accepted invalid OUT `{}` → {:02x?}; llvm-mc rejected: {}",
                    asm, bytes, le
                )));
            }
            (Ok(mc), Err(se)) => {
                return Err(TestCaseError::fail(format!(
                    "SUT rejected valid OUT `{}` ({}); llvm-mc → {:02x?}",
                    asm, se, mc
                )));
            }
        }
    }

    // Oracle: negative_error — imm outside byte range that llvm-mc rejects
    #[test]
    fn encode_out_neg_imm_out_of_range(
        mnemonic in prop::sample::select(MNEMONICS),
        imm_v in prop_oneof![
            Just(256i64),
            Just(257i64),
            Just(0x100i64),
            Just(0x1234i64),
            Just(0xffffi64),
            Just(0x1_0000i64),
            Just(-129i64),
            Just(-256i64),
            Just(-1000i64),
            (256i64..=4096i64),
            (-4096i64..=-129i64),
        ],
    ) {
        let data = data_reg(mnemonic);
        let asm = format!("{mnemonic} %{data}, ${imm_v}");
        let llvm = llvm_mc_bytes(&asm);
        let sut = sut_encode(mnemonic, vec![reg(data), imm(imm_v)]);
        match (llvm, sut) {
            (Err(_), Err(_)) => {}
            (Ok(mc), Ok(b)) => prop_assert_eq!(b, mc),
            (Err(le), Ok(bytes)) => {
                return Err(TestCaseError::fail(format!(
                    "SUT silently encoded out-of-range port `{}` → {:02x?} (truncated?); llvm-mc rejected: {}",
                    asm, bytes, le
                )));
            }
            (Ok(mc), Err(se)) => {
                return Err(TestCaseError::fail(format!(
                    "SUT rejected imm llvm accepts `{}` ({}); mc={:02x?}",
                    asm, se, mc
                )));
            }
        }
    }

    // Oracle: differential — AT&T (%dx) memory port form
    #[test]
    fn encode_out_diff_dx_memory_form(mnemonic in prop::sample::select(MNEMONICS)) {
        let data = data_reg(mnemonic);
        let asm = format!("{mnemonic} %{data}, (%dx)");
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let sut = sut_encode(mnemonic, vec![reg(data), mem_dx()]);
        match sut {
            Ok(bytes) => prop_assert_eq!(bytes, mc, "(%dx) diff for `{}`", asm),
            Err(e) => {
                return Err(TestCaseError::fail(format!(
                    "SUT rejected valid AT&T form `{}`: {}; llvm-mc={:02x?}",
                    asm, e, mc
                )));
            }
        }
    }

    // Oracle: algebraic.metamorphic — DX form opcode independent of ignored-reg bug surface:
    // when both src and dst are the canonical pair, imm and dx forms stay in their opcode families.
    #[test]
    fn encode_out_meta_dx_vs_imm_families(
        mnemonic in prop::sample::select(MNEMONICS),
        port in any::<u8>(),
    ) {
        let data = data_reg(mnemonic);
        let dx = sut_encode(mnemonic, vec![reg(data), reg("dx")]).expect("dx");
        let im = sut_encode(mnemonic, vec![reg(data), imm(port as i64)]).expect("imm");
        // DX family: EE or 66 EF or EF — never E6/E7
        let dx_op = *dx.last().unwrap();
        prop_assert!(dx_op == 0xEE || dx_op == 0xEF, "DX form opcode {dx_op:#x}");
        let imm_op_idx = if mnemonic == "outw" { 1 } else { 0 };
        prop_assert!(im.len() > imm_op_idx);
        let imm_op = im[imm_op_idx];
        prop_assert!(imm_op == 0xE6 || imm_op == 0xE7, "imm form opcode {imm_op:#x}");
        prop_assert_ne!(dx_op, imm_op, "DX and imm forms must differ");
    }
}

// ─── Deterministic regression witnesses (filled after triage) ───────────────

#[test]
fn test_encode_out_regression_wrong_reg_bl_dx() {
    // outb %bl, %dx is architecturally invalid (data must be AL).
    let sut = sut_encode("outb", vec![reg("bl"), reg("dx")]);
    assert!(
        sut.is_err(),
        "outb %bl, %dx must Err, got Ok({sut:?})"
    );
}

#[test]
fn test_encode_out_regression_imm_256_truncated() {
    // Port immediate 256 is not an imm8; must not encode as E6 00.
    let sut = sut_encode("outb", vec![reg("al"), imm(256)]);
    assert!(
        sut.is_err(),
        "outb %al, $256 must Err, got Ok({sut:?})"
    );
}

#[test]
fn test_encode_out_regression_dx_mem_form() {
    let sut = sut_encode("outl", vec![reg("eax"), mem_dx()]);
    assert_eq!(
        sut.expect("(%dx) must encode"),
        vec![0xef],
        "outl %eax, (%dx)"
    );
}
