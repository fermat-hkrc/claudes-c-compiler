// Oracle: differential — llvm-mc i686 assembler (MOVZX: 0F B6/B7 /r + optional 0x66 + seg)
// Evidence: gp_integer.rs:302-330 encode_movzx;
//   encoder/mod.rs:179-181 movzbl/movzwl/movzbw → encode_movzx(src,dst);
//   core.rs:31-42 emit_segment_prefix (es/cs/ss/ds/fs/gs) — NOT called by encode_movzx;
//   Intel SDM Vol.2 MOVZX — r16/r32,r/m8 (0F B6) / r32,r/m16 (0F B7);
//   AT&T order: movzbl/movzbw/movzwl src, %dst with ModRM.reg=dst, r/m=src.
// Stronger considered:
//   - State machine: rejected — pure encoding, no lifecycle
//   - Algebraic round-trip: rejected — no in-tree i686 MOVZX decoder
// Differential: candidate=encode_movzx (via InstructionEncoder::encode movzbl/movzbw/movzwl),
//   reference=llvm-mc -triple=i686 -show-encoding (LLVM 15),
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=AT&T operand bytes <-> llvm-mc encoding bytes.
// Also: algebraic.invariant (opcode/modrm), algebraic.metamorphic (movzx vs movsx),
//   negative_error (arity, mismatched widths, non-GP, unsupported shapes).

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
const R16: &[&str] = &["ax", "cx", "dx", "bx", "sp", "bp", "si", "di"];
const R8: &[&str] = &["al", "cl", "dl", "bl", "ah", "ch", "dh", "bh"];
const SEG_REGS: &[&str] = &["es", "cs", "ss", "ds", "fs", "gs"];
const SCALES: &[u8] = &[1, 2, 4, 8];
const NON_GP: &[&str] = &[
    "xmm0", "xmm1", "xmm7", "mm0", "mm3", "mm7", "st", "st(0)", "st(1)", "ymm0",
];

/// (mnemonic, src_size, dst_size, opcode_lo)
/// movzbl: 1→4, 0xB6; movzbw: 1→2, 0xB6; movzwl: 2→4, 0xB7
const FORMS: &[(&str, u8, u8, u8)] = &[
    ("movzbl", 1, 4, 0xB6),
    ("movzbw", 1, 2, 0xB6),
    ("movzwl", 2, 4, 0xB7),
];

fn src_regs(src_size: u8) -> &'static [&'static str] {
    match src_size {
        1 => R8,
        _ => R16,
    }
}

fn dst_regs(dst_size: u8) -> &'static [&'static str] {
    match dst_size {
        2 => R16,
        _ => GP32,
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
        // llvm-mc may emit fixup placeholders "A"
        if t.chars().all(|c| c.is_ascii_alphabetic()) {
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

fn strip_seg_os_prefixes(bytes: &[u8]) -> &[u8] {
    let mut i = 0;
    while i < bytes.len()
        && matches!(bytes[i], 0x26 | 0x2E | 0x36 | 0x3E | 0x64 | 0x65 | 0x66)
    {
        i += 1;
    }
    &bytes[i..]
}

fn form_at(i: usize) -> (&'static str, u8, u8, u8) {
    FORMS[i % FORMS.len()]
}

// --- KAT gate (reference oracle prerequisite) ---

#[test]
fn encode_movzx_kat_llvm_mc_rr_sbl() {
    let mc = llvm_mc_bytes("movzbl %al, %eax").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x0f, 0xb6, 0xc0], "llvm-mc KAT mapping broken");
    let sut = sut_encode(
        "movzbl",
        vec![
            Operand::Register(Register::new("al")),
            Operand::Register(Register::new("eax")),
        ],
    )
    .expect("SUT KAT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_movzx_kat_llvm_mc_rr_sbw() {
    let mc = llvm_mc_bytes("movzbw %al, %ax").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x66, 0x0f, 0xb6, 0xc0]);
    let sut = sut_encode(
        "movzbw",
        vec![
            Operand::Register(Register::new("al")),
            Operand::Register(Register::new("ax")),
        ],
    )
    .expect("SUT KAT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_movzx_kat_llvm_mc_rr_swl() {
    let mc = llvm_mc_bytes("movzwl %ax, %eax").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x0f, 0xb7, 0xc0]);
    let sut = sut_encode(
        "movzwl",
        vec![
            Operand::Register(Register::new("ax")),
            Operand::Register(Register::new("eax")),
        ],
    )
    .expect("SUT KAT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_movzx_kat_llvm_mc_mem() {
    let mc = llvm_mc_bytes("movzbl (%eax), %ecx").expect("llvm-mc KAT mem");
    assert_eq!(mc, vec![0x0f, 0xb6, 0x08]);
    let sut = sut_encode(
        "movzbl",
        vec![
            Operand::Memory(mem_base("eax", Displacement::None, None)),
            Operand::Register(Register::new("ecx")),
        ],
    )
    .expect("SUT KAT mem");
    assert_eq!(sut, mc);
}

#[test]
fn encode_movzx_kat_llvm_mc_ah() {
    let mc = llvm_mc_bytes("movzbl %ah, %ebx").expect("llvm-mc KAT ah");
    assert_eq!(mc, vec![0x0f, 0xb6, 0xdc]);
    let sut = sut_encode(
        "movzbl",
        vec![
            Operand::Register(Register::new("ah")),
            Operand::Register(Register::new("ebx")),
        ],
    )
    .expect("SUT KAT ah");
    assert_eq!(sut, mc);
}

#[test]
fn encode_movzx_kat_llvm_mc_es_segment() {
    // Contract: all six segment overrides are valid on i686 (core.rs emit_segment_prefix).
    let mc = llvm_mc_bytes("movzbl %es:(%eax), %ecx").expect("llvm-mc KAT es");
    assert_eq!(mc, vec![0x26, 0x0f, 0xb6, 0x08]);
    let sut = sut_encode(
        "movzbl",
        vec![
            Operand::Memory(mem_base("eax", Displacement::None, Some("es"))),
            Operand::Register(Register::new("ecx")),
        ],
    );
    match sut {
        Ok(bytes) => assert_eq!(bytes, mc, "SUT must emit ES override 0x26 before 0F B6"),
        Err(e) => panic!("SUT erred on valid ES movzx: {e}"),
    }
}

proptest! {
    #![proptest_config(cfg())]

    // P1: differential — RR same-size GP pairs match llvm-mc
    #[test]
    fn encode_movzx_diff_llvm_mc_rr(
        fi in 0usize..3,
        si in 0usize..8,
        di in 0usize..8,
    ) {
        let (mnemonic, src_size, dst_size, _) = form_at(fi);
        let src = src_regs(src_size)[si % 8];
        let dst = dst_regs(dst_size)[di % 8];
        let asm = format!("{mnemonic} %{src}, %{dst}");
        let mc = llvm_mc_bytes(&asm)
            .map_err(|e| TestCaseError::fail(format!("llvm-mc rejected `{asm}`: {e}")))?;
        let sut = sut_encode(
            mnemonic,
            vec![
                Operand::Register(Register::new(src)),
                Operand::Register(Register::new(dst)),
            ],
        )
        .map_err(|e| TestCaseError::fail(format!("SUT rejected `{asm}`: {e}")))?;
        prop_assert_eq!(&sut, &mc, "RR diff `{}`: sut={:02x?} mc={:02x?}", asm, &sut, &mc);
    }

    // P2: differential — base+disp memory source
    #[test]
    fn encode_movzx_diff_llvm_mc_base_disp(
        fi in 0usize..3,
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
        di in 0usize..8,
    ) {
        let (mnemonic, _src_size, dst_size, _) = form_at(fi);
        let dst = dst_regs(dst_size)[di % 8];
        let mem = mem_base(base, Displacement::Integer(disp), None);
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
        prop_assert_eq!(&sut, &mc, "base/disp diff `{}`: sut={:02x?} mc={:02x?}", asm, &sut, &mc);
    }

    // P3: differential — SIB forms
    #[test]
    fn encode_movzx_diff_llvm_mc_sib(
        fi in 0usize..3,
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
        di in 0usize..8,
    ) {
        let (mnemonic, _src_size, dst_size, _) = form_at(fi);
        let dst = dst_regs(dst_size)[di % 8];
        let mem = mem_base_index(base, index, scale, Displacement::Integer(disp), None);
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

    // P4: differential — all six segment overrides (documented contract: emit_segment_prefix)
    #[test]
    fn encode_movzx_diff_llvm_mc_segment(
        fi in 0usize..3,
        seg in prop::sample::select(SEG_REGS),
        base in prop::sample::select(GP32),
        disp in prop_oneof![Just(0i64), Just(8i64), Just(-4i64), Just(127i64), Just(-128i64)],
        di in 0usize..8,
    ) {
        let (mnemonic, _src_size, dst_size, _) = form_at(fi);
        let dst = dst_regs(dst_size)[di % 8];
        let mem = mem_base(base, Displacement::Integer(disp), Some(seg));
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
             i686 MOVZX mem→reg must emit segment override (core.rs emit_segment_prefix; \
             Intel SDM 2.1.1)."
        )))?;
        prop_assert_eq!(
            &sut, &mc,
            "segment diff `{}`: sut={:02x?} mc={:02x?}",
            asm, &sut, &mc
        );
    }

    // P4b strengthen: segment + SIB (order seg → 66? → 0F B6/B7)
    #[test]
    fn encode_movzx_diff_llvm_mc_segment_sib(
        fi in 0usize..3,
        seg in prop::sample::select(SEG_REGS),
        base in prop::sample::select(GP32),
        index in prop::sample::select(
            GP32.iter().copied().filter(|r| *r != "esp").collect::<Vec<_>>()
        ),
        scale in prop::sample::select(SCALES),
        disp in prop_oneof![Just(0i64), Just(8i64), Just(-4i64)],
        di in 0usize..8,
    ) {
        let (mnemonic, _src_size, dst_size, _) = form_at(fi);
        let dst = dst_regs(dst_size)[di % 8];
        let mem = mem_base_index(Some(base), index, scale, Displacement::Integer(disp), Some(seg));
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
        .map_err(|e| TestCaseError::fail(format!("SUT rejected segment+SIB `{asm}`: {e}")))?;
        prop_assert_eq!(
            &sut, &mc,
            "segment+SIB diff `{}`: sut={:02x?} mc={:02x?}",
            asm, &sut, &mc
        );
    }

    // P5: differential — ESP/EBP/SIB/abs edges
    #[test]
    fn encode_movzx_diff_edges_esp_ebp_abs(
        fi in 0usize..3,
        edge in 0u8..14,
        di in 0usize..8,
    ) {
        let (mnemonic, _src_size, dst_size, _) = form_at(fi);
        let dst = dst_regs(dst_size)[di % 8];
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
        let asm = format!("{mnemonic} {}, %{}", att_mem(&mem), dst);
        let mc = match llvm_mc_bytes(&asm) {
            Ok(b) => b,
            Err(e) => return Err(TestCaseError::fail(format!("llvm-mc edge `{asm}`: {e}"))),
        };
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

    // P6: algebraic.invariant — 0x66? + 0F B6/B7 + ModRM.reg = dst
    #[test]
    fn encode_movzx_invariant_opcode_modrm(
        fi in 0usize..3,
        si in 0usize..8,
        di in 0usize..8,
        use_mem in any::<bool>(),
        base in prop::sample::select(GP32),
        disp in prop_oneof![Just(0i64), Just(7i64), Just(-3i64), (-200i64..=300i64)],
    ) {
        let (mnemonic, src_size, dst_size, op_lo) = form_at(fi);
        let dst = dst_regs(dst_size)[di % 8];
        let (ops, rm_is_reg, src_name) = if use_mem {
            (
                vec![
                    Operand::Memory(mem_base(base, Displacement::Integer(disp), None)),
                    Operand::Register(Register::new(dst)),
                ],
                false,
                base,
            )
        } else {
            let src = src_regs(src_size)[si % 8];
            (
                vec![
                    Operand::Register(Register::new(src)),
                    Operand::Register(Register::new(dst)),
                ],
                true,
                src,
            )
        };
        let bytes = sut_encode(mnemonic, ops).map_err(|e| TestCaseError::fail(e))?;
        let mut i = 0usize;
        if dst_size == 2 {
            prop_assert_eq!(bytes.get(i).copied(), Some(0x66), "missing 0x66 for movzbw");
            i += 1;
        }
        prop_assert_eq!(bytes.get(i).copied(), Some(0x0F), "MOVZX escape 0F");
        i += 1;
        prop_assert_eq!(bytes.get(i).copied(), Some(op_lo), "MOVZX opcode lo");
        i += 1;
        prop_assert!(bytes.len() > i, "need ModRM");
        let modrm = bytes[i];
        prop_assert_eq!((modrm >> 3) & 7, gp_num(dst), "ModRM.reg = dst");
        if rm_is_reg {
            prop_assert_eq!(modrm >> 6, 3, "RR mod=11");
            prop_assert_eq!(modrm & 7, gp_num(src_name), "ModRM.rm = src");
        }
    }

    // P7: algebraic.metamorphic — movzx vs movsx share prefixes+ModRM; only opcode lo differs
    // B6↔BE (byte) or B7↔BF (word). Evidence: Intel SDM MOVZX/MOVSX; body gp_integer twin.
    #[test]
    fn encode_movzx_meta_vs_movsx(
        fi in 0usize..3,
        si in 0usize..8,
        di in 0usize..8,
        use_mem in any::<bool>(),
        base in prop::sample::select(GP32),
        disp in prop_oneof![Just(0i64), Just(16i64), Just(-8i64), (-100i64..=100i64)],
    ) {
        let (zx_mn, src_size, dst_size, zx_lo) = form_at(fi);
        let sx_mn = match zx_mn {
            "movzbl" => "movsbl",
            "movzbw" => "movsbw",
            "movzwl" => "movswl",
            _ => unreachable!(),
        };
        let sx_lo = match zx_lo {
            0xB6 => 0xBEu8,
            0xB7 => 0xBFu8,
            _ => unreachable!(),
        };
        let dst = dst_regs(dst_size)[di % 8];
        let ops = if use_mem {
            vec![
                Operand::Memory(mem_base(base, Displacement::Integer(disp), None)),
                Operand::Register(Register::new(dst)),
            ]
        } else {
            let src = src_regs(src_size)[si % 8];
            vec![
                Operand::Register(Register::new(src)),
                Operand::Register(Register::new(dst)),
            ]
        };
        let zx = sut_encode(zx_mn, ops.clone()).map_err(|e| TestCaseError::fail(format!("zx: {e}")))?;
        let sx = sut_encode(sx_mn, ops).map_err(|e| TestCaseError::fail(format!("sx: {e}")))?;
        prop_assert_eq!(zx.len(), sx.len(), "movzx/movsx length");
        let body_zx = strip_seg_os_prefixes(&zx);
        let body_sx = strip_seg_os_prefixes(&sx);
        prop_assert!(body_zx.len() >= 3 && body_sx.len() >= 3);
        prop_assert_eq!(body_zx[0], 0x0F);
        prop_assert_eq!(body_sx[0], 0x0F);
        prop_assert_eq!(body_zx[1], zx_lo);
        prop_assert_eq!(body_sx[1], sx_lo);
        prop_assert_eq!(&body_zx[2..], &body_sx[2..], "ModRM+rest must match across zx/sx");
        let pref_zx: Vec<u8> = zx[..zx.len() - body_zx.len()].to_vec();
        let pref_sx: Vec<u8> = sx[..sx.len() - body_sx.len()].to_vec();
        prop_assert_eq!(&pref_zx, &pref_sx, "OS/seg prefixes must match");
    }

    // P8: negative — wrong arity must Err
    #[test]
    fn encode_movzx_neg_arity(
        fi in 0usize..3,
        n in 0usize..4,
        r0 in prop::sample::select(R8),
        r1 in prop::sample::select(GP32),
        r2 in prop::sample::select(GP32),
    ) {
        let (mnemonic, _, _, _) = form_at(fi);
        prop_assume!(n != 2);
        let mut ops = Vec::new();
        if n >= 1 {
            ops.push(Operand::Register(Register::new(r0)));
        }
        if n >= 2 {
            ops.push(Operand::Register(Register::new(r1)));
        }
        if n >= 3 {
            ops.push(Operand::Register(Register::new(r2)));
        }
        match sut_encode(mnemonic, ops) {
            Err(e) => {
                prop_assert!(
                    e.contains("requires 2") || e.contains("movzx"),
                    "arity error text: {e}"
                );
            }
            Ok(bytes) => {
                return Err(TestCaseError::fail(format!(
                    "SUT accepted {n}-operand {mnemonic} → {bytes:02x?}"
                )));
            }
        }
    }

    // P9: negative — mismatched register widths (llvm-mc rejects; SUT must Err)
    #[test]
    fn encode_movzx_neg_mismatched_width(
        mode in 0u8..9,
        si in 0usize..8,
        di in 0usize..8,
    ) {
        let (mnemonic, src, dst) = match mode {
            // movzbl needs r8→r32
            0 => ("movzbl", R16[si % 8], GP32[di % 8]),
            1 => ("movzbl", GP32[si % 8], GP32[di % 8]),
            2 => ("movzbl", R8[si % 8], R16[di % 8]),
            // movzbw needs r8→r16
            3 => ("movzbw", R16[si % 8], R16[di % 8]),
            4 => ("movzbw", R8[si % 8], GP32[di % 8]),
            5 => ("movzbw", GP32[si % 8], R16[di % 8]),
            // movzwl needs r16→r32
            6 => ("movzwl", R8[si % 8], GP32[di % 8]),
            7 => ("movzwl", GP32[si % 8], GP32[di % 8]),
            _ => ("movzwl", R16[si % 8], R16[di % 8]),
        };
        let expected_src = match mnemonic {
            "movzbl" | "movzbw" => 1u8,
            _ => 2u8,
        };
        let expected_dst = match mnemonic {
            "movzbw" => 2u8,
            _ => 4u8,
        };
        prop_assume!(reg_size_local(src) != expected_src || reg_size_local(dst) != expected_dst);
        let asm = format!("{mnemonic} %{src}, %{dst}");
        prop_assert!(
            llvm_mc_bytes(&asm).is_err(),
            "expected llvm-mc to reject mismatched `{asm}`"
        );
        match sut_encode(
            mnemonic,
            vec![
                Operand::Register(Register::new(src)),
                Operand::Register(Register::new(dst)),
            ],
        ) {
            Err(_) => {}
            Ok(bytes) => {
                return Err(TestCaseError::fail(format!(
                    "SUT accepted size-mismatched MOVZX `{asm}` → {bytes:02x?}; \
                     MOVZX requires matching operand sizes (Intel SDM; llvm-mc rejects). \
                     encode_movzx ignores register widths and only uses mnemonic sizes."
                )));
            }
        }
    }

    // P10: negative — non-GP registers that alias via reg_num
    #[test]
    fn encode_movzx_neg_non_gp(
        fi in 0usize..3,
        ni in 0usize..10,
        on_src in any::<bool>(),
        gi in 0usize..8,
    ) {
        let (mnemonic, src_size, dst_size, _) = form_at(fi);
        let non_gp = NON_GP[ni % NON_GP.len()];
        let (src, dst) = if on_src {
            (non_gp, dst_regs(dst_size)[gi % 8])
        } else {
            (src_regs(src_size)[gi % 8], non_gp)
        };
        let asm = format!("{mnemonic} %{src}, %{dst}");
        prop_assert!(
            llvm_mc_bytes(&asm).is_err(),
            "expected llvm-mc to reject non-GP `{asm}`"
        );
        match sut_encode(
            mnemonic,
            vec![
                Operand::Register(Register::new(src)),
                Operand::Register(Register::new(dst)),
            ],
        ) {
            Err(_) => {}
            Ok(bytes) => {
                return Err(TestCaseError::fail(format!(
                    "SUT accepted non-GP MOVZX `{asm}` → {bytes:02x?}; \
                     0F B6/B7 form is GP-only (Intel SDM). reg_num aliases xmm/mm/st to 0-7."
                )));
            }
        }
    }

    // P11 (contract-surface sweep): negative — unsupported operand shapes must Err
    // Documented path: gp_integer.rs `_ => return Err("unsupported movzx operands")`
    // and Reg→Mem / Imm→Reg / Mem→Mem are not MOVZX forms (Intel SDM; llvm-mc rejects).
    #[test]
    fn encode_movzx_neg_unsupported_shape(
        fi in 0usize..3,
        shape in 0u8..5,
        r0 in prop::sample::select(R8),
        r1 in prop::sample::select(GP32),
        base in prop::sample::select(GP32),
        imm in -128i64..=127i64,
    ) {
        let (mnemonic, _src_size, dst_size, _) = form_at(fi);
        let dst = dst_regs(dst_size)[0];
        let ops = match shape {
            0 => vec![ // Imm → Reg
                Operand::Immediate(ImmediateValue::Integer(imm)),
                Operand::Register(Register::new(dst)),
            ],
            1 => vec![ // Reg → Mem (wrong direction)
                Operand::Register(Register::new(r0)),
                Operand::Memory(mem_base(base, Displacement::None, None)),
            ],
            2 => vec![ // Mem → Mem
                Operand::Memory(mem_base(base, Displacement::None, None)),
                Operand::Memory(mem_base("ecx", Displacement::None, None)),
            ],
            3 => vec![ // Reg → Imm
                Operand::Register(Register::new(r0)),
                Operand::Immediate(ImmediateValue::Integer(imm)),
            ],
            _ => vec![ // Imm → Mem
                Operand::Immediate(ImmediateValue::Integer(imm)),
                Operand::Memory(mem_base(base, Displacement::None, None)),
            ],
        };
        let _ = r1; // keep generator live
        match sut_encode(mnemonic, ops) {
            Err(e) => {
                prop_assert!(
                    e.contains("unsupported") || e.contains("movzx") || e.contains("requires"),
                    "unexpected error text for unsupported shape: {e}"
                );
            }
            Ok(bytes) => {
                return Err(TestCaseError::fail(format!(
                    "SUT accepted unsupported MOVZX shape={shape} → {bytes:02x?}; \
                     only Reg→Reg and Mem→Reg are valid (Intel SDM MOVZX)."
                )));
            }
        }
    }
}

// Deterministic regression witnesses

#[test]
fn encode_movzx_regression_es_segment_prefix() {
    // Witness: movzbl %es:(%eax), %ecx — SUT omits 0x26; llvm-mc emits [26, 0f, b6, 08].
    let mc = llvm_mc_bytes("movzbl %es:(%eax), %ecx").expect("llvm-mc");
    assert_eq!(mc, vec![0x26, 0x0f, 0xb6, 0x08]);
    let sut = sut_encode(
        "movzbl",
        vec![
            Operand::Memory(mem_base("eax", Displacement::None, Some("es"))),
            Operand::Register(Register::new("ecx")),
        ],
    );
    match sut {
        Ok(bytes) => assert_eq!(bytes, mc, "regression: ES override must be present"),
        Err(e) => panic!("regression: SUT rejected valid ES form: {e}"),
    }
}

#[test]
fn encode_movzx_regression_mismatched_width_eax_src() {
    // Witness: movzbl %eax, %ebx — llvm-mc rejects; SUT must Err.
    assert!(llvm_mc_bytes("movzbl %eax, %ebx").is_err());
    let r = sut_encode(
        "movzbl",
        vec![
            Operand::Register(Register::new("eax")),
            Operand::Register(Register::new("ebx")),
        ],
    );
    assert!(
        r.is_err(),
        "regression: mismatched width must Err, got {r:?}"
    );
}

#[test]
fn encode_movzx_regression_non_gp_xmm_dst() {
    assert!(llvm_mc_bytes("movzbl %al, %xmm0").is_err());
    let r = sut_encode(
        "movzbl",
        vec![
            Operand::Register(Register::new("al")),
            Operand::Register(Register::new("xmm0")),
        ],
    );
    assert!(r.is_err(), "regression: non-GP dst must Err, got {r:?}");
}
