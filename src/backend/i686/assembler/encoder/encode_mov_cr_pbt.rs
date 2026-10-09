// Oracle: differential — llvm-mc i686 assembler (MOV CR: 0F 20 /r read, 0F 22 /r write)
// Evidence: system.rs:249 "Encode MOV to/from control register: 0F 20 /r (read) or 0F 22 /r (write)";
//   gp_integer.rs:18-19 routes encode_mov → encode_mov_cr when is_control_reg;
//   registers.rs:37-49 is_control_reg/control_reg_num = cr0/cr2/cr3/cr4;
//   Intel SDM Vol.2 MOV — to/from control registers: r32 only on IA-32;
//   sibling x86 encode_mov_cr at x86/.../system.rs:136 (REX for cr8/r8+).
// Stronger considered:
//   - State machine: rejected — pure encoding, no lifecycle
//   - Algebraic round-trip: rejected — no in-tree i686 CR decoder
// Differential: candidate=encode_mov_cr (via InstructionEncoder::encode movl/mov),
//   reference=llvm-mc -triple=i686 -show-encoding (LLVM 15),
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=AT&T operand bytes <-> llvm-mc encoding bytes.
// Also: algebraic.invariant (0F 20|22 + mod=3 + CR in reg), algebraic.metamorphic
//   (read↔write share ModRM), negative_error (arity + bad ops + non-r32 GP).

use super::InstructionEncoder;
use crate::backend::x86::assembler::parser::{
    ImmediateValue, Instruction, MemoryOperand, Operand, Register, Displacement,
};
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;

fn cfg() -> ProptestConfig {
    ProptestConfig::with_cases(CASES)
}

const CR_REGS: &[&str] = &["cr0", "cr2", "cr3", "cr4"];
const GP32: &[&str] = &["eax", "ecx", "edx", "ebx", "esp", "ebp", "esi", "edi"];
const R16: &[&str] = &["ax", "cx", "dx", "bx", "sp", "bp", "si", "di"];
const R8: &[&str] = &["al", "cl", "dl", "bl", "ah", "ch", "dh", "bh"];
const SEG: &[&str] = &["es", "cs", "ss", "ds", "fs", "gs"];

fn cr_num(name: &str) -> u8 {
    match name {
        "cr0" => 0,
        "cr2" => 2,
        "cr3" => 3,
        "cr4" => 4,
        _ => panic!("bad cr {name}"),
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

fn mem_base(base: &str) -> MemoryOperand {
    MemoryOperand {
        segment: None,
        displacement: Displacement::None,
        base: Some(Register::new(base)),
        index: None,
        scale: None,
    }
}

// --- KAT gate (reference oracle prerequisite) ---

#[test]
fn encode_mov_cr_kat_llvm_mc_cr0_eax() {
    let mc = llvm_mc_bytes("movl %cr0, %eax").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x0f, 0x20, 0xc0], "llvm-mc KAT mapping broken");
    let sut = sut_encode(
        "movl",
        vec![
            Operand::Register(Register::new("cr0")),
            Operand::Register(Register::new("eax")),
        ],
    )
    .expect("SUT KAT");
    assert_eq!(sut, mc, "SUT KAT movl %cr0, %eax");
}

#[test]
fn encode_mov_cr_kat_llvm_mc_eax_cr0() {
    let mc = llvm_mc_bytes("movl %eax, %cr0").expect("llvm-mc KAT write");
    assert_eq!(mc, vec![0x0f, 0x22, 0xc0]);
    let sut = sut_encode(
        "movl",
        vec![
            Operand::Register(Register::new("eax")),
            Operand::Register(Register::new("cr0")),
        ],
    )
    .expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_mov_cr_kat_llvm_mc_cr3_esp() {
    let mc = llvm_mc_bytes("movl %cr3, %esp").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x0f, 0x20, 0xdc]);
    let sut = sut_encode(
        "movl",
        vec![
            Operand::Register(Register::new("cr3")),
            Operand::Register(Register::new("esp")),
        ],
    )
    .expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_mov_cr_kat_llvm_mc_ebx_cr2() {
    let mc = llvm_mc_bytes("movl %ebx, %cr2").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x0f, 0x22, 0xd3]);
    let sut = sut_encode(
        "movl",
        vec![
            Operand::Register(Register::new("ebx")),
            Operand::Register(Register::new("cr2")),
        ],
    )
    .expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_mov_cr_kat_llvm_mc_cr4_edi() {
    let mc = llvm_mc_bytes("movl %cr4, %edi").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x0f, 0x20, 0xe7]);
    let sut = sut_encode(
        "movl",
        vec![
            Operand::Register(Register::new("cr4")),
            Operand::Register(Register::new("edi")),
        ],
    )
    .expect("SUT");
    assert_eq!(sut, mc);
}

proptest! {
    #![proptest_config(cfg())]

    // Oracle: differential — CR → GP (read, 0F 20)
    #[test]
    fn encode_mov_cr_diff_cr_to_gp(
        cr in prop::sample::select(CR_REGS),
        gp in prop::sample::select(GP32),
    ) {
        let asm = format!("movl %{cr}, %{gp}");
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let sut = sut_encode(
            "movl",
            vec![
                Operand::Register(Register::new(cr)),
                Operand::Register(Register::new(gp)),
            ],
        )
        .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(sut, mc, "CR->GP diff vs llvm-mc for {}", asm);
    }

    // Oracle: differential — GP → CR (write, 0F 22)
    #[test]
    fn encode_mov_cr_diff_gp_to_cr(
        gp in prop::sample::select(GP32),
        cr in prop::sample::select(CR_REGS),
    ) {
        let asm = format!("movl %{gp}, %{cr}");
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let sut = sut_encode(
            "movl",
            vec![
                Operand::Register(Register::new(gp)),
                Operand::Register(Register::new(cr)),
            ],
        )
        .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(sut, mc, "GP->CR diff vs llvm-mc for {}", asm);
    }

    // Oracle: algebraic.invariant — 0F 20|22 + mod=3 + CR in ModRM.reg
    #[test]
    fn encode_mov_cr_invariant_opcode_modrm(
        write in any::<bool>(),
        cr in prop::sample::select(CR_REGS),
        gp in prop::sample::select(GP32),
    ) {
        let ops = if write {
            vec![
                Operand::Register(Register::new(gp)),
                Operand::Register(Register::new(cr)),
            ]
        } else {
            vec![
                Operand::Register(Register::new(cr)),
                Operand::Register(Register::new(gp)),
            ]
        };
        let bytes = sut_encode("movl", ops).expect("valid CR mov must encode");
        prop_assert_eq!(bytes.len(), 3, "expected 3-byte encoding, got {:02x?}", bytes);
        prop_assert_eq!(bytes[0], 0x0F);
        let expect_opc = if write { 0x22u8 } else { 0x20u8 };
        prop_assert_eq!(bytes[1], expect_opc, "opcode byte");
        let m = bytes[2];
        prop_assert_eq!(m >> 6, 3u8, "mod must be 11b (register)");
        prop_assert_eq!((m >> 3) & 7, cr_num(cr), "ModRM.reg = CR number");
        prop_assert_eq!(m & 7, gp_num(gp), "ModRM.rm = GP number");
    }

    // Oracle: algebraic.metamorphic — read/write share ModRM; only opc differs
    #[test]
    fn encode_mov_cr_metamorphic_read_write(
        cr in prop::sample::select(CR_REGS),
        gp in prop::sample::select(GP32),
    ) {
        let read = sut_encode(
            "movl",
            vec![
                Operand::Register(Register::new(cr)),
                Operand::Register(Register::new(gp)),
            ],
        )
        .expect("read");
        let write = sut_encode(
            "movl",
            vec![
                Operand::Register(Register::new(gp)),
                Operand::Register(Register::new(cr)),
            ],
        )
        .expect("write");
        prop_assert_eq!(read.len(), 3);
        prop_assert_eq!(write.len(), 3);
        prop_assert_eq!(read[0], 0x0F);
        prop_assert_eq!(write[0], 0x0F);
        prop_assert_eq!(read[1], 0x20u8);
        prop_assert_eq!(write[1], 0x22u8);
        prop_assert_eq!(
            read[2], write[2],
            "ModRM must match for same CR/GP; read={:02x?} write={:02x?}",
            read, write
        );
    }

    // Oracle: differential — unsuffixed `mov` agrees with `movl` / llvm-mc
    #[test]
    fn encode_mov_cr_diff_mnemonic_aliases(
        write in any::<bool>(),
        cr in prop::sample::select(CR_REGS),
        gp in prop::sample::select(GP32),
    ) {
        let (ops, asm) = if write {
            (
                vec![
                    Operand::Register(Register::new(gp)),
                    Operand::Register(Register::new(cr)),
                ],
                format!("movl %{gp}, %{cr}"),
            )
        } else {
            (
                vec![
                    Operand::Register(Register::new(cr)),
                    Operand::Register(Register::new(gp)),
                ],
                format!("movl %{cr}, %{gp}"),
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

    // Oracle: negative_error — wrong arity
    #[test]
    fn encode_mov_cr_neg_arity(
        n in 0usize..5,
    ) {
        prop_assume!(n != 2);
        let mut ops = Vec::new();
        for i in 0..n {
            if i % 2 == 0 {
                ops.push(Operand::Register(Register::new("cr0")));
            } else {
                ops.push(Operand::Register(Register::new("eax")));
            }
        }
        // Drive through encode_mov_cr via movl only when CR is present and len!=2.
        // For empty / single non-CR, encode_mov may take a different path — call
        // with at least one CR when n>=1 so is_control_reg triggers, else still
        // expect mov arity error from encode_mov.
        let result = sut_encode("movl", ops.clone());
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

    // Oracle: negative_error — imm / mem / label / seg / non-r32 GP
    #[test]
    fn encode_mov_cr_neg_bad_operands(
        kind in 0u8..8,
        cr in prop::sample::select(CR_REGS),
        gp32 in prop::sample::select(GP32),
        r16 in prop::sample::select(R16),
        r8 in prop::sample::select(R8),
        seg in prop::sample::select(SEG),
        imm in any::<i32>(),
    ) {
        match kind {
            0 => {
                // imm, CR — not a CR↔GP register pair; must Err (any message)
                let err = sut_encode(
                    "movl",
                    vec![
                        Operand::Immediate(ImmediateValue::Integer(imm as i64)),
                        Operand::Register(Register::new(cr)),
                    ],
                )
                .expect_err("imm->CR must Err");
                let _ = err;
            }
            1 => {
                // CR, mem
                let err = sut_encode(
                    "movl",
                    vec![
                        Operand::Register(Register::new(cr)),
                        Operand::Memory(mem_base(gp32)),
                    ],
                )
                .expect_err("CR->mem must Err");
                let _ = err;
            }
            2 => {
                // label, CR
                let err = sut_encode(
                    "movl",
                    vec![
                        Operand::Label("sym".into()),
                        Operand::Register(Register::new(cr)),
                    ],
                )
                .expect_err("label->CR must Err");
                let _ = err;
            }
            3 => {
                // CR → r16: Intel MOV CR is r32; llvm-mc rejects
                let asm = format!("movl %{cr}, %{r16}");
                let mc_rejects = llvm_mc_bytes(&asm).is_err();
                prop_assert!(mc_rejects, "expected llvm-mc to reject `{asm}`");
                match sut_encode(
                    "movl",
                    vec![
                        Operand::Register(Register::new(cr)),
                        Operand::Register(Register::new(r16)),
                    ],
                ) {
                    Err(_) => {}
                    Ok(bytes) => {
                        return Err(TestCaseError::fail(format!(
                            "SUT accepted invalid-width GP `{asm}` → {bytes:02x?}; \
                             MOV CR requires r32 (Intel SDM; llvm-mc rejects)"
                        )));
                    }
                }
            }
            4 => {
                // r16 → CR
                let asm = format!("movl %{r16}, %{cr}");
                let mc_rejects = llvm_mc_bytes(&asm).is_err();
                prop_assert!(mc_rejects, "expected llvm-mc to reject `{asm}`");
                match sut_encode(
                    "movl",
                    vec![
                        Operand::Register(Register::new(r16)),
                        Operand::Register(Register::new(cr)),
                    ],
                ) {
                    Err(_) => {}
                    Ok(bytes) => {
                        return Err(TestCaseError::fail(format!(
                            "SUT accepted invalid-width GP `{asm}` → {bytes:02x?}; \
                             MOV CR requires r32 (Intel SDM; llvm-mc rejects)"
                        )));
                    }
                }
            }
            5 => {
                // CR → r8
                let asm = format!("movl %{cr}, %{r8}");
                let mc_rejects = llvm_mc_bytes(&asm).is_err();
                prop_assert!(mc_rejects, "expected llvm-mc to reject `{asm}`");
                match sut_encode(
                    "movl",
                    vec![
                        Operand::Register(Register::new(cr)),
                        Operand::Register(Register::new(r8)),
                    ],
                ) {
                    Err(_) => {}
                    Ok(bytes) => {
                        return Err(TestCaseError::fail(format!(
                            "SUT accepted invalid-width GP `{asm}` → {bytes:02x?}; \
                             MOV CR requires r32 (Intel SDM; llvm-mc rejects)"
                        )));
                    }
                }
            }
            6 => {
                // r8 → CR
                let asm = format!("movl %{r8}, %{cr}");
                let mc_rejects = llvm_mc_bytes(&asm).is_err();
                prop_assert!(mc_rejects, "expected llvm-mc to reject `{asm}`");
                match sut_encode(
                    "movl",
                    vec![
                        Operand::Register(Register::new(r8)),
                        Operand::Register(Register::new(cr)),
                    ],
                ) {
                    Err(_) => {}
                    Ok(bytes) => {
                        return Err(TestCaseError::fail(format!(
                            "SUT accepted invalid-width GP `{asm}` → {bytes:02x?}; \
                             MOV CR requires r32 (Intel SDM; llvm-mc rejects)"
                        )));
                    }
                }
            }
            _ => {
                // segment register with CR
                let result = sut_encode(
                    "movl",
                    vec![
                        Operand::Register(Register::new(cr)),
                        Operand::Register(Register::new(seg)),
                    ],
                );
                // seg is not a GP; either Err or must not silently look like CR→GP
                match result {
                    Err(_) => {}
                    Ok(bytes) => {
                        // If accepted, it is wrong: segment is not r32 GP
                        return Err(TestCaseError::fail(format!(
                            "SUT accepted CR→seg movl %{cr}, %{seg} → {bytes:02x?}"
                        )));
                    }
                }
            }
        }
    }

    // Strengthen: movw mnemonic with CR must not silently emit 0F 20/22 as if r32
    #[test]
    fn encode_mov_cr_neg_movw_width(
        write in any::<bool>(),
        cr in prop::sample::select(CR_REGS),
        r16 in prop::sample::select(R16),
    ) {
        let (ops, asm) = if write {
            (
                vec![
                    Operand::Register(Register::new(r16)),
                    Operand::Register(Register::new(cr)),
                ],
                format!("movw %{r16}, %{cr}"),
            )
        } else {
            (
                vec![
                    Operand::Register(Register::new(cr)),
                    Operand::Register(Register::new(r16)),
                ],
                format!("movw %{cr}, %{r16}"),
            )
        };
        let mc_rejects = llvm_mc_bytes(&asm).is_err()
            || llvm_mc_bytes(&asm.replace("movw", "movl")).is_err();
        // llvm-mc rejects movw with CR entirely
        let mc_movw = llvm_mc_bytes(&asm);
        prop_assert!(mc_movw.is_err(), "expected llvm-mc to reject `{asm}`");
        let _ = mc_rejects;
        match sut_encode("movw", ops) {
            Err(_) => {}
            Ok(bytes) => {
                return Err(TestCaseError::fail(format!(
                    "SUT accepted `{asm}` → {bytes:02x?}; MOV CR is r32-only \
                     (Intel SDM; llvm-mc rejects movw with CR)"
                )));
            }
        }
    }
}

/// Deterministic regression: 16-bit GP must be rejected (r32 only).
#[test]
fn test_encode_mov_cr_regression_rejects_ax() {
    let result = sut_encode(
        "movl",
        vec![
            Operand::Register(Register::new("cr0")),
            Operand::Register(Register::new("ax")),
        ],
    );
    assert!(
        result.is_err(),
        "movl %cr0, %ax must Err (r32 only); got Ok({result:?})"
    );
}

/// Deterministic regression: 8-bit GP must be rejected (r32 only).
#[test]
fn test_encode_mov_cr_regression_rejects_al() {
    let result = sut_encode(
        "movl",
        vec![
            Operand::Register(Register::new("al")),
            Operand::Register(Register::new("cr0")),
        ],
    );
    assert!(
        result.is_err(),
        "movl %al, %cr0 must Err (r32 only); got Ok({result:?})"
    );
}
