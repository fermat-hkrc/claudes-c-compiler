// Oracle: differential — llvm-mc i686 assembler (PREFETCHW 0F 0D /1)
// Evidence: system.rs:24 "Encode prefetchw (0F 0D /1)";
//   encoder/mod.rs:746 "prefetchw" => encode_prefetch_0f0d(ops, 1);
//   Intel/AMD PREFETCHW: 0F 0D /1 + ModR/M memory;
//   sibling x86 encode_sse_mem_only(ops, &[0x0F, 0x0D], 1) emits segment
//   prefix before opcode; i686 core.rs:31-42 emit_segment_prefix for fs/gs/…
// Stronger considered:
//   - State machine: rejected — pure encoding, no lifecycle
//   - Algebraic round-trip: rejected — no in-tree i686 decoder for prefetchw
// Differential: candidate=encode_prefetch_0f0d (via InstructionEncoder::encode),
//   reference=llvm-mc -triple=i686 -show-encoding (LLVM 15),
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=AT&T memory operand bytes <-> llvm-mc encoding bytes.
// Also: algebraic.invariant (opcode/hint), algebraic.metamorphic (opcode stable),
//   negative_error (arity + non-memory).

use super::InstructionEncoder;
use crate::backend::x86::assembler::parser::{
    Displacement, ImmediateValue, Instruction, MemoryOperand, Operand, Register,
};
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;
const MNEMONIC: &str = "prefetchw";
/// Dispatch hardcodes hint=1 (mod.rs:746).
const HINT: u8 = 1;

fn cfg() -> ProptestConfig {
    ProptestConfig::with_cases(CASES)
}

const GP_REGS: &[&str] = &["eax", "ecx", "edx", "ebx", "esp", "ebp", "esi", "edi"];
const SEG_REGS: &[&str] = &["es", "cs", "ss", "ds", "fs", "gs"];
const SCALES: &[u8] = &[1, 2, 4, 8];

fn mem_base(base: &str, disp: Displacement, segment: Option<&str>) -> MemoryOperand {
    MemoryOperand {
        segment: segment.map(|s| s.to_string()),
        displacement: disp,
        base: Some(Register::new(base)),
        index: None,
        scale: None,
    }
}

fn mem_base_index(
    base: Option<&str>,
    index: &str,
    scale: u8,
    disp: Displacement,
    segment: Option<&str>,
) -> MemoryOperand {
    MemoryOperand {
        segment: segment.map(|s| s.to_string()),
        displacement: disp,
        base: base.map(Register::new),
        index: Some(Register::new(index)),
        scale: Some(scale),
    }
}

fn mem_abs(disp: i64, segment: Option<&str>) -> MemoryOperand {
    MemoryOperand {
        segment: segment.map(|s| s.to_string()),
        displacement: Displacement::Integer(disp),
        base: None,
        index: None,
        scale: None,
    }
}

fn att_disp(d: &Displacement) -> String {
    match d {
        Displacement::None => String::new(),
        Displacement::Integer(0) => String::new(),
        Displacement::Integer(v) => format!("{v}"),
        Displacement::Symbol(s) => s.clone(),
        Displacement::SymbolAddend(s, a) => {
            if *a >= 0 {
                format!("{s}+{a}")
            } else {
                format!("{s}{a}")
            }
        }
        Displacement::SymbolPlusOffset(s, a) => {
            if *a >= 0 {
                format!("{s}+{a}")
            } else {
                format!("{s}{a}")
            }
        }
        Displacement::SymbolMod(s, m) => format!("{s}@{m}"),
    }
}

fn att_mem(mem: &MemoryOperand) -> String {
    let mut s = String::new();
    if let Some(ref seg) = mem.segment {
        s.push('%');
        s.push_str(seg);
        s.push(':');
    }
    s.push_str(&att_disp(&mem.displacement));
    s.push('(');
    match (&mem.base, &mem.index, mem.scale) {
        (None, None, _) => {}
        (Some(b), None, _) => {
            s.push('%');
            s.push_str(&b.name);
        }
        (Some(b), Some(i), sc) => {
            s.push('%');
            s.push_str(&b.name);
            s.push(',');
            s.push('%');
            s.push_str(&i.name);
            s.push(',');
            s.push_str(&format!("{}", sc.unwrap_or(1)));
        }
        (None, Some(i), sc) => {
            s.push(',');
            s.push('%');
            s.push_str(&i.name);
            s.push(',');
            s.push_str(&format!("{}", sc.unwrap_or(1)));
        }
    }
    s.push(')');
    if mem.base.is_none() && mem.index.is_none() {
        let mut abs = String::new();
        if let Some(ref seg) = mem.segment {
            abs.push('%');
            abs.push_str(seg);
            abs.push(':');
        }
        match &mem.displacement {
            Displacement::None => abs.push_str("0"),
            Displacement::Integer(v) => abs.push_str(&format!("{v}")),
            other => abs.push_str(&att_disp(other)),
        }
        return abs;
    }
    s
}

fn sut_encode(ops: Vec<Operand>) -> Result<Vec<u8>, String> {
    let mut enc = InstructionEncoder::new();
    enc.encode(&Instruction {
        prefix: None,
        mnemonic: MNEMONIC.to_string(),
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

/// Known-answer gate: llvm-mc connection + SUT basic path.
#[test]
fn encode_prefetch_0f0d_kat_llvm_mc_eax() {
    let mc = llvm_mc_bytes("prefetchw (%eax)").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x0f, 0x0d, 0x08], "llvm-mc KAT mapping broken");
    let sut = sut_encode(vec![Operand::Memory(mem_base(
        "eax",
        Displacement::None,
        None,
    ))])
    .expect("SUT KAT");
    assert_eq!(sut, mc, "SUT KAT prefetchw (%eax)");
}

#[test]
fn encode_prefetch_0f0d_kat_llvm_mc_esp() {
    let mc = llvm_mc_bytes("prefetchw (%esp)").expect("llvm-mc KAT esp");
    assert_eq!(mc, vec![0x0f, 0x0d, 0x0c, 0x24]);
    let sut = sut_encode(vec![Operand::Memory(mem_base(
        "esp",
        Displacement::None,
        None,
    ))])
    .expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_prefetch_0f0d_kat_llvm_mc_ebp() {
    let mc = llvm_mc_bytes("prefetchw (%ebp)").expect("llvm-mc KAT ebp");
    assert_eq!(mc, vec![0x0f, 0x0d, 0x4d, 0x00]);
    let sut = sut_encode(vec![Operand::Memory(mem_base(
        "ebp",
        Displacement::None,
        None,
    ))])
    .expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_prefetch_0f0d_kat_llvm_mc_disp8() {
    let mc = llvm_mc_bytes("prefetchw 8(%eax)").expect("llvm-mc KAT disp8");
    assert_eq!(mc, vec![0x0f, 0x0d, 0x48, 0x08]);
    let sut = sut_encode(vec![Operand::Memory(mem_base(
        "eax",
        Displacement::Integer(8),
        None,
    ))])
    .expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_prefetch_0f0d_kat_llvm_mc_segment_fs() {
    let mc = llvm_mc_bytes("prefetchw %fs:(%eax)").expect("llvm-mc KAT fs");
    assert_eq!(mc, vec![0x64, 0x0f, 0x0d, 0x08]);
    let sut = sut_encode(vec![Operand::Memory(mem_base(
        "eax",
        Displacement::None,
        Some("fs"),
    ))]);
    // Expect agreement with llvm-mc; failure is a SUT bug (missing segment prefix).
    match sut {
        Ok(bytes) => assert_eq!(bytes, mc, "SUT must emit FS override 0x64 before 0F 0D"),
        Err(e) => panic!("SUT erred on valid FS mem: {e}"),
    }
}

proptest! {
    #![proptest_config(cfg())]

    // Oracle: differential — SUT bytes == llvm-mc for base+disp forms
    #[test]
    fn encode_prefetch_0f0d_diff_llvm_mc_base_disp(
        base in prop::sample::select(GP_REGS),
        disp in prop_oneof![
            Just(0i64),
            Just(-1i64),
            Just(1i64),
            Just(127i64),
            Just(-128i64),
            Just(128i64),
            Just(-129i64),
            Just(255i64),
            Just(0x7fff_ffffi64),
            Just(-0x8000_0000i64),
            (-0x8000_0000i64..=0x7fff_ffffi64),
        ],
    ) {
        let mem = mem_base(base, Displacement::Integer(disp), None);
        let asm = format!("{MNEMONIC} {}", att_mem(&mem));
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let sut = sut_encode(vec![Operand::Memory(mem)])
            .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(sut, mc, "diff vs llvm-mc for `{}`", asm);
    }

    // Oracle: differential — base+index*scale+disp (index ≠ esp)
    #[test]
    fn encode_prefetch_0f0d_diff_llvm_mc_sib(
        base in prop::option::of(prop::sample::select(GP_REGS)),
        index in prop::sample::select(
            GP_REGS.iter().copied().filter(|r| *r != "esp").collect::<Vec<_>>()
        ),
        scale in prop::sample::select(SCALES),
        disp in prop_oneof![
            Just(0i64),
            Just(-1i64),
            Just(127i64),
            Just(-128i64),
            Just(128i64),
            (-512i64..=512i64),
        ],
    ) {
        let mem = mem_base_index(
            base,
            index,
            scale,
            Displacement::Integer(disp),
            None,
        );
        let asm = format!("{MNEMONIC} {}", att_mem(&mem));
        let mc = match llvm_mc_bytes(&asm) {
            Ok(b) => b,
            Err(_) => return Ok(()), // skip forms llvm-mc rejects (rare)
        };
        let sut = sut_encode(vec![Operand::Memory(mem)])
            .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(sut, mc, "SIB diff vs llvm-mc for `{}`", asm);
    }

    // Oracle: algebraic.invariant — 0F 0D /1 in ModRM.reg
    #[test]
    fn encode_prefetch_0f0d_hint_modrm_reg(
        base in prop::sample::select(GP_REGS),
        disp in prop_oneof![Just(0i64), Just(7i64), Just(-3i64), (-200i64..=300i64)],
    ) {
        let mem = mem_base(base, Displacement::Integer(disp), None);
        let bytes = sut_encode(vec![Operand::Memory(mem)])
            .expect("valid mem must encode");
        prop_assert!(bytes.len() >= 3, "need opcode+modrm, got {bytes:?}");
        prop_assert_eq!(bytes[0], 0x0F);
        prop_assert_eq!(bytes[1], 0x0D);
        let reg = (bytes[2] >> 3) & 7;
        prop_assert_eq!(reg, HINT, "ModRM.reg must be hint=1 for prefetchw");
    }

    // Oracle: algebraic.metamorphic — opcode 0F 0D and reg=1 stable across mem forms
    #[test]
    fn encode_prefetch_0f0d_meta_opcode_stable(
        base1 in prop::sample::select(GP_REGS),
        base2 in prop::sample::select(GP_REGS),
        disp1 in prop_oneof![Just(0i64), Just(40i64), Just(-5i64), (-1000i64..=1000i64)],
        disp2 in prop_oneof![Just(0i64), Just(1i64), Just(127i64), (-500i64..=500i64)],
        index in prop::option::of(prop::sample::select(
            GP_REGS.iter().copied().filter(|r| *r != "esp").collect::<Vec<_>>()
        )),
        scale in prop::sample::select(SCALES),
    ) {
        let mem1 = mem_base(base1, Displacement::Integer(disp1), None);
        let mem2 = match index {
            Some(idx) => mem_base_index(Some(base2), idx, scale, Displacement::Integer(disp2), None),
            None => mem_base(base2, Displacement::Integer(disp2), None),
        };
        let a = sut_encode(vec![Operand::Memory(mem1)]).expect("m1");
        let b = sut_encode(vec![Operand::Memory(mem2)]).expect("m2");
        prop_assert!(a.len() >= 3 && b.len() >= 3);
        prop_assert_eq!(&a[0..2], &[0x0Fu8, 0x0D]);
        prop_assert_eq!(&b[0..2], &[0x0Fu8, 0x0D]);
        prop_assert_eq!((a[2] >> 3) & 7, HINT);
        prop_assert_eq!((b[2] >> 3) & 7, HINT);
    }

    // Oracle: negative_error — wrong arity
    #[test]
    fn encode_prefetch_0f0d_neg_arity(
        n_extra in 0usize..4,
        use_empty in any::<bool>(),
    ) {
        let ops: Vec<Operand> = if use_empty {
            vec![]
        } else {
            let mut v = vec![
                Operand::Memory(mem_base("eax", Displacement::None, None)),
                Operand::Memory(mem_base("ebx", Displacement::None, None)),
            ];
            for _ in 0..n_extra {
                v.push(Operand::Register(Register::new("ecx")));
            }
            v
        };
        prop_assume!(ops.len() != 1);
        let err = sut_encode(ops).expect_err("arity ≠ 1 must Err");
        prop_assert!(
            err.contains("prefetchw requires 1 operand")
                || err.contains("requires 1 operand"),
            "unexpected err: {err}"
        );
    }

    // Oracle: negative_error — non-memory operand
    #[test]
    fn encode_prefetch_0f0d_neg_non_memory(
        kind in 0u8..4,
        reg in prop::sample::select(GP_REGS),
        imm in any::<i32>(),
    ) {
        let op = match kind {
            0 => Operand::Register(Register::new(reg)),
            1 => Operand::Immediate(ImmediateValue::Integer(imm as i64)),
            2 => Operand::Label("target".into()),
            _ => Operand::Indirect(Box::new(Operand::Register(Register::new(reg)))),
        };
        let err = sut_encode(vec![op]).expect_err("non-mem must Err");
        prop_assert!(
            err.contains("memory operand") || err.contains("prefetchw requires"),
            "unexpected err: {err}"
        );
    }

    // Oracle: differential — ESP/EBP/scale/disp boundary edges
    #[test]
    fn encode_prefetch_0f0d_diff_edges_and_abs(
        edge in 0u8..12,
    ) {
        let mem = match edge {
            0 => mem_base("esp", Displacement::None, None),
            1 => mem_base("ebp", Displacement::None, None),
            2 => mem_base("esp", Displacement::Integer(0), None),
            3 => mem_base("ebp", Displacement::Integer(0), None),
            4 => mem_base("esp", Displacement::Integer(127), None),
            5 => mem_base("ebp", Displacement::Integer(-128), None),
            6 => mem_base("eax", Displacement::Integer(128), None),
            7 => mem_base("eax", Displacement::Integer(-129), None),
            8 => mem_base_index(Some("esp"), "eax", 1, Displacement::None, None),
            9 => mem_base_index(Some("ebp"), "ecx", 4, Displacement::Integer(8), None),
            10 => mem_base_index(Some("eax"), "ebx", 8, Displacement::Integer(-1), None),
            _ => mem_abs(0x1234_5678, None),
        };
        let asm = format!("{MNEMONIC} {}", att_mem(&mem));
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected edge `{asm}`: {e}"));
        let sut = sut_encode(vec![Operand::Memory(mem)])
            .unwrap_or_else(|e| panic!("SUT rejected edge `{asm}`: {e}"));
        prop_assert_eq!(sut, mc, "edge diff for `{}`", asm);
    }

    // Oracle: differential — segment override prefixes
    #[test]
    fn encode_prefetch_0f0d_diff_segment_prefix(
        seg in prop::sample::select(SEG_REGS),
        base in prop::sample::select(GP_REGS),
        disp in prop_oneof![Just(0i64), Just(8i64), Just(-4i64), Just(127i64)],
    ) {
        let mem = mem_base(base, Displacement::Integer(disp), Some(seg));
        let asm = format!("{MNEMONIC} {}", att_mem(&mem));
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let sut = sut_encode(vec![Operand::Memory(mem)])
            .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(
            sut.clone(), mc.clone(),
            "segment prefix diff for `{}`: SUT={:02x?} llvm-mc={:02x?}",
            asm, sut, mc
        );
    }

    // Absolute disp32 form
    #[test]
    fn encode_prefetch_0f0d_diff_abs_disp32(
        disp in prop_oneof![
            Just(0i64),
            Just(1i64),
            Just(0x1234_5678i64),
            Just(-1i64),
            (-0x8000_0000i64..=0x7fff_ffffi64),
        ],
    ) {
        let mem = mem_abs(disp, None);
        let asm = format!("{MNEMONIC} {}", att_mem(&mem));
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let sut = sut_encode(vec![Operand::Memory(mem)])
            .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(sut, mc, "abs diff for `{}`", asm);
    }
}

/// Deterministic regression: missing FS segment prefix on prefetchw.
#[test]
fn test_encode_prefetch_0f0d_regression_missing_fs_prefix() {
    let mem = mem_base("eax", Displacement::None, Some("fs"));
    let sut = sut_encode(vec![Operand::Memory(mem)]).expect("encode");
    let mc = vec![0x64u8, 0x0f, 0x0d, 0x08];
    assert_eq!(
        sut, mc,
        "prefetchw %fs:(%eax) must be [64, 0f, 0d, 08], got {sut:02x?}"
    );
}
