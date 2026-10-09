// Oracle: differential — llvm-mc i686 assembler (MOV Sreg: 8C /r store, 8E /r load)
// Evidence: system.rs:273 "Encode MOV to/from segment register";
//   gp_integer.rs:21-34 routes encode_mov → encode_mov_seg when is_segment_reg;
//   registers.rs:19-34 seg_reg_num/is_segment_reg = es/cs/ss/ds/fs/gs;
//   Intel SDM Vol.2 MOV — to/from segment registers: r/m16 (r32 allowed on IA-32);
//   core.rs:31-42 emit_segment_prefix for memory segment overrides;
//   sibling x86 encode_mov_seg at x86/.../x87_misc.rs:211 (emit_rex_rm includes seg).
// Stronger considered:
//   - State machine: rejected — pure encoding, no lifecycle
//   - Algebraic round-trip: rejected — no in-tree i686 Sreg decoder
// Differential: candidate=encode_mov_seg (via InstructionEncoder::encode movl/movw/mov),
//   reference=llvm-mc -triple=i686 -show-encoding (LLVM 15),
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=AT&T operand bytes <-> llvm-mc encoding bytes.
// Also: algebraic.invariant (8C|8E + mod=3 + sreg in reg), algebraic.metamorphic
//   (read↔write share ModRM), negative_error (arity + r8 GP).

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

const SEG: &[&str] = &["es", "cs", "ss", "ds", "fs", "gs"];
const GP32: &[&str] = &["eax", "ecx", "edx", "ebx", "esp", "ebp", "esi", "edi"];
const R16: &[&str] = &["ax", "cx", "dx", "bx", "sp", "bp", "si", "di"];
const R8: &[&str] = &["al", "cl", "dl", "bl", "ah", "ch", "dh", "bh"];
const SCALES: &[u8] = &[1, 2, 4, 8];

fn seg_num(name: &str) -> u8 {
    match name {
        "es" => 0,
        "cs" => 1,
        "ss" => 2,
        "ds" => 3,
        "fs" => 4,
        "gs" => 5,
        _ => panic!("bad seg {name}"),
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

// --- KAT gate (reference oracle prerequisite) ---

#[test]
fn encode_mov_seg_kat_llvm_mc_ds_eax() {
    let mc = llvm_mc_bytes("movl %ds, %eax").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x8c, 0xd8], "llvm-mc KAT mapping broken");
    let sut = sut_encode(
        "movl",
        vec![
            Operand::Register(Register::new("ds")),
            Operand::Register(Register::new("eax")),
        ],
    )
    .expect("SUT KAT");
    assert_eq!(sut, mc, "SUT KAT movl %ds, %eax");
}

#[test]
fn encode_mov_seg_kat_llvm_mc_eax_ds() {
    let mc = llvm_mc_bytes("movl %eax, %ds").expect("llvm-mc KAT write");
    assert_eq!(mc, vec![0x8e, 0xd8]);
    let sut = sut_encode(
        "movl",
        vec![
            Operand::Register(Register::new("eax")),
            Operand::Register(Register::new("ds")),
        ],
    )
    .expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_mov_seg_kat_llvm_mc_es_mem() {
    let mc = llvm_mc_bytes("movw %es, (%eax)").expect("llvm-mc KAT mem");
    assert_eq!(mc, vec![0x8c, 0x00]);
    let sut = sut_encode(
        "movw",
        vec![
            Operand::Register(Register::new("es")),
            Operand::Memory(mem_base("eax", Displacement::None, None)),
        ],
    )
    .expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_mov_seg_kat_llvm_mc_mem_fs() {
    let mc = llvm_mc_bytes("movw (%eax), %fs").expect("llvm-mc KAT load");
    assert_eq!(mc, vec![0x8e, 0x20]);
    let sut = sut_encode(
        "movw",
        vec![
            Operand::Memory(mem_base("eax", Displacement::None, None)),
            Operand::Register(Register::new("fs")),
        ],
    )
    .expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_mov_seg_kat_llvm_mc_ds_ax_66() {
    // 16-bit dest requires 0x66
    let mc = llvm_mc_bytes("movw %ds, %ax").expect("llvm-mc KAT r16");
    assert_eq!(mc, vec![0x66, 0x8c, 0xd8]);
    let sut = sut_encode(
        "movw",
        vec![
            Operand::Register(Register::new("ds")),
            Operand::Register(Register::new("ax")),
        ],
    );
    // May fail if SUT omits 0x66 — keep as KAT that documents the contract
    match sut {
        Ok(bytes) => assert_eq!(bytes, mc, "SUT KAT movw %ds, %ax must include 0x66"),
        Err(e) => panic!("SUT rejected movw %ds, %ax: {e}"),
    }
}

#[test]
fn encode_mov_seg_kat_llvm_mc_segment_es() {
    let mc = llvm_mc_bytes("movw %ds, %es:(%eax)").expect("llvm-mc KAT seg");
    assert_eq!(mc, vec![0x26, 0x8c, 0x18]);
    let sut = sut_encode(
        "movw",
        vec![
            Operand::Register(Register::new("ds")),
            Operand::Memory(mem_base("eax", Displacement::None, Some("es"))),
        ],
    );
    match sut {
        Ok(bytes) => assert_eq!(
            bytes, mc,
            "SUT KAT movw %ds, %es:(%eax) must include segment prefix 0x26"
        ),
        Err(e) => panic!("SUT rejected: {e}"),
    }
}

proptest! {
    #![proptest_config(cfg())]

    // Oracle: differential — Sreg → GP32 (8C, no 0x66)
    #[test]
    fn encode_mov_seg_diff_sreg_to_gp32(
        sreg in prop::sample::select(SEG),
        gp in prop::sample::select(GP32),
    ) {
        let asm = format!("movl %{sreg}, %{gp}");
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let sut = sut_encode(
            "movl",
            vec![
                Operand::Register(Register::new(sreg)),
                Operand::Register(Register::new(gp)),
            ],
        )
        .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(sut, mc, "Sreg->GP32 diff vs llvm-mc for {}", asm);
    }

    // Oracle: differential — GP32 → Sreg (8E)
    #[test]
    fn encode_mov_seg_diff_gp32_to_sreg(
        gp in prop::sample::select(GP32),
        sreg in prop::sample::select(SEG),
    ) {
        let asm = format!("movl %{gp}, %{sreg}");
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let sut = sut_encode(
            "movl",
            vec![
                Operand::Register(Register::new(gp)),
                Operand::Register(Register::new(sreg)),
            ],
        )
        .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(sut, mc, "GP32->Sreg diff vs llvm-mc for {}", asm);
    }

    // Oracle: differential — Sreg → r16 (must emit 0x66)
    #[test]
    fn encode_mov_seg_diff_sreg_to_r16(
        sreg in prop::sample::select(SEG),
        r16 in prop::sample::select(R16),
    ) {
        let asm = format!("movw %{sreg}, %{r16}");
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let sut = sut_encode(
            "movw",
            vec![
                Operand::Register(Register::new(sreg)),
                Operand::Register(Register::new(r16)),
            ],
        )
        .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(
            sut, mc,
            "Sreg->r16 must match llvm-mc (incl. 0x66) for {}",
            asm
        );
    }

    // Oracle: differential — r16 → Sreg (no 0x66)
    #[test]
    fn encode_mov_seg_diff_r16_to_sreg(
        r16 in prop::sample::select(R16),
        sreg in prop::sample::select(SEG),
    ) {
        let asm = format!("movw %{r16}, %{sreg}");
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let sut = sut_encode(
            "movw",
            vec![
                Operand::Register(Register::new(r16)),
                Operand::Register(Register::new(sreg)),
            ],
        )
        .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(sut, mc, "r16->Sreg diff vs llvm-mc for {}", asm);
    }

    // Oracle: differential — memory forms without segment override
    #[test]
    fn encode_mov_seg_diff_mem(
        sreg in prop::sample::select(SEG),
        store in any::<bool>(),
        kind in 0u8..6,
        base in prop::sample::select(GP32),
        index in prop::sample::select(GP32),
        scale in prop::sample::select(SCALES),
        disp in prop_oneof![
            Just(0i64),
            Just(4i64),
            Just(-1i64),
            Just(0x12345678i64),
            any::<i8>().prop_map(|v| v as i64),
        ],
    ) {
        // Avoid SIB with index=esp (invalid; scale/index encoding uses 4 = none)
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
        let (ops, asm) = if store {
            (
                vec![
                    Operand::Register(Register::new(sreg)),
                    Operand::Memory(mem.clone()),
                ],
                format!("movw %{sreg}, {att}"),
            )
        } else {
            (
                vec![
                    Operand::Memory(mem.clone()),
                    Operand::Register(Register::new(sreg)),
                ],
                format!("movw {att}, %{sreg}"),
            )
        };

        let mc = match llvm_mc_bytes(&asm) {
            Ok(b) => b,
            Err(e) => {
                // Some abs/disp combos may be rejected; skip those
                prop_assume!(false);
                let _ = e;
                return Ok(());
            }
        };
        let sut = sut_encode("movw", ops)
            .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(sut, mc, "mem diff vs llvm-mc for {}", asm);
    }

    // Oracle: differential — memory with segment override (expects prefix)
    #[test]
    fn encode_mov_seg_diff_mem_segment(
        sreg in prop::sample::select(SEG),
        mseg in prop::sample::select(SEG),
        store in any::<bool>(),
        base in prop::sample::select(GP32),
        disp in prop_oneof![Just(0i64), Just(4i64), Just(8i64)],
    ) {
        let mem = mem_base(base, Displacement::Integer(disp), Some(mseg));
        let att = att_mem(&mem);
        let (ops, asm) = if store {
            (
                vec![
                    Operand::Register(Register::new(sreg)),
                    Operand::Memory(mem.clone()),
                ],
                format!("movw %{sreg}, {att}"),
            )
        } else {
            (
                vec![
                    Operand::Memory(mem.clone()),
                    Operand::Register(Register::new(sreg)),
                ],
                format!("movw {att}, %{sreg}"),
            )
        };
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let sut = sut_encode("movw", ops)
            .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(
            sut, mc,
            "segmented mem must include override prefix for {}",
            asm
        );
    }

    // Oracle: algebraic.invariant — 8C|8E + mod=3 + sreg in ModRM.reg
    #[test]
    fn encode_mov_seg_invariant_opcode_modrm(
        write in any::<bool>(),
        sreg in prop::sample::select(SEG),
        gp in prop::sample::select(GP32),
    ) {
        let ops = if write {
            vec![
                Operand::Register(Register::new(gp)),
                Operand::Register(Register::new(sreg)),
            ]
        } else {
            vec![
                Operand::Register(Register::new(sreg)),
                Operand::Register(Register::new(gp)),
            ]
        };
        let bytes = sut_encode("movl", ops).expect("valid mov seg must encode");
        prop_assert_eq!(bytes.len(), 2, "expected 2-byte encoding, got {:02x?}", bytes);
        let expect_opc = if write { 0x8Eu8 } else { 0x8Cu8 };
        prop_assert_eq!(bytes[0], expect_opc, "opcode byte");
        let m = bytes[1];
        prop_assert_eq!(m >> 6, 3u8, "mod must be 11b (register)");
        prop_assert_eq!((m >> 3) & 7, seg_num(sreg), "ModRM.reg = sreg number");
        prop_assert_eq!(m & 7, gp_num(gp), "ModRM.rm = GP number");
    }

    // Oracle: algebraic.metamorphic — read/write share ModRM; only opc differs
    #[test]
    fn encode_mov_seg_metamorphic_read_write(
        sreg in prop::sample::select(SEG),
        gp in prop::sample::select(GP32),
    ) {
        let read = sut_encode(
            "movl",
            vec![
                Operand::Register(Register::new(sreg)),
                Operand::Register(Register::new(gp)),
            ],
        )
        .expect("read");
        let write = sut_encode(
            "movl",
            vec![
                Operand::Register(Register::new(gp)),
                Operand::Register(Register::new(sreg)),
            ],
        )
        .expect("write");
        prop_assert_eq!(read.len(), 2);
        prop_assert_eq!(write.len(), 2);
        prop_assert_eq!(read[0], 0x8Cu8);
        prop_assert_eq!(write[0], 0x8Eu8);
        prop_assert_eq!(
            read[1], write[1],
            "ModRM must match for same sreg/GP; read={:02x?} write={:02x?}",
            read, write
        );
    }

    // Oracle: negative_error — wrong arity
    #[test]
    fn encode_mov_seg_neg_arity(
        n in 0usize..5,
    ) {
        prop_assume!(n != 2);
        let mut ops = Vec::new();
        for i in 0..n {
            if i % 2 == 0 {
                ops.push(Operand::Register(Register::new("ds")));
            } else {
                ops.push(Operand::Register(Register::new("eax")));
            }
        }
        let result = sut_encode("movl", ops);
        prop_assert!(
            result.is_err(),
            "arity {n} must Err, got Ok({result:?})"
        );
        if let Err(e) = result {
            prop_assert!(
                e.contains("2 operand") || e.contains("requires 2"),
                "unexpected err for arity {n}: {e}"
            );
        }
    }

    // Oracle: negative_error — r8 GP must be rejected (not r/m16 or r32)
    #[test]
    fn encode_mov_seg_neg_r8(
        to_sreg in any::<bool>(),
        sreg in prop::sample::select(SEG),
        r8 in prop::sample::select(R8),
    ) {
        let (ops, asm) = if to_sreg {
            (
                vec![
                    Operand::Register(Register::new(r8)),
                    Operand::Register(Register::new(sreg)),
                ],
                format!("movl %{r8}, %{sreg}"),
            )
        } else {
            (
                vec![
                    Operand::Register(Register::new(sreg)),
                    Operand::Register(Register::new(r8)),
                ],
                format!("movl %{sreg}, %{r8}"),
            )
        };
        let mc_rejects = llvm_mc_bytes(&asm).is_err();
        prop_assert!(mc_rejects, "expected llvm-mc to reject `{asm}`");
        match sut_encode("movl", ops) {
            Err(_) => {}
            Ok(bytes) => {
                return Err(TestCaseError::fail(format!(
                    "SUT accepted invalid-width r8 `{asm}` → {bytes:02x?}; \
                     MOV Sreg requires r/m16 or r32 (Intel SDM; llvm-mc rejects)"
                )));
            }
        }
    }

    // Strengthen: unsuffixed mov agrees with movl for r32 forms
    #[test]
    fn encode_mov_seg_diff_mnemonic_aliases(
        write in any::<bool>(),
        sreg in prop::sample::select(SEG),
        gp in prop::sample::select(GP32),
    ) {
        let (ops, asm) = if write {
            (
                vec![
                    Operand::Register(Register::new(gp)),
                    Operand::Register(Register::new(sreg)),
                ],
                format!("movl %{gp}, %{sreg}"),
            )
        } else {
            (
                vec![
                    Operand::Register(Register::new(sreg)),
                    Operand::Register(Register::new(gp)),
                ],
                format!("movl %{sreg}, %{gp}"),
            )
        };
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let movl = sut_encode("movl", ops.clone())
            .unwrap_or_else(|e| panic!("movl rejected: {e}"));
        let mov = sut_encode("mov", ops)
            .unwrap_or_else(|e| panic!("mov rejected: {e}"));
        prop_assert_eq!(&movl, &mc, "movl vs llvm-mc {}", asm);
        prop_assert_eq!(&mov, &mc, "unsuffixed mov vs llvm-mc {}", asm);
    }

    // Strengthen: SIB + segment override
    #[test]
    fn encode_mov_seg_diff_segment_sib(
        sreg in prop::sample::select(SEG),
        mseg in prop::sample::select(SEG),
        base in prop::sample::select(GP32),
        index in prop::sample::select(GP32),
        scale in prop::sample::select(SCALES),
        store in any::<bool>(),
    ) {
        prop_assume!(index != "esp");
        let mem = mem_base_index(
            Some(base),
            index,
            scale,
            Displacement::Integer(4),
            Some(mseg),
        );
        let att = att_mem(&mem);
        let (ops, asm) = if store {
            (
                vec![
                    Operand::Register(Register::new(sreg)),
                    Operand::Memory(mem.clone()),
                ],
                format!("movw %{sreg}, {att}"),
            )
        } else {
            (
                vec![
                    Operand::Memory(mem.clone()),
                    Operand::Register(Register::new(sreg)),
                ],
                format!("movw {att}, %{sreg}"),
            )
        };
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let sut = sut_encode("movw", ops)
            .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(sut, mc, "seg+SIB must match llvm-mc for {}", asm);
    }
}

/// Deterministic regression: sreg→r16 must emit 0x66.
#[test]
fn test_encode_mov_seg_regression_movw_ds_ax_66() {
    let result = sut_encode(
        "movw",
        vec![
            Operand::Register(Register::new("ds")),
            Operand::Register(Register::new("ax")),
        ],
    );
    assert_eq!(
        result.expect("must encode"),
        vec![0x66, 0x8c, 0xd8],
        "movw %ds, %ax must be [66, 8c, d8]"
    );
}

/// Deterministic regression: memory segment override must be emitted.
#[test]
fn test_encode_mov_seg_regression_es_segment_prefix() {
    let result = sut_encode(
        "movw",
        vec![
            Operand::Register(Register::new("ds")),
            Operand::Memory(mem_base("eax", Displacement::None, Some("es"))),
        ],
    );
    assert_eq!(
        result.expect("must encode"),
        vec![0x26, 0x8c, 0x18],
        "movw %ds, %es:(%eax) must be [26, 8c, 18]"
    );
}

/// Deterministic regression: r8 GP must be rejected.
#[test]
fn test_encode_mov_seg_regression_rejects_al() {
    let result = sut_encode(
        "movl",
        vec![
            Operand::Register(Register::new("al")),
            Operand::Register(Register::new("ds")),
        ],
    );
    assert!(
        result.is_err(),
        "movl %al, %ds must Err (not r8); got Ok({result:?})"
    );
}
