// Oracle: differential — llvm-mc i686 assembler (LMSW 0F 01 /6 r/m16)
// Evidence: system.rs:201-202 "Encode LMSW (Load Machine Status Word): 0F 01 /6
//   Accepts a 16-bit register or memory operand.";
//   encoder/mod.rs:343 "lmsw" => encode_lmsw(ops);
//   Intel SDM LMSW: 0F 01 /6 + r/m16;
//   sibling x86 encode_lmsw at x86/.../system.rs:239 calls emit_rex_rm before opcode;
//   i686 core.rs:31-42 emit_segment_prefix for fs/gs/es/cs/ss/ds.
// Stronger considered:
//   - State machine: rejected — pure encoding, no lifecycle
//   - Algebraic round-trip: rejected — no in-tree i686 decoder for lmsw
// Differential: candidate=encode_lmsw (via InstructionEncoder::encode),
//   reference=llvm-mc -triple=i686 -show-encoding (LLVM 15),
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=AT&T operand bytes <-> llvm-mc encoding bytes.
// Also: algebraic.invariant (opcode/ext6), algebraic.metamorphic (vs lidt /3),
//   negative_error (arity + bad ops + non-r16).

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
const BAD_WIDTH_REGS: &[&str] = &[
    "eax", "ecx", "edx", "ebx", "esp", "ebp", "esi", "edi", "al", "cl", "dl", "bl", "ah", "ch",
    "dh", "bh",
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

fn strip_seg_prefixes(bytes: &[u8]) -> &[u8] {
    let mut i = 0;
    while i < bytes.len() && matches!(bytes[i], 0x26 | 0x2E | 0x36 | 0x3E | 0x64 | 0x65) {
        i += 1;
    }
    &bytes[i..]
}

fn set_modrm_reg(bytes: &[u8], new_reg: u8) -> Vec<u8> {
    let body = strip_seg_prefixes(bytes);
    let mut out = body.to_vec();
    // body: 0F 01 ModRM ...
    if out.len() >= 3 {
        let modrm = out[2];
        out[2] = (modrm & 0b1100_0111) | ((new_reg & 7) << 3);
    }
    out
}

// --- KAT gate ---

#[test]
fn encode_lmsw_kat_llvm_mc_ax() {
    let mc = llvm_mc_bytes("lmsw %ax").expect("llvm-mc KAT ax");
    assert_eq!(mc, vec![0x0f, 0x01, 0xf0], "llvm-mc KAT mapping broken");
    let sut = sut_encode("lmsw", vec![Operand::Register(Register::new("ax"))]).expect("SUT KAT");
    assert_eq!(sut, mc, "SUT KAT lmsw %ax");
}

#[test]
fn encode_lmsw_kat_llvm_mc_eax_mem() {
    let mc = llvm_mc_bytes("lmsw (%eax)").expect("llvm-mc KAT mem");
    assert_eq!(mc, vec![0x0f, 0x01, 0x30]);
    let sut = sut_encode(
        "lmsw",
        vec![Operand::Memory(mem_base("eax", Displacement::None, None))],
    )
    .expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_lmsw_kat_llvm_mc_esp() {
    let mc = llvm_mc_bytes("lmsw (%esp)").expect("llvm-mc KAT esp");
    assert_eq!(mc, vec![0x0f, 0x01, 0x34, 0x24]);
    let sut = sut_encode(
        "lmsw",
        vec![Operand::Memory(mem_base("esp", Displacement::None, None))],
    )
    .expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_lmsw_kat_llvm_mc_ebp_disp() {
    let mc = llvm_mc_bytes("lmsw 8(%ebp)").expect("llvm-mc KAT 8(%ebp)");
    assert_eq!(mc, vec![0x0f, 0x01, 0x75, 0x08]);
    let sut = sut_encode(
        "lmsw",
        vec![Operand::Memory(mem_base(
            "ebp",
            Displacement::Integer(8),
            None,
        ))],
    )
    .expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_lmsw_kat_llvm_mc_segment_es() {
    let mc = llvm_mc_bytes("lmsw %es:(%eax)").expect("llvm-mc KAT es");
    assert_eq!(mc, vec![0x26, 0x0f, 0x01, 0x30]);
    let sut = sut_encode(
        "lmsw",
        vec![Operand::Memory(mem_base(
            "eax",
            Displacement::None,
            Some("es"),
        ))],
    );
    match sut {
        Ok(bytes) => assert_eq!(bytes, mc, "SUT must emit ES override 0x26 before 0F 01"),
        Err(e) => panic!("SUT erred on valid ES mem: {e}"),
    }
}

#[test]
fn encode_lmsw_kat_llvm_mc_segment_fs() {
    let mc = llvm_mc_bytes("lmsw %fs:4(%esi)").expect("llvm-mc KAT fs");
    assert_eq!(mc, vec![0x64, 0x0f, 0x01, 0x76, 0x04]);
    let sut = sut_encode(
        "lmsw",
        vec![Operand::Memory(mem_base(
            "esi",
            Displacement::Integer(4),
            Some("fs"),
        ))],
    );
    match sut {
        Ok(bytes) => assert_eq!(bytes, mc, "SUT must emit FS override 0x64 before 0F 01"),
        Err(e) => panic!("SUT erred on valid FS mem: {e}"),
    }
}

proptest! {
    #![proptest_config(cfg())]

    // Oracle: differential — 16-bit register form vs llvm-mc
    #[test]
    fn encode_lmsw_diff_reg16(
        reg in prop::sample::select(R16_REGS),
    ) {
        let asm = format!("lmsw %{reg}");
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let sut = sut_encode("lmsw", vec![Operand::Register(Register::new(reg))])
            .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(sut, mc, "reg16 diff vs llvm-mc for `{}`", asm);
    }

    // Oracle: differential — base+disp memory
    #[test]
    fn encode_lmsw_diff_base_disp(
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
    ) {
        let mem = mem_base(base, Displacement::Integer(disp), None);
        let asm = format!("lmsw {}", att_mem(&mem));
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let sut = sut_encode("lmsw", vec![Operand::Memory(mem)])
            .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(sut, mc, "diff vs llvm-mc for `{}`", asm);
    }

    // Oracle: differential — SIB (index ≠ esp)
    #[test]
    fn encode_lmsw_diff_sib(
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
    ) {
        let mem = mem_base_index(
            base,
            index,
            scale,
            Displacement::Integer(disp),
            None,
        );
        let asm = format!("lmsw {}", att_mem(&mem));
        let mc = match llvm_mc_bytes(&asm) {
            Ok(b) => b,
            Err(_) => return Ok(()),
        };
        let sut = sut_encode("lmsw", vec![Operand::Memory(mem)])
            .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(sut, mc, "SIB diff vs llvm-mc for `{}`", asm);
    }

    // Oracle: differential — segment override prefixes
    #[test]
    fn encode_lmsw_diff_segment(
        seg in prop::sample::select(SEG_REGS),
        base in prop::sample::select(GP_REGS),
        disp in prop_oneof![Just(0i64), Just(8i64), Just(-4i64), Just(127i64), Just(-128i64)],
    ) {
        let mem = mem_base(base, Displacement::Integer(disp), Some(seg));
        let asm = format!("lmsw {}", att_mem(&mem));
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let sut = sut_encode("lmsw", vec![Operand::Memory(mem)])
            .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(
            sut.clone(), mc.clone(),
            "segment prefix diff for `{}`: SUT={:02x?} llvm-mc={:02x?}",
            asm, sut, mc
        );
    }

    // Oracle: differential — ESP/EBP/abs/SIB edges
    #[test]
    fn encode_lmsw_diff_edges(
        edge in 0u8..12,
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
        let asm = format!("lmsw {}", att_mem(&mem));
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected edge `{asm}`: {e}"));
        let sut = sut_encode("lmsw", vec![Operand::Memory(mem)])
            .unwrap_or_else(|e| panic!("SUT rejected edge `{asm}`: {e}"));
        prop_assert_eq!(sut, mc, "edge diff for `{}`", asm);
    }

    // Oracle: algebraic.invariant — 0F 01 /6
    #[test]
    fn encode_lmsw_invariant_opcode_ext6(
        kind in 0u8..3,
        r16 in prop::sample::select(R16_REGS),
        base in prop::sample::select(GP_REGS),
        disp in prop_oneof![Just(0i64), Just(7i64), Just(-3i64), (-200i64..=300i64)],
        index in prop::sample::select(
            GP_REGS.iter().copied().filter(|r| *r != "esp").collect::<Vec<_>>()
        ),
        scale in prop::sample::select(SCALES),
    ) {
        let ops = match kind {
            0 => vec![Operand::Register(Register::new(r16))],
            1 => vec![Operand::Memory(mem_base(base, Displacement::Integer(disp), None))],
            _ => vec![Operand::Memory(mem_base_index(
                Some(base),
                index,
                scale,
                Displacement::Integer(disp),
                None,
            ))],
        };
        let bytes = sut_encode("lmsw", ops).expect("valid op must encode");
        let body = strip_seg_prefixes(&bytes);
        prop_assert!(body.len() >= 3, "need opcode+modrm, got {bytes:?}");
        prop_assert_eq!(body[0], 0x0F);
        prop_assert_eq!(body[1], 0x01);
        let reg = (body[2] >> 3) & 7;
        prop_assert_eq!(reg, 6u8, "ModRM.reg must be /6 for LMSW");
    }

    // Oracle: algebraic.metamorphic — same mem shape as lidt; only ModRM.reg differs
    #[test]
    fn encode_lmsw_metamorphic_vs_lidt(
        base in prop::sample::select(GP_REGS),
        disp in prop_oneof![
            Just(0i64),
            Just(-1i64),
            Just(8i64),
            Just(127i64),
            Just(-128i64),
            Just(128i64),
            (-256i64..=256i64),
        ],
        kind in 0u8..3,
        index in prop::sample::select(
            GP_REGS.iter().copied().filter(|r| *r != "esp").collect::<Vec<_>>()
        ),
        scale in prop::sample::select(SCALES),
    ) {
        let mem = match kind {
            0 => mem_base(base, Displacement::Integer(disp), None),
            1 => mem_base_index(Some(base), index, scale, Displacement::Integer(disp), None),
            _ => mem_abs(disp.wrapping_mul(0x0101_0101i64) as i32 as i64, None),
        };
        let lmsw = sut_encode("lmsw", vec![Operand::Memory(mem.clone())])
            .expect("lmsw must encode");
        let lidt = sut_encode("lidt", vec![Operand::Memory(mem)])
            .expect("lidt must encode");
        let lmsw_as_lidt = set_modrm_reg(&lmsw, 3);
        let lidt_body = strip_seg_prefixes(&lidt).to_vec();
        prop_assert_eq!(
            lmsw_as_lidt, lidt_body,
            "lmsw and lidt must share mod+rm/SIB/disp (reg 6 vs 3); lmsw={:02x?} lidt={:02x?}",
            lmsw, lidt
        );
    }

    // Oracle: negative_error — wrong arity
    #[test]
    fn encode_lmsw_neg_arity(
        n_extra in 0usize..4,
        use_empty in any::<bool>(),
    ) {
        let ops: Vec<Operand> = if use_empty {
            vec![]
        } else {
            let mut v = vec![
                Operand::Register(Register::new("ax")),
                Operand::Register(Register::new("bx")),
            ];
            for _ in 0..n_extra {
                v.push(Operand::Memory(mem_base("eax", Displacement::None, None)));
            }
            v
        };
        prop_assume!(ops.len() != 1);
        let err = sut_encode("lmsw", ops).expect_err("arity ≠ 1 must Err");
        prop_assert!(
            err.contains("lmsw requires 1 operand") || err.contains("requires 1 operand"),
            "unexpected err: {err}"
        );
    }

    // Oracle: negative_error — imm/label and non-r/m16 register widths
    #[test]
    fn encode_lmsw_neg_bad_operand(
        kind in 0u8..4,
        bad_reg in prop::sample::select(BAD_WIDTH_REGS),
        imm in any::<i32>(),
    ) {
        match kind {
            0 => {
                let err = sut_encode(
                    "lmsw",
                    vec![Operand::Immediate(ImmediateValue::Integer(imm as i64))],
                )
                .expect_err("imm must Err");
                prop_assert!(
                    err.contains("register or memory") || err.contains("lmsw requires"),
                    "unexpected err: {err}"
                );
            }
            1 => {
                let err = sut_encode("lmsw", vec![Operand::Label("target".into())])
                    .expect_err("label must Err");
                prop_assert!(
                    err.contains("register or memory") || err.contains("lmsw requires"),
                    "unexpected err: {err}"
                );
            }
            2 | _ => {
                // 32-bit or 8-bit register: Intel LMSW is r/m16; llvm-mc rejects.
                let asm = format!("lmsw %{bad_reg}");
                let mc_rejects = llvm_mc_bytes(&asm).is_err();
                prop_assert!(mc_rejects, "expected llvm-mc to reject `{asm}`");
                match sut_encode("lmsw", vec![Operand::Register(Register::new(bad_reg))]) {
                    Err(_) => {}
                    Ok(bytes) => {
                        return Err(TestCaseError::fail(format!(
                            "SUT accepted invalid-width register `{asm}` → {bytes:02x?}; \
                             LMSW requires r/m16 (doc: 16-bit register; llvm-mc rejects)"
                        )));
                    }
                }
            }
        }
    }

    // Absolute disp32
    #[test]
    fn encode_lmsw_diff_abs_disp32(
        disp in prop_oneof![
            Just(0i64),
            Just(1i64),
            Just(0x1234_5678i64),
            Just(-1i64),
            (-0x8000_0000i64..=0x7fff_ffffi64),
        ],
    ) {
        let mem = mem_abs(disp, None);
        let asm = format!("lmsw {}", att_mem(&mem));
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let sut = sut_encode("lmsw", vec![Operand::Memory(mem)])
            .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(sut, mc, "abs diff for `{}`", asm);
    }

    // Segment + SIB
    #[test]
    fn encode_lmsw_diff_segment_sib(
        seg in prop::sample::select(SEG_REGS),
        base in prop::sample::select(GP_REGS),
        index in prop::sample::select(
            GP_REGS.iter().copied().filter(|r| *r != "esp").collect::<Vec<_>>()
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
        let asm = format!("lmsw {}", att_mem(&mem));
        let mc = match llvm_mc_bytes(&asm) {
            Ok(b) => b,
            Err(_) => return Ok(()),
        };
        let sut = sut_encode("lmsw", vec![Operand::Memory(mem)])
            .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(
            sut.clone(), mc.clone(),
            "seg+SIB diff for `{}`: SUT={:02x?} llvm-mc={:02x?}",
            asm, sut, mc
        );
    }
}

/// Deterministic regression: missing ES segment prefix on lmsw.
#[test]
fn test_encode_lmsw_regression_missing_es_prefix() {
    let mem = mem_base("eax", Displacement::None, Some("es"));
    let sut = sut_encode("lmsw", vec![Operand::Memory(mem)]).expect("encode");
    let mc = vec![0x26u8, 0x0f, 0x01, 0x30];
    assert_eq!(
        sut, mc,
        "lmsw %es:(%eax) must be [26, 0f, 01, 30], got {sut:02x?}"
    );
}

/// Deterministic regression: missing FS segment prefix on lmsw.
#[test]
fn test_encode_lmsw_regression_missing_fs_prefix() {
    let mem = mem_base("esi", Displacement::Integer(4), Some("fs"));
    let sut = sut_encode("lmsw", vec![Operand::Memory(mem)]).expect("encode");
    let mc = vec![0x64u8, 0x0f, 0x01, 0x76, 0x04];
    assert_eq!(
        sut, mc,
        "lmsw %fs:4(%esi) must be [64, 0f, 01, 76, 04], got {sut:02x?}"
    );
}

/// Deterministic regression: 32-bit register must be rejected (r/m16 only).
#[test]
fn test_encode_lmsw_regression_rejects_eax() {
    let result = sut_encode("lmsw", vec![Operand::Register(Register::new("eax"))]);
    assert!(
        result.is_err(),
        "lmsw %eax must Err (r/m16 only); got Ok({result:?})"
    );
}

/// Deterministic regression: 8-bit register must be rejected (r/m16 only).
#[test]
fn test_encode_lmsw_regression_rejects_al() {
    let result = sut_encode("lmsw", vec![Operand::Register(Register::new("al"))]);
    assert!(
        result.is_err(),
        "lmsw %al must Err (r/m16 only); got Ok({result:?})"
    );
}
