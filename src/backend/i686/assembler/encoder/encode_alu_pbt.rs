// Oracle: differential — llvm-mc i686 assembler (ALU: ADD/OR/ADC/SBB/AND/SUB/XOR/CMP)
// Evidence: gp_integer.rs:433 encode_alu; mod.rs:204-211 add/or/adc/sbb/and/sub/xor/cmp → encode_alu;
//   Intel SDM Vol.2 ALU r/m forms (00/01/02/03 + op*8; 80/81/83 /r; short AL/AX/EAX 04/05+op*8);
//   core.rs:31-42 emit_segment_prefix for fs/gs/es/cs/ss/ds;
//   x86-64 sibling gp_integer.rs encode_alu calls emit_segment_prefix on memory arms.
// Stronger considered:
//   - State machine: rejected — pure encoding, no lifecycle
//   - Algebraic round-trip: rejected — no in-tree i686 ALU decoder
// Differential: candidate=encode_alu (via InstructionEncoder::encode *l/*w/*b),
//   reference=llvm-mc -triple=i686 -show-encoding (LLVM 15),
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=AT&T operand bytes <-> llvm-mc encoding bytes.
// Also: algebraic.invariant (RR opcode/modrm), algebraic.metamorphic (seg ‖ bare),
//   negative_error (arity / mismatched width / non-GP).

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

const ALU_OPS: &[&str] = &["add", "or", "adc", "sbb", "and", "sub", "xor", "cmp"];
const GP32: &[&str] = &["eax", "ecx", "edx", "ebx", "esp", "ebp", "esi", "edi"];
const R16: &[&str] = &["ax", "cx", "dx", "bx", "sp", "bp", "si", "di"];
const R8: &[&str] = &["al", "cl", "dl", "bl", "ah", "ch", "dh", "bh"];
const SREGS: &[&str] = &["es", "cs", "ss", "ds", "fs", "gs"];
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

fn mnemonic(op: &str, w: u8) -> String {
    match w {
        1 => format!("{op}b"),
        2 => format!("{op}w"),
        _ => format!("{op}l"),
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
    if let Some(b) = &mem.base {
        s.push('%');
        s.push_str(&b.name);
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

/// Clamp generated imm into a domain llvm-mc accepts for the given width.
fn imm_for_width(raw: i64, width: u8) -> i64 {
    match width {
        1 => {
            // Keep in i8 range so gas/llvm-mc don't re-encode as truncated surprise
            let v = raw.rem_euclid(256) as i8 as i64;
            v
        }
        2 => (raw as i16) as i64,
        _ => (raw as i32) as i64,
    }
}

// --- KAT gate (reference oracle prerequisite) ---

#[test]
fn encode_alu_kat_llvm_mc_addl_rr() {
    let mc = llvm_mc_bytes("addl %eax, %ebx").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x01, 0xc3], "llvm-mc KAT mapping broken");
    let sut = sut_encode(
        "addl",
        vec![
            Operand::Register(Register::new("eax")),
            Operand::Register(Register::new("ebx")),
        ],
    )
    .expect("SUT KAT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_alu_kat_llvm_mc_subl_imm_eax_short() {
    // Large imm on EAX uses short form 0x2D (= 0x05 + 5*8)
    let mc = llvm_mc_bytes("subl $128, %eax").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x2d, 0x80, 0x00, 0x00, 0x00]);
    let sut = sut_encode(
        "subl",
        vec![
            Operand::Immediate(ImmediateValue::Integer(128)),
            Operand::Register(Register::new("eax")),
        ],
    )
    .expect("SUT KAT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_alu_kat_llvm_mc_addb_imm_bl() {
    // Non-AL imm8 uses 80 /r — both SUT and llvm-mc agree (AL short form is a separate bug).
    let mc = llvm_mc_bytes("addb $1, %bl").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x80, 0xc3, 0x01]);
    let sut = sut_encode(
        "addb",
        vec![
            Operand::Immediate(ImmediateValue::Integer(1)),
            Operand::Register(Register::new("bl")),
        ],
    )
    .expect("SUT KAT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_alu_kat_llvm_mc_addb_imm_al_short_reference_only() {
    // Reference gate: llvm-mc prefers AL short form 04 ib (SUT comparison is the regression).
    let mc = llvm_mc_bytes("addb $1, %al").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x04, 0x01]);
}

#[test]
fn encode_alu_kat_llvm_mc_xorl_identity() {
    let mc = llvm_mc_bytes("xorl %eax, %eax").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x31, 0xc0]);
    let sut = sut_encode(
        "xorl",
        vec![
            Operand::Register(Register::new("eax")),
            Operand::Register(Register::new("eax")),
        ],
    )
    .expect("SUT KAT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_alu_kat_llvm_mc_addl_mem_reg() {
    let mc = llvm_mc_bytes("addl (%eax), %ebx").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x03, 0x18]);
    let mem = mem_base("eax", Displacement::None, None);
    let sut = sut_encode(
        "addl",
        vec![
            Operand::Memory(mem),
            Operand::Register(Register::new("ebx")),
        ],
    )
    .expect("SUT KAT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_alu_kat_llvm_mc_es_segment_reference_only() {
    // Reference gate: llvm-mc emits 0x26 for %es: (SUT comparison is the regression).
    let mc = llvm_mc_bytes("addl %ebx, %es:(%eax)").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x26, 0x01, 0x18]);
}

proptest! {
    #![proptest_config(cfg())]

    // P1: differential — same-width GP RR pairs match llvm-mc
    #[test]
    fn encode_alu_diff_rr_same_width(
        op_i in 0usize..8,
        width in prop::sample::select(vec![1u8, 2, 4]),
        si in 0usize..8,
        di in 0usize..8,
    ) {
        let op = ALU_OPS[op_i % ALU_OPS.len()];
        let regs = gp_for_width(width);
        let src = regs[si % regs.len()];
        let dst = regs[di % regs.len()];
        let mnem = mnemonic(op, width);
        let asm = format!("{mnem} %{src}, %{dst}");
        let mc = llvm_mc_bytes(&asm)
            .map_err(|e| TestCaseError::fail(format!("llvm-mc failed on valid `{asm}`: {e}")))?;
        let sut = sut_encode(
            &mnem,
            vec![
                Operand::Register(Register::new(src)),
                Operand::Register(Register::new(dst)),
            ],
        )
        .map_err(|e| TestCaseError::fail(format!("SUT Err on valid `{asm}`: {e}")))?;
        prop_assert_eq!(&sut, &mc, "diff RR `{}`: sut={:02x?} mc={:02x?}", asm, &sut, &mc);
    }

    // P2: differential — imm → reg (incl. sign-ext imm8, EAX short form, AL short form)
    #[test]
    fn encode_alu_diff_imm_reg(
        op_i in 0usize..8,
        width in prop::sample::select(vec![1u8, 2, 4]),
        di in 0usize..8,
        raw in any::<i32>(),
        edge in 0u8..8,
    ) {
        let op = ALU_OPS[op_i % ALU_OPS.len()];
        let regs = gp_for_width(width);
        let dst = regs[di % regs.len()];
        // Skew generators toward sign-ext boundary and short-form cutover
        let imm = match edge {
            0 => -128i64,
            1 => -129,
            2 => 127,
            3 => 128,
            4 => 0,
            5 => -1,
            6 => if width == 1 { 0x7fi64 } else { 0x7fff },
            _ => imm_for_width(raw as i64, width),
        };
        let imm = if edge < 7 { imm } else { imm_for_width(imm, width) };
        // For width 1 keep imm in i8 so both sides see the same value
        let imm = if width == 1 { (imm as i8) as i64 } else { imm };

        let mnem = mnemonic(op, width);
        let asm = format!("{mnem} ${imm}, %{dst}");
        let mc = llvm_mc_bytes(&asm)
            .map_err(|e| TestCaseError::fail(format!("llvm-mc failed on valid `{asm}`: {e}")))?;
        let sut = sut_encode(
            &mnem,
            vec![
                Operand::Immediate(ImmediateValue::Integer(imm)),
                Operand::Register(Register::new(dst)),
            ],
        )
        .map_err(|e| TestCaseError::fail(format!("SUT Err on valid `{asm}`: {e}")))?;
        prop_assert_eq!(&sut, &mc, "diff imm→reg `{}`: sut={:02x?} mc={:02x?}", asm, &sut, &mc);
    }

    // P3: differential — memory forms without segment (m→r, r→m, imm→m)
    #[test]
    fn encode_alu_diff_mem_forms(
        op_i in 0usize..8,
        width in prop::sample::select(vec![1u8, 2, 4]),
        shape in 0u8..3,
        bi in 0usize..8,
        ri in 0usize..8,
        disp in prop::sample::select(vec![0i64, 1, -1, 4, 127, 128, -128, 0x1000]),
        raw_imm in any::<i16>(),
    ) {
        let op = ALU_OPS[op_i % ALU_OPS.len()];
        let base = GP32[bi % GP32.len()];
        // Avoid esp-only weirdness already covered; keep all bases
        let regs = gp_for_width(width);
        let reg = regs[ri % regs.len()];
        let mnem = mnemonic(op, width);
        let mem = mem_base(base, Displacement::Integer(disp), None);
        let imm = imm_for_width(raw_imm as i64, width);
        let imm = if width == 1 { (imm as i8) as i64 } else { imm };

        let (asm, ops) = match shape {
            0 => {
                // mem → reg
                let asm = format!("{mnem} {}, %{reg}", att_mem(&mem));
                let ops = vec![
                    Operand::Memory(mem.clone()),
                    Operand::Register(Register::new(reg)),
                ];
                (asm, ops)
            }
            1 => {
                // reg → mem
                let asm = format!("{mnem} %{reg}, {}", att_mem(&mem));
                let ops = vec![
                    Operand::Register(Register::new(reg)),
                    Operand::Memory(mem.clone()),
                ];
                (asm, ops)
            }
            _ => {
                // imm → mem
                let asm = format!("{mnem} ${imm}, {}", att_mem(&mem));
                let ops = vec![
                    Operand::Immediate(ImmediateValue::Integer(imm)),
                    Operand::Memory(mem.clone()),
                ];
                (asm, ops)
            }
        };

        let mc = llvm_mc_bytes(&asm)
            .map_err(|e| TestCaseError::fail(format!("llvm-mc failed on valid `{asm}`: {e}")))?;
        let sut = sut_encode(&mnem, ops)
            .map_err(|e| TestCaseError::fail(format!("SUT Err on valid `{asm}`: {e}")))?;
        prop_assert_eq!(&sut, &mc, "diff mem `{}`: sut={:02x?} mc={:02x?}", asm, &sut, &mc);
    }

    // P4: differential — all six segment overrides on memory forms
    #[test]
    fn encode_alu_diff_segment_prefix(
        op_i in 0usize..8,
        width in prop::sample::select(vec![1u8, 2, 4]),
        shape in 0u8..3,
        si in 0usize..6,
        bi in 0usize..8,
        ri in 0usize..8,
        disp in prop::sample::select(vec![0i64, 4, -8]),
    ) {
        let op = ALU_OPS[op_i % ALU_OPS.len()];
        let seg = SREGS[si % SREGS.len()];
        let base = GP32[bi % GP32.len()];
        let regs = gp_for_width(width);
        let reg = regs[ri % regs.len()];
        let mnem = mnemonic(op, width);
        let mem = mem_base(base, Displacement::Integer(disp), Some(seg));
        let imm = 1i64;

        let (asm, ops) = match shape {
            0 => {
                let asm = format!("{mnem} {}, %{reg}", att_mem(&mem));
                let ops = vec![
                    Operand::Memory(mem.clone()),
                    Operand::Register(Register::new(reg)),
                ];
                (asm, ops)
            }
            1 => {
                let asm = format!("{mnem} %{reg}, {}", att_mem(&mem));
                let ops = vec![
                    Operand::Register(Register::new(reg)),
                    Operand::Memory(mem.clone()),
                ];
                (asm, ops)
            }
            _ => {
                let asm = format!("{mnem} ${imm}, {}", att_mem(&mem));
                let ops = vec![
                    Operand::Immediate(ImmediateValue::Integer(imm)),
                    Operand::Memory(mem.clone()),
                ];
                (asm, ops)
            }
        };

        let mc = llvm_mc_bytes(&asm)
            .map_err(|e| TestCaseError::fail(format!("llvm-mc failed on valid `{asm}`: {e}")))?;
        let sut = sut_encode(&mnem, ops).map_err(|e| {
            TestCaseError::fail(format!(
                "SUT rejected valid segment form `{asm}`: {e}; \
                 i686 ALU mem must emit segment override (core.rs emit_segment_prefix; \
                 x86-64 sibling encode_alu calls it)"
            ))
        })?;
        prop_assert_eq!(
            &sut, &mc,
            "segment diff `{}`: sut={:02x?} mc={:02x?}; expected prefix 0x{:02x}",
            asm, &sut, &mc, seg_prefix_byte(seg)
        );
    }

    // P5: algebraic invariant — RR opcode / optional 0x66 / mod=3 / fields
    #[test]
    fn encode_alu_invariant_rr_opcode_modrm(
        op_i in 0usize..8,
        width in prop::sample::select(vec![1u8, 2, 4]),
        si in 0usize..8,
        di in 0usize..8,
    ) {
        let op = ALU_OPS[op_i % ALU_OPS.len()];
        let alu_op = (op_i % 8) as u8;
        let regs = gp_for_width(width);
        let src = regs[si % regs.len()];
        let dst = regs[di % regs.len()];
        let mnem = mnemonic(op, width);
        let bytes = sut_encode(
            &mnem,
            vec![
                Operand::Register(Register::new(src)),
                Operand::Register(Register::new(dst)),
            ],
        )
        .map_err(|e| TestCaseError::fail(e))?;
        let mut i = 0usize;
        if width == 2 {
            prop_assert_eq!(bytes[i], 0x66, "missing 0x66 for *w");
            i += 1;
        } else {
            prop_assert_ne!(bytes.get(0).copied().unwrap_or(0), 0x66, "spurious 0x66");
        }
        let expect_op = if width == 1 {
            0x00u8 + alu_op * 8
        } else {
            0x01u8 + alu_op * 8
        };
        prop_assert_eq!(bytes[i], expect_op, "RR opcode for {}", mnem);
        i += 1;
        prop_assert_eq!(bytes.len(), i + 1, "exact length");
        let modrm = bytes[i];
        prop_assert_eq!(modrm >> 6, 0b11, "mod must be 11 (register)");
        prop_assert_eq!((modrm >> 3) & 7, gp_num(src), "ModRM.reg = src");
        prop_assert_eq!(modrm & 7, gp_num(dst), "ModRM.rm = dst");
    }

    // P6: metamorphic — segmented mem = seg_prefix ‖ bare mem
    #[test]
    fn encode_alu_metamorphic_segment_prefix(
        op_i in 0usize..8,
        width in prop::sample::select(vec![1u8, 2, 4]),
        shape in 0u8..3,
        si in 0usize..6,
        bi in 0usize..8,
        ri in 0usize..8,
    ) {
        let op = ALU_OPS[op_i % ALU_OPS.len()];
        let seg = SREGS[si % SREGS.len()];
        let base = GP32[bi % GP32.len()];
        let regs = gp_for_width(width);
        let reg = regs[ri % regs.len()];
        let mnem = mnemonic(op, width);
        let bare = mem_base(base, Displacement::None, None);
        let segd = mem_base(base, Displacement::None, Some(seg));
        let imm = 5i64;

        let (ops_bare, ops_seg) = match shape {
            0 => (
                vec![
                    Operand::Memory(bare.clone()),
                    Operand::Register(Register::new(reg)),
                ],
                vec![
                    Operand::Memory(segd.clone()),
                    Operand::Register(Register::new(reg)),
                ],
            ),
            1 => (
                vec![
                    Operand::Register(Register::new(reg)),
                    Operand::Memory(bare.clone()),
                ],
                vec![
                    Operand::Register(Register::new(reg)),
                    Operand::Memory(segd.clone()),
                ],
            ),
            _ => (
                vec![
                    Operand::Immediate(ImmediateValue::Integer(imm)),
                    Operand::Memory(bare.clone()),
                ],
                vec![
                    Operand::Immediate(ImmediateValue::Integer(imm)),
                    Operand::Memory(segd.clone()),
                ],
            ),
        };

        let bare_bytes = sut_encode(&mnem, ops_bare)
            .map_err(|e| TestCaseError::fail(format!("bare encode: {e}")))?;
        let seg_bytes = sut_encode(&mnem, ops_seg).map_err(|e| {
            TestCaseError::fail(format!(
                "segmented encode must succeed (emit_segment_prefix contract): {e}"
            ))
        })?;
        let mut expect = vec![seg_prefix_byte(seg)];
        expect.extend_from_slice(&bare_bytes);
        prop_assert_eq!(
            &seg_bytes, &expect,
            "metamorphic seg||bare for {} %{}:(%{}): got {:02x?} expect {:02x?}",
            mnem, seg, base, &seg_bytes, &expect
        );
    }

    // P7a: negative — wrong arity
    #[test]
    fn encode_alu_neg_arity(
        op_i in 0usize..8,
        width in prop::sample::select(vec![1u8, 2, 4]),
        arity in 0u8..4,
        ri in 0usize..8,
    ) {
        prop_assume!(arity != 2);
        let op = ALU_OPS[op_i % ALU_OPS.len()];
        let mnem = mnemonic(op, width);
        let regs = gp_for_width(width);
        let r = regs[ri % regs.len()];
        let ops: Vec<Operand> = (0..arity)
            .map(|_| Operand::Register(Register::new(r)))
            .collect();
        match sut_encode(&mnem, ops) {
            Err(_) => {}
            Ok(bytes) => {
                return Err(TestCaseError::fail(format!(
                    "SUT accepted arity-{arity} `{mnem}` → {bytes:02x?}; \
                     encode_alu requires exactly 2 operands"
                )));
            }
        }
    }

    // P7b: negative — size-mismatched GP pairs that llvm-mc rejects
    #[test]
    fn encode_alu_neg_mismatched_width(
        op_i in 0usize..8,
        mode in 0u8..6,
        a_i in 0usize..8,
        b_i in 0usize..8,
    ) {
        let op = ALU_OPS[op_i % ALU_OPS.len()];
        let (mnem, src, dst) = match mode {
            0 => {
                let src = R16[a_i % R16.len()];
                let dst = GP32[b_i % GP32.len()];
                (mnemonic(op, 4), src, dst)
            }
            1 => {
                let src = GP32[a_i % GP32.len()];
                let dst = R16[b_i % R16.len()];
                (mnemonic(op, 4), src, dst)
            }
            2 => {
                let src = GP32[a_i % GP32.len()];
                let dst = R16[b_i % R16.len()];
                (mnemonic(op, 2), src, dst)
            }
            3 => {
                let src = R16[a_i % R16.len()];
                let dst = R8[b_i % R8.len()];
                (mnemonic(op, 2), src, dst)
            }
            4 => {
                let src = GP32[a_i % GP32.len()];
                let dst = R8[b_i % R8.len()];
                (mnemonic(op, 1), src, dst)
            }
            _ => {
                let src = R8[a_i % R8.len()];
                let dst = R16[b_i % R16.len()];
                (mnemonic(op, 1), src, dst)
            }
        };
        let expected_w = if mnem.ends_with('b') {
            1u8
        } else if mnem.ends_with('w') {
            2u8
        } else {
            4u8
        };
        let sw = reg_size_local(src);
        let dw = reg_size_local(dst);
        prop_assume!(sw != expected_w || dw != expected_w);

        let asm = format!("{mnem} %{src}, %{dst}");
        prop_assert!(
            llvm_mc_bytes(&asm).is_err(),
            "expected llvm-mc to reject mismatched `{asm}`"
        );
        match sut_encode(
            &mnem,
            vec![
                Operand::Register(Register::new(src)),
                Operand::Register(Register::new(dst)),
            ],
        ) {
            Err(_) => {}
            Ok(bytes) => {
                return Err(TestCaseError::fail(format!(
                    "SUT accepted size-mismatched `{asm}` → {bytes:02x?}; \
                     ALU RR requires matching operand size (Intel SDM; llvm-mc rejects). \
                     reg_num aliases widths so bytes look like a valid same-width form."
                )));
            }
        }
    }

    // P7c: negative — non-GP names that alias through reg_num must Err
    #[test]
    fn encode_alu_neg_non_gp(
        op_i in 0usize..8,
        ni in 0usize..10,
        gi in 0usize..8,
        width in prop::sample::select(vec![1u8, 2, 4]),
        non_gp_as_src in any::<bool>(),
    ) {
        let op = ALU_OPS[op_i % ALU_OPS.len()];
        let non_gp = NON_GP[ni % NON_GP.len()];
        let gp = gp_for_width(width)[gi % 8];
        let mnem = mnemonic(op, width);
        let (src, dst) = if non_gp_as_src {
            (non_gp, gp)
        } else {
            (gp, non_gp)
        };
        let asm = format!("{mnem} %{src}, %{dst}");
        prop_assert!(
            llvm_mc_bytes(&asm).is_err(),
            "expected llvm-mc to reject non-GP `{asm}`"
        );
        match sut_encode(
            &mnem,
            vec![
                Operand::Register(Register::new(src)),
                Operand::Register(Register::new(dst)),
            ],
        ) {
            Err(_) => {}
            Ok(bytes) => {
                return Err(TestCaseError::fail(format!(
                    "SUT accepted non-GP ALU RR `{asm}` → {bytes:02x?}; \
                     ALU r/m forms are GP-only (Intel SDM). reg_num aliases xmm/mm/st to 0-7."
                )));
            }
        }
    }
}

// --- Deterministic regression placeholders (filled after triage) ---

#[test]
fn encode_alu_regression_es_segment_prefix_reg_mem() {
    // Witness: addl %ebx, %es:(%eax) — SUT must emit 0x26 prefix
    let mc = llvm_mc_bytes("addl %ebx, %es:(%eax)").expect("llvm-mc");
    assert_eq!(mc, vec![0x26, 0x01, 0x18]);
    let mem = mem_base("eax", Displacement::None, Some("es"));
    let sut = sut_encode(
        "addl",
        vec![
            Operand::Register(Register::new("ebx")),
            Operand::Memory(mem),
        ],
    );
    match sut {
        Ok(bytes) => assert_eq!(
            bytes, mc,
            "regression: segmented ALU must match llvm-mc (got {bytes:02x?})"
        ),
        Err(e) => panic!("regression: segmented ALU must encode, got Err({e})"),
    }
}

#[test]
fn encode_alu_regression_addb_al_short_form() {
    // Witness: addb $1, %al — preferred short form 04 ib (Intel SDM; llvm-mc)
    let mc = llvm_mc_bytes("addb $1, %al").expect("llvm-mc");
    assert_eq!(mc, vec![0x04, 0x01]);
    let sut = sut_encode(
        "addb",
        vec![
            Operand::Immediate(ImmediateValue::Integer(1)),
            Operand::Register(Register::new("al")),
        ],
    )
    .expect("must encode");
    assert_eq!(
        sut, mc,
        "regression: AL imm8 short form (got {sut:02x?})"
    );
}

#[test]
fn encode_alu_regression_mismatched_width_addl_ax_ebx() {
    // Witness: addl %ax, %ebx — llvm-mc rejects; SUT must Err
    assert!(llvm_mc_bytes("addl %ax, %ebx").is_err());
    let r = sut_encode(
        "addl",
        vec![
            Operand::Register(Register::new("ax")),
            Operand::Register(Register::new("ebx")),
        ],
    );
    assert!(
        r.is_err(),
        "regression: size-mismatched ALU must Err, got {r:?}"
    );
}

#[test]
fn encode_alu_regression_non_gp_xmm() {
    // Witness: addb %al, %xmm0 — llvm-mc rejects; SUT must Err
    assert!(llvm_mc_bytes("addb %al, %xmm0").is_err());
    let r = sut_encode(
        "addb",
        vec![
            Operand::Register(Register::new("al")),
            Operand::Register(Register::new("xmm0")),
        ],
    );
    assert!(
        r.is_err(),
        "regression: non-GP ALU must Err, got {r:?}"
    );
}
