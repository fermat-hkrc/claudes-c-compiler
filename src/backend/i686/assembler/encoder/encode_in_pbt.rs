// Oracle: differential — llvm-mc i686 assembler (IN E4/E5/EC/ED + 66h)
// Evidence: system.rs:74 "Encode IN instruction: inb/inw/inl";
//   encoder/mod.rs:363 "inb"|"inw"|"inl" => encode_in;
//   Intel SDM IN: E4 ib / E5 ib / EC / ED; 66h operand-size for AX;
//   x86 sibling system.rs:52-97 documents AT&T inb %dx,%al / $imm8 / (%dx);
//   llvm-mc -triple=i686 -show-encoding.
// Stronger considered:
//   - State machine: rejected — pure encoding, no lifecycle
//   - Algebraic round-trip: rejected — no in-tree i686 decoder for IN
// Differential: candidate=encode_in (via InstructionEncoder::encode),
//   reference=llvm-mc -triple=i686 -show-encoding (LLVM 15),
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=AT&T IN operands <-> llvm-mc encoding bytes.
// Also: algebraic.invariant (fixed opcodes), algebraic.metamorphic (inw=66|inl),
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

const MNEMONICS: &[&str] = &["inb", "inw", "inl"];

/// Canonical data register for each IN size (Intel fixed).
fn data_reg(mnemonic: &str) -> &'static str {
    match mnemonic {
        "inb" => "al",
        "inw" => "ax",
        "inl" => "eax",
        _ => "al",
    }
}

fn intel_dx_bytes(mnemonic: &str) -> Vec<u8> {
    match mnemonic {
        "inb" => vec![0xEC],
        "inw" => vec![0x66, 0xED],
        "inl" => vec![0xED],
        _ => vec![],
    }
}

fn intel_imm_bytes(mnemonic: &str, imm: u8) -> Vec<u8> {
    match mnemonic {
        "inb" => vec![0xE4, imm],
        "inw" => vec![0x66, 0xE5, imm],
        "inl" => vec![0xE5, imm],
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
fn encode_in_kat_llvm_mc_inb_dx() {
    let mc = llvm_mc_bytes("inb %dx, %al").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0xec]);
    let sut = sut_encode("inb", vec![reg("dx"), reg("al")]).expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_in_kat_llvm_mc_inw_dx() {
    let mc = llvm_mc_bytes("inw %dx, %ax").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x66, 0xed]);
    let sut = sut_encode("inw", vec![reg("dx"), reg("ax")]).expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_in_kat_llvm_mc_inl_dx() {
    let mc = llvm_mc_bytes("inl %dx, %eax").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0xed]);
    let sut = sut_encode("inl", vec![reg("dx"), reg("eax")]).expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_in_kat_llvm_mc_inb_imm80() {
    let mc = llvm_mc_bytes("inb $0x80, %al").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0xe4, 0x80]);
    let sut = sut_encode("inb", vec![imm(0x80), reg("al")]).expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_in_kat_llvm_mc_inw_imm() {
    let mc = llvm_mc_bytes("inw $0x42, %ax").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x66, 0xe5, 0x42]);
    let sut = sut_encode("inw", vec![imm(0x42), reg("ax")]).expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_in_kat_llvm_mc_inl_dx_mem() {
    let mc = llvm_mc_bytes("inl (%dx), %eax").expect("llvm-mc KAT (%dx)");
    assert_eq!(mc, vec![0xed]);
    let sut = sut_encode("inl", vec![mem_dx(), reg("eax")]);
    match sut {
        Ok(bytes) => assert_eq!(bytes, mc, "inl (%dx), %eax must match llvm-mc"),
        Err(e) => panic!("SUT erred on valid AT&T (%dx) form: {e}"),
    }
}

proptest! {
    #![proptest_config(cfg())]

    // Oracle: differential — DX port form vs llvm-mc
    // Formal: ∀ m ∈ {inb,inw,inl}. encode_in(m,[%dx,data_reg(m)]) = llvm_mc(...)
    #[test]
    fn encode_in_diff_dx_port(mnemonic in prop::sample::select(MNEMONICS)) {
        let data = data_reg(mnemonic);
        let asm = format!("{mnemonic} %dx, %{data}");
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let sut = sut_encode(mnemonic, vec![reg("dx"), reg(data)])
            .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(sut, mc, "DX-port diff for `{}`", asm);
    }

    // Oracle: differential — imm8 port vs llvm-mc (accepted imm domain)
    // Formal: ∀ m, v. agree_or_dual_reject(sut, llvm)
    #[test]
    fn encode_in_diff_imm8_port(
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
        let asm = format!("{mnemonic} ${imm_v}, %{data}");
        let mc = match llvm_mc_bytes(&asm) {
            Ok(b) => b,
            Err(e) => {
                let sut = sut_encode(mnemonic, vec![imm(imm_v), reg(data)]);
                prop_assert!(
                    sut.is_err(),
                    "llvm-mc rejected `{asm}` ({e}) but SUT returned Ok({sut:?})"
                );
                return Ok(());
            }
        };
        let sut = sut_encode(mnemonic, vec![imm(imm_v), reg(data)])
            .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(sut, mc, "imm8 diff for `{}`", asm);
    }

    // Oracle: algebraic.invariant — fixed Intel opcodes for DX form
    #[test]
    fn encode_in_invariant_opcodes(mnemonic in prop::sample::select(MNEMONICS)) {
        let data = data_reg(mnemonic);
        let bytes = sut_encode(mnemonic, vec![reg("dx"), reg(data)])
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
    fn encode_in_invariant_imm_opcodes(
        mnemonic in prop::sample::select(MNEMONICS),
        port in any::<u8>(),
    ) {
        let data = data_reg(mnemonic);
        let bytes = sut_encode(mnemonic, vec![imm(port as i64), reg(data)])
            .expect("imm8 form must encode");
        prop_assert_eq!(
            bytes,
            intel_imm_bytes(mnemonic, port),
            "imm opcode invariant for {} port={}",
            mnemonic,
            port
        );
    }

    // Oracle: algebraic.metamorphic — inw bytes == [0x66] ++ inl bytes
    #[test]
    fn encode_in_meta_size_prefix(
        use_imm in any::<bool>(),
        port in any::<u8>(),
    ) {
        let (ops_l, ops_w) = if use_imm {
            (
                vec![imm(port as i64), reg("eax")],
                vec![imm(port as i64), reg("ax")],
            )
        } else {
            (vec![reg("dx"), reg("eax")], vec![reg("dx"), reg("ax")])
        };
        let inl = sut_encode("inl", ops_l).expect("inl");
        let inw = sut_encode("inw", ops_w).expect("inw");
        let mut expected = vec![0x66u8];
        expected.extend_from_slice(&inl);
        prop_assert_eq!(inw, expected, "inw must be 66-prefixed inl");
    }

    // Oracle: negative_error — wrong arity (not 0 or 2)
    #[test]
    fn encode_in_neg_arity(
        mnemonic in prop::sample::select(MNEMONICS),
        n in 1usize..6,
    ) {
        prop_assume!(n != 2);
        let data = data_reg(mnemonic);
        let mut ops = Vec::new();
        for i in 0..n {
            if i == 0 {
                ops.push(reg("dx"));
            } else if i == 1 {
                ops.push(reg(data));
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

    // Oracle: negative_error — non-canonical port/data registers must Err
    // (Intel fixed DX + AL/AX/EAX; llvm-mc and gas reject others)
    // AT&T order for IN: port, data  (opposite of OUT)
    #[test]
    fn encode_in_neg_wrong_registers(
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
        // Skip the single valid pair for this mnemonic: %dx, %data
        prop_assume!(!(src == "dx" && dst == data));
        let asm = format!("{mnemonic} %{src}, %{dst}");
        let llvm = llvm_mc_bytes(&asm);
        let sut = sut_encode(mnemonic, vec![reg(src), reg(dst)]);
        match (llvm, sut) {
            (Err(_), Err(_)) => {}
            (Ok(mc), Ok(sut_b)) => {
                prop_assert_eq!(sut_b, mc, "unexpected dual-accept for `{}`", asm);
            }
            (Err(le), Ok(bytes)) => {
                return Err(TestCaseError::fail(format!(
                    "SUT accepted invalid IN `{}` → {:02x?}; llvm-mc rejected: {}",
                    asm, bytes, le
                )));
            }
            (Ok(mc), Err(se)) => {
                return Err(TestCaseError::fail(format!(
                    "SUT rejected valid IN `{}` ({}); llvm-mc → {:02x?}",
                    asm, se, mc
                )));
            }
        }
    }

    // Oracle: negative_error — imm outside byte range that llvm-mc rejects
    #[test]
    fn encode_in_neg_imm_out_of_range(
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
        let asm = format!("{mnemonic} ${imm_v}, %{data}");
        let llvm = llvm_mc_bytes(&asm);
        let sut = sut_encode(mnemonic, vec![imm(imm_v), reg(data)]);
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
    fn encode_in_diff_dx_memory_form(mnemonic in prop::sample::select(MNEMONICS)) {
        let data = data_reg(mnemonic);
        let asm = format!("{mnemonic} (%dx), %{data}");
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let sut = sut_encode(mnemonic, vec![mem_dx(), reg(data)]);
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

    // Oracle: algebraic.metamorphic — DX vs imm opcode families differ
    #[test]
    fn encode_in_meta_dx_vs_imm_families(
        mnemonic in prop::sample::select(MNEMONICS),
        port in any::<u8>(),
    ) {
        let data = data_reg(mnemonic);
        let dx = sut_encode(mnemonic, vec![reg("dx"), reg(data)]).expect("dx");
        let im = sut_encode(mnemonic, vec![imm(port as i64), reg(data)]).expect("imm");
        let dx_op = *dx.last().unwrap();
        prop_assert!(dx_op == 0xEC || dx_op == 0xED, "DX form opcode {dx_op:#x}");
        let imm_op_idx = if mnemonic == "inw" { 1 } else { 0 };
        prop_assert!(im.len() > imm_op_idx);
        let imm_op = im[imm_op_idx];
        prop_assert!(imm_op == 0xE4 || imm_op == 0xE5, "imm form opcode {imm_op:#x}");
        prop_assert_ne!(dx_op, imm_op, "DX and imm forms must differ");
    }
}

// ─── Deterministic regression witnesses ─────────────────────────────────────

#[test]
fn test_encode_in_regression_wrong_reg_al_al() {
    // inb %al, %al is architecturally invalid (port must be DX, data must be AL only with DX).
    let sut = sut_encode("inb", vec![reg("al"), reg("al")]);
    assert!(
        sut.is_err(),
        "inb %al, %al must Err, got Ok({sut:?})"
    );
}

#[test]
fn test_encode_in_regression_imm_256_truncated() {
    // Port immediate 256 is not an imm8; must not encode as E4 00.
    let sut = sut_encode("inb", vec![imm(256), reg("al")]);
    assert!(
        sut.is_err(),
        "inb $256, %al must Err, got Ok({sut:?})"
    );
}

#[test]
fn test_encode_in_regression_dx_mem_form() {
    let sut = sut_encode("inl", vec![mem_dx(), reg("eax")]);
    assert_eq!(
        sut.expect("(%dx) must encode"),
        vec![0xed],
        "inl (%dx), %eax"
    );
}
