// Oracle: differential — llvm-mc i686 assembler (LEA mem→reg: 8D /r + optional seg/0x66)
// Evidence: gp_integer.rs:332-344 encode_lea;
//   encoder/mod.rs:186 leal|lea → encode_lea(ops, 4);
//   core.rs:31-42 emit_segment_prefix (es/cs/ss/ds/fs/gs);
//   Intel SDM Vol.2 LEA — r16,m / r32,m;
//   AT&T order: leal mem, %dst with ModRM.reg=dst under 8D.
// Stronger considered:
//   - State machine: rejected — pure encoding, no lifecycle
//   - Algebraic round-trip: rejected — no in-tree i686 LEA decoder
// Differential: candidate=encode_lea (via InstructionEncoder::encode leal/lea),
//   reference=llvm-mc -triple=i686 -show-encoding (LLVM 15),
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=AT&T operand bytes <-> llvm-mc encoding bytes.
// Also: algebraic.invariant (opcode/modrm), algebraic.metamorphic (lea vs movl),
//   negative_error (arity, shapes, non-GP / r8 dest).

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

/// Strip leading segment-override prefixes for opcode checks.
fn strip_seg_prefixes(bytes: &[u8]) -> &[u8] {
    let mut i = 0;
    while i < bytes.len() && matches!(bytes[i], 0x26 | 0x2E | 0x36 | 0x3E | 0x64 | 0x65 | 0x66) {
        i += 1;
    }
    &bytes[i..]
}

// --- KAT gate (reference oracle prerequisite) ---

#[test]
fn encode_lea_kat_llvm_mc_base32() {
    let mc = llvm_mc_bytes("leal (%eax), %ebx").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x8d, 0x18], "llvm-mc KAT mapping broken");
    let sut = sut_encode(
        "leal",
        vec![
            Operand::Memory(mem_base("eax", Displacement::None, None)),
            Operand::Register(Register::new("ebx")),
        ],
    )
    .expect("SUT KAT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_lea_kat_llvm_mc_sib() {
    let mc = llvm_mc_bytes("leal 4(%eax,%ecx,2), %edx").expect("llvm-mc KAT sib");
    assert_eq!(mc, vec![0x8d, 0x54, 0x48, 0x04]);
    let sut = sut_encode(
        "leal",
        vec![
            Operand::Memory(mem_base_index(
                Some("eax"),
                "ecx",
                2,
                Displacement::Integer(4),
                None,
            )),
            Operand::Register(Register::new("edx")),
        ],
    )
    .expect("SUT KAT sib");
    assert_eq!(sut, mc);
}

#[test]
fn encode_lea_kat_llvm_mc_esp() {
    let mc = llvm_mc_bytes("leal (%esp), %eax").expect("llvm-mc KAT esp");
    assert_eq!(mc, vec![0x8d, 0x04, 0x24]);
    let sut = sut_encode(
        "leal",
        vec![
            Operand::Memory(mem_base("esp", Displacement::None, None)),
            Operand::Register(Register::new("eax")),
        ],
    )
    .expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_lea_kat_llvm_mc_es_segment() {
    // Contract: all six segment overrides are valid on i686 (core.rs emit_segment_prefix).
    let mc = llvm_mc_bytes("leal %es:(%eax), %ebx").expect("llvm-mc KAT es");
    assert_eq!(mc, vec![0x26, 0x8d, 0x18]);
    let sut = sut_encode(
        "leal",
        vec![
            Operand::Memory(mem_base("eax", Displacement::None, Some("es"))),
            Operand::Register(Register::new("ebx")),
        ],
    );
    match sut {
        Ok(bytes) => assert_eq!(bytes, mc, "SUT must emit ES override 0x26 before 8D"),
        Err(e) => panic!("SUT erred on valid ES lea: {e}"),
    }
}

#[test]
fn encode_lea_kat_llvm_mc_fs_segment() {
    let mc = llvm_mc_bytes("leal %fs:(%eax), %ecx").expect("llvm-mc KAT fs");
    assert_eq!(mc, vec![0x64, 0x8d, 0x08]);
    let sut = sut_encode(
        "leal",
        vec![
            Operand::Memory(mem_base("eax", Displacement::None, Some("fs"))),
            Operand::Register(Register::new("ecx")),
        ],
    );
    match sut {
        Ok(bytes) => assert_eq!(bytes, mc, "SUT must emit FS override 0x64 before 8D"),
        Err(e) => panic!("SUT erred on valid FS lea: {e}"),
    }
}

/// Deterministic regression: missing segment override (B1).
/// Witness: leal %es:(%eax), %eax → SUT omits 0x26.
#[test]
fn encode_lea_regression_missing_es_prefix() {
    let mc = llvm_mc_bytes("leal %es:(%eax), %eax").expect("llvm-mc");
    assert_eq!(mc, vec![0x26, 0x8d, 0x00]);
    let sut = sut_encode(
        "leal",
        vec![
            Operand::Memory(mem_base("eax", Displacement::None, Some("es"))),
            Operand::Register(Register::new("eax")),
        ],
    )
    .expect("valid ES lea must encode");
    assert_eq!(
        sut, mc,
        "regression: encode_lea must emit segment override before 0x8D"
    );
}

/// Deterministic regression: non-GP dest accepted via reg_num alias (B2).
/// Witness: leal (%eax), %xmm0 → SUT emits [0x8d, 0x00].
#[test]
fn encode_lea_regression_non_gp_xmm0_dest() {
    let asm = "leal (%eax), %xmm0";
    assert!(
        llvm_mc_bytes(asm).is_err(),
        "llvm-mc must reject non-GP LEA dest"
    );
    let sut = sut_encode(
        "leal",
        vec![
            Operand::Memory(mem_base("eax", Displacement::None, None)),
            Operand::Register(Register::new("xmm0")),
        ],
    );
    assert!(
        sut.is_err(),
        "regression: encode_lea must reject non-GP dest xmm0, got {sut:?}"
    );
}

proptest! {
    #![proptest_config(cfg())]

    // P1: differential — base+disp, no segment, 32-bit dest
    #[test]
    fn encode_lea_diff_llvm_mc_base_disp(
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
        let dst = GP32[di % 8];
        let mem = mem_base(base, Displacement::Integer(disp), None);
        let asm = format!("leal {}, %{}", att_mem(&mem), dst);
        let mc = llvm_mc_bytes(&asm)
            .map_err(|e| TestCaseError::fail(format!("llvm-mc rejected `{asm}`: {e}")))?;
        let sut = sut_encode(
            "leal",
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
    fn encode_lea_diff_llvm_mc_sib(
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
        let dst = GP32[di % 8];
        let mem = mem_base_index(base, index, scale, Displacement::Integer(disp), None);
        let asm = format!("leal {}, %{}", att_mem(&mem), dst);
        let mc = match llvm_mc_bytes(&asm) {
            Ok(b) => b,
            Err(_) => return Ok(()),
        };
        let sut = sut_encode(
            "leal",
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
    fn encode_lea_diff_llvm_mc_segment(
        seg in prop::sample::select(SEG_REGS),
        base in prop::sample::select(GP32),
        disp in prop_oneof![Just(0i64), Just(8i64), Just(-4i64), Just(127i64), Just(-128i64)],
        di in 0usize..8,
    ) {
        let dst = GP32[di % 8];
        let mem = mem_base(base, Displacement::Integer(disp), Some(seg));
        let asm = format!("leal {}, %{}", att_mem(&mem), dst);
        let mc = llvm_mc_bytes(&asm)
            .map_err(|e| TestCaseError::fail(format!("llvm-mc rejected `{asm}`: {e}")))?;
        let sut = sut_encode(
            "leal",
            vec![
                Operand::Memory(mem),
                Operand::Register(Register::new(dst)),
            ],
        )
        .map_err(|e| TestCaseError::fail(format!(
            "SUT rejected valid segment form `{asm}`: {e}; \
             i686 LEA must emit segment override (core.rs emit_segment_prefix; \
             Intel SDM 2.1.1)."
        )))?;
        prop_assert_eq!(
            &sut, &mc,
            "segment diff `{}`: sut={:02x?} mc={:02x?}",
            asm, &sut, &mc
        );
    }

    // P4: differential — ESP/EBP/SIB/abs edges
    #[test]
    fn encode_lea_diff_edges_esp_ebp_abs(
        edge in 0u8..14,
        di in 0usize..8,
    ) {
        let dst = GP32[di % 8];
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
        let asm = format!("leal {}, %{}", att_mem(&mem), dst);
        let mc = match llvm_mc_bytes(&asm) {
            Ok(b) => b,
            Err(e) => return Err(TestCaseError::fail(format!("llvm-mc edge `{asm}`: {e}"))),
        };
        let sut = sut_encode(
            "leal",
            vec![
                Operand::Memory(mem),
                Operand::Register(Register::new(dst)),
            ],
        )
        .map_err(|e| TestCaseError::fail(format!("SUT edge `{asm}`: {e}")))?;
        prop_assert_eq!(&sut, &mc, "edge diff `{}`", asm);
    }

    // P5: algebraic.invariant — opcode 0x8D, ModRM.reg = dst
    #[test]
    fn encode_lea_invariant_opcode_modrm(
        base in prop::sample::select(GP32),
        disp in prop_oneof![Just(0i64), Just(7i64), Just(-3i64), (-200i64..=300i64)],
        use_index in any::<bool>(),
        index in prop::sample::select(
            GP32.iter().copied().filter(|r| *r != "esp").collect::<Vec<_>>()
        ),
        scale in prop::sample::select(SCALES),
        di in 0usize..8,
    ) {
        let dst = GP32[di % 8];
        let mem = if use_index {
            mem_base_index(Some(base), index, scale, Displacement::Integer(disp), None)
        } else {
            mem_base(base, Displacement::Integer(disp), None)
        };
        let bytes = sut_encode(
            "leal",
            vec![
                Operand::Memory(mem),
                Operand::Register(Register::new(dst)),
            ],
        )
        .map_err(|e| TestCaseError::fail(e))?;
        let body = strip_seg_prefixes(&bytes);
        prop_assert!(!body.is_empty(), "empty encoding");
        prop_assert_eq!(body[0], 0x8D, "LEA opcode must be 0x8D");
        prop_assert!(body.len() > 1, "need ModRM");
        let modrm = body[1];
        prop_assert_eq!((modrm >> 3) & 7, gp_num(dst), "ModRM.reg = dst");
    }

    // P6: algebraic.metamorphic — lea vs movl share Mod+RM/SIB/disp (no segment)
    #[test]
    fn encode_lea_meta_vs_movl_modrm(
        base in prop::sample::select(GP32),
        disp in prop_oneof![Just(0i64), Just(40i64), Just(-5i64), (-1000i64..=1000i64)],
        index in prop::option::of(prop::sample::select(
            GP32.iter().copied().filter(|r| *r != "esp").collect::<Vec<_>>()
        )),
        scale in prop::sample::select(SCALES),
        di in 0usize..8,
    ) {
        let dst = GP32[di % 8];
        let mem = match index {
            Some(idx) => mem_base_index(Some(base), idx, scale, Displacement::Integer(disp), None),
            None => mem_base(base, Displacement::Integer(disp), None),
        };
        let lea = sut_encode(
            "leal",
            vec![
                Operand::Memory(mem.clone()),
                Operand::Register(Register::new(dst)),
            ],
        )
        .map_err(|e| TestCaseError::fail(format!("lea: {e}")))?;
        let mov = sut_encode(
            "movl",
            vec![
                Operand::Memory(mem),
                Operand::Register(Register::new(dst)),
            ],
        )
        .map_err(|e| TestCaseError::fail(format!("movl: {e}")))?;
        let lea_body = strip_seg_prefixes(&lea);
        let mov_body = strip_seg_prefixes(&mov);
        prop_assert_eq!(lea_body[0], 0x8D, "lea opcode");
        prop_assert_eq!(mov_body[0], 0x8B, "movl mem→reg opcode");
        prop_assert_eq!(
            &lea_body[1..],
            &mov_body[1..],
            "LEA and MOV must share ModRM/SIB/disp for same mem+dst"
        );
    }

    // P7: negative — wrong arity must Err
    #[test]
    fn encode_lea_neg_arity(
        n in 0usize..4,
        r0 in prop::sample::select(GP32),
        r1 in prop::sample::select(GP32),
        r2 in prop::sample::select(GP32),
    ) {
        prop_assume!(n != 2);
        let mut ops = Vec::new();
        if n >= 1 {
            ops.push(Operand::Memory(mem_base(r0, Displacement::None, None)));
        }
        if n >= 2 {
            ops.push(Operand::Register(Register::new(r1)));
        }
        if n >= 3 {
            ops.push(Operand::Register(Register::new(r2)));
        }
        match sut_encode("leal", ops) {
            Err(e) => {
                prop_assert!(
                    e.contains("requires 2") || e.contains("lea"),
                    "arity error text: {e}"
                );
            }
            Ok(bytes) => {
                return Err(TestCaseError::fail(format!(
                    "SUT accepted {n}-operand leal → {bytes:02x?}"
                )));
            }
        }
    }

    // P8: negative — unsupported shapes and non-GP / r8 dest must Err
    #[test]
    fn encode_lea_neg_shape_and_dest(
        mode in 0u8..8,
        ri in 0usize..8,
        ni in 0usize..10,
    ) {
        let gp = GP32[ri % 8];
        let r16 = R16[ri % 8];
        let r8 = R8[ri % 8];
        let non_gp = NON_GP[ni % NON_GP.len()];
        let mem = mem_base("eax", Displacement::None, None);
        let (ops, label) = match mode {
            // RR
            0 => (
                vec![
                    Operand::Register(Register::new(gp)),
                    Operand::Register(Register::new("ebx")),
                ],
                format!("leal %{gp}, %ebx"),
            ),
            // Imm, Reg
            1 => (
                vec![
                    Operand::Immediate(ImmediateValue::Integer(0)),
                    Operand::Register(Register::new("eax")),
                ],
                "leal $0, %eax".to_string(),
            ),
            // Reg, Mem (reversed)
            2 => (
                vec![
                    Operand::Register(Register::new("eax")),
                    Operand::Memory(mem.clone()),
                ],
                "leal %eax, (%eax)".to_string(),
            ),
            // Mem, Mem
            3 => (
                vec![Operand::Memory(mem.clone()), Operand::Memory(mem.clone())],
                "leal (%eax), (%eax)".to_string(),
            ),
            // non-GP dest
            4 => (
                vec![
                    Operand::Memory(mem.clone()),
                    Operand::Register(Register::new(non_gp)),
                ],
                format!("leal (%eax), %{non_gp}"),
            ),
            // r8 dest
            5 => (
                vec![
                    Operand::Memory(mem.clone()),
                    Operand::Register(Register::new(r8)),
                ],
                format!("leal (%eax), %{r8}"),
            ),
            // r16 dest under leal (needs leaw + 0x66; leal rejects)
            6 => (
                vec![
                    Operand::Memory(mem.clone()),
                    Operand::Register(Register::new(r16)),
                ],
                format!("leal (%eax), %{r16}"),
            ),
            // empty / single reg only already covered by arity; use two imm
            _ => (
                vec![
                    Operand::Immediate(ImmediateValue::Integer(1)),
                    Operand::Immediate(ImmediateValue::Integer(2)),
                ],
                "leal $1, $2".to_string(),
            ),
        };

        // Prefer llvm-mc as independent rejecter when the asm is parseable.
        let mc_rejects = llvm_mc_bytes(&label).is_err();
        match sut_encode("leal", ops) {
            Err(_) => {}
            Ok(bytes) => {
                if mc_rejects || mode >= 4 {
                    return Err(TestCaseError::fail(format!(
                        "SUT accepted invalid LEA `{label}` → {bytes:02x?}; \
                         LEA requires memory source and r16/r32 GP dest (Intel SDM). \
                         encode_lea uses reg_num which aliases r8/xmm/mm/st and ignores size."
                    )));
                }
                return Err(TestCaseError::fail(format!(
                    "SUT accepted invalid shape `{label}` → {bytes:02x?}"
                )));
            }
        }
    }
}
