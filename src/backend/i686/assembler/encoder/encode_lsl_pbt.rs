// Oracle: differential — llvm-mc i686 assembler (LSL 0F 03 /r)
// Evidence: system.rs:143 "Encode LSL (Load Segment Limit): 0F 03 /r";
//   encoder/mod.rs:337 "lsl" => encode_lsl(ops);
//   Intel SDM LSL r16/r32, r/m16: opcode 0F 03 /r; osize from destination;
//   sibling x86 encode_lsl at x86/.../system.rs:497 emits REX (out of i686 ISA);
//   i686 core.rs:31-42 emit_segment_prefix for fs/gs/es/cs/ss/ds.
// Stronger considered:
//   - State machine: rejected — pure encoding, no lifecycle
//   - Algebraic round-trip: rejected — no in-tree i686 decoder for lsl
// Differential: candidate=encode_lsl (via InstructionEncoder::encode),
//   reference=llvm-mc -triple=i686 -show-encoding (LLVM 15),
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=AT&T operand bytes <-> llvm-mc encoding bytes.
// Also: algebraic.invariant (opcode 0F 03 + ModRM.reg=dst),
//   negative_error (arity + bad shapes).

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

const GP_REGS: &[&str] = &["eax", "ecx", "edx", "ebx", "esp", "ebp", "esi", "edi"];
const R16_REGS: &[&str] = &["ax", "cx", "dx", "bx", "sp", "bp", "si", "di"];
const MIXED_REGS: &[&str] = &[
    "eax", "ecx", "edx", "ebx", "esp", "ebp", "esi", "edi", "ax", "cx", "dx", "bx", "sp", "bp",
    "si", "di",
];
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

fn strip_prefixes(bytes: &[u8]) -> &[u8] {
    let mut i = 0;
    while i < bytes.len()
        && matches!(
            bytes[i],
            0x26 | 0x2E | 0x36 | 0x3E | 0x64 | 0x65 | 0x66 | 0x67
        )
    {
        i += 1;
    }
    &bytes[i..]
}

fn is_r16(name: &str) -> bool {
    matches!(name, "ax" | "bx" | "cx" | "dx" | "si" | "di" | "sp" | "bp")
}

// --- KAT gate ---

#[test]
fn encode_lsl_kat_llvm_mc_eax_ebx() {
    let mc = llvm_mc_bytes("lsl %eax, %ebx").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x0f, 0x03, 0xd8], "llvm-mc KAT mapping broken");
    let sut = sut_encode(
        "lsl",
        vec![
            Operand::Register(Register::new("eax")),
            Operand::Register(Register::new("ebx")),
        ],
    )
    .expect("SUT KAT");
    assert_eq!(sut, mc, "SUT KAT lsl %eax, %ebx");
}

#[test]
fn encode_lsl_kat_llvm_mc_ax_bx() {
    let mc = llvm_mc_bytes("lsl %ax, %bx").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x66, 0x0f, 0x03, 0xd8]);
    let sut = sut_encode(
        "lsl",
        vec![
            Operand::Register(Register::new("ax")),
            Operand::Register(Register::new("bx")),
        ],
    )
    .expect("SUT");
    assert_eq!(sut, mc, "SUT KAT lsl %ax, %bx");
}

#[test]
fn encode_lsl_kat_llvm_mc_mem_ebx() {
    let mc = llvm_mc_bytes("lsl (%eax), %ebx").expect("llvm-mc KAT mem");
    assert_eq!(mc, vec![0x0f, 0x03, 0x18]);
    let sut = sut_encode(
        "lsl",
        vec![
            Operand::Memory(mem_base("eax", Displacement::None, None)),
            Operand::Register(Register::new("ebx")),
        ],
    )
    .expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_lsl_kat_llvm_mc_mem16_bx() {
    // Dest is 16-bit → must emit 0x66 before 0F 03
    let mc = llvm_mc_bytes("lsl (%eax), %bx").expect("llvm-mc KAT mem16");
    assert_eq!(mc, vec![0x66, 0x0f, 0x03, 0x18]);
    let sut = sut_encode(
        "lsl",
        vec![
            Operand::Memory(mem_base("eax", Displacement::None, None)),
            Operand::Register(Register::new("bx")),
        ],
    );
    match sut {
        Ok(bytes) => assert_eq!(
            bytes, mc,
            "SUT must emit 0x66 for 16-bit dest memory form"
        ),
        Err(e) => panic!("SUT erred on valid mem16: {e}"),
    }
}

#[test]
fn encode_lsl_kat_llvm_mc_segment_es() {
    let mc = llvm_mc_bytes("lsl %es:(%eax), %ebx").expect("llvm-mc KAT es");
    assert_eq!(mc, vec![0x26, 0x0f, 0x03, 0x18]);
    let sut = sut_encode(
        "lsl",
        vec![
            Operand::Memory(mem_base("eax", Displacement::None, Some("es"))),
            Operand::Register(Register::new("ebx")),
        ],
    );
    match sut {
        Ok(bytes) => assert_eq!(
            bytes, mc,
            "SUT must emit ES override 0x26 before 0F 03"
        ),
        Err(e) => panic!("SUT erred on valid ES mem: {e}"),
    }
}

#[test]
fn encode_lsl_kat_mixed_ax_ebx() {
    // osize follows DEST (32-bit) → no 0x66; SUT wrongly keys off SRC
    let mc = llvm_mc_bytes("lsl %ax, %ebx").expect("llvm-mc");
    assert_eq!(mc, vec![0x0f, 0x03, 0xd8]);
    let sut = sut_encode(
        "lsl",
        vec![
            Operand::Register(Register::new("ax")),
            Operand::Register(Register::new("ebx")),
        ],
    )
    .expect("SUT");
    assert_eq!(
        sut, mc,
        "osize must follow dest (ebx=32): no 0x66; SUT keyed off src?"
    );
}

#[test]
fn encode_lsl_kat_mixed_eax_bx() {
    // osize follows DEST (16-bit) → 0x66; SUT wrongly keys off SRC
    let mc = llvm_mc_bytes("lsl %eax, %bx").expect("llvm-mc");
    assert_eq!(mc, vec![0x66, 0x0f, 0x03, 0xd8]);
    let sut = sut_encode(
        "lsl",
        vec![
            Operand::Register(Register::new("eax")),
            Operand::Register(Register::new("bx")),
        ],
    )
    .expect("SUT");
    assert_eq!(
        sut, mc,
        "osize must follow dest (bx=16): need 0x66; SUT keyed off src?"
    );
}

proptest! {
    #![proptest_config(cfg())]

    // Oracle: differential — 32-bit register form vs llvm-mc
    #[test]
    fn encode_lsl_diff_reg32_llvm_mc(
        src in prop::sample::select(GP_REGS),
        dst in prop::sample::select(GP_REGS),
    ) {
        let asm = format!("lsl %{src}, %{dst}");
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let sut = sut_encode(
            "lsl",
            vec![
                Operand::Register(Register::new(src)),
                Operand::Register(Register::new(dst)),
            ],
        )
        .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(sut, mc, "reg32 diff vs llvm-mc for `{}`", asm);
    }

    // Oracle: differential — 16-bit register form vs llvm-mc
    #[test]
    fn encode_lsl_diff_reg16_llvm_mc(
        src in prop::sample::select(R16_REGS),
        dst in prop::sample::select(R16_REGS),
    ) {
        let asm = format!("lsl %{src}, %{dst}");
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let sut = sut_encode(
            "lsl",
            vec![
                Operand::Register(Register::new(src)),
                Operand::Register(Register::new(dst)),
            ],
        )
        .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(sut, mc, "reg16 diff vs llvm-mc for `{}`", asm);
    }

    // Oracle: differential — mixed-width; osize follows destination
    #[test]
    fn encode_lsl_diff_mixed_width_dest_drives_osize(
        src in prop::sample::select(MIXED_REGS),
        dst in prop::sample::select(MIXED_REGS),
    ) {
        let asm = format!("lsl %{src}, %{dst}");
        let mc = match llvm_mc_bytes(&asm) {
            Ok(b) => b,
            Err(_) => return Ok(()),
        };
        let sut = sut_encode(
            "lsl",
            vec![
                Operand::Register(Register::new(src)),
                Operand::Register(Register::new(dst)),
            ],
        )
        .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(
            sut.clone(), mc.clone(),
            "mixed-width osize (dest-driven) for `{}`: SUT={:02x?} llvm-mc={:02x?}",
            asm, sut, mc
        );
    }

    // Oracle: differential — base+disp memory, 32- and 16-bit dest
    #[test]
    fn encode_lsl_diff_mem_base_disp(
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
        dst in prop::sample::select(MIXED_REGS),
    ) {
        let mem = mem_base(base, Displacement::Integer(disp), None);
        let asm = format!("lsl {}, %{}", att_mem(&mem), dst);
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let sut = sut_encode(
            "lsl",
            vec![
                Operand::Memory(mem),
                Operand::Register(Register::new(dst)),
            ],
        )
        .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(
            sut.clone(), mc.clone(),
            "mem base+disp diff for `{}`: SUT={:02x?} llvm-mc={:02x?}",
            asm, sut, mc
        );
    }

    // Oracle: differential — segment override prefixes
    #[test]
    fn encode_lsl_diff_segment_prefix(
        seg in prop::sample::select(SEG_REGS),
        base in prop::sample::select(GP_REGS),
        disp in prop_oneof![Just(0i64), Just(8i64), Just(-4i64), Just(127i64), Just(-128i64)],
        dst in prop::sample::select(GP_REGS),
    ) {
        let mem = mem_base(base, Displacement::Integer(disp), Some(seg));
        let asm = format!("lsl {}, %{}", att_mem(&mem), dst);
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let sut = sut_encode(
            "lsl",
            vec![
                Operand::Memory(mem),
                Operand::Register(Register::new(dst)),
            ],
        )
        .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(
            sut.clone(), mc.clone(),
            "segment prefix diff for `{}`: SUT={:02x?} llvm-mc={:02x?}",
            asm, sut, mc
        );
    }

    // Oracle: differential — SIB (index ≠ esp)
    #[test]
    fn encode_lsl_diff_sib(
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
        dst in prop::sample::select(MIXED_REGS),
    ) {
        let mem = mem_base_index(
            base,
            index,
            scale,
            Displacement::Integer(disp),
            None,
        );
        let asm = format!("lsl {}, %{}", att_mem(&mem), dst);
        let mc = match llvm_mc_bytes(&asm) {
            Ok(b) => b,
            Err(_) => return Ok(()),
        };
        let sut = sut_encode(
            "lsl",
            vec![
                Operand::Memory(mem),
                Operand::Register(Register::new(dst)),
            ],
        )
        .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(
            sut.clone(), mc.clone(),
            "SIB diff for `{}`: SUT={:02x?} llvm-mc={:02x?}",
            asm, sut, mc
        );
    }

    // Oracle: differential — abs disp32 + ESP/EBP edges
    #[test]
    fn encode_lsl_diff_edges_esp_ebp_abs(
        edge in 0u8..12,
        dst in prop::sample::select(MIXED_REGS),
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
        let asm = format!("lsl {}, %{}", att_mem(&mem), dst);
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected edge `{asm}`: {e}"));
        let sut = sut_encode(
            "lsl",
            vec![
                Operand::Memory(mem),
                Operand::Register(Register::new(dst)),
            ],
        )
        .unwrap_or_else(|e| panic!("SUT rejected edge `{asm}`: {e}"));
        prop_assert_eq!(
            sut.clone(), mc.clone(),
            "edge diff for `{}`: SUT={:02x?} llvm-mc={:02x?}",
            asm, sut, mc
        );
    }

    // Oracle: differential — segment + 16-bit dest (prefix order: seg, 66, 0F 03)
    #[test]
    fn encode_lsl_diff_segment_mem16(
        seg in prop::sample::select(SEG_REGS),
        base in prop::sample::select(GP_REGS),
        dst in prop::sample::select(R16_REGS),
    ) {
        let mem = mem_base(base, Displacement::None, Some(seg));
        let asm = format!("lsl {}, %{}", att_mem(&mem), dst);
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let sut = sut_encode(
            "lsl",
            vec![
                Operand::Memory(mem),
                Operand::Register(Register::new(dst)),
            ],
        )
        .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(
            sut.clone(), mc.clone(),
            "seg+mem16 diff for `{}`: SUT={:02x?} llvm-mc={:02x?}",
            asm, sut, mc
        );
    }

    // Oracle: algebraic.invariant — 0F 03 /r with ModRM.reg = dst
    #[test]
    fn encode_lsl_invariant_opcode_0f03(
        kind in 0u8..3,
        src in prop::sample::select(GP_REGS),
        dst in prop::sample::select(GP_REGS),
        base in prop::sample::select(GP_REGS),
        disp in prop_oneof![Just(0i64), Just(7i64), Just(-3i64), (-200i64..=300i64)],
        index in prop::sample::select(
            GP_REGS.iter().copied().filter(|r| *r != "esp").collect::<Vec<_>>()
        ),
        scale in prop::sample::select(SCALES),
    ) {
        let (ops, dst_name) = match kind {
            0 => (
                vec![
                    Operand::Register(Register::new(src)),
                    Operand::Register(Register::new(dst)),
                ],
                dst,
            ),
            1 => (
                vec![
                    Operand::Memory(mem_base(base, Displacement::Integer(disp), None)),
                    Operand::Register(Register::new(dst)),
                ],
                dst,
            ),
            _ => (
                vec![
                    Operand::Memory(mem_base_index(
                        Some(base),
                        index,
                        scale,
                        Displacement::Integer(disp),
                        None,
                    )),
                    Operand::Register(Register::new(dst)),
                ],
                dst,
            ),
        };
        let bytes = sut_encode("lsl", ops).expect("valid op must encode");
        let body = strip_prefixes(&bytes);
        prop_assert!(body.len() >= 3, "need opcode+modrm, got {bytes:?}");
        prop_assert_eq!(body[0], 0x0F, "LSL opcode high");
        prop_assert_eq!(body[1], 0x03, "LSL opcode low");
        let reg = (body[2] >> 3) & 7;
        let expect = match dst_name {
            "eax" | "ax" => 0u8,
            "ecx" | "cx" => 1,
            "edx" | "dx" => 2,
            "ebx" | "bx" => 3,
            "esp" | "sp" => 4,
            "ebp" | "bp" => 5,
            "esi" | "si" => 6,
            "edi" | "di" => 7,
            _ => panic!("bad dst"),
        };
        prop_assert_eq!(reg, expect, "ModRM.reg must be dst for LSL");
    }

    // Oracle: negative_error — wrong arity
    #[test]
    fn encode_lsl_neg_arity(
        n in 0usize..5,
    ) {
        prop_assume!(n != 2);
        let mut ops = Vec::new();
        for i in 0..n {
            if i % 2 == 0 {
                ops.push(Operand::Register(Register::new("eax")));
            } else {
                ops.push(Operand::Register(Register::new("ebx")));
            }
        }
        let err = sut_encode("lsl", ops).expect_err("arity ≠ 2 must Err");
        prop_assert!(
            err.contains("lsl requires 2 operands") || err.contains("requires 2 operand"),
            "unexpected err: {err}"
        );
    }

    // Oracle: negative_error — imm/label/reg-mem reversed/bad shapes
    #[test]
    fn encode_lsl_neg_bad_operand(
        kind in 0u8..5,
        imm in any::<i32>(),
    ) {
        let ops = match kind {
            0 => vec![
                Operand::Immediate(ImmediateValue::Integer(imm as i64)),
                Operand::Register(Register::new("eax")),
            ],
            1 => vec![
                Operand::Label("target".into()),
                Operand::Register(Register::new("eax")),
            ],
            2 => vec![
                Operand::Register(Register::new("eax")),
                Operand::Memory(mem_base("ebx", Displacement::None, None)),
            ],
            3 => vec![
                Operand::Register(Register::new("eax")),
                Operand::Immediate(ImmediateValue::Integer(0)),
            ],
            _ => vec![
                Operand::Memory(mem_base("eax", Displacement::None, None)),
                Operand::Memory(mem_base("ebx", Displacement::None, None)),
            ],
        };
        let err = sut_encode("lsl", ops).expect_err("bad shape must Err");
        prop_assert!(
            err.contains("unsupported lsl")
                || err.contains("lsl requires")
                || err.contains("bad register"),
            "unexpected err: {err}"
        );
    }
}

/// Deterministic regression: missing ES segment prefix on lsl mem form.
#[test]
fn test_encode_lsl_regression_missing_es_prefix() {
    let mem = mem_base("eax", Displacement::None, Some("es"));
    let sut = sut_encode(
        "lsl",
        vec![
            Operand::Memory(mem),
            Operand::Register(Register::new("ebx")),
        ],
    )
    .expect("encode");
    let mc = vec![0x26u8, 0x0f, 0x03, 0x18];
    assert_eq!(
        sut, mc,
        "lsl %es:(%eax), %ebx must be [26, 0f, 03, 18], got {sut:02x?}"
    );
}

/// Deterministic regression: 16-bit dest memory form missing 0x66.
#[test]
fn test_encode_lsl_regression_mem16_missing_66() {
    let mem = mem_base("eax", Displacement::None, None);
    let sut = sut_encode(
        "lsl",
        vec![
            Operand::Memory(mem),
            Operand::Register(Register::new("bx")),
        ],
    )
    .expect("encode");
    let mc = vec![0x66u8, 0x0f, 0x03, 0x18];
    assert_eq!(
        sut, mc,
        "lsl (%eax), %bx must be [66, 0f, 03, 18], got {sut:02x?}"
    );
}

/// Deterministic regression: osize keyed off src instead of dest (ax→ebx).
#[test]
fn test_encode_lsl_regression_osize_from_src_ax_ebx() {
    let sut = sut_encode(
        "lsl",
        vec![
            Operand::Register(Register::new("ax")),
            Operand::Register(Register::new("ebx")),
        ],
    )
    .expect("encode");
    // dest=ebx → 32-bit → NO 0x66
    let mc = vec![0x0fu8, 0x03, 0xd8];
    assert_eq!(
        sut, mc,
        "lsl %ax, %ebx must be [0f, 03, d8] (dest-driven), got {sut:02x?}"
    );
}

/// Deterministic regression: osize keyed off src instead of dest (eax→bx).
#[test]
fn test_encode_lsl_regression_osize_from_src_eax_bx() {
    let sut = sut_encode(
        "lsl",
        vec![
            Operand::Register(Register::new("eax")),
            Operand::Register(Register::new("bx")),
        ],
    )
    .expect("encode");
    // dest=bx → 16-bit → NEED 0x66
    let mc = vec![0x66u8, 0x0f, 0x03, 0xd8];
    assert_eq!(
        sut, mc,
        "lsl %eax, %bx must be [66, 0f, 03, d8] (dest-driven), got {sut:02x?}"
    );
}

#[allow(dead_code)]
fn _keep_is_r16() {
    let _ = is_r16("ax");
}
