use super::*;
use crate::backend::arm::assembler::parser::Operand;

// ── Compare ──────────────────────────────────────────────────────────────

pub(crate) fn encode_cmp(operands: &[Operand]) -> Result<EncodeResult, String> {
    // CMP Rn, op -> SUBS XZR, Rn, op
    let mut new_ops = vec![Operand::Reg("xzr".to_string())];
    new_ops.extend(operands.iter().cloned());
    // Determine if 32-bit or 64-bit from the first operand
    let is_32 = if let Some(Operand::Reg(r)) = operands.first() {
        is_32bit_reg(r)
    } else {
        false
    };
    if is_32 {
        new_ops[0] = Operand::Reg("wzr".to_string());
    }
    encode_add_sub(&new_ops, true, true)
}

pub(crate) fn encode_cmn(operands: &[Operand]) -> Result<EncodeResult, String> {
    // CMN Rn, op -> ADDS XZR, Rn, op
    let mut new_ops = vec![Operand::Reg("xzr".to_string())];
    new_ops.extend(operands.iter().cloned());
    let is_32 = if let Some(Operand::Reg(r)) = operands.first() {
        is_32bit_reg(r)
    } else {
        false
    };
    if is_32 {
        new_ops[0] = Operand::Reg("wzr".to_string());
    }
    encode_add_sub(&new_ops, false, true)
}

pub(crate) fn encode_tst(operands: &[Operand]) -> Result<EncodeResult, String> {
    // TST Rn, op -> ANDS XZR, Rn, op
    let mut new_ops = vec![Operand::Reg("xzr".to_string())];
    new_ops.extend(operands.iter().cloned());
    let is_32 = if let Some(Operand::Reg(r)) = operands.first() {
        is_32bit_reg(r)
    } else {
        false
    };
    if is_32 {
        new_ops[0] = Operand::Reg("wzr".to_string());
    }
    encode_logical(&new_ops, 0b11)
}

pub(crate) fn encode_ccmp_ccmn(operands: &[Operand], is_ccmp: bool) -> Result<EncodeResult, String> {
    // CCMP/CCMN Rn, #imm5, #nzcv, cond
    // The only difference: CCMP has bit 30 = 1, CCMN has bit 30 = 0
    let (rn, is_64) = get_reg(operands, 0)?;
    let sf = sf_bit(is_64);
    let op = if is_ccmp { 1u32 << 30 } else { 0u32 };

    if let (Some(Operand::Imm(imm5)), Some(Operand::Imm(nzcv)), Some(Operand::Cond(cond))) =
        (operands.get(1), operands.get(2), operands.get(3))
    {
        let cond_val = encode_cond(cond).ok_or("invalid condition")?;
        let word = (sf << 31) | op | (1 << 29) | (0b11010010 << 21)
            | ((*imm5 as u32 & 0x1F) << 16) | (cond_val << 12) | (1 << 11) | (rn << 5) | (*nzcv as u32 & 0xF);
        return Ok(EncodeResult::Word(word));
    }

    // CCMP/CCMN Rn, Rm, #nzcv, cond
    if let (Some(Operand::Reg(rm_name)), Some(Operand::Imm(nzcv)), Some(Operand::Cond(cond))) =
        (operands.get(1), operands.get(2), operands.get(3))
    {
        let rm = parse_reg_num(rm_name).ok_or("invalid rm")?;
        let cond_val = encode_cond(cond).ok_or("invalid condition")?;
        let word = (sf << 31) | op | (1 << 29) | (0b11010010 << 21)
            | (rm << 16) | (cond_val << 12) | (rn << 5) | (*nzcv as u32 & 0xF);
        return Ok(EncodeResult::Word(word));
    }

    let name = if is_ccmp { "ccmp" } else { "ccmn" };
    Err(format!("unsupported {} operands", name))
}

// ── Conditional select ───────────────────────────────────────────────────

pub(crate) fn encode_csel(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
    let cond = match operands.get(3) {
        Some(Operand::Cond(c)) => encode_cond(c).ok_or("invalid cond")?,
        _ => return Err("csel requires condition".to_string()),
    };
    let sf = sf_bit(is_64);
    let word = ((sf << 31) | (0b11010100 << 21)
        | (rm << 16) | (cond << 12)) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

pub(crate) fn encode_csinc(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
    let cond = match operands.get(3) {
        Some(Operand::Cond(c)) => encode_cond(c).ok_or("invalid cond")?,
        _ => return Err("csinc requires condition".to_string()),
    };
    let sf = sf_bit(is_64);
    let word = (sf << 31) | (0b11010100 << 21)
        | (rm << 16) | (cond << 12) | (0b01 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

pub(crate) fn encode_csinv(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
    let cond = match operands.get(3) {
        Some(Operand::Cond(c)) => encode_cond(c).ok_or("invalid cond")?,
        _ => return Err("csinv requires condition".to_string()),
    };
    let sf = sf_bit(is_64);
    let word = (((sf << 31) | (1 << 30)) | (0b11010100 << 21)
        | (rm << 16) | (cond << 12)) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

pub(crate) fn encode_csneg(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
    let cond = match operands.get(3) {
        Some(Operand::Cond(c)) => encode_cond(c).ok_or("invalid cond")?,
        _ => return Err("csneg requires condition".to_string()),
    };
    let sf = sf_bit(is_64);
    let word = ((sf << 31) | (1 << 30)) | (0b11010100 << 21)
        | (rm << 16) | (cond << 12) | (0b01 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

pub(crate) fn encode_cset(operands: &[Operand]) -> Result<EncodeResult, String> {
    // CSET Rd, cond -> CSINC Rd, XZR, XZR, invert(cond)
    let (rd, is_64) = get_reg(operands, 0)?;
    let cond = match operands.get(1) {
        Some(Operand::Cond(c)) => encode_cond(c).ok_or("invalid cond")?,
        _ => return Err("cset requires condition".to_string()),
    };
    let sf = sf_bit(is_64);
    let inv_cond = cond ^ 1; // invert least significant bit
    let word = (sf << 31) | (0b11010100 << 21)
        | (0b11111 << 16) | (inv_cond << 12) | (0b01 << 10) | (0b11111 << 5) | rd;
    Ok(EncodeResult::Word(word))
}

pub(crate) fn encode_csetm(operands: &[Operand]) -> Result<EncodeResult, String> {
    // CSETM Rd, cond -> CSINV Rd, XZR, XZR, invert(cond)
    let (rd, is_64) = get_reg(operands, 0)?;
    let cond = match operands.get(1) {
        Some(Operand::Cond(c)) => encode_cond(c).ok_or("invalid cond")?,
        _ => return Err("csetm requires condition".to_string()),
    };
    let sf = sf_bit(is_64);
    let inv_cond = cond ^ 1;
    let word = (((sf << 31) | (1 << 30)) | (0b11010100 << 21)
        | (0b11111 << 16) | (inv_cond << 12)) | (0b11111 << 5) | rd;
    Ok(EncodeResult::Word(word))
}

// ── Branches ─────────────────────────────────────────────────────────────

pub(crate) fn encode_branch(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (sym, addend) = get_symbol(operands, 0)?;
    // B: 000101 imm26 (filled by linker/assembler)
    Ok(EncodeResult::WordWithReloc {
        word: 0b000101 << 26,
        reloc: Relocation {
            reloc_type: RelocType::Jump26,
            symbol: sym,
            addend,
        },
    })
}

pub(crate) fn encode_bl(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (sym, addend) = get_symbol(operands, 0)?;
    // BL: 100101 imm26
    Ok(EncodeResult::WordWithReloc {
        word: 0b100101 << 26,
        reloc: Relocation {
            reloc_type: RelocType::Call26,
            symbol: sym,
            addend,
        },
    })
}

pub(crate) fn encode_cond_branch(cond: &str, operands: &[Operand]) -> Result<EncodeResult, String> {
    let cond_val = encode_cond(cond).ok_or_else(|| format!("unknown condition: {}", cond))?;
    let (sym, addend) = get_symbol(operands, 0)?;
    // B.cond: 01010100 imm19 0 cond
    let word = (0b01010100 << 24) | cond_val;
    Ok(EncodeResult::WordWithReloc {
        word,
        reloc: Relocation {
            reloc_type: RelocType::CondBr19,
            symbol: sym,
            addend,
        },
    })
}

pub(crate) fn encode_br(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rn, _) = get_reg(operands, 0)?;
    // BR: 1101011 0000 11111 000000 Rn 00000
    let word = 0xd61f0000 | (rn << 5);
    Ok(EncodeResult::Word(word))
}

pub(crate) fn encode_blr(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rn, _) = get_reg(operands, 0)?;
    // BLR: 1101011 0001 11111 000000 Rn 00000
    let word = 0xd63f0000 | (rn << 5);
    Ok(EncodeResult::Word(word))
}

pub(crate) fn encode_ret(operands: &[Operand]) -> Result<EncodeResult, String> {
    let rn = if operands.is_empty() {
        30 // default to x30 (LR)
    } else {
        get_reg(operands, 0)?.0
    };
    // RET: 1101011 0010 11111 000000 Rn 00000
    let word = 0xd65f0000 | (rn << 5);
    Ok(EncodeResult::Word(word))
}

pub(crate) fn encode_cbz(operands: &[Operand], is_nz: bool) -> Result<EncodeResult, String> {
    let (rt, is_64) = get_reg(operands, 0)?;
    let (sym, addend) = get_symbol(operands, 1)?;
    let sf = sf_bit(is_64);
    let op = if is_nz { 1u32 } else { 0u32 };
    // CBZ/CBNZ: sf 011010 op imm19 Rt
    let word = (sf << 31) | (0b011010 << 25) | (op << 24) | rt;
    Ok(EncodeResult::WordWithReloc {
        word,
        reloc: Relocation {
            reloc_type: RelocType::CondBr19,
            symbol: sym,
            addend,
        },
    })
}

pub(crate) fn encode_tbz(operands: &[Operand], is_nz: bool) -> Result<EncodeResult, String> {
    let (rt, _) = get_reg(operands, 0)?;
    let bit = get_imm(operands, 1)?;
    let (sym, addend) = get_symbol(operands, 2)?;
    let b5 = ((bit as u32) >> 5) & 1;
    let b40 = (bit as u32) & 0x1F;
    let op = if is_nz { 1u32 } else { 0u32 };
    // TBZ/TBNZ: b5 011011 op b40 imm14 Rt
    let word = (b5 << 31) | (0b011011 << 25) | (op << 24) | (b40 << 19) | rt;
    Ok(EncodeResult::WordWithReloc {
        word,
        reloc: Relocation {
            reloc_type: RelocType::TstBr14,
            symbol: sym,
            addend,
        },
    })
}

// ── Additional conditional operations ────────────────────────────────────

/// Encode CNEG Rd, Rn, cond -> CSNEG Rd, Rn, Rn, invert(cond)
pub(crate) fn encode_cneg(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let cond = match operands.get(2) {
        Some(Operand::Cond(c)) => encode_cond(c).ok_or_else(|| format!("unknown condition: {}", c))?,
        _ => return Err("cneg: expected condition code as third operand".to_string()),
    };
    let sf = sf_bit(is_64);
    // Invert the condition (flip bit 0)
    let inv_cond = cond ^ 1;
    // CSNEG: sf 1 0 11010100 Rm cond 0 1 Rn Rd (with Rm = Rn)
    let word = (sf << 31) | (1 << 30) | (0b011010100 << 21) | (rn << 16)
        | (inv_cond << 12) | (0b01 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode CINC Rd, Rn, cond -> CSINC Rd, Rn, Rn, invert(cond)
pub(crate) fn encode_cinc(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let cond = match operands.get(2) {
        Some(Operand::Cond(c)) => encode_cond(c).ok_or_else(|| format!("unknown condition: {}", c))?,
        _ => return Err("cinc: expected condition code as third operand".to_string()),
    };
    let sf = sf_bit(is_64);
    let inv_cond = cond ^ 1;
    // CSINC: sf 0 0 11010100 Rm cond 0 1 Rn Rd (with Rm = Rn)
    let word = (sf << 31) | (0b011010100 << 21) | (rn << 16)
        | (inv_cond << 12) | (0b01 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode CINV Rd, Rn, cond -> CSINV Rd, Rn, Rn, invert(cond)
pub(crate) fn encode_cinv(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let cond = match operands.get(2) {
        Some(Operand::Cond(c)) => encode_cond(c).ok_or_else(|| format!("unknown condition: {}", c))?,
        _ => return Err("cinv: expected condition code as third operand".to_string()),
    };
    let sf = sf_bit(is_64);
    let inv_cond = cond ^ 1;
    // CSINV: sf 1 0 11010100 Rm cond 0 0 Rn Rd (with Rm = Rn)
    let word = (sf << 31) | (1 << 30) | (0b011010100 << 21) | (rn << 16)
        | (inv_cond << 12) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

#[cfg(test)]
mod encode_bl_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler (immediate form);
    //   algebraic.invariant (symbol reloc); algebraic.metamorphic (BL vs B);
    //   negative_error (arity / range / extra / bad kinds).
    // Evidence: src/backend/arm/assembler/README.md:5-14 gas-compat;
    //   README.md:220 Branches lists bl; README.md:247-253 Call26 ELF 283 for bl;
    //   encoder/mod.rs:1-7 32-bit AArch64 words; encoder/mod.rs:317 bl dispatch;
    //   encoder/mod.rs:49 R_AARCH64_CALL26; compare_branch.rs:186 BL 100101 imm26;
    //   ARM ARM Unconditional branch (immediate): bits[31:26]=100101, imm26=offset/4.
    // Stronger considered:
    //   - State machine: rejected — encode_bl is a pure function with no lifecycle
    //   - Algebraic round-trip via in-tree decoder: rejected — no BL decoder
    //   - encode_branch as differential sibling: rejected — different job (B/Jump26, no link)
    // Weaker available: algebraic.invariant (opcode/reloc fields), algebraic.metamorphic
    //   (bit-31 XOR vs encode_branch), negative_error (empty/unaligned/OOR/extra/modifier)
    // Differential: candidate=encode_bl, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=[Imm(imm)] <-> asm text `bl #imm`; [Symbol(s)|Label(s)|SymbolOffset(s,a)] <-> `bl s{+a}`

    use super::{encode_bl, encode_branch};
    use super::super::{EncodeResult, RelocType};
    use crate::backend::arm::assembler::parser::Operand;
    use proptest::prelude::*;
    use std::io::Write;
    use std::process::{Command, Stdio};

    const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
    const CASES: u32 = 1000;
    /// ARM ARM BL signed PC offset: ±128 MiB, multiple of 4.
    const IMM_MIN: i64 = -134_217_728; // -2^27
    const IMM_MAX: i64 = 134_217_724; // 2^27 - 4
    const BL_OPCODE: u32 = 0b100101 << 26; // 0x94000000

    fn cfg() -> ProptestConfig {
        ProptestConfig::with_cases(CASES)
    }

    fn parse_llvm_encoding(stdout: &str) -> Result<u32, String> {
        let marker = "encoding: [";
        let start = stdout
            .find(marker)
            .ok_or_else(|| format!("no encoding in stdout: {stdout}"))?;
        let rest = &stdout[start + marker.len()..];
        let end = rest
            .find(']')
            .ok_or_else(|| format!("no closing bracket: {stdout}"))?;
        let inner = &rest[..end];
        let mut bytes = [0u8; 4];
        let parts: Vec<&str> = inner.split(',').collect();
        if parts.len() != 4 {
            return Err(format!("expected 4 bytes, got {inner}"));
        }
        for (i, p) in parts.iter().enumerate() {
            let p = p.trim();
            let hex = p
                .strip_prefix("0x")
                .ok_or_else(|| format!("non-hex byte {p}"))?;
            bytes[i] = u8::from_str_radix(hex, 16).map_err(|e| e.to_string())?;
        }
        Ok(u32::from_le_bytes(bytes))
    }

    fn llvm_mc_word(asm: &str) -> Result<u32, String> {
        let mut child = Command::new(LLVM_MC)
            .args(["-triple=aarch64", "-show-encoding"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("spawn llvm-mc: {e}"))?;
        {
            let mut stdin = child.stdin.take().ok_or("llvm-mc stdin")?;
            stdin
                .write_all(asm.as_bytes())
                .map_err(|e| format!("write llvm-mc: {e}"))?;
            stdin
                .write_all(b"\n")
                .map_err(|e| format!("write llvm-mc: {e}"))?;
        }
        let out = child.wait_with_output().map_err(|e| format!("wait llvm-mc: {e}"))?;
        let stdout = String::from_utf8_lossy(&out.stdout);
        let stderr = String::from_utf8_lossy(&out.stderr);
        if !out.status.success() || stderr.contains("error:") {
            return Err(format!("llvm-mc error: {stderr}"));
        }
        parse_llvm_encoding(&stdout)
    }

    fn aligned_imm() -> impl Strategy<Value = i64> {
        prop_oneof![
            Just(IMM_MIN),
            Just(IMM_MIN + 4),
            Just(-8i64),
            Just(-4i64),
            Just(0i64),
            Just(4i64),
            Just(8i64),
            Just(IMM_MAX - 4),
            Just(IMM_MAX),
            (-(1i64 << 25)..(1i64 << 25)).prop_map(|k| k * 4),
        ]
    }

    fn invalid_imm() -> impl Strategy<Value = i64> {
        prop_oneof![
            Just(IMM_MIN - 4),
            Just(IMM_MIN - 1),
            Just(-1i64),
            Just(1i64),
            Just(2i64),
            Just(3i64),
            Just(5i64),
            Just(IMM_MAX + 1),
            Just(IMM_MAX + 4),
            Just(i64::MIN),
            Just(i64::MAX),
            (-(1i64 << 25) + 1..(1i64 << 25)).prop_map(|k| k * 4 + 1),
            (1i64..=1024).prop_map(|k| IMM_MAX + 4 + k * 4),
            (1i64..=1024).prop_map(|k| IMM_MIN - k * 4),
        ]
    }

    fn addend_strat() -> impl Strategy<Value = i64> {
        prop_oneof![
            Just(0i64),
            Just(-1i64),
            Just(4i64),
            Just(8i64),
            Just(-8i64),
            Just(-4i64),
            -4096i64..=4096i64,
        ]
    }

    fn is_call26(t: &RelocType) -> bool {
        matches!(t, RelocType::Call26)
    }

    fn is_jump26(t: &RelocType) -> bool {
        matches!(t, RelocType::Jump26)
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_bl_kat_llvm_mc_bl_imm0() {
        let want = 0x94000000u32;
        let mc = llvm_mc_word("bl #0").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [Operand::Imm(0)];
        match encode_bl(&ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for bl #0, got {:?}",
                want, other
            ),
        }
    }

    #[test]
    fn encode_bl_kat_llvm_mc_bl_imm4() {
        let want = 0x94000001u32;
        let mc = llvm_mc_word("bl #4").expect("llvm-mc KAT #4");
        assert_eq!(mc, want, "llvm-mc KAT #4 mapping broken");
        let ops = [Operand::Imm(4)];
        match encode_bl(&ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for bl #4, got {:?}",
                want, other
            ),
        }
    }

    #[test]
    fn encode_bl_kat_symbol_foo() {
        let ops = [Operand::Symbol("foo".into())];
        match encode_bl(&ops) {
            Ok(EncodeResult::WordWithReloc { word, reloc }) => {
                assert_eq!(word, BL_OPCODE);
                assert!(is_call26(&reloc.reloc_type));
                assert_eq!(reloc.reloc_type.elf_type(), 283);
                assert_eq!(reloc.symbol, "foo");
                assert_eq!(reloc.addend, 0);
            }
            other => panic!("expected WordWithReloc Call26 for bl foo, got {:?}", other),
        }
    }

    proptest! {
        #![proptest_config(cfg())]

        // Oracle: differential
        // Target: encoder.compare_branch.encode_bl
        #[test]
        fn encode_bl_diff_imm_llvm_mc(imm in aligned_imm()) {
            let asm = format!("bl #{}", imm);
            let ops = [Operand::Imm(imm)];
            let sut = match encode_bl(&ops) {
                Ok(EncodeResult::Word(w)) => w,
                other => {
                    return Err(TestCaseError::fail(format!(
                        "SUT rejected valid BL {}: {:?}",
                        asm, other
                    )));
                }
            };
            let mc = llvm_mc_word(&asm)
                .map_err(|e| TestCaseError::fail(format!(
                    "llvm-mc rejected valid BL {}: {}",
                    asm, e
                )))?;
            prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
        }

        // Oracle: algebraic.invariant
        // Target: encoder.compare_branch.encode_bl
        #[test]
        fn encode_bl_symbol_reloc(
            suffix in 0u32..=1000,
            addend in addend_strat(),
        ) {
            let sym = format!("labl{}", suffix);
            let check = |ops: &[Operand], expect_addend: i64, tag: &str| {
                match encode_bl(ops) {
                    Ok(EncodeResult::WordWithReloc { word, reloc }) => {
                        prop_assert_eq!(word, BL_OPCODE, "{} word", tag);
                        prop_assert!(
                            is_call26(&reloc.reloc_type),
                            "{} expected Call26, got {:?}",
                            tag, reloc.reloc_type
                        );
                        prop_assert_eq!(
                            reloc.reloc_type.elf_type(),
                            283u32,
                            "{} ELF type", tag
                        );
                        prop_assert_eq!(&reloc.symbol, &sym, "{} symbol", tag);
                        prop_assert_eq!(reloc.addend, expect_addend, "{} addend", tag);
                        prop_assert_eq!(word >> 26, 0b100101u32, "{} opcode", tag);
                        prop_assert_eq!(word & 0x03ff_ffff, 0, "{} imm26 must be 0", tag);
                        Ok(())
                    }
                    other => Err(TestCaseError::fail(format!(
                        "{} expected WordWithReloc, got {:?}",
                        tag, other
                    ))),
                }
            };
            check(&[Operand::Symbol(sym.clone())], 0, "Symbol")?;
            check(&[Operand::Label(sym.clone())], 0, "Label")?;
            check(
                &[Operand::SymbolOffset(sym.clone(), addend)],
                addend,
                "SymbolOffset",
            )?;
        }

        // Oracle: algebraic.metamorphic
        // Target: encoder.compare_branch.encode_bl
        #[test]
        fn encode_bl_meta_vs_b(
            suffix in 0u32..=1000,
            addend in addend_strat(),
        ) {
            let sym = format!("labl{}", suffix);
            let ops = [Operand::SymbolOffset(sym.clone(), addend)];
            let bl = encode_bl(&ops).map_err(|e| TestCaseError::fail(e))?;
            let b = encode_branch(&ops).map_err(|e| TestCaseError::fail(e))?;
            match (bl, b) {
                (
                    EncodeResult::WordWithReloc { word: w_bl, reloc: r_bl },
                    EncodeResult::WordWithReloc { word: w_b, reloc: r_b },
                ) => {
                    prop_assert_eq!(
                        w_bl ^ w_b,
                        1u32 << 31,
                        "BL XOR B must be bit 31 (bl={:#010x} b={:#010x})",
                        w_bl, w_b
                    );
                    prop_assert!(is_call26(&r_bl.reloc_type), "BL reloc Call26");
                    prop_assert!(is_jump26(&r_b.reloc_type), "B reloc Jump26");
                    prop_assert_eq!(&r_bl.symbol, &sym);
                    prop_assert_eq!(&r_b.symbol, &sym);
                    prop_assert_eq!(r_bl.addend, addend);
                    prop_assert_eq!(r_b.addend, addend);
                }
                other => {
                    return Err(TestCaseError::fail(format!(
                        "expected WordWithReloc pair, got {:?}",
                        other
                    )));
                }
            }
        }

        // Oracle: algebraic.invariant
        // Target: encoder.compare_branch.encode_bl
        #[test]
        fn encode_bl_word_layout(suffix in 0u32..=1000) {
            let sym = format!("labl{}", suffix);
            match encode_bl(&[Operand::Symbol(sym)]) {
                Ok(EncodeResult::WordWithReloc { word, reloc }) => {
                    prop_assert_eq!((word >> 26) & 0x3f, 0b100101u32);
                    prop_assert_eq!(word & 0x03ff_ffff, 0);
                    prop_assert!(is_call26(&reloc.reloc_type));
                }
                other => {
                    return Err(TestCaseError::fail(format!(
                        "expected WordWithReloc, got {:?}",
                        other
                    )));
                }
            }
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_bl
        #[test]
        fn encode_bl_neg_arity(_dummy in 0u32..=0) {
            prop_assert!(
                encode_bl(&[]).is_err(),
                "bare bl must Err (llvm-mc: too few operands)"
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_bl
        #[test]
        fn encode_bl_neg_imm_unaligned_oor(imm in invalid_imm()) {
            let ops = [Operand::Imm(imm)];
            prop_assert!(
                encode_bl(&ops).is_err(),
                "BL offset {} is unaligned or out of [-134217728, 134217724] and must Err",
                imm
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_bl
        #[test]
        fn encode_bl_neg_extra_operand(
            suffix in 0u32..=1000,
            which in 0u32..=3,
        ) {
            let sym = format!("labl{}", suffix);
            let extra = match which {
                0 => Operand::Reg("x0".into()),
                1 => Operand::Imm(0),
                2 => Operand::Symbol("bar".into()),
                _ => Operand::Mem { base: "x1".into(), offset: 0 },
            };
            let ops = [Operand::Symbol(sym), extra];
            prop_assert!(
                encode_bl(&ops).is_err(),
                "bl <label>, extra (which={}) must Err (llvm-mc: invalid operand)",
                which
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_bl
        #[test]
        fn encode_bl_neg_bad_operand(which in 0u32..=5) {
            let bad = match which {
                0 => Operand::Mem { base: "x0".into(), offset: 0 },
                1 => Operand::Shift { kind: "lsl".into(), amount: 0 },
                2 => Operand::Extend { kind: "sxtw".into(), amount: 0 },
                3 => Operand::RegArrangement { reg: "v0".into(), arrangement: "16b".into() },
                4 => Operand::Modifier { kind: "lo12".into(), symbol: "foo".into() },
                _ => Operand::ModifierOffset {
                    kind: "lo12".into(),
                    symbol: "foo".into(),
                    offset: 8,
                },
            };
            prop_assert!(
                encode_bl(&[bad]).is_err(),
                "BL does not take Mem/Shift/Extend/RegArrangement/Modifier (which={})",
                which
            );
        }

        // Oracle: algebraic.invariant (coverage sweep: get_symbol parser-misclassification arms)
        // Target: encoder.compare_branch.encode_bl
        #[test]
        fn encode_bl_symbol_misclassified(
            which in 0u32..=2,
            name in prop::sample::select(vec!["eq", "ne", "lt", "gt", "sy", "ish", "st", "ld"]),
        ) {
            let op = match which {
                0 => Operand::Reg(name.to_string()),
                1 => Operand::Cond(name.to_string()),
                _ => Operand::Barrier(name.to_string()),
            };
            match encode_bl(&[op]) {
                Ok(EncodeResult::WordWithReloc { word, reloc }) => {
                    prop_assert_eq!(word, BL_OPCODE);
                    prop_assert!(is_call26(&reloc.reloc_type));
                    prop_assert_eq!(&reloc.symbol, name);
                    prop_assert_eq!(reloc.addend, 0);
                }
                other => {
                    return Err(TestCaseError::fail(format!(
                        "parser-misclassified {} as BL target must be a Call26 reloc, got {:?}",
                        name, other
                    )));
                }
            }
        }
    }

    #[test]
    fn test_encode_bl_regression_imm_offset() {
        let ops = [Operand::Imm(-134_217_728)];
        match encode_bl(&ops) {
            Ok(EncodeResult::Word(w)) => {
                let mc = llvm_mc_word("bl #-134217728").expect("llvm-mc");
                assert_eq!(w, mc, "bl #-134217728 must match llvm-mc");
            }
            other => panic!(
                "bl #-134217728 must encode as Word matching llvm-mc, got {:?}",
                other
            ),
        }
    }

    #[test]
    fn test_encode_bl_regression_extra_operand() {
        let ops = [Operand::Symbol("labl0".into()), Operand::Reg("x0".into())];
        assert!(
            encode_bl(&ops).is_err(),
            "bl labl0, x0 must Err; llvm-mc rejects a second operand"
        );
    }

    #[test]
    fn test_encode_bl_regression_modifier() {
        let ops = [Operand::Modifier {
            kind: "lo12".into(),
            symbol: "foo".into(),
        }];
        assert!(
            encode_bl(&ops).is_err(),
            "bl :lo12:foo must Err; BL does not take :lo12: modifiers"
        );
    }
}

#[cfg(test)]
mod encode_blr_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler (register form);
    //   algebraic.invariant (word layout); algebraic.metamorphic (BLR vs BR);
    //   negative_error (arity / W-reg / extra / bad kinds / wrong class).
    // Evidence: src/backend/arm/assembler/README.md:5-14 gas-compat;
    //   README.md:220 Branches lists blr; encoder/mod.rs:1-7 32-bit AArch64 words;
    //   encoder/mod.rs:319 blr dispatch; compare_branch.rs:221 BLR 1101011 0001 11111 Rn;
    //   codegen/calls.rs:233 blr x17; ARM ARM Unconditional branch (register):
    //   bits[31:25]=1101011 opc=0001 op2=11111 op3=000000 Rn[9:5] op4=00000.
    // Stronger considered:
    //   - State machine: rejected — encode_blr is a pure function with no lifecycle
    //   - Algebraic round-trip via in-tree decoder: rejected — no BLR decoder
    //   - encode_br as differential sibling: rejected — different job (BR, no link)
    // Weaker available: algebraic.invariant (opcode/Rn fields), algebraic.metamorphic
    //   (bit-21 XOR vs encode_br), negative_error (empty/W/extra/non-GPR/SP/FP)
    // Differential: candidate=encode_blr, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=[Reg("xN"|"xzr"|"lr")] <-> asm text `blr xN`

    use super::{encode_blr, encode_br};
    use super::super::EncodeResult;
    use crate::backend::arm::assembler::parser::Operand;
    use proptest::prelude::*;
    use std::io::Write;
    use std::process::{Command, Stdio};

    const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
    const CASES: u32 = 1000;
    const BLR_FIXED: u32 = 0xd63f0000;

    fn cfg() -> ProptestConfig {
        ProptestConfig::with_cases(CASES)
    }

    fn parse_llvm_encoding(stdout: &str) -> Result<u32, String> {
        let marker = "encoding: [";
        let start = stdout
            .find(marker)
            .ok_or_else(|| format!("no encoding in stdout: {stdout}"))?;
        let rest = &stdout[start + marker.len()..];
        let end = rest
            .find(']')
            .ok_or_else(|| format!("no closing bracket: {stdout}"))?;
        let inner = &rest[..end];
        let mut bytes = [0u8; 4];
        let parts: Vec<&str> = inner.split(',').collect();
        if parts.len() != 4 {
            return Err(format!("expected 4 bytes, got {inner}"));
        }
        for (i, p) in parts.iter().enumerate() {
            let p = p.trim();
            let hex = p
                .strip_prefix("0x")
                .ok_or_else(|| format!("non-hex byte {p}"))?;
            bytes[i] = u8::from_str_radix(hex, 16).map_err(|e| e.to_string())?;
        }
        Ok(u32::from_le_bytes(bytes))
    }

    fn llvm_mc_word(asm: &str) -> Result<u32, String> {
        let mut child = Command::new(LLVM_MC)
            .args(["-triple=aarch64", "-show-encoding"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("spawn llvm-mc: {e}"))?;
        {
            let mut stdin = child.stdin.take().ok_or("llvm-mc stdin")?;
            stdin
                .write_all(asm.as_bytes())
                .map_err(|e| format!("write llvm-mc: {e}"))?;
            stdin
                .write_all(b"\n")
                .map_err(|e| format!("write llvm-mc: {e}"))?;
        }
        let out = child.wait_with_output().map_err(|e| format!("wait llvm-mc: {e}"))?;
        let stdout = String::from_utf8_lossy(&out.stdout);
        let stderr = String::from_utf8_lossy(&out.stderr);
        if !out.status.success() || stderr.contains("error:") {
            return Err(format!("llvm-mc error: {stderr}"));
        }
        parse_llvm_encoding(&stdout)
    }

    /// n=0..30 -> xN; 31 -> xzr; 32 -> lr. Bounds x0 / x17 (codegen) / x30 / xzr / lr forced.
    fn x_name(n: u32) -> String {
        match n {
            31 => "xzr".to_string(),
            32 => "lr".to_string(),
            n => format!("x{}", n.min(30)),
        }
    }

    /// n=0..30 -> wN; 31 -> wzr; 32 -> wsp.
    fn w_name(n: u32) -> String {
        match n {
            31 => "wzr".to_string(),
            32 => "wsp".to_string(),
            n => format!("w{}", n.min(30)),
        }
    }

    fn x_name_strat() -> impl Strategy<Value = String> {
        prop_oneof![
            Just("x0".to_string()),
            Just("x17".to_string()),
            Just("x30".to_string()),
            Just("xzr".to_string()),
            Just("lr".to_string()),
            Just("X0".to_string()),
            (0u32..=32).prop_map(x_name),
        ]
    }

    fn extra_operand(which: u32) -> Operand {
        match which {
            0 => Operand::Reg("x1".into()),
            1 => Operand::Imm(0),
            2 => Operand::Symbol("bar".into()),
            _ => Operand::Mem {
                base: "x1".into(),
                offset: 0,
            },
        }
    }

    fn bad_operand(which: u32) -> Operand {
        match which {
            0 => Operand::Imm(0),
            1 => Operand::Mem {
                base: "x0".into(),
                offset: 0,
            },
            2 => Operand::Shift {
                kind: "lsl".into(),
                amount: 0,
            },
            3 => Operand::Extend {
                kind: "sxtw".into(),
                amount: 0,
            },
            4 => Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "16b".into(),
            },
            5 => Operand::Modifier {
                kind: "lo12".into(),
                symbol: "foo".into(),
            },
            6 => Operand::Symbol("foo".into()),
            _ => Operand::Label("foo".into()),
        }
    }

    fn wrong_reg_name(which: u32, n: u32) -> String {
        let n = n.min(31);
        match which {
            0 => "sp".to_string(),
            1 => "wsp".to_string(),
            2 => format!("d{}", n),
            3 => format!("s{}", n),
            4 => format!("q{}", n),
            5 => format!("v{}", n),
            6 => format!("h{}", n),
            7 => format!("b{}", n),
            _ => match n {
                0 => "x32".to_string(),
                1 => "w32".to_string(),
                2 => "foo".to_string(),
                3 => "".to_string(),
                4 => "r0".to_string(),
                5 => "x".to_string(),
                6 => "x-1".to_string(),
                _ => "x99".to_string(),
            },
        }
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_blr_kat_llvm_mc_blr_x0() {
        let want = 0xd63f0000u32;
        let mc = llvm_mc_word("blr x0").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [Operand::Reg("x0".into())];
        match encode_blr(&ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for blr x0, got {:?}",
                want, other
            ),
        }
    }

    #[test]
    fn encode_blr_kat_llvm_mc_blr_x17() {
        let want = 0xd63f0220u32;
        let mc = llvm_mc_word("blr x17").expect("llvm-mc KAT x17");
        assert_eq!(mc, want, "llvm-mc KAT x17 mapping broken");
        let ops = [Operand::Reg("x17".into())];
        match encode_blr(&ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for blr x17, got {:?}",
                want, other
            ),
        }
    }

    proptest! {
        #![proptest_config(cfg())]

        // Oracle: differential
        // Target: encoder.compare_branch.encode_blr
        #[test]
        fn encode_blr_diff_xn_llvm_mc(name in x_name_strat()) {
            let asm = format!("blr {}", name);
            let ops = [Operand::Reg(name.clone())];
            let sut = match encode_blr(&ops) {
                Ok(EncodeResult::Word(w)) => w,
                other => {
                    return Err(TestCaseError::fail(format!(
                        "SUT rejected valid BLR {}: {:?}",
                        asm, other
                    )));
                }
            };
            let mc = llvm_mc_word(&asm)
                .map_err(|e| TestCaseError::fail(format!(
                    "llvm-mc rejected valid BLR {}: {}",
                    asm, e
                )))?;
            prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
        }

        // Oracle: algebraic.invariant
        // Target: encoder.compare_branch.encode_blr
        #[test]
        fn encode_blr_word_layout(n in 0u32..=31) {
            let name = if n == 31 {
                "xzr".to_string()
            } else {
                format!("x{}", n)
            };
            match encode_blr(&[Operand::Reg(name)]) {
                Ok(EncodeResult::Word(w)) => {
                    prop_assert_eq!(w, BLR_FIXED | (n << 5));
                    prop_assert_eq!(w >> 10, BLR_FIXED >> 10, "bits[31:10] fixed");
                    prop_assert_eq!(w & 0x1F, 0u32, "op4 [4:0] must be 0");
                    prop_assert_eq!((w >> 5) & 0x1F, n, "Rn [9:5]");
                    prop_assert_eq!(w >> 25, 0b1101011u32, "bits[31:25]");
                    prop_assert_eq!((w >> 21) & 0xF, 0b0001u32, "opc [24:21]");
                }
                other => {
                    return Err(TestCaseError::fail(format!(
                        "expected Word, got {:?}",
                        other
                    )));
                }
            }
        }

        // Oracle: algebraic.metamorphic
        // Target: encoder.compare_branch.encode_blr
        #[test]
        fn encode_blr_meta_vs_br(n in 0u32..=31) {
            let name = if n == 31 {
                "xzr".to_string()
            } else {
                format!("x{}", n)
            };
            let ops = [Operand::Reg(name)];
            let blr = encode_blr(&ops).map_err(|e| TestCaseError::fail(e))?;
            let br = encode_br(&ops).map_err(|e| TestCaseError::fail(e))?;
            match (blr, br) {
                (EncodeResult::Word(w_blr), EncodeResult::Word(w_br)) => {
                    prop_assert_eq!(
                        w_blr ^ w_br,
                        1u32 << 21,
                        "BLR XOR BR must be bit 21 (blr={:#010x} br={:#010x})",
                        w_blr, w_br
                    );
                }
                other => {
                    return Err(TestCaseError::fail(format!(
                        "expected Word pair, got {:?}",
                        other
                    )));
                }
            }
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_blr
        #[test]
        fn encode_blr_neg_arity(_dummy in 0u32..=0) {
            prop_assert!(
                encode_blr(&[]).is_err(),
                "bare blr must Err (llvm-mc: too few operands)"
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_blr
        #[test]
        fn encode_blr_neg_w_reg(n in 0u32..=32) {
            let name = w_name(n);
            let ops = [Operand::Reg(name.clone())];
            prop_assert!(
                encode_blr(&ops).is_err(),
                "blr {} must Err (llvm-mc rejects W-form Rn; ARM ARM Rn is Xn)",
                name
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_blr
        #[test]
        fn encode_blr_neg_extra_operand(n in 0u32..=30, which in 0u32..=3) {
            let extra = extra_operand(which);
            let ops = [Operand::Reg(format!("x{}", n)), extra];
            prop_assert!(
                encode_blr(&ops).is_err(),
                "blr x{}, extra (which={}) must Err (llvm-mc: invalid operand)",
                n, which
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_blr
        #[test]
        fn encode_blr_neg_bad_operand(which in 0u32..=7) {
            let bad = bad_operand(which);
            prop_assert!(
                encode_blr(&[bad]).is_err(),
                "BLR does not take Imm/Mem/Shift/Extend/RegArrangement/Modifier/Symbol/Label (which={})",
                which
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_blr
        #[test]
        fn encode_blr_neg_wrong_reg_class(which in 0u32..=8, n in 0u32..=31) {
            let name = wrong_reg_name(which, n);
            let ops = [Operand::Reg(name.clone())];
            prop_assert!(
                encode_blr(&ops).is_err(),
                "blr {} must Err (llvm-mc rejects SP / FP / invalid names)",
                name
            );
        }

        // Oracle: negative_error (coverage sweep: get_reg parse_reg_num None arm)
        // Target: encoder.compare_branch.encode_blr
        #[test]
        fn encode_blr_neg_invalid_name(which in 0u32..=7) {
            let name = match which {
                0 => "x32",
                1 => "w32",
                2 => "foo",
                3 => "",
                4 => "r0",
                5 => "x",
                6 => "x-1",
                _ => "x99",
            };
            let ops = [Operand::Reg(name.to_string())];
            prop_assert!(
                encode_blr(&ops).is_err(),
                "blr {} must Err (not a valid register name)",
                name
            );
        }
    }

    #[test]
    fn test_encode_blr_regression_w_reg() {
        let ops = [Operand::Reg("w0".into())];
        assert!(
            encode_blr(&ops).is_err(),
            "blr w0 must Err; llvm-mc rejects W-form Rn"
        );
    }

    #[test]
    fn test_encode_blr_regression_extra_operand() {
        let ops = [Operand::Reg("x0".into()), Operand::Reg("x1".into())];
        assert!(
            encode_blr(&ops).is_err(),
            "blr x0, x1 must Err; llvm-mc rejects a second operand"
        );
    }

    #[test]
    fn test_encode_blr_regression_sp() {
        let ops = [Operand::Reg("sp".into())];
        assert!(
            encode_blr(&ops).is_err(),
            "blr sp must Err; llvm-mc rejects SP (register 31 is XZR)"
        );
    }

    #[test]
    fn test_encode_blr_regression_fp_reg() {
        let ops = [Operand::Reg("d0".into())];
        assert!(
            encode_blr(&ops).is_err(),
            "blr d0 must Err; llvm-mc rejects FP/SIMD Rn"
        );
    }
}

#[cfg(test)]
mod encode_br_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler (register form);
    //   algebraic.invariant (word layout); algebraic.metamorphic (BR vs BLR);
    //   negative_error (arity / W-reg / extra / bad kinds / wrong class).
    // Evidence: src/backend/arm/assembler/README.md:5-14 gas-compat;
    //   README.md:220 Branches lists br; encoder/mod.rs:1-7 32-bit AArch64 words;
    //   encoder/mod.rs:318 br dispatch; compare_branch.rs:214 BR 1101011 0000 11111 Rn;
    //   codegen/emit.rs:1760 br x0; emit.rs:1808 br x17;
    //   ARM ARM Unconditional branch (register):
    //   bits[31:25]=1101011 opc=0000 op2=11111 op3=000000 Rn[9:5] op4=00000.
    // Stronger considered:
    //   - State machine: rejected — encode_br is a pure function with no lifecycle
    //   - Algebraic round-trip via in-tree decoder: rejected — no BR decoder
    //   - encode_blr as differential sibling: rejected — different job (BLR, with link)
    // Weaker available: algebraic.invariant (opcode/Rn fields), algebraic.metamorphic
    //   (bit-21 XOR vs encode_blr), negative_error (empty/W/extra/non-GPR/SP/FP)
    // Differential: candidate=encode_br, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=[Reg("xN"|"xzr"|"lr")] <-> asm text `br xN`

    use super::{encode_blr, encode_br};
    use super::super::EncodeResult;
    use crate::backend::arm::assembler::parser::Operand;
    use proptest::prelude::*;
    use std::io::Write;
    use std::process::{Command, Stdio};

    const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
    const CASES: u32 = 1000;
    const BR_FIXED: u32 = 0xd61f0000;

    fn cfg() -> ProptestConfig {
        ProptestConfig::with_cases(CASES)
    }

    fn parse_llvm_encoding(stdout: &str) -> Result<u32, String> {
        let marker = "encoding: [";
        let start = stdout
            .find(marker)
            .ok_or_else(|| format!("no encoding in stdout: {stdout}"))?;
        let rest = &stdout[start + marker.len()..];
        let end = rest
            .find(']')
            .ok_or_else(|| format!("no closing bracket: {stdout}"))?;
        let inner = &rest[..end];
        let mut bytes = [0u8; 4];
        let parts: Vec<&str> = inner.split(',').collect();
        if parts.len() != 4 {
            return Err(format!("expected 4 bytes, got {inner}"));
        }
        for (i, p) in parts.iter().enumerate() {
            let p = p.trim();
            let hex = p
                .strip_prefix("0x")
                .ok_or_else(|| format!("non-hex byte {p}"))?;
            bytes[i] = u8::from_str_radix(hex, 16).map_err(|e| e.to_string())?;
        }
        Ok(u32::from_le_bytes(bytes))
    }

    fn llvm_mc_word(asm: &str) -> Result<u32, String> {
        let mut child = Command::new(LLVM_MC)
            .args(["-triple=aarch64", "-show-encoding"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("spawn llvm-mc: {e}"))?;
        {
            let mut stdin = child.stdin.take().ok_or("llvm-mc stdin")?;
            stdin
                .write_all(asm.as_bytes())
                .map_err(|e| format!("write llvm-mc: {e}"))?;
            stdin
                .write_all(b"\n")
                .map_err(|e| format!("write llvm-mc: {e}"))?;
        }
        let out = child.wait_with_output().map_err(|e| format!("wait llvm-mc: {e}"))?;
        let stdout = String::from_utf8_lossy(&out.stdout);
        let stderr = String::from_utf8_lossy(&out.stderr);
        if !out.status.success() || stderr.contains("error:") {
            return Err(format!("llvm-mc error: {stderr}"));
        }
        parse_llvm_encoding(&stdout)
    }

    /// n=0..30 -> xN; 31 -> xzr; 32 -> lr. Bounds x0 / x17 (codegen) / x30 / xzr / lr forced.
    fn x_name(n: u32) -> String {
        match n {
            31 => "xzr".to_string(),
            32 => "lr".to_string(),
            n => format!("x{}", n.min(30)),
        }
    }

    /// n=0..30 -> wN; 31 -> wzr; 32 -> wsp.
    fn w_name(n: u32) -> String {
        match n {
            31 => "wzr".to_string(),
            32 => "wsp".to_string(),
            n => format!("w{}", n.min(30)),
        }
    }

    fn x_name_strat() -> impl Strategy<Value = String> {
        prop_oneof![
            Just("x0".to_string()),
            Just("x17".to_string()),
            Just("x30".to_string()),
            Just("xzr".to_string()),
            Just("lr".to_string()),
            Just("X0".to_string()),
            (0u32..=32).prop_map(x_name),
        ]
    }

    fn extra_operand(which: u32) -> Operand {
        match which {
            0 => Operand::Reg("x1".into()),
            1 => Operand::Imm(0),
            2 => Operand::Symbol("bar".into()),
            _ => Operand::Mem {
                base: "x1".into(),
                offset: 0,
            },
        }
    }

    fn bad_operand(which: u32) -> Operand {
        match which {
            0 => Operand::Imm(0),
            1 => Operand::Mem {
                base: "x0".into(),
                offset: 0,
            },
            2 => Operand::Shift {
                kind: "lsl".into(),
                amount: 0,
            },
            3 => Operand::Extend {
                kind: "sxtw".into(),
                amount: 0,
            },
            4 => Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "16b".into(),
            },
            5 => Operand::Modifier {
                kind: "lo12".into(),
                symbol: "foo".into(),
            },
            6 => Operand::Symbol("foo".into()),
            _ => Operand::Label("foo".into()),
        }
    }

    fn wrong_reg_name(which: u32, n: u32) -> String {
        let n = n.min(31);
        match which {
            0 => "sp".to_string(),
            1 => "wsp".to_string(),
            2 => format!("d{}", n),
            3 => format!("s{}", n),
            4 => format!("q{}", n),
            5 => format!("v{}", n),
            6 => format!("h{}", n),
            7 => format!("b{}", n),
            _ => match n {
                0 => "x32".to_string(),
                1 => "w32".to_string(),
                2 => "foo".to_string(),
                3 => "".to_string(),
                4 => "r0".to_string(),
                5 => "x".to_string(),
                6 => "x-1".to_string(),
                _ => "x99".to_string(),
            },
        }
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_br_kat_llvm_mc_br_x0() {
        let want = 0xd61f0000u32;
        let mc = llvm_mc_word("br x0").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [Operand::Reg("x0".into())];
        match encode_br(&ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for br x0, got {:?}",
                want, other
            ),
        }
    }

    #[test]
    fn encode_br_kat_llvm_mc_br_x17() {
        let want = 0xd61f0220u32;
        let mc = llvm_mc_word("br x17").expect("llvm-mc KAT x17");
        assert_eq!(mc, want, "llvm-mc KAT x17 mapping broken");
        let ops = [Operand::Reg("x17".into())];
        match encode_br(&ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for br x17, got {:?}",
                want, other
            ),
        }
    }

    proptest! {
        #![proptest_config(cfg())]

        // Oracle: differential
        // Target: encoder.compare_branch.encode_br
        #[test]
        fn encode_br_diff_xn_llvm_mc(name in x_name_strat()) {
            let asm = format!("br {}", name);
            let ops = [Operand::Reg(name.clone())];
            let sut = match encode_br(&ops) {
                Ok(EncodeResult::Word(w)) => w,
                other => {
                    return Err(TestCaseError::fail(format!(
                        "SUT rejected valid BR {}: {:?}",
                        asm, other
                    )));
                }
            };
            let mc = llvm_mc_word(&asm)
                .map_err(|e| TestCaseError::fail(format!(
                    "llvm-mc rejected valid BR {}: {}",
                    asm, e
                )))?;
            prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
        }

        // Oracle: algebraic.invariant
        // Target: encoder.compare_branch.encode_br
        #[test]
        fn encode_br_word_layout(n in 0u32..=31) {
            let name = if n == 31 {
                "xzr".to_string()
            } else {
                format!("x{}", n)
            };
            match encode_br(&[Operand::Reg(name)]) {
                Ok(EncodeResult::Word(w)) => {
                    prop_assert_eq!(w, BR_FIXED | (n << 5));
                    prop_assert_eq!(w >> 10, BR_FIXED >> 10, "bits[31:10] fixed");
                    prop_assert_eq!(w & 0x1F, 0u32, "op4 [4:0] must be 0");
                    prop_assert_eq!((w >> 5) & 0x1F, n, "Rn [9:5]");
                    prop_assert_eq!(w >> 25, 0b1101011u32, "bits[31:25]");
                    prop_assert_eq!((w >> 21) & 0xF, 0b0000u32, "opc [24:21]");
                }
                other => {
                    return Err(TestCaseError::fail(format!(
                        "expected Word, got {:?}",
                        other
                    )));
                }
            }
        }

        // Oracle: algebraic.metamorphic
        // Target: encoder.compare_branch.encode_br
        #[test]
        fn encode_br_meta_vs_blr(n in 0u32..=31) {
            let name = if n == 31 {
                "xzr".to_string()
            } else {
                format!("x{}", n)
            };
            let ops = [Operand::Reg(name)];
            let br = encode_br(&ops).map_err(|e| TestCaseError::fail(e))?;
            let blr = encode_blr(&ops).map_err(|e| TestCaseError::fail(e))?;
            match (br, blr) {
                (EncodeResult::Word(w_br), EncodeResult::Word(w_blr)) => {
                    prop_assert_eq!(
                        w_br ^ w_blr,
                        1u32 << 21,
                        "BR XOR BLR must be bit 21 (br={:#010x} blr={:#010x})",
                        w_br, w_blr
                    );
                }
                other => {
                    return Err(TestCaseError::fail(format!(
                        "expected Word pair, got {:?}",
                        other
                    )));
                }
            }
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_br
        #[test]
        fn encode_br_neg_arity(_dummy in 0u32..=0) {
            prop_assert!(
                encode_br(&[]).is_err(),
                "bare br must Err (llvm-mc: too few operands)"
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_br
        #[test]
        fn encode_br_neg_w_reg(n in 0u32..=32) {
            let name = w_name(n);
            let ops = [Operand::Reg(name.clone())];
            prop_assert!(
                encode_br(&ops).is_err(),
                "br {} must Err (llvm-mc rejects W-form Rn; ARM ARM Rn is Xn)",
                name
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_br
        #[test]
        fn encode_br_neg_extra_operand(n in 0u32..=30, which in 0u32..=3) {
            let extra = extra_operand(which);
            let ops = [Operand::Reg(format!("x{}", n)), extra];
            prop_assert!(
                encode_br(&ops).is_err(),
                "br x{}, extra (which={}) must Err (llvm-mc: invalid operand)",
                n, which
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_br
        #[test]
        fn encode_br_neg_bad_operand(which in 0u32..=7) {
            let bad = bad_operand(which);
            prop_assert!(
                encode_br(&[bad]).is_err(),
                "BR does not take Imm/Mem/Shift/Extend/RegArrangement/Modifier/Symbol/Label (which={})",
                which
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_br
        #[test]
        fn encode_br_neg_wrong_reg_class(which in 0u32..=8, n in 0u32..=31) {
            let name = wrong_reg_name(which, n);
            let ops = [Operand::Reg(name.clone())];
            prop_assert!(
                encode_br(&ops).is_err(),
                "br {} must Err (llvm-mc rejects SP / FP / invalid names)",
                name
            );
        }

        // Oracle: negative_error (coverage sweep: get_reg parse_reg_num None arm)
        // Target: encoder.compare_branch.encode_br
        #[test]
        fn encode_br_neg_invalid_name(which in 0u32..=7) {
            let name = match which {
                0 => "x32",
                1 => "w32",
                2 => "foo",
                3 => "",
                4 => "r0",
                5 => "x",
                6 => "x-1",
                _ => "x99",
            };
            let ops = [Operand::Reg(name.to_string())];
            prop_assert!(
                encode_br(&ops).is_err(),
                "br {} must Err (not a valid register name)",
                name
            );
        }
    }

    #[test]
    fn test_encode_br_regression_w_reg() {
        let ops = [Operand::Reg("w0".into())];
        assert!(
            encode_br(&ops).is_err(),
            "br w0 must Err; llvm-mc rejects W-form Rn"
        );
    }

    #[test]
    fn test_encode_br_regression_extra_operand() {
        let ops = [Operand::Reg("x0".into()), Operand::Reg("x1".into())];
        assert!(
            encode_br(&ops).is_err(),
            "br x0, x1 must Err; llvm-mc rejects a second operand"
        );
    }

    #[test]
    fn test_encode_br_regression_sp() {
        let ops = [Operand::Reg("sp".into())];
        assert!(
            encode_br(&ops).is_err(),
            "br sp must Err; llvm-mc rejects SP (register 31 is XZR)"
        );
    }

    #[test]
    fn test_encode_br_regression_fp_reg() {
        let ops = [Operand::Reg("d0".into())];
        assert!(
            encode_br(&ops).is_err(),
            "br d0 must Err; llvm-mc rejects FP/SIMD Rn"
        );
    }
}

#[cfg(test)]
mod encode_branch_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler (immediate form);
    //   algebraic.invariant (symbol reloc); algebraic.metamorphic (B vs BL);
    //   negative_error (arity / range / extra / bad kinds).
    // Evidence: src/backend/arm/assembler/README.md:5-14 gas-compat;
    //   README.md:220 Branches lists b; README.md:254 Jump26 ELF 282 for b;
    //   encoder/mod.rs:1-7 32-bit AArch64 words; encoder/mod.rs:316 b dispatch;
    //   encoder/mod.rs:51-52 R_AARCH64_JUMP26; compare_branch.rs:173 B 000101 imm26;
    //   ARM ARM Unconditional branch (immediate): bits[31:26]=000101, imm26=offset/4.
    // Stronger considered:
    //   - State machine: rejected — encode_branch is a pure function with no lifecycle
    //   - Algebraic round-trip via in-tree decoder: rejected — no B decoder
    //   - encode_bl as differential sibling: rejected — different job (BL/Call26, with link)
    // Weaker available: algebraic.invariant (opcode/reloc fields), algebraic.metamorphic
    //   (bit-31 XOR vs encode_bl), negative_error (empty/unaligned/OOR/extra/modifier)
    // Differential: candidate=encode_branch, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=[Imm(imm)] <-> asm text `b #imm`; [Symbol(s)|Label(s)|SymbolOffset(s,a)] <-> `b s{+a}`

    use super::{encode_bl, encode_branch};
    use super::super::{EncodeResult, RelocType};
    use crate::backend::arm::assembler::parser::Operand;
    use proptest::prelude::*;
    use std::io::Write;
    use std::process::{Command, Stdio};

    const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
    const CASES: u32 = 1000;
    /// ARM ARM B signed PC offset: ±128 MiB, multiple of 4.
    const IMM_MIN: i64 = -134_217_728; // -2^27
    const IMM_MAX: i64 = 134_217_724; // 2^27 - 4
    const B_OPCODE: u32 = 0b000101 << 26; // 0x14000000

    fn cfg() -> ProptestConfig {
        ProptestConfig::with_cases(CASES)
    }

    fn parse_llvm_encoding(stdout: &str) -> Result<u32, String> {
        let marker = "encoding: [";
        let start = stdout
            .find(marker)
            .ok_or_else(|| format!("no encoding in stdout: {stdout}"))?;
        let rest = &stdout[start + marker.len()..];
        let end = rest
            .find(']')
            .ok_or_else(|| format!("no closing bracket: {stdout}"))?;
        let inner = &rest[..end];
        let mut bytes = [0u8; 4];
        let parts: Vec<&str> = inner.split(',').collect();
        if parts.len() != 4 {
            return Err(format!("expected 4 bytes, got {inner}"));
        }
        for (i, p) in parts.iter().enumerate() {
            let p = p.trim();
            let hex = p
                .strip_prefix("0x")
                .ok_or_else(|| format!("non-hex byte {p}"))?;
            bytes[i] = u8::from_str_radix(hex, 16).map_err(|e| e.to_string())?;
        }
        Ok(u32::from_le_bytes(bytes))
    }

    fn llvm_mc_word(asm: &str) -> Result<u32, String> {
        let mut child = Command::new(LLVM_MC)
            .args(["-triple=aarch64", "-show-encoding"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("spawn llvm-mc: {e}"))?;
        {
            let mut stdin = child.stdin.take().ok_or("llvm-mc stdin")?;
            stdin
                .write_all(asm.as_bytes())
                .map_err(|e| format!("write llvm-mc: {e}"))?;
            stdin
                .write_all(b"\n")
                .map_err(|e| format!("write llvm-mc: {e}"))?;
        }
        let out = child.wait_with_output().map_err(|e| format!("wait llvm-mc: {e}"))?;
        let stdout = String::from_utf8_lossy(&out.stdout);
        let stderr = String::from_utf8_lossy(&out.stderr);
        if !out.status.success() || stderr.contains("error:") {
            return Err(format!("llvm-mc error: {stderr}"));
        }
        parse_llvm_encoding(&stdout)
    }

    fn aligned_imm() -> impl Strategy<Value = i64> {
        prop_oneof![
            Just(IMM_MIN),
            Just(IMM_MIN + 4),
            Just(-8i64),
            Just(-4i64),
            Just(0i64),
            Just(4i64),
            Just(8i64),
            Just(IMM_MAX - 4),
            Just(IMM_MAX),
            (-(1i64 << 25)..(1i64 << 25)).prop_map(|k| k * 4),
        ]
    }

    fn invalid_imm() -> impl Strategy<Value = i64> {
        prop_oneof![
            Just(IMM_MIN - 4),
            Just(IMM_MIN - 1),
            Just(-1i64),
            Just(1i64),
            Just(2i64),
            Just(3i64),
            Just(5i64),
            Just(IMM_MAX + 1),
            Just(IMM_MAX + 4),
            Just(i64::MIN),
            Just(i64::MAX),
            (-(1i64 << 25) + 1..(1i64 << 25)).prop_map(|k| k * 4 + 1),
            (1i64..=1024).prop_map(|k| IMM_MAX + 4 + k * 4),
            (1i64..=1024).prop_map(|k| IMM_MIN - k * 4),
        ]
    }

    fn addend_strat() -> impl Strategy<Value = i64> {
        prop_oneof![
            Just(0i64),
            Just(-1i64),
            Just(4i64),
            Just(8i64),
            Just(-8i64),
            Just(-4i64),
            -4096i64..=4096i64,
        ]
    }

    fn is_jump26(t: &RelocType) -> bool {
        matches!(t, RelocType::Jump26)
    }

    fn is_call26(t: &RelocType) -> bool {
        matches!(t, RelocType::Call26)
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_branch_kat_llvm_mc_b_imm0() {
        let want = 0x14000000u32;
        let mc = llvm_mc_word("b #0").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [Operand::Imm(0)];
        match encode_branch(&ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for b #0, got {:?}",
                want, other
            ),
        }
    }

    #[test]
    fn encode_branch_kat_llvm_mc_b_imm4() {
        let want = 0x14000001u32;
        let mc = llvm_mc_word("b #4").expect("llvm-mc KAT #4");
        assert_eq!(mc, want, "llvm-mc KAT #4 mapping broken");
        let ops = [Operand::Imm(4)];
        match encode_branch(&ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for b #4, got {:?}",
                want, other
            ),
        }
    }

    #[test]
    fn encode_branch_kat_symbol_foo() {
        let ops = [Operand::Symbol("foo".into())];
        match encode_branch(&ops) {
            Ok(EncodeResult::WordWithReloc { word, reloc }) => {
                assert_eq!(word, B_OPCODE);
                assert!(is_jump26(&reloc.reloc_type));
                assert_eq!(reloc.reloc_type.elf_type(), 282);
                assert_eq!(reloc.symbol, "foo");
                assert_eq!(reloc.addend, 0);
            }
            other => panic!("expected WordWithReloc Jump26 for b foo, got {:?}", other),
        }
    }

    proptest! {
        #![proptest_config(cfg())]

        // Oracle: differential
        // Target: encoder.compare_branch.encode_branch
        #[test]
        fn encode_branch_diff_imm_llvm_mc(imm in aligned_imm()) {
            let asm = format!("b #{}", imm);
            let ops = [Operand::Imm(imm)];
            let sut = match encode_branch(&ops) {
                Ok(EncodeResult::Word(w)) => w,
                other => {
                    return Err(TestCaseError::fail(format!(
                        "SUT rejected valid B {}: {:?}",
                        asm, other
                    )));
                }
            };
            let mc = llvm_mc_word(&asm)
                .map_err(|e| TestCaseError::fail(format!(
                    "llvm-mc rejected valid B {}: {}",
                    asm, e
                )))?;
            prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
        }

        // Oracle: algebraic.invariant
        // Target: encoder.compare_branch.encode_branch
        #[test]
        fn encode_branch_symbol_reloc(
            suffix in 0u32..=1000,
            addend in addend_strat(),
        ) {
            let sym = format!("labl{}", suffix);
            let check = |ops: &[Operand], expect_addend: i64, tag: &str| {
                match encode_branch(ops) {
                    Ok(EncodeResult::WordWithReloc { word, reloc }) => {
                        prop_assert_eq!(word, B_OPCODE, "{} word", tag);
                        prop_assert!(
                            is_jump26(&reloc.reloc_type),
                            "{} expected Jump26, got {:?}",
                            tag, reloc.reloc_type
                        );
                        prop_assert_eq!(
                            reloc.reloc_type.elf_type(),
                            282u32,
                            "{} ELF type", tag
                        );
                        prop_assert_eq!(&reloc.symbol, &sym, "{} symbol", tag);
                        prop_assert_eq!(reloc.addend, expect_addend, "{} addend", tag);
                        prop_assert_eq!(word >> 26, 0b000101u32, "{} opcode", tag);
                        prop_assert_eq!(word & 0x03ff_ffff, 0, "{} imm26 must be 0", tag);
                        Ok(())
                    }
                    other => Err(TestCaseError::fail(format!(
                        "{} expected WordWithReloc, got {:?}",
                        tag, other
                    ))),
                }
            };
            check(&[Operand::Symbol(sym.clone())], 0, "Symbol")?;
            check(&[Operand::Label(sym.clone())], 0, "Label")?;
            check(
                &[Operand::SymbolOffset(sym.clone(), addend)],
                addend,
                "SymbolOffset",
            )?;
        }

        // Oracle: algebraic.metamorphic
        // Target: encoder.compare_branch.encode_branch
        #[test]
        fn encode_branch_meta_vs_bl(
            suffix in 0u32..=1000,
            addend in addend_strat(),
        ) {
            let sym = format!("labl{}", suffix);
            let ops = [Operand::SymbolOffset(sym.clone(), addend)];
            let bl = encode_bl(&ops).map_err(|e| TestCaseError::fail(e))?;
            let b = encode_branch(&ops).map_err(|e| TestCaseError::fail(e))?;
            match (bl, b) {
                (
                    EncodeResult::WordWithReloc { word: w_bl, reloc: r_bl },
                    EncodeResult::WordWithReloc { word: w_b, reloc: r_b },
                ) => {
                    prop_assert_eq!(
                        w_bl ^ w_b,
                        1u32 << 31,
                        "BL XOR B must be bit 31 (bl={:#010x} b={:#010x})",
                        w_bl, w_b
                    );
                    prop_assert!(is_call26(&r_bl.reloc_type), "BL reloc Call26");
                    prop_assert!(is_jump26(&r_b.reloc_type), "B reloc Jump26");
                    prop_assert_eq!(&r_bl.symbol, &sym);
                    prop_assert_eq!(&r_b.symbol, &sym);
                    prop_assert_eq!(r_bl.addend, addend);
                    prop_assert_eq!(r_b.addend, addend);
                }
                other => {
                    return Err(TestCaseError::fail(format!(
                        "expected WordWithReloc pair, got {:?}",
                        other
                    )));
                }
            }
        }

        // Oracle: algebraic.invariant
        // Target: encoder.compare_branch.encode_branch
        #[test]
        fn encode_branch_word_layout(suffix in 0u32..=1000) {
            let sym = format!("labl{}", suffix);
            match encode_branch(&[Operand::Symbol(sym)]) {
                Ok(EncodeResult::WordWithReloc { word, reloc }) => {
                    prop_assert_eq!((word >> 26) & 0x3f, 0b000101u32);
                    prop_assert_eq!(word & 0x03ff_ffff, 0);
                    prop_assert!(is_jump26(&reloc.reloc_type));
                }
                other => {
                    return Err(TestCaseError::fail(format!(
                        "expected WordWithReloc, got {:?}",
                        other
                    )));
                }
            }
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_branch
        #[test]
        fn encode_branch_neg_arity(_dummy in 0u32..=0) {
            prop_assert!(
                encode_branch(&[]).is_err(),
                "bare b must Err (llvm-mc: too few operands)"
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_branch
        #[test]
        fn encode_branch_neg_imm_unaligned_oor(imm in invalid_imm()) {
            let ops = [Operand::Imm(imm)];
            prop_assert!(
                encode_branch(&ops).is_err(),
                "B offset {} is unaligned or out of [-134217728, 134217724] and must Err",
                imm
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_branch
        #[test]
        fn encode_branch_neg_extra_operand(
            suffix in 0u32..=1000,
            which in 0u32..=3,
        ) {
            let sym = format!("labl{}", suffix);
            let extra = match which {
                0 => Operand::Reg("x0".into()),
                1 => Operand::Imm(0),
                2 => Operand::Symbol("bar".into()),
                _ => Operand::Mem { base: "x1".into(), offset: 0 },
            };
            let ops = [Operand::Symbol(sym), extra];
            prop_assert!(
                encode_branch(&ops).is_err(),
                "b <label>, extra (which={}) must Err (llvm-mc: invalid operand)",
                which
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_branch
        #[test]
        fn encode_branch_neg_bad_operand(which in 0u32..=5) {
            let bad = match which {
                0 => Operand::Mem { base: "x0".into(), offset: 0 },
                1 => Operand::Shift { kind: "lsl".into(), amount: 0 },
                2 => Operand::Extend { kind: "sxtw".into(), amount: 0 },
                3 => Operand::RegArrangement { reg: "v0".into(), arrangement: "16b".into() },
                4 => Operand::Modifier { kind: "lo12".into(), symbol: "foo".into() },
                _ => Operand::ModifierOffset {
                    kind: "lo12".into(),
                    symbol: "foo".into(),
                    offset: 8,
                },
            };
            prop_assert!(
                encode_branch(&[bad]).is_err(),
                "B does not take Mem/Shift/Extend/RegArrangement/Modifier (which={})",
                which
            );
        }

        // Oracle: algebraic.invariant (coverage sweep: get_symbol parser-misclassification arms)
        // Target: encoder.compare_branch.encode_branch
        #[test]
        fn encode_branch_symbol_misclassified(
            which in 0u32..=2,
            name in prop::sample::select(vec!["eq", "ne", "lt", "gt", "sy", "ish", "st", "ld"]),
        ) {
            let op = match which {
                0 => Operand::Reg(name.to_string()),
                1 => Operand::Cond(name.to_string()),
                _ => Operand::Barrier(name.to_string()),
            };
            match encode_branch(&[op]) {
                Ok(EncodeResult::WordWithReloc { word, reloc }) => {
                    prop_assert_eq!(word, B_OPCODE);
                    prop_assert!(is_jump26(&reloc.reloc_type));
                    prop_assert_eq!(&reloc.symbol, name);
                    prop_assert_eq!(reloc.addend, 0);
                }
                other => {
                    return Err(TestCaseError::fail(format!(
                        "parser-misclassified {} as B target must be a Jump26 reloc, got {:?}",
                        name, other
                    )));
                }
            }
        }
    }

    #[test]
    fn test_encode_branch_regression_imm_offset() {
        let ops = [Operand::Imm(-134_217_728)];
        match encode_branch(&ops) {
            Ok(EncodeResult::Word(w)) => {
                let mc = llvm_mc_word("b #-134217728").expect("llvm-mc");
                assert_eq!(w, mc, "b #-134217728 must match llvm-mc");
            }
            other => panic!(
                "b #-134217728 must encode as Word matching llvm-mc, got {:?}",
                other
            ),
        }
    }

    #[test]
    fn test_encode_branch_regression_extra_operand() {
        let ops = [Operand::Symbol("labl0".into()), Operand::Reg("x0".into())];
        assert!(
            encode_branch(&ops).is_err(),
            "b labl0, x0 must Err; llvm-mc rejects a second operand"
        );
    }

    #[test]
    fn test_encode_branch_regression_modifier() {
        let ops = [Operand::Modifier {
            kind: "lo12".into(),
            symbol: "foo".into(),
        }];
        assert!(
            encode_branch(&ops).is_err(),
            "b :lo12:foo must Err; B does not take :lo12: modifiers"
        );
    }
}

#[cfg(test)]
mod encode_cbz_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler (immediate form);
    //   algebraic.invariant (symbol reloc / word layout); algebraic.metamorphic (CBZ vs CBNZ);
    //   negative_error (arity / extra / wrong Rt / unaligned-OOR imm).
    // Evidence: src/backend/arm/assembler/README.md:5-14 gas-compat;
    //   README.md:220 Branches lists cbz/cbnz; README.md:267 CondBr19 ELF 280;
    //   encoder/mod.rs:1-7 32-bit AArch64 words; encoder/mod.rs:321-322 cbz/cbnz dispatch;
    //   encoder/mod.rs:76 R_AARCH64_CONDBR19; compare_branch.rs:242 CBZ/CBNZ sf 011010 op imm19 Rt;
    //   ARM ARM Compare and branch (immediate): sf 011010 op imm19 Rt, offset/4, ±1 MiB.
    // Stronger considered:
    //   - State machine: rejected — encode_cbz is a pure function with no lifecycle
    //   - Algebraic round-trip via in-tree decoder: rejected — no CBZ decoder
    //   - encode_tbz / encode_cond_branch as differential siblings: rejected — different jobs
    //     (test-and-branch TstBr14 / B.cond)
    // Weaker available: algebraic.invariant (opcode/reloc fields), algebraic.metamorphic
    //   (bit-24 XOR vs CBNZ), negative_error (empty/unaligned/OOR/extra/SP/FP)
    // Differential: candidate=encode_cbz, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=[Reg(rt), Imm(imm)] <-> asm text `cbz/cbnz rt, #imm`;
    //   [Reg(rt), Symbol(s)|Label(s)|SymbolOffset(s,a)] <-> `cbz/cbnz rt, s{+a}`

    use super::encode_cbz;
    use super::super::{EncodeResult, RelocType};
    use crate::backend::arm::assembler::parser::Operand;
    use proptest::prelude::*;
    use std::io::Write;
    use std::process::{Command, Stdio};

    const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
    const CASES: u32 = 1000;
    /// ARM ARM CBZ/CBNZ signed PC offset: ±1 MiB, multiple of 4.
    const IMM_MIN: i64 = -1_048_576; // -2^20
    const IMM_MAX: i64 = 1_048_572; // 2^20 - 4
    const CBZ_X_BASE: u32 = 0xb4000000;
    const CBNZ_X_BASE: u32 = 0xb5000000;
    const CBZ_W_BASE: u32 = 0x34000000;
    const CBNZ_W_BASE: u32 = 0x35000000;

    fn cfg() -> ProptestConfig {
        ProptestConfig::with_cases(CASES)
    }

    fn parse_llvm_encoding(stdout: &str) -> Result<u32, String> {
        let marker = "encoding: [";
        let start = stdout
            .find(marker)
            .ok_or_else(|| format!("no encoding in stdout: {stdout}"))?;
        let rest = &stdout[start + marker.len()..];
        let end = rest
            .find(']')
            .ok_or_else(|| format!("no closing bracket: {stdout}"))?;
        let inner = &rest[..end];
        let mut bytes = [0u8; 4];
        let parts: Vec<&str> = inner.split(',').collect();
        if parts.len() != 4 {
            return Err(format!("expected 4 bytes, got {inner}"));
        }
        for (i, p) in parts.iter().enumerate() {
            let p = p.trim();
            let hex = p
                .strip_prefix("0x")
                .ok_or_else(|| format!("non-hex byte {p}"))?;
            bytes[i] = u8::from_str_radix(hex, 16).map_err(|e| e.to_string())?;
        }
        Ok(u32::from_le_bytes(bytes))
    }

    fn llvm_mc_word(asm: &str) -> Result<u32, String> {
        let mut child = Command::new(LLVM_MC)
            .args(["-triple=aarch64", "-show-encoding"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("spawn llvm-mc: {e}"))?;
        {
            let mut stdin = child.stdin.take().ok_or("llvm-mc stdin")?;
            stdin
                .write_all(asm.as_bytes())
                .map_err(|e| format!("write llvm-mc: {e}"))?;
            stdin
                .write_all(b"\n")
                .map_err(|e| format!("write llvm-mc: {e}"))?;
        }
        let out = child.wait_with_output().map_err(|e| format!("wait llvm-mc: {e}"))?;
        let stdout = String::from_utf8_lossy(&out.stdout);
        let stderr = String::from_utf8_lossy(&out.stderr);
        if !out.status.success() || stderr.contains("error:") {
            return Err(format!("llvm-mc error: {stderr}"));
        }
        parse_llvm_encoding(&stdout)
    }

    fn mnemonic(is_nz: bool) -> &'static str {
        if is_nz { "cbnz" } else { "cbz" }
    }

    /// n=0..30 -> xN; 31 -> xzr; 32 -> lr.
    fn x_name(n: u32) -> String {
        match n {
            31 => "xzr".to_string(),
            32 => "lr".to_string(),
            n => format!("x{}", n.min(30)),
        }
    }

    /// n=0..30 -> wN; 31 -> wzr.
    fn w_name(n: u32) -> String {
        match n {
            31 => "wzr".to_string(),
            n => format!("w{}", n.min(30)),
        }
    }

    fn gpr_name_strat() -> impl Strategy<Value = String> {
        prop_oneof![
            Just("x0".to_string()),
            Just("x17".to_string()),
            Just("x30".to_string()),
            Just("xzr".to_string()),
            Just("lr".to_string()),
            Just("w0".to_string()),
            Just("w4".to_string()),
            Just("wzr".to_string()),
            Just("X0".to_string()),
            (0u32..=32).prop_map(x_name),
            (0u32..=31).prop_map(w_name),
        ]
    }

    fn aligned_imm() -> impl Strategy<Value = i64> {
        prop_oneof![
            Just(IMM_MIN),
            Just(IMM_MIN + 4),
            Just(-8i64),
            Just(-4i64),
            Just(0i64),
            Just(4i64),
            Just(8i64),
            Just(IMM_MAX - 4),
            Just(IMM_MAX),
            (-(1i64 << 18)..(1i64 << 18)).prop_map(|k| k * 4),
        ]
    }

    fn invalid_imm() -> impl Strategy<Value = i64> {
        prop_oneof![
            Just(IMM_MIN - 4),
            Just(IMM_MIN - 1),
            Just(-1i64),
            Just(1i64),
            Just(2i64),
            Just(3i64),
            Just(5i64),
            Just(IMM_MAX + 1),
            Just(IMM_MAX + 4),
            Just(i64::MIN),
            Just(i64::MAX),
            (-(1i64 << 18) + 1..(1i64 << 18)).prop_map(|k| k * 4 + 1),
            (1i64..=1024).prop_map(|k| IMM_MAX + 4 + k * 4),
            (1i64..=1024).prop_map(|k| IMM_MIN - k * 4),
        ]
    }

    fn addend_strat() -> impl Strategy<Value = i64> {
        prop_oneof![
            Just(0i64),
            Just(-1i64),
            Just(4i64),
            Just(8i64),
            Just(-8i64),
            Just(-4i64),
            -4096i64..=4096i64,
        ]
    }

    fn extra_operand(which: u32) -> Operand {
        match which {
            0 => Operand::Reg("x1".into()),
            1 => Operand::Imm(0),
            2 => Operand::Symbol("bar".into()),
            _ => Operand::Mem {
                base: "x1".into(),
                offset: 0,
            },
        }
    }

    fn wrong_reg_name(which: u32, n: u32) -> String {
        let n = n.min(31);
        match which {
            0 => "sp".to_string(),
            1 => "wsp".to_string(),
            2 => format!("d{}", n),
            3 => format!("s{}", n),
            4 => format!("q{}", n),
            5 => format!("v{}", n),
            6 => format!("h{}", n),
            7 => format!("b{}", n),
            _ => match n {
                0 => "x32".to_string(),
                1 => "w32".to_string(),
                2 => "foo".to_string(),
                3 => "".to_string(),
                4 => "r0".to_string(),
                5 => "x".to_string(),
                6 => "x-1".to_string(),
                _ => "x99".to_string(),
            },
        }
    }

    fn is_condbr19(t: &RelocType) -> bool {
        matches!(t, RelocType::CondBr19)
    }

    fn reloc_base(is_64: bool, is_nz: bool) -> u32 {
        match (is_64, is_nz) {
            (true, false) => CBZ_X_BASE,
            (true, true) => CBNZ_X_BASE,
            (false, false) => CBZ_W_BASE,
            (false, true) => CBNZ_W_BASE,
        }
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_cbz_kat_llvm_mc_cbz_x0_imm0() {
        let want = 0xb4000000u32;
        let mc = llvm_mc_word("cbz x0, #0").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [Operand::Reg("x0".into()), Operand::Imm(0)];
        match encode_cbz(&ops, false) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for cbz x0, #0, got {:?}",
                want, other
            ),
        }
    }

    #[test]
    fn encode_cbz_kat_llvm_mc_cbz_w0_imm0() {
        let want = 0x34000000u32;
        let mc = llvm_mc_word("cbz w0, #0").expect("llvm-mc KAT w0");
        assert_eq!(mc, want, "llvm-mc KAT w0 mapping broken");
        let ops = [Operand::Reg("w0".into()), Operand::Imm(0)];
        match encode_cbz(&ops, false) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for cbz w0, #0, got {:?}",
                want, other
            ),
        }
    }

    #[test]
    fn encode_cbz_kat_llvm_mc_cbnz_x0_imm4() {
        let want = 0xb5000020u32;
        let mc = llvm_mc_word("cbnz x0, #4").expect("llvm-mc KAT cbnz #4");
        assert_eq!(mc, want, "llvm-mc KAT cbnz #4 mapping broken");
        let ops = [Operand::Reg("x0".into()), Operand::Imm(4)];
        match encode_cbz(&ops, true) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for cbnz x0, #4, got {:?}",
                want, other
            ),
        }
    }

    #[test]
    fn encode_cbz_kat_symbol_foo() {
        let ops = [Operand::Reg("x0".into()), Operand::Symbol("foo".into())];
        match encode_cbz(&ops, false) {
            Ok(EncodeResult::WordWithReloc { word, reloc }) => {
                assert_eq!(word, CBZ_X_BASE);
                assert!(is_condbr19(&reloc.reloc_type));
                assert_eq!(reloc.reloc_type.elf_type(), 280);
                assert_eq!(reloc.symbol, "foo");
                assert_eq!(reloc.addend, 0);
            }
            other => panic!(
                "expected WordWithReloc CondBr19 for cbz x0, foo, got {:?}",
                other
            ),
        }
    }

    proptest! {
        #![proptest_config(cfg())]

        // Oracle: differential
        // Target: encoder.compare_branch.encode_cbz
        #[test]
        fn encode_cbz_diff_imm_llvm_mc(
            rt in gpr_name_strat(),
            is_nz in any::<bool>(),
            imm in aligned_imm(),
        ) {
            let asm = format!("{} {}, #{}", mnemonic(is_nz), rt, imm);
            let ops = [Operand::Reg(rt.clone()), Operand::Imm(imm)];
            let sut = match encode_cbz(&ops, is_nz) {
                Ok(EncodeResult::Word(w)) => w,
                other => {
                    return Err(TestCaseError::fail(format!(
                        "SUT rejected valid {}: {:?}",
                        asm, other
                    )));
                }
            };
            let mc = llvm_mc_word(&asm)
                .map_err(|e| TestCaseError::fail(format!(
                    "llvm-mc rejected valid {}: {}",
                    asm, e
                )))?;
            prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
        }

        // Oracle: algebraic.invariant
        // Target: encoder.compare_branch.encode_cbz
        #[test]
        fn encode_cbz_symbol_reloc(
            n in 0u32..=31,
            is_64 in any::<bool>(),
            is_nz in any::<bool>(),
            suffix in 0u32..=1000,
            addend in addend_strat(),
        ) {
            let rt = if is_64 {
                if n == 31 { "xzr".to_string() } else { format!("x{}", n) }
            } else if n == 31 {
                "wzr".to_string()
            } else {
                format!("w{}", n)
            };
            let sym = format!("labl{}", suffix);
            let base = reloc_base(is_64, is_nz) | n;
            let check = |ops: &[Operand], expect_addend: i64, tag: &str| {
                match encode_cbz(ops, is_nz) {
                    Ok(EncodeResult::WordWithReloc { word, reloc }) => {
                        prop_assert_eq!(word, base, "{} word", tag);
                        prop_assert!(
                            is_condbr19(&reloc.reloc_type),
                            "{} expected CondBr19, got {:?}",
                            tag, reloc.reloc_type
                        );
                        prop_assert_eq!(
                            reloc.reloc_type.elf_type(),
                            280u32,
                            "{} ELF type", tag
                        );
                        prop_assert_eq!(&reloc.symbol, &sym, "{} symbol", tag);
                        prop_assert_eq!(reloc.addend, expect_addend, "{} addend", tag);
                        prop_assert_eq!(word & 0x00ff_ffe0, 0, "{} imm19 must be 0", tag);
                        Ok(())
                    }
                    other => Err(TestCaseError::fail(format!(
                        "{} expected WordWithReloc, got {:?}",
                        tag, other
                    ))),
                }
            };
            check(&[Operand::Reg(rt.clone()), Operand::Symbol(sym.clone())], 0, "Symbol")?;
            check(&[Operand::Reg(rt.clone()), Operand::Label(sym.clone())], 0, "Label")?;
            check(
                &[Operand::Reg(rt), Operand::SymbolOffset(sym.clone(), addend)],
                addend,
                "SymbolOffset",
            )?;
        }

        // Oracle: algebraic.metamorphic
        // Target: encoder.compare_branch.encode_cbz
        #[test]
        fn encode_cbz_meta_cbz_vs_cbnz(
            n in 0u32..=31,
            is_64 in any::<bool>(),
            suffix in 0u32..=1000,
            addend in addend_strat(),
        ) {
            let rt = if is_64 {
                if n == 31 { "xzr".to_string() } else { format!("x{}", n) }
            } else if n == 31 {
                "wzr".to_string()
            } else {
                format!("w{}", n)
            };
            let sym = format!("labl{}", suffix);
            let ops = [Operand::Reg(rt), Operand::SymbolOffset(sym.clone(), addend)];
            let cbnz = encode_cbz(&ops, true).map_err(|e| TestCaseError::fail(e))?;
            let cbz = encode_cbz(&ops, false).map_err(|e| TestCaseError::fail(e))?;
            match (cbnz, cbz) {
                (
                    EncodeResult::WordWithReloc { word: w_nz, reloc: r_nz },
                    EncodeResult::WordWithReloc { word: w_z, reloc: r_z },
                ) => {
                    prop_assert_eq!(
                        w_nz ^ w_z,
                        1u32 << 24,
                        "CBNZ XOR CBZ must be bit 24 (cbnz={:#010x} cbz={:#010x})",
                        w_nz, w_z
                    );
                    prop_assert!(is_condbr19(&r_nz.reloc_type), "CBNZ reloc CondBr19");
                    prop_assert!(is_condbr19(&r_z.reloc_type), "CBZ reloc CondBr19");
                    prop_assert_eq!(&r_nz.symbol, &sym);
                    prop_assert_eq!(&r_z.symbol, &sym);
                    prop_assert_eq!(r_nz.addend, addend);
                    prop_assert_eq!(r_z.addend, addend);
                }
                other => {
                    return Err(TestCaseError::fail(format!(
                        "expected WordWithReloc pair, got {:?}",
                        other
                    )));
                }
            }
        }

        // Oracle: algebraic.invariant
        // Target: encoder.compare_branch.encode_cbz
        #[test]
        fn encode_cbz_word_layout(
            n in 0u32..=31,
            is_64 in any::<bool>(),
            is_nz in any::<bool>(),
        ) {
            let rt = if is_64 {
                if n == 31 { "xzr".to_string() } else { format!("x{}", n) }
            } else if n == 31 {
                "wzr".to_string()
            } else {
                format!("w{}", n)
            };
            match encode_cbz(&[Operand::Reg(rt), Operand::Symbol("L".into())], is_nz) {
                Ok(EncodeResult::WordWithReloc { word, reloc }) => {
                    prop_assert_eq!((word >> 25) & 0x3f, 0b011010u32, "bits[30:25]");
                    prop_assert_eq!(word >> 31, if is_64 { 1u32 } else { 0u32 }, "sf");
                    prop_assert_eq!((word >> 24) & 1, if is_nz { 1u32 } else { 0u32 }, "op");
                    prop_assert_eq!(word & 0x1f, n, "Rt");
                    prop_assert_eq!(word & 0x00ff_ffe0, 0, "imm19");
                    prop_assert!(is_condbr19(&reloc.reloc_type));
                }
                other => {
                    return Err(TestCaseError::fail(format!(
                        "expected WordWithReloc, got {:?}",
                        other
                    )));
                }
            }
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_cbz
        #[test]
        fn encode_cbz_neg_arity(is_nz in any::<bool>()) {
            prop_assert!(
                encode_cbz(&[], is_nz).is_err(),
                "bare {} must Err (llvm-mc: too few operands)",
                mnemonic(is_nz)
            );
            prop_assert!(
                encode_cbz(&[Operand::Reg("x0".into())], is_nz).is_err(),
                "{} x0 must Err (llvm-mc: too few operands)",
                mnemonic(is_nz)
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_cbz
        #[test]
        fn encode_cbz_neg_extra_operand(
            n in 0u32..=30,
            is_nz in any::<bool>(),
            suffix in 0u32..=1000,
            which in 0u32..=3,
        ) {
            let extra = extra_operand(which);
            let ops = [
                Operand::Reg(format!("x{}", n)),
                Operand::Symbol(format!("labl{}", suffix)),
                extra,
            ];
            prop_assert!(
                encode_cbz(&ops, is_nz).is_err(),
                "{} x{}, label, extra (which={}) must Err (llvm-mc: invalid operand)",
                mnemonic(is_nz), n, which
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_cbz
        #[test]
        fn encode_cbz_neg_wrong_reg(
            which in 0u32..=8,
            n in 0u32..=31,
            is_nz in any::<bool>(),
        ) {
            let name = wrong_reg_name(which, n);
            let ops = [Operand::Reg(name.clone()), Operand::Symbol("L".into())];
            prop_assert!(
                encode_cbz(&ops, is_nz).is_err(),
                "{} {} , L must Err (llvm-mc rejects SP / FP / invalid names)",
                mnemonic(is_nz), name
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_cbz
        #[test]
        fn encode_cbz_neg_imm_unaligned_oor(
            rt in gpr_name_strat(),
            is_nz in any::<bool>(),
            imm in invalid_imm(),
        ) {
            let ops = [Operand::Reg(rt.clone()), Operand::Imm(imm)];
            prop_assert!(
                encode_cbz(&ops, is_nz).is_err(),
                "{} {}, #{} is unaligned or out of [-1048576, 1048572] and must Err",
                mnemonic(is_nz), rt, imm
            );
        }

        // Oracle: algebraic.invariant (coverage sweep: get_symbol parser-misclassification arms)
        // Target: encoder.compare_branch.encode_cbz
        #[test]
        fn encode_cbz_symbol_misclassified(
            which in 0u32..=2,
            name in prop::sample::select(vec!["eq", "ne", "lt", "gt", "sy", "ish", "st", "ld"]),
            is_nz in any::<bool>(),
        ) {
            let label = match which {
                0 => Operand::Reg(name.to_string()),
                1 => Operand::Cond(name.to_string()),
                _ => Operand::Barrier(name.to_string()),
            };
            let ops = [Operand::Reg("x0".into()), label];
            match encode_cbz(&ops, is_nz) {
                Ok(EncodeResult::WordWithReloc { word, reloc }) => {
                    prop_assert_eq!(word, reloc_base(true, is_nz));
                    prop_assert!(is_condbr19(&reloc.reloc_type));
                    prop_assert_eq!(&reloc.symbol, name);
                    prop_assert_eq!(reloc.addend, 0);
                }
                other => {
                    return Err(TestCaseError::fail(format!(
                        "parser-misclassified {} as CBZ target must be a CondBr19 reloc, got {:?}",
                        name, other
                    )));
                }
            }
        }

        // Oracle: negative_error (coverage sweep: get_symbol other-kind arm)
        // Target: encoder.compare_branch.encode_cbz
        #[test]
        fn encode_cbz_neg_bad_label_kind(which in 0u32..=5, is_nz in any::<bool>()) {
            let bad = match which {
                0 => Operand::Mem { base: "x0".into(), offset: 0 },
                1 => Operand::Shift { kind: "lsl".into(), amount: 0 },
                2 => Operand::Extend { kind: "sxtw".into(), amount: 0 },
                3 => Operand::RegArrangement { reg: "v0".into(), arrangement: "16b".into() },
                4 => Operand::Expr("foo+bar".into()),
                _ => Operand::RegList(vec![]),
            };
            let ops = [Operand::Reg("x0".into()), bad];
            prop_assert!(
                encode_cbz(&ops, is_nz).is_err(),
                "CBZ label slot does not take Mem/Shift/Extend/RegArrangement/Expr/RegList (which={})",
                which
            );
        }
    }

    #[test]
    fn test_encode_cbz_regression_imm_offset() {
        let ops = [Operand::Reg("x0".into()), Operand::Imm(IMM_MIN)];
        match encode_cbz(&ops, false) {
            Ok(EncodeResult::Word(w)) => {
                let mc = llvm_mc_word("cbz x0, #-1048576").expect("llvm-mc");
                assert_eq!(w, mc, "cbz x0, #-1048576 must match llvm-mc");
            }
            other => panic!(
                "cbz x0, #-1048576 must encode as Word matching llvm-mc, got {:?}",
                other
            ),
        }
    }

    #[test]
    fn test_encode_cbz_regression_extra_operand() {
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Symbol("labl0".into()),
            Operand::Reg("x1".into()),
        ];
        assert!(
            encode_cbz(&ops, false).is_err(),
            "cbz x0, labl0, x1 must Err; llvm-mc rejects a third operand"
        );
    }

    #[test]
    fn test_encode_cbz_regression_sp() {
        let ops = [Operand::Reg("sp".into()), Operand::Symbol("L".into())];
        assert!(
            encode_cbz(&ops, false).is_err(),
            "cbz sp, L must Err; llvm-mc rejects SP (register 31 is XZR)"
        );
    }

    #[test]
    fn test_encode_cbz_regression_fp_reg() {
        let ops = [Operand::Reg("d0".into()), Operand::Symbol("L".into())];
        assert!(
            encode_cbz(&ops, false).is_err(),
            "cbz d0, L must Err; llvm-mc rejects FP/SIMD Rt"
        );
    }
}

#[cfg(test)]
mod encode_ccmp_ccmn_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler (imm and register forms);
    //   algebraic.invariant (word layout); algebraic.metamorphic (CCMP vs CCMN);
    //   negative_error (arity / imm5-nzcv range / extra / wrong class).
    // Evidence: src/backend/arm/assembler/README.md:5-14 gas-compat;
    //   README.md:218 Compare lists ccmp; encoder/mod.rs:1-7 32-bit AArch64 words;
    //   encoder/mod.rs:304-305 ccmp/ccmn dispatch; compare_branch.rs:53-54
    //   CCMP bit 30 = 1, CCMN bit 30 = 0; parser.rs:1987 ccmp x10, x13, 0, eq;
    //   ARM ARM Conditional compare (immediate): sf op S 11010010 imm5 cond 1 0 Rn 0 nzcv;
    //   ARM ARM Conditional compare (register): sf op S 11010010 Rm cond 0 0 Rn 0 nzcv.
    // Stronger considered:
    //   - State machine: rejected — encode_ccmp_ccmn is a pure function with no lifecycle
    //   - Algebraic round-trip via in-tree decoder: rejected — no CCMP/CCMN decoder
    //   - encode_cmp / encode_cmn as differential sibling: rejected — different job
    //     (SUBS/ADDS XZR aliases, no cond/nzcv)
    // Weaker available: algebraic.invariant (opcode/sf/op/S/o2/Rn/Rm/imm5/cond/nzcv),
    //   algebraic.metamorphic (bit-30 XOR), negative_error (arity/range/extra/SP/FP/mixed)
    // Differential: candidate=encode_ccmp_ccmn, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=[Reg(rn), Imm(imm5), Imm(nzcv), Cond(c)] <-> `ccmp/ccmn rn, #imm5, #nzcv, c`;
    //   [Reg(rn), Reg(rm), Imm(nzcv), Cond(c)] <-> `ccmp/ccmn rn, rm, #nzcv, c`

    use super::encode_ccmp_ccmn;
    use super::super::EncodeResult;
    use crate::backend::arm::assembler::parser::Operand;
    use proptest::prelude::*;
    use std::io::Write;
    use std::process::{Command, Stdio};

    const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
    const CASES: u32 = 1000;
    const CONDS: [&str; 18] = [
        "eq", "ne", "cs", "hs", "cc", "lo", "mi", "pl",
        "vs", "vc", "hi", "ls", "ge", "lt", "gt", "le", "al", "nv",
    ];
    const COND_CANON: [&str; 16] = [
        "eq", "ne", "cs", "cc", "mi", "pl", "vs", "vc",
        "hi", "ls", "ge", "lt", "gt", "le", "al", "nv",
    ];

    fn cfg() -> ProptestConfig {
        ProptestConfig::with_cases(CASES)
    }

    fn parse_llvm_encoding(stdout: &str) -> Result<u32, String> {
        let marker = "encoding: [";
        let start = stdout
            .find(marker)
            .ok_or_else(|| format!("no encoding in stdout: {stdout}"))?;
        let rest = &stdout[start + marker.len()..];
        let end = rest
            .find(']')
            .ok_or_else(|| format!("no closing bracket: {stdout}"))?;
        let inner = &rest[..end];
        let mut bytes = [0u8; 4];
        let parts: Vec<&str> = inner.split(',').collect();
        if parts.len() != 4 {
            return Err(format!("expected 4 bytes, got {inner}"));
        }
        for (i, p) in parts.iter().enumerate() {
            let p = p.trim();
            let hex = p
                .strip_prefix("0x")
                .ok_or_else(|| format!("non-hex byte {p}"))?;
            bytes[i] = u8::from_str_radix(hex, 16).map_err(|e| e.to_string())?;
        }
        Ok(u32::from_le_bytes(bytes))
    }

    fn llvm_mc_word(asm: &str) -> Result<u32, String> {
        let mut child = Command::new(LLVM_MC)
            .args(["-triple=aarch64", "-show-encoding"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("spawn llvm-mc: {e}"))?;
        {
            let mut stdin = child.stdin.take().ok_or("llvm-mc stdin")?;
            stdin
                .write_all(asm.as_bytes())
                .map_err(|e| format!("write llvm-mc: {e}"))?;
            stdin
                .write_all(b"\n")
                .map_err(|e| format!("write llvm-mc: {e}"))?;
        }
        let out = child.wait_with_output().map_err(|e| format!("wait llvm-mc: {e}"))?;
        let stdout = String::from_utf8_lossy(&out.stdout);
        let stderr = String::from_utf8_lossy(&out.stderr);
        if !out.status.success() || stderr.contains("error:") {
            return Err(format!("llvm-mc error: {stderr}"));
        }
        parse_llvm_encoding(&stdout)
    }

    fn mnemonic(is_ccmp: bool) -> &'static str {
        if is_ccmp { "ccmp" } else { "ccmn" }
    }

    fn gpr(is_64: bool, n: u32) -> String {
        let n = n.min(31);
        if is_64 {
            if n == 31 {
                "xzr".to_string()
            } else {
                format!("x{}", n)
            }
        } else if n == 31 {
            "wzr".to_string()
        } else {
            format!("w{}", n)
        }
    }

    fn gpr_strat() -> impl Strategy<Value = String> {
        prop_oneof![
            Just("x0".to_string()),
            Just("x30".to_string()),
            Just("xzr".to_string()),
            Just("lr".to_string()),
            Just("X0".to_string()),
            Just("w0".to_string()),
            Just("w30".to_string()),
            Just("wzr".to_string()),
            (0u32..=31).prop_map(|n| gpr(true, n)),
            (0u32..=31).prop_map(|n| gpr(false, n)),
        ]
    }

    fn same_width_pair() -> impl Strategy<Value = (String, String)> {
        (any::<bool>(), 0u32..=31, 0u32..=31).prop_map(|(is_64, n, m)| (gpr(is_64, n), gpr(is_64, m)))
    }

    fn imm5_strat() -> impl Strategy<Value = i64> {
        prop_oneof![
            Just(0i64),
            Just(1i64),
            Just(15i64),
            Just(16i64),
            Just(30i64),
            Just(31i64),
            0i64..=31,
        ]
    }

    fn nzcv_strat() -> impl Strategy<Value = i64> {
        prop_oneof![
            Just(0i64),
            Just(1i64),
            Just(14i64),
            Just(15i64),
            0i64..=15,
        ]
    }

    fn cond_strat() -> impl Strategy<Value = String> {
        prop::sample::select(CONDS.iter().map(|s| s.to_string()).collect::<Vec<_>>())
    }

    fn oor_imm5() -> impl Strategy<Value = i64> {
        prop_oneof![
            Just(-1i64),
            Just(-2i64),
            Just(32i64),
            Just(33i64),
            Just(64i64),
            Just(i64::MIN),
            Just(i64::MAX),
            32i64..=256,
            -256i64..=-1,
        ]
    }

    fn oor_nzcv() -> impl Strategy<Value = i64> {
        prop_oneof![
            Just(-1i64),
            Just(-2i64),
            Just(16i64),
            Just(17i64),
            Just(255i64),
            Just(i64::MIN),
            Just(i64::MAX),
            16i64..=256,
            -256i64..=-1,
        ]
    }

    fn extra_operand(which: u32) -> Operand {
        match which % 6 {
            0 => Operand::Reg("x1".into()),
            1 => Operand::Imm(0),
            2 => Operand::Symbol("bar".into()),
            3 => Operand::Mem {
                base: "x1".into(),
                offset: 0,
            },
            4 => Operand::Cond("eq".into()),
            _ => Operand::Shift {
                kind: "lsl".into(),
                amount: 0,
            },
        }
    }

    fn wrong_reg_name(which: u32, n: u32) -> String {
        let n = n.min(31);
        match which % 16 {
            0 => "sp".to_string(),
            1 => "wsp".to_string(),
            2 => format!("d{}", n),
            3 => format!("s{}", n),
            4 => format!("q{}", n),
            5 => format!("v{}", n),
            6 => format!("h{}", n),
            7 => format!("b{}", n),
            8 => "x32".to_string(),
            9 => "w32".to_string(),
            10 => "foo".to_string(),
            11 => "".to_string(),
            12 => "r0".to_string(),
            13 => "x".to_string(),
            14 => "x-1".to_string(),
            _ => "x99".to_string(),
        }
    }

    fn word_of(r: Result<EncodeResult, String>) -> Result<u32, TestCaseError> {
        match r {
            Ok(EncodeResult::Word(w)) => Ok(w),
            other => Err(TestCaseError::fail(format!("expected Word, got {:?}", other))),
        }
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_ccmp_ccmn_kat_llvm_mc_ccmp_imm() {
        let want = 0xfa400800u32;
        let mc = llvm_mc_word("ccmp x0, #0, #0, eq").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Imm(0),
            Operand::Imm(0),
            Operand::Cond("eq".into()),
        ];
        match encode_ccmp_ccmn(&ops, true) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for ccmp x0, #0, #0, eq, got {:?}",
                want, other
            ),
        }
    }

    #[test]
    fn encode_ccmp_ccmn_kat_llvm_mc_ccmp_reg() {
        let want = 0xfa410000u32;
        let mc = llvm_mc_word("ccmp x0, x1, #0, eq").expect("llvm-mc KAT reg");
        assert_eq!(mc, want, "llvm-mc KAT reg mapping broken");
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("x1".into()),
            Operand::Imm(0),
            Operand::Cond("eq".into()),
        ];
        match encode_ccmp_ccmn(&ops, true) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for ccmp x0, x1, #0, eq, got {:?}",
                want, other
            ),
        }
    }

    #[test]
    fn encode_ccmp_ccmn_kat_llvm_mc_ccmn_imm() {
        let want = 0xba400800u32;
        let mc = llvm_mc_word("ccmn x0, #0, #0, eq").expect("llvm-mc KAT ccmn");
        assert_eq!(mc, want, "llvm-mc KAT ccmn mapping broken");
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Imm(0),
            Operand::Imm(0),
            Operand::Cond("eq".into()),
        ];
        match encode_ccmp_ccmn(&ops, false) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for ccmn x0, #0, #0, eq, got {:?}",
                want, other
            ),
        }
    }

    proptest! {
        #![proptest_config(cfg())]

        // Oracle: differential
        // Target: encoder.compare_branch.encode_ccmp_ccmn
        #[test]
        fn encode_ccmp_ccmn_diff_imm_llvm_mc(
            rn in gpr_strat(),
            is_ccmp in any::<bool>(),
            imm5 in imm5_strat(),
            nzcv in nzcv_strat(),
            cond in cond_strat(),
        ) {
            let asm = format!(
                "{} {}, #{}, #{}, {}",
                mnemonic(is_ccmp), rn, imm5, nzcv, cond
            );
            let ops = [
                Operand::Reg(rn.clone()),
                Operand::Imm(imm5),
                Operand::Imm(nzcv),
                Operand::Cond(cond.clone()),
            ];
            let sut = word_of(encode_ccmp_ccmn(&ops, is_ccmp))?;
            let mc = llvm_mc_word(&asm).map_err(|e| TestCaseError::fail(format!(
                "llvm-mc rejected valid {}: {}",
                asm, e
            )))?;
            prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
        }

        // Oracle: differential
        // Target: encoder.compare_branch.encode_ccmp_ccmn
        #[test]
        fn encode_ccmp_ccmn_diff_reg_llvm_mc(
            pair in same_width_pair(),
            is_ccmp in any::<bool>(),
            nzcv in nzcv_strat(),
            cond in cond_strat(),
        ) {
            let (rn, rm) = pair;
            let asm = format!(
                "{} {}, {}, #{}, {}",
                mnemonic(is_ccmp), rn, rm, nzcv, cond
            );
            let ops = [
                Operand::Reg(rn.clone()),
                Operand::Reg(rm.clone()),
                Operand::Imm(nzcv),
                Operand::Cond(cond.clone()),
            ];
            let sut = word_of(encode_ccmp_ccmn(&ops, is_ccmp))?;
            let mc = llvm_mc_word(&asm).map_err(|e| TestCaseError::fail(format!(
                "llvm-mc rejected valid {}: {}",
                asm, e
            )))?;
            prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
        }

        // Oracle: algebraic.metamorphic
        // Target: encoder.compare_branch.encode_ccmp_ccmn
        #[test]
        fn encode_ccmp_ccmn_meta_ccmp_vs_ccmn(
            rn in gpr_strat(),
            use_imm in any::<bool>(),
            rm_n in 0u32..=31,
            imm5 in imm5_strat(),
            nzcv in nzcv_strat(),
            cond in cond_strat(),
        ) {
            let ops = if use_imm {
                vec![
                    Operand::Reg(rn.clone()),
                    Operand::Imm(imm5),
                    Operand::Imm(nzcv),
                    Operand::Cond(cond.clone()),
                ]
            } else {
                let is_64 = rn.eq_ignore_ascii_case("xzr")
                    || rn.eq_ignore_ascii_case("lr")
                    || rn.to_ascii_lowercase().starts_with('x');
                vec![
                    Operand::Reg(rn.clone()),
                    Operand::Reg(gpr(is_64, rm_n)),
                    Operand::Imm(nzcv),
                    Operand::Cond(cond.clone()),
                ]
            };
            let w_ccmp = word_of(encode_ccmp_ccmn(&ops, true))?;
            let w_ccmn = word_of(encode_ccmp_ccmn(&ops, false))?;
            prop_assert_eq!(
                w_ccmp ^ w_ccmn,
                1u32 << 30,
                "CCMP XOR CCMN must be bit 30 (ccmp={:#010x} ccmn={:#010x})",
                w_ccmp, w_ccmn
            );
        }

        // Oracle: algebraic.invariant
        // Target: encoder.compare_branch.encode_ccmp_ccmn
        #[test]
        fn encode_ccmp_ccmn_word_layout(
            rn_num in 0u32..=31,
            rm_num in 0u32..=31,
            is_64 in any::<bool>(),
            is_ccmp in any::<bool>(),
            use_imm in any::<bool>(),
            imm5 in imm5_strat(),
            nzcv in nzcv_strat(),
            cond_val in 0u32..=15,
        ) {
            let rn = gpr(is_64, rn_num);
            let cond = COND_CANON[cond_val as usize].to_string();
            let ops = if use_imm {
                vec![
                    Operand::Reg(rn),
                    Operand::Imm(imm5),
                    Operand::Imm(nzcv),
                    Operand::Cond(cond),
                ]
            } else {
                vec![
                    Operand::Reg(rn),
                    Operand::Reg(gpr(is_64, rm_num)),
                    Operand::Imm(nzcv),
                    Operand::Cond(cond),
                ]
            };
            let w = word_of(encode_ccmp_ccmn(&ops, is_ccmp))?;
            let sf = if is_64 { 1u32 } else { 0u32 };
            let op = if is_ccmp { 1u32 } else { 0u32 };
            prop_assert_eq!(w >> 31, sf, "sf [31]");
            prop_assert_eq!((w >> 30) & 1, op, "op [30]");
            prop_assert_eq!((w >> 29) & 1, 1u32, "S [29]");
            prop_assert_eq!((w >> 21) & 0xFF, 0b11010010u32, "opcode [28:21]");
            if use_imm {
                prop_assert_eq!((w >> 16) & 0x1F, imm5 as u32, "imm5 [20:16]");
                prop_assert_eq!((w >> 11) & 1, 1u32, "o2 [11] immediate");
            } else {
                prop_assert_eq!((w >> 16) & 0x1F, rm_num, "Rm [20:16]");
                prop_assert_eq!((w >> 11) & 1, 0u32, "o2 [11] register");
            }
            prop_assert_eq!((w >> 12) & 0xF, cond_val, "cond [15:12]");
            prop_assert_eq!((w >> 10) & 1, 0u32, "bit 10");
            prop_assert_eq!((w >> 5) & 0x1F, rn_num, "Rn [9:5]");
            prop_assert_eq!((w >> 4) & 1, 0u32, "bit 4");
            prop_assert_eq!(w & 0xF, nzcv as u32, "nzcv [3:0]");
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_ccmp_ccmn
        #[test]
        fn encode_ccmp_ccmn_neg_arity(is_ccmp in any::<bool>(), n in 0u32..=3) {
            let full = [
                Operand::Reg("x0".into()),
                Operand::Imm(0),
                Operand::Imm(0),
                Operand::Cond("eq".into()),
            ];
            let ops = &full[..n as usize];
            prop_assert!(
                encode_ccmp_ccmn(ops, is_ccmp).is_err(),
                "{} with {} operands must Err (llvm-mc: too few operands)",
                mnemonic(is_ccmp), n
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_ccmp_ccmn
        #[test]
        fn encode_ccmp_ccmn_neg_imm5_nzcv_oor(
            rn in gpr_strat(),
            is_ccmp in any::<bool>(),
            cond in cond_strat(),
            kind in 0u32..=2,
            bad_imm5 in oor_imm5(),
            bad_nzcv in oor_nzcv(),
            good_imm5 in imm5_strat(),
            good_nzcv in nzcv_strat(),
        ) {
            let (imm5, nzcv) = match kind {
                0 => (bad_imm5, good_nzcv),
                1 => (good_imm5, bad_nzcv),
                _ => (bad_imm5, bad_nzcv),
            };
            prop_assume!(!(0..=31).contains(&imm5) || !(0..=15).contains(&nzcv));
            let ops = [
                Operand::Reg(rn.clone()),
                Operand::Imm(imm5),
                Operand::Imm(nzcv),
                Operand::Cond(cond.clone()),
            ];
            prop_assert!(
                encode_ccmp_ccmn(&ops, is_ccmp).is_err(),
                "{} {}, #{}, #{}, {} must Err (imm5 in [0,31], nzcv in [0,15])",
                mnemonic(is_ccmp), rn, imm5, nzcv, cond
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_ccmp_ccmn
        #[test]
        fn encode_ccmp_ccmn_neg_extra_operand(
            rn in gpr_strat(),
            is_ccmp in any::<bool>(),
            imm5 in imm5_strat(),
            nzcv in nzcv_strat(),
            cond in cond_strat(),
            which in 0u32..=5,
        ) {
            let extra = extra_operand(which);
            let ops = [
                Operand::Reg(rn),
                Operand::Imm(imm5),
                Operand::Imm(nzcv),
                Operand::Cond(cond),
                extra,
            ];
            prop_assert!(
                encode_ccmp_ccmn(&ops, is_ccmp).is_err(),
                "{} with a 5th operand (which={}) must Err (llvm-mc: invalid operand)",
                mnemonic(is_ccmp), which
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_ccmp_ccmn
        #[test]
        fn encode_ccmp_ccmn_neg_wrong_reg_class(
            which in 0u32..=15,
            n in 0u32..=31,
            is_ccmp in any::<bool>(),
        ) {
            let name = wrong_reg_name(which, n);
            let ops = [
                Operand::Reg(name.clone()),
                Operand::Imm(0),
                Operand::Imm(0),
                Operand::Cond("eq".into()),
            ];
            prop_assert!(
                encode_ccmp_ccmn(&ops, is_ccmp).is_err(),
                "{} {} must Err (llvm-mc rejects SP / FP / invalid names)",
                mnemonic(is_ccmp), name
            );
        }

        // Oracle: negative_error (mixed-width Rn/Rm — same property)
        // Target: encoder.compare_branch.encode_ccmp_ccmn
        #[test]
        fn encode_ccmp_ccmn_neg_mixed_width(
            n in 0u32..=30,
            m in 0u32..=30,
            x_first in any::<bool>(),
            is_ccmp in any::<bool>(),
            nzcv in nzcv_strat(),
            cond in cond_strat(),
        ) {
            let (rn, rm) = if x_first {
                (format!("x{}", n), format!("w{}", m))
            } else {
                (format!("w{}", n), format!("x{}", m))
            };
            let ops = [
                Operand::Reg(rn.clone()),
                Operand::Reg(rm.clone()),
                Operand::Imm(nzcv),
                Operand::Cond(cond),
            ];
            prop_assert!(
                encode_ccmp_ccmn(&ops, is_ccmp).is_err(),
                "{} {}, {} mixed width must Err (llvm-mc rejects mixed x/w)",
                mnemonic(is_ccmp), rn, rm
            );
        }

        // Oracle: negative_error (coverage sweep: encode_cond None arm)
        // Target: encoder.compare_branch.encode_ccmp_ccmn
        #[test]
        fn encode_ccmp_ccmn_neg_invalid_cond(
            rn in gpr_strat(),
            is_ccmp in any::<bool>(),
            use_imm in any::<bool>(),
            which in 0u32..=5,
        ) {
            let bad = match which {
                0 => "xx",
                1 => "foo",
                2 => "",
                3 => "eqz",
                4 => "n",
                _ => "zzzz",
            };
            let ops = if use_imm {
                vec![
                    Operand::Reg(rn),
                    Operand::Imm(0),
                    Operand::Imm(0),
                    Operand::Cond(bad.into()),
                ]
            } else {
                vec![
                    Operand::Reg(rn.clone()),
                    Operand::Reg(rn),
                    Operand::Imm(0),
                    Operand::Cond(bad.into()),
                ]
            };
            prop_assert!(
                encode_ccmp_ccmn(&ops, is_ccmp).is_err(),
                "{} with cond '{}' must Err (llvm-mc: invalid condition code)",
                mnemonic(is_ccmp), bad
            );
        }

        // Oracle: negative_error (coverage sweep: parse_reg_num None on Rm)
        // Target: encoder.compare_branch.encode_ccmp_ccmn
        #[test]
        fn encode_ccmp_ccmn_neg_invalid_rm(
            n in 0u32..=30,
            is_ccmp in any::<bool>(),
            which in 0u32..=7,
        ) {
            let rm = match which {
                0 => "x32",
                1 => "w32",
                2 => "foo",
                3 => "",
                4 => "r0",
                5 => "x",
                6 => "x-1",
                _ => "x99",
            };
            let ops = [
                Operand::Reg(format!("x{}", n)),
                Operand::Reg(rm.to_string()),
                Operand::Imm(0),
                Operand::Cond("eq".into()),
            ];
            prop_assert!(
                encode_ccmp_ccmn(&ops, is_ccmp).is_err(),
                "{} x{}, {} must Err (invalid rm)",
                mnemonic(is_ccmp), n, rm
            );
        }

        // Oracle: negative_error (coverage sweep: neither Imm nor Reg at op1 / non-Imm nzcv / non-Cond)
        // Target: encoder.compare_branch.encode_ccmp_ccmn
        #[test]
        fn encode_ccmp_ccmn_neg_bad_operand_kind(
            is_ccmp in any::<bool>(),
            slot in 1u32..=3,
            which in 0u32..=5,
        ) {
            let bad = match which {
                0 => Operand::Mem {
                    base: "x0".into(),
                    offset: 0,
                },
                1 => Operand::Symbol("foo".into()),
                2 => Operand::Shift {
                    kind: "lsl".into(),
                    amount: 0,
                },
                3 => Operand::Extend {
                    kind: "sxtw".into(),
                    amount: 0,
                },
                4 => Operand::Label("L".into()),
                _ => Operand::Barrier("sy".into()),
            };
            let mut ops = vec![
                Operand::Reg("x0".into()),
                Operand::Imm(0),
                Operand::Imm(0),
                Operand::Cond("eq".into()),
            ];
            ops[slot as usize] = bad;
            prop_assert!(
                encode_ccmp_ccmn(&ops, is_ccmp).is_err(),
                "{} with bad kind at slot {} (which={}) must Err",
                mnemonic(is_ccmp), slot, which
            );
        }
    }

    #[test]
    fn test_encode_ccmp_ccmn_regression_imm5_oor() {
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Imm(-1),
            Operand::Imm(0),
            Operand::Cond("eq".into()),
        ];
        assert!(
            encode_ccmp_ccmn(&ops, false).is_err(),
            "ccmn x0, #-1, #0, eq must Err; llvm-mc rejects imm5 outside [0, 31]"
        );
    }

    #[test]
    fn test_encode_ccmp_ccmn_regression_nzcv_oor() {
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Imm(0),
            Operand::Imm(16),
            Operand::Cond("eq".into()),
        ];
        assert!(
            encode_ccmp_ccmn(&ops, true).is_err(),
            "ccmp x0, #0, #16, eq must Err; llvm-mc rejects nzcv outside [0, 15]"
        );
    }

    #[test]
    fn test_encode_ccmp_ccmn_regression_extra_operand() {
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Imm(0),
            Operand::Imm(0),
            Operand::Cond("eq".into()),
            Operand::Reg("x1".into()),
        ];
        assert!(
            encode_ccmp_ccmn(&ops, false).is_err(),
            "ccmn x0, #0, #0, eq, x1 must Err; llvm-mc rejects a fifth operand"
        );
    }

    #[test]
    fn test_encode_ccmp_ccmn_regression_sp() {
        let ops = [
            Operand::Reg("sp".into()),
            Operand::Imm(0),
            Operand::Imm(0),
            Operand::Cond("eq".into()),
        ];
        assert!(
            encode_ccmp_ccmn(&ops, false).is_err(),
            "ccmn sp, #0, #0, eq must Err; llvm-mc rejects SP (register 31 is XZR)"
        );
    }

    #[test]
    fn test_encode_ccmp_ccmn_regression_fp_reg() {
        let ops = [
            Operand::Reg("d0".into()),
            Operand::Imm(0),
            Operand::Imm(0),
            Operand::Cond("eq".into()),
        ];
        assert!(
            encode_ccmp_ccmn(&ops, true).is_err(),
            "ccmp d0, #0, #0, eq must Err; llvm-mc rejects FP/SIMD Rn"
        );
    }

    #[test]
    fn test_encode_ccmp_ccmn_regression_mixed_width() {
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Reg("x0".into()),
            Operand::Imm(0),
            Operand::Cond("eq".into()),
        ];
        assert!(
            encode_ccmp_ccmn(&ops, false).is_err(),
            "ccmn w0, x0, #0, eq must Err; llvm-mc rejects mixed x/w"
        );
    }
}

#[cfg(test)]
mod encode_cinc_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler (CINC Rd, Rn, cond);
    //   algebraic.metamorphic (CINC vs CSINC invert(cond); CINC vs CSET when Rn=ZR);
    //   algebraic.invariant (CSINC field layout); negative_error (arity / extra /
    //   AL-NV / SP-FP-mixed-invalid).
    // Evidence: src/backend/arm/assembler/README.md:5-14 gas-compat;
    //   encoder/mod.rs:1-7 32-bit AArch64 words; encoder/mod.rs:898 cinc dispatch;
    //   compare_branch.rs:292 CINC Rd, Rn, cond -> CSINC Rd, Rn, Rn, invert(cond);
    //   compare_branch.rs:141 CSET Rd, cond -> CSINC Rd, XZR, XZR, invert(cond);
    //   ARM ARM Conditional Increment alias of CSINC (not valid for AL/NV);
    //   ARM ARM CSINC: sf 0 0 11010100 Rm cond 0 1 Rn Rd; invert(cond)=cond XOR 1.
    // Stronger considered:
    //   - State machine: rejected — encode_cinc is a pure function with no lifecycle
    //   - Algebraic round-trip via in-tree decoder: rejected — no CINC decoder
    //   - encode_csinc as differential sibling: rejected — different job (4-operand
    //     CSINC mnemonic/arity); used only as metamorphic alias transform
    // Weaker available: algebraic.invariant (opcode/Rm=Rn/op2/invert fields),
    //   algebraic.metamorphic (vs encode_csinc / encode_cset), negative_error
    //   (arity/extra/AL-NV/SP/FP/mixed)
    // Differential: candidate=encode_cinc, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=[Reg(rd), Reg(rn), Cond(c)] <-> asm text `cinc rd, rn, c`

    use super::{encode_cinc, encode_cset, encode_csinc};
    use super::super::EncodeResult;
    use crate::backend::arm::assembler::parser::Operand;
    use proptest::prelude::*;
    use std::io::Write;
    use std::process::{Command, Stdio};

    const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
    const CASES: u32 = 1000;
    /// Canonical cond names whose encodings are 0..=13 (AL=14 / NV=15 excluded).
    const COND14: [&str; 14] = [
        "eq", "ne", "cs", "cc", "mi", "pl", "vs", "vc",
        "hi", "ls", "ge", "lt", "gt", "le",
    ];
    const COND14_WITH_ALIASES: [&str; 16] = [
        "eq", "ne", "cs", "hs", "cc", "lo", "mi", "pl",
        "vs", "vc", "hi", "ls", "ge", "lt", "gt", "le",
    ];

    fn cfg() -> ProptestConfig {
        ProptestConfig::with_cases(CASES)
    }

    fn parse_llvm_encoding(stdout: &str) -> Result<u32, String> {
        let marker = "encoding: [";
        let start = stdout
            .find(marker)
            .ok_or_else(|| format!("no encoding in stdout: {stdout}"))?;
        let rest = &stdout[start + marker.len()..];
        let end = rest
            .find(']')
            .ok_or_else(|| format!("no closing bracket: {stdout}"))?;
        let inner = &rest[..end];
        let mut bytes = [0u8; 4];
        let parts: Vec<&str> = inner.split(',').collect();
        if parts.len() != 4 {
            return Err(format!("expected 4 bytes, got {inner}"));
        }
        for (i, p) in parts.iter().enumerate() {
            let p = p.trim();
            let hex = p
                .strip_prefix("0x")
                .ok_or_else(|| format!("non-hex byte {p}"))?;
            bytes[i] = u8::from_str_radix(hex, 16).map_err(|e| e.to_string())?;
        }
        Ok(u32::from_le_bytes(bytes))
    }

    fn llvm_mc_word(asm: &str) -> Result<u32, String> {
        let mut child = Command::new(LLVM_MC)
            .args(["-triple=aarch64", "-show-encoding"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("spawn llvm-mc: {e}"))?;
        {
            let mut stdin = child.stdin.take().ok_or("llvm-mc stdin")?;
            stdin
                .write_all(asm.as_bytes())
                .map_err(|e| format!("write llvm-mc: {e}"))?;
            stdin
                .write_all(b"\n")
                .map_err(|e| format!("write llvm-mc: {e}"))?;
        }
        let out = child.wait_with_output().map_err(|e| format!("wait llvm-mc: {e}"))?;
        let stdout = String::from_utf8_lossy(&out.stdout);
        let stderr = String::from_utf8_lossy(&out.stderr);
        if !out.status.success() || stderr.contains("error:") {
            return Err(format!("llvm-mc error: {stderr}"));
        }
        parse_llvm_encoding(&stdout)
    }

    /// ARM ARM invert(cond) = cond XOR 1, expressed as a cond name.
    fn invert_cond_name(c: &str) -> &'static str {
        match c {
            "eq" => "ne",
            "ne" => "eq",
            "cs" => "cc",
            "hs" => "lo",
            "cc" => "cs",
            "lo" => "hs",
            "mi" => "pl",
            "pl" => "mi",
            "vs" => "vc",
            "vc" => "vs",
            "hi" => "ls",
            "ls" => "hi",
            "ge" => "lt",
            "lt" => "ge",
            "gt" => "le",
            "le" => "gt",
            "al" => "nv",
            "nv" => "al",
            other => panic!("not a cond name: {other}"),
        }
    }

    /// n=0..30 -> xN; 31 -> xzr; 32 -> lr.
    fn x_name(n: u32) -> String {
        match n {
            31 => "xzr".to_string(),
            32 => "lr".to_string(),
            n => format!("x{}", n.min(30)),
        }
    }

    /// n=0..30 -> wN; 31 -> wzr. wsp is not in the valid CINC domain.
    fn w_name(n: u32) -> String {
        match n {
            31 => "wzr".to_string(),
            n => format!("w{}", n.min(30)),
        }
    }

    fn x_name_strat() -> impl Strategy<Value = String> {
        prop_oneof![
            Just("x0".to_string()),
            Just("x30".to_string()),
            Just("xzr".to_string()),
            Just("lr".to_string()),
            Just("X0".to_string()),
            (0u32..=32).prop_map(x_name),
        ]
    }

    fn w_name_strat() -> impl Strategy<Value = String> {
        prop_oneof![
            Just("w0".to_string()),
            Just("w30".to_string()),
            Just("wzr".to_string()),
            (0u32..=31).prop_map(w_name),
        ]
    }

    fn same_width_pair() -> impl Strategy<Value = (String, String)> {
        prop_oneof![
            (x_name_strat(), x_name_strat()),
            (w_name_strat(), w_name_strat()),
        ]
    }

    fn cond14_strat() -> impl Strategy<Value = String> {
        prop_oneof![
            Just("eq".to_string()),
            Just("le".to_string()),
            Just("hs".to_string()),
            Just("lo".to_string()),
            Just("gt".to_string()),
            prop::sample::select(
                COND14_WITH_ALIASES.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
            ),
        ]
    }

    fn zr_of(rd: &str) -> String {
        let l = rd.to_ascii_lowercase();
        if l.starts_with('w') || l == "wzr" || l == "wsp" {
            "wzr".to_string()
        } else {
            "xzr".to_string()
        }
    }

    fn extra_operand(which: u32) -> Operand {
        match which {
            0 => Operand::Reg("x2".into()),
            1 => Operand::Imm(0),
            2 => Operand::Symbol("bar".into()),
            _ => Operand::Mem {
                base: "x1".into(),
                offset: 0,
            },
        }
    }

    fn gpr_name(n: u32, is_64: bool) -> String {
        if is_64 {
            if n == 31 {
                "xzr".to_string()
            } else {
                format!("x{}", n.min(30))
            }
        } else if n == 31 {
            "wzr".to_string()
        } else {
            format!("w{}", n.min(30))
        }
    }

    fn bad_ops(kind: u32, n: u32) -> [Operand; 3] {
        let n = n.min(31);
        let eq = Operand::Cond("eq".into());
        match kind {
            0 => [
                Operand::Reg("sp".into()),
                Operand::Reg(format!("x{}", n.min(30))),
                eq,
            ],
            1 => [
                Operand::Reg(format!("x{}", n.min(30))),
                Operand::Reg("sp".into()),
                eq,
            ],
            2 => [
                Operand::Reg("wsp".into()),
                Operand::Reg(format!("w{}", n.min(30))),
                eq,
            ],
            3 => [
                Operand::Reg(format!("x{}", n.min(30))),
                Operand::Reg(format!("w{}", n.min(30))),
                eq,
            ],
            4 => [
                Operand::Reg(format!("d{}", n)),
                Operand::Reg(format!("d{}", n)),
                eq,
            ],
            5 => [
                Operand::Reg(format!("s{}", n)),
                Operand::Reg("w0".into()),
                eq,
            ],
            6 => [
                Operand::Reg(format!("q{}", n)),
                Operand::Reg("x0".into()),
                eq,
            ],
            7 => [
                Operand::Reg(format!("v{}", n)),
                Operand::Reg("x0".into()),
                eq,
            ],
            _ => {
                let name = match n % 8 {
                    0 => "x32".to_string(),
                    1 => "w32".to_string(),
                    2 => "foo".to_string(),
                    3 => "".to_string(),
                    4 => "r0".to_string(),
                    5 => "x".to_string(),
                    6 => "x-1".to_string(),
                    _ => "x99".to_string(),
                };
                [Operand::Reg(name), Operand::Reg("x0".into()), eq]
            }
        }
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_cinc_kat_llvm_mc_x0_x1_eq() {
        let want = 0x9a811420u32;
        let mc = llvm_mc_word("cinc x0, x1, eq").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("x1".into()),
            Operand::Cond("eq".into()),
        ];
        match encode_cinc(&ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for cinc x0, x1, eq, got {:?}",
                want, other
            ),
        }
    }

    #[test]
    fn encode_cinc_kat_llvm_mc_w0_w1_ne() {
        let want = 0x1a810420u32;
        let mc = llvm_mc_word("cinc w0, w1, ne").expect("llvm-mc KAT w-form");
        assert_eq!(mc, want, "llvm-mc KAT w-form mapping broken");
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Reg("w1".into()),
            Operand::Cond("ne".into()),
        ];
        match encode_cinc(&ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for cinc w0, w1, ne, got {:?}",
                want, other
            ),
        }
    }

    #[test]
    fn encode_cinc_kat_cset_alias() {
        let want = 0x9a9f17e0u32;
        let mc = llvm_mc_word("cinc x0, xzr, eq").expect("llvm-mc KAT cset alias");
        assert_eq!(mc, want, "llvm-mc KAT cset-alias mapping broken");
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("xzr".into()),
            Operand::Cond("eq".into()),
        ];
        match encode_cinc(&ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for cinc x0, xzr, eq, got {:?}",
                want, other
            ),
        }
        match encode_cset(&[Operand::Reg("x0".into()), Operand::Cond("eq".into())]) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!("encode_cset KAT mismatch: {:?}", other),
        }
    }

    proptest! {
        #![proptest_config(cfg())]

        // Oracle: differential
        // Target: encoder.compare_branch.encode_cinc
        #[test]
        fn encode_cinc_diff_llvm_mc(
            (rd, rn) in same_width_pair(),
            cond in cond14_strat(),
        ) {
            let asm = format!("cinc {}, {}, {}", rd, rn, cond);
            let ops = [
                Operand::Reg(rd.clone()),
                Operand::Reg(rn.clone()),
                Operand::Cond(cond.clone()),
            ];
            let sut = match encode_cinc(&ops) {
                Ok(EncodeResult::Word(w)) => w,
                other => {
                    return Err(TestCaseError::fail(format!(
                        "SUT rejected valid CINC {}: {:?}",
                        asm, other
                    )));
                }
            };
            let mc = llvm_mc_word(&asm)
                .map_err(|e| TestCaseError::fail(format!(
                    "llvm-mc rejected valid CINC {}: {}",
                    asm, e
                )))?;
            prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
        }

        // Oracle: algebraic.metamorphic
        // Target: encoder.compare_branch.encode_cinc
        #[test]
        fn encode_cinc_meta_vs_csinc(
            (rd, rn) in same_width_pair(),
            cond in cond14_strat(),
        ) {
            let inv = invert_cond_name(&cond).to_string();
            let cinc_ops = [
                Operand::Reg(rd.clone()),
                Operand::Reg(rn.clone()),
                Operand::Cond(cond.clone()),
            ];
            let csinc_ops = [
                Operand::Reg(rd.clone()),
                Operand::Reg(rn.clone()),
                Operand::Reg(rn.clone()),
                Operand::Cond(inv),
            ];
            let left = encode_cinc(&cinc_ops).map_err(|e| TestCaseError::fail(e))?;
            let right = encode_csinc(&csinc_ops).map_err(|e| TestCaseError::fail(e))?;
            match (left, right) {
                (EncodeResult::Word(w_cinc), EncodeResult::Word(w_csinc)) => {
                    prop_assert_eq!(
                        w_cinc, w_csinc,
                        "CINC {}, {}, {} must equal CSINC {}, {}, {}, {}",
                        rd, rn, cond, rd, rn, rn, invert_cond_name(&cond)
                    );
                }
                other => {
                    return Err(TestCaseError::fail(format!(
                        "expected Word pair, got {:?}",
                        other
                    )));
                }
            }
        }

        // Oracle: algebraic.metamorphic
        // Target: encoder.compare_branch.encode_cinc
        #[test]
        fn encode_cinc_meta_vs_cset(
            rd in prop_oneof![x_name_strat(), w_name_strat()],
            cond in cond14_strat(),
        ) {
            let zr = zr_of(&rd);
            let cinc_ops = [
                Operand::Reg(rd.clone()),
                Operand::Reg(zr),
                Operand::Cond(cond.clone()),
            ];
            let cset_ops = [Operand::Reg(rd.clone()), Operand::Cond(cond.clone())];
            let left = encode_cinc(&cinc_ops).map_err(|e| TestCaseError::fail(e))?;
            let right = encode_cset(&cset_ops).map_err(|e| TestCaseError::fail(e))?;
            match (left, right) {
                (EncodeResult::Word(w_cinc), EncodeResult::Word(w_cset)) => {
                    prop_assert_eq!(
                        w_cinc, w_cset,
                        "CINC {}, ZR, {} must equal CSET {}, {}",
                        rd, cond, rd, cond
                    );
                }
                other => {
                    return Err(TestCaseError::fail(format!(
                        "expected Word pair, got {:?}",
                        other
                    )));
                }
            }
        }

        // Oracle: algebraic.invariant
        // Target: encoder.compare_branch.encode_cinc
        #[test]
        fn encode_cinc_word_layout(
            rd_n in 0u32..=31u32,
            rn_n in 0u32..=31u32,
            is_64 in any::<bool>(),
            cond_enc in 0u32..=13u32,
        ) {
            let rd = gpr_name(rd_n, is_64);
            let rn = gpr_name(rn_n, is_64);
            let cond = COND14[cond_enc as usize];
            let ops = [
                Operand::Reg(rd),
                Operand::Reg(rn),
                Operand::Cond(cond.to_string()),
            ];
            match encode_cinc(&ops) {
                Ok(EncodeResult::Word(w)) => {
                    let sf = if is_64 { 1u32 } else { 0u32 };
                    let inv = cond_enc ^ 1;
                    prop_assert_eq!((w >> 31) & 1, sf, "sf bit 31");
                    prop_assert_eq!((w >> 30) & 1, 0u32, "op bit 30");
                    prop_assert_eq!((w >> 29) & 1, 0u32, "S bit 29");
                    prop_assert_eq!((w >> 21) & 0xFF, 0b11010100u32, "bits[28:21]");
                    prop_assert_eq!((w >> 16) & 0x1F, rn_n, "Rm [20:16] == Rn");
                    prop_assert_eq!((w >> 12) & 0xF, inv, "cond [15:12] == invert");
                    prop_assert_eq!((w >> 10) & 0x3, 0b01u32, "op2 [11:10]");
                    prop_assert_eq!((w >> 5) & 0x1F, rn_n, "Rn [9:5]");
                    prop_assert_eq!(w & 0x1F, rd_n, "Rd [4:0]");
                }
                other => {
                    return Err(TestCaseError::fail(format!(
                        "expected Word, got {:?}",
                        other
                    )));
                }
            }
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_cinc
        #[test]
        fn encode_cinc_neg_arity(arity in 0u32..=2u32) {
            let ops: Vec<Operand> = match arity {
                0 => vec![],
                1 => vec![Operand::Reg("x0".into())],
                _ => vec![Operand::Reg("x0".into()), Operand::Reg("x1".into())],
            };
            prop_assert!(
                encode_cinc(&ops).is_err(),
                "cinc with {} operand(s) must Err (llvm-mc: too few operands)",
                arity
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_cinc
        #[test]
        fn encode_cinc_neg_extra_operand(
            (rd, rn) in same_width_pair(),
            cond in cond14_strat(),
            which in 0u32..=3u32,
        ) {
            let extra = extra_operand(which);
            let ops = [
                Operand::Reg(rd.clone()),
                Operand::Reg(rn.clone()),
                Operand::Cond(cond.clone()),
                extra,
            ];
            prop_assert!(
                encode_cinc(&ops).is_err(),
                "cinc {}, {}, {}, extra (which={}) must Err (llvm-mc: invalid operand)",
                rd, rn, cond, which
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_cinc
        #[test]
        fn encode_cinc_neg_al_nv(
            (rd, rn) in same_width_pair(),
            which in 0u32..=1u32,
        ) {
            let cond = if which == 0 { "al" } else { "nv" };
            let ops = [
                Operand::Reg(rd.clone()),
                Operand::Reg(rn.clone()),
                Operand::Cond(cond.to_string()),
            ];
            prop_assert!(
                encode_cinc(&ops).is_err(),
                "cinc {}, {}, {} must Err (llvm-mc: AL and NV invalid for CINC)",
                rd, rn, cond
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_cinc
        #[test]
        fn encode_cinc_neg_wrong_reg(kind in 0u32..=8u32, n in 0u32..=31u32) {
            let ops = bad_ops(kind, n);
            prop_assert!(
                encode_cinc(&ops).is_err(),
                "cinc with wrong-reg kind={} n={} must Err (llvm-mc rejects SP/FP/mixed/invalid)",
                kind, n
            );
        }

        // Oracle: negative_error (coverage sweep: encode_cond None arm)
        // Target: encoder.compare_branch.encode_cinc
        #[test]
        fn encode_cinc_neg_unknown_cond(
            (rd, rn) in same_width_pair(),
            which in 0u32..=5u32,
        ) {
            let cond = match which {
                0 => "zz",
                1 => "foo",
                2 => "eqq",
                3 => "",
                4 => "eq ",
                _ => "always",
            };
            let ops = [
                Operand::Reg(rd),
                Operand::Reg(rn),
                Operand::Cond(cond.to_string()),
            ];
            prop_assert!(
                encode_cinc(&ops).is_err(),
                "cinc with unknown cond '{}' must Err",
                cond
            );
        }

        // Oracle: negative_error (coverage sweep: get_reg parse_reg_num None arm)
        // Target: encoder.compare_branch.encode_cinc
        #[test]
        fn encode_cinc_neg_invalid_name(which in 0u32..=7u32) {
            let name = match which {
                0 => "x32",
                1 => "w32",
                2 => "foo",
                3 => "",
                4 => "r0",
                5 => "x",
                6 => "x-1",
                _ => "x99",
            };
            let ops = [
                Operand::Reg(name.to_string()),
                Operand::Reg("x0".into()),
                Operand::Cond("eq".into()),
            ];
            prop_assert!(
                encode_cinc(&ops).is_err(),
                "cinc {} , x0, eq must Err (not a valid register name)",
                name
            );
        }

        // Oracle: negative_error (coverage sweep: get_reg non-Reg / cond not Cond)
        // Target: encoder.compare_branch.encode_cinc
        #[test]
        fn encode_cinc_neg_bad_operand_kind(slot in 0u32..=2u32, which in 0u32..=4u32) {
            let bad = match which {
                0 => Operand::Imm(0),
                1 => Operand::Mem { base: "x0".into(), offset: 0 },
                2 => Operand::Symbol("foo".into()),
                3 => Operand::Shift { kind: "lsl".into(), amount: 0 },
                _ => Operand::Label("foo".into()),
            };
            let mut ops = vec![
                Operand::Reg("x0".into()),
                Operand::Reg("x1".into()),
                Operand::Cond("eq".into()),
            ];
            ops[slot as usize] = bad;
            prop_assert!(
                encode_cinc(&ops).is_err(),
                "cinc with non-Reg/non-Cond at slot {} which={} must Err",
                slot, which
            );
        }
    }

    #[test]
    fn test_encode_cinc_regression_extra_operand() {
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("x0".into()),
            Operand::Cond("eq".into()),
            Operand::Reg("x2".into()),
        ];
        assert!(
            encode_cinc(&ops).is_err(),
            "cinc x0, x0, eq, x2 must Err; llvm-mc rejects a fourth operand"
        );
    }

    #[test]
    fn test_encode_cinc_regression_al() {
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("x0".into()),
            Operand::Cond("al".into()),
        ];
        assert!(
            encode_cinc(&ops).is_err(),
            "cinc x0, x0, al must Err; llvm-mc rejects AL/NV for CINC"
        );
    }

    #[test]
    fn test_encode_cinc_regression_nv() {
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("x0".into()),
            Operand::Cond("nv".into()),
        ];
        assert!(
            encode_cinc(&ops).is_err(),
            "cinc x0, x0, nv must Err; llvm-mc rejects AL/NV for CINC"
        );
    }

    #[test]
    fn test_encode_cinc_regression_sp() {
        let ops = [
            Operand::Reg("sp".into()),
            Operand::Reg("x0".into()),
            Operand::Cond("eq".into()),
        ];
        assert!(
            encode_cinc(&ops).is_err(),
            "cinc sp, x0, eq must Err; llvm-mc rejects SP (register 31 is XZR)"
        );
    }

    #[test]
    fn test_encode_cinc_regression_mixed_width() {
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("w0".into()),
            Operand::Cond("eq".into()),
        ];
        assert!(
            encode_cinc(&ops).is_err(),
            "cinc x0, w0, eq must Err; llvm-mc rejects mixed x/w"
        );
    }

    #[test]
    fn test_encode_cinc_regression_fp_reg() {
        let ops = [
            Operand::Reg("d0".into()),
            Operand::Reg("d0".into()),
            Operand::Cond("eq".into()),
        ];
        assert!(
            encode_cinc(&ops).is_err(),
            "cinc d0, d0, eq must Err; llvm-mc rejects FP/SIMD registers"
        );
    }
}
