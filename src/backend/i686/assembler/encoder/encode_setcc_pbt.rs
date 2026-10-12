// Oracle: differential — llvm-mc i686 assembler (SETcc 0F 90+cc /0, r/m8)
// Evidence: gp_integer.rs:1075 encode_setcc; mod.rs:279-281 set* dispatch;
//   registers.rs:118 cc_from_mnemonic; Intel SDM Vol.2 SETcc — Opcode 0F 90+cc /0,
//   operand is r/m8 only; core.rs:31 emit_segment_prefix for memory segment overrides
//   (sibling encoders call it; encode_setcc memory arm does not).
// Stronger considered:
//   - State machine: rejected — pure encoding, no lifecycle
//   - Algebraic round-trip: rejected — no in-tree i686 decoder for setcc
// Differential: candidate=encode_setcc (via InstructionEncoder::encode),
//   reference=llvm-mc -triple=i686 -show-encoding (LLVM 15),
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=AT&T operand bytes <-> llvm-mc encoding bytes.
// Also: algebraic.invariant (0F 90+cc /0), algebraic.metamorphic (cc aliases),
//   negative_error (arity / non-r/m / wrong width).

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

/// Mnemonics dispatched to encode_setcc (mod.rs:279-281).
const MNEMS: &[&str] = &[
    "sete", "setz", "setne", "setnz", "setl", "setle", "setg", "setge", "setb", "setc",
    "setbe", "seta", "setae", "setnc", "setnp", "setp", "sets", "setns", "seto", "setno",
];

const R8: &[&str] = &["al", "cl", "dl", "bl", "ah", "ch", "dh", "bh"];
const R16: &[&str] = &["ax", "cx", "dx", "bx", "sp", "bp", "si", "di"];
const GP32: &[&str] = &["eax", "ecx", "edx", "ebx", "esp", "ebp", "esi", "edi"];
const BAD_WIDTH: &[&str] = &[
    "ax", "cx", "dx", "bx", "sp", "bp", "si", "di", "eax", "ecx", "edx", "ebx", "esp", "ebp",
    "esi", "edi",
];
const NON_GP_ALIASED: &[&str] = &[
    "xmm0", "xmm1", "xmm7", "mm0", "mm3", "st", "st(0)", "st(1)", "ymm0", "ymm7",
];
const SEG_REGS: &[&str] = &["es", "cs", "ss", "ds", "fs", "gs"];
const SCALES: &[u8] = &[1, 2, 4, 8];

/// Alias pairs that share a condition-code encoding (registers.rs cc_from_mnemonic).
const ALIAS_PAIRS: &[(&str, &str)] = &[
    ("sete", "setz"),
    ("setne", "setnz"),
    ("setb", "setc"),
    ("setae", "setnc"),
];

fn cc_of(mnemonic: &str) -> u8 {
    match &mnemonic[3..] {
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
fn encode_setcc_kat_llvm_mc_sete_al() {
    let mc = llvm_mc_bytes("sete %al").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x0f, 0x94, 0xc0], "llvm-mc KAT mapping broken");
    let sut = sut_encode("sete", vec![Operand::Register(Register::new("al"))]).expect("SUT KAT");
    assert_eq!(sut, mc, "SUT KAT sete %al");
}

#[test]
fn encode_setcc_kat_llvm_mc_setz_al() {
    let mc = llvm_mc_bytes("setz %al").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x0f, 0x94, 0xc0]);
    let sut = sut_encode("setz", vec![Operand::Register(Register::new("al"))]).expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_setcc_kat_llvm_mc_seto_ah() {
    let mc = llvm_mc_bytes("seto %ah").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x0f, 0x90, 0xc4]);
    let sut = sut_encode("seto", vec![Operand::Register(Register::new("ah"))]).expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_setcc_kat_llvm_mc_setne_mem() {
    let mc = llvm_mc_bytes("setne (%eax)").expect("llvm-mc KAT mem");
    assert_eq!(mc, vec![0x0f, 0x95, 0x00]);
    let sut = sut_encode(
        "setne",
        vec![Operand::Memory(mem_base("eax", Displacement::None, None))],
    )
    .expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_setcc_kat_llvm_mc_sete_esp() {
    let mc = llvm_mc_bytes("sete (%esp)").expect("llvm-mc KAT esp");
    assert_eq!(mc, vec![0x0f, 0x94, 0x04, 0x24]);
    let sut = sut_encode(
        "sete",
        vec![Operand::Memory(mem_base("esp", Displacement::None, None))],
    )
    .expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_setcc_kat_llvm_mc_segment_es() {
    let mc = llvm_mc_bytes("sete %es:(%eax)").expect("llvm-mc KAT es");
    assert_eq!(mc, vec![0x26, 0x0f, 0x94, 0x00]);
    let sut = sut_encode(
        "sete",
        vec![Operand::Memory(mem_base(
            "eax",
            Displacement::None,
            Some("es"),
        ))],
    );
    match sut {
        Ok(bytes) => assert_eq!(bytes, mc, "SUT must emit ES override 0x26 before 0F 94"),
        Err(e) => panic!("SUT erred on valid ES mem: {e}"),
    }
}

#[test]
fn encode_setcc_kat_llvm_mc_segment_fs() {
    let mc = llvm_mc_bytes("sete %fs:8(%esi,%edi,4)").expect("llvm-mc KAT fs");
    assert_eq!(mc, vec![0x64, 0x0f, 0x94, 0x44, 0xbe, 0x08]);
    let sut = sut_encode(
        "sete",
        vec![Operand::Memory(mem_base_index(
            Some("esi"),
            "edi",
            4,
            Displacement::Integer(8),
            Some("fs"),
        ))],
    );
    match sut {
        Ok(bytes) => assert_eq!(bytes, mc, "SUT must emit FS override 0x64 before 0F 94"),
        Err(e) => panic!("SUT erred on valid FS SIB mem: {e}"),
    }
}

proptest! {
    #![proptest_config(cfg())]

    // Oracle: differential — r8 register form vs llvm-mc
    #[test]
    fn encode_setcc_diff_r8_llvm_mc(
        m in prop::sample::select(MNEMS),
        r in prop::sample::select(R8),
    ) {
        let asm = format!("{m} %{r}");
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let sut = sut_encode(m, vec![Operand::Register(Register::new(r))])
            .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(sut, mc, "diff vs llvm-mc for {}", asm);
    }

    // Oracle: differential — base+disp memory (no segment)
    #[test]
    fn encode_setcc_diff_mem_base_disp_llvm_mc(
        m in prop::sample::select(MNEMS),
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
    ) {
        let mem = mem_base(base, Displacement::Integer(disp), None);
        let asm = format!("{m} {}", att_mem(&mem));
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let sut = sut_encode(m, vec![Operand::Memory(mem)])
            .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(sut, mc, "diff vs llvm-mc for `{}`", asm);
    }

    // Oracle: differential — SIB (index ≠ esp)
    #[test]
    fn encode_setcc_diff_mem_sib_llvm_mc(
        m in prop::sample::select(MNEMS),
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
    ) {
        let mem = mem_base_index(
            base,
            index,
            scale,
            Displacement::Integer(disp),
            None,
        );
        let asm = format!("{m} {}", att_mem(&mem));
        let mc = match llvm_mc_bytes(&asm) {
            Ok(b) => b,
            Err(_) => return Ok(()),
        };
        let sut = sut_encode(m, vec![Operand::Memory(mem)])
            .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(sut, mc, "SIB diff vs llvm-mc for `{}`", asm);
    }

    // Oracle: differential — absolute disp32
    #[test]
    fn encode_setcc_diff_abs_disp32(
        m in prop::sample::select(&["sete", "setne", "seto", "setb"][..]),
        disp in prop_oneof![
            Just(0i64),
            Just(1i64),
            Just(0x1234_5678i64),
            Just(-1i64),
            (-0x8000_0000i64..=0x7fff_ffffi64),
        ],
    ) {
        let mem = mem_abs(disp, None);
        let asm = format!("{m} {}", att_mem(&mem));
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let sut = sut_encode(m, vec![Operand::Memory(mem)])
            .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(sut, mc, "abs diff for `{}`", asm);
    }

    // Oracle: differential — segment override prefixes (expect SUT bug: missing prefix)
    #[test]
    fn encode_setcc_diff_segment_llvm_mc(
        m in prop::sample::select(MNEMS),
        seg in prop::sample::select(SEG_REGS),
        base in prop::sample::select(GP32),
        disp in prop_oneof![Just(0i64), Just(8i64), Just(-4i64), Just(127i64), Just(-128i64)],
    ) {
        let mem = mem_base(base, Displacement::Integer(disp), Some(seg));
        let asm = format!("{m} {}", att_mem(&mem));
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let sut = sut_encode(m, vec![Operand::Memory(mem)])
            .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(
            sut.clone(), mc.clone(),
            "segment prefix diff for `{}`: SUT={:02x?} llvm-mc={:02x?}",
            asm, sut, mc
        );
    }

    // Oracle: differential — segment + SIB
    #[test]
    fn encode_setcc_diff_segment_sib(
        m in prop::sample::select(&["sete", "setne", "setl", "seta"][..]),
        seg in prop::sample::select(SEG_REGS),
        base in prop::sample::select(GP32),
        index in prop::sample::select(
            GP32.iter().copied().filter(|r| *r != "esp").collect::<Vec<_>>()
        ),
        scale in prop::sample::select(SCALES),
        disp in prop_oneof![Just(0i64), Just(16i64), Just(-8i64)],
    ) {
        let mem = mem_base_index(
            Some(base),
            index,
            scale,
            Displacement::Integer(disp),
            Some(seg),
        );
        let asm = format!("{m} {}", att_mem(&mem));
        let mc = match llvm_mc_bytes(&asm) {
            Ok(b) => b,
            Err(_) => return Ok(()),
        };
        let sut = sut_encode(m, vec![Operand::Memory(mem)])
            .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(
            sut.clone(), mc.clone(),
            "seg+SIB diff for `{}`: SUT={:02x?} llvm-mc={:02x?}",
            asm, sut, mc
        );
    }

    // Oracle: algebraic.invariant — 0F 90+cc /0 on r8
    #[test]
    fn encode_setcc_invariant_opcode_cc(
        m in prop::sample::select(MNEMS),
        r in prop::sample::select(R8),
    ) {
        let bytes = sut_encode(m, vec![Operand::Register(Register::new(r))])
            .expect("valid r8 must encode");
        prop_assert_eq!(bytes.len(), 3, "r8 setcc is 3 bytes, got {:02x?}", bytes);
        prop_assert_eq!(bytes[0], 0x0Fu8);
        prop_assert_eq!(bytes[1], 0x90 + cc_of(m));
        prop_assert_eq!((bytes[2] >> 3) & 7, 0u8, "ModRM.reg must be /0");
        prop_assert_eq!(bytes[2] & 7, reg_num(r), "ModRM.rm must be reg number");
        prop_assert_eq!((bytes[2] >> 6) & 3, 3u8, "mod must be 11b");
    }

    // Oracle: algebraic.metamorphic — condition-code aliases encode identically
    #[test]
    fn encode_setcc_meta_alias_pairs(
        pair in prop::sample::select(ALIAS_PAIRS),
        kind in 0u8..4,
        r in prop::sample::select(R8),
        base in prop::sample::select(GP32),
        disp in prop_oneof![Just(0i64), Just(7i64), Just(-3i64), (-200i64..=300i64)],
        index in prop::sample::select(
            GP32.iter().copied().filter(|x| *x != "esp").collect::<Vec<_>>()
        ),
        scale in prop::sample::select(SCALES),
    ) {
        let (m1, m2) = pair;
        let ops = match kind {
            0 => vec![Operand::Register(Register::new(r))],
            1 => vec![Operand::Memory(mem_base(base, Displacement::Integer(disp), None))],
            2 => vec![Operand::Memory(mem_base_index(
                Some(base),
                index,
                scale,
                Displacement::Integer(disp),
                None,
            ))],
            _ => vec![Operand::Memory(mem_abs(disp, None))],
        };
        let a = sut_encode(m1, ops.clone()).expect("alias m1");
        let b = sut_encode(m2, ops).expect("alias m2");
        prop_assert_eq!(a, b, "{} ≡ {}", m1, m2);
    }

    // Oracle: negative_error — arity ≠ 1
    #[test]
    fn encode_setcc_neg_arity(
        n in 0usize..5,
        r in prop::sample::select(R8),
        m in prop::sample::select(MNEMS),
    ) {
        prop_assume!(n != 1);
        let ops: Vec<Operand> = (0..n)
            .map(|_| Operand::Register(Register::new(r)))
            .collect();
        let err = sut_encode(m, ops).expect_err("arity ≠ 1 must Err");
        prop_assert!(
            err.contains("setcc requires 1 operand") || err.contains("operand"),
            "unexpected err text: {err}"
        );
    }

    // Oracle: negative_error — imm / label
    #[test]
    fn encode_setcc_neg_bad_operand(
        kind in 0u8..2,
        m in prop::sample::select(MNEMS),
        imm in any::<i32>(),
    ) {
        let op = match kind {
            0 => Operand::Immediate(ImmediateValue::Integer(imm as i64)),
            _ => Operand::Label("sym".into()),
        };
        let err = sut_encode(m, vec![op]).expect_err("non r/m must Err");
        prop_assert!(
            err.contains("register or memory") || err.contains("setcc"),
            "unexpected err: {err}"
        );
    }

    // Oracle: negative_error — r16/r32 must be rejected (Intel SETcc is r/m8)
    #[test]
    fn encode_setcc_neg_wrong_width(
        r in prop::sample::select(BAD_WIDTH),
        m in prop::sample::select(MNEMS),
    ) {
        let asm = format!("{m} %{r}");
        let mc = llvm_mc_bytes(&asm);
        prop_assert!(mc.is_err(), "expected llvm-mc to reject `{asm}`");
        match sut_encode(m, vec![Operand::Register(Register::new(r))]) {
            Err(_) => {}
            Ok(bytes) => {
                return Err(TestCaseError::fail(format!(
                    "SUT accepted invalid-width `{asm}` → {bytes:02x?}; \
                     SETcc is r/m8 only (Intel SDM; llvm-mc rejects)"
                )));
            }
        }
    }

    // Oracle: negative_error — non-GP names aliased by reg_num
    #[test]
    fn encode_setcc_neg_non_gp(
        r in prop::sample::select(NON_GP_ALIASED),
        m in prop::sample::select(&["sete", "setne", "seto"][..]),
    ) {
        let asm = format!("{m} %{r}");
        // llvm-mc rejects non-GP; SUT must too
        let mc = llvm_mc_bytes(&asm);
        prop_assert!(mc.is_err(), "expected llvm-mc to reject `{asm}`");
        match sut_encode(m, vec![Operand::Register(Register::new(r))]) {
            Err(_) => {}
            Ok(bytes) => {
                return Err(TestCaseError::fail(format!(
                    "SUT accepted non-GP `{asm}` → {bytes:02x?}; SETcc is r/m8 GP only"
                )));
            }
        }
    }
}

/// Deterministic regression: missing ES segment prefix on sete.
#[test]
fn test_encode_setcc_regression_missing_es_prefix() {
    let mem = mem_base("eax", Displacement::None, Some("es"));
    let sut = sut_encode("sete", vec![Operand::Memory(mem)]).expect("encode");
    let mc = vec![0x26u8, 0x0f, 0x94, 0x00];
    assert_eq!(
        sut, mc,
        "sete %es:(%eax) must be [26, 0f, 94, 00], got {sut:02x?}"
    );
}

/// Deterministic regression: missing FS segment prefix on sete SIB.
#[test]
fn test_encode_setcc_regression_missing_fs_prefix() {
    let mem = mem_base_index(
        Some("esi"),
        "edi",
        4,
        Displacement::Integer(8),
        Some("fs"),
    );
    let sut = sut_encode("sete", vec![Operand::Memory(mem)]).expect("encode");
    let mc = vec![0x64u8, 0x0f, 0x94, 0x44, 0xbe, 0x08];
    assert_eq!(
        sut, mc,
        "sete %fs:8(%esi,%edi,4) must be [64, 0f, 94, 44, be, 08], got {sut:02x?}"
    );
}

/// Deterministic regression: r32 must be rejected (SETcc is r/m8).
#[test]
fn test_encode_setcc_regression_rejects_eax() {
    let result = sut_encode("sete", vec![Operand::Register(Register::new("eax"))]);
    assert!(
        result.is_err(),
        "sete %eax must Err (r32 invalid); got Ok({result:?})"
    );
}

/// Deterministic regression: r16 must be rejected (SETcc is r/m8).
#[test]
fn test_encode_setcc_regression_rejects_ax() {
    let result = sut_encode("sete", vec![Operand::Register(Register::new("ax"))]);
    assert!(
        result.is_err(),
        "sete %ax must Err (r16 invalid); got Ok({result:?})"
    );
}

// Silence unused import warnings for R16 (kept for documentation / future sweep).
#[allow(dead_code)]
fn _keep_r16() {
    let _ = R16;
}
