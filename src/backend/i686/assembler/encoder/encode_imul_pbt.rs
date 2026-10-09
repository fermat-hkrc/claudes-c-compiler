// Oracle: differential — llvm-mc i686 assembler (AT&T)
// Evidence: gp_integer.rs:711 encode_imul; mod.rs:219/818 imull|imul|imulw → encode_imul;
//   Intel SDM Vol.2 IMUL (F6/F7 /5 r/m; 0F AF /r; 6B/69 r,r/m,imm);
//   core.rs:31-42 emit_segment_prefix for fs/gs/es/cs/ss/ds;
//   1-op path delegates to encode_unary_rm /5.
// Stronger considered:
//   - State machine: rejected — pure encoding, no lifecycle
//   - Algebraic round-trip: rejected — no in-tree i686 IMUL decoder
// Differential: candidate=encode_imul (via InstructionEncoder::encode imul*),
//   reference=llvm-mc -triple=i686 -show-encoding (LLVM 15),
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=AT&T operand bytes <-> llvm-mc encoding bytes.
// Also: algebraic.invariant (RR 0F AF + optional 0x66), algebraic.metamorphic (seg ‖ bare),
//   negative_error (arity / unsupported shapes).

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
const SREGS: &[&str] = &["es", "cs", "ss", "ds", "fs", "gs"];
const NON_GP: &[&str] = &[
    "xmm0", "xmm1", "xmm7", "mm0", "mm3", "mm7", "st", "st(0)", "st(1)", "ymm0",
];

fn gp_for_width(w: u8) -> &'static [&'static str] {
    match w {
        2 => R16,
        _ => GP32,
    }
}

fn mnemonic(w: u8) -> &'static str {
    match w {
        2 => "imulw",
        _ => "imull",
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
        "xmm7" | "mm7" | "st(7)" | "ymm7" => 7,
        _ => panic!("bad gp {name}"),
    }
}

fn seg_prefix_byte(seg: &str) -> u8 {
    match seg {
        "es" => 0x26,
        "cs" => 0x2E,
        "ss" => 0x36,
        "ds" => 0x3E,
        "fs" => 0x64,
        "gs" => 0x65,
        _ => panic!("bad seg {seg}"),
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

fn mem_sib(
    base: &str,
    index: &str,
    scale: u8,
    disp: Displacement,
    segment: Option<&str>,
) -> MemoryOperand {
    MemoryOperand {
        segment: segment.map(|s| s.to_string()),
        displacement: disp,
        base: Some(Register::new(base)),
        index: Some(Register::new(index)),
        scale: Some(scale),
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
    if let Some(b) = &mem.base {
        s.push('%');
        s.push_str(&b.name);
    }
    if let Some(ix) = &mem.index {
        s.push(',');
        s.push('%');
        s.push_str(&ix.name);
        if let Some(sc) = mem.scale {
            s.push(',');
            s.push_str(&sc.to_string());
        }
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

fn imm_edge(edge: u8, raw: i32, width: u8) -> i64 {
    let v = match edge {
        0 => -128i64,
        1 => -129,
        2 => 127,
        3 => 128,
        4 => 0,
        5 => -1,
        6 => {
            if width == 2 {
                300
            } else {
                0x1234_5678
            }
        }
        7 => {
            if width == 2 {
                -200
            } else {
                0x7fff_ffff
            }
        }
        8 => {
            if width == 2 {
                0x7fff
            } else {
                -0x8000_0000i64
            }
        }
        _ => raw as i64,
    };
    if width == 2 {
        (v as i16) as i64
    } else {
        (v as i32) as i64
    }
}

// --- KAT gate (reference oracle prerequisite) ---

#[test]
fn encode_imul_kat_llvm_mc_imull_rr() {
    let mc = llvm_mc_bytes("imull %eax, %ebx").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x0f, 0xaf, 0xd8], "llvm-mc KAT mapping broken");
    let sut = sut_encode(
        "imull",
        vec![
            Operand::Register(Register::new("eax")),
            Operand::Register(Register::new("ebx")),
        ],
    )
    .expect("SUT KAT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_imul_kat_llvm_mc_imulw_rr() {
    let mc = llvm_mc_bytes("imulw %ax, %bx").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x66, 0x0f, 0xaf, 0xd8]);
    // SUT may disagree — property/regression owns the fail; KAT pins llvm-mc path.
}

#[test]
fn encode_imul_kat_llvm_mc_imull_imm8() {
    let mc = llvm_mc_bytes("imull $5, %ebx").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x6b, 0xdb, 0x05]);
    let sut = sut_encode(
        "imull",
        vec![
            Operand::Immediate(ImmediateValue::Integer(5)),
            Operand::Register(Register::new("ebx")),
        ],
    )
    .expect("SUT KAT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_imul_kat_llvm_mc_imull_imm32() {
    let mc = llvm_mc_bytes("imull $128, %ebx").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x69, 0xdb, 0x80, 0x00, 0x00, 0x00]);
    let sut = sut_encode(
        "imull",
        vec![
            Operand::Immediate(ImmediateValue::Integer(128)),
            Operand::Register(Register::new("ebx")),
        ],
    )
    .expect("SUT KAT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_imul_kat_llvm_mc_imull_mem_reg() {
    let mc = llvm_mc_bytes("imull (%eax), %ebx").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x0f, 0xaf, 0x18]);
    let mem = mem_base("eax", Displacement::None, None);
    let sut = sut_encode(
        "imull",
        vec![
            Operand::Memory(mem),
            Operand::Register(Register::new("ebx")),
        ],
    )
    .expect("SUT KAT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_imul_kat_llvm_mc_unary() {
    let mc = llvm_mc_bytes("imull %eax").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0xf7, 0xe8]);
    let sut = sut_encode("imull", vec![Operand::Register(Register::new("eax"))]).expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_imul_kat_llvm_mc_es_segment_reference_only() {
    let mc = llvm_mc_bytes("imull %es:(%eax), %ebx").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x26, 0x0f, 0xaf, 0x18]);
}

#[test]
fn encode_imul_kat_llvm_mc_imulw_imm16_reference_only() {
    let mc = llvm_mc_bytes("imulw $300, %ax").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x66, 0x69, 0xc0, 0x2c, 0x01]);
}

proptest! {
    #![proptest_config(cfg())]

    // P1: differential — same-width GP RR (0F AF [/ + 0x66 for 16-bit])
    #[test]
    fn encode_imul_diff_rr_same_width(
        width in prop::sample::select(vec![2u8, 4]),
        si in 0usize..8,
        di in 0usize..8,
    ) {
        let regs = gp_for_width(width);
        let src = regs[si % regs.len()];
        let dst = regs[di % regs.len()];
        let mnem = mnemonic(width);
        let asm = format!("{mnem} %{src}, %{dst}");
        let mc = llvm_mc_bytes(&asm)
            .map_err(|e| TestCaseError::fail(format!("llvm-mc failed on valid `{asm}`: {e}")))?;
        let sut = sut_encode(
            mnem,
            vec![
                Operand::Register(Register::new(src)),
                Operand::Register(Register::new(dst)),
            ],
        )
        .map_err(|e| TestCaseError::fail(format!("SUT Err on valid `{asm}`: {e}")))?;
        prop_assert_eq!(&sut, &mc, "diff RR `{}`: sut={:02x?} mc={:02x?}", asm, &sut, &mc);
    }

    // P2: differential — Mem→Reg bare + SIB + abs, no segment
    #[test]
    fn encode_imul_diff_mem_reg(
        width in prop::sample::select(vec![2u8, 4]),
        form in 0u8..4,
        bi in 0usize..8,
        ii in 0usize..8,
        di in 0usize..8,
        scale in prop::sample::select(vec![1u8, 2, 4, 8]),
        disp in prop::sample::select(vec![0i64, 1, -1, 4, 127, 128, -128, 0x1000]),
    ) {
        // Avoid ESP as SIB index (illegal)
        let base = GP32[bi % GP32.len()];
        let index = GP32[ii % GP32.len()];
        let regs = gp_for_width(width);
        let dst = regs[di % regs.len()];
        let mnem = mnemonic(width);

        let mem = match form {
            0 => mem_base(base, Displacement::Integer(disp), None),
            1 if index != "esp" => mem_sib(base, index, scale, Displacement::Integer(disp), None),
            2 => MemoryOperand {
                segment: None,
                displacement: Displacement::Integer(disp.max(1)), // abs needs non-empty
                base: None,
                index: None,
                scale: None,
            },
            _ => mem_base(base, Displacement::None, None),
        };
        // Skip illegal SIB with esp index by falling back
        let mem = if form == 1 && index == "esp" {
            mem_base(base, Displacement::Integer(disp), None)
        } else {
            mem
        };

        let asm = format!("{mnem} {}, %{dst}", att_mem(&mem));
        let ops = vec![
            Operand::Memory(mem),
            Operand::Register(Register::new(dst)),
        ];
        let mc = llvm_mc_bytes(&asm)
            .map_err(|e| TestCaseError::fail(format!("llvm-mc failed on valid `{asm}`: {e}")))?;
        let sut = sut_encode(mnem, ops)
            .map_err(|e| TestCaseError::fail(format!("SUT Err on valid `{asm}`: {e}")))?;
        prop_assert_eq!(&sut, &mc, "diff mem→reg `{}`: sut={:02x?} mc={:02x?}", asm, &sut, &mc);
    }

    // P3: differential — Imm→Reg (2-op shorthand 6B/69), edges on ±128 and 16-bit imm width
    #[test]
    fn encode_imul_diff_imm_reg(
        width in prop::sample::select(vec![2u8, 4]),
        di in 0usize..8,
        raw in any::<i32>(),
        edge in 0u8..12,
    ) {
        let regs = gp_for_width(width);
        let dst = regs[di % regs.len()];
        let imm = imm_edge(edge, raw, width);
        let mnem = mnemonic(width);
        let asm = format!("{mnem} ${imm}, %{dst}");
        let mc = llvm_mc_bytes(&asm)
            .map_err(|e| TestCaseError::fail(format!("llvm-mc failed on valid `{asm}`: {e}")))?;
        let sut = sut_encode(
            mnem,
            vec![
                Operand::Immediate(ImmediateValue::Integer(imm)),
                Operand::Register(Register::new(dst)),
            ],
        )
        .map_err(|e| TestCaseError::fail(format!("SUT Err on valid `{asm}`: {e}")))?;
        prop_assert_eq!(&sut, &mc, "diff imm→reg `{}`: sut={:02x?} mc={:02x?}", asm, &sut, &mc);
    }

    // P4: differential — Imm, Reg, Reg three-operand
    #[test]
    fn encode_imul_diff_imm_reg_reg(
        width in prop::sample::select(vec![2u8, 4]),
        si in 0usize..8,
        di in 0usize..8,
        raw in any::<i32>(),
        edge in 0u8..12,
    ) {
        let regs = gp_for_width(width);
        let src = regs[si % regs.len()];
        let dst = regs[di % regs.len()];
        let imm = imm_edge(edge, raw, width);
        let mnem = mnemonic(width);
        let asm = format!("{mnem} ${imm}, %{src}, %{dst}");
        let mc = llvm_mc_bytes(&asm)
            .map_err(|e| TestCaseError::fail(format!("llvm-mc failed on valid `{asm}`: {e}")))?;
        let sut = sut_encode(
            mnem,
            vec![
                Operand::Immediate(ImmediateValue::Integer(imm)),
                Operand::Register(Register::new(src)),
                Operand::Register(Register::new(dst)),
            ],
        )
        .map_err(|e| TestCaseError::fail(format!("SUT Err on valid `{asm}`: {e}")))?;
        prop_assert_eq!(&sut, &mc, "diff imm,reg,reg `{}`: sut={:02x?} mc={:02x?}", asm, &sut, &mc);
    }

    // P5: differential — Imm, Mem, Reg three-operand (bare + SIB + segment)
    #[test]
    fn encode_imul_diff_imm_mem_reg(
        width in prop::sample::select(vec![2u8, 4]),
        form in 0u8..3,
        bi in 0usize..8,
        ii in 0usize..8,
        di in 0usize..8,
        si in 0usize..7, // 0..6 = sreg, 6 = no seg
        scale in prop::sample::select(vec![1u8, 2, 4, 8]),
        disp in prop::sample::select(vec![0i64, 4, -8, 128]),
        raw in any::<i16>(),
        edge in 0u8..10,
    ) {
        let base = GP32[bi % GP32.len()];
        let index = GP32[ii % GP32.len()];
        let regs = gp_for_width(width);
        let dst = regs[di % regs.len()];
        let seg = if si < 6 { Some(SREGS[si]) } else { None };
        let mnem = mnemonic(width);
        let imm = imm_edge(edge, raw as i32, width);

        let mem = match form {
            0 => mem_base(base, Displacement::Integer(disp), seg),
            1 if index != "esp" => mem_sib(base, index, scale, Displacement::Integer(disp), seg),
            _ => mem_base(base, Displacement::None, seg),
        };
        let mem = if form == 1 && index == "esp" {
            mem_base(base, Displacement::Integer(disp), seg)
        } else {
            mem
        };

        let asm = format!("{mnem} ${imm}, {}, %{dst}", att_mem(&mem));
        let ops = vec![
            Operand::Immediate(ImmediateValue::Integer(imm)),
            Operand::Memory(mem),
            Operand::Register(Register::new(dst)),
        ];
        let mc = llvm_mc_bytes(&asm)
            .map_err(|e| TestCaseError::fail(format!("llvm-mc failed on valid `{asm}`: {e}")))?;
        let sut = sut_encode(mnem, ops)
            .map_err(|e| TestCaseError::fail(format!("SUT Err on valid `{asm}`: {e}")))?;
        prop_assert_eq!(&sut, &mc, "diff imm,mem,reg `{}`: sut={:02x?} mc={:02x?}", asm, &sut, &mc);
    }

    // P6: differential — 1-op unary (reg / mem / segment)
    #[test]
    fn encode_imul_diff_unary(
        width in prop::sample::select(vec![2u8, 4]),
        form in 0u8..3,
        ri in 0usize..8,
        bi in 0usize..8,
        si in 0usize..7,
        disp in prop::sample::select(vec![0i64, 4, -1, 128]),
    ) {
        let regs = gp_for_width(width);
        let mnem = mnemonic(width);
        let seg = if si < 6 { Some(SREGS[si]) } else { None };
        let (asm, ops) = match form {
            0 => {
                let r = regs[ri % regs.len()];
                (format!("{mnem} %{r}"), vec![Operand::Register(Register::new(r))])
            }
            1 => {
                let base = GP32[bi % GP32.len()];
                let mem = mem_base(base, Displacement::Integer(disp), None);
                (format!("{mnem} {}", att_mem(&mem)), vec![Operand::Memory(mem)])
            }
            _ => {
                let base = GP32[bi % GP32.len()];
                let mem = mem_base(base, Displacement::Integer(disp), seg);
                (format!("{mnem} {}", att_mem(&mem)), vec![Operand::Memory(mem)])
            }
        };
        let mc = llvm_mc_bytes(&asm)
            .map_err(|e| TestCaseError::fail(format!("llvm-mc failed on valid `{asm}`: {e}")))?;
        let sut = sut_encode(mnem, ops)
            .map_err(|e| TestCaseError::fail(format!("SUT Err on valid `{asm}`: {e}")))?;
        prop_assert_eq!(&sut, &mc, "diff unary `{}`: sut={:02x?} mc={:02x?}", asm, &sut, &mc);
    }

    // P7: algebraic invariant — RR is [0x66?] 0F AF modrm(3, dst, src)
    #[test]
    fn encode_imul_invariant_rr_opcode(
        width in prop::sample::select(vec![2u8, 4]),
        si in 0usize..8,
        di in 0usize..8,
    ) {
        let regs = gp_for_width(width);
        let src = regs[si % regs.len()];
        let dst = regs[di % regs.len()];
        let mnem = mnemonic(width);
        let bytes = sut_encode(
            mnem,
            vec![
                Operand::Register(Register::new(src)),
                Operand::Register(Register::new(dst)),
            ],
        )
        .map_err(|e| TestCaseError::fail(e))?;
        let mut i = 0usize;
        if width == 2 {
            prop_assert_eq!(bytes.get(i).copied(), Some(0x66), "imulw must emit 0x66 operand-size prefix");
            i += 1;
        } else {
            prop_assert_ne!(bytes.get(0).copied().unwrap_or(0), 0x66, "spurious 0x66 on imull");
        }
        prop_assert_eq!(bytes.get(i).copied(), Some(0x0F), "opcode 0F");
        prop_assert_eq!(bytes.get(i + 1).copied(), Some(0xAF), "opcode AF");
        let modrm = *bytes.get(i + 2).ok_or_else(|| TestCaseError::fail("missing modrm"))?;
        prop_assert_eq!(modrm >> 6, 0b11, "mod=11");
        prop_assert_eq!((modrm >> 3) & 7, gp_num(dst), "ModRM.reg = dst");
        prop_assert_eq!(modrm & 7, gp_num(src), "ModRM.rm = src");
        prop_assert_eq!(bytes.len(), i + 3, "exact length");
    }

    // P8: metamorphic — segment Mem→Reg = seg_byte ‖ bare Mem→Reg (imull 32-bit)
    #[test]
    fn encode_imul_metamorphic_seg_prefix(
        si in 0usize..6,
        bi in 0usize..8,
        di in 0usize..8,
        disp in prop::sample::select(vec![0i64, 4, -8]),
        form in 0u8..2, // 0=2-op mem/reg, 1=3-op imm/mem/reg
    ) {
        let seg = SREGS[si % SREGS.len()];
        let base = GP32[bi % GP32.len()];
        let dst = GP32[di % GP32.len()];
        let bare = mem_base(base, Displacement::Integer(disp), None);
        let segd = mem_base(base, Displacement::Integer(disp), Some(seg));
        let pref = seg_prefix_byte(seg);

        let (bare_ops, seg_ops) = if form == 0 {
            (
                vec![
                    Operand::Memory(bare),
                    Operand::Register(Register::new(dst)),
                ],
                vec![
                    Operand::Memory(segd),
                    Operand::Register(Register::new(dst)),
                ],
            )
        } else {
            let imm = 5i64;
            (
                vec![
                    Operand::Immediate(ImmediateValue::Integer(imm)),
                    Operand::Memory(bare),
                    Operand::Register(Register::new(dst)),
                ],
                vec![
                    Operand::Immediate(ImmediateValue::Integer(imm)),
                    Operand::Memory(segd),
                    Operand::Register(Register::new(dst)),
                ],
            )
        };

        let bare_bytes = sut_encode("imull", bare_ops)
            .map_err(|e| TestCaseError::fail(format!("bare failed: {e}")))?;
        let seg_bytes = sut_encode("imull", seg_ops)
            .map_err(|e| TestCaseError::fail(format!("seg failed: {e}")))?;
        let mut expect = Vec::with_capacity(1 + bare_bytes.len());
        expect.push(pref);
        expect.extend_from_slice(&bare_bytes);
        prop_assert_eq!(
            &seg_bytes, &expect,
            "metamorphic seg: expected [{:02x}]||{:02x?} got {:02x?}",
            pref, bare_bytes, seg_bytes
        );
    }

    // P9: negative — wrong arity
    #[test]
    fn encode_imul_neg_arity(n in prop::sample::select(vec![0usize, 4, 5, 6])) {
        let mut ops = Vec::new();
        for i in 0..n {
            ops.push(Operand::Register(Register::new(GP32[i % 8])));
        }
        let r = sut_encode("imull", ops);
        prop_assert!(r.is_err(), "arity {n} must Err, got Ok({:02x?})", r.as_ref().ok());
    }

    // P11 strengthen: bare 32-bit forms only (no seg, no 16-bit) — should match llvm-mc
    #[test]
    fn encode_imul_diff_bare32_all_forms(
        form in 0u8..5,
        si in 0usize..8,
        di in 0usize..8,
        bi in 0usize..8,
        raw in any::<i16>(),
        edge in 0u8..10,
        disp in prop::sample::select(vec![0i64, 4, -1, 128]),
    ) {
        let src = GP32[si % 8];
        let dst = GP32[di % 8];
        let base = GP32[bi % 8];
        let imm = imm_edge(edge, raw as i32, 4);
        let mem = mem_base(base, Displacement::Integer(disp), None);
        let (asm, ops) = match form {
            0 => (
                format!("imull %{src}, %{dst}"),
                vec![
                    Operand::Register(Register::new(src)),
                    Operand::Register(Register::new(dst)),
                ],
            ),
            1 => (
                format!("imull {}, %{dst}", att_mem(&mem)),
                vec![
                    Operand::Memory(mem.clone()),
                    Operand::Register(Register::new(dst)),
                ],
            ),
            2 => (
                format!("imull ${imm}, %{dst}"),
                vec![
                    Operand::Immediate(ImmediateValue::Integer(imm)),
                    Operand::Register(Register::new(dst)),
                ],
            ),
            3 => (
                format!("imull ${imm}, %{src}, %{dst}"),
                vec![
                    Operand::Immediate(ImmediateValue::Integer(imm)),
                    Operand::Register(Register::new(src)),
                    Operand::Register(Register::new(dst)),
                ],
            ),
            4 => (
                format!("imull ${imm}, {}, %{dst}", att_mem(&mem)),
                vec![
                    Operand::Immediate(ImmediateValue::Integer(imm)),
                    Operand::Memory(mem.clone()),
                    Operand::Register(Register::new(dst)),
                ],
            ),
            _ => (
                format!("imull %{dst}"),
                vec![Operand::Register(Register::new(dst))],
            ),
        };
        let mc = llvm_mc_bytes(&asm)
            .map_err(|e| TestCaseError::fail(format!("llvm-mc failed on valid `{asm}`: {e}")))?;
        let sut = sut_encode("imull", ops)
            .map_err(|e| TestCaseError::fail(format!("SUT Err on valid `{asm}`: {e}")))?;
        prop_assert_eq!(&sut, &mc, "bare32 `{}`: sut={:02x?} mc={:02x?}", asm, &sut, &mc);
    }

    // P12 strengthen: mismatched width / non-GP should not silently encode as GP32
    #[test]
    fn encode_imul_neg_mismatched_or_non_gp(
        kind in 0u8..3,
        di in 0usize..8,
        ni in 0usize..10,
    ) {
        let dst32 = GP32[di % 8];
        let dst16 = R16[di % 8];
        let non = NON_GP[ni % NON_GP.len()];
        let (mnem, ops) = match kind {
            // imull with 16-bit src — llvm-mc rejects; SUT may alias via reg_num
            0 => (
                "imull",
                vec![
                    Operand::Register(Register::new(dst16)),
                    Operand::Register(Register::new(dst32)),
                ],
            ),
            1 => (
                "imulw",
                vec![
                    Operand::Register(Register::new(dst32)),
                    Operand::Register(Register::new(dst16)),
                ],
            ),
            _ => (
                "imull",
                vec![
                    Operand::Register(Register::new(non)),
                    Operand::Register(Register::new(dst32)),
                ],
            ),
        };
        let asm = match kind {
            0 => format!("{mnem} %{dst16}, %{dst32}"),
            1 => format!("{mnem} %{dst32}, %{dst16}"),
            _ => format!("{mnem} %{non}, %{dst32}"),
        };
        let mc = llvm_mc_bytes(&asm);
        let sut = sut_encode(mnem, ops);
        // If llvm-mc rejects, SUT must also Err (no silent wrong encoding).
        if mc.is_err() {
            prop_assert!(
                sut.is_err(),
                "SUT accepted invalid `{}` as Ok({:02x?}); llvm-mc rejected",
                asm,
                sut.as_ref().ok()
            );
        } else if let (Ok(s), Ok(m)) = (sut, mc) {
            prop_assert_eq!(&s, &m, "if both accept, bytes must match `{}`", asm);
        }
    }

    // P10: negative — unsupported shapes (Reg→Mem, Imm→Imm, non-GP)
    #[test]
    fn encode_imul_neg_unsupported_shape(
        kind in 0u8..4,
        bi in 0usize..8,
        ni in 0usize..10,
    ) {
        let base = GP32[bi % GP32.len()];
        let non = NON_GP[ni % NON_GP.len()];
        let ops = match kind {
            0 => vec![
                // Reg → Mem (Intel IMUL dest is always register for 2/3-op)
                Operand::Register(Register::new("eax")),
                Operand::Memory(mem_base(base, Displacement::None, None)),
            ],
            1 => vec![
                Operand::Immediate(ImmediateValue::Integer(1)),
                Operand::Immediate(ImmediateValue::Integer(2)),
            ],
            2 => vec![
                Operand::Register(Register::new(non)),
                Operand::Register(Register::new("eax")),
            ],
            _ => vec![
                Operand::Immediate(ImmediateValue::Integer(5)),
                Operand::Register(Register::new("eax")),
                Operand::Memory(mem_base(base, Displacement::None, None)),
            ],
        };
        let r = sut_encode("imull", ops);
        prop_assert!(
            r.is_err(),
            "unsupported shape kind={kind} must Err, got Ok({:02x?})",
            r.as_ref().ok()
        );
    }
}

// --- Deterministic regression witnesses (filled after first failing run) ---

#[test]
fn encode_imul_regression_imulw_rr_missing_66() {
    let mc = llvm_mc_bytes("imulw %ax, %bx").expect("llvm-mc");
    assert_eq!(mc, vec![0x66, 0x0f, 0xaf, 0xd8]);
    let sut = sut_encode(
        "imulw",
        vec![
            Operand::Register(Register::new("ax")),
            Operand::Register(Register::new("bx")),
        ],
    )
    .expect("SUT should accept imulw RR");
    assert_eq!(
        sut, mc,
        "imulw RR must emit 0x66 operand-size prefix; sut={sut:02x?} mc={mc:02x?}"
    );
}

#[test]
fn encode_imul_regression_segment_mem_reg() {
    let mc = llvm_mc_bytes("imull %es:(%eax), %ebx").expect("llvm-mc");
    assert_eq!(mc, vec![0x26, 0x0f, 0xaf, 0x18]);
    let mem = mem_base("eax", Displacement::None, Some("es"));
    let sut = sut_encode(
        "imull",
        vec![
            Operand::Memory(mem),
            Operand::Register(Register::new("ebx")),
        ],
    )
    .expect("SUT should accept segment mem");
    assert_eq!(
        sut, mc,
        "Mem→Reg must emit segment prefix; sut={sut:02x?} mc={mc:02x?}"
    );
}

#[test]
fn encode_imul_regression_imulw_imm16_width() {
    let mc = llvm_mc_bytes("imulw $300, %ax").expect("llvm-mc");
    assert_eq!(mc, vec![0x66, 0x69, 0xc0, 0x2c, 0x01]);
    let sut = sut_encode(
        "imulw",
        vec![
            Operand::Immediate(ImmediateValue::Integer(300)),
            Operand::Register(Register::new("ax")),
        ],
    )
    .expect("SUT should accept imulw imm");
    assert_eq!(
        sut, mc,
        "imulw imm16 must be 0x66 + 0x69 + imm16 (not imm32); sut={sut:02x?} mc={mc:02x?}"
    );
}

#[test]
fn encode_imul_regression_seg_imm_mem_reg() {
    let mc = llvm_mc_bytes("imull $5, %es:(%eax), %ebx").expect("llvm-mc");
    assert_eq!(mc, vec![0x26, 0x6b, 0x18, 0x05]);
    let mem = mem_base("eax", Displacement::None, Some("es"));
    let sut = sut_encode(
        "imull",
        vec![
            Operand::Immediate(ImmediateValue::Integer(5)),
            Operand::Memory(mem),
            Operand::Register(Register::new("ebx")),
        ],
    )
    .expect("SUT should accept 3-op seg");
    assert_eq!(
        sut, mc,
        "3-op Imm,Mem,Reg must emit segment prefix; sut={sut:02x?} mc={mc:02x?}"
    );
}

#[test]
fn encode_imul_regression_unary_segment() {
    let mc = llvm_mc_bytes("imull %es:(%eax)").expect("llvm-mc");
    assert_eq!(mc, vec![0x26, 0xf7, 0x28]);
    let mem = mem_base("eax", Displacement::None, Some("es"));
    let sut = sut_encode("imull", vec![Operand::Memory(mem)]).expect("SUT unary mem");
    assert_eq!(
        sut, mc,
        "1-op mem must emit segment prefix via encode_unary_rm; sut={sut:02x?} mc={mc:02x?}"
    );
}
