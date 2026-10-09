// Oracle: differential — llvm-mc i686 assembler (BSF/BSR r16, r/m16)
// Evidence: system.rs:352 "Encode 16-bit BSF/BSR: bsfw/bsrw";
//   mod.rs:233 "bsrw" | "bsfw" => encode_bsr_bsf_16;
//   sibling encode_bsr_bsf (gp_integer.rs:984) handles bsrl/bsfl without 0x66;
//   Intel SDM Vol.2 BSF/BSR — r16, r/m16 with opcode 0F BC /r (BSF) / 0F BD /r (BSR);
//   operand-size override 0x66 required for 16-bit forms in 32-bit code.
// Stronger considered:
//   - State machine: rejected — pure encoding, no lifecycle
//   - Algebraic round-trip: rejected — no in-tree i686 decoder for bsfw/bsrw
// Differential: candidate=encode_bsr_bsf_16 (via InstructionEncoder::encode),
//   reference=llvm-mc -triple=i686 -show-encoding (LLVM 15),
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=AT&T operand bytes <-> llvm-mc encoding bytes.
// Also: algebraic.invariant (r16 = 66 0F BC/BD /r),
//   algebraic.metamorphic (bsfw = 66 ‖ bsfl),
//   negative_error (arity / r32 / r8 / unsupported ops).

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

const R16: &[&str] = &["ax", "cx", "dx", "bx", "sp", "bp", "si", "di"];
const SREG: &[&str] = &["es", "cs", "ss", "ds", "fs", "gs"];
const GP32: &[&str] = &["eax", "ecx", "edx", "ebx", "esp", "ebp", "esi", "edi"];
const SCALES: &[u8] = &[1, 2, 4, 8];
const MNEMS16: &[&str] = &["bsfw", "bsrw"];
const BAD_WIDTH: &[&str] = &[
    "eax", "ecx", "edx", "ebx", "esp", "ebp", "esi", "edi",
    "al", "cl", "dl", "bl", "ah", "ch", "dh", "bh",
];

const R16_R32: &[(&str, &str)] = &[
    ("ax", "eax"),
    ("cx", "ecx"),
    ("dx", "edx"),
    ("bx", "ebx"),
    ("sp", "esp"),
    ("bp", "ebp"),
    ("si", "esi"),
    ("di", "edi"),
];

fn reg_num(name: &str) -> u8 {
    match name {
        "ax" | "eax" | "al" => 0,
        "cx" | "ecx" | "cl" => 1,
        "dx" | "edx" | "dl" => 2,
        "bx" | "ebx" | "bl" => 3,
        "sp" | "esp" | "ah" => 4,
        "bp" | "ebp" | "ch" => 5,
        "si" | "esi" | "dh" => 6,
        "di" | "edi" | "bh" => 7,
        _ => panic!("bad reg {name}"),
    }
}

fn opc(mnemonic: &str) -> u8 {
    match mnemonic {
        "bsfw" | "bsfl" | "bsf" => 0xBC,
        "bsrw" | "bsrl" | "bsr" => 0xBD,
        _ => panic!("bad mnem {mnemonic}"),
    }
}

fn m32_for(m16: &str) -> &'static str {
    match m16 {
        "bsfw" => "bsfl",
        "bsrw" => "bsrl",
        _ => panic!("bad m16 {m16}"),
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

// --- KAT gate (reference oracle prerequisite) ---

#[test]
fn encode_bsr_bsf_16_kat_llvm_mc_bsfw_ax_bx() {
    let mc = llvm_mc_bytes("bsfw %ax, %bx").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x66, 0x0f, 0xbc, 0xd8], "llvm-mc KAT mapping broken");
    let sut = sut_encode(
        "bsfw",
        vec![
            Operand::Register(Register::new("ax")),
            Operand::Register(Register::new("bx")),
        ],
    )
    .expect("SUT KAT");
    assert_eq!(sut, mc, "SUT KAT bsfw %ax, %bx");
}

#[test]
fn encode_bsr_bsf_16_kat_llvm_mc_bsrw_cx_dx() {
    let mc = llvm_mc_bytes("bsrw %cx, %dx").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x66, 0x0f, 0xbd, 0xd1]);
    let sut = sut_encode(
        "bsrw",
        vec![
            Operand::Register(Register::new("cx")),
            Operand::Register(Register::new("dx")),
        ],
    )
    .expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_bsr_bsf_16_kat_llvm_mc_mem() {
    let mc = llvm_mc_bytes("bsfw (%eax), %bx").expect("llvm-mc KAT mem");
    assert_eq!(mc, vec![0x66, 0x0f, 0xbc, 0x18]);
    let sut = sut_encode(
        "bsfw",
        vec![
            Operand::Memory(mem_base("eax", Displacement::None, None)),
            Operand::Register(Register::new("bx")),
        ],
    )
    .expect("SUT mem");
    assert_eq!(sut, mc);
}

#[test]
fn encode_bsr_bsf_16_kat_llvm_mc_segment_es() {
    // Segment override must precede 0x66: [26, 66, 0f, bc, 18]
    let mc = llvm_mc_bytes("bsfw %es:(%eax), %bx").expect("llvm-mc KAT es");
    assert_eq!(mc, vec![0x26, 0x66, 0x0f, 0xbc, 0x18]);
    let sut = sut_encode(
        "bsfw",
        vec![
            Operand::Memory(mem_base("eax", Displacement::None, Some("es"))),
            Operand::Register(Register::new("bx")),
        ],
    );
    match sut {
        Ok(bytes) => assert_eq!(bytes, mc, "SUT KAT bsfw %es:(%eax), %bx must include 0x26"),
        Err(e) => panic!("SUT rejected bsfw %es:(%eax), %bx: {e}"),
    }
}

#[test]
fn encode_bsr_bsf_16_kat_llvm_mc_segment_fs() {
    let mc = llvm_mc_bytes("bsrw %fs:4(%esi), %di").expect("llvm-mc KAT fs");
    assert_eq!(mc, vec![0x64, 0x66, 0x0f, 0xbd, 0x7e, 0x04]);
    let sut = sut_encode(
        "bsrw",
        vec![
            Operand::Memory(mem_base("esi", Displacement::Integer(4), Some("fs"))),
            Operand::Register(Register::new("di")),
        ],
    );
    match sut {
        Ok(bytes) => assert_eq!(bytes, mc, "SUT KAT bsrw %fs:4(%esi), %di"),
        Err(e) => panic!("SUT rejected: {e}"),
    }
}

proptest! {
    #![proptest_config(cfg())]

    // Oracle: differential — bsfw/bsrw r16, r16
    #[test]
    fn encode_bsr_bsf_16_diff_r16(
        mnemonic in prop::sample::select(MNEMS16),
        src in prop::sample::select(R16),
        dst in prop::sample::select(R16),
    ) {
        let asm = format!("{mnemonic} %{src}, %{dst}");
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let sut = sut_encode(
            mnemonic,
            vec![
                Operand::Register(Register::new(src)),
                Operand::Register(Register::new(dst)),
            ],
        )
        .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(sut, mc, "r16 diff vs llvm-mc for {}", asm);
    }

    // Oracle: differential — memory source (no segment)
    #[test]
    fn encode_bsr_bsf_16_diff_mem(
        mnemonic in prop::sample::select(MNEMS16),
        dst in prop::sample::select(R16),
        kind in 0u8..6,
        base in prop::sample::select(GP32),
        index in prop::sample::select(GP32),
        scale in prop::sample::select(SCALES),
        disp in prop_oneof![
            Just(0i64),
            Just(4i64),
            Just(-1i64),
            Just(0x1234i64),
            any::<i8>().prop_map(|v| v as i64),
        ],
    ) {
        prop_assume!(index != "esp" || kind < 3);

        let mem = match kind {
            0 => mem_base(base, Displacement::None, None),
            1 => mem_base(base, Displacement::Integer(disp), None),
            2 => mem_base("esp", Displacement::None, None),
            3 => mem_base("ebp", Displacement::Integer(disp.max(1).min(127)), None),
            4 => mem_base_index(Some(base), index, scale, Displacement::Integer(disp), None),
            _ => mem_abs(disp, None),
        };

        let att = att_mem(&mem);
        let asm = format!("{mnemonic} {att}, %{dst}");
        let mc = match llvm_mc_bytes(&asm) {
            Ok(b) => b,
            Err(_e) => {
                prop_assume!(false);
                return Ok(());
            }
        };
        let sut = sut_encode(
            mnemonic,
            vec![
                Operand::Memory(mem),
                Operand::Register(Register::new(dst)),
            ],
        )
        .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(sut, mc, "mem diff vs llvm-mc for {}", asm);
    }

    // Oracle: differential — memory with segment override
    #[test]
    fn encode_bsr_bsf_16_diff_mem_segment(
        mnemonic in prop::sample::select(MNEMS16),
        mseg in prop::sample::select(SREG),
        base in prop::sample::select(GP32),
        dst in prop::sample::select(R16),
        disp in prop_oneof![Just(0i64), Just(4i64), Just(8i64)],
    ) {
        let mem = mem_base(base, Displacement::Integer(disp), Some(mseg));
        let att = att_mem(&mem);
        let asm = format!("{mnemonic} {att}, %{dst}");
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let sut = sut_encode(
            mnemonic,
            vec![
                Operand::Memory(mem),
                Operand::Register(Register::new(dst)),
            ],
        )
        .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(
            sut, mc,
            "segmented mem must include override before 0x66 for {}",
            asm
        );
    }

    // Oracle: algebraic.invariant — r16 form layout
    #[test]
    fn encode_bsr_bsf_16_invariant_r16(
        mnemonic in prop::sample::select(MNEMS16),
        src in prop::sample::select(R16),
        dst in prop::sample::select(R16),
    ) {
        let bytes = sut_encode(
            mnemonic,
            vec![
                Operand::Register(Register::new(src)),
                Operand::Register(Register::new(dst)),
            ],
        )
        .expect("valid bsfw/bsrw r16 must encode");
        let expect = vec![
            0x66u8,
            0x0Fu8,
            opc(mnemonic),
            0xC0u8 | (reg_num(dst) << 3) | reg_num(src),
        ];
        prop_assert_eq!(bytes, expect, "invariant layout for {} %{}/{}", mnemonic, src, dst);
    }

    // Oracle: algebraic.metamorphic — bsfw/bsrw = 0x66 ‖ bsfl/bsrl
    #[test]
    fn encode_bsr_bsf_16_metamorphic_vs_32(
        mnemonic in prop::sample::select(MNEMS16),
        src_pair in prop::sample::select(R16_R32),
        dst_pair in prop::sample::select(R16_R32),
    ) {
        let (s16, s32) = src_pair;
        let (d16, d32) = dst_pair;
        let m32 = m32_for(mnemonic);
        let w = sut_encode(
            mnemonic,
            vec![
                Operand::Register(Register::new(s16)),
                Operand::Register(Register::new(d16)),
            ],
        )
        .expect("16-bit");
        let l = sut_encode(
            m32,
            vec![
                Operand::Register(Register::new(s32)),
                Operand::Register(Register::new(d32)),
            ],
        )
        .expect("32-bit");
        let mut expect = vec![0x66u8];
        expect.extend_from_slice(&l);
        prop_assert_eq!(
            &w, &expect,
            "{} %{},%{} must be 0x66 || {} %{},%{} (w={:02x?} l={:02x?})",
            mnemonic, s16, d16, m32, s32, d32, w, l
        );
    }

    // Oracle: negative_error — wrong arity
    #[test]
    fn encode_bsr_bsf_16_neg_arity(
        mnemonic in prop::sample::select(MNEMS16),
        n in 0usize..4,
    ) {
        prop_assume!(n != 2);
        let ops: Vec<Operand> = (0..n)
            .map(|i| {
                if i % 2 == 0 {
                    Operand::Register(Register::new("ax"))
                } else {
                    Operand::Register(Register::new("bx"))
                }
            })
            .collect();
        let r = sut_encode(mnemonic, ops);
        prop_assert!(r.is_err(), "arity {n} must be Err, got {r:?}");
    }

    // Oracle: negative_error — r32/r8 invalid for bsfw/bsrw (llvm-mc rejects)
    #[test]
    fn encode_bsr_bsf_16_neg_wrong_width(
        mnemonic in prop::sample::select(MNEMS16),
        bad in prop::sample::select(BAD_WIDTH),
        good in prop::sample::select(R16),
        bad_is_src in any::<bool>(),
    ) {
        let (src, dst) = if bad_is_src {
            (bad, good)
        } else {
            (good, bad)
        };
        let asm = format!("{mnemonic} %{src}, %{dst}");
        prop_assert!(
            llvm_mc_bytes(&asm).is_err(),
            "llvm-mc unexpectedly accepted `{asm}`"
        );
        let r = sut_encode(
            mnemonic,
            vec![
                Operand::Register(Register::new(src)),
                Operand::Register(Register::new(dst)),
            ],
        );
        prop_assert!(
            r.is_err(),
            "`{asm}` must be Err like llvm-mc, got Ok({:02x?})",
            r.as_ref().unwrap_or(&vec![])
        );
    }

    // Oracle: negative_error — unsupported operand shapes (dst must be register)
    #[test]
    fn encode_bsr_bsf_16_neg_unsupported_ops(
        mnemonic in prop::sample::select(MNEMS16),
        kind in 0u8..3,
        r in prop::sample::select(R16),
        base in prop::sample::select(GP32),
    ) {
        let ops = match kind {
            // mem, mem
            0 => vec![
                Operand::Memory(mem_base(base, Displacement::None, None)),
                Operand::Memory(mem_base(base, Displacement::None, None)),
            ],
            // reg, mem  (dst is memory — invalid for BSF/BSR)
            1 => vec![
                Operand::Register(Register::new(r)),
                Operand::Memory(mem_base(base, Displacement::None, None)),
            ],
            // two identical regs is valid — use imm instead
            _ => vec![
                Operand::Register(Register::new(r)),
                Operand::Register(Register::new(r)), // placeholder replaced below
            ],
        };
        let ops = if kind >= 2 {
            use crate::backend::x86::assembler::parser::ImmediateValue;
            vec![
                Operand::Immediate(ImmediateValue::Integer(0)),
                Operand::Register(Register::new(r)),
            ]
        } else {
            ops
        };
        let r = sut_encode(mnemonic, ops);
        prop_assert!(r.is_err(), "unsupported ops kind {kind} must be Err, got {r:?}");
    }
}

// Deterministic regression witnesses (filled after triage)

#[test]
fn test_encode_bsr_bsf_16_regression_segment_es_missing() {
    // Witness: bsfw %es:(%eax), %bx → SUT omits 0x26
    let mc = llvm_mc_bytes("bsfw %es:(%eax), %bx").expect("llvm-mc");
    assert_eq!(mc, vec![0x26, 0x66, 0x0f, 0xbc, 0x18]);
    let sut = sut_encode(
        "bsfw",
        vec![
            Operand::Memory(mem_base("eax", Displacement::None, Some("es"))),
            Operand::Register(Register::new("bx")),
        ],
    )
    .expect("SUT should encode");
    assert_eq!(
        sut, mc,
        "regression: bsfw %es:(%eax), %bx must include 0x26 (got {:02x?})",
        sut
    );
}

#[test]
fn test_encode_bsr_bsf_16_regression_r32_src_accepted() {
    // Witness: bsfw %eax, %bx should be Err (llvm-mc rejects); SUT may emit 66 0f bc d8
    assert!(llvm_mc_bytes("bsfw %eax, %bx").is_err());
    let sut = sut_encode(
        "bsfw",
        vec![
            Operand::Register(Register::new("eax")),
            Operand::Register(Register::new("bx")),
        ],
    );
    assert!(
        sut.is_err(),
        "regression: bsfw %eax, %bx must be Err, got Ok({:02x?})",
        sut.unwrap_or_default()
    );
}

#[test]
fn test_encode_bsr_bsf_16_regression_r8_dst_accepted() {
    assert!(llvm_mc_bytes("bsrw %ax, %al").is_err());
    let sut = sut_encode(
        "bsrw",
        vec![
            Operand::Register(Register::new("ax")),
            Operand::Register(Register::new("al")),
        ],
    );
    assert!(
        sut.is_err(),
        "regression: bsrw %ax, %al must be Err, got Ok({:02x?})",
        sut.unwrap_or_default()
    );
}
