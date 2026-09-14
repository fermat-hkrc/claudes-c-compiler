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
