// Oracle: differential — llvm-mc i686 assembler (MOV GP RR: 88/89 /r + optional 0x66)
// Evidence: gp_integer.rs:162-192 encode_mov_rr;
//   encoder/mod.rs:161-163 movl/movw/movb → encode_mov(size) → encode_mov_rr for RR;
//   registers.rs:4-15 reg_num; Intel SDM Vol.2 MOV — r/m8,r8 / r/m16,r16 / r/m32,r32;
//   AT&T order: movb/movw/movl %src, %dst with ModRM.reg=src, ModRM.rm=dst under 88/89.
// Stronger considered:
//   - State machine: rejected — pure encoding, no lifecycle
//   - Algebraic round-trip: rejected — no in-tree i686 MOV decoder
// Differential: candidate=encode_mov_rr (via InstructionEncoder::encode movb/movw/movl),
//   reference=llvm-mc -triple=i686 -show-encoding (LLVM 15),
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=AT&T operand bytes <-> llvm-mc encoding bytes.
// Also: algebraic.invariant (opcode/modrm), algebraic.metamorphic (identity + swap),
//   negative_error (mismatched width + non-GP aliases).

use super::InstructionEncoder;
use crate::backend::x86::assembler::parser::{Instruction, Operand, Register};
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

fn suffix_for(w: u8) -> &'static str {
    match w {
        1 => "movb",
        2 => "movw",
        _ => "movl",
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
        // non-GP aliases that share reg_num encoding
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

fn expected_rr_bytes(src: &str, dst: &str, width: u8) -> Vec<u8> {
    let mut b = Vec::new();
    if width == 2 {
        b.push(0x66);
    }
    if width == 1 {
        b.push(0x88);
    } else {
        b.push(0x89);
    }
    let modrm = (3u8 << 6) | ((gp_num(src) & 7) << 3) | (gp_num(dst) & 7);
    b.push(modrm);
    b
}

// --- KAT gate (reference oracle prerequisite) ---

#[test]
fn encode_mov_rr_kat_llvm_mc_rr32() {
    let mc = llvm_mc_bytes("movl %eax, %ebx").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x89, 0xc3], "llvm-mc KAT mapping broken");
    let sut = sut_encode(
        "movl",
        vec![
            Operand::Register(Register::new("eax")),
            Operand::Register(Register::new("ebx")),
        ],
    )
    .expect("SUT KAT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_mov_rr_kat_llvm_mc_rr16() {
    let mc = llvm_mc_bytes("movw %ax, %bx").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x66, 0x89, 0xc3]);
    let sut = sut_encode(
        "movw",
        vec![
            Operand::Register(Register::new("ax")),
            Operand::Register(Register::new("bx")),
        ],
    )
    .expect("SUT KAT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_mov_rr_kat_llvm_mc_rr8() {
    let mc = llvm_mc_bytes("movb %al, %bl").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x88, 0xc3]);
    let sut = sut_encode(
        "movb",
        vec![
            Operand::Register(Register::new("al")),
            Operand::Register(Register::new("bl")),
        ],
    )
    .expect("SUT KAT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_mov_rr_kat_llvm_mc_ah() {
    let mc = llvm_mc_bytes("movb %ah, %al").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x88, 0xe0]);
    let sut = sut_encode(
        "movb",
        vec![
            Operand::Register(Register::new("ah")),
            Operand::Register(Register::new("al")),
        ],
    )
    .expect("SUT KAT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_mov_rr_kat_llvm_mc_identity() {
    let mc = llvm_mc_bytes("movl %esp, %esp").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x89, 0xe4]);
    let sut = sut_encode(
        "movl",
        vec![
            Operand::Register(Register::new("esp")),
            Operand::Register(Register::new("esp")),
        ],
    )
    .expect("SUT KAT");
    assert_eq!(sut, mc);
}

proptest! {
    #![proptest_config(cfg())]

    // P1: differential — same-width GP pairs match llvm-mc
    #[test]
    fn encode_mov_rr_diff_same_width_gp(
        width in prop::sample::select(vec![1u8, 2, 4]),
        si in 0usize..8,
        di in 0usize..8,
    ) {
        let regs = gp_for_width(width);
        let src = regs[si % regs.len()];
        let dst = regs[di % regs.len()];
        let mnemonic = suffix_for(width);
        let asm = format!("{mnemonic} %{src}, %{dst}");
        let mc = llvm_mc_bytes(&asm)
            .map_err(|e| TestCaseError::fail(format!("llvm-mc failed on valid `{asm}`: {e}")))?;
        let sut = sut_encode(
            mnemonic,
            vec![
                Operand::Register(Register::new(src)),
                Operand::Register(Register::new(dst)),
            ],
        )
        .map_err(|e| TestCaseError::fail(format!("SUT Err on valid `{asm}`: {e}")))?;
        prop_assert_eq!(&sut, &mc, "diff `{}`: sut={:02x?} mc={:02x?}", asm, &sut, &mc);
        prop_assert_eq!(&sut, &expected_rr_bytes(src, dst, width));
    }

    // P2: algebraic invariant — opcode / optional 0x66 / mod=3 / fields
    #[test]
    fn encode_mov_rr_invariant_opcode_modrm(
        width in prop::sample::select(vec![1u8, 2, 4]),
        si in 0usize..8,
        di in 0usize..8,
    ) {
        let regs = gp_for_width(width);
        let src = regs[si % regs.len()];
        let dst = regs[di % regs.len()];
        let mnemonic = suffix_for(width);
        let bytes = sut_encode(
            mnemonic,
            vec![
                Operand::Register(Register::new(src)),
                Operand::Register(Register::new(dst)),
            ],
        )
        .map_err(|e| TestCaseError::fail(e))?;
        let mut i = 0usize;
        if width == 2 {
            prop_assert_eq!(bytes[i], 0x66, "missing 0x66 for movw");
            i += 1;
        } else {
            prop_assert_ne!(bytes.get(0).copied().unwrap_or(0), 0x66, "spurious 0x66");
        }
        let op = bytes[i];
        if width == 1 {
            prop_assert_eq!(op, 0x88, "movb opcode");
        } else {
            prop_assert_eq!(op, 0x89, "movw/movl opcode");
        }
        i += 1;
        prop_assert_eq!(bytes.len(), i + 1, "exact length");
        let modrm = bytes[i];
        prop_assert_eq!(modrm >> 6, 0b11, "mod must be 11 (register)");
        prop_assert_eq!((modrm >> 3) & 7, gp_num(src), "ModRM.reg = src");
        prop_assert_eq!(modrm & 7, gp_num(dst), "ModRM.rm = dst");
    }

    // P3: metamorphic — identity mov matches llvm-mc
    #[test]
    fn encode_mov_rr_metamorphic_identity(
        width in prop::sample::select(vec![1u8, 2, 4]),
        ri in 0usize..8,
    ) {
        let regs = gp_for_width(width);
        let r = regs[ri % regs.len()];
        let mnemonic = suffix_for(width);
        let asm = format!("{mnemonic} %{r}, %{r}");
        let mc = llvm_mc_bytes(&asm)
            .map_err(|e| TestCaseError::fail(format!("llvm-mc `{asm}`: {e}")))?;
        let sut = sut_encode(
            mnemonic,
            vec![
                Operand::Register(Register::new(r)),
                Operand::Register(Register::new(r)),
            ],
        )
        .map_err(|e| TestCaseError::fail(format!("SUT `{asm}`: {e}")))?;
        prop_assert_eq!(&sut, &mc, "identity `{}`", asm);
    }

    // P4: negative — size-mismatched GP pairs that llvm-mc rejects must Err
    #[test]
    fn encode_mov_rr_neg_mismatched_width(
        mode in 0u8..6,
        a_i in 0usize..8,
        b_i in 0usize..8,
    ) {
        // Build pairs where mnemonic width disagrees with at least one operand.
        let (mnemonic, src, dst, width_claim) = match mode {
            0 => {
                // movl with r16 src
                let src = R16[a_i % R16.len()];
                let dst = GP32[b_i % GP32.len()];
                ("movl", src, dst, 4u8)
            }
            1 => {
                // movl with r16 dst
                let src = GP32[a_i % GP32.len()];
                let dst = R16[b_i % R16.len()];
                ("movl", src, dst, 4u8)
            }
            2 => {
                // movw with r32 src
                let src = GP32[a_i % GP32.len()];
                let dst = R16[b_i % R16.len()];
                ("movw", src, dst, 2u8)
            }
            3 => {
                // movw with r8 dst
                let src = R16[a_i % R16.len()];
                let dst = R8[b_i % R8.len()];
                ("movw", src, dst, 2u8)
            }
            4 => {
                // movb with r32 src
                let src = GP32[a_i % GP32.len()];
                let dst = R8[b_i % R8.len()];
                ("movb", src, dst, 1u8)
            }
            _ => {
                // movb with r16 dst
                let src = R8[a_i % R8.len()];
                let dst = R16[b_i % R16.len()];
                ("movb", src, dst, 1u8)
            }
        };
        let _ = width_claim;
        // Only assert when sizes truly disagree with the mnemonic or each other
        let sw = reg_size_local(src);
        let dw = reg_size_local(dst);
        let expected_w = match mnemonic {
            "movb" => 1u8,
            "movw" => 2u8,
            _ => 4u8,
        };
        prop_assume!(sw != expected_w || dw != expected_w);

        let asm = format!("{mnemonic} %{src}, %{dst}");
        let mc_err = llvm_mc_bytes(&asm).is_err();
        prop_assert!(
            mc_err,
            "expected llvm-mc to reject mismatched `{asm}`"
        );
        match sut_encode(
            mnemonic,
            vec![
                Operand::Register(Register::new(src)),
                Operand::Register(Register::new(dst)),
            ],
        ) {
            Err(_) => {}
            Ok(bytes) => {
                return Err(TestCaseError::fail(format!(
                    "SUT accepted size-mismatched `{asm}` → {bytes:02x?}; \
                     MOV RR requires matching operand size (Intel SDM; llvm-mc rejects). \
                     reg_num aliases widths so bytes look like a valid same-width form."
                )));
            }
        }
    }

    // P5: negative — non-GP names that alias through reg_num must Err
    #[test]
    fn encode_mov_rr_neg_non_gp(
        ni in 0usize..10,
        gi in 0usize..8,
        width in prop::sample::select(vec![1u8, 2, 4]),
        non_gp_as_src in any::<bool>(),
    ) {
        let non_gp = NON_GP[ni % NON_GP.len()];
        let gp = gp_for_width(width)[gi % 8];
        let mnemonic = suffix_for(width);
        let (src, dst) = if non_gp_as_src {
            (non_gp, gp)
        } else {
            (gp, non_gp)
        };
        let asm = format!("{mnemonic} %{src}, %{dst}");
        // llvm-mc should reject GP-MOV with xmm/mm/st
        let mc = llvm_mc_bytes(&asm);
        prop_assert!(
            mc.is_err(),
            "expected llvm-mc to reject non-GP `{asm}`, got {mc:?}"
        );
        match sut_encode(
            mnemonic,
            vec![
                Operand::Register(Register::new(src)),
                Operand::Register(Register::new(dst)),
            ],
        ) {
            Err(_) => {}
            Ok(bytes) => {
                return Err(TestCaseError::fail(format!(
                    "SUT accepted non-GP MOV RR `{asm}` → {bytes:02x?}; \
                     88/89 form is GP-only (Intel SDM). reg_num aliases xmm/mm/st to 0-7."
                )));
            }
        }
    }

    // P6: differential — all R8 including high-byte regs
    #[test]
    fn encode_mov_rr_diff_ah_high_byte(
        si in 0usize..8,
        di in 0usize..8,
    ) {
        let src = R8[si % R8.len()];
        let dst = R8[di % R8.len()];
        let asm = format!("movb %{src}, %{dst}");
        let mc = llvm_mc_bytes(&asm)
            .map_err(|e| TestCaseError::fail(format!("llvm-mc `{asm}`: {e}")))?;
        let sut = sut_encode(
            "movb",
            vec![
                Operand::Register(Register::new(src)),
                Operand::Register(Register::new(dst)),
            ],
        )
        .map_err(|e| TestCaseError::fail(format!("SUT `{asm}`: {e}")))?;
        prop_assert_eq!(&sut, &mc, "r8 `{}`", asm);
    }

    // P7: metamorphic — swap src/dst swaps ModRM reg/rm, same opcode/prefix
    #[test]
    fn encode_mov_rr_metamorphic_commute_modrm(
        width in prop::sample::select(vec![1u8, 2, 4]),
        ai in 0usize..8,
        bi in 0usize..8,
    ) {
        let regs = gp_for_width(width);
        let a = regs[ai % regs.len()];
        let b = regs[bi % regs.len()];
        prop_assume!(a != b);
        let mnemonic = suffix_for(width);
        let ab = sut_encode(
            mnemonic,
            vec![
                Operand::Register(Register::new(a)),
                Operand::Register(Register::new(b)),
            ],
        )
        .map_err(|e| TestCaseError::fail(e))?;
        let ba = sut_encode(
            mnemonic,
            vec![
                Operand::Register(Register::new(b)),
                Operand::Register(Register::new(a)),
            ],
        )
        .map_err(|e| TestCaseError::fail(e))?;
        prop_assert_eq!(ab.len(), ba.len());
        let off = if width == 2 { 1 } else { 0 };
        if width == 2 {
            prop_assert_eq!(ab[0], 0x66);
            prop_assert_eq!(ba[0], 0x66);
        }
        prop_assert_eq!(ab[off], ba[off], "same opcode");
        let m_ab = ab[off + 1];
        let m_ba = ba[off + 1];
        prop_assert_eq!(m_ab >> 6, 0b11);
        prop_assert_eq!(m_ba >> 6, 0b11);
        prop_assert_eq!((m_ab >> 3) & 7, m_ba & 7, "reg(ab) == rm(ba)");
        prop_assert_eq!(m_ab & 7, (m_ba >> 3) & 7, "rm(ab) == reg(ba)");
        // both sides match llvm-mc
        let asm_ab = format!("{mnemonic} %{a}, %{b}");
        let asm_ba = format!("{mnemonic} %{b}, %{a}");
        let mc_ab = llvm_mc_bytes(&asm_ab)
            .map_err(|e| TestCaseError::fail(e))?;
        let mc_ba = llvm_mc_bytes(&asm_ba)
            .map_err(|e| TestCaseError::fail(e))?;
        prop_assert_eq!(&ab, &mc_ab);
        prop_assert_eq!(&ba, &mc_ba);
    }

    // Strengthen: both operands wrong width for the suffix
    #[test]
    fn encode_mov_rr_neg_both_wrong_width(
        pair in 0u8..3,
        a_i in 0usize..8,
        b_i in 0usize..8,
    ) {
        let (mnemonic, srcs, dsts) = match pair {
            0 => ("movl", R8, R8),   // movb-sized regs under movl
            1 => ("movl", R16, R16), // movw-sized under movl
            _ => ("movb", GP32, GP32), // r32 under movb
        };
        let src = srcs[a_i % srcs.len()];
        let dst = dsts[b_i % dsts.len()];
        let asm = format!("{mnemonic} %{src}, %{dst}");
        prop_assert!(
            llvm_mc_bytes(&asm).is_err(),
            "llvm-mc should reject `{asm}`"
        );
        match sut_encode(
            mnemonic,
            vec![
                Operand::Register(Register::new(src)),
                Operand::Register(Register::new(dst)),
            ],
        ) {
            Err(_) => {}
            Ok(bytes) => {
                return Err(TestCaseError::fail(format!(
                    "SUT accepted both-wrong-width `{asm}` → {bytes:02x?}"
                )));
            }
        }
    }
}

/// Deterministic regression: movl with r16 must be rejected (width match required).
#[test]
fn test_encode_mov_rr_regression_rejects_movl_ax_ebx() {
    let result = sut_encode(
        "movl",
        vec![
            Operand::Register(Register::new("ax")),
            Operand::Register(Register::new("ebx")),
        ],
    );
    assert!(
        result.is_err(),
        "movl %ax, %ebx must Err (size mismatch); got Ok({result:?})"
    );
}

/// Deterministic regression: movb with r32 must be rejected.
#[test]
fn test_encode_mov_rr_regression_rejects_movb_eax_bl() {
    let result = sut_encode(
        "movb",
        vec![
            Operand::Register(Register::new("eax")),
            Operand::Register(Register::new("bl")),
        ],
    );
    assert!(
        result.is_err(),
        "movb %eax, %bl must Err (size mismatch); got Ok({result:?})"
    );
}

/// Deterministic regression: xmm must not encode as GP MOV.
#[test]
fn test_encode_mov_rr_regression_rejects_xmm() {
    let result = sut_encode(
        "movl",
        vec![
            Operand::Register(Register::new("xmm0")),
            Operand::Register(Register::new("eax")),
        ],
    );
    assert!(
        result.is_err(),
        "movl %xmm0, %eax must Err; got Ok({result:?})"
    );
}
