// Oracle: differential — llvm-mc i686 assembler (CMOVcc 0F 40+cc /r, r16/r32)
// Evidence: gp_integer.rs:1097 encode_cmovcc; mod.rs:286-297 cmov* dispatch;
//   registers.rs:118 cc_from_mnemonic; Intel SDM Vol.2 CMOVcc — Opcode 0F 40+cc /r,
//   operands are r16/r32 and r/m16/r/m32 (dest register, src reg/mem);
//   core.rs:31 emit_segment_prefix for memory segment overrides
//   (sibling encoders call it; encode_cmovcc memory arm does not).
// Stronger considered:
//   - State machine: rejected — pure encoding, no lifecycle
//   - Algebraic round-trip: rejected — no in-tree i686 decoder for cmovcc
// Differential: candidate=encode_cmovcc (via InstructionEncoder::encode),
//   reference=llvm-mc -triple=i686 -show-encoding (LLVM 15),
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=AT&T operand bytes <-> llvm-mc encoding bytes.
// Also: algebraic.invariant (0F 40+cc /r), algebraic.metamorphic (cc aliases),
//   negative_error (arity / r8 / non-GP / width mismatch / reg→mem).

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

/// Unsuffixed / 32-bit mnemonics dispatched to encode_cmovcc (mod.rs:286-297).
const MNEMS32: &[&str] = &[
    "cmove", "cmovne", "cmovl", "cmovle", "cmovg", "cmovge", "cmovb", "cmovbe", "cmova",
    "cmovae", "cmovs", "cmovns", "cmovz", "cmovnz", "cmovp", "cmovnp", "cmovo", "cmovno",
    "cmovc", "cmovnc", "cmovel", "cmovnel", "cmovll", "cmovlel", "cmovgl", "cmovgel",
    "cmovbl", "cmovbel", "cmoval", "cmovael", "cmovsl", "cmovnsl", "cmovzl", "cmovnzl",
    "cmovpl", "cmovnpl", "cmovol", "cmovnol", "cmovcl", "cmovncl",
];

/// Explicit 16-bit `w` suffix mnemonics.
const MNEMS_W: &[&str] = &[
    "cmovew", "cmovnew", "cmovlw", "cmovlew", "cmovgw", "cmovgew", "cmovbw", "cmovbew",
    "cmovaw", "cmovaew", "cmovsw", "cmovnsw", "cmovzw", "cmovnzw", "cmovpw", "cmovnpw",
    "cmovow", "cmovnow", "cmovcw", "cmovncw",
];

/// Unsuffixed only (size inferred by AT&T/llvm-mc from register width).
const MNEMS_UNSUFF: &[&str] = &[
    "cmove", "cmovne", "cmovl", "cmovle", "cmovg", "cmovge", "cmovb", "cmovbe", "cmova",
    "cmovae", "cmovs", "cmovns", "cmovz", "cmovnz", "cmovp", "cmovnp", "cmovo", "cmovno",
    "cmovc", "cmovnc",
];

const GP32: &[&str] = &["eax", "ecx", "edx", "ebx", "esp", "ebp", "esi", "edi"];
const GP16: &[&str] = &["ax", "cx", "dx", "bx", "sp", "bp", "si", "di"];
const R8: &[&str] = &["al", "cl", "dl", "bl", "ah", "ch", "dh", "bh"];
const NON_GP: &[&str] = &[
    "xmm0", "xmm1", "xmm7", "mm0", "mm3", "st", "st(0)", "st(1)", "ymm0", "ymm7",
];
const SEG_REGS: &[&str] = &["es", "cs", "ss", "ds", "fs", "gs"];
const SCALES: &[u8] = &[1, 2, 4, 8];

const ALIAS_PAIRS: &[(&str, &str)] = &[
    ("cmove", "cmovz"),
    ("cmovne", "cmovnz"),
    ("cmovb", "cmovc"),
    ("cmovae", "cmovnc"),
];

fn cc_of(mnemonic: &str) -> u8 {
    // Strip leading "cmov" then optional trailing size suffix w/l when it is a size suffix.
    let rest = &mnemonic[4..];
    let cc_str = if rest.ends_with('w') && rest != "w" {
        let cand = &rest[..rest.len() - 1];
        if matches_cc(cand) {
            cand
        } else {
            rest
        }
    } else if rest.ends_with('l') && rest != "l" {
        let cand = &rest[..rest.len() - 1];
        if matches_cc(cand) {
            cand
        } else {
            rest
        }
    } else {
        rest
    };
    match cc_str {
        "o" => 0,
        "no" => 1,
        "b" | "c" | "nae" => 2,
        "ae" | "nb" | "nc" => 3,
        "e" | "z" => 4,
        "ne" | "nz" => 5,
        "be" | "na" => 6,
        "a" | "nbe" => 7,
        "s" => 8,
        "ns" => 9,
        "p" | "pe" => 10,
        "np" | "po" => 11,
        "l" | "nge" => 12,
        "ge" | "nl" => 13,
        "le" | "ng" => 14,
        "g" | "nle" => 15,
        other => panic!("unknown cc suffix {other} from {mnemonic}"),
    }
}

fn matches_cc(s: &str) -> bool {
    matches!(
        s,
        "o" | "no"
            | "b"
            | "c"
            | "nae"
            | "ae"
            | "nb"
            | "nc"
            | "e"
            | "z"
            | "ne"
            | "nz"
            | "be"
            | "na"
            | "a"
            | "nbe"
            | "s"
            | "ns"
            | "p"
            | "pe"
            | "np"
            | "po"
            | "l"
            | "nge"
            | "ge"
            | "nl"
            | "le"
            | "ng"
            | "g"
            | "nle"
    )
}

fn reg_num(name: &str) -> u8 {
    match name {
        "al" | "ax" | "eax" | "xmm0" | "mm0" | "st" | "st(0)" | "ymm0" => 0,
        "cl" | "cx" | "ecx" | "xmm1" | "mm1" | "st(1)" | "ymm1" => 1,
        "dl" | "dx" | "edx" | "xmm2" | "mm2" | "st(2)" | "ymm2" => 2,
        "bl" | "bx" | "ebx" | "xmm3" | "mm3" | "st(3)" | "ymm3" => 3,
        "ah" | "sp" | "esp" | "xmm4" | "mm4" | "st(4)" | "ymm4" => 4,
        "ch" | "bp" | "ebp" | "xmm5" | "mm5" | "st(5)" | "ymm5" => 5,
        "dh" | "si" | "esi" | "xmm6" | "mm6" | "st(6)" | "ymm6" => 6,
        "bh" | "di" | "edi" | "xmm7" | "mm7" | "st(7)" | "ymm7" => 7,
        _ => panic!("bad reg {name}"),
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
        _ => {}
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

// --- KAT gate (reference/differential prerequisite) ---

#[test]
fn encode_cmovcc_kat_llvm_mc_cmove_eax_ebx() {
    let mc = llvm_mc_bytes("cmove %eax, %ebx").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x0f, 0x44, 0xd8], "llvm-mc KAT mapping broken");
    let sut = sut_encode(
        "cmove",
        vec![
            Operand::Register(Register::new("eax")),
            Operand::Register(Register::new("ebx")),
        ],
    )
    .expect("SUT KAT");
    assert_eq!(sut, mc, "SUT KAT cmove %eax, %ebx");
}

#[test]
fn encode_cmovcc_kat_llvm_mc_cmovz_eax_ebx() {
    let mc = llvm_mc_bytes("cmovz %eax, %ebx").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x0f, 0x44, 0xd8]);
    let sut = sut_encode(
        "cmovz",
        vec![
            Operand::Register(Register::new("eax")),
            Operand::Register(Register::new("ebx")),
        ],
    )
    .expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_cmovcc_kat_llvm_mc_cmovew_ax_bx() {
    let mc = llvm_mc_bytes("cmovew %ax, %bx").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x66, 0x0f, 0x44, 0xd8]);
    let sut = sut_encode(
        "cmovew",
        vec![
            Operand::Register(Register::new("ax")),
            Operand::Register(Register::new("bx")),
        ],
    )
    .expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_cmovcc_kat_llvm_mc_cmovll_ecx_edx() {
    let mc = llvm_mc_bytes("cmovll %ecx, %edx").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x0f, 0x4c, 0xd1]);
    let sut = sut_encode(
        "cmovll",
        vec![
            Operand::Register(Register::new("ecx")),
            Operand::Register(Register::new("edx")),
        ],
    )
    .expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_cmovcc_kat_llvm_mc_mem() {
    let mc = llvm_mc_bytes("cmove (%eax), %ebx").expect("llvm-mc KAT mem");
    assert_eq!(mc, vec![0x0f, 0x44, 0x18]);
    let sut = sut_encode(
        "cmove",
        vec![
            Operand::Memory(mem_base("eax", Displacement::None, None)),
            Operand::Register(Register::new("ebx")),
        ],
    )
    .expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_cmovcc_kat_llvm_mc_esp() {
    let mc = llvm_mc_bytes("cmove (%esp), %eax").expect("llvm-mc KAT esp");
    assert_eq!(mc, vec![0x0f, 0x44, 0x04, 0x24]);
    let sut = sut_encode(
        "cmove",
        vec![
            Operand::Memory(mem_base("esp", Displacement::None, None)),
            Operand::Register(Register::new("eax")),
        ],
    )
    .expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_cmovcc_kat_llvm_mc_segment_es() {
    let mc = llvm_mc_bytes("cmove %es:(%eax), %ebx").expect("llvm-mc KAT es");
    assert_eq!(mc, vec![0x26, 0x0f, 0x44, 0x18]);
    let sut = sut_encode(
        "cmove",
        vec![
            Operand::Memory(mem_base("eax", Displacement::None, Some("es"))),
            Operand::Register(Register::new("ebx")),
        ],
    );
    match sut {
        Ok(bytes) => assert_eq!(bytes, mc, "SUT must emit ES override 0x26 before 0F 44"),
        Err(e) => panic!("SUT erred on valid ES mem: {e}"),
    }
}

#[test]
fn encode_cmovcc_kat_llvm_mc_segment_fs_sib() {
    let mc = llvm_mc_bytes("cmove %fs:8(%esi,%edi,4), %ecx").expect("llvm-mc KAT fs");
    assert_eq!(mc, vec![0x64, 0x0f, 0x44, 0x4c, 0xbe, 0x08]);
    let sut = sut_encode(
        "cmove",
        vec![
            Operand::Memory(mem_base_index(
                Some("esi"),
                "edi",
                4,
                Displacement::Integer(8),
                Some("fs"),
            )),
            Operand::Register(Register::new("ecx")),
        ],
    );
    match sut {
        Ok(bytes) => assert_eq!(bytes, mc, "SUT must emit FS override 0x64 before 0F 44"),
        Err(e) => panic!("SUT erred on valid FS SIB mem: {e}"),
    }
}

#[test]
fn encode_cmovcc_kat_llvm_mc_unsuffixed_r16() {
    let mc = llvm_mc_bytes("cmove %ax, %bx").expect("llvm-mc KAT r16");
    assert_eq!(mc, vec![0x66, 0x0f, 0x44, 0xd8]);
    let sut = sut_encode(
        "cmove",
        vec![
            Operand::Register(Register::new("ax")),
            Operand::Register(Register::new("bx")),
        ],
    );
    match sut {
        Ok(bytes) => assert_eq!(
            bytes, mc,
            "unsuffixed cmove %ax,%bx must emit 0x66 like llvm-mc"
        ),
        Err(e) => panic!("SUT erred on valid r16 pair: {e}"),
    }
}

proptest! {
    #![proptest_config(cfg())]

    // Oracle: differential — r32 register-register vs llvm-mc
    #[test]
    fn encode_cmovcc_diff_rr32_llvm_mc(
        m in prop::sample::select(MNEMS32),
        src in prop::sample::select(GP32),
        dst in prop::sample::select(GP32),
    ) {
        let asm = format!("{m} %{src}, %{dst}");
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let sut = sut_encode(
            m,
            vec![
                Operand::Register(Register::new(src)),
                Operand::Register(Register::new(dst)),
            ],
        )
        .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(sut, mc, "diff vs llvm-mc for {}", asm);
    }

    // Oracle: differential — explicit w-suffix r16 RR vs llvm-mc
    #[test]
    fn encode_cmovcc_diff_rr16_w_llvm_mc(
        m in prop::sample::select(MNEMS_W),
        src in prop::sample::select(GP16),
        dst in prop::sample::select(GP16),
    ) {
        let asm = format!("{m} %{src}, %{dst}");
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let sut = sut_encode(
            m,
            vec![
                Operand::Register(Register::new(src)),
                Operand::Register(Register::new(dst)),
            ],
        )
        .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(sut, mc, "diff vs llvm-mc for {}", asm);
    }

    // Oracle: differential — base+disp memory (no segment), r32 dest
    #[test]
    fn encode_cmovcc_diff_mem_base_disp_llvm_mc(
        m in prop::sample::select(MNEMS32),
        base in prop::sample::select(GP32),
        dst in prop::sample::select(GP32),
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
        let asm = format!("{m} {}, %{dst}", att_mem(&mem));
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let sut = sut_encode(
            m,
            vec![
                Operand::Memory(mem),
                Operand::Register(Register::new(dst)),
            ],
        )
        .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(sut, mc, "diff vs llvm-mc for `{}`", asm);
    }

    // Oracle: differential — SIB (index ≠ esp), r32 dest
    #[test]
    fn encode_cmovcc_diff_mem_sib_llvm_mc(
        m in prop::sample::select(&["cmove", "cmovne", "cmovl", "cmovg", "cmovo"][..]),
        base in prop::option::of(prop::sample::select(GP32)),
        index in prop::sample::select(
            GP32.iter().copied().filter(|r| *r != "esp").collect::<Vec<_>>()
        ),
        scale in prop::sample::select(SCALES),
        dst in prop::sample::select(GP32),
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
        let asm = format!("{m} {}, %{dst}", att_mem(&mem));
        let mc = match llvm_mc_bytes(&asm) {
            Ok(b) => b,
            Err(_) => return Ok(()),
        };
        let sut = sut_encode(
            m,
            vec![
                Operand::Memory(mem),
                Operand::Register(Register::new(dst)),
            ],
        )
        .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(sut, mc, "SIB diff vs llvm-mc for `{}`", asm);
    }

    // Oracle: differential — absolute disp32
    #[test]
    fn encode_cmovcc_diff_abs_disp32(
        m in prop::sample::select(&["cmove", "cmovne", "cmovo", "cmovb"][..]),
        dst in prop::sample::select(GP32),
        disp in prop_oneof![
            Just(0i64),
            Just(1i64),
            Just(0x1234_5678i64),
            Just(-1i64),
            (-0x8000_0000i64..=0x7fff_ffffi64),
        ],
    ) {
        let mem = mem_abs(disp, None);
        let asm = format!("{m} {}, %{dst}", att_mem(&mem));
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let sut = sut_encode(
            m,
            vec![
                Operand::Memory(mem),
                Operand::Register(Register::new(dst)),
            ],
        )
        .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(sut, mc, "abs diff for `{}`", asm);
    }

    // Oracle: differential — segment override prefixes (expect SUT bug: missing prefix)
    #[test]
    fn encode_cmovcc_diff_segment_llvm_mc(
        m in prop::sample::select(MNEMS_UNSUFF),
        seg in prop::sample::select(SEG_REGS),
        base in prop::sample::select(GP32),
        dst in prop::sample::select(GP32),
        disp in prop_oneof![Just(0i64), Just(8i64), Just(-4i64), Just(127i64), Just(-128i64)],
    ) {
        let mem = mem_base(base, Displacement::Integer(disp), Some(seg));
        let asm = format!("{m} {}, %{dst}", att_mem(&mem));
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let sut = sut_encode(
            m,
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

    // Oracle: differential — segment + SIB
    #[test]
    fn encode_cmovcc_diff_segment_sib(
        m in prop::sample::select(&["cmove", "cmovne", "cmovl", "cmova"][..]),
        seg in prop::sample::select(SEG_REGS),
        base in prop::sample::select(GP32),
        index in prop::sample::select(
            GP32.iter().copied().filter(|r| *r != "esp").collect::<Vec<_>>()
        ),
        scale in prop::sample::select(SCALES),
        dst in prop::sample::select(GP32),
        disp in prop_oneof![Just(0i64), Just(16i64), Just(-8i64)],
    ) {
        let mem = mem_base_index(
            Some(base),
            index,
            scale,
            Displacement::Integer(disp),
            Some(seg),
        );
        let asm = format!("{m} {}, %{dst}", att_mem(&mem));
        let mc = match llvm_mc_bytes(&asm) {
            Ok(b) => b,
            Err(_) => return Ok(()),
        };
        let sut = sut_encode(
            m,
            vec![
                Operand::Memory(mem),
                Operand::Register(Register::new(dst)),
            ],
        )
        .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(
            sut.clone(), mc.clone(),
            "seg+SIB diff for `{}`: SUT={:02x?} llvm-mc={:02x?}",
            asm, sut, mc
        );
    }

    // Oracle: differential — explicit w-suffix mem→r16 (hits is_16bit on memory arm)
    #[test]
    fn encode_cmovcc_diff_mem_w16_llvm_mc(
        m in prop::sample::select(MNEMS_W),
        base in prop::sample::select(GP32),
        dst in prop::sample::select(GP16),
        disp in prop_oneof![Just(0i64), Just(8i64), Just(-1i64), Just(127i64), Just(-128i64)],
    ) {
        let mem = mem_base(base, Displacement::Integer(disp), None);
        let asm = format!("{m} {}, %{dst}", att_mem(&mem));
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let sut = sut_encode(
            m,
            vec![
                Operand::Memory(mem),
                Operand::Register(Register::new(dst)),
            ],
        )
        .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(sut, mc, "mem w16 diff for `{}`", asm);
    }

    // Oracle: differential — unsuffixed mnemonic + r16 must match llvm-mc (0x66)
    #[test]
    fn encode_cmovcc_diff_unsuffixed_r16_size(
        m in prop::sample::select(MNEMS_UNSUFF),
        src in prop::sample::select(GP16),
        dst in prop::sample::select(GP16),
    ) {
        let asm = format!("{m} %{src}, %{dst}");
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let sut = sut_encode(
            m,
            vec![
                Operand::Register(Register::new(src)),
                Operand::Register(Register::new(dst)),
            ],
        )
        .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(
            sut.clone(), mc.clone(),
            "unsuffixed r16 size diff for `{}`: SUT={:02x?} llvm-mc={:02x?}",
            asm, sut, mc
        );
    }

    // Oracle: algebraic.invariant — 0F 40+cc /r on r32 RR
    #[test]
    fn encode_cmovcc_invariant_opcode_rr32(
        m in prop::sample::select(MNEMS_UNSUFF),
        src in prop::sample::select(GP32),
        dst in prop::sample::select(GP32),
    ) {
        let bytes = sut_encode(
            m,
            vec![
                Operand::Register(Register::new(src)),
                Operand::Register(Register::new(dst)),
            ],
        )
        .expect("valid r32 RR must encode");
        prop_assert_eq!(bytes.len(), 3, "r32 cmov is 3 bytes, got {:02x?}", bytes);
        prop_assert_eq!(bytes[0], 0x0Fu8);
        prop_assert_eq!(bytes[1], 0x40 + cc_of(m));
        let modrm = bytes[2];
        prop_assert_eq!((modrm >> 6) & 3, 3u8, "mod must be 11b");
        prop_assert_eq!((modrm >> 3) & 7, reg_num(dst), "ModRM.reg must be dst");
        prop_assert_eq!(modrm & 7, reg_num(src), "ModRM.rm must be src");
    }

    // Oracle: algebraic.metamorphic — condition-code aliases encode identically
    #[test]
    fn encode_cmovcc_meta_alias_pairs(
        pair in prop::sample::select(ALIAS_PAIRS),
        kind in 0u8..4,
        src in prop::sample::select(GP32),
        dst in prop::sample::select(GP32),
        base in prop::sample::select(GP32),
        disp in prop_oneof![Just(0i64), Just(7i64), Just(-3i64), (-200i64..=300i64)],
        index in prop::sample::select(
            GP32.iter().copied().filter(|x| *x != "esp").collect::<Vec<_>>()
        ),
        scale in prop::sample::select(SCALES),
    ) {
        let (m1, m2) = pair;
        let ops = match kind {
            0 => vec![
                Operand::Register(Register::new(src)),
                Operand::Register(Register::new(dst)),
            ],
            1 => vec![
                Operand::Memory(mem_base(base, Displacement::Integer(disp), None)),
                Operand::Register(Register::new(dst)),
            ],
            2 => vec![
                Operand::Memory(mem_base_index(
                    Some(base),
                    index,
                    scale,
                    Displacement::Integer(disp),
                    None,
                )),
                Operand::Register(Register::new(dst)),
            ],
            _ => vec![
                Operand::Memory(mem_abs(disp, None)),
                Operand::Register(Register::new(dst)),
            ],
        };
        let a = sut_encode(m1, ops.clone()).expect("alias m1");
        let b = sut_encode(m2, ops).expect("alias m2");
        prop_assert_eq!(a, b, "{} ≡ {}", m1, m2);
    }

    // Oracle: negative_error — arity ≠ 2
    #[test]
    fn encode_cmovcc_neg_arity(
        n in 0usize..5,
        r in prop::sample::select(GP32),
        m in prop::sample::select(MNEMS_UNSUFF),
    ) {
        prop_assume!(n != 2);
        let ops: Vec<Operand> = (0..n)
            .map(|_| Operand::Register(Register::new(r)))
            .collect();
        let err = sut_encode(m, ops).expect_err("arity ≠ 2 must Err");
        prop_assert!(
            err.contains("cmovcc requires 2 operands") || err.contains("operand"),
            "unexpected err text: {err}"
        );
    }

    // Oracle: negative_error — imm / label / reg→mem
    #[test]
    fn encode_cmovcc_neg_bad_operand(
        kind in 0u8..3,
        m in prop::sample::select(MNEMS_UNSUFF),
        imm in any::<i32>(),
        r in prop::sample::select(GP32),
    ) {
        let ops = match kind {
            0 => vec![
                Operand::Immediate(ImmediateValue::Integer(imm as i64)),
                Operand::Register(Register::new(r)),
            ],
            1 => vec![
                Operand::Label("sym".into()),
                Operand::Register(Register::new(r)),
            ],
            _ => vec![
                Operand::Register(Register::new(r)),
                Operand::Memory(mem_base("eax", Displacement::None, None)),
            ],
        };
        let err = sut_encode(m, ops).expect_err("non r/m→reg must Err");
        prop_assert!(
            err.contains("unsupported cmov") || err.contains("cmov") || err.contains("operand"),
            "unexpected err: {err}"
        );
    }

    // Oracle: negative_error — r8 must be rejected (CMOVcc is r16/r32 only)
    #[test]
    fn encode_cmovcc_neg_r8(
        src in prop::sample::select(R8),
        dst in prop::sample::select(R8),
        m in prop::sample::select(MNEMS_UNSUFF),
    ) {
        let asm = format!("{m} %{src}, %{dst}");
        let mc = llvm_mc_bytes(&asm);
        prop_assert!(mc.is_err(), "expected llvm-mc to reject `{asm}`");
        match sut_encode(
            m,
            vec![
                Operand::Register(Register::new(src)),
                Operand::Register(Register::new(dst)),
            ],
        ) {
            Err(_) => {}
            Ok(bytes) => {
                return Err(TestCaseError::fail(format!(
                    "SUT accepted invalid-width r8 `{asm}` → {bytes:02x?}; \
                     CMOVcc is r16/r32 only (Intel SDM; llvm-mc rejects)"
                )));
            }
        }
    }

    // Oracle: negative_error — mismatched widths must be rejected
    #[test]
    fn encode_cmovcc_neg_width_mismatch(
        src16 in prop::sample::select(GP16),
        dst32 in prop::sample::select(GP32),
        m in prop::sample::select(MNEMS_UNSUFF),
        swap in any::<bool>(),
    ) {
        let (src, dst) = if swap {
            (dst32, src16)
        } else {
            (src16, dst32)
        };
        let asm = format!("{m} %{src}, %{dst}");
        let mc = llvm_mc_bytes(&asm);
        prop_assert!(mc.is_err(), "expected llvm-mc to reject `{asm}`");
        match sut_encode(
            m,
            vec![
                Operand::Register(Register::new(src)),
                Operand::Register(Register::new(dst)),
            ],
        ) {
            Err(_) => {}
            Ok(bytes) => {
                return Err(TestCaseError::fail(format!(
                    "SUT accepted mismatched-width `{asm}` → {bytes:02x?}"
                )));
            }
        }
    }

    // Oracle: negative_error — non-GP names aliased by reg_num
    #[test]
    fn encode_cmovcc_neg_non_gp(
        r in prop::sample::select(NON_GP),
        dst in prop::sample::select(GP32),
        m in prop::sample::select(&["cmove", "cmovne", "cmovo"][..]),
        as_src in any::<bool>(),
    ) {
        let (ops, asm) = if as_src {
            (
                vec![
                    Operand::Register(Register::new(r)),
                    Operand::Register(Register::new(dst)),
                ],
                format!("{m} %{r}, %{dst}"),
            )
        } else {
            (
                vec![
                    Operand::Register(Register::new(dst)),
                    Operand::Register(Register::new(r)),
                ],
                format!("{m} %{dst}, %{r}"),
            )
        };
        let mc = llvm_mc_bytes(&asm);
        prop_assert!(mc.is_err(), "expected llvm-mc to reject `{asm}`");
        match sut_encode(m, ops) {
            Err(_) => {}
            Ok(bytes) => {
                return Err(TestCaseError::fail(format!(
                    "SUT accepted non-GP `{asm}` → {bytes:02x?}; CMOVcc is GP only"
                )));
            }
        }
    }
}

/// Deterministic regression: missing ES segment prefix on cmove.
#[test]
fn test_encode_cmovcc_regression_missing_es_prefix() {
    let mem = mem_base("eax", Displacement::None, Some("es"));
    let sut = sut_encode(
        "cmove",
        vec![
            Operand::Memory(mem),
            Operand::Register(Register::new("ebx")),
        ],
    )
    .expect("encode");
    let mc = vec![0x26u8, 0x0f, 0x44, 0x18];
    assert_eq!(
        sut, mc,
        "cmove %es:(%eax), %ebx must be [26, 0f, 44, 18], got {sut:02x?}"
    );
}

/// Deterministic regression: missing FS segment prefix on cmove SIB.
#[test]
fn test_encode_cmovcc_regression_missing_fs_prefix() {
    let mem = mem_base_index(
        Some("esi"),
        "edi",
        4,
        Displacement::Integer(8),
        Some("fs"),
    );
    let sut = sut_encode(
        "cmove",
        vec![
            Operand::Memory(mem),
            Operand::Register(Register::new("ecx")),
        ],
    )
    .expect("encode");
    let mc = vec![0x64u8, 0x0f, 0x44, 0x4c, 0xbe, 0x08];
    assert_eq!(
        sut, mc,
        "cmove %fs:8(%esi,%edi,4), %ecx must be [64, 0f, 44, 4c, be, 08], got {sut:02x?}"
    );
}

/// Deterministic regression: unsuffixed r16 must emit 0x66.
#[test]
fn test_encode_cmovcc_regression_unsuffixed_r16_prefix() {
    let sut = sut_encode(
        "cmove",
        vec![
            Operand::Register(Register::new("ax")),
            Operand::Register(Register::new("bx")),
        ],
    )
    .expect("encode");
    let mc = vec![0x66u8, 0x0f, 0x44, 0xd8];
    assert_eq!(
        sut, mc,
        "cmove %ax, %bx must be [66, 0f, 44, d8], got {sut:02x?}"
    );
}

/// Deterministic regression: r8 must be rejected (CMOVcc is r16/r32).
#[test]
fn test_encode_cmovcc_regression_rejects_al_bl() {
    let result = sut_encode(
        "cmove",
        vec![
            Operand::Register(Register::new("al")),
            Operand::Register(Register::new("bl")),
        ],
    );
    assert!(
        result.is_err(),
        "cmove %al, %bl must Err (r8 invalid); got Ok({result:?})"
    );
}

/// Deterministic regression: mismatched width must be rejected.
#[test]
fn test_encode_cmovcc_regression_rejects_ax_ebx() {
    let result = sut_encode(
        "cmove",
        vec![
            Operand::Register(Register::new("ax")),
            Operand::Register(Register::new("ebx")),
        ],
    );
    assert!(
        result.is_err(),
        "cmove %ax, %ebx must Err (width mismatch); got Ok({result:?})"
    );
}

/// Deterministic regression: non-GP must be rejected.
#[test]
fn test_encode_cmovcc_regression_rejects_xmm0() {
    let result = sut_encode(
        "cmove",
        vec![
            Operand::Register(Register::new("xmm0")),
            Operand::Register(Register::new("eax")),
        ],
    );
    assert!(
        result.is_err(),
        "cmove %xmm0, %eax must Err (non-GP); got Ok({result:?})"
    );
}
