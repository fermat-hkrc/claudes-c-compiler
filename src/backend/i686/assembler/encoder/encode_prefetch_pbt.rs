// Oracle: differential — llvm-mc i686 assembler (PREFETCHh 0F 18 /hint)
// Evidence: system.rs:10 "Encode prefetch instructions (0F 18 /hint)";
//   encoder/mod.rs:739-742 prefetcht0/t1/t2/nta → encode_prefetch(ops, hint);
//   Intel SDM PREFETCHh: 0F 18 /0=nta /1=t0 /2=t1 /3=t2 + ModR/M memory;
//   sibling x86 encode_sse_mem_only emits segment prefix before 0F 18;
//   i686 core.rs:31-42 emit_segment_prefix for fs/gs/es/cs/ss/ds.
// Stronger considered:
//   - State machine: rejected — pure encoding, no lifecycle
//   - Algebraic round-trip: rejected — no in-tree i686 decoder for prefetch
// Differential: candidate=encode_prefetch (via InstructionEncoder::encode dispatch),
//   reference=llvm-mc -triple=i686 -show-encoding (LLVM 15),
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=AT&T memory operand bytes <-> llvm-mc encoding bytes.
// Also: algebraic.invariant (opcode/hint), algebraic.metamorphic (hint isolates
//   ModRM.reg), negative_error (arity + non-memory).

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

const MNEMONICS: &[&str] = &["prefetcht0", "prefetcht1", "prefetcht2", "prefetchnta"];

fn hint_of(mnemonic: &str) -> u8 {
    match mnemonic {
        "prefetchnta" => 0,
        "prefetcht0" => 1,
        "prefetcht1" => 2,
        "prefetcht2" => 3,
        _ => panic!("unknown prefetch mnemonic {mnemonic}"),
    }
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
        (None, None, _) => {
            // absolute: gas/llvm-mc want bare disp, not "()"
            // handled by caller for abs form
        }
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
    // Absolute no-base no-index: use bare displacement form
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

/// Known-answer gate: llvm-mc connection + SUT basic path.
#[test]
fn encode_prefetch_kat_llvm_mc_t0_eax() {
    let mc = llvm_mc_bytes("prefetcht0 (%eax)").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x0f, 0x18, 0x08], "llvm-mc KAT mapping broken");
    let sut = sut_encode(
        "prefetcht0",
        vec![Operand::Memory(mem_base("eax", Displacement::None, None))],
    )
    .expect("SUT KAT");
    assert_eq!(sut, mc, "SUT KAT prefetcht0 (%eax)");
}

#[test]
fn encode_prefetch_kat_llvm_mc_nta_eax() {
    let mc = llvm_mc_bytes("prefetchnta (%eax)").expect("llvm-mc KAT nta");
    assert_eq!(mc, vec![0x0f, 0x18, 0x00]);
    let sut = sut_encode(
        "prefetchnta",
        vec![Operand::Memory(mem_base("eax", Displacement::None, None))],
    )
    .expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_prefetch_kat_llvm_mc_t1_esp() {
    let mc = llvm_mc_bytes("prefetcht1 (%esp)").expect("llvm-mc KAT esp");
    assert_eq!(mc, vec![0x0f, 0x18, 0x14, 0x24]);
    let sut = sut_encode(
        "prefetcht1",
        vec![Operand::Memory(mem_base("esp", Displacement::None, None))],
    )
    .expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_prefetch_kat_llvm_mc_t2_ebp() {
    let mc = llvm_mc_bytes("prefetcht2 (%ebp)").expect("llvm-mc KAT ebp");
    assert_eq!(mc, vec![0x0f, 0x18, 0x5d, 0x00]);
    let sut = sut_encode(
        "prefetcht2",
        vec![Operand::Memory(mem_base("ebp", Displacement::None, None))],
    )
    .expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_prefetch_kat_llvm_mc_segment_fs() {
    let mc = llvm_mc_bytes("prefetcht0 %fs:(%eax)").expect("llvm-mc KAT fs");
    assert_eq!(mc, vec![0x64, 0x0f, 0x18, 0x08]);
    let sut = sut_encode(
        "prefetcht0",
        vec![Operand::Memory(mem_base(
            "eax",
            Displacement::None,
            Some("fs"),
        ))],
    );
    // Expect agreement with llvm-mc; failure is a SUT bug (missing segment prefix).
    match sut {
        Ok(bytes) => assert_eq!(bytes, mc, "SUT must emit FS override 0x64 before 0F 18"),
        Err(e) => panic!("SUT erred on valid FS mem: {e}"),
    }
}

proptest! {
    #![proptest_config(cfg())]

    // Oracle: differential — SUT bytes == llvm-mc for base+disp forms
    #[test]
    fn encode_prefetch_diff_llvm_mc_base_disp(
        mnemonic in prop::sample::select(MNEMONICS),
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
        let asm = format!("{mnemonic} {}", att_mem(&mem));
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let sut = sut_encode(mnemonic, vec![Operand::Memory(mem)])
            .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(sut, mc, "diff vs llvm-mc for `{}`", asm);
    }

    // Oracle: differential — base+index*scale+disp (index ≠ esp)
    #[test]
    fn encode_prefetch_diff_llvm_mc_sib(
        mnemonic in prop::sample::select(MNEMONICS),
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
        let asm = format!("{mnemonic} {}", att_mem(&mem));
        let mc = match llvm_mc_bytes(&asm) {
            Ok(b) => b,
            Err(_) => return Ok(()), // skip forms llvm-mc rejects (rare)
        };
        let sut = sut_encode(mnemonic, vec![Operand::Memory(mem)])
            .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(sut, mc, "SIB diff vs llvm-mc for `{}`", asm);
    }

    // Oracle: algebraic.invariant — 0F 18 /hint in ModRM.reg
    #[test]
    fn encode_prefetch_hint_modrm_reg(
        mnemonic in prop::sample::select(MNEMONICS),
        base in prop::sample::select(GP_REGS),
        disp in prop_oneof![Just(0i64), Just(7i64), Just(-3i64), (-200i64..=300i64)],
    ) {
        let mem = mem_base(base, Displacement::Integer(disp), None);
        let bytes = sut_encode(mnemonic, vec![Operand::Memory(mem)])
            .expect("valid mem must encode");
        prop_assert!(bytes.len() >= 3, "need opcode+modrm, got {bytes:?}");
        prop_assert_eq!(bytes[0], 0x0F);
        prop_assert_eq!(bytes[1], 0x18);
        let reg = (bytes[2] >> 3) & 7;
        prop_assert_eq!(reg, hint_of(mnemonic), "ModRM.reg must be hint");
    }

    // Oracle: algebraic.metamorphic — hint change only flips ModRM.reg
    #[test]
    fn encode_prefetch_meta_hint_isolates_reg(
        m1 in prop::sample::select(MNEMONICS),
        m2 in prop::sample::select(MNEMONICS),
        base in prop::sample::select(GP_REGS),
        disp in prop_oneof![Just(0i64), Just(40i64), Just(-5i64), (-1000i64..=1000i64)],
        index in prop::option::of(prop::sample::select(
            GP_REGS.iter().copied().filter(|r| *r != "esp").collect::<Vec<_>>()
        )),
        scale in prop::sample::select(SCALES),
    ) {
        let mem = match index {
            Some(idx) => mem_base_index(Some(base), idx, scale, Displacement::Integer(disp), None),
            None => mem_base(base, Displacement::Integer(disp), None),
        };
        let a = sut_encode(m1, vec![Operand::Memory(mem.clone())]).expect("m1");
        let b = sut_encode(m2, vec![Operand::Memory(mem)]).expect("m2");
        prop_assert_eq!(&a[0..2], &[0x0Fu8, 0x18]);
        prop_assert_eq!(&b[0..2], &[0x0Fu8, 0x18]);
        prop_assert_eq!(a.len(), b.len(), "length must match across hints");
        // Only reg field of ModRM may differ
        prop_assert_eq!(a[2] & 0xC7, b[2] & 0xC7, "mod+rm must be identical");
        prop_assert_eq!((a[2] >> 3) & 7, hint_of(m1));
        prop_assert_eq!((b[2] >> 3) & 7, hint_of(m2));
        if a.len() > 3 {
            prop_assert_eq!(&a[3..], &b[3..], "SIB/disp must be identical");
        }
    }

    // Oracle: negative_error — wrong arity
    #[test]
    fn encode_prefetch_neg_arity(
        mnemonic in prop::sample::select(MNEMONICS),
        n_extra in 0usize..4,
        use_empty in any::<bool>(),
    ) {
        let ops: Vec<Operand> = if use_empty {
            vec![]
        } else {
            // 2+ memory ops
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
        let err = sut_encode(mnemonic, ops).expect_err("arity ≠ 1 must Err");
        prop_assert!(
            err.contains("prefetch requires 1 operand")
                || err.contains("requires 1 operand"),
            "unexpected err: {err}"
        );
    }

    // Oracle: negative_error — non-memory operand
    #[test]
    fn encode_prefetch_neg_non_memory(
        mnemonic in prop::sample::select(MNEMONICS),
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
        let err = sut_encode(mnemonic, vec![op]).expect_err("non-mem must Err");
        prop_assert!(
            err.contains("memory operand") || err.contains("prefetch requires"),
            "unexpected err: {err}"
        );
    }

    // Oracle: differential — ESP/EBP/scale/disp boundary edges
    #[test]
    fn encode_prefetch_diff_sib_esp_ebp_edges(
        mnemonic in prop::sample::select(MNEMONICS),
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
        let asm = format!("{mnemonic} {}", att_mem(&mem));
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected edge `{asm}`: {e}"));
        let sut = sut_encode(mnemonic, vec![Operand::Memory(mem)])
            .unwrap_or_else(|e| panic!("SUT rejected edge `{asm}`: {e}"));
        prop_assert_eq!(sut, mc, "edge diff for `{}`", asm);
    }

    // Oracle: differential — segment override prefixes
    #[test]
    fn encode_prefetch_diff_segment_prefix(
        mnemonic in prop::sample::select(MNEMONICS),
        seg in prop::sample::select(SEG_REGS),
        base in prop::sample::select(GP_REGS),
        disp in prop_oneof![Just(0i64), Just(8i64), Just(-4i64), Just(127i64)],
    ) {
        let mem = mem_base(base, Displacement::Integer(disp), Some(seg));
        let asm = format!("{mnemonic} {}", att_mem(&mem));
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let sut = sut_encode(mnemonic, vec![Operand::Memory(mem)])
            .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(
            sut.clone(), mc.clone(),
            "segment prefix diff for `{}`: SUT={:02x?} llvm-mc={:02x?}",
            asm, sut, mc
        );
    }

    // Oracle: algebraic.invariant — successful encode length / opcode
    #[test]
    fn encode_prefetch_opcode_len_ge3(
        mnemonic in prop::sample::select(MNEMONICS),
        base in prop::sample::select(GP_REGS),
        disp in (-0x10000i64..=0x10000i64),
        use_index in any::<bool>(),
        index in prop::sample::select(
            GP_REGS.iter().copied().filter(|r| *r != "esp").collect::<Vec<_>>()
        ),
        scale in prop::sample::select(SCALES),
    ) {
        let mem = if use_index {
            mem_base_index(Some(base), index, scale, Displacement::Integer(disp), None)
        } else {
            mem_base(base, Displacement::Integer(disp), None)
        };
        let bytes = sut_encode(mnemonic, vec![Operand::Memory(mem)]).expect("encode");
        prop_assert!(bytes.len() >= 3);
        prop_assert_eq!(bytes[0], 0x0F);
        prop_assert_eq!(bytes[1], 0x18);
    }

    // Absolute disp32 form
    #[test]
    fn encode_prefetch_diff_abs_disp32(
        mnemonic in prop::sample::select(MNEMONICS),
        disp in prop_oneof![
            Just(0i64),
            Just(1i64),
            Just(0x1234_5678i64),
            Just(-1i64),
            (-0x8000_0000i64..=0x7fff_ffffi64),
        ],
    ) {
        let mem = mem_abs(disp, None);
        let asm = format!("{mnemonic} {}", att_mem(&mem));
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let sut = sut_encode(mnemonic, vec![Operand::Memory(mem)])
            .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(sut, mc, "abs diff for `{}`", asm);
    }
}

/// Deterministic regression: missing FS segment prefix on prefetcht0.
#[test]
fn test_encode_prefetch_regression_missing_fs_prefix() {
    let mem = mem_base("eax", Displacement::None, Some("fs"));
    let sut = sut_encode("prefetcht0", vec![Operand::Memory(mem)]).expect("encode");
    let mc = vec![0x64u8, 0x0f, 0x18, 0x08];
    assert_eq!(
        sut, mc,
        "prefetcht0 %fs:(%eax) must be [64, 0f, 18, 08], got {sut:02x?}"
    );
}
