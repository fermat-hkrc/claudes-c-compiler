// Oracle: differential — llvm-mc i686 assembler (TEST)
// Evidence: gp_integer.rs:645 encode_test; mod.rs:214 testl|testw|testb|test → encode_test;
//   Intel SDM Vol.2 TEST (84/85 r/m,r; A8/A9 AL/AX/EAX imm; F6/F7 /0 r/m,imm);
//   core.rs:31-42 emit_segment_prefix for fs/gs/es/cs/ss/ds;
//   x86-64 sibling gp_integer.rs encode_test has Reg→Mem + emit_segment_prefix on both mem arms.
// Stronger considered:
//   - State machine: rejected — pure encoding, no lifecycle
//   - Algebraic round-trip: rejected — no in-tree i686 TEST decoder
// Differential: candidate=encode_test (via InstructionEncoder::encode test*),
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

fn mnemonic(w: u8) -> &'static str {
    match w {
        1 => "testb",
        2 => "testw",
        _ => "testl",
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

fn imm_for_width(raw: i64, width: u8) -> i64 {
    match width {
        1 => (raw.rem_euclid(256) as i8) as i64,
        2 => (raw as i16) as i64,
        _ => (raw as i32) as i64,
    }
}

// --- KAT gate (reference oracle prerequisite) ---

#[test]
fn encode_test_kat_llvm_mc_testl_rr() {
    let mc = llvm_mc_bytes("testl %eax, %ebx").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x85, 0xc3], "llvm-mc KAT mapping broken");
    let sut = sut_encode(
        "testl",
        vec![
            Operand::Register(Register::new("eax")),
            Operand::Register(Register::new("ebx")),
        ],
    )
    .expect("SUT KAT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_test_kat_llvm_mc_testb_al_short() {
    let mc = llvm_mc_bytes("testb $1, %al").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0xa8, 0x01]);
    let sut = sut_encode(
        "testb",
        vec![
            Operand::Immediate(ImmediateValue::Integer(1)),
            Operand::Register(Register::new("al")),
        ],
    )
    .expect("SUT KAT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_test_kat_llvm_mc_testl_imm_ebx() {
    let mc = llvm_mc_bytes("testl $1, %ebx").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0xf7, 0xc3, 0x01, 0x00, 0x00, 0x00]);
    let sut = sut_encode(
        "testl",
        vec![
            Operand::Immediate(ImmediateValue::Integer(1)),
            Operand::Register(Register::new("ebx")),
        ],
    )
    .expect("SUT KAT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_test_kat_llvm_mc_testl_imm_mem() {
    let mc = llvm_mc_bytes("testl $1, (%eax)").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0xf7, 0x00, 0x01, 0x00, 0x00, 0x00]);
    let mem = mem_base("eax", Displacement::None, None);
    let sut = sut_encode(
        "testl",
        vec![
            Operand::Immediate(ImmediateValue::Integer(1)),
            Operand::Memory(mem),
        ],
    )
    .expect("SUT KAT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_test_kat_llvm_mc_reg_mem_reference_only() {
    // Reference gate: llvm-mc accepts testl %eax, (%ebx) as 85 03 (SUT comparison is the property/regression).
    let mc = llvm_mc_bytes("testl %eax, (%ebx)").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x85, 0x03]);
}

#[test]
fn encode_test_kat_llvm_mc_es_segment_reference_only() {
    // Reference gate: llvm-mc emits 0x26 for %es: on Imm→Mem.
    let mc = llvm_mc_bytes("testl $5, %es:(%eax)").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x26, 0xf7, 0x00, 0x05, 0x00, 0x00, 0x00]);
}

proptest! {
    #![proptest_config(cfg())]

    // P1: differential — same-width GP RR pairs match llvm-mc
    #[test]
    fn encode_test_diff_rr_same_width(
        width in prop::sample::select(vec![1u8, 2, 4]),
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

    // P2: differential — imm → reg (AL/AX/EAX short + F6/F7 /0)
    #[test]
    fn encode_test_diff_imm_reg(
        width in prop::sample::select(vec![1u8, 2, 4]),
        di in 0usize..8,
        raw in any::<i32>(),
        edge in 0u8..8,
    ) {
        let regs = gp_for_width(width);
        let dst = regs[di % regs.len()];
        let imm = match edge {
            0 => -128i64,
            1 => -129,
            2 => 127,
            3 => 128,
            4 => 0,
            5 => -1,
            6 => if width == 1 { 0x7fi64 } else if width == 2 { 0x7fff } else { 0x7fffffff },
            _ => imm_for_width(raw as i64, width),
        };
        let imm = if edge < 7 {
            if width == 1 {
                (imm as i8) as i64
            } else if width == 2 {
                (imm as i16) as i64
            } else {
                (imm as i32) as i64
            }
        } else {
            imm_for_width(imm, width)
        };

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

    // P3: differential — Imm→Mem bare (no segment)
    #[test]
    fn encode_test_diff_imm_mem_bare(
        width in prop::sample::select(vec![1u8, 2, 4]),
        bi in 0usize..8,
        disp in prop::sample::select(vec![0i64, 1, -1, 4, 127, 128, -128, 0x1000]),
        raw_imm in any::<i16>(),
    ) {
        let base = GP32[bi % GP32.len()];
        let mnem = mnemonic(width);
        let mem = mem_base(base, Displacement::Integer(disp), None);
        let imm = imm_for_width(raw_imm as i64, width);
        let imm = if width == 1 { (imm as i8) as i64 } else { imm };

        let asm = format!("{mnem} ${imm}, {}", att_mem(&mem));
        let ops = vec![
            Operand::Immediate(ImmediateValue::Integer(imm)),
            Operand::Memory(mem),
        ];
        let mc = llvm_mc_bytes(&asm)
            .map_err(|e| TestCaseError::fail(format!("llvm-mc failed on valid `{asm}`: {e}")))?;
        let sut = sut_encode(mnem, ops)
            .map_err(|e| TestCaseError::fail(format!("SUT Err on valid `{asm}`: {e}")))?;
        prop_assert_eq!(&sut, &mc, "diff imm→mem `{}`: sut={:02x?} mc={:02x?}", asm, &sut, &mc);
    }

    // P4: differential — Reg→Mem bare (Intel TEST r/m,r; AT&T test %reg, mem)
    #[test]
    fn encode_test_diff_reg_mem_bare(
        width in prop::sample::select(vec![1u8, 2, 4]),
        si in 0usize..8,
        bi in 0usize..8,
        disp in prop::sample::select(vec![0i64, 1, -1, 4, 127, 128, -128, 0x1000]),
    ) {
        let regs = gp_for_width(width);
        let src = regs[si % regs.len()];
        let base = GP32[bi % GP32.len()];
        let mnem = mnemonic(width);
        let mem = mem_base(base, Displacement::Integer(disp), None);
        let asm = format!("{mnem} %{src}, {}", att_mem(&mem));
        let ops = vec![
            Operand::Register(Register::new(src)),
            Operand::Memory(mem),
        ];
        let mc = llvm_mc_bytes(&asm)
            .map_err(|e| TestCaseError::fail(format!("llvm-mc failed on valid `{asm}`: {e}")))?;
        let sut = sut_encode(mnem, ops).map_err(|e| {
            TestCaseError::fail(format!(
                "SUT rejected valid Reg→Mem TEST `{asm}`: {e}; \
                 Intel TEST r/m,r / AT&T test %reg, mem; x86-64 sibling encode_test has this arm"
            ))
        })?;
        prop_assert_eq!(&sut, &mc, "diff reg→mem `{}`: sut={:02x?} mc={:02x?}", asm, &sut, &mc);
    }

    // P5: differential — segment overrides on Imm→Mem and Reg→Mem
    #[test]
    fn encode_test_diff_segment_prefix(
        width in prop::sample::select(vec![1u8, 2, 4]),
        form in 0u8..2,
        si in 0usize..6,
        bi in 0usize..8,
        ri in 0usize..8,
        disp in prop::sample::select(vec![0i64, 4, -8]),
    ) {
        let seg = SREGS[si % SREGS.len()];
        let base = GP32[bi % GP32.len()];
        let regs = gp_for_width(width);
        let reg = regs[ri % regs.len()];
        let mnem = mnemonic(width);
        let mem = mem_base(base, Displacement::Integer(disp), Some(seg));
        let imm = 5i64;

        let (asm, ops) = if form == 0 {
            let asm = format!("{mnem} ${imm}, {}", att_mem(&mem));
            let ops = vec![
                Operand::Immediate(ImmediateValue::Integer(imm)),
                Operand::Memory(mem.clone()),
            ];
            (asm, ops)
        } else {
            let asm = format!("{mnem} %{reg}, {}", att_mem(&mem));
            let ops = vec![
                Operand::Register(Register::new(reg)),
                Operand::Memory(mem.clone()),
            ];
            (asm, ops)
        };

        let mc = llvm_mc_bytes(&asm)
            .map_err(|e| TestCaseError::fail(format!("llvm-mc failed on valid `{asm}`: {e}")))?;
        let sut = sut_encode(mnem, ops).map_err(|e| {
            TestCaseError::fail(format!(
                "SUT rejected valid segment TEST `{asm}`: {e}; \
                 i686 TEST mem must emit segment override (core.rs emit_segment_prefix; \
                 x86-64 sibling encode_test calls it)"
            ))
        })?;
        prop_assert_eq!(
            &sut, &mc,
            "segment diff `{}`: sut={:02x?} mc={:02x?}; expected prefix 0x{:02x}",
            asm, &sut, &mc, seg_prefix_byte(seg)
        );
    }

    // P6: algebraic invariant — RR opcode / optional 0x66 / mod=3 / fields
    #[test]
    fn encode_test_invariant_rr_opcode_modrm(
        width in prop::sample::select(vec![1u8, 2, 4]),
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
            prop_assert_eq!(bytes[i], 0x66, "missing 0x66 for testw");
            i += 1;
        } else {
            prop_assert_ne!(bytes.get(0).copied().unwrap_or(0), 0x66, "spurious 0x66");
        }
        let expect_op = if width == 1 { 0x84u8 } else { 0x85u8 };
        prop_assert_eq!(bytes[i], expect_op, "RR opcode for {}", mnem);
        i += 1;
        prop_assert_eq!(bytes.len(), i + 1, "exact length");
        let modrm = bytes[i];
        prop_assert_eq!(modrm >> 6, 0b11, "mod must be 11 (register)");
        prop_assert_eq!((modrm >> 3) & 7, gp_num(src), "ModRM.reg = src");
        prop_assert_eq!(modrm & 7, gp_num(dst), "ModRM.rm = dst");
    }

    // P7: metamorphic — segmented Imm→Mem = seg_prefix ‖ bare Imm→Mem
    #[test]
    fn encode_test_metamorphic_segment_prefix(
        width in prop::sample::select(vec![1u8, 2, 4]),
        si in 0usize..6,
        bi in 0usize..8,
        raw_imm in any::<i8>(),
    ) {
        let seg = SREGS[si % SREGS.len()];
        let base = GP32[bi % GP32.len()];
        let mnem = mnemonic(width);
        let bare = mem_base(base, Displacement::None, None);
        let segd = mem_base(base, Displacement::None, Some(seg));
        let imm = imm_for_width(raw_imm as i64, width);
        let imm = if width == 1 { (imm as i8) as i64 } else { imm };

        let ops_bare = vec![
            Operand::Immediate(ImmediateValue::Integer(imm)),
            Operand::Memory(bare),
        ];
        let ops_seg = vec![
            Operand::Immediate(ImmediateValue::Integer(imm)),
            Operand::Memory(segd),
        ];

        let bare_bytes = sut_encode(mnem, ops_bare)
            .map_err(|e| TestCaseError::fail(format!("bare encode: {e}")))?;
        let seg_bytes = sut_encode(mnem, ops_seg).map_err(|e| {
            TestCaseError::fail(format!(
                "segmented encode must succeed (emit_segment_prefix contract): {e}"
            ))
        })?;
        let mut expect = vec![seg_prefix_byte(seg)];
        expect.extend_from_slice(&bare_bytes);
        prop_assert_eq!(
            &seg_bytes, &expect,
            "metamorphic seg||bare for {} ${} %{}:(%{}): got {:02x?} expect {:02x?}",
            mnem, imm, seg, base, &seg_bytes, &expect
        );
    }

    // P8a: negative — wrong arity
    #[test]
    fn encode_test_neg_arity(
        width in prop::sample::select(vec![1u8, 2, 4]),
        arity in 0u8..4,
        ri in 0usize..8,
    ) {
        prop_assume!(arity != 2);
        let mnem = mnemonic(width);
        let regs = gp_for_width(width);
        let r = regs[ri % regs.len()];
        let ops: Vec<Operand> = (0..arity)
            .map(|_| Operand::Register(Register::new(r)))
            .collect();
        match sut_encode(mnem, ops) {
            Err(_) => {}
            Ok(bytes) => {
                return Err(TestCaseError::fail(format!(
                    "SUT accepted arity-{arity} `{mnem}` → {bytes:02x?}; \
                     encode_test requires exactly 2 operands"
                )));
            }
        }
    }

    // P8b: negative — size-mismatched GP pairs that llvm-mc rejects
    #[test]
    fn encode_test_neg_mismatched_width(
        mode in 0u8..6,
        a_i in 0usize..8,
        b_i in 0usize..8,
    ) {
        let (mnem, src, dst) = match mode {
            0 => {
                let src = R16[a_i % R16.len()];
                let dst = GP32[b_i % GP32.len()];
                (mnemonic(4), src, dst)
            }
            1 => {
                let src = GP32[a_i % GP32.len()];
                let dst = R16[b_i % R16.len()];
                (mnemonic(4), src, dst)
            }
            2 => {
                let src = GP32[a_i % GP32.len()];
                let dst = R16[b_i % R16.len()];
                (mnemonic(2), src, dst)
            }
            3 => {
                let src = R16[a_i % R16.len()];
                let dst = R8[b_i % R8.len()];
                (mnemonic(2), src, dst)
            }
            4 => {
                let src = GP32[a_i % GP32.len()];
                let dst = R8[b_i % R8.len()];
                (mnemonic(1), src, dst)
            }
            _ => {
                let src = R8[a_i % R8.len()];
                let dst = R16[b_i % R16.len()];
                (mnemonic(1), src, dst)
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
            mnem,
            vec![
                Operand::Register(Register::new(src)),
                Operand::Register(Register::new(dst)),
            ],
        ) {
            Err(_) => {}
            Ok(bytes) => {
                return Err(TestCaseError::fail(format!(
                    "SUT accepted size-mismatched `{asm}` → {bytes:02x?}; \
                     TEST RR requires matching operand size (Intel SDM; llvm-mc rejects). \
                     reg_num aliases widths so bytes look like a valid same-width form."
                )));
            }
        }
    }

    // P8c: negative — non-GP names that alias through reg_num must Err
    #[test]
    fn encode_test_neg_non_gp(
        ni in 0usize..10,
        gi in 0usize..8,
        width in prop::sample::select(vec![1u8, 2, 4]),
        non_gp_as_src in any::<bool>(),
    ) {
        let non_gp = NON_GP[ni % NON_GP.len()];
        let gp = gp_for_width(width)[gi % 8];
        let mnem = mnemonic(width);
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
            mnem,
            vec![
                Operand::Register(Register::new(src)),
                Operand::Register(Register::new(dst)),
            ],
        ) {
            Err(_) => {}
            Ok(bytes) => {
                return Err(TestCaseError::fail(format!(
                    "SUT accepted non-GP TEST RR `{asm}` → {bytes:02x?}; \
                     TEST r/m forms are GP-only (Intel SDM). reg_num aliases xmm/mm/st to 0-7."
                )));
            }
        }
    }
}

// --- Deterministic regression placeholders (filled after triage) ---

#[test]
fn encode_test_regression_reg_mem_bare() {
    // Witness: testl %eax, (%ebx) — Intel TEST r/m,r; SUT must encode 85 03
    let mc = llvm_mc_bytes("testl %eax, (%ebx)").expect("llvm-mc");
    assert_eq!(mc, vec![0x85, 0x03]);
    let mem = mem_base("ebx", Displacement::None, None);
    let sut = sut_encode(
        "testl",
        vec![
            Operand::Register(Register::new("eax")),
            Operand::Memory(mem),
        ],
    );
    match sut {
        Ok(bytes) => assert_eq!(
            bytes, mc,
            "regression: Reg→Mem TEST must match llvm-mc (got {bytes:02x?})"
        ),
        Err(e) => panic!("regression: Reg→Mem TEST must encode, got Err({e})"),
    }
}

#[test]
fn encode_test_regression_es_segment_imm_mem() {
    // Witness: testl $5, %es:(%eax) — SUT must emit 0x26 prefix
    let mc = llvm_mc_bytes("testl $5, %es:(%eax)").expect("llvm-mc");
    assert_eq!(mc, vec![0x26, 0xf7, 0x00, 0x05, 0x00, 0x00, 0x00]);
    let mem = mem_base("eax", Displacement::None, Some("es"));
    let sut = sut_encode(
        "testl",
        vec![
            Operand::Immediate(ImmediateValue::Integer(5)),
            Operand::Memory(mem),
        ],
    );
    match sut {
        Ok(bytes) => assert_eq!(
            bytes, mc,
            "regression: segmented Imm→Mem TEST must match llvm-mc (got {bytes:02x?})"
        ),
        Err(e) => panic!("regression: segmented Imm→Mem must encode, got Err({e})"),
    }
}

#[test]
fn encode_test_regression_mismatched_width_testl_ax_ebx() {
    // Witness: testl %ax, %ebx — llvm-mc rejects; SUT must Err
    assert!(llvm_mc_bytes("testl %ax, %ebx").is_err());
    let r = sut_encode(
        "testl",
        vec![
            Operand::Register(Register::new("ax")),
            Operand::Register(Register::new("ebx")),
        ],
    );
    assert!(
        r.is_err(),
        "regression: size-mismatched TEST must Err, got {r:?}"
    );
}

#[test]
fn encode_test_regression_non_gp_xmm() {
    // Witness: testb %al, %xmm0 — llvm-mc rejects; SUT must Err
    assert!(llvm_mc_bytes("testb %al, %xmm0").is_err());
    let r = sut_encode(
        "testb",
        vec![
            Operand::Register(Register::new("al")),
            Operand::Register(Register::new("xmm0")),
        ],
    );
    assert!(
        r.is_err(),
        "regression: non-GP TEST must Err, got {r:?}"
    );
}
