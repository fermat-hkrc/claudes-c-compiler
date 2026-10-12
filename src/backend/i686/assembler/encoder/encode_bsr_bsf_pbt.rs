// Oracle: differential — llvm-mc i686 assembler (BSF/BSR r32, r/m32)
// Evidence: gp_integer.rs:984 encode_bsr_bsf; mod.rs:270
//   "bsrl" | "bsfl" | "bsr" | "bsf" => encode_bsr_bsf;
//   Intel SDM Vol.2 BSF/BSR — Opcode 0F BC/BD /r, form r32, r/m32 (AT&T: src, dst);
//   memory source is architecturally valid; segment override must precede opcode;
//   registers.rs:4 reg_num aliases r16/r8/xmm/mm/st to the same 3-bit numbers.
// Stronger considered:
//   - State machine: rejected — pure encoding, no lifecycle
//   - Algebraic round-trip: rejected — no in-tree i686 decoder for bsf/bsr
// Differential: candidate=encode_bsr_bsf (via InstructionEncoder::encode),
//   reference=llvm-mc -triple=i686 -show-encoding (LLVM 15),
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=AT&T operand bytes <-> llvm-mc encoding bytes.
// Also: algebraic.invariant (0F opc /r), algebraic.metamorphic (bare ≡ *l),
//   negative_error (arity / wrong width / unsupported ops).

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
const SREG: &[&str] = &["es", "cs", "ss", "ds", "fs", "gs"];
const SCALES: &[u8] = &[1, 2, 4, 8];
const MNEMS: &[&str] = &["bsfl", "bsrl", "bsf", "bsr"];
const MNEMS_L: &[&str] = &["bsfl", "bsrl"];
const BAD_WIDTH: &[&str] = &[
    "ax", "cx", "dx", "bx", "sp", "bp", "si", "di", "al", "cl", "dl", "bl", "ah", "ch",
    "dh", "bh",
];

fn reg_num(name: &str) -> u8 {
    match name {
        "eax" | "ax" | "al" => 0,
        "ecx" | "cx" | "cl" => 1,
        "edx" | "dx" | "dl" => 2,
        "ebx" | "bx" | "bl" => 3,
        "esp" | "sp" | "ah" => 4,
        "ebp" | "bp" | "ch" => 5,
        "esi" | "si" | "dh" => 6,
        "edi" | "di" | "bh" => 7,
        _ => panic!("bad reg {name}"),
    }
}

fn opc(mnemonic: &str) -> u8 {
    match mnemonic {
        "bsfl" | "bsf" => 0xBC,
        "bsrl" | "bsr" => 0xBD,
        _ => panic!("bad mnem {mnemonic}"),
    }
}

fn modrm(mod_: u8, reg: u8, rm: u8) -> u8 {
    (mod_ << 6) | ((reg & 7) << 3) | (rm & 7)
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

// --- KAT gate (reference/differential prerequisite) ---

#[test]
fn encode_bsr_bsf_kat_llvm_mc_bsfl_eax_ebx() {
    let mc = llvm_mc_bytes("bsfl %eax, %ebx").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x0f, 0xbc, 0xd8], "llvm-mc KAT mapping broken");
    let sut = sut_encode(
        "bsfl",
        vec![
            Operand::Register(Register::new("eax")),
            Operand::Register(Register::new("ebx")),
        ],
    )
    .expect("SUT KAT");
    assert_eq!(sut, mc, "SUT KAT bsfl %eax, %ebx");
}

#[test]
fn encode_bsr_bsf_kat_llvm_mc_bsrl_ecx_edx() {
    let mc = llvm_mc_bytes("bsrl %ecx, %edx").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x0f, 0xbd, 0xd1]);
    let sut = sut_encode(
        "bsrl",
        vec![
            Operand::Register(Register::new("ecx")),
            Operand::Register(Register::new("edx")),
        ],
    )
    .expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_bsr_bsf_kat_llvm_mc_bare_bsf() {
    let mc = llvm_mc_bytes("bsf %esi, %edi").expect("llvm-mc KAT bare");
    assert_eq!(mc, vec![0x0f, 0xbc, 0xfe]);
    let sut = sut_encode(
        "bsf",
        vec![
            Operand::Register(Register::new("esi")),
            Operand::Register(Register::new("edi")),
        ],
    )
    .expect("SUT bare");
    assert_eq!(sut, mc);
}

#[test]
fn encode_bsr_bsf_kat_llvm_mc_mem() {
    let mc = llvm_mc_bytes("bsfl (%eax), %ebx").expect("llvm-mc KAT mem");
    assert_eq!(mc, vec![0x0f, 0xbc, 0x18]);
    let sut = sut_encode(
        "bsfl",
        vec![
            Operand::Memory(mem_base("eax", Displacement::None, None)),
            Operand::Register(Register::new("ebx")),
        ],
    )
    .expect("SUT mem");
    assert_eq!(sut, mc);
}

#[test]
fn encode_bsr_bsf_kat_llvm_mc_segment_es() {
    // Segment override must precede opcode: [26, 0f, bc, 18]
    let mc = llvm_mc_bytes("bsfl %es:(%eax), %ebx").expect("llvm-mc KAT es");
    assert_eq!(mc, vec![0x26, 0x0f, 0xbc, 0x18]);
    let sut = sut_encode(
        "bsfl",
        vec![
            Operand::Memory(mem_base("eax", Displacement::None, Some("es"))),
            Operand::Register(Register::new("ebx")),
        ],
    );
    match sut {
        Ok(bytes) => assert_eq!(
            bytes, mc,
            "SUT KAT bsfl %es:(%eax), %ebx must include 0x26"
        ),
        Err(e) => panic!("SUT rejected bsfl %es:(%eax), %ebx: {e}"),
    }
}

#[test]
fn encode_bsr_bsf_kat_llvm_mc_segment_fs() {
    let mc = llvm_mc_bytes("bsrl %fs:4(%esi), %edi").expect("llvm-mc KAT fs");
    assert_eq!(mc, vec![0x64, 0x0f, 0xbd, 0x7e, 0x04]);
    let sut = sut_encode(
        "bsrl",
        vec![
            Operand::Memory(mem_base("esi", Displacement::Integer(4), Some("fs"))),
            Operand::Register(Register::new("edi")),
        ],
    );
    match sut {
        Ok(bytes) => assert_eq!(bytes, mc, "SUT KAT bsrl %fs:4(%esi), %edi"),
        Err(e) => panic!("SUT rejected: {e}"),
    }
}

proptest! {
    #![proptest_config(cfg())]

    // Oracle: differential — r32, r32 vs llvm-mc
    #[test]
    fn encode_bsr_bsf_diff_rr(
        mnemonic in prop::sample::select(MNEMS),
        src in prop::sample::select(GP32),
        dst in prop::sample::select(GP32),
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
        prop_assert_eq!(sut, mc, "r32 diff vs llvm-mc for {}", asm);
    }

    // Oracle: differential — memory source (no segment)
    #[test]
    fn encode_bsr_bsf_diff_mem(
        mnemonic in prop::sample::select(MNEMS_L),
        dst in prop::sample::select(GP32),
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
    fn encode_bsr_bsf_diff_mem_segment(
        mnemonic in prop::sample::select(MNEMS_L),
        mseg in prop::sample::select(SREG),
        base in prop::sample::select(GP32),
        dst in prop::sample::select(GP32),
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
            "segmented mem must include override before opcode for {}",
            asm
        );
    }

    // Oracle: algebraic.invariant — r32 form layout
    #[test]
    fn encode_bsr_bsf_invariant_rr(
        mnemonic in prop::sample::select(MNEMS),
        src in prop::sample::select(GP32),
        dst in prop::sample::select(GP32),
    ) {
        let bytes = sut_encode(
            mnemonic,
            vec![
                Operand::Register(Register::new(src)),
                Operand::Register(Register::new(dst)),
            ],
        )
        .expect("valid bsfl/bsrl r32 must encode");
        let expect = vec![
            0x0Fu8,
            opc(mnemonic),
            modrm(3, reg_num(dst), reg_num(src)),
        ];
        prop_assert_eq!(bytes, expect, "invariant layout for {} %{}/{}", mnemonic, src, dst);
    }

    // Oracle: algebraic.metamorphic — bare bsf/bsr ≡ bsfl/bsrl for r32
    #[test]
    fn encode_bsr_bsf_metamorphic_bare_vs_l(
        bare in prop::sample::select(&["bsf", "bsr"][..]),
        src in prop::sample::select(GP32),
        dst in prop::sample::select(GP32),
    ) {
        let suffixed = format!("{bare}l");
        let bare_bytes = sut_encode(
            bare,
            vec![
                Operand::Register(Register::new(src)),
                Operand::Register(Register::new(dst)),
            ],
        )
        .expect("bare");
        let l_bytes = sut_encode(
            &suffixed,
            vec![
                Operand::Register(Register::new(src)),
                Operand::Register(Register::new(dst)),
            ],
        )
        .expect("suffixed");
        prop_assert_eq!(
            bare_bytes, l_bytes,
            "{} %{},%{} must equal {} %{},%{}",
            bare, src, dst, suffixed, src, dst
        );
    }

    // Oracle: negative_error — wrong arity
    #[test]
    fn encode_bsr_bsf_neg_arity(
        mnemonic in prop::sample::select(MNEMS),
        n in 0usize..4,
    ) {
        prop_assume!(n != 2);
        let ops: Vec<Operand> = (0..n)
            .map(|i| {
                if i % 2 == 0 {
                    Operand::Register(Register::new("eax"))
                } else {
                    Operand::Register(Register::new("ebx"))
                }
            })
            .collect();
        let r = sut_encode(mnemonic, ops);
        prop_assert!(r.is_err(), "arity {n} must be Err, got {r:?}");
    }

    // Oracle: negative_error — r16/r8 invalid for bsfl/bsrl (llvm-mc rejects)
    #[test]
    fn encode_bsr_bsf_neg_wrong_width(
        mnemonic in prop::sample::select(MNEMS_L),
        bad in prop::sample::select(BAD_WIDTH),
        good in prop::sample::select(GP32),
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

    // Oracle: negative_error — unsupported operand shapes
    #[test]
    fn encode_bsr_bsf_neg_unsupported(
        mnemonic in prop::sample::select(MNEMS),
        kind in 0u8..3,
        r in prop::sample::select(GP32),
        base in prop::sample::select(GP32),
    ) {
        let ops = match kind {
            0 => vec![
                Operand::Memory(mem_base(base, Displacement::None, None)),
                Operand::Memory(mem_base(base, Displacement::None, None)),
            ],
            1 => vec![
                Operand::Register(Register::new(r)),
                Operand::Memory(mem_base(base, Displacement::None, None)),
            ],
            _ => vec![
                Operand::Immediate(ImmediateValue::Integer(0)),
                Operand::Register(Register::new(r)),
            ],
        };
        let res = sut_encode(mnemonic, ops);
        prop_assert!(res.is_err(), "unsupported ops kind {kind} must be Err, got {res:?}");
    }
}

// Deterministic regression witnesses (filled after triage)

#[test]
fn test_encode_bsr_bsf_regression_segment_es_missing() {
    // Witness: bsfl %es:(%eax), %ebx → SUT omits 0x26
    let mc = llvm_mc_bytes("bsfl %es:(%eax), %ebx").expect("llvm-mc");
    assert_eq!(mc, vec![0x26, 0x0f, 0xbc, 0x18]);
    let sut = sut_encode(
        "bsfl",
        vec![
            Operand::Memory(mem_base("eax", Displacement::None, Some("es"))),
            Operand::Register(Register::new("ebx")),
        ],
    )
    .expect("SUT should encode");
    assert_eq!(
        sut, mc,
        "regression: bsfl %es:(%eax), %ebx must include 0x26 (got {:02x?})",
        sut
    );
}

#[test]
fn test_encode_bsr_bsf_regression_r16_src_accepted() {
    // Witness: bsfl %ax, %ebx should be Err (llvm-mc rejects)
    assert!(llvm_mc_bytes("bsfl %ax, %ebx").is_err());
    let sut = sut_encode(
        "bsfl",
        vec![
            Operand::Register(Register::new("ax")),
            Operand::Register(Register::new("ebx")),
        ],
    );
    assert!(
        sut.is_err(),
        "regression: bsfl %ax, %ebx must be Err, got Ok({:02x?})",
        sut.unwrap_or_default()
    );
}

#[test]
fn test_encode_bsr_bsf_regression_r8_dst_accepted() {
    assert!(llvm_mc_bytes("bsrl %eax, %al").is_err());
    let sut = sut_encode(
        "bsrl",
        vec![
            Operand::Register(Register::new("eax")),
            Operand::Register(Register::new("al")),
        ],
    );
    assert!(
        sut.is_err(),
        "regression: bsrl %eax, %al must be Err, got Ok({:02x?})",
        sut.unwrap_or_default()
    );
}
