// Oracle: differential — llvm-mc i686 assembler (MOV mem→reg: 8A/8B /r + optional 0x66 + seg)
// Evidence: gp_integer.rs:194-214 encode_mov_mem_reg;
//   encoder/mod.rs:161-165 movl/movw/movb → encode_mov(size) → encode_mov_mem_reg for Memory,Register;
//   core.rs:31-42 emit_segment_prefix (es/cs/ss/ds/fs/gs);
//   Intel SDM Vol.2 MOV — r8,m8 / r16,m16 / r32,m32;
//   AT&T order: movb/movw/movl mem, %dst with ModRM.reg=dst under 8A/8B.
// Stronger considered:
//   - State machine: rejected — pure encoding, no lifecycle
//   - Algebraic round-trip: rejected — no in-tree i686 MOV decoder
// Differential: candidate=encode_mov_mem_reg (via InstructionEncoder::encode movb/movw/movl),
//   reference=llvm-mc -triple=i686 -show-encoding (LLVM 15),
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=AT&T operand bytes <-> llvm-mc encoding bytes.
// Also: algebraic.invariant (opcode/modrm), algebraic.metamorphic (load vs store),
//   negative_error (mismatched width + non-GP dest).

use super::InstructionEncoder;
use crate::backend::x86::assembler::parser::{
    Displacement, Instruction, MemoryOperand, Operand, Register,
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
const R16: &[&str] = &["ax", "cx", "dx", "bx", "sp", "bp", "si", "di"];
const R8: &[&str] = &["al", "cl", "dl", "bl", "ah", "ch", "dh", "bh"];
const SEG_REGS: &[&str] = &["es", "cs", "ss", "ds", "fs", "gs"];
const SCALES: &[u8] = &[1, 2, 4, 8];
const NON_GP: &[&str] = &[
    "xmm0", "xmm1", "xmm7", "mm0", "mm3", "mm7", "st", "st(0)", "st(1)", "ymm0",
];

fn gp_for_width(w: u8) -> &'static [&'static str] {
    match w {
        1 => R8,
        2 => R16,
        _ => GP32,
    }
}

fn suffix_for(w: u8) -> &'static str {
    match w {
        1 => "movb",
        2 => "movw",
        _ => "movl",
    }
}

fn gp_num(name: &str) -> u8 {
    match name {
        "eax" | "ax" | "al" => 0,
        "ecx" | "cx" | "cl" => 1,
        "edx" | "dx" | "dl" => 2,
        "ebx" | "bx" | "bl" => 3,
        "esp" | "sp" | "ah" => 4,
        "ebp" | "bp" | "ch" => 5,
        "esi" | "si" | "dh" => 6,
        "edi" | "di" | "bh" => 7,
        "xmm0" | "mm0" | "st" | "st(0)" | "ymm0" => 0,
        "xmm1" | "mm1" | "st(1)" | "ymm1" => 1,
        "xmm2" | "mm2" | "st(2)" | "ymm2" => 2,
        "xmm3" | "mm3" | "st(3)" | "ymm3" => 3,
        "xmm4" | "mm4" | "st(4)" | "ymm4" => 4,
        "xmm5" | "mm5" | "st(5)" | "ymm5" => 5,
        "xmm6" | "mm6" | "st(6)" | "ymm6" => 6,
        "xmm7" | "mm7" | "st(7)" | "ymm7" => 7,
        _ => panic!("bad gp {name}"),
    }
}

fn reg_size_local(name: &str) -> u8 {
    match name {
        "al" | "ah" | "bl" | "bh" | "cl" | "ch" | "dl" | "dh" => 1,
        "ax" | "bx" | "cx" | "dx" | "sp" | "bp" | "si" | "di" => 2,
        "es" | "cs" | "ss" | "ds" | "fs" | "gs" => 2,
        _ => 4,
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

/// Strip leading segment-override and optional 0x66 for opcode checks.
fn strip_seg_os_prefixes(bytes: &[u8]) -> &[u8] {
    let mut i = 0;
    while i < bytes.len()
        && matches!(bytes[i], 0x26 | 0x2E | 0x36 | 0x3E | 0x64 | 0x65 | 0x66)
    {
        i += 1;
    }
    &bytes[i..]
}

/// True when llvm-mc chose the short moffs form (A0/A1) for abs→eAX.
fn is_moffs_encoding(bytes: &[u8]) -> bool {
    let b = strip_seg_os_prefixes(bytes);
    matches!(b.first().copied(), Some(0xA0) | Some(0xA1))
}

// --- KAT gate (reference oracle prerequisite) ---

#[test]
fn encode_mov_mem_reg_kat_llvm_mc_base32() {
    let mc = llvm_mc_bytes("movl (%eax), %ebx").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x8b, 0x18], "llvm-mc KAT mapping broken");
    let sut = sut_encode(
        "movl",
        vec![
            Operand::Memory(mem_base("eax", Displacement::None, None)),
            Operand::Register(Register::new("ebx")),
        ],
    )
    .expect("SUT KAT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_mov_mem_reg_kat_llvm_mc_16() {
    let mc = llvm_mc_bytes("movw (%eax), %bx").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x66, 0x8b, 0x18]);
    let sut = sut_encode(
        "movw",
        vec![
            Operand::Memory(mem_base("eax", Displacement::None, None)),
            Operand::Register(Register::new("bx")),
        ],
    )
    .expect("SUT KAT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_mov_mem_reg_kat_llvm_mc_8() {
    let mc = llvm_mc_bytes("movb (%eax), %bl").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x8a, 0x18]);
    let sut = sut_encode(
        "movb",
        vec![
            Operand::Memory(mem_base("eax", Displacement::None, None)),
            Operand::Register(Register::new("bl")),
        ],
    )
    .expect("SUT KAT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_mov_mem_reg_kat_llvm_mc_fs() {
    let mc = llvm_mc_bytes("movl %fs:(%eax), %ebx").expect("llvm-mc KAT fs");
    assert_eq!(mc, vec![0x64, 0x8b, 0x18]);
    let sut = sut_encode(
        "movl",
        vec![
            Operand::Memory(mem_base("eax", Displacement::None, Some("fs"))),
            Operand::Register(Register::new("ebx")),
        ],
    )
    .expect("SUT KAT fs");
    assert_eq!(sut, mc);
}

#[test]
fn encode_mov_mem_reg_kat_llvm_mc_esp_ebp() {
    let mc = llvm_mc_bytes("movl (%esp), %eax").expect("llvm-mc KAT esp");
    assert_eq!(mc, vec![0x8b, 0x04, 0x24]);
    let sut = sut_encode(
        "movl",
        vec![
            Operand::Memory(mem_base("esp", Displacement::None, None)),
            Operand::Register(Register::new("eax")),
        ],
    )
    .expect("SUT");
    assert_eq!(sut, mc);

    let mc = llvm_mc_bytes("movl (%ebp), %eax").expect("llvm-mc KAT ebp");
    assert_eq!(mc, vec![0x8b, 0x45, 0x00]);
    let sut = sut_encode(
        "movl",
        vec![
            Operand::Memory(mem_base("ebp", Displacement::None, None)),
            Operand::Register(Register::new("eax")),
        ],
    )
    .expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_mov_mem_reg_kat_llvm_mc_es_segment() {
    // Contract: all six segment overrides are valid on i686 (core.rs emit_segment_prefix).
    let mc = llvm_mc_bytes("movl %es:(%eax), %ebx").expect("llvm-mc KAT es");
    assert_eq!(mc, vec![0x26, 0x8b, 0x18]);
    let sut = sut_encode(
        "movl",
        vec![
            Operand::Memory(mem_base("eax", Displacement::None, Some("es"))),
            Operand::Register(Register::new("ebx")),
        ],
    );
    match sut {
        Ok(bytes) => assert_eq!(bytes, mc, "SUT must emit ES override 0x26 before 8B"),
        Err(e) => panic!("SUT erred on valid ES mem→reg: {e}"),
    }
}

proptest! {
    #![proptest_config(cfg())]

    // P1: differential — base+disp, no segment, same-width dest
    #[test]
    fn encode_mov_mem_reg_diff_llvm_mc_base_disp(
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
        di in 0usize..8,
    ) {
        let dst = gp_for_width(width)[di % 8];
        let mem = mem_base(base, Displacement::Integer(disp), None);
        let mnemonic = suffix_for(width);
        let asm = format!("{mnemonic} {}, %{}", att_mem(&mem), dst);
        let mc = llvm_mc_bytes(&asm)
            .map_err(|e| TestCaseError::fail(format!("llvm-mc rejected `{asm}`: {e}")))?;
        let sut = sut_encode(
            mnemonic,
            vec![
                Operand::Memory(mem),
                Operand::Register(Register::new(dst)),
            ],
        )
        .map_err(|e| TestCaseError::fail(format!("SUT rejected `{asm}`: {e}")))?;
        prop_assert_eq!(&sut, &mc, "diff `{}`: sut={:02x?} mc={:02x?}", asm, &sut, &mc);
    }

    // P2: differential — SIB forms
    #[test]
    fn encode_mov_mem_reg_diff_llvm_mc_sib(
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
        di in 0usize..8,
    ) {
        let dst = gp_for_width(width)[di % 8];
        let mem = mem_base_index(base, index, scale, Displacement::Integer(disp), None);
        let mnemonic = suffix_for(width);
        let asm = format!("{mnemonic} {}, %{}", att_mem(&mem), dst);
        let mc = match llvm_mc_bytes(&asm) {
            Ok(b) => b,
            Err(_) => return Ok(()),
        };
        let sut = sut_encode(
            mnemonic,
            vec![
                Operand::Memory(mem),
                Operand::Register(Register::new(dst)),
            ],
        )
        .map_err(|e| TestCaseError::fail(format!("SUT rejected `{asm}`: {e}")))?;
        prop_assert_eq!(&sut, &mc, "SIB diff `{}`: sut={:02x?} mc={:02x?}", asm, &sut, &mc);
    }

    // P3: differential — all six segment overrides
    #[test]
    fn encode_mov_mem_reg_diff_llvm_mc_segment(
        seg in prop::sample::select(SEG_REGS),
        base in prop::sample::select(GP32),
        disp in prop_oneof![Just(0i64), Just(8i64), Just(-4i64), Just(127i64), Just(-128i64)],
        width in prop::sample::select(vec![1u8, 2, 4]),
        di in 0usize..8,
    ) {
        let dst = gp_for_width(width)[di % 8];
        let mem = mem_base(base, Displacement::Integer(disp), Some(seg));
        let mnemonic = suffix_for(width);
        let asm = format!("{mnemonic} {}, %{}", att_mem(&mem), dst);
        let mc = llvm_mc_bytes(&asm)
            .map_err(|e| TestCaseError::fail(format!("llvm-mc rejected `{asm}`: {e}")))?;
        let sut = sut_encode(
            mnemonic,
            vec![
                Operand::Memory(mem),
                Operand::Register(Register::new(dst)),
            ],
        )
        .map_err(|e| TestCaseError::fail(format!(
            "SUT rejected valid segment form `{asm}`: {e}; \
             i686 MOV mem→reg must emit segment override (core.rs emit_segment_prefix; \
             Intel SDM 2.1.1). Body only accepts fs/gs."
        )))?;
        prop_assert_eq!(
            &sut, &mc,
            "segment diff `{}`: sut={:02x?} mc={:02x?}",
            asm, &sut, &mc
        );
    }

    // P4: differential — ESP/EBP/SIB/abs edges (skip moffs A0/A1)
    #[test]
    fn encode_mov_mem_reg_diff_edges_esp_ebp_abs(
        edge in 0u8..14,
        width in prop::sample::select(vec![1u8, 2, 4]),
        di in 0usize..8,
    ) {
        let dst = gp_for_width(width)[di % 8];
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
        let asm = format!("{mnemonic} {}, %{}", att_mem(&mem), dst);
        let mc = match llvm_mc_bytes(&asm) {
            Ok(b) => b,
            Err(e) => return Err(TestCaseError::fail(format!("llvm-mc edge `{asm}`: {e}"))),
        };
        // Encoding choice: llvm-mc may emit A0/A1 moffs for abs→al/ax/eax; SUT uses 8A/8B.
        if is_moffs_encoding(&mc) {
            return Ok(());
        }
        let sut = sut_encode(
            mnemonic,
            vec![
                Operand::Memory(mem),
                Operand::Register(Register::new(dst)),
            ],
        )
        .map_err(|e| TestCaseError::fail(format!("SUT edge `{asm}`: {e}")))?;
        prop_assert_eq!(&sut, &mc, "edge diff `{}`", asm);
    }

    // P5: algebraic.invariant — opcode 8A/8B, ModRM.reg = dst
    #[test]
    fn encode_mov_mem_reg_invariant_opcode_modrm(
        base in prop::sample::select(GP32),
        disp in prop_oneof![Just(0i64), Just(7i64), Just(-3i64), (-200i64..=300i64)],
        use_index in any::<bool>(),
        index in prop::sample::select(
            GP32.iter().copied().filter(|r| *r != "esp").collect::<Vec<_>>()
        ),
        scale in prop::sample::select(SCALES),
        width in prop::sample::select(vec![1u8, 2, 4]),
        di in 0usize..8,
    ) {
        let dst = gp_for_width(width)[di % 8];
        let mem = if use_index {
            mem_base_index(Some(base), index, scale, Displacement::Integer(disp), None)
        } else {
            mem_base(base, Displacement::Integer(disp), None)
        };
        let mnemonic = suffix_for(width);
        let bytes = sut_encode(
            mnemonic,
            vec![
                Operand::Memory(mem),
                Operand::Register(Register::new(dst)),
            ],
        )
        .map_err(|e| TestCaseError::fail(e))?;
        let mut i = 0usize;
        // optional 0x66 only
        if width == 2 {
            prop_assert_eq!(bytes[i], 0x66, "missing 0x66 for movw");
            i += 1;
        }
        let op = bytes[i];
        if width == 1 {
            prop_assert_eq!(op, 0x8A, "movb mem→reg opcode");
        } else {
            prop_assert_eq!(op, 0x8B, "movw/movl mem→reg opcode");
        }
        i += 1;
        prop_assert!(bytes.len() > i, "need ModRM");
        let modrm = bytes[i];
        prop_assert_eq!((modrm >> 3) & 7, gp_num(dst), "ModRM.reg = dst");
    }

    // P6: algebraic.metamorphic — load vs store share Mod+RM/SIB/disp (+seg/66)
    #[test]
    fn encode_mov_mem_reg_meta_load_vs_store_modrm(
        base in prop::sample::select(GP32),
        disp in prop_oneof![Just(0i64), Just(40i64), Just(-5i64), (-1000i64..=1000i64)],
        index in prop::option::of(prop::sample::select(
            GP32.iter().copied().filter(|r| *r != "esp").collect::<Vec<_>>()
        )),
        scale in prop::sample::select(SCALES),
        width in prop::sample::select(vec![1u8, 2, 4]),
        di in 0usize..8,
        // include fs/gs only for meta so both arms succeed under current SUT
        use_fs in any::<bool>(),
    ) {
        let reg = gp_for_width(width)[di % 8];
        let seg = if use_fs { Some("fs") } else { None };
        let mem = match index {
            Some(idx) => mem_base_index(Some(base), idx, scale, Displacement::Integer(disp), seg),
            None => mem_base(base, Displacement::Integer(disp), seg),
        };
        let mnemonic = suffix_for(width);
        let load = sut_encode(
            mnemonic,
            vec![
                Operand::Memory(mem.clone()),
                Operand::Register(Register::new(reg)),
            ],
        )
        .map_err(|e| TestCaseError::fail(format!("load: {e}")))?;
        let store = sut_encode(
            mnemonic,
            vec![
                Operand::Register(Register::new(reg)),
                Operand::Memory(mem),
            ],
        )
        .map_err(|e| TestCaseError::fail(format!("store: {e}")))?;
        // Find opcode indices after shared prefixes
        fn split_prefix_op(bytes: &[u8]) -> (Vec<u8>, u8, &[u8]) {
            let mut i = 0;
            let mut pref = Vec::new();
            while i < bytes.len()
                && matches!(bytes[i], 0x26 | 0x2E | 0x36 | 0x3E | 0x64 | 0x65 | 0x66)
            {
                pref.push(bytes[i]);
                i += 1;
            }
            let op = bytes[i];
            (pref, op, &bytes[i + 1..])
        }
        let (lp, lo, lt) = split_prefix_op(&load);
        let (sp, so, st) = split_prefix_op(&store);
        prop_assert_eq!(&lp, &sp, "prefixes must match load/store");
        if width == 1 {
            prop_assert_eq!(lo, 0x8A);
            prop_assert_eq!(so, 0x88);
        } else {
            prop_assert_eq!(lo, 0x8B);
            prop_assert_eq!(so, 0x89);
        }
        prop_assert_eq!(lt, st, "ModRM+SIB+disp must match across load/store");
    }

    // P7: negative — mismatched dest width must Err (llvm-mc rejects)
    #[test]
    fn encode_mov_mem_reg_neg_mismatched_width(
        mode in 0u8..6,
        bi in 0usize..8,
        di in 0usize..8,
        disp in prop_oneof![Just(0i64), Just(4i64), Just(-8i64)],
    ) {
        let base = GP32[bi % 8];
        let (mnemonic, dst, expected_w) = match mode {
            0 => ("movl", R16[di % 8], 4u8),
            1 => ("movl", R8[di % 8], 4u8),
            2 => ("movw", GP32[di % 8], 2u8),
            3 => ("movw", R8[di % 8], 2u8),
            4 => ("movb", GP32[di % 8], 1u8),
            _ => ("movb", R16[di % 8], 1u8),
        };
        prop_assume!(reg_size_local(dst) != expected_w);
        let mem = mem_base(base, Displacement::Integer(disp), None);
        let asm = format!("{mnemonic} {}, %{}", att_mem(&mem), dst);
        prop_assert!(
            llvm_mc_bytes(&asm).is_err(),
            "expected llvm-mc to reject mismatched `{asm}`"
        );
        match sut_encode(
            mnemonic,
            vec![
                Operand::Memory(mem),
                Operand::Register(Register::new(dst)),
            ],
        ) {
            Err(_) => {}
            Ok(bytes) => {
                return Err(TestCaseError::fail(format!(
                    "SUT accepted size-mismatched mem→reg `{asm}` → {bytes:02x?}; \
                     MOV r,m requires matching operand size (Intel SDM; llvm-mc rejects). \
                     reg_num aliases widths so bytes look like a valid same-width form."
                )));
            }
        }
    }

    // P8: negative — non-GP dest that aliases via reg_num
    #[test]
    fn encode_mov_mem_reg_neg_non_gp_dest(
        ni in 0usize..10,
        bi in 0usize..8,
        width in prop::sample::select(vec![1u8, 2, 4]),
        disp in prop_oneof![Just(0i64), Just(8i64)],
    ) {
        let non_gp = NON_GP[ni % NON_GP.len()];
        let base = GP32[bi % 8];
        let mnemonic = suffix_for(width);
        let mem = mem_base(base, Displacement::Integer(disp), None);
        let asm = format!("{mnemonic} {}, %{}", att_mem(&mem), non_gp);
        prop_assert!(
            llvm_mc_bytes(&asm).is_err(),
            "expected llvm-mc to reject non-GP dest `{asm}`"
        );
        match sut_encode(
            mnemonic,
            vec![
                Operand::Memory(mem),
                Operand::Register(Register::new(non_gp)),
            ],
        ) {
            Err(_) => {}
            Ok(bytes) => {
                return Err(TestCaseError::fail(format!(
                    "SUT accepted non-GP MOV mem→reg `{asm}` → {bytes:02x?}; \
                     8A/8B form is GP-only (Intel SDM). reg_num aliases xmm/mm/st to 0-7."
                )));
            }
        }
    }
}

// Deterministic regression witnesses (filled after PBT shrinks)

#[test]
fn encode_mov_mem_reg_regression_es_segment_prefix() {
    // Witness: movl %es:(%eax), %ebx — SUT returns Err("unsupported segment: es")
    // or omits 0x26; llvm-mc emits [26, 8b, 18].
    let mc = llvm_mc_bytes("movl %es:(%eax), %ebx").expect("llvm-mc");
    assert_eq!(mc, vec![0x26, 0x8b, 0x18]);
    let sut = sut_encode(
        "movl",
        vec![
            Operand::Memory(mem_base("eax", Displacement::None, Some("es"))),
            Operand::Register(Register::new("ebx")),
        ],
    );
    match sut {
        Ok(bytes) => assert_eq!(
            bytes, mc,
            "regression: ES override must be present"
        ),
        Err(e) => panic!("regression: SUT rejected valid ES form: {e}"),
    }
}

#[test]
fn encode_mov_mem_reg_regression_mismatched_width_ax() {
    // Witness: movl (%eax), %ax — llvm-mc rejects; SUT must Err.
    assert!(llvm_mc_bytes("movl (%eax), %ax").is_err());
    let r = sut_encode(
        "movl",
        vec![
            Operand::Memory(mem_base("eax", Displacement::None, None)),
            Operand::Register(Register::new("ax")),
        ],
    );
    assert!(
        r.is_err(),
        "regression: mismatched width must Err, got {r:?}"
    );
}
