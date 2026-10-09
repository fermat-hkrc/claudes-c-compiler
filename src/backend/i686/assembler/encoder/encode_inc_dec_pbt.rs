// Oracle: differential — llvm-mc i686 assembler (INC/DEC r/m8|16|32)
// Evidence: gp_integer.rs:801-846 encode_inc_dec; mod.rs:233-238 incl/incw/incb/decl/decw/decb;
//   Intel SDM Vol.2 INC/DEC — r32 compact 40+rd / 48+rd; r/m FE/FF /0|/1; 0x66 for word;
//   core.rs:31-42 emit_segment_prefix for fs/gs/es/cs/ss/ds.
// Stronger considered:
//   - State machine: rejected — pure encoding, no lifecycle
//   - Algebraic round-trip: rejected — no in-tree i686 decoder for inc/dec
// Differential: candidate=encode_inc_dec (via InstructionEncoder::encode),
//   reference=llvm-mc -triple=i686 -show-encoding (LLVM 15),
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=AT&T operand bytes <-> llvm-mc encoding bytes.
// Also: algebraic.invariant (r32 compact), algebraic.metamorphic (segmented=prefix‖bare),
//   negative_error (arity / xmm / mismatched width).

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

const GP32: &[&str] = &["eax", "ecx", "edx", "ebx", "esp", "ebp", "esi", "edi"];
const GP16: &[&str] = &["ax", "cx", "dx", "bx", "sp", "bp", "si", "di"];
const GP8: &[&str] = &["al", "cl", "dl", "bl", "ah", "ch", "dh", "bh"];
const SREGS: &[&str] = &["es", "cs", "ss", "ds", "fs", "gs"];
const XMM: &[&str] = &["xmm0", "xmm1", "xmm2", "xmm3", "xmm4", "xmm5", "xmm6", "xmm7"];
const SCALES: &[u8] = &[1, 2, 4, 8];
const MNEMS32: &[&str] = &["incl", "decl"];
const MNEMS16: &[&str] = &["incw", "decw"];
const MNEMS8: &[&str] = &["incb", "decb"];
const MNEMS_ALL: &[&str] = &["incl", "incw", "incb", "decl", "decw", "decb"];

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
        if t.chars().all(|c| c == 'A') {
            for _ in 0..t.len() {
                bytes.push(0);
            }
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
    while i < bytes.len() && matches!(bytes[i], 0x26 | 0x2E | 0x36 | 0x3E | 0x64 | 0x65 | 0x66) {
        i += 1;
    }
    &bytes[i..]
}

// --- KAT gate (reference oracle prerequisite) ---

#[test]
fn encode_inc_dec_kat_llvm_mc_incl_eax() {
    let mc = llvm_mc_bytes("incl %eax").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x40], "llvm-mc KAT mapping broken");
    let sut = sut_encode("incl", vec![Operand::Register(Register::new("eax"))])
        .expect("SUT KAT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_inc_dec_kat_llvm_mc_decl_ebx() {
    let mc = llvm_mc_bytes("decl %ebx").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x4b]);
    let sut = sut_encode("decl", vec![Operand::Register(Register::new("ebx"))]).expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_inc_dec_kat_llvm_mc_incw_ax() {
    let mc = llvm_mc_bytes("incw %ax").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x66, 0x40]);
    let sut = sut_encode("incw", vec![Operand::Register(Register::new("ax"))]).expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_inc_dec_kat_llvm_mc_incb_al() {
    let mc = llvm_mc_bytes("incb %al").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0xfe, 0xc0]);
    let sut = sut_encode("incb", vec![Operand::Register(Register::new("al"))]).expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_inc_dec_kat_llvm_mc_incl_mem() {
    let mc = llvm_mc_bytes("incl (%eax)").expect("llvm-mc KAT mem");
    assert_eq!(mc, vec![0xff, 0x00]);
    let sut = sut_encode(
        "incl",
        vec![Operand::Memory(mem_base("eax", Displacement::None, None))],
    )
    .expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_inc_dec_kat_llvm_mc_es_segment() {
    // Contract: all six segment overrides are valid on i686 (core.rs emit_segment_prefix).
    let mc = llvm_mc_bytes("incl %es:(%eax)").expect("llvm-mc KAT es");
    assert_eq!(mc, vec![0x26, 0xff, 0x00]);
    let sut = sut_encode(
        "incl",
        vec![Operand::Memory(mem_base(
            "eax",
            Displacement::None,
            Some("es"),
        ))],
    );
    match sut {
        Ok(bytes) => assert_eq!(
            bytes, mc,
            "SUT must emit ES override 0x26 before FF /0"
        ),
        Err(e) => panic!("SUT erred on valid ES incl: {e}"),
    }
}

#[test]
fn encode_inc_dec_kat_llvm_mc_fs_segment() {
    let mc = llvm_mc_bytes("incl %fs:(%ebx)").expect("llvm-mc KAT fs");
    assert_eq!(mc, vec![0x64, 0xff, 0x03]);
    let sut = sut_encode(
        "incl",
        vec![Operand::Memory(mem_base(
            "ebx",
            Displacement::None,
            Some("fs"),
        ))],
    );
    match sut {
        Ok(bytes) => assert_eq!(
            bytes, mc,
            "SUT must emit FS override 0x64 before FF /0"
        ),
        Err(e) => panic!("SUT erred on valid FS incl: {e}"),
    }
}

/// Deterministic regression: missing segment override.
/// Witness: incl %es:(%eax) → SUT omits 0x26.
#[test]
fn encode_inc_dec_regression_missing_es_prefix() {
    let mc = llvm_mc_bytes("incl %es:(%eax)").expect("llvm-mc");
    assert_eq!(mc, vec![0x26, 0xff, 0x00]);
    let sut = sut_encode(
        "incl",
        vec![Operand::Memory(mem_base(
            "eax",
            Displacement::None,
            Some("es"),
        ))],
    )
    .expect("valid ES incl must encode");
    assert_eq!(
        sut, mc,
        "regression: encode_inc_dec must emit segment override before 0xFF /0"
    );
}

/// Deterministic regression: non-GP xmm accepted via reg_num alias.
/// Witness: incl %xmm0 → SUT emits [0x40].
#[test]
fn encode_inc_dec_regression_xmm0_accepted() {
    assert!(
        llvm_mc_bytes("incl %xmm0").is_err(),
        "llvm-mc must reject incl %xmm0"
    );
    let r = sut_encode("incl", vec![Operand::Register(Register::new("xmm0"))]);
    assert!(
        r.is_err(),
        "encode_inc_dec must reject non-GP xmm0, got {r:?}"
    );
}

/// Deterministic regression: width-mismatched register accepted.
/// Witness: incl %ax → SUT emits [0x40] (same as incl %eax).
#[test]
fn encode_inc_dec_regression_mismatched_width() {
    assert!(
        llvm_mc_bytes("incl %ax").is_err(),
        "llvm-mc must reject incl %ax"
    );
    let r = sut_encode("incl", vec![Operand::Register(Register::new("ax"))]);
    assert!(
        r.is_err(),
        "encode_inc_dec must reject width-mismatched incl %ax, got {r:?}"
    );
}

// --- Properties ---

proptest! {
    #![proptest_config(cfg())]

    // P1: differential — GP r32 compact form
    #[test]
    fn encode_inc_dec_diff_reg32(
        r in prop::sample::select(GP32),
        mnem in prop::sample::select(MNEMS32),
    ) {
        let asm = format!("{mnem} %{r}");
        let mc = llvm_mc_bytes(&asm)
            .map_err(|e| TestCaseError::fail(format!("llvm-mc rejected `{asm}`: {e}")))?;
        let sut = sut_encode(mnem, vec![Operand::Register(Register::new(r))])
            .map_err(|e| TestCaseError::fail(format!("SUT rejected `{asm}`: {e}")))?;
        prop_assert_eq!(&sut, &mc, "r32 diff `{}`: sut={:02x?} mc={:02x?}", asm, &sut, &mc);
    }

    // P2: differential — GP r16 with 0x66 prefix
    #[test]
    fn encode_inc_dec_diff_reg16(
        r in prop::sample::select(GP16),
        mnem in prop::sample::select(MNEMS16),
    ) {
        let asm = format!("{mnem} %{r}");
        let mc = llvm_mc_bytes(&asm)
            .map_err(|e| TestCaseError::fail(format!("llvm-mc rejected `{asm}`: {e}")))?;
        let sut = sut_encode(mnem, vec![Operand::Register(Register::new(r))])
            .map_err(|e| TestCaseError::fail(format!("SUT rejected `{asm}`: {e}")))?;
        prop_assert_eq!(&sut, &mc, "r16 diff `{}`: sut={:02x?} mc={:02x?}", asm, &sut, &mc);
    }

    // P3: differential — GP r8 FE /ext form
    #[test]
    fn encode_inc_dec_diff_reg8(
        r in prop::sample::select(GP8),
        mnem in prop::sample::select(MNEMS8),
    ) {
        let asm = format!("{mnem} %{r}");
        let mc = llvm_mc_bytes(&asm)
            .map_err(|e| TestCaseError::fail(format!("llvm-mc rejected `{asm}`: {e}")))?;
        let sut = sut_encode(mnem, vec![Operand::Register(Register::new(r))])
            .map_err(|e| TestCaseError::fail(format!("SUT rejected `{asm}`: {e}")))?;
        prop_assert_eq!(&sut, &mc, "r8 diff `{}`: sut={:02x?} mc={:02x?}", asm, &sut, &mc);
    }

    // P4: differential — bare memory forms across sizes
    #[test]
    fn encode_inc_dec_diff_mem(
        kind in 0u8..12,
        base in prop::sample::select(GP32),
        index in prop::sample::select(
            GP32.iter().copied().filter(|r| *r != "esp").collect::<Vec<_>>()
        ),
        scale in prop::sample::select(SCALES),
        disp in prop_oneof![
            Just(0i64),
            Just(-1i64),
            Just(1i64),
            Just(127i64),
            Just(-128i64),
            Just(128i64),
            Just(-129i64),
            Just(0x1234i64),
            (-512i64..=512i64),
        ],
        mnem in prop::sample::select(MNEMS_ALL),
    ) {
        let mem = match kind {
            0 => mem_base(base, Displacement::None, None),
            1 => mem_base(base, Displacement::Integer(disp), None),
            2 => mem_base("esp", Displacement::None, None),
            3 => mem_base("ebp", Displacement::None, None),
            4 => mem_base("esp", Displacement::Integer(disp), None),
            5 => mem_base("ebp", Displacement::Integer(if disp == 0 { 1 } else { disp }), None),
            6 => mem_base_index(Some(base), index, scale, Displacement::None, None),
            7 => mem_base_index(Some(base), index, scale, Displacement::Integer(disp), None),
            8 => mem_base_index(Some("esp"), index, scale, Displacement::Integer(disp), None),
            9 => mem_abs(disp, None),
            10 => mem_abs(0x1234_5678, None),
            _ => mem_abs(0, None),
        };
        let asm = format!("{mnem} {}", att_mem(&mem));
        let mc = match llvm_mc_bytes(&asm) {
            Ok(b) => b,
            Err(_) => return Ok(()), // skip unencodable edge
        };
        let sut = sut_encode(mnem, vec![Operand::Memory(mem)])
            .map_err(|e| TestCaseError::fail(format!("SUT rejected `{asm}`: {e}")))?;
        prop_assert_eq!(&sut, &mc, "mem diff `{}`: sut={:02x?} mc={:02x?}", asm, &sut, &mc);
    }

    // P5: differential — all six segment overrides on memory form
    #[test]
    fn encode_inc_dec_diff_mem_segment(
        seg in prop::sample::select(SREGS),
        base in prop::sample::select(GP32),
        disp in prop_oneof![Just(0i64), Just(8i64), Just(-4i64), Just(127i64), Just(-128i64)],
        mnem in prop::sample::select(MNEMS32),
    ) {
        let mem = mem_base(base, Displacement::Integer(disp), Some(seg));
        let asm = format!("{mnem} {}", att_mem(&mem));
        let mc = llvm_mc_bytes(&asm)
            .map_err(|e| TestCaseError::fail(format!("llvm-mc rejected `{asm}`: {e}")))?;
        let sut = sut_encode(mnem, vec![Operand::Memory(mem)])
            .map_err(|e| TestCaseError::fail(format!(
                "SUT rejected valid segment form `{asm}`: {e}; \
                 i686 INC/DEC must emit segment override (core.rs emit_segment_prefix; \
                 Intel SDM 2.1.1)."
            )))?;
        prop_assert_eq!(
            &sut, &mc,
            "segment diff `{}`: sut={:02x?} mc={:02x?}",
            asm, &sut, &mc
        );
    }

    // P6: algebraic.metamorphic — segmented = seg_prefix ‖ bare; strip recovers bare
    #[test]
    fn encode_inc_dec_meta_segment_stripped_eq_bare(
        seg in prop::sample::select(SREGS),
        base in prop::sample::select(GP32),
        disp in prop_oneof![Just(0i64), Just(4i64), Just(-1i64), Just(127i64)],
        mnem in prop::sample::select(MNEMS_ALL),
    ) {
        let bare_mem = mem_base(base, Displacement::Integer(disp), None);
        let seg_mem = mem_base(base, Displacement::Integer(disp), Some(seg));
        let bare = sut_encode(mnem, vec![Operand::Memory(bare_mem)])
            .map_err(|e| TestCaseError::fail(format!("bare: {e}")))?;
        let with_seg = sut_encode(mnem, vec![Operand::Memory(seg_mem)])
            .map_err(|e| TestCaseError::fail(format!(
                "segmented inc/dec must encode (emit_segment_prefix contract): {e}"
            )))?;
        let expect_prefix = seg_prefix_byte(seg);
        // Word forms start with 0x66 after segment; segment must still lead.
        prop_assert!(
            !with_seg.is_empty() && with_seg[0] == expect_prefix,
            "segmented {mnem} must start with 0x{expect_prefix:02x} for %{seg}:, got {:02x?}",
            with_seg
        );
        let stripped = strip_seg_prefixes(&with_seg);
        // strip_seg also strips 0x66; bare may start with 0x66 for word — re-compare carefully
        let bare_body = if bare.first() == Some(&0x66) {
            &bare[..]
        } else {
            &bare[..]
        };
        // After stripping only segment prefixes (not 0x66), body must match bare.
        // Our strip_seg_prefixes strips 0x66 too — so strip bare the same way.
        let bare_stripped = strip_seg_prefixes(bare_body);
        prop_assert_eq!(
            stripped, bare_stripped,
            "strip_seg(seg) must equal strip_seg(bare); seg={:02x?} bare={:02x?}",
            with_seg, bare
        );
    }

    // P7: algebraic.invariant — r32 compact opcodes
    #[test]
    fn encode_inc_dec_invariant_compact_r32(r in prop::sample::select(GP32)) {
        let n = reg_num(r);
        let inc = sut_encode("incl", vec![Operand::Register(Register::new(r))])
            .map_err(|e| TestCaseError::fail(e))?;
        prop_assert_eq!(&inc, &vec![0x40 + n], "incl %{} → 0x40+n", r);
        let dec = sut_encode("decl", vec![Operand::Register(Register::new(r))])
            .map_err(|e| TestCaseError::fail(e))?;
        prop_assert_eq!(&dec, &vec![0x48 + n], "decl %{} → 0x48+n", r);
    }

    // P8: negative_error — wrong arity
    #[test]
    fn encode_inc_dec_neg_arity(n in 0usize..4) {
        prop_assume!(n != 1);
        let ops: Vec<Operand> = (0..n)
            .map(|_| Operand::Register(Register::new("eax")))
            .collect();
        let r = sut_encode("incl", ops);
        prop_assert!(r.is_err(), "arity {n} must be Err, got {r:?}");
    }

    // P9: negative_error — non-GP xmm must be rejected (llvm-mc rejects)
    #[test]
    fn encode_inc_dec_neg_xmm(
        x in prop::sample::select(XMM),
        mnem in prop::sample::select(MNEMS32),
    ) {
        let asm = format!("{mnem} %{x}");
        prop_assert!(
            llvm_mc_bytes(&asm).is_err(),
            "llvm-mc must reject `{asm}`"
        );
        let r = sut_encode(mnem, vec![Operand::Register(Register::new(x))]);
        prop_assert!(
            r.is_err(),
            "encode_inc_dec must reject non-GP `{x}` for {mnem}, got {r:?}"
        );
    }

    // P10: negative_error — width-mismatched GP must be rejected
    #[test]
    fn encode_inc_dec_neg_mismatched_width(
        // incl/decl with r16/r8; incw/decw with r32/r8; incb/decb with r32/r16
        case in 0u8..6,
        r32 in prop::sample::select(GP32),
        r16 in prop::sample::select(GP16),
        r8 in prop::sample::select(GP8),
    ) {
        let (mnem, bad) = match case {
            0 => ("incl", r16),
            1 => ("incl", r8),
            2 => ("incw", r32),
            3 => ("incw", r8),
            4 => ("incb", r32),
            _ => ("incb", r16),
        };
        let asm = format!("{mnem} %{bad}");
        prop_assert!(
            llvm_mc_bytes(&asm).is_err(),
            "llvm-mc must reject `{asm}`"
        );
        let r = sut_encode(mnem, vec![Operand::Register(Register::new(bad))]);
        prop_assert!(
            r.is_err(),
            "encode_inc_dec must reject width-mismatched `{asm}`, got {r:?}"
        );
    }
}
