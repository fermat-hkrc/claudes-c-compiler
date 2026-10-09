// Oracle: differential — llvm-mc i686 assembler (MOV imm→mem: C6/C7 /0 + optional 0x66 + seg)
// Evidence: gp_integer.rs:238-270 encode_mov_imm_mem;
//   encoder/mod.rs:167-169 movl/movw/movb → encode_mov(size) → encode_mov_imm_mem for Imm,Memory;
//   core.rs:31-42 emit_segment_prefix (es/cs/ss/ds/fs/gs);
//   x86-64 sibling gp_integer.rs:179-180 calls emit_segment_prefix before C6/C7;
//   Intel SDM Vol.2 MOV — r/m8,imm8 / r/m16,imm16 / r/m32,imm32 (opcodes C6/C7 /0);
//   AT&T order: movb/movw/movl $imm, mem with ModRM.reg=/0.
// Stronger considered:
//   - State machine: rejected — pure encoding, no lifecycle
//   - Algebraic round-trip: rejected — no in-tree i686 MOV decoder
// Differential: candidate=encode_mov_imm_mem (via InstructionEncoder::encode movb/movw/movl),
//   reference=llvm-mc -triple=i686 -show-encoding (LLVM 15),
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=AT&T operand bytes <-> llvm-mc encoding bytes.
// Also: algebraic.invariant (opcode/modrm/imm trail), algebraic.metamorphic (same-mem imm),
//   negative_error (narrow symbol + SymbolMod/Diff).

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
const SEG_REGS: &[&str] = &["es", "cs", "ss", "ds", "fs", "gs"];
const SCALES: &[u8] = &[1, 2, 4, 8];

fn suffix_for(w: u8) -> &'static str {
    match w {
        1 => "movb",
        2 => "movw",
        _ => "movl",
    }
}

fn imm_in_range(width: u8, imm: i64) -> i64 {
    match width {
        1 => imm as i8 as i64,
        2 => imm as i16 as i64,
        _ => imm as i32 as i64,
    }
}

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
        Displacement::SymbolAddend(s, a) | Displacement::SymbolPlusOffset(s, a) => {
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
    let mut s = String::new();
    if let Some(ref seg) = mem.segment {
        s.push('%');
        s.push_str(seg);
        s.push(':');
    }
    s.push_str(&att_disp(&mem.displacement));
    s.push('(');
    match (&mem.base, &mem.index, mem.scale) {
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
        (None, None, _) => {}
    }
    s.push(')');
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

fn sut_encode_full(
    mnemonic: &str,
    ops: Vec<Operand>,
) -> Result<(Vec<u8>, Vec<super::Relocation>), String> {
    let mut enc = InstructionEncoder::new();
    enc.encode(&Instruction {
        prefix: None,
        mnemonic: mnemonic.to_string(),
        operands: ops,
    })?;
    Ok((enc.bytes, enc.relocations))
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
        // llvm-mc uses 'A' for relocatable fixup bytes — treat as 0
        if t == "A" {
            bytes.push(0);
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

fn imm_le_bytes(imm: i64, width: u8) -> Vec<u8> {
    match width {
        1 => vec![imm as u8],
        2 => (imm as i16).to_le_bytes().to_vec(),
        _ => (imm as i32).to_le_bytes().to_vec(),
    }
}

fn split_imm_trail(bytes: &[u8], width: u8) -> Option<(&[u8], &[u8])> {
    let n = width as usize;
    if bytes.len() < n {
        return None;
    }
    let split = bytes.len() - n;
    Some((&bytes[..split], &bytes[split..]))
}

// --- KAT gate (reference oracle prerequisite) ---

#[test]
fn encode_mov_imm_mem_kat_llvm_mc_base32() {
    let mc = llvm_mc_bytes("movl $0x12345678, (%eax)").expect("llvm-mc KAT");
    assert_eq!(
        mc,
        vec![0xc7, 0x00, 0x78, 0x56, 0x34, 0x12],
        "llvm-mc KAT mapping broken"
    );
    let sut = sut_encode(
        "movl",
        vec![
            Operand::Immediate(ImmediateValue::Integer(0x1234_5678)),
            Operand::Memory(mem_base("eax", Displacement::None, None)),
        ],
    )
    .expect("SUT KAT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_mov_imm_mem_kat_llvm_mc_16() {
    let mc = llvm_mc_bytes("movw $0x1234, (%eax)").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x66, 0xc7, 0x00, 0x34, 0x12]);
    let sut = sut_encode(
        "movw",
        vec![
            Operand::Immediate(ImmediateValue::Integer(0x1234)),
            Operand::Memory(mem_base("eax", Displacement::None, None)),
        ],
    )
    .expect("SUT KAT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_mov_imm_mem_kat_llvm_mc_8() {
    let mc = llvm_mc_bytes("movb $0x12, (%eax)").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0xc6, 0x00, 0x12]);
    let sut = sut_encode(
        "movb",
        vec![
            Operand::Immediate(ImmediateValue::Integer(0x12)),
            Operand::Memory(mem_base("eax", Displacement::None, None)),
        ],
    )
    .expect("SUT KAT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_mov_imm_mem_kat_llvm_mc_esp_ebp() {
    let mc = llvm_mc_bytes("movl $1, (%esp)").expect("llvm-mc KAT esp");
    assert_eq!(mc, vec![0xc7, 0x04, 0x24, 0x01, 0x00, 0x00, 0x00]);
    let sut = sut_encode(
        "movl",
        vec![
            Operand::Immediate(ImmediateValue::Integer(1)),
            Operand::Memory(mem_base("esp", Displacement::None, None)),
        ],
    )
    .expect("SUT");
    assert_eq!(sut, mc);

    let mc = llvm_mc_bytes("movl $1, (%ebp)").expect("llvm-mc KAT ebp");
    assert_eq!(mc, vec![0xc7, 0x45, 0x00, 0x01, 0x00, 0x00, 0x00]);
    let sut = sut_encode(
        "movl",
        vec![
            Operand::Immediate(ImmediateValue::Integer(1)),
            Operand::Memory(mem_base("ebp", Displacement::None, None)),
        ],
    )
    .expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_mov_imm_mem_kat_llvm_mc_fs() {
    let mc = llvm_mc_bytes("movl $1, %fs:(%eax)").expect("llvm-mc KAT fs");
    assert_eq!(mc, vec![0x64, 0xc7, 0x00, 0x01, 0x00, 0x00, 0x00]);
    let sut = sut_encode(
        "movl",
        vec![
            Operand::Immediate(ImmediateValue::Integer(1)),
            Operand::Memory(mem_base("eax", Displacement::None, Some("fs"))),
        ],
    );
    match sut {
        Ok(bytes) => assert_eq!(bytes, mc, "SUT must emit FS override 0x64 before C7"),
        Err(e) => panic!("SUT erred on valid FS imm→mem: {e}"),
    }
}

#[test]
fn encode_mov_imm_mem_kat_llvm_mc_es_segment() {
    // Contract: all six segment overrides are valid on i686 (core.rs emit_segment_prefix).
    // x86-64 sibling calls emit_segment_prefix; i686 encode_mov_imm_mem emits none.
    let mc = llvm_mc_bytes("movl $1, %es:(%eax)").expect("llvm-mc KAT es");
    assert_eq!(mc, vec![0x26, 0xc7, 0x00, 0x01, 0x00, 0x00, 0x00]);
    let sut = sut_encode(
        "movl",
        vec![
            Operand::Immediate(ImmediateValue::Integer(1)),
            Operand::Memory(mem_base("eax", Displacement::None, Some("es"))),
        ],
    );
    match sut {
        Ok(bytes) => assert_eq!(bytes, mc, "SUT must emit ES override 0x26 before C7"),
        Err(e) => panic!("SUT erred on valid ES imm→mem: {e}"),
    }
}

#[test]
fn encode_mov_imm_mem_kat_symbol_imm32() {
    let mc = llvm_mc_bytes("movl $sym, (%eax)").expect("llvm-mc KAT sym");
    assert_eq!(mc, vec![0xc7, 0x00, 0, 0, 0, 0]);
    let (sut, relocs) = sut_encode_full(
        "movl",
        vec![
            Operand::Immediate(ImmediateValue::Symbol("sym".into())),
            Operand::Memory(mem_base("eax", Displacement::None, None)),
        ],
    )
    .expect("SUT symbol");
    assert_eq!(sut, mc);
    assert_eq!(relocs.len(), 1);
    assert_eq!(relocs[0].symbol, "sym");
    assert_eq!(relocs[0].reloc_type, super::R_386_32);
    assert_eq!(relocs[0].addend, 0);
}

// --- Deterministic regressions (filled when bugs confirmed) ---

#[test]
fn test_encode_mov_imm_mem_regression_missing_es_prefix() {
    // Witness: movl $1, %es:(%eax) — SUT omits 0x26
    let mc = vec![0x26, 0xc7, 0x00, 0x01, 0x00, 0x00, 0x00];
    let sut = sut_encode(
        "movl",
        vec![
            Operand::Immediate(ImmediateValue::Integer(1)),
            Operand::Memory(mem_base("eax", Displacement::None, Some("es"))),
        ],
    )
    .expect("SUT should accept ES form");
    assert_eq!(
        sut, mc,
        "regression: missing ES segment override on imm→mem"
    );
}

#[test]
fn test_encode_mov_imm_mem_regression_missing_fs_prefix() {
    // Witness: movl $1, %fs:(%eax) — SUT omits 0x64 (unlike encode_mov_reg_mem which inlines fs/gs)
    let mc = vec![0x64, 0xc7, 0x00, 0x01, 0x00, 0x00, 0x00];
    let sut = sut_encode(
        "movl",
        vec![
            Operand::Immediate(ImmediateValue::Integer(1)),
            Operand::Memory(mem_base("eax", Displacement::None, Some("fs"))),
        ],
    )
    .expect("SUT should accept FS form");
    assert_eq!(
        sut, mc,
        "regression: missing FS segment override on imm→mem"
    );
}

proptest! {
    #![proptest_config(cfg())]

    // P1: differential — base+disp, no segment, in-range imm
    #[test]
    fn encode_mov_imm_mem_diff_llvm_mc_base_disp(
        base in prop::sample::select(GP32),
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
        width in prop::sample::select(vec![1u8, 2, 4]),
        imm in -0x8000_0000i64..=0x7fff_ffffi64,
    ) {
        let imm = imm_in_range(width, imm);
        let mem = mem_base(base, Displacement::Integer(disp), None);
        let mnemonic = suffix_for(width);
        let asm = format!("{mnemonic} ${imm}, {}", att_mem(&mem));
        let mc = llvm_mc_bytes(&asm)
            .map_err(|e| TestCaseError::fail(format!("llvm-mc rejected `{asm}`: {e}")))?;
        let sut = sut_encode(
            mnemonic,
            vec![
                Operand::Immediate(ImmediateValue::Integer(imm)),
                Operand::Memory(mem),
            ],
        )
        .map_err(|e| TestCaseError::fail(format!("SUT rejected `{asm}`: {e}")))?;
        prop_assert_eq!(&sut, &mc, "diff `{}`: sut={:02x?} mc={:02x?}", asm, &sut, &mc);
    }

    // P2: differential — SIB forms
    #[test]
    fn encode_mov_imm_mem_diff_llvm_mc_sib(
        base in prop::option::of(prop::sample::select(GP32)),
        index in prop::sample::select(
            GP32.iter().copied().filter(|r| *r != "esp").collect::<Vec<_>>()
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
        width in prop::sample::select(vec![1u8, 2, 4]),
        imm in -0x8000_0000i64..=0x7fff_ffffi64,
    ) {
        let imm = imm_in_range(width, imm);
        let mem = mem_base_index(base, index, scale, Displacement::Integer(disp), None);
        let mnemonic = suffix_for(width);
        let asm = format!("{mnemonic} ${imm}, {}", att_mem(&mem));
        let mc = match llvm_mc_bytes(&asm) {
            Ok(b) => b,
            Err(_) => return Ok(()),
        };
        let sut = sut_encode(
            mnemonic,
            vec![
                Operand::Immediate(ImmediateValue::Integer(imm)),
                Operand::Memory(mem),
            ],
        )
        .map_err(|e| TestCaseError::fail(format!("SUT rejected `{asm}`: {e}")))?;
        prop_assert_eq!(&sut, &mc, "SIB diff `{}`: sut={:02x?} mc={:02x?}", asm, &sut, &mc);
    }

    // P3: differential — all six segment overrides
    #[test]
    fn encode_mov_imm_mem_diff_llvm_mc_segment(
        seg in prop::sample::select(SEG_REGS),
        base in prop::sample::select(GP32),
        disp in prop_oneof![Just(0i64), Just(8i64), Just(-4i64), Just(127i64), Just(-128i64)],
        width in prop::sample::select(vec![1u8, 2, 4]),
        imm in prop_oneof![Just(0i64), Just(1i64), Just(-1i64), Just(0x7fi64), Just(0x12i64)],
    ) {
        let imm = imm_in_range(width, imm);
        let mem = mem_base(base, Displacement::Integer(disp), Some(seg));
        let mnemonic = suffix_for(width);
        let asm = format!("{mnemonic} ${imm}, {}", att_mem(&mem));
        let mc = llvm_mc_bytes(&asm)
            .map_err(|e| TestCaseError::fail(format!("llvm-mc rejected `{asm}`: {e}")))?;
        let sut = sut_encode(
            mnemonic,
            vec![
                Operand::Immediate(ImmediateValue::Integer(imm)),
                Operand::Memory(mem),
            ],
        )
        .map_err(|e| TestCaseError::fail(format!(
            "SUT rejected valid segment form `{asm}`: {e}; \
             i686 MOV imm→mem must emit segment override (core.rs emit_segment_prefix; \
             Intel SDM 2.1.1; x86-64 sibling emits prefix). Body emits none."
        )))?;
        prop_assert_eq!(
            &sut, &mc,
            "segment diff `{}`: sut={:02x?} mc={:02x?}",
            asm, &sut, &mc
        );
    }

    // P4: differential — ESP/EBP/SIB/abs edges
    #[test]
    fn encode_mov_imm_mem_diff_edges_esp_ebp_abs(
        edge in 0u8..14,
        width in prop::sample::select(vec![1u8, 2, 4]),
        imm in prop_oneof![Just(0i64), Just(1i64), Just(-1i64), Just(0x55i64), Just(0x7fffi64)],
    ) {
        let imm = imm_in_range(width, imm);
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
            11 => mem_abs(0x1234_5678, None),
            12 => mem_abs(0, None),
            _ => mem_abs(-1, None),
        };
        let mnemonic = suffix_for(width);
        let asm = format!("{mnemonic} ${imm}, {}", att_mem(&mem));
        let mc = match llvm_mc_bytes(&asm) {
            Ok(b) => b,
            Err(e) => return Err(TestCaseError::fail(format!("llvm-mc edge `{asm}`: {e}"))),
        };
        let sut = sut_encode(
            mnemonic,
            vec![
                Operand::Immediate(ImmediateValue::Integer(imm)),
                Operand::Memory(mem),
            ],
        )
        .map_err(|e| TestCaseError::fail(format!("SUT edge `{asm}`: {e}")))?;
        prop_assert_eq!(&sut, &mc, "edge diff `{}`", asm);
    }

    // P5: algebraic.invariant — opcode C6/C7, ModRM.reg = 0, imm trail LE
    #[test]
    fn encode_mov_imm_mem_invariant_opcode_modrm_imm(
        base in prop::sample::select(GP32),
        disp in prop_oneof![Just(0i64), Just(7i64), Just(-3i64), (-200i64..=300i64)],
        use_index in any::<bool>(),
        index in prop::sample::select(
            GP32.iter().copied().filter(|r| *r != "esp").collect::<Vec<_>>()
        ),
        scale in prop::sample::select(SCALES),
        width in prop::sample::select(vec![1u8, 2, 4]),
        imm in -0x8000_0000i64..=0x7fff_ffffi64,
    ) {
        let imm = imm_in_range(width, imm);
        let mem = if use_index {
            mem_base_index(Some(base), index, scale, Displacement::Integer(disp), None)
        } else {
            mem_base(base, Displacement::Integer(disp), None)
        };
        let mnemonic = suffix_for(width);
        let bytes = sut_encode(
            mnemonic,
            vec![
                Operand::Immediate(ImmediateValue::Integer(imm)),
                Operand::Memory(mem),
            ],
        )
        .map_err(|e| TestCaseError::fail(e))?;
        let (body, trail) = split_imm_trail(&bytes, width)
            .ok_or_else(|| TestCaseError::fail("too short for imm trail"))?;
        let want_trail = imm_le_bytes(imm, width);
        prop_assert_eq!(trail, want_trail.as_slice(), "imm LE trail");
        let mut i = 0usize;
        if width == 2 {
            prop_assert_eq!(body[i], 0x66, "missing 0x66 for movw");
            i += 1;
        }
        let op = body[i];
        if width == 1 {
            prop_assert_eq!(op, 0xC6, "movb imm→mem opcode");
        } else {
            prop_assert_eq!(op, 0xC7, "movw/movl imm→mem opcode");
        }
        i += 1;
        prop_assert!(body.len() > i, "need ModRM");
        let modrm = body[i];
        prop_assert_eq!((modrm >> 3) & 7, 0, "ModRM.reg = /0 for imm→mem");
    }

    // P6: algebraic.metamorphic — same mem, two imms share prefix+opcode+modrm; trails differ
    #[test]
    fn encode_mov_imm_mem_meta_same_mem_imm_trail(
        base in prop::sample::select(GP32),
        disp in prop_oneof![Just(0i64), Just(40i64), Just(-5i64), (-1000i64..=1000i64)],
        index in prop::option::of(prop::sample::select(
            GP32.iter().copied().filter(|r| *r != "esp").collect::<Vec<_>>()
        )),
        scale in prop::sample::select(SCALES),
        width in prop::sample::select(vec![1u8, 2, 4]),
        imm1 in -0x8000_0000i64..=0x7fff_ffffi64,
        imm2 in -0x8000_0000i64..=0x7fff_ffffi64,
    ) {
        let imm1 = imm_in_range(width, imm1);
        let imm2 = imm_in_range(width, imm2);
        prop_assume!(imm1 != imm2);
        let mem = match index {
            Some(idx) => mem_base_index(Some(base), idx, scale, Displacement::Integer(disp), None),
            None => mem_base(base, Displacement::Integer(disp), None),
        };
        let mnemonic = suffix_for(width);
        let a = sut_encode(
            mnemonic,
            vec![
                Operand::Immediate(ImmediateValue::Integer(imm1)),
                Operand::Memory(mem.clone()),
            ],
        )
        .map_err(|e| TestCaseError::fail(format!("imm1: {e}")))?;
        let b = sut_encode(
            mnemonic,
            vec![
                Operand::Immediate(ImmediateValue::Integer(imm2)),
                Operand::Memory(mem),
            ],
        )
        .map_err(|e| TestCaseError::fail(format!("imm2: {e}")))?;
        let (ba, ta) = split_imm_trail(&a, width)
            .ok_or_else(|| TestCaseError::fail("a short"))?;
        let (bb, tb) = split_imm_trail(&b, width)
            .ok_or_else(|| TestCaseError::fail("b short"))?;
        let want1 = imm_le_bytes(imm1, width);
        let want2 = imm_le_bytes(imm2, width);
        prop_assert_eq!(ba, bb, "prefix+opcode+ModRM+SIB+disp must match across imms");
        prop_assert_eq!(ta, want1.as_slice());
        prop_assert_eq!(tb, want2.as_slice());
        prop_assert_ne!(ta, tb, "distinct imms must yield distinct trails");
    }

    // P7: differential — symbol imm32 (movl $sym) + reloc
    #[test]
    fn encode_mov_imm_mem_diff_symbol_imm32(
        base in prop::sample::select(GP32),
        disp in prop_oneof![Just(0i64), Just(8i64), Just(-4i64), Just(127i64)],
        use_plus in any::<bool>(),
        addend in prop_oneof![Just(0i64), Just(4i64), Just(-8i64), Just(0x10i64)],
    ) {
        let sym = "pbt_sym";
        let mem = mem_base(base, Displacement::Integer(disp), None);
        let (imm_op, asm_imm, expect_addend) = if use_plus && addend != 0 {
            (
                ImmediateValue::SymbolPlusOffset(sym.into(), addend),
                if addend >= 0 {
                    format!("{sym}+{addend}")
                } else {
                    format!("{sym}{addend}")
                },
                addend,
            )
        } else {
            (ImmediateValue::Symbol(sym.into()), sym.to_string(), 0i64)
        };
        let asm = format!("movl ${asm_imm}, {}", att_mem(&mem));
        let mc = llvm_mc_bytes(&asm)
            .map_err(|e| TestCaseError::fail(format!("llvm-mc `{asm}`: {e}")))?;
        let (sut, relocs) = sut_encode_full(
            "movl",
            vec![Operand::Immediate(imm_op), Operand::Memory(mem)],
        )
        .map_err(|e| TestCaseError::fail(format!("SUT `{asm}`: {e}")))?;
        prop_assert_eq!(&sut, &mc, "symbol diff `{}`", asm);
        prop_assert_eq!(relocs.len(), 1, "one R_386_32 reloc");
        prop_assert_eq!(&relocs[0].symbol, sym);
        prop_assert_eq!(relocs[0].reloc_type, super::R_386_32);
        prop_assert_eq!(relocs[0].addend, expect_addend);
        // trailing 4 imm bytes are zeros (placeholder)
        prop_assert_eq!(&sut[sut.len() - 4..], &[0, 0, 0, 0]);
    }

    // P8: differential — llvm-mc accepts movb/movw $sym (FK_Data_1/2); SUT must not reject
    // Body returns Err("symbol immediate only supported for 32-bit...") — documented limitation
    // on an input the AT&T/public path accepts via movb/movw dispatch (mod.rs:167-169).
    #[test]
    fn encode_mov_imm_mem_diff_symbol_narrow(
        width in prop::sample::select(vec![1u8, 2]),
        base in prop::sample::select(GP32),
        disp in prop_oneof![Just(0i64), Just(4i64), Just(-8i64)],
    ) {
        let mem = mem_base(base, Displacement::Integer(disp), None);
        let mnemonic = suffix_for(width);
        let asm = format!("{mnemonic} $sym, {}", att_mem(&mem));
        let mc = llvm_mc_bytes(&asm)
            .map_err(|e| TestCaseError::fail(format!("llvm-mc accepted path broken `{asm}`: {e}")))?;
        let sut = sut_encode(
            mnemonic,
            vec![
                Operand::Immediate(ImmediateValue::Symbol("sym".into())),
                Operand::Memory(mem),
            ],
        )
        .map_err(|e| TestCaseError::fail(format!(
            "SUT rejected valid narrow symbol imm `{asm}`: {e}; llvm-mc encodes {:02x?}. \
             AT&T movb/movw $sym,mem is valid (FK_Data_1/2); body only allows size==4.",
            mc
        )))?;
        prop_assert_eq!(&sut, &mc, "narrow symbol diff `{}`", asm);
    }

    // P9: negative — SymbolMod / SymbolDiff are unsupported immediate kinds (always Err)
    #[test]
    fn encode_mov_imm_mem_neg_symbol_mod_diff(
        mode in 0u8..2,
        base in prop::sample::select(GP32),
        width in prop::sample::select(vec![1u8, 2, 4]),
    ) {
        let mem = mem_base(base, Displacement::None, None);
        let imm = if mode == 0 {
            ImmediateValue::SymbolMod("sym".into(), "GOT".into())
        } else {
            ImmediateValue::SymbolDiff("a".into(), "b".into())
        };
        let r = sut_encode(
            suffix_for(width),
            vec![Operand::Immediate(imm), Operand::Memory(mem)],
        );
        prop_assert!(
            r.is_err(),
            "SymbolMod/SymbolDiff must Err, got Ok({:02x?})",
            r.as_ref().ok()
        );
    }
}
