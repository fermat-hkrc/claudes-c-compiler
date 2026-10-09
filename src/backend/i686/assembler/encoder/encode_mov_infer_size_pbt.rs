// Oracle: differential — llvm-mc i686 assembler (unsuffixed MOV size inference)
// Evidence: gp_integer.rs:111 "Handle unsuffixed `mov` from inline asm - infer size
//   from operands"; gp_integer.rs:112-123 encode_mov_infer_size;
//   encoder/mod.rs:163 "mov" => encode_mov_infer_size;
//   registers.rs:62-69 reg_size (r8→1, r16/sreg→2, else 4);
//   Intel/GAS: unsuffixed mov infers width from register operands; ambiguous
//   forms (imm→mem, mismatched GP widths) require an explicit suffix.
// Stronger considered:
//   - State machine: rejected — pure encoding, no lifecycle
//   - Algebraic round-trip: rejected — no in-tree i686 MOV decoder
// Differential: candidate=encode_mov_infer_size (via InstructionEncoder::encode "mov"),
//   reference=llvm-mc -triple=i686 -show-encoding (LLVM 15),
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=AT&T operand bytes <-> llvm-mc encoding bytes.
// Also: algebraic.metamorphic (mov ≡ movb/movw/movl), algebraic.invariant
//   (infer rule), negative_error (arity / ambiguous imm-mem / mismatched width).

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
const BASE32: &[&str] = &["eax", "ecx", "edx", "ebx", "ebp", "esi", "edi"]; // no esp alone without SIB hassle for simple base

fn gp_for_width(w: u8) -> &'static [&'static str] {
    match w {
        1 => R8,
        2 => R16,
        4 => GP32,
        _ => GP32,
    }
}

fn suffix_for(w: u8) -> &'static str {
    match w {
        1 => "movb",
        2 => "movw",
        4 => "movl",
        _ => "movl",
    }
}

fn infer_size(ops: &[Operand]) -> u8 {
    match (&ops[0], &ops.get(1)) {
        (Operand::Register(r), _) => reg_size_local(&r.name),
        (_, Some(Operand::Register(r))) => reg_size_local(&r.name),
        _ => 4,
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

fn mem_base(base: &str, disp: i32) -> MemoryOperand {
    MemoryOperand {
        segment: None,
        displacement: if disp == 0 {
            Displacement::None
        } else {
            Displacement::Integer(disp as i64)
        },
        base: Some(Register::new(base)),
        index: None,
        scale: None,
    }
}

fn att_disp_i(disp: i32) -> String {
    if disp == 0 {
        String::new()
    } else {
        format!("{disp}")
    }
}

fn att_mem_base(base: &str, disp: i32) -> String {
    format!("{}(%{base})", att_disp_i(disp))
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

fn llvm_mc_rejects(asm: &str) -> bool {
    llvm_mc_bytes(asm).is_err()
}

fn imm_for_width(w: u8, raw: i64) -> i64 {
    match w {
        1 => (raw as i8) as i64,
        2 => (raw as i16) as i64,
        _ => (raw as i32) as i64,
    }
}

// --- KAT gate (reference oracle prerequisite) ---

#[test]
fn encode_mov_infer_size_kat_llvm_mc_rr32() {
    let mc = llvm_mc_bytes("mov %eax, %ebx").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x89, 0xc3], "llvm-mc KAT mapping broken");
    let sut = sut_encode(
        "mov",
        vec![
            Operand::Register(Register::new("eax")),
            Operand::Register(Register::new("ebx")),
        ],
    )
    .expect("SUT KAT");
    assert_eq!(sut, mc, "SUT KAT mov %eax, %ebx");
}

#[test]
fn encode_mov_infer_size_kat_llvm_mc_rr16() {
    let mc = llvm_mc_bytes("mov %ax, %bx").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x66, 0x89, 0xc3]);
    let sut = sut_encode(
        "mov",
        vec![
            Operand::Register(Register::new("ax")),
            Operand::Register(Register::new("bx")),
        ],
    )
    .expect("SUT");
    assert_eq!(sut, mc, "SUT must emit 0x66 for r16");
}

#[test]
fn encode_mov_infer_size_kat_llvm_mc_rr8() {
    let mc = llvm_mc_bytes("mov %al, %bl").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x88, 0xc3]);
    let sut = sut_encode(
        "mov",
        vec![
            Operand::Register(Register::new("al")),
            Operand::Register(Register::new("bl")),
        ],
    )
    .expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_mov_infer_size_kat_llvm_mc_imm_reg() {
    let mc = llvm_mc_bytes("mov $1, %eax").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0xb8, 0x01, 0x00, 0x00, 0x00]);
    let sut = sut_encode(
        "mov",
        vec![
            Operand::Immediate(ImmediateValue::Integer(1)),
            Operand::Register(Register::new("eax")),
        ],
    )
    .expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_mov_infer_size_kat_llvm_mc_imm_r16() {
    let mc = llvm_mc_bytes("mov $0x1234, %bx").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x66, 0xbb, 0x34, 0x12]);
    let sut = sut_encode(
        "mov",
        vec![
            Operand::Immediate(ImmediateValue::Integer(0x1234)),
            Operand::Register(Register::new("bx")),
        ],
    )
    .expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_mov_infer_size_kat_llvm_mc_mem_reg() {
    let mc = llvm_mc_bytes("mov (%eax), %ecx").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x8b, 0x08]);
    let sut = sut_encode(
        "mov",
        vec![
            Operand::Memory(mem_base("eax", 0)),
            Operand::Register(Register::new("ecx")),
        ],
    )
    .expect("SUT");
    assert_eq!(sut, mc);
}

// --- Deterministic regression witnesses (fail until SUT fixed) ---

/// Regression: ambiguous imm→mem must reject (GAS/llvm-mc require suffix).
/// Witness: `mov $0, (%eax)` → SUT emits movl [c7, 00, 00, 00, 00, 00].
#[test]
fn test_encode_mov_infer_size_regression_ambiguous_imm_mem() {
    let asm = "mov $0, (%eax)";
    assert!(
        llvm_mc_rejects(asm),
        "precondition: llvm-mc rejects `{asm}`"
    );
    let sut = sut_encode(
        "mov",
        vec![
            Operand::Immediate(ImmediateValue::Integer(0)),
            Operand::Memory(mem_base("eax", 0)),
        ],
    );
    assert!(
        sut.is_err(),
        "ambiguous `{asm}` must Err; got Ok({:02x?})",
        sut.as_ref().ok()
    );
}

/// Regression: mismatched GP widths must reject unsuffixed mov.
/// Witness: `mov %ax, %al` → SUT emits [66, 89, c0] (first-reg size=2).
#[test]
fn test_encode_mov_infer_size_regression_mismatched_width() {
    let asm = "mov %ax, %al";
    assert!(
        llvm_mc_rejects(asm),
        "precondition: llvm-mc rejects `{asm}`"
    );
    let sut = sut_encode(
        "mov",
        vec![
            Operand::Register(Register::new("ax")),
            Operand::Register(Register::new("al")),
        ],
    );
    assert!(
        sut.is_err(),
        "mismatched `{asm}` must Err; got Ok({:02x?})",
        sut.as_ref().ok()
    );
}

// Strengthening round: first-vs-second register preference on Imm is already
// covered; add CR/Sreg path through unsuffixed mov (size inference must not
// break the specialized CR/seg encode paths).
#[test]
fn encode_mov_infer_size_kat_cr_and_seg() {
    let cases = [
        ("mov %cr0, %eax", vec!["cr0", "eax"], true),
        ("mov %eax, %cr3", vec!["eax", "cr3"], true),
        ("mov %ds, %eax", vec!["ds", "eax"], true),
        ("mov %eax, %es", vec!["eax", "es"], true),
    ];
    for (asm, names, _reg_reg) in cases {
        let mc = llvm_mc_bytes(asm).unwrap_or_else(|e| panic!("{asm}: {e}"));
        let ops = vec![
            Operand::Register(Register::new(names[0])),
            Operand::Register(Register::new(names[1])),
        ];
        let sut = sut_encode("mov", ops).unwrap_or_else(|e| panic!("{asm} SUT: {e}"));
        assert_eq!(sut, mc, "unsuffixed mov CR/seg KAT {asm}");
    }
}

proptest! {
    #![proptest_config(cfg())]

    // Oracle: differential — same-width GP register-register
    #[test]
    fn encode_mov_infer_size_diff_rr_same_width(
        width in prop::sample::select(vec![1u8, 2, 4]),
        si in 0usize..8,
        di in 0usize..8,
    ) {
        let regs = gp_for_width(width);
        let src = regs[si % regs.len()];
        let dst = regs[di % regs.len()];
        let asm = format!("mov %{src}, %{dst}");
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let sut = sut_encode(
            "mov",
            vec![
                Operand::Register(Register::new(src)),
                Operand::Register(Register::new(dst)),
            ],
        )
        .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(sut, mc, "RR same-width diff vs llvm-mc for {}", asm);
    }

    // Oracle: differential — immediate → register (size from dst)
    #[test]
    fn encode_mov_infer_size_diff_imm_reg(
        width in prop::sample::select(vec![1u8, 2, 4]),
        di in 0usize..8,
        raw in any::<i32>(),
    ) {
        let regs = gp_for_width(width);
        let dst = regs[di % regs.len()];
        let imm = imm_for_width(width, raw as i64);
        let asm = format!("mov ${imm}, %{dst}");
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let sut = sut_encode(
            "mov",
            vec![
                Operand::Immediate(ImmediateValue::Integer(imm)),
                Operand::Register(Register::new(dst)),
            ],
        )
        .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(sut, mc, "Imm→Reg diff vs llvm-mc for {}", asm);
    }

    // Oracle: differential — Mem↔Reg both directions
    #[test]
    fn encode_mov_infer_size_diff_mem_reg(
        width in prop::sample::select(vec![1u8, 2, 4]),
        ri in 0usize..8,
        bi in 0usize..7,
        disp in prop::sample::select(vec![0i32, 4, -4, 0x100, -0x80]),
        store in any::<bool>(),
    ) {
        let regs = gp_for_width(width);
        let r = regs[ri % regs.len()];
        let base = BASE32[bi % BASE32.len()];
        // Avoid (%ebp) with disp0 — encoder/llvm both use mod=01 disp8=0; keep simple.
        let disp = if base == "ebp" && disp == 0 { 4 } else { disp };
        let mem_att = att_mem_base(base, disp);
        let (asm, ops) = if store {
            (
                format!("mov %{r}, {mem_att}"),
                vec![
                    Operand::Register(Register::new(r)),
                    Operand::Memory(mem_base(base, disp)),
                ],
            )
        } else {
            (
                format!("mov {mem_att}, %{r}"),
                vec![
                    Operand::Memory(mem_base(base, disp)),
                    Operand::Register(Register::new(r)),
                ],
            )
        };
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let sut = sut_encode("mov", ops)
            .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(sut, mc, "Mem↔Reg diff vs llvm-mc for {}", asm);
    }

    // Oracle: algebraic.metamorphic — unsuffixed mov ≡ sized mnemonic ≡ llvm-mc
    #[test]
    fn encode_mov_infer_size_metamorphic_suffix(
        width in prop::sample::select(vec![1u8, 2, 4]),
        kind in 0u8..4,
        a in 0usize..8,
        b in 0usize..8,
        raw in any::<i16>(),
        disp in prop::sample::select(vec![0i32, 8, -8]),
    ) {
        let regs = gp_for_width(width);
        let r1 = regs[a % regs.len()];
        let r2 = regs[b % regs.len()];
        let base = BASE32[b % BASE32.len()];
        let disp = if base == "ebp" && disp == 0 { 4 } else { disp };
        let imm = imm_for_width(width, raw as i64);
        let suf = suffix_for(width);

        let (ops, asm_suf, asm_bare) = match kind % 4 {
            0 => {
                // RR
                (
                    vec![
                        Operand::Register(Register::new(r1)),
                        Operand::Register(Register::new(r2)),
                    ],
                    format!("{suf} %{r1}, %{r2}"),
                    format!("mov %{r1}, %{r2}"),
                )
            }
            1 => {
                // Imm→Reg
                (
                    vec![
                        Operand::Immediate(ImmediateValue::Integer(imm)),
                        Operand::Register(Register::new(r1)),
                    ],
                    format!("{suf} ${imm}, %{r1}"),
                    format!("mov ${imm}, %{r1}"),
                )
            }
            2 => {
                // Mem→Reg
                let mem_att = att_mem_base(base, disp);
                (
                    vec![
                        Operand::Memory(mem_base(base, disp)),
                        Operand::Register(Register::new(r1)),
                    ],
                    format!("{suf} {mem_att}, %{r1}"),
                    format!("mov {mem_att}, %{r1}"),
                )
            }
            _ => {
                // Reg→Mem
                let mem_att = att_mem_base(base, disp);
                (
                    vec![
                        Operand::Register(Register::new(r1)),
                        Operand::Memory(mem_base(base, disp)),
                    ],
                    format!("{suf} %{r1}, {mem_att}"),
                    format!("mov %{r1}, {mem_att}"),
                )
            }
        };

        let mc = llvm_mc_bytes(&asm_suf)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm_suf}`: {e}"));
        let bare = sut_encode("mov", ops.clone())
            .unwrap_or_else(|e| panic!("SUT mov rejected `{asm_bare}`: {e}"));
        let sized = sut_encode(suf, ops)
            .unwrap_or_else(|e| panic!("SUT {suf} rejected `{asm_suf}`: {e}"));
        prop_assert_eq!(&bare, &sized, "metamorphic mov≡{} for {}", suf, asm_bare);
        prop_assert_eq!(&bare, &mc, "metamorphic mov vs llvm-mc for {}", asm_bare);
        let _ = asm_bare;
    }

    // Oracle: algebraic.invariant — infer(ops) drives the same bytes as suffix(infer)
    #[test]
    fn encode_mov_infer_size_invariant_inferred_size(
        width in prop::sample::select(vec![1u8, 2, 4]),
        form in 0u8..3,
        a in 0usize..8,
        b in 0usize..8,
        raw in any::<i8>(),
    ) {
        let regs = gp_for_width(width);
        let r1 = regs[a % regs.len()];
        let r2 = regs[b % regs.len()];
        let base = BASE32[b % BASE32.len()];
        let imm = imm_for_width(width, raw as i64);
        let ops = match form % 3 {
            0 => vec![
                Operand::Register(Register::new(r1)),
                Operand::Register(Register::new(r2)),
            ],
            1 => vec![
                Operand::Immediate(ImmediateValue::Integer(imm)),
                Operand::Register(Register::new(r1)),
            ],
            _ => vec![
                Operand::Memory(mem_base(base, if base == "ebp" { 4 } else { 0 })),
                Operand::Register(Register::new(r1)),
            ],
        };
        let inferred = infer_size(&ops);
        prop_assert_eq!(inferred, width, "oracle infer must match generator width");
        let suf = suffix_for(inferred);
        let bare = sut_encode("mov", ops.clone())
            .unwrap_or_else(|e| panic!("mov err: {e}"));
        let sized = sut_encode(suf, ops)
            .unwrap_or_else(|e| panic!("{suf} err: {e}"));
        prop_assert_eq!(bare, sized, "invariant: mov ≡ {} for width {}", suf, width);
    }

    // Oracle: negative_error — wrong arity
    #[test]
    fn encode_mov_infer_size_neg_arity(n in 0usize..5) {
        prop_assume!(n != 2);
        let mut ops = Vec::new();
        for i in 0..n {
            ops.push(Operand::Register(Register::new(
                if i % 2 == 0 { "eax" } else { "ebx" },
            )));
        }
        let result = sut_encode("mov", ops);
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

    // Oracle: differential (accept/reject) — ambiguous imm → mem must be rejected
    // llvm-mc: "ambiguous instructions require an explicit suffix"
    #[test]
    fn encode_mov_infer_size_neg_ambiguous_imm_mem(
        bi in 0usize..7,
        imm in -128i64..128,
        disp in prop::sample::select(vec![0i32, 4, 16]),
    ) {
        let base = BASE32[bi % BASE32.len()];
        let disp = if base == "ebp" && disp == 0 { 4 } else { disp };
        let mem_att = att_mem_base(base, disp);
        let asm = format!("mov ${imm}, {mem_att}");
        prop_assert!(
            llvm_mc_rejects(&asm),
            "precondition: llvm-mc must reject ambiguous `{asm}`"
        );
        let sut = sut_encode(
            "mov",
            vec![
                Operand::Immediate(ImmediateValue::Integer(imm)),
                Operand::Memory(mem_base(base, disp)),
            ],
        );
        prop_assert!(
            sut.is_err(),
            "ambiguous imm→mem `{asm}` must Err (no register to infer size); \
             got Ok({:02x?}) — silently defaulted to size 4?",
            sut.as_ref().ok()
        );
    }

    // Strengthening: differential — unsuffixed mov through CR/Sreg path
    #[test]
    fn encode_mov_infer_size_diff_cr_seg(
        kind in 0u8..4,
        gi in 0usize..8,
        ci in 0usize..4,
        si in 0usize..6,
    ) {
        const CR: &[&str] = &["cr0", "cr2", "cr3", "cr4"];
        const SEG: &[&str] = &["es", "cs", "ss", "ds", "fs", "gs"];
        let gp = GP32[gi % GP32.len()];
        let (asm, ops) = match kind % 4 {
            0 => {
                let cr = CR[ci % CR.len()];
                (
                    format!("mov %{cr}, %{gp}"),
                    vec![
                        Operand::Register(Register::new(cr)),
                        Operand::Register(Register::new(gp)),
                    ],
                )
            }
            1 => {
                let cr = CR[ci % CR.len()];
                (
                    format!("mov %{gp}, %{cr}"),
                    vec![
                        Operand::Register(Register::new(gp)),
                        Operand::Register(Register::new(cr)),
                    ],
                )
            }
            2 => {
                let sg = SEG[si % SEG.len()];
                (
                    format!("mov %{sg}, %{gp}"),
                    vec![
                        Operand::Register(Register::new(sg)),
                        Operand::Register(Register::new(gp)),
                    ],
                )
            }
            _ => {
                let sg = SEG[si % SEG.len()];
                (
                    format!("mov %{gp}, %{sg}"),
                    vec![
                        Operand::Register(Register::new(gp)),
                        Operand::Register(Register::new(sg)),
                    ],
                )
            }
        };
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let sut = sut_encode("mov", ops)
            .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(sut, mc, "CR/Sreg unsuffixed mov vs llvm-mc for {}", asm);
    }

    // Oracle: differential (accept/reject) — mismatched GP widths
    // llvm-mc: "unknown use of instruction mnemonic without a size suffix"
    #[test]
    fn encode_mov_infer_size_neg_mismatched_width(
        w1 in prop::sample::select(vec![1u8, 2, 4]),
        w2 in prop::sample::select(vec![1u8, 2, 4]),
        si in 0usize..8,
        di in 0usize..8,
    ) {
        prop_assume!(w1 != w2);
        let src = gp_for_width(w1)[si % 8];
        let dst = gp_for_width(w2)[di % 8];
        let asm = format!("mov %{src}, %{dst}");
        prop_assert!(
            llvm_mc_rejects(&asm),
            "precondition: llvm-mc must reject mismatched `{asm}`"
        );
        let sut = sut_encode(
            "mov",
            vec![
                Operand::Register(Register::new(src)),
                Operand::Register(Register::new(dst)),
            ],
        );
        prop_assert!(
            sut.is_err(),
            "mismatched-width `{asm}` must Err; got Ok({:02x?}) \
             (first-reg size={} silently applied?)",
            sut.as_ref().ok(),
            w1
        );
    }
}
