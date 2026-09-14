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

#[cfg(test)]
mod encode_cinv_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler (CINV Rd, Rn, cond);
    //   algebraic.metamorphic (CINV vs CSINV invert(cond); CINV vs CSETM when Rn=ZR);
    //   algebraic.invariant (CSINV field layout); negative_error (arity / extra /
    //   AL-NV / SP-FP-mixed-invalid).
    // Evidence: src/backend/arm/assembler/README.md:5-14 gas-compat;
    //   encoder/mod.rs:1-7 32-bit AArch64 words; encoder/mod.rs:899 cinv dispatch;
    //   compare_branch.rs:308 CINV Rd, Rn, cond -> CSINV Rd, Rn, Rn, invert(cond);
    //   compare_branch.rs:155 CSETM Rd, cond -> CSINV Rd, XZR, XZR, invert(cond);
    //   ARM ARM Conditional Invert alias of CSINV (not valid for AL/NV);
    //   ARM ARM CSINV: sf 1 0 11010100 Rm cond 0 0 Rn Rd; invert(cond)=cond XOR 1.
    // Stronger considered:
    //   - State machine: rejected — encode_cinv is a pure function with no lifecycle
    //   - Algebraic round-trip via in-tree decoder: rejected — no CINV decoder
    //   - encode_csinv as differential sibling: rejected — different job (4-operand
    //     CSINV mnemonic/arity); used only as metamorphic alias transform
    // Weaker available: algebraic.invariant (opcode/Rm=Rn/op2/invert fields),
    //   algebraic.metamorphic (vs encode_csinv / encode_csetm), negative_error
    //   (arity/extra/AL-NV/SP/FP/mixed)
    // Differential: candidate=encode_cinv, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=[Reg(rd), Reg(rn), Cond(c)] <-> asm text `cinv rd, rn, c`

    use super::{encode_cinv, encode_csetm, encode_csinv};
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

    /// n=0..30 -> wN; 31 -> wzr. wsp is not in the valid CINV domain.
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
    fn encode_cinv_kat_llvm_mc_x0_x1_eq() {
        let want = 0xda811020u32;
        let mc = llvm_mc_word("cinv x0, x1, eq").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("x1".into()),
            Operand::Cond("eq".into()),
        ];
        match encode_cinv(&ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for cinv x0, x1, eq, got {:?}",
                want, other
            ),
        }
    }

    #[test]
    fn encode_cinv_kat_llvm_mc_w0_w1_ne() {
        let want = 0x5a810020u32;
        let mc = llvm_mc_word("cinv w0, w1, ne").expect("llvm-mc KAT w-form");
        assert_eq!(mc, want, "llvm-mc KAT w-form mapping broken");
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Reg("w1".into()),
            Operand::Cond("ne".into()),
        ];
        match encode_cinv(&ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for cinv w0, w1, ne, got {:?}",
                want, other
            ),
        }
    }

    #[test]
    fn encode_cinv_kat_csetm_alias() {
        let want = 0xda9f13e0u32;
        let mc = llvm_mc_word("cinv x0, xzr, eq").expect("llvm-mc KAT csetm alias");
        assert_eq!(mc, want, "llvm-mc KAT csetm-alias mapping broken");
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("xzr".into()),
            Operand::Cond("eq".into()),
        ];
        match encode_cinv(&ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for cinv x0, xzr, eq, got {:?}",
                want, other
            ),
        }
        match encode_csetm(&[Operand::Reg("x0".into()), Operand::Cond("eq".into())]) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!("encode_csetm KAT mismatch: {:?}", other),
        }
    }

    proptest! {
        #![proptest_config(cfg())]

        // Oracle: differential
        // Target: encoder.compare_branch.encode_cinv
        #[test]
        fn encode_cinv_diff_llvm_mc(
            (rd, rn) in same_width_pair(),
            cond in cond14_strat(),
        ) {
            let asm = format!("cinv {}, {}, {}", rd, rn, cond);
            let ops = [
                Operand::Reg(rd.clone()),
                Operand::Reg(rn.clone()),
                Operand::Cond(cond.clone()),
            ];
            let sut = match encode_cinv(&ops) {
                Ok(EncodeResult::Word(w)) => w,
                other => {
                    return Err(TestCaseError::fail(format!(
                        "SUT rejected valid CINV {}: {:?}",
                        asm, other
                    )));
                }
            };
            let mc = llvm_mc_word(&asm)
                .map_err(|e| TestCaseError::fail(format!(
                    "llvm-mc rejected valid CINV {}: {}",
                    asm, e
                )))?;
            prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
        }

        // Oracle: algebraic.metamorphic
        // Target: encoder.compare_branch.encode_cinv
        #[test]
        fn encode_cinv_meta_vs_csinv(
            (rd, rn) in same_width_pair(),
            cond in cond14_strat(),
        ) {
            let inv = invert_cond_name(&cond).to_string();
            let cinv_ops = [
                Operand::Reg(rd.clone()),
                Operand::Reg(rn.clone()),
                Operand::Cond(cond.clone()),
            ];
            let csinv_ops = [
                Operand::Reg(rd.clone()),
                Operand::Reg(rn.clone()),
                Operand::Reg(rn.clone()),
                Operand::Cond(inv),
            ];
            let left = encode_cinv(&cinv_ops).map_err(|e| TestCaseError::fail(e))?;
            let right = encode_csinv(&csinv_ops).map_err(|e| TestCaseError::fail(e))?;
            match (left, right) {
                (EncodeResult::Word(w_cinv), EncodeResult::Word(w_csinv)) => {
                    prop_assert_eq!(
                        w_cinv, w_csinv,
                        "CINV {}, {}, {} must equal CSINV {}, {}, {}, {}",
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
        // Target: encoder.compare_branch.encode_cinv
        #[test]
        fn encode_cinv_meta_vs_csetm(
            rd in prop_oneof![x_name_strat(), w_name_strat()],
            cond in cond14_strat(),
        ) {
            let zr = zr_of(&rd);
            let cinv_ops = [
                Operand::Reg(rd.clone()),
                Operand::Reg(zr),
                Operand::Cond(cond.clone()),
            ];
            let csetm_ops = [Operand::Reg(rd.clone()), Operand::Cond(cond.clone())];
            let left = encode_cinv(&cinv_ops).map_err(|e| TestCaseError::fail(e))?;
            let right = encode_csetm(&csetm_ops).map_err(|e| TestCaseError::fail(e))?;
            match (left, right) {
                (EncodeResult::Word(w_cinv), EncodeResult::Word(w_csetm)) => {
                    prop_assert_eq!(
                        w_cinv, w_csetm,
                        "CINV {}, ZR, {} must equal CSETM {}, {}",
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
        // Target: encoder.compare_branch.encode_cinv
        #[test]
        fn encode_cinv_word_layout(
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
            match encode_cinv(&ops) {
                Ok(EncodeResult::Word(w)) => {
                    let sf = if is_64 { 1u32 } else { 0u32 };
                    let inv = cond_enc ^ 1;
                    prop_assert_eq!((w >> 31) & 1, sf, "sf bit 31");
                    prop_assert_eq!((w >> 30) & 1, 1u32, "op bit 30");
                    prop_assert_eq!((w >> 29) & 1, 0u32, "S bit 29");
                    prop_assert_eq!((w >> 21) & 0xFF, 0b11010100u32, "bits[28:21]");
                    prop_assert_eq!((w >> 16) & 0x1F, rn_n, "Rm [20:16] == Rn");
                    prop_assert_eq!((w >> 12) & 0xF, inv, "cond [15:12] == invert");
                    prop_assert_eq!((w >> 10) & 0x3, 0b00u32, "op2 [11:10]");
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
        // Target: encoder.compare_branch.encode_cinv
        #[test]
        fn encode_cinv_neg_arity(arity in 0u32..=2u32) {
            let ops: Vec<Operand> = match arity {
                0 => vec![],
                1 => vec![Operand::Reg("x0".into())],
                _ => vec![Operand::Reg("x0".into()), Operand::Reg("x1".into())],
            };
            prop_assert!(
                encode_cinv(&ops).is_err(),
                "cinv with {} operand(s) must Err (llvm-mc: too few operands)",
                arity
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_cinv
        #[test]
        fn encode_cinv_neg_extra_operand(
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
                encode_cinv(&ops).is_err(),
                "cinv {}, {}, {}, extra (which={}) must Err (llvm-mc: invalid operand)",
                rd, rn, cond, which
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_cinv
        #[test]
        fn encode_cinv_neg_al_nv(
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
                encode_cinv(&ops).is_err(),
                "cinv {}, {}, {} must Err (llvm-mc: AL and NV invalid for CINV)",
                rd, rn, cond
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_cinv
        #[test]
        fn encode_cinv_neg_wrong_reg(kind in 0u32..=8u32, n in 0u32..=31u32) {
            let ops = bad_ops(kind, n);
            prop_assert!(
                encode_cinv(&ops).is_err(),
                "cinv with wrong-reg kind={} n={} must Err (llvm-mc rejects SP/FP/mixed/invalid)",
                kind, n
            );
        }

        // Oracle: negative_error (coverage sweep: encode_cond None arm)
        // Target: encoder.compare_branch.encode_cinv
        #[test]
        fn encode_cinv_neg_unknown_cond(
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
                encode_cinv(&ops).is_err(),
                "cinv with unknown cond '{}' must Err",
                cond
            );
        }

        // Oracle: negative_error (coverage sweep: get_reg parse_reg_num None arm)
        // Target: encoder.compare_branch.encode_cinv
        #[test]
        fn encode_cinv_neg_invalid_name(which in 0u32..=7u32) {
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
                encode_cinv(&ops).is_err(),
                "cinv {} , x0, eq must Err (not a valid register name)",
                name
            );
        }

        // Oracle: negative_error (coverage sweep: get_reg non-Reg / cond not Cond)
        // Target: encoder.compare_branch.encode_cinv
        #[test]
        fn encode_cinv_neg_bad_operand_kind(slot in 0u32..=2u32, which in 0u32..=4u32) {
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
                encode_cinv(&ops).is_err(),
                "cinv with non-Reg/non-Cond at slot {} which={} must Err",
                slot, which
            );
        }
    }

    #[test]
    fn test_encode_cinv_regression_extra_operand() {
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("x0".into()),
            Operand::Cond("eq".into()),
            Operand::Reg("x2".into()),
        ];
        assert!(
            encode_cinv(&ops).is_err(),
            "cinv x0, x0, eq, x2 must Err; llvm-mc rejects a fourth operand"
        );
    }

    #[test]
    fn test_encode_cinv_regression_al() {
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("x0".into()),
            Operand::Cond("al".into()),
        ];
        assert!(
            encode_cinv(&ops).is_err(),
            "cinv x0, x0, al must Err; llvm-mc rejects AL/NV for CINV"
        );
    }

    #[test]
    fn test_encode_cinv_regression_nv() {
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("x0".into()),
            Operand::Cond("nv".into()),
        ];
        assert!(
            encode_cinv(&ops).is_err(),
            "cinv x0, x0, nv must Err; llvm-mc rejects AL/NV for CINV"
        );
    }

    #[test]
    fn test_encode_cinv_regression_sp() {
        let ops = [
            Operand::Reg("sp".into()),
            Operand::Reg("x0".into()),
            Operand::Cond("eq".into()),
        ];
        assert!(
            encode_cinv(&ops).is_err(),
            "cinv sp, x0, eq must Err; llvm-mc rejects SP (register 31 is XZR)"
        );
    }

    #[test]
    fn test_encode_cinv_regression_mixed_width() {
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("w0".into()),
            Operand::Cond("eq".into()),
        ];
        assert!(
            encode_cinv(&ops).is_err(),
            "cinv x0, w0, eq must Err; llvm-mc rejects mixed x/w"
        );
    }

    #[test]
    fn test_encode_cinv_regression_fp_reg() {
        let ops = [
            Operand::Reg("d0".into()),
            Operand::Reg("d0".into()),
            Operand::Cond("eq".into()),
        ];
        assert!(
            encode_cinv(&ops).is_err(),
            "cinv d0, d0, eq must Err; llvm-mc rejects FP/SIMD registers"
        );
    }
}

#[cfg(test)]
mod encode_cmn_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler (CMN Rn, #imm / Rn, Rm{, shift});
    //   algebraic.metamorphic (CMN vs ADDS XZR/WZR); algebraic.invariant (ADDS-imm
    //   field layout with Rd=31); negative_error (arity / imm range / extra / wrong-reg).
    // Evidence: src/backend/arm/assembler/README.md:5-14 gas-compat;
    //   README.md:218 Compare lists cmn; README.md:507 compare_branch.rs CMP/CMN/TST;
    //   encoder/mod.rs:1-7 32-bit AArch64 words; encoder/mod.rs:302 cmn dispatch;
    //   compare_branch.rs:23 CMN Rn, op -> ADDS XZR, Rn, op; codegen/emit.rs:511-562
    //   cmn wN/xN, #imm12; ARM ARM Compare Negative alias of ADDS (Rd=XZR/WZR).
    // Stronger considered:
    //   - State machine: rejected — encode_cmn is a pure function with no lifecycle
    //   - Algebraic round-trip via in-tree decoder: rejected — no CMN decoder
    //   - encode_cmp as differential sibling: rejected — different job (CMP/SUBS-XZR)
    //   - encode_add_sub as differential sibling: rejected — different job (3-operand
    //     ADDS mnemonic/arity); used only as metamorphic alias transform
    // Weaker available: algebraic.invariant (Rd=31/S=1/op=0/imm12 fields),
    //   algebraic.metamorphic (vs encode_add_sub), negative_error
    //   (arity/imm-oor/extra/XZR-imm/mixed/FP/SP-as-Rm)
    // Differential: candidate=encode_cmn, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=[Reg(rn), Imm(imm)] <-> `cmn rn, #imm`;
    //   [Reg(rn), Reg(rm){, Shift}] <-> `cmn rn, rm{, shift}`

    use super::encode_cmn;
    use super::super::{encode_add_sub, EncodeResult};
    use crate::backend::arm::assembler::parser::Operand;
    use proptest::prelude::*;
    use std::io::Write;
    use std::process::{Command, Stdio};

    const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
    const CASES: u32 = 1000;
    const IMM12_MAX: i64 = 4095;
    const IMM12_SHIFTED_MAX: i64 = 4095 << 12; // 16773120

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

    fn word_of(r: Result<EncodeResult, String>) -> Result<u32, TestCaseError> {
        match r {
            Ok(EncodeResult::Word(w)) => Ok(w),
            other => Err(TestCaseError::fail(format!("expected Word, got {:?}", other))),
        }
    }

    /// Immediate-form Rn: X/W GPR plus SP/WSP/LR. XZR/WZR are invalid for CMN #imm.
    fn imm_rn_name(n: u32) -> String {
        match n {
            31 => "sp".to_string(),
            32 => "lr".to_string(),
            33 => "wsp".to_string(),
            n if n >= 34 => format!("w{}", (n - 34).min(30)),
            n => format!("x{}", n.min(30)),
        }
    }

    fn imm_rn_strat() -> impl Strategy<Value = String> {
        prop_oneof![
            Just("x0".to_string()),
            Just("x30".to_string()),
            Just("sp".to_string()),
            Just("lr".to_string()),
            Just("w0".to_string()),
            Just("w30".to_string()),
            Just("wsp".to_string()),
            Just("X0".to_string()),
            (0u32..=63).prop_map(imm_rn_name),
        ]
    }

    /// Unshifted imm12 and auto-shifted (N<<12). Bounds 0/1/4095/4096/16773120 forced.
    fn imm_strat() -> impl Strategy<Value = i64> {
        prop_oneof![
            Just(0i64),
            Just(1i64),
            Just(IMM12_MAX),
            Just(4096i64),
            Just(8192i64),
            Just(IMM12_SHIFTED_MAX),
            0i64..=IMM12_MAX,
            (1i64..=IMM12_MAX).prop_map(|n| n << 12),
        ]
    }

    fn explicit_imm12_strat() -> impl Strategy<Value = i64> {
        prop_oneof![
            Just(0i64),
            Just(1i64),
            Just(IMM12_MAX),
            0i64..=IMM12_MAX,
        ]
    }

    fn x_name(n: u32) -> String {
        match n {
            31 => "xzr".to_string(),
            32 => "lr".to_string(),
            n => format!("x{}", n.min(30)),
        }
    }

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

    fn is_32_name(rn: &str) -> bool {
        let l = rn.to_ascii_lowercase();
        l.starts_with('w') || l == "wsp" || l == "wzr"
    }

    fn zr_of(rn: &str) -> String {
        if is_32_name(rn) {
            "wzr".to_string()
        } else {
            "xzr".to_string()
        }
    }

    fn shift_kind_strat() -> impl Strategy<Value = Option<(String, u32)>> {
        prop_oneof![
            Just(None),
            Just(Some(("lsl".to_string(), 0u32))),
            Just(Some(("lsl".to_string(), 3u32))),
            Just(Some(("lsr".to_string(), 0u32))),
            Just(Some(("asr".to_string(), 4u32))),
            (0u32..=2, 0u32..=63).prop_map(|(k, a)| {
                let kind = match k {
                    0 => "lsl",
                    1 => "lsr",
                    _ => "asr",
                };
                Some((kind.to_string(), a))
            }),
        ]
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

    fn unencodable_imm() -> impl Strategy<Value = i64> {
        prop_oneof![
            Just(4097i64),
            Just(4098i64),
            Just(8191i64),
            Just(IMM12_SHIFTED_MAX + 1),
            Just(IMM12_SHIFTED_MAX + 4096),
            Just(-4097i64),
            Just(-8191i64),
            Just(i64::MAX),
            Just(i64::MIN),
            (1i64..=4095).prop_map(|n| 4096 + n), // 4097..=8191, none are N<<12
            (1i64..=1024).prop_map(|k| IMM12_SHIFTED_MAX + k),
            (1i64..=1024).prop_map(|k| -(4096 + k)),
        ]
    }

    fn reg_num(name: &str) -> u32 {
        let n = name.to_ascii_lowercase();
        match n.as_str() {
            "sp" | "wsp" | "xzr" | "wzr" => 31,
            "lr" => 30,
            s => s[1..].parse().unwrap_or(0),
        }
    }

    fn sf_of(name: &str) -> u32 {
        if is_32_name(name) {
            0
        } else {
            1
        }
    }

    fn extend_case() -> impl Strategy<Value = (String, String, String, u32)> {
        // (rn, rm, extend, amount in 0..=4). Amount bounds 0 and 4 forced.
        let amt = prop_oneof![Just(0u32), Just(4u32), 0u32..=4u32];
        prop_oneof![
            // 64-bit Rn, Wm, sxtw/uxtw
            (
                prop_oneof![
                    Just("x0".to_string()),
                    Just("x30".to_string()),
                    Just("sp".to_string()),
                    Just("lr".to_string()),
                    (0u32..=30).prop_map(|n| format!("x{}", n)),
                ],
                (0u32..=30).prop_map(|n| format!("w{}", n)),
                prop_oneof![Just("sxtw".to_string()), Just("uxtw".to_string())],
                amt.clone(),
            ),
            // 64-bit Rn, Xm, sxtx/uxtx
            (
                prop_oneof![
                    Just("x0".to_string()),
                    Just("sp".to_string()),
                    (0u32..=30).prop_map(|n| format!("x{}", n)),
                ],
                x_name_strat(),
                prop_oneof![Just("sxtx".to_string()), Just("uxtx".to_string())],
                amt.clone(),
            ),
            // 32-bit Rn, Wm, uxtw
            (
                prop_oneof![
                    Just("w0".to_string()),
                    Just("wsp".to_string()),
                    (0u32..=30).prop_map(|n| format!("w{}", n)),
                ],
                w_name_strat(),
                Just("uxtw".to_string()),
                amt,
            ),
        ]
    }

    fn neg_imm_strat() -> impl Strategy<Value = i64> {
        prop_oneof![
            Just(1i64),
            Just(4095i64),
            1i64..=4095i64,
        ]
    }

    fn non_reg_first(kind: u32) -> Operand {
        match kind {
            0 => Operand::Imm(0),
            1 => Operand::Symbol("foo".into()),
            2 => Operand::Mem {
                base: "x0".into(),
                offset: 0,
            },
            3 => Operand::Cond("eq".into()),
            _ => Operand::Shift {
                kind: "lsl".into(),
                amount: 0,
            },
        }
    }

    fn invalid_name(which: u32) -> String {
        match which {
            0 => "x32".to_string(),
            1 => "w32".to_string(),
            2 => "foo".to_string(),
            3 => "".to_string(),
            4 => "r0".to_string(),
            5 => "x".to_string(),
            6 => "x-1".to_string(),
            _ => "x99".to_string(),
        }
    }

    fn wrong_reg_ops(kind: u32, n: u32, imm: i64) -> Vec<Operand> {
        let n = n.min(31);
        match kind {
            0 => vec![Operand::Reg("xzr".into()), Operand::Imm(imm.abs() % 4096)],
            1 => vec![Operand::Reg("wzr".into()), Operand::Imm(imm.abs() % 4096)],
            2 => vec![
                Operand::Reg(format!("x{}", n.min(30))),
                Operand::Reg(format!("w{}", n.min(30))),
            ],
            3 => vec![
                Operand::Reg(format!("w{}", n.min(30))),
                Operand::Reg(format!("x{}", n.min(30))),
            ],
            4 => vec![Operand::Reg(format!("d{}", n)), Operand::Imm(0)],
            5 => vec![
                Operand::Reg(format!("x{}", n.min(30))),
                Operand::Reg(format!("d{}", n)),
            ],
            6 => vec![
                Operand::Reg(format!("x{}", n.min(30))),
                Operand::Reg("sp".into()),
            ],
            7 => vec![Operand::Reg(invalid_name(n % 8)), Operand::Imm(0)],
            _ => vec![
                Operand::Reg(format!("s{}", n)),
                Operand::Reg("w0".into()),
            ],
        }
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_cmn_kat_llvm_mc_x0_imm42() {
        let want = 0xb100a81fu32;
        let mc = llvm_mc_word("cmn x0, #42").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [Operand::Reg("x0".into()), Operand::Imm(42)];
        match encode_cmn(&ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for cmn x0, #42, got {:?}",
                want, other
            ),
        }
    }

    #[test]
    fn encode_cmn_kat_llvm_mc_w0_imm42() {
        let want = 0x3100a81fu32;
        let mc = llvm_mc_word("cmn w0, #42").expect("llvm-mc KAT w");
        assert_eq!(mc, want, "llvm-mc KAT w mapping broken");
        let ops = [Operand::Reg("w0".into()), Operand::Imm(42)];
        match encode_cmn(&ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for cmn w0, #42, got {:?}",
                want, other
            ),
        }
    }

    #[test]
    fn encode_cmn_kat_llvm_mc_x0_x1() {
        let want = 0xab01001fu32;
        let mc = llvm_mc_word("cmn x0, x1").expect("llvm-mc KAT reg");
        assert_eq!(mc, want, "llvm-mc KAT reg mapping broken");
        let ops = [Operand::Reg("x0".into()), Operand::Reg("x1".into())];
        match encode_cmn(&ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for cmn x0, x1, got {:?}",
                want, other
            ),
        }
    }

    #[test]
    fn encode_cmn_kat_llvm_mc_sp_imm0() {
        let want = 0xb10003ffu32;
        let mc = llvm_mc_word("cmn sp, #0").expect("llvm-mc KAT sp");
        assert_eq!(mc, want, "llvm-mc KAT sp mapping broken");
        let ops = [Operand::Reg("sp".into()), Operand::Imm(0)];
        match encode_cmn(&ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for cmn sp, #0, got {:?}",
                want, other
            ),
        }
    }

    #[test]
    fn encode_cmn_kat_llvm_mc_extend_sxtw() {
        let want = 0xab21c01fu32;
        let mc = llvm_mc_word("cmn x0, w1, sxtw").expect("llvm-mc KAT extend");
        assert_eq!(mc, want, "llvm-mc KAT extend mapping broken");
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("w1".into()),
            Operand::Extend {
                kind: "sxtw".into(),
                amount: 0,
            },
        ];
        match encode_cmn(&ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for cmn x0, w1, sxtw, got {:?}",
                want, other
            ),
        }
    }

    #[test]
    fn encode_cmn_kat_llvm_mc_neg_imm() {
        let want = 0xf100041fu32; // cmp x0, #1
        let mc = llvm_mc_word("cmn x0, #-1").expect("llvm-mc KAT neg");
        assert_eq!(mc, want, "llvm-mc KAT neg mapping broken");
        let ops = [Operand::Reg("x0".into()), Operand::Imm(-1)];
        match encode_cmn(&ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for cmn x0, #-1, got {:?}",
                want, other
            ),
        }
    }

    #[test]
    fn encode_cmn_kat_adds_alias() {
        let mc = llvm_mc_word("adds xzr, x0, #42").expect("llvm-mc ADDS alias");
        let ops = [Operand::Reg("x0".into()), Operand::Imm(42)];
        match encode_cmn(&ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, mc),
            other => panic!("CMN must match ADDS XZR alias, got {:?}", other),
        }
    }

    proptest! {
        #![proptest_config(cfg())]

        // Oracle: differential
        // Target: encoder.compare_branch.encode_cmn
        #[test]
        fn encode_cmn_diff_imm_llvm_mc(
            rn in imm_rn_strat(),
            imm in imm_strat(),
            mode in 0u32..=2u32,
            expl in explicit_imm12_strat(),
        ) {
            let (ops, asm) = if mode == 2 {
                let ops = vec![
                    Operand::Reg(rn.clone()),
                    Operand::Imm(expl),
                    Operand::Shift { kind: "lsl".into(), amount: 12 },
                ];
                let asm = format!("cmn {}, #{}, lsl #12", rn, expl);
                (ops, asm)
            } else {
                let ops = vec![Operand::Reg(rn.clone()), Operand::Imm(imm)];
                let asm = format!("cmn {}, #{}", rn, imm);
                (ops, asm)
            };
            let sut = match encode_cmn(&ops) {
                Ok(EncodeResult::Word(w)) => w,
                other => {
                    return Err(TestCaseError::fail(format!(
                        "SUT rejected valid CMN {}: {:?}",
                        asm, other
                    )));
                }
            };
            let mc = llvm_mc_word(&asm)
                .map_err(|e| TestCaseError::fail(format!(
                    "llvm-mc rejected valid CMN {}: {}",
                    asm, e
                )))?;
            prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
        }

        // Oracle: differential
        // Target: encoder.compare_branch.encode_cmn
        #[test]
        fn encode_cmn_diff_reg_llvm_mc(
            pair in same_width_pair(),
            shift in shift_kind_strat(),
        ) {
            let (rn, rm) = pair;
            let is32 = is_32_name(&rn);
            let mut ops = vec![Operand::Reg(rn.clone()), Operand::Reg(rm.clone())];
            let mut asm = format!("cmn {}, {}", rn, rm);
            if let Some((kind, raw_amt)) = shift {
                let max_amt = if is32 { 31u32 } else { 63u32 };
                let amt = raw_amt.min(max_amt);
                ops.push(Operand::Shift { kind: kind.clone(), amount: amt });
                asm = format!("cmn {}, {}, {} #{}", rn, rm, kind, amt);
            }
            let sut = match encode_cmn(&ops) {
                Ok(EncodeResult::Word(w)) => w,
                other => {
                    return Err(TestCaseError::fail(format!(
                        "SUT rejected valid CMN {}: {:?}",
                        asm, other
                    )));
                }
            };
            let mc = llvm_mc_word(&asm)
                .map_err(|e| TestCaseError::fail(format!(
                    "llvm-mc rejected valid CMN {}: {}",
                    asm, e
                )))?;
            prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
        }

        // Oracle: algebraic.metamorphic
        // Target: encoder.compare_branch.encode_cmn
        #[test]
        fn encode_cmn_meta_vs_adds(
            rn in imm_rn_strat(),
            imm in imm_strat(),
            pair in same_width_pair(),
            use_reg in any::<bool>(),
            shift in shift_kind_strat(),
        ) {
            let ops: Vec<Operand> = if use_reg {
                let (a, b) = pair;
                let is32 = is_32_name(&a);
                let mut o = vec![Operand::Reg(a), Operand::Reg(b)];
                if let Some((kind, raw_amt)) = shift {
                    let max_amt = if is32 { 31u32 } else { 63u32 };
                    o.push(Operand::Shift { kind, amount: raw_amt.min(max_amt) });
                }
                o
            } else {
                vec![Operand::Reg(rn), Operand::Imm(imm)]
            };
            let zr = match &ops[0] {
                Operand::Reg(r) => zr_of(r),
                _ => "xzr".to_string(),
            };
            let mut adds_ops = vec![Operand::Reg(zr)];
            adds_ops.extend(ops.iter().cloned());
            let cmn = encode_cmn(&ops);
            let adds = encode_add_sub(&adds_ops, false, true);
            prop_assert_eq!(
                format!("{:?}", cmn),
                format!("{:?}", adds),
                "CMN must equal ADDS ZR, ..."
            );
        }

        // Oracle: algebraic.invariant
        // Target: encoder.compare_branch.encode_cmn
        #[test]
        fn encode_cmn_word_layout_imm(
            rn in imm_rn_strat(),
            imm in 0i64..=IMM12_MAX,
        ) {
            let ops = [Operand::Reg(rn.clone()), Operand::Imm(imm)];
            match encode_cmn(&ops) {
                Ok(EncodeResult::Word(w)) => {
                    let sf = sf_of(&rn);
                    let rn_n = reg_num(&rn);
                    prop_assert_eq!(w & 0x1F, 31u32, "Rd must be XZR/WZR (31)");
                    prop_assert_eq!((w >> 29) & 1, 1u32, "S bit");
                    prop_assert_eq!((w >> 30) & 1, 0u32, "op must be ADD (0)");
                    prop_assert_eq!((w >> 24) & 0x1F, 0b10001u32, "imm form opcode");
                    prop_assert_eq!(w >> 31, sf, "sf from Rn width");
                    prop_assert_eq!((w >> 22) & 1, 0u32, "unshifted sh");
                    prop_assert_eq!((w >> 10) & 0xFFF, imm as u32, "imm12");
                    prop_assert_eq!((w >> 5) & 0x1F, rn_n, "Rn");
                }
                other => {
                    return Err(TestCaseError::fail(format!(
                        "expected Word for cmn {}, #{}, got {:?}",
                        rn, imm, other
                    )));
                }
            }
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_cmn
        #[test]
        fn encode_cmn_neg_arity(arity in 0u32..=1u32, n in 0u32..=30u32) {
            let ops: Vec<Operand> = if arity == 0 {
                vec![]
            } else {
                vec![Operand::Reg(format!("x{}", n))]
            };
            prop_assert!(
                encode_cmn(&ops).is_err(),
                "cmn with {} operand(s) must Err (llvm-mc: too few operands)",
                arity
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_cmn
        #[test]
        fn encode_cmn_neg_imm_oor(rn in imm_rn_strat(), imm in unencodable_imm()) {
            let ops = [Operand::Reg(rn.clone()), Operand::Imm(imm)];
            prop_assert!(
                encode_cmn(&ops).is_err(),
                "cmn {}, #{} is not an encodable imm12 / (imm12<<12) and must Err",
                rn, imm
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_cmn
        #[test]
        fn encode_cmn_neg_extra_operand(
            pair in same_width_pair(),
            which in 0u32..=3u32,
        ) {
            let (rn, rm) = pair;
            let extra = extra_operand(which);
            let ops = [
                Operand::Reg(rn.clone()),
                Operand::Reg(rm.clone()),
                extra,
            ];
            prop_assert!(
                encode_cmn(&ops).is_err(),
                "cmn {}, {}, extra (which={}) must Err (llvm-mc: invalid operand)",
                rn, rm, which
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_cmn
        #[test]
        fn encode_cmn_neg_wrong_reg(kind in 0u32..=8u32, n in 0u32..=31u32, imm in 0i64..=IMM12_MAX) {
            let ops = wrong_reg_ops(kind, n, imm);
            prop_assert!(
                encode_cmn(&ops).is_err(),
                "cmn wrong-reg kind={} n={} must Err (llvm-mc rejects XZR-imm/mixed/FP/SP-Rm/invalid)",
                kind, n
            );
        }

        // Oracle: differential (coverage sweep: extended-register form)
        // Target: encoder.compare_branch.encode_cmn
        #[test]
        fn encode_cmn_diff_extend_llvm_mc(case in extend_case()) {
            let (rn, rm, ext, amt) = case;
            let ops = [
                Operand::Reg(rn.clone()),
                Operand::Reg(rm.clone()),
                Operand::Extend { kind: ext.clone(), amount: amt },
            ];
            let asm = if amt == 0 {
                format!("cmn {}, {}, {}", rn, rm, ext)
            } else {
                format!("cmn {}, {}, {} #{}", rn, rm, ext, amt)
            };
            let sut = match encode_cmn(&ops) {
                Ok(EncodeResult::Word(w)) => w,
                other => {
                    return Err(TestCaseError::fail(format!(
                        "SUT rejected valid CMN {}: {:?}",
                        asm, other
                    )));
                }
            };
            let mc = llvm_mc_word(&asm)
                .map_err(|e| TestCaseError::fail(format!(
                    "llvm-mc rejected valid CMN {}: {}",
                    asm, e
                )))?;
            prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
        }

        // Oracle: differential (coverage sweep: gas negative-imm rewrite)
        // Target: encoder.compare_branch.encode_cmn
        #[test]
        fn encode_cmn_diff_neg_imm_llvm_mc(
            rn in imm_rn_strat(),
            n in neg_imm_strat(),
        ) {
            let ops = [Operand::Reg(rn.clone()), Operand::Imm(-n)];
            let asm = format!("cmn {}, #-{}", rn, n);
            let sut = match encode_cmn(&ops) {
                Ok(EncodeResult::Word(w)) => w,
                other => {
                    return Err(TestCaseError::fail(format!(
                        "SUT rejected gas-valid CMN {}: {:?}",
                        asm, other
                    )));
                }
            };
            let mc = llvm_mc_word(&asm)
                .map_err(|e| TestCaseError::fail(format!(
                    "llvm-mc rejected gas-valid CMN {}: {}",
                    asm, e
                )))?;
            prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
        }

        // Oracle: negative_error (coverage sweep: first operand not Reg)
        // Target: encoder.compare_branch.encode_cmn
        #[test]
        fn encode_cmn_neg_non_reg_first(kind in 0u32..=4u32, second in 0u32..=1u32) {
            let first = non_reg_first(kind);
            let snd = if second == 0 {
                Operand::Reg("x0".into())
            } else {
                Operand::Imm(0)
            };
            let ops = [first, snd];
            prop_assert!(
                encode_cmn(&ops).is_err(),
                "cmn with non-register first operand (kind={}) must Err",
                kind
            );
        }
    }

    #[test]
    fn test_encode_cmn_regression_extra_operand() {
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("x0".into()),
            Operand::Reg("x2".into()),
        ];
        assert!(
            encode_cmn(&ops).is_err(),
            "cmn x0, x0, x2 must Err; llvm-mc rejects a third register operand"
        );
    }

    #[test]
    fn test_encode_cmn_regression_xzr_imm() {
        let ops = [Operand::Reg("xzr".into()), Operand::Imm(0)];
        assert!(
            encode_cmn(&ops).is_err(),
            "cmn xzr, #0 must Err; llvm-mc rejects XZR as CMN-immediate Rn (Rn=31 is SP)"
        );
    }

    #[test]
    fn test_encode_cmn_regression_wzr_imm() {
        let ops = [Operand::Reg("wzr".into()), Operand::Imm(0)];
        assert!(
            encode_cmn(&ops).is_err(),
            "cmn wzr, #0 must Err; llvm-mc rejects WZR as CMN-immediate Rn (Rn=31 is WSP)"
        );
    }

    #[test]
    fn test_encode_cmn_regression_mixed_width() {
        let ops = [Operand::Reg("x0".into()), Operand::Reg("w0".into())];
        assert!(
            encode_cmn(&ops).is_err(),
            "cmn x0, w0 must Err; llvm-mc rejects mixed x/w without an extend"
        );
    }

    #[test]
    fn test_encode_cmn_regression_fp_reg() {
        let ops = [Operand::Reg("d0".into()), Operand::Imm(0)];
        assert!(
            encode_cmn(&ops).is_err(),
            "cmn d0, #0 must Err; llvm-mc rejects FP/SIMD registers"
        );
    }

    #[test]
    fn test_encode_cmn_regression_sp_rm() {
        let ops = [Operand::Reg("x0".into()), Operand::Reg("sp".into())];
        assert!(
            encode_cmn(&ops).is_err(),
            "cmn x0, sp must Err; llvm-mc rejects SP as Rm without an extend"
        );
    }

    #[test]
    fn test_encode_cmn_regression_imm_min_overflow() {
        let ops = [Operand::Reg("x0".into()), Operand::Imm(i64::MIN)];
        assert!(
            encode_cmn(&ops).is_err(),
            "cmn x0, #i64::MIN must Err, not panic on negate overflow"
        );
    }
}

#[cfg(test)]
mod encode_cmp_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler (CMP Rn, #imm / Rn, Rm{, shift});
    //   algebraic.metamorphic (CMP vs SUBS XZR/WZR); algebraic.invariant (SUBS-imm
    //   field layout with Rd=31); negative_error (arity / imm range / extra / wrong-reg).
    // Evidence: src/backend/arm/assembler/README.md:5-14 gas-compat;
    //   README.md:218 Compare lists cmp; README.md:507 compare_branch.rs CMP/CMN/TST;
    //   encoder/mod.rs:1-7 32-bit AArch64 words; encoder/mod.rs:301 cmp dispatch;
    //   compare_branch.rs:7 CMP Rn, op -> SUBS XZR, Rn, op; codegen/emit.rs:492-571
    //   cmp wN/xN, #imm12 and cmp wN/xN, wM/xM; ARM ARM Compare alias of SUBS (Rd=XZR/WZR).
    // Stronger considered:
    //   - State machine: rejected — encode_cmp is a pure function with no lifecycle
    //   - Algebraic round-trip via in-tree decoder: rejected — no CMP decoder
    //   - encode_cmn as differential sibling: rejected — different job (CMN/ADDS-XZR)
    //   - encode_add_sub as differential sibling: rejected — different job (3-operand
    //     SUBS mnemonic/arity); used only as metamorphic alias transform
    // Weaker available: algebraic.invariant (Rd=31/S=1/op=1/imm12 fields),
    //   algebraic.metamorphic (vs encode_add_sub), negative_error
    //   (arity/imm-oor/extra/XZR-imm/mixed/FP/SP-as-Rm)
    // Differential: candidate=encode_cmp, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=[Reg(rn), Imm(imm)] <-> `cmp rn, #imm`;
    //   [Reg(rn), Reg(rm){, Shift}] <-> `cmp rn, rm{, shift}`

    use super::encode_cmp;
    use super::super::{encode_add_sub, EncodeResult};
    use crate::backend::arm::assembler::parser::Operand;
    use proptest::prelude::*;
    use std::io::Write;
    use std::process::{Command, Stdio};

    const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
    const CASES: u32 = 1000;
    const IMM12_MAX: i64 = 4095;
    const IMM12_SHIFTED_MAX: i64 = 4095 << 12; // 16773120

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

    /// Immediate-form Rn: X/W GPR plus SP/WSP/LR. XZR/WZR are invalid for CMP #imm.
    fn imm_rn_name(n: u32) -> String {
        match n {
            31 => "sp".to_string(),
            32 => "lr".to_string(),
            33 => "wsp".to_string(),
            n if n >= 34 => format!("w{}", (n - 34).min(30)),
            n => format!("x{}", n.min(30)),
        }
    }

    fn imm_rn_strat() -> impl Strategy<Value = String> {
        prop_oneof![
            Just("x0".to_string()),
            Just("x30".to_string()),
            Just("sp".to_string()),
            Just("lr".to_string()),
            Just("w0".to_string()),
            Just("w30".to_string()),
            Just("wsp".to_string()),
            Just("X0".to_string()),
            (0u32..=63).prop_map(imm_rn_name),
        ]
    }

    /// Unshifted imm12 and auto-shifted (N<<12). Bounds 0/1/4095/4096/16773120 forced.
    fn imm_strat() -> impl Strategy<Value = i64> {
        prop_oneof![
            Just(0i64),
            Just(1i64),
            Just(IMM12_MAX),
            Just(4096i64),
            Just(8192i64),
            Just(IMM12_SHIFTED_MAX),
            0i64..=IMM12_MAX,
            (1i64..=IMM12_MAX).prop_map(|n| n << 12),
        ]
    }

    fn explicit_imm12_strat() -> impl Strategy<Value = i64> {
        prop_oneof![
            Just(0i64),
            Just(1i64),
            Just(IMM12_MAX),
            0i64..=IMM12_MAX,
        ]
    }

    fn x_name(n: u32) -> String {
        match n {
            31 => "xzr".to_string(),
            32 => "lr".to_string(),
            n => format!("x{}", n.min(30)),
        }
    }

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

    fn is_32_name(rn: &str) -> bool {
        let l = rn.to_ascii_lowercase();
        l.starts_with('w') || l == "wsp" || l == "wzr"
    }

    fn zr_of(rn: &str) -> String {
        if is_32_name(rn) {
            "wzr".to_string()
        } else {
            "xzr".to_string()
        }
    }

    fn shift_kind_strat() -> impl Strategy<Value = Option<(String, u32)>> {
        prop_oneof![
            Just(None),
            Just(Some(("lsl".to_string(), 0u32))),
            Just(Some(("lsl".to_string(), 3u32))),
            Just(Some(("lsr".to_string(), 0u32))),
            Just(Some(("asr".to_string(), 4u32))),
            Just(Some(("lsl".to_string(), 31u32))),
            Just(Some(("lsr".to_string(), 63u32))),
            Just(Some(("asr".to_string(), 63u32))),
            (0u32..=2, 0u32..=63).prop_map(|(k, a)| {
                let kind = match k {
                    0 => "lsl",
                    1 => "lsr",
                    _ => "asr",
                };
                Some((kind.to_string(), a))
            }),
        ]
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

    fn unencodable_imm() -> impl Strategy<Value = i64> {
        prop_oneof![
            Just(4097i64),
            Just(4098i64),
            Just(8191i64),
            Just(IMM12_SHIFTED_MAX + 1),
            Just(IMM12_SHIFTED_MAX + 4096),
            Just(-4097i64),
            Just(-8191i64),
            Just(i64::MAX),
            Just(i64::MIN),
            (1i64..=4095).prop_map(|n| 4096 + n), // 4097..=8191, none are N<<12
            (1i64..=1024).prop_map(|k| IMM12_SHIFTED_MAX + k),
            (1i64..=1024).prop_map(|k| -(4096 + k)),
        ]
    }

    fn reg_num(name: &str) -> u32 {
        let n = name.to_ascii_lowercase();
        match n.as_str() {
            "sp" | "wsp" | "xzr" | "wzr" => 31,
            "lr" => 30,
            s => s[1..].parse().unwrap_or(0),
        }
    }

    fn sf_of(name: &str) -> u32 {
        if is_32_name(name) {
            0
        } else {
            1
        }
    }

    fn invalid_name(which: u32) -> String {
        match which {
            0 => "x32".to_string(),
            1 => "w32".to_string(),
            2 => "foo".to_string(),
            3 => "".to_string(),
            4 => "r0".to_string(),
            5 => "x".to_string(),
            6 => "x-1".to_string(),
            _ => "x99".to_string(),
        }
    }

    fn wrong_reg_ops(kind: u32, n: u32, imm: i64) -> Vec<Operand> {
        let n = n.min(31);
        match kind {
            0 => vec![Operand::Reg("xzr".into()), Operand::Imm(imm.abs() % 4096)],
            1 => vec![Operand::Reg("wzr".into()), Operand::Imm(imm.abs() % 4096)],
            2 => vec![
                Operand::Reg(format!("x{}", n.min(30))),
                Operand::Reg(format!("w{}", n.min(30))),
            ],
            3 => vec![
                Operand::Reg(format!("w{}", n.min(30))),
                Operand::Reg(format!("x{}", n.min(30))),
            ],
            4 => vec![Operand::Reg(format!("d{}", n)), Operand::Imm(0)],
            5 => vec![
                Operand::Reg(format!("x{}", n.min(30))),
                Operand::Reg(format!("d{}", n)),
            ],
            6 => vec![
                Operand::Reg(format!("x{}", n.min(30))),
                Operand::Reg("sp".into()),
            ],
            7 => vec![Operand::Reg(invalid_name(n % 8)), Operand::Imm(0)],
            _ => vec![
                Operand::Reg(format!("s{}", n)),
                Operand::Reg("w0".into()),
            ],
        }
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_cmp_kat_llvm_mc_x0_imm42() {
        let want = 0xf100a81fu32;
        let mc = llvm_mc_word("cmp x0, #42").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [Operand::Reg("x0".into()), Operand::Imm(42)];
        match encode_cmp(&ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for cmp x0, #42, got {:?}",
                want, other
            ),
        }
    }

    #[test]
    fn encode_cmp_kat_llvm_mc_w0_imm42() {
        let want = 0x7100a81fu32;
        let mc = llvm_mc_word("cmp w0, #42").expect("llvm-mc KAT w");
        assert_eq!(mc, want, "llvm-mc KAT w mapping broken");
        let ops = [Operand::Reg("w0".into()), Operand::Imm(42)];
        match encode_cmp(&ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for cmp w0, #42, got {:?}",
                want, other
            ),
        }
    }

    #[test]
    fn encode_cmp_kat_llvm_mc_x0_x1() {
        let want = 0xeb01001fu32;
        let mc = llvm_mc_word("cmp x0, x1").expect("llvm-mc KAT reg");
        assert_eq!(mc, want, "llvm-mc KAT reg mapping broken");
        let ops = [Operand::Reg("x0".into()), Operand::Reg("x1".into())];
        match encode_cmp(&ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for cmp x0, x1, got {:?}",
                want, other
            ),
        }
    }

    #[test]
    fn encode_cmp_kat_llvm_mc_sp_imm0() {
        let want = 0xf10003ffu32;
        let mc = llvm_mc_word("cmp sp, #0").expect("llvm-mc KAT sp");
        assert_eq!(mc, want, "llvm-mc KAT sp mapping broken");
        let ops = [Operand::Reg("sp".into()), Operand::Imm(0)];
        match encode_cmp(&ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for cmp sp, #0, got {:?}",
                want, other
            ),
        }
    }

    #[test]
    fn encode_cmp_kat_llvm_mc_extend_sxtw() {
        let want = 0xeb21c01fu32;
        let mc = llvm_mc_word("cmp x0, w1, sxtw").expect("llvm-mc KAT extend");
        assert_eq!(mc, want, "llvm-mc KAT extend mapping broken");
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("w1".into()),
            Operand::Extend {
                kind: "sxtw".into(),
                amount: 0,
            },
        ];
        match encode_cmp(&ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for cmp x0, w1, sxtw, got {:?}",
                want, other
            ),
        }
    }

    #[test]
    fn encode_cmp_kat_llvm_mc_neg_imm() {
        let want = 0xb100041fu32; // cmn x0, #1
        let mc = llvm_mc_word("cmp x0, #-1").expect("llvm-mc KAT neg");
        assert_eq!(mc, want, "llvm-mc KAT neg mapping broken");
        let ops = [Operand::Reg("x0".into()), Operand::Imm(-1)];
        match encode_cmp(&ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for cmp x0, #-1, got {:?}",
                want, other
            ),
        }
    }

    #[test]
    fn encode_cmp_kat_subs_alias() {
        let mc = llvm_mc_word("subs xzr, x0, #42").expect("llvm-mc SUBS alias");
        let ops = [Operand::Reg("x0".into()), Operand::Imm(42)];
        match encode_cmp(&ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, mc),
            other => panic!("CMP must match SUBS XZR alias, got {:?}", other),
        }
    }

    proptest! {
        #![proptest_config(cfg())]

        // Oracle: differential
        // Target: encoder.compare_branch.encode_cmp
        #[test]
        fn encode_cmp_diff_imm_llvm_mc(
            rn in imm_rn_strat(),
            imm in imm_strat(),
            mode in 0u32..=2u32,
            expl in explicit_imm12_strat(),
        ) {
            let (ops, asm) = if mode == 2 {
                let ops = vec![
                    Operand::Reg(rn.clone()),
                    Operand::Imm(expl),
                    Operand::Shift { kind: "lsl".into(), amount: 12 },
                ];
                let asm = format!("cmp {}, #{}, lsl #12", rn, expl);
                (ops, asm)
            } else {
                let ops = vec![Operand::Reg(rn.clone()), Operand::Imm(imm)];
                let asm = format!("cmp {}, #{}", rn, imm);
                (ops, asm)
            };
            let sut = match encode_cmp(&ops) {
                Ok(EncodeResult::Word(w)) => w,
                other => {
                    return Err(TestCaseError::fail(format!(
                        "SUT rejected valid CMP {}: {:?}",
                        asm, other
                    )));
                }
            };
            let mc = llvm_mc_word(&asm)
                .map_err(|e| TestCaseError::fail(format!(
                    "llvm-mc rejected valid CMP {}: {}",
                    asm, e
                )))?;
            prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
        }

        // Oracle: differential
        // Target: encoder.compare_branch.encode_cmp
        #[test]
        fn encode_cmp_diff_reg_llvm_mc(
            pair in same_width_pair(),
            shift in shift_kind_strat(),
        ) {
            let (rn, rm) = pair;
            let is32 = is_32_name(&rn);
            let mut ops = vec![Operand::Reg(rn.clone()), Operand::Reg(rm.clone())];
            let mut asm = format!("cmp {}, {}", rn, rm);
            if let Some((kind, raw_amt)) = shift {
                let max_amt = if is32 { 31u32 } else { 63u32 };
                let amt = raw_amt.min(max_amt);
                ops.push(Operand::Shift { kind: kind.clone(), amount: amt });
                asm = format!("cmp {}, {}, {} #{}", rn, rm, kind, amt);
            }
            let sut = match encode_cmp(&ops) {
                Ok(EncodeResult::Word(w)) => w,
                other => {
                    return Err(TestCaseError::fail(format!(
                        "SUT rejected valid CMP {}: {:?}",
                        asm, other
                    )));
                }
            };
            let mc = llvm_mc_word(&asm)
                .map_err(|e| TestCaseError::fail(format!(
                    "llvm-mc rejected valid CMP {}: {}",
                    asm, e
                )))?;
            prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
        }

        // Oracle: algebraic.metamorphic
        // Target: encoder.compare_branch.encode_cmp
        #[test]
        fn encode_cmp_meta_vs_subs(
            rn in imm_rn_strat(),
            imm in imm_strat(),
            pair in same_width_pair(),
            use_reg in any::<bool>(),
            shift in shift_kind_strat(),
        ) {
            let ops: Vec<Operand> = if use_reg {
                let (a, b) = pair;
                let is32 = is_32_name(&a);
                let mut o = vec![Operand::Reg(a), Operand::Reg(b)];
                if let Some((kind, raw_amt)) = shift {
                    let max_amt = if is32 { 31u32 } else { 63u32 };
                    o.push(Operand::Shift { kind, amount: raw_amt.min(max_amt) });
                }
                o
            } else {
                vec![Operand::Reg(rn), Operand::Imm(imm)]
            };
            let zr = match &ops[0] {
                Operand::Reg(r) => zr_of(r),
                _ => "xzr".to_string(),
            };
            let mut subs_ops = vec![Operand::Reg(zr)];
            subs_ops.extend(ops.iter().cloned());
            let cmp = encode_cmp(&ops);
            let subs = encode_add_sub(&subs_ops, true, true);
            prop_assert_eq!(
                format!("{:?}", cmp),
                format!("{:?}", subs),
                "CMP must equal SUBS ZR, ..."
            );
        }

        // Oracle: algebraic.invariant
        // Target: encoder.compare_branch.encode_cmp
        #[test]
        fn encode_cmp_word_layout_imm(
            rn in imm_rn_strat(),
            imm in 0i64..=IMM12_MAX,
        ) {
            let ops = [Operand::Reg(rn.clone()), Operand::Imm(imm)];
            match encode_cmp(&ops) {
                Ok(EncodeResult::Word(w)) => {
                    let sf = sf_of(&rn);
                    let rn_n = reg_num(&rn);
                    prop_assert_eq!(w & 0x1F, 31u32, "Rd must be XZR/WZR (31)");
                    prop_assert_eq!((w >> 29) & 1, 1u32, "S bit");
                    prop_assert_eq!((w >> 30) & 1, 1u32, "op must be SUB (1)");
                    prop_assert_eq!((w >> 24) & 0x1F, 0b10001u32, "imm form opcode");
                    prop_assert_eq!(w >> 31, sf, "sf from Rn width");
                    prop_assert_eq!((w >> 22) & 1, 0u32, "unshifted sh");
                    prop_assert_eq!((w >> 10) & 0xFFF, imm as u32, "imm12");
                    prop_assert_eq!((w >> 5) & 0x1F, rn_n, "Rn");
                }
                other => {
                    return Err(TestCaseError::fail(format!(
                        "expected Word for cmp {}, #{}, got {:?}",
                        rn, imm, other
                    )));
                }
            }
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_cmp
        #[test]
        fn encode_cmp_neg_arity(arity in 0u32..=1u32, n in 0u32..=30u32) {
            let ops: Vec<Operand> = if arity == 0 {
                vec![]
            } else {
                vec![Operand::Reg(format!("x{}", n))]
            };
            prop_assert!(
                encode_cmp(&ops).is_err(),
                "cmp with {} operand(s) must Err (llvm-mc: too few operands)",
                arity
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_cmp
        #[test]
        fn encode_cmp_neg_imm_oor(rn in imm_rn_strat(), imm in unencodable_imm()) {
            let ops = [Operand::Reg(rn.clone()), Operand::Imm(imm)];
            prop_assert!(
                encode_cmp(&ops).is_err(),
                "cmp {}, #{} is not an encodable imm12 / (imm12<<12) and must Err",
                rn, imm
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_cmp
        #[test]
        fn encode_cmp_neg_extra_operand(
            pair in same_width_pair(),
            which in 0u32..=3u32,
        ) {
            let (rn, rm) = pair;
            let extra = extra_operand(which);
            let ops = [
                Operand::Reg(rn.clone()),
                Operand::Reg(rm.clone()),
                extra,
            ];
            prop_assert!(
                encode_cmp(&ops).is_err(),
                "cmp {}, {}, extra (which={}) must Err (llvm-mc: invalid operand)",
                rn, rm, which
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_cmp
        #[test]
        fn encode_cmp_neg_wrong_reg(kind in 0u32..=8u32, n in 0u32..=31u32, imm in 0i64..=IMM12_MAX) {
            let ops = wrong_reg_ops(kind, n, imm);
            prop_assert!(
                encode_cmp(&ops).is_err(),
                "cmp wrong-reg kind={} n={} must Err (llvm-mc rejects XZR-imm/mixed/FP/SP-Rm/invalid)",
                kind, n
            );
        }

        // Oracle: differential (coverage sweep: extended-register form)
        // Target: encoder.compare_branch.encode_cmp
        #[test]
        fn encode_cmp_diff_extend_llvm_mc(case in extend_case()) {
            let (rn, rm, ext, amt) = case;
            let ops = [
                Operand::Reg(rn.clone()),
                Operand::Reg(rm.clone()),
                Operand::Extend { kind: ext.clone(), amount: amt },
            ];
            let asm = if amt == 0 {
                format!("cmp {}, {}, {}", rn, rm, ext)
            } else {
                format!("cmp {}, {}, {} #{}", rn, rm, ext, amt)
            };
            let sut = match encode_cmp(&ops) {
                Ok(EncodeResult::Word(w)) => w,
                other => {
                    return Err(TestCaseError::fail(format!(
                        "SUT rejected valid CMP {}: {:?}",
                        asm, other
                    )));
                }
            };
            let mc = llvm_mc_word(&asm)
                .map_err(|e| TestCaseError::fail(format!(
                    "llvm-mc rejected valid CMP {}: {}",
                    asm, e
                )))?;
            prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
        }

        // Oracle: differential (coverage sweep: gas negative-imm rewrite)
        // Target: encoder.compare_branch.encode_cmp
        #[test]
        fn encode_cmp_diff_neg_imm_llvm_mc(
            rn in imm_rn_strat(),
            n in neg_imm_strat(),
        ) {
            let ops = [Operand::Reg(rn.clone()), Operand::Imm(-n)];
            let asm = format!("cmp {}, #-{}", rn, n);
            let sut = match encode_cmp(&ops) {
                Ok(EncodeResult::Word(w)) => w,
                other => {
                    return Err(TestCaseError::fail(format!(
                        "SUT rejected gas-valid CMP {}: {:?}",
                        asm, other
                    )));
                }
            };
            let mc = llvm_mc_word(&asm)
                .map_err(|e| TestCaseError::fail(format!(
                    "llvm-mc rejected gas-valid CMP {}: {}",
                    asm, e
                )))?;
            prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
        }

        // Oracle: negative_error (coverage sweep: first operand not Reg)
        // Target: encoder.compare_branch.encode_cmp
        #[test]
        fn encode_cmp_neg_non_reg_first(kind in 0u32..=4u32, second in 0u32..=1u32) {
            let first = non_reg_first(kind);
            let snd = if second == 0 {
                Operand::Reg("x0".into())
            } else {
                Operand::Imm(0)
            };
            let ops = [first, snd];
            prop_assert!(
                encode_cmp(&ops).is_err(),
                "cmp with non-register first operand (kind={}) must Err",
                kind
            );
        }
    }

    fn extend_case() -> impl Strategy<Value = (String, String, String, u32)> {
        let amt = prop_oneof![Just(0u32), Just(4u32), 0u32..=4u32];
        prop_oneof![
            (
                prop_oneof![
                    Just("x0".to_string()),
                    Just("x30".to_string()),
                    Just("sp".to_string()),
                    Just("lr".to_string()),
                    (0u32..=30).prop_map(|n| format!("x{}", n)),
                ],
                (0u32..=30).prop_map(|n| format!("w{}", n)),
                prop_oneof![Just("sxtw".to_string()), Just("uxtw".to_string())],
                amt.clone(),
            ),
            (
                prop_oneof![
                    Just("x0".to_string()),
                    Just("sp".to_string()),
                    (0u32..=30).prop_map(|n| format!("x{}", n)),
                ],
                x_name_strat(),
                prop_oneof![Just("sxtx".to_string()), Just("uxtx".to_string())],
                amt.clone(),
            ),
            (
                prop_oneof![
                    Just("w0".to_string()),
                    Just("wsp".to_string()),
                    (0u32..=30).prop_map(|n| format!("w{}", n)),
                ],
                w_name_strat(),
                Just("uxtw".to_string()),
                amt,
            ),
        ]
    }

    fn neg_imm_strat() -> impl Strategy<Value = i64> {
        prop_oneof![
            Just(1i64),
            Just(4095i64),
            1i64..=4095i64,
        ]
    }

    fn non_reg_first(kind: u32) -> Operand {
        match kind {
            0 => Operand::Imm(0),
            1 => Operand::Symbol("foo".into()),
            2 => Operand::Mem {
                base: "x0".into(),
                offset: 0,
            },
            3 => Operand::Cond("eq".into()),
            _ => Operand::Shift {
                kind: "lsl".into(),
                amount: 0,
            },
        }
    }

    #[test]
    fn test_encode_cmp_regression_extra_operand() {
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("x0".into()),
            Operand::Reg("x2".into()),
        ];
        assert!(
            encode_cmp(&ops).is_err(),
            "cmp x0, x0, x2 must Err; llvm-mc rejects a third register operand"
        );
    }

    #[test]
    fn test_encode_cmp_regression_xzr_imm() {
        let ops = [Operand::Reg("xzr".into()), Operand::Imm(0)];
        assert!(
            encode_cmp(&ops).is_err(),
            "cmp xzr, #0 must Err; llvm-mc rejects XZR as CMP-immediate Rn (Rn=31 is SP)"
        );
    }

    #[test]
    fn test_encode_cmp_regression_wzr_imm() {
        let ops = [Operand::Reg("wzr".into()), Operand::Imm(0)];
        assert!(
            encode_cmp(&ops).is_err(),
            "cmp wzr, #0 must Err; llvm-mc rejects WZR as CMP-immediate Rn (Rn=31 is WSP)"
        );
    }

    #[test]
    fn test_encode_cmp_regression_mixed_width() {
        let ops = [Operand::Reg("x0".into()), Operand::Reg("w0".into())];
        assert!(
            encode_cmp(&ops).is_err(),
            "cmp x0, w0 must Err; llvm-mc rejects mixed x/w without an extend"
        );
    }

    #[test]
    fn test_encode_cmp_regression_fp_reg() {
        let ops = [Operand::Reg("d0".into()), Operand::Imm(0)];
        assert!(
            encode_cmp(&ops).is_err(),
            "cmp d0, #0 must Err; llvm-mc rejects FP/SIMD registers"
        );
    }

    #[test]
    fn test_encode_cmp_regression_sp_rm() {
        let ops = [Operand::Reg("x0".into()), Operand::Reg("sp".into())];
        assert!(
            encode_cmp(&ops).is_err(),
            "cmp x0, sp must Err; llvm-mc rejects SP as Rm without an extend"
        );
    }

    #[test]
    fn test_encode_cmp_regression_imm_min_overflow() {
        let ops = [Operand::Reg("x0".into()), Operand::Imm(i64::MIN)];
        assert!(
            encode_cmp(&ops).is_err(),
            "cmp x0, #i64::MIN must Err, not panic on negate overflow"
        );
    }
}

#[cfg(test)]
mod encode_cneg_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler (CNEG Rd, Rn, cond);
    //   algebraic.metamorphic (CNEG vs CSNEG invert(cond));
    //   algebraic.invariant (CSNEG field layout); negative_error (arity / extra /
    //   AL-NV / SP-FP-mixed-invalid / bad operand kind).
    // Evidence: src/backend/arm/assembler/README.md:5-14 gas-compat;
    //   encoder/mod.rs:1-7 32-bit AArch64 words; encoder/mod.rs:897 cneg dispatch;
    //   compare_branch.rs:275 CNEG Rd, Rn, cond -> CSNEG Rd, Rn, Rn, invert(cond);
    //   ARM ARM Conditional Negate alias of CSNEG (not valid for AL/NV);
    //   ARM ARM CSNEG: sf 1 0 11010100 Rm cond 0 1 Rn Rd; invert(cond)=cond XOR 1.
    // Stronger considered:
    //   - State machine: rejected — encode_cneg is a pure function with no lifecycle
    //   - Algebraic round-trip via in-tree decoder: rejected — no CNEG decoder
    //   - encode_csneg as differential sibling: rejected — different job (4-operand
    //     CSNEG mnemonic/arity); used only as metamorphic alias transform
    //   - encode_cinc / encode_cinv: rejected — different jobs (CSINC / CSINV)
    // Weaker available: algebraic.invariant (opcode/Rm=Rn/op2/invert fields),
    //   algebraic.metamorphic (vs encode_csneg), negative_error
    //   (arity/extra/AL-NV/SP/FP/mixed/bad-kind)
    // Differential: candidate=encode_cneg, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=[Reg(rd), Reg(rn), Cond(c)] <-> asm text `cneg rd, rn, c`

    use super::{encode_cneg, encode_csneg};
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

    /// n=0..30 -> wN; 31 -> wzr. wsp is not in the valid CNEG domain.
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
    fn encode_cneg_kat_llvm_mc_x0_x1_eq() {
        let want = 0xda811420u32;
        let mc = llvm_mc_word("cneg x0, x1, eq").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("x1".into()),
            Operand::Cond("eq".into()),
        ];
        match encode_cneg(&ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for cneg x0, x1, eq, got {:?}",
                want, other
            ),
        }
    }

    #[test]
    fn encode_cneg_kat_llvm_mc_w0_w1_ne() {
        let want = 0x5a810420u32;
        let mc = llvm_mc_word("cneg w0, w1, ne").expect("llvm-mc KAT w-form");
        assert_eq!(mc, want, "llvm-mc KAT w-form mapping broken");
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Reg("w1".into()),
            Operand::Cond("ne".into()),
        ];
        match encode_cneg(&ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for cneg w0, w1, ne, got {:?}",
                want, other
            ),
        }
    }

    #[test]
    fn encode_cneg_kat_csneg_alias() {
        let want = 0xda811420u32;
        let mc = llvm_mc_word("csneg x0, x1, x1, ne").expect("llvm-mc KAT csneg alias");
        assert_eq!(mc, want, "llvm-mc KAT csneg-alias mapping broken");
        let cneg_ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("x1".into()),
            Operand::Cond("eq".into()),
        ];
        match encode_cneg(&cneg_ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for cneg x0, x1, eq, got {:?}",
                want, other
            ),
        }
        let csneg_ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("x1".into()),
            Operand::Reg("x1".into()),
            Operand::Cond("ne".into()),
        ];
        match encode_csneg(&csneg_ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!("encode_csneg KAT mismatch: {:?}", other),
        }
    }

    proptest! {
        #![proptest_config(cfg())]

        // Oracle: differential
        // Target: encoder.compare_branch.encode_cneg
        #[test]
        fn encode_cneg_diff_llvm_mc(
            (rd, rn) in same_width_pair(),
            cond in cond14_strat(),
        ) {
            let asm = format!("cneg {}, {}, {}", rd, rn, cond);
            let ops = [
                Operand::Reg(rd.clone()),
                Operand::Reg(rn.clone()),
                Operand::Cond(cond.clone()),
            ];
            let sut = match encode_cneg(&ops) {
                Ok(EncodeResult::Word(w)) => w,
                other => {
                    return Err(TestCaseError::fail(format!(
                        "SUT rejected valid CNEG {}: {:?}",
                        asm, other
                    )));
                }
            };
            let mc = llvm_mc_word(&asm)
                .map_err(|e| TestCaseError::fail(format!(
                    "llvm-mc rejected valid CNEG {}: {}",
                    asm, e
                )))?;
            prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
        }

        // Oracle: algebraic.metamorphic
        // Target: encoder.compare_branch.encode_cneg
        #[test]
        fn encode_cneg_meta_vs_csneg(
            (rd, rn) in same_width_pair(),
            cond in cond14_strat(),
        ) {
            let inv = invert_cond_name(&cond).to_string();
            let cneg_ops = [
                Operand::Reg(rd.clone()),
                Operand::Reg(rn.clone()),
                Operand::Cond(cond.clone()),
            ];
            let csneg_ops = [
                Operand::Reg(rd.clone()),
                Operand::Reg(rn.clone()),
                Operand::Reg(rn.clone()),
                Operand::Cond(inv),
            ];
            let left = encode_cneg(&cneg_ops).map_err(|e| TestCaseError::fail(e))?;
            let right = encode_csneg(&csneg_ops).map_err(|e| TestCaseError::fail(e))?;
            match (left, right) {
                (EncodeResult::Word(w_cneg), EncodeResult::Word(w_csneg)) => {
                    prop_assert_eq!(
                        w_cneg, w_csneg,
                        "CNEG {}, {}, {} must equal CSNEG {}, {}, {}, {}",
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

        // Oracle: algebraic.invariant
        // Target: encoder.compare_branch.encode_cneg
        #[test]
        fn encode_cneg_word_layout(
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
            match encode_cneg(&ops) {
                Ok(EncodeResult::Word(w)) => {
                    let sf = if is_64 { 1u32 } else { 0u32 };
                    let inv = cond_enc ^ 1;
                    prop_assert_eq!((w >> 31) & 1, sf, "sf bit 31");
                    prop_assert_eq!((w >> 30) & 1, 1u32, "op bit 30");
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
        // Target: encoder.compare_branch.encode_cneg
        #[test]
        fn encode_cneg_neg_arity(arity in 0u32..=2u32) {
            let ops: Vec<Operand> = match arity {
                0 => vec![],
                1 => vec![Operand::Reg("x0".into())],
                _ => vec![Operand::Reg("x0".into()), Operand::Reg("x1".into())],
            };
            prop_assert!(
                encode_cneg(&ops).is_err(),
                "cneg with {} operand(s) must Err (llvm-mc: too few operands)",
                arity
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_cneg
        #[test]
        fn encode_cneg_neg_extra_operand(
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
                encode_cneg(&ops).is_err(),
                "cneg {}, {}, {}, extra (which={}) must Err (llvm-mc: invalid operand)",
                rd, rn, cond, which
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_cneg
        #[test]
        fn encode_cneg_neg_al_nv(
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
                encode_cneg(&ops).is_err(),
                "cneg {}, {}, {} must Err (llvm-mc: AL and NV invalid for CNEG)",
                rd, rn, cond
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_cneg
        #[test]
        fn encode_cneg_neg_wrong_reg(kind in 0u32..=8u32, n in 0u32..=31u32) {
            let ops = bad_ops(kind, n);
            prop_assert!(
                encode_cneg(&ops).is_err(),
                "cneg with wrong-reg kind={} n={} must Err (llvm-mc rejects SP/FP/mixed/invalid)",
                kind, n
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_cneg
        #[test]
        fn encode_cneg_neg_bad_operand_kind(slot in 0u32..=2u32, which in 0u32..=4u32) {
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
                encode_cneg(&ops).is_err(),
                "cneg with non-Reg/non-Cond at slot {} which={} must Err",
                slot, which
            );
        }

        // Oracle: negative_error (coverage sweep: encode_cond None arm)
        // Target: encoder.compare_branch.encode_cneg
        #[test]
        fn encode_cneg_neg_unknown_cond(
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
                encode_cneg(&ops).is_err(),
                "cneg with unknown cond '{}' must Err",
                cond
            );
        }

        // Oracle: negative_error (coverage sweep: get_reg parse_reg_num None arm)
        // Target: encoder.compare_branch.encode_cneg
        #[test]
        fn encode_cneg_neg_invalid_name(which in 0u32..=7u32) {
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
                encode_cneg(&ops).is_err(),
                "cneg {} , x0, eq must Err (not a valid register name)",
                name
            );
        }
    }

    #[test]
    fn test_encode_cneg_regression_extra_operand() {
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("x0".into()),
            Operand::Cond("eq".into()),
            Operand::Reg("x2".into()),
        ];
        assert!(
            encode_cneg(&ops).is_err(),
            "cneg x0, x0, eq, x2 must Err; llvm-mc rejects a fourth operand"
        );
    }

    #[test]
    fn test_encode_cneg_regression_al() {
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("x0".into()),
            Operand::Cond("al".into()),
        ];
        assert!(
            encode_cneg(&ops).is_err(),
            "cneg x0, x0, al must Err; llvm-mc rejects AL/NV for CNEG"
        );
    }

    #[test]
    fn test_encode_cneg_regression_nv() {
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("x0".into()),
            Operand::Cond("nv".into()),
        ];
        assert!(
            encode_cneg(&ops).is_err(),
            "cneg x0, x0, nv must Err; llvm-mc rejects AL/NV for CNEG"
        );
    }

    #[test]
    fn test_encode_cneg_regression_sp() {
        let ops = [
            Operand::Reg("sp".into()),
            Operand::Reg("x0".into()),
            Operand::Cond("eq".into()),
        ];
        assert!(
            encode_cneg(&ops).is_err(),
            "cneg sp, x0, eq must Err; llvm-mc rejects SP (register 31 is XZR)"
        );
    }

    #[test]
    fn test_encode_cneg_regression_mixed_width() {
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("w0".into()),
            Operand::Cond("eq".into()),
        ];
        assert!(
            encode_cneg(&ops).is_err(),
            "cneg x0, w0, eq must Err; llvm-mc rejects mixed x/w"
        );
    }

    #[test]
    fn test_encode_cneg_regression_fp_reg() {
        let ops = [
            Operand::Reg("d0".into()),
            Operand::Reg("d0".into()),
            Operand::Cond("eq".into()),
        ];
        assert!(
            encode_cneg(&ops).is_err(),
            "cneg d0, d0, eq must Err; llvm-mc rejects FP/SIMD registers"
        );
    }
}

#[cfg(test)]
mod encode_csel_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler (CSEL Rd, Rn, Rm, cond);
    //   algebraic.metamorphic (CSEL vs CSINC XOR bit 10; CSEL vs CSINV XOR bit 30);
    //   algebraic.invariant (CSEL field layout); negative_error (arity / extra /
    //   SP-FP-mixed-invalid / bad kind).
    // Evidence: src/backend/arm/assembler/README.md:5-14 gas-compat;
    //   encoder/mod.rs:1-7 32-bit AArch64 words; encoder/mod.rs:308 csel dispatch;
    //   codegen/comparison.rs:71 csel x0, x2, x1, ne;
    //   ARM ARM Conditional Select (architectural): sf 0 0 11010100 Rm cond 00 Rn Rd;
    //   AL/NV valid (unlike CINC/CINV/CNEG aliases); register 31 is XZR/WZR, Wt/Xt only.
    // Stronger considered:
    //   - State machine: rejected — encode_csel is a pure function with no lifecycle
    //   - Algebraic round-trip via in-tree decoder: rejected — no CSEL decoder
    //   - encode_csinc / encode_csinv as differential siblings: rejected — different
    //     jobs (CSINC op2=01 / CSINV op=1); used only as metamorphic XOR transforms
    // Weaker available: algebraic.invariant (opcode/Rm/cond/op2/Rn/Rd fields),
    //   algebraic.metamorphic (vs encode_csinc / encode_csinv), negative_error
    //   (arity/extra/SP/FP/mixed/bad kind)
    // Differential: candidate=encode_csel, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=[Reg(rd), Reg(rn), Reg(rm), Cond(c)] <-> asm text `csel rd, rn, rm, c`

    use super::{encode_csel, encode_csinc, encode_csinv};
    use super::super::EncodeResult;
    use crate::backend::arm::assembler::parser::Operand;
    use proptest::prelude::*;
    use std::io::Write;
    use std::process::{Command, Stdio};

    const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
    const CASES: u32 = 1000;
    /// Canonical cond names whose encodings are 0..=15 (AL=14 / NV=15 included).
    const COND16: [&str; 16] = [
        "eq", "ne", "cs", "cc", "mi", "pl", "vs", "vc",
        "hi", "ls", "ge", "lt", "gt", "le", "al", "nv",
    ];
    const COND16_WITH_ALIASES: [&str; 18] = [
        "eq", "ne", "cs", "hs", "cc", "lo", "mi", "pl",
        "vs", "vc", "hi", "ls", "ge", "lt", "gt", "le", "al", "nv",
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

    /// n=0..30 -> xN; 31 -> xzr; 32 -> lr.
    fn x_name(n: u32) -> String {
        match n {
            31 => "xzr".to_string(),
            32 => "lr".to_string(),
            n => format!("x{}", n.min(30)),
        }
    }

    /// n=0..30 -> wN; 31 -> wzr. wsp is not in the valid CSEL domain.
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

    fn same_width_triple() -> impl Strategy<Value = (String, String, String)> {
        prop_oneof![
            (x_name_strat(), x_name_strat(), x_name_strat()),
            (w_name_strat(), w_name_strat(), w_name_strat()),
        ]
    }

    fn cond16_strat() -> impl Strategy<Value = String> {
        prop_oneof![
            Just("eq".to_string()),
            Just("al".to_string()),
            Just("nv".to_string()),
            Just("hs".to_string()),
            Just("lo".to_string()),
            Just("le".to_string()),
            prop::sample::select(
                COND16_WITH_ALIASES.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
            ),
        ]
    }

    fn extra_operand(which: u32) -> Operand {
        match which {
            0 => Operand::Reg("x3".into()),
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

    fn bad_ops(kind: u32, n: u32) -> [Operand; 4] {
        let n = n.min(31);
        let eq = Operand::Cond("eq".into());
        let xn = Operand::Reg(format!("x{}", n.min(30)));
        let wn = Operand::Reg(format!("w{}", n.min(30)));
        match kind {
            0 => [Operand::Reg("sp".into()), xn.clone(), xn, eq],
            1 => [xn.clone(), Operand::Reg("sp".into()), xn, eq],
            2 => [xn.clone(), xn, Operand::Reg("sp".into()), eq],
            3 => [Operand::Reg("wsp".into()), wn.clone(), wn, eq],
            4 => [xn.clone(), wn, xn, eq],
            5 => [
                Operand::Reg(format!("d{}", n)),
                Operand::Reg(format!("d{}", n)),
                Operand::Reg(format!("d{}", n)),
                eq,
            ],
            6 => [
                Operand::Reg(format!("s{}", n)),
                Operand::Reg("w0".into()),
                Operand::Reg("w1".into()),
                eq,
            ],
            7 => [
                Operand::Reg(format!("q{}", n)),
                Operand::Reg("x0".into()),
                Operand::Reg("x1".into()),
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
                [
                    Operand::Reg(name),
                    Operand::Reg("x0".into()),
                    Operand::Reg("x1".into()),
                    eq,
                ]
            }
        }
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_csel_kat_llvm_mc_x0_x1_x2_eq() {
        let want = 0x9a820020u32;
        let mc = llvm_mc_word("csel x0, x1, x2, eq").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("x1".into()),
            Operand::Reg("x2".into()),
            Operand::Cond("eq".into()),
        ];
        match encode_csel(&ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for csel x0, x1, x2, eq, got {:?}",
                want, other
            ),
        }
    }

    #[test]
    fn encode_csel_kat_llvm_mc_w0_w1_w2_ne() {
        let want = 0x1a821020u32;
        let mc = llvm_mc_word("csel w0, w1, w2, ne").expect("llvm-mc KAT w-form");
        assert_eq!(mc, want, "llvm-mc KAT w-form mapping broken");
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Reg("w1".into()),
            Operand::Reg("w2".into()),
            Operand::Cond("ne".into()),
        ];
        match encode_csel(&ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for csel w0, w1, w2, ne, got {:?}",
                want, other
            ),
        }
    }

    #[test]
    fn encode_csel_kat_llvm_mc_al() {
        let want = 0x9a82e020u32;
        let mc = llvm_mc_word("csel x0, x1, x2, al").expect("llvm-mc KAT al");
        assert_eq!(mc, want, "llvm-mc KAT al mapping broken");
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("x1".into()),
            Operand::Reg("x2".into()),
            Operand::Cond("al".into()),
        ];
        match encode_csel(&ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for csel x0, x1, x2, al, got {:?}",
                want, other
            ),
        }
    }

    proptest! {
        #![proptest_config(cfg())]

        // Oracle: differential
        // Target: encoder.compare_branch.encode_csel
        #[test]
        fn encode_csel_diff_llvm_mc(
            (rd, rn, rm) in same_width_triple(),
            cond in cond16_strat(),
        ) {
            let asm = format!("csel {}, {}, {}, {}", rd, rn, rm, cond);
            let ops = [
                Operand::Reg(rd.clone()),
                Operand::Reg(rn.clone()),
                Operand::Reg(rm.clone()),
                Operand::Cond(cond.clone()),
            ];
            let sut = match encode_csel(&ops) {
                Ok(EncodeResult::Word(w)) => w,
                other => {
                    return Err(TestCaseError::fail(format!(
                        "SUT rejected valid CSEL {}: {:?}",
                        asm, other
                    )));
                }
            };
            let mc = llvm_mc_word(&asm)
                .map_err(|e| TestCaseError::fail(format!(
                    "llvm-mc rejected valid CSEL {}: {}",
                    asm, e
                )))?;
            prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
        }

        // Oracle: algebraic.metamorphic
        // Target: encoder.compare_branch.encode_csel
        #[test]
        fn encode_csel_meta_vs_csinc(
            (rd, rn, rm) in same_width_triple(),
            cond in cond16_strat(),
        ) {
            let ops = [
                Operand::Reg(rd.clone()),
                Operand::Reg(rn.clone()),
                Operand::Reg(rm.clone()),
                Operand::Cond(cond.clone()),
            ];
            let left = encode_csel(&ops).map_err(|e| TestCaseError::fail(e))?;
            let right = encode_csinc(&ops).map_err(|e| TestCaseError::fail(e))?;
            match (left, right) {
                (EncodeResult::Word(w_csel), EncodeResult::Word(w_csinc)) => {
                    prop_assert_eq!(
                        w_csel ^ w_csinc,
                        1u32 << 10,
                        "CSEL XOR CSINC must be bit 10 (csel={:#010x} csinc={:#010x}) for {}, {}, {}, {}",
                        w_csel, w_csinc, rd, rn, rm, cond
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
        // Target: encoder.compare_branch.encode_csel
        #[test]
        fn encode_csel_meta_vs_csinv(
            (rd, rn, rm) in same_width_triple(),
            cond in cond16_strat(),
        ) {
            let ops = [
                Operand::Reg(rd.clone()),
                Operand::Reg(rn.clone()),
                Operand::Reg(rm.clone()),
                Operand::Cond(cond.clone()),
            ];
            let left = encode_csel(&ops).map_err(|e| TestCaseError::fail(e))?;
            let right = encode_csinv(&ops).map_err(|e| TestCaseError::fail(e))?;
            match (left, right) {
                (EncodeResult::Word(w_csel), EncodeResult::Word(w_csinv)) => {
                    prop_assert_eq!(
                        w_csel ^ w_csinv,
                        1u32 << 30,
                        "CSEL XOR CSINV must be bit 30 (csel={:#010x} csinv={:#010x}) for {}, {}, {}, {}",
                        w_csel, w_csinv, rd, rn, rm, cond
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
        // Target: encoder.compare_branch.encode_csel
        #[test]
        fn encode_csel_word_layout(
            rd_n in 0u32..=31u32,
            rn_n in 0u32..=31u32,
            rm_n in 0u32..=31u32,
            is_64 in any::<bool>(),
            cond_enc in 0u32..=15u32,
        ) {
            let rd = gpr_name(rd_n, is_64);
            let rn = gpr_name(rn_n, is_64);
            let rm = gpr_name(rm_n, is_64);
            let cond = COND16[cond_enc as usize];
            let ops = [
                Operand::Reg(rd),
                Operand::Reg(rn),
                Operand::Reg(rm),
                Operand::Cond(cond.to_string()),
            ];
            match encode_csel(&ops) {
                Ok(EncodeResult::Word(w)) => {
                    let sf = if is_64 { 1u32 } else { 0u32 };
                    prop_assert_eq!((w >> 31) & 1, sf, "sf bit 31");
                    prop_assert_eq!((w >> 30) & 1, 0u32, "op bit 30");
                    prop_assert_eq!((w >> 29) & 1, 0u32, "S bit 29");
                    prop_assert_eq!((w >> 21) & 0xFF, 0b11010100u32, "bits[28:21]");
                    prop_assert_eq!((w >> 16) & 0x1F, rm_n, "Rm [20:16]");
                    prop_assert_eq!((w >> 12) & 0xF, cond_enc, "cond [15:12]");
                    prop_assert_eq!((w >> 10) & 0x3, 0b00u32, "op2 [11:10]");
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
        // Target: encoder.compare_branch.encode_csel
        #[test]
        fn encode_csel_neg_arity(arity in 0u32..=3u32) {
            let ops: Vec<Operand> = match arity {
                0 => vec![],
                1 => vec![Operand::Reg("x0".into())],
                2 => vec![Operand::Reg("x0".into()), Operand::Reg("x1".into())],
                _ => vec![
                    Operand::Reg("x0".into()),
                    Operand::Reg("x1".into()),
                    Operand::Reg("x2".into()),
                ],
            };
            prop_assert!(
                encode_csel(&ops).is_err(),
                "csel with {} operand(s) must Err (llvm-mc: too few operands)",
                arity
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_csel
        #[test]
        fn encode_csel_neg_extra_operand(
            (rd, rn, rm) in same_width_triple(),
            cond in cond16_strat(),
            which in 0u32..=3u32,
        ) {
            let extra = extra_operand(which);
            let ops = [
                Operand::Reg(rd.clone()),
                Operand::Reg(rn.clone()),
                Operand::Reg(rm.clone()),
                Operand::Cond(cond.clone()),
                extra,
            ];
            prop_assert!(
                encode_csel(&ops).is_err(),
                "csel {}, {}, {}, {}, extra (which={}) must Err (llvm-mc: invalid operand)",
                rd, rn, rm, cond, which
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_csel
        #[test]
        fn encode_csel_neg_wrong_reg(kind in 0u32..=8u32, n in 0u32..=31u32) {
            let ops = bad_ops(kind, n);
            prop_assert!(
                encode_csel(&ops).is_err(),
                "csel with wrong-reg kind={} n={} must Err (llvm-mc rejects SP/FP/mixed/invalid)",
                kind, n
            );
        }

        // Oracle: negative_error (coverage sweep: encode_cond None arm)
        // Target: encoder.compare_branch.encode_csel
        #[test]
        fn encode_csel_neg_unknown_cond(
            (rd, rn, rm) in same_width_triple(),
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
                Operand::Reg(rm),
                Operand::Cond(cond.to_string()),
            ];
            prop_assert!(
                encode_csel(&ops).is_err(),
                "csel with unknown cond '{}' must Err",
                cond
            );
        }

        // Oracle: negative_error (coverage sweep: get_reg parse_reg_num None arm)
        // Target: encoder.compare_branch.encode_csel
        #[test]
        fn encode_csel_neg_invalid_name(which in 0u32..=7u32) {
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
                Operand::Reg("x1".into()),
                Operand::Cond("eq".into()),
            ];
            prop_assert!(
                encode_csel(&ops).is_err(),
                "csel {} , x0, x1, eq must Err (not a valid register name)",
                name
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_csel
        #[test]
        fn encode_csel_neg_bad_operand_kind(slot in 0u32..=3u32, which in 0u32..=4u32) {
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
                Operand::Reg("x2".into()),
                Operand::Cond("eq".into()),
            ];
            ops[slot as usize] = bad;
            prop_assert!(
                encode_csel(&ops).is_err(),
                "csel with non-Reg/non-Cond at slot {} which={} must Err",
                slot, which
            );
        }
    }

    #[test]
    fn test_encode_csel_regression_extra_operand() {
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("x1".into()),
            Operand::Reg("x2".into()),
            Operand::Cond("eq".into()),
            Operand::Reg("x3".into()),
        ];
        assert!(
            encode_csel(&ops).is_err(),
            "csel x0, x1, x2, eq, x3 must Err; llvm-mc rejects a fifth operand"
        );
    }

    #[test]
    fn test_encode_csel_regression_sp() {
        let ops = [
            Operand::Reg("sp".into()),
            Operand::Reg("x0".into()),
            Operand::Reg("x1".into()),
            Operand::Cond("eq".into()),
        ];
        assert!(
            encode_csel(&ops).is_err(),
            "csel sp, x0, x1, eq must Err; llvm-mc rejects SP (register 31 is XZR)"
        );
    }

    #[test]
    fn test_encode_csel_regression_mixed_width() {
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("w1".into()),
            Operand::Reg("x2".into()),
            Operand::Cond("eq".into()),
        ];
        assert!(
            encode_csel(&ops).is_err(),
            "csel x0, w1, x2, eq must Err; llvm-mc rejects mixed x/w"
        );
    }

    #[test]
    fn test_encode_csel_regression_fp_reg() {
        let ops = [
            Operand::Reg("d0".into()),
            Operand::Reg("d1".into()),
            Operand::Reg("d2".into()),
            Operand::Cond("eq".into()),
        ];
        assert!(
            encode_csel(&ops).is_err(),
            "csel d0, d1, d2, eq must Err; llvm-mc rejects FP/SIMD registers"
        );
    }
}

#[cfg(test)]
mod encode_cset_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler (CSET Rd, cond);
    //   algebraic.metamorphic (CSET vs CSINC invert(cond) with Rm=Rn=ZR;
    //     CSET vs CINC Rd, ZR, cond);
    //   algebraic.invariant (CSINC field layout with Rm=Rn=31);
    //   negative_error (arity / extra / AL-NV / SP-FP-invalid).
    // Evidence: src/backend/arm/assembler/README.md:5-14 gas-compat;
    //   encoder/mod.rs:1-7 32-bit AArch64 words; encoder/mod.rs:312 cset dispatch;
    //   codegen/comparison.rs:29 and :40 cset x0, <cond>;
    //   compare_branch.rs:142 CSET Rd, cond -> CSINC Rd, XZR, XZR, invert(cond);
    //   ARM ARM Conditional Set alias of CSINC (not valid for AL/NV);
    //   ARM ARM CSINC: sf 0 0 11010100 Rm cond 0 1 Rn Rd; invert(cond)=cond XOR 1;
    //   CSET forces Rm=Rn=31 (XZR/WZR). Wt/Xt only; register 31 is XZR/WZR.
    // Stronger considered:
    //   - State machine: rejected — encode_cset is a pure function with no lifecycle
    //   - Algebraic round-trip via in-tree decoder: rejected — no CSET decoder
    //   - encode_csinc / encode_cinc as differential siblings: rejected — different
    //     jobs (4-operand CSINC / 3-operand CINC); used only as metamorphic alias
    // Weaker available: algebraic.invariant (opcode/Rm=31/op2/invert/Rn=31/Rd),
    //   algebraic.metamorphic (vs encode_csinc / encode_cinc), negative_error
    //   (arity/extra/AL-NV/SP/FP/invalid)
    // Differential: candidate=encode_cset, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=[Reg(rd), Cond(c)] <-> asm text `cset rd, c`

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

    /// n=0..30 -> wN; 31 -> wzr. wsp is not in the valid CSET domain.
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

    fn rd_strat() -> impl Strategy<Value = String> {
        prop_oneof![x_name_strat(), w_name_strat()]
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

    fn wrong_reg_name(kind: u32, n: u32) -> String {
        let n = n.min(31);
        match kind {
            0 => "sp".to_string(),
            1 => "wsp".to_string(),
            2 => format!("d{}", n),
            3 => format!("s{}", n),
            4 => format!("q{}", n),
            5 => format!("v{}", n),
            6 => format!("h{}", n),
            7 => format!("b{}", n),
            _ => match n % 8 {
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
    fn encode_cset_kat_llvm_mc_x0_eq() {
        let want = 0x9a9f17e0u32;
        let mc = llvm_mc_word("cset x0, eq").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [Operand::Reg("x0".into()), Operand::Cond("eq".into())];
        match encode_cset(&ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for cset x0, eq, got {:?}",
                want, other
            ),
        }
    }

    #[test]
    fn encode_cset_kat_llvm_mc_w0_ne() {
        let want = 0x1a9f07e0u32;
        let mc = llvm_mc_word("cset w0, ne").expect("llvm-mc KAT w-form");
        assert_eq!(mc, want, "llvm-mc KAT w-form mapping broken");
        let ops = [Operand::Reg("w0".into()), Operand::Cond("ne".into())];
        match encode_cset(&ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for cset w0, ne, got {:?}",
                want, other
            ),
        }
    }

    #[test]
    fn encode_cset_kat_csinc_alias() {
        let want = 0x9a9f17e0u32;
        let mc = llvm_mc_word("csinc x0, xzr, xzr, ne").expect("llvm-mc KAT csinc alias");
        assert_eq!(mc, want, "llvm-mc KAT csinc-alias mapping broken");
        let cset_ops = [Operand::Reg("x0".into()), Operand::Cond("eq".into())];
        let csinc_ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("xzr".into()),
            Operand::Reg("xzr".into()),
            Operand::Cond("ne".into()),
        ];
        match encode_cset(&cset_ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!("encode_cset KAT mismatch: {:?}", other),
        }
        match encode_csinc(&csinc_ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!("encode_csinc KAT mismatch: {:?}", other),
        }
    }

    proptest! {
        #![proptest_config(cfg())]

        // Oracle: differential
        // Target: encoder.compare_branch.encode_cset
        #[test]
        fn encode_cset_diff_llvm_mc(
            rd in rd_strat(),
            cond in cond14_strat(),
        ) {
            let asm = format!("cset {}, {}", rd, cond);
            let ops = [Operand::Reg(rd.clone()), Operand::Cond(cond.clone())];
            let sut = match encode_cset(&ops) {
                Ok(EncodeResult::Word(w)) => w,
                other => {
                    return Err(TestCaseError::fail(format!(
                        "SUT rejected valid CSET {}: {:?}",
                        asm, other
                    )));
                }
            };
            let mc = llvm_mc_word(&asm)
                .map_err(|e| TestCaseError::fail(format!(
                    "llvm-mc rejected valid CSET {}: {}",
                    asm, e
                )))?;
            prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
        }

        // Oracle: algebraic.metamorphic
        // Target: encoder.compare_branch.encode_cset
        #[test]
        fn encode_cset_meta_vs_csinc(
            rd in rd_strat(),
            cond in cond14_strat(),
        ) {
            let inv = invert_cond_name(&cond).to_string();
            let zr = zr_of(&rd);
            let cset_ops = [Operand::Reg(rd.clone()), Operand::Cond(cond.clone())];
            let csinc_ops = [
                Operand::Reg(rd.clone()),
                Operand::Reg(zr.clone()),
                Operand::Reg(zr.clone()),
                Operand::Cond(inv),
            ];
            let left = encode_cset(&cset_ops).map_err(|e| TestCaseError::fail(e))?;
            let right = encode_csinc(&csinc_ops).map_err(|e| TestCaseError::fail(e))?;
            match (left, right) {
                (EncodeResult::Word(w_cset), EncodeResult::Word(w_csinc)) => {
                    prop_assert_eq!(
                        w_cset, w_csinc,
                        "CSET {}, {} must equal CSINC {}, {}, {}, {}",
                        rd, cond, rd, zr, zr, invert_cond_name(&cond)
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
        // Target: encoder.compare_branch.encode_cset
        #[test]
        fn encode_cset_meta_vs_cinc(
            rd in rd_strat(),
            cond in cond14_strat(),
        ) {
            let zr = zr_of(&rd);
            let cset_ops = [Operand::Reg(rd.clone()), Operand::Cond(cond.clone())];
            let cinc_ops = [
                Operand::Reg(rd.clone()),
                Operand::Reg(zr.clone()),
                Operand::Cond(cond.clone()),
            ];
            let left = encode_cset(&cset_ops).map_err(|e| TestCaseError::fail(e))?;
            let right = encode_cinc(&cinc_ops).map_err(|e| TestCaseError::fail(e))?;
            match (left, right) {
                (EncodeResult::Word(w_cset), EncodeResult::Word(w_cinc)) => {
                    prop_assert_eq!(
                        w_cset, w_cinc,
                        "CSET {}, {} must equal CINC {}, {}, {}",
                        rd, cond, rd, zr, cond
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
        // Target: encoder.compare_branch.encode_cset
        #[test]
        fn encode_cset_word_layout(
            rd_n in 0u32..=31u32,
            is_64 in any::<bool>(),
            cond_enc in 0u32..=13u32,
        ) {
            let rd = gpr_name(rd_n, is_64);
            let cond = COND14[cond_enc as usize];
            let ops = [Operand::Reg(rd), Operand::Cond(cond.to_string())];
            match encode_cset(&ops) {
                Ok(EncodeResult::Word(w)) => {
                    let sf = if is_64 { 1u32 } else { 0u32 };
                    let inv = cond_enc ^ 1;
                    prop_assert_eq!((w >> 31) & 1, sf, "sf bit 31");
                    prop_assert_eq!((w >> 30) & 1, 0u32, "op bit 30");
                    prop_assert_eq!((w >> 29) & 1, 0u32, "S bit 29");
                    prop_assert_eq!((w >> 21) & 0xFF, 0b11010100u32, "bits[28:21]");
                    prop_assert_eq!((w >> 16) & 0x1F, 31u32, "Rm [20:16] == ZR");
                    prop_assert_eq!((w >> 12) & 0xF, inv, "cond [15:12] == invert");
                    prop_assert_eq!((w >> 10) & 0x3, 0b01u32, "op2 [11:10]");
                    prop_assert_eq!((w >> 5) & 0x1F, 31u32, "Rn [9:5] == ZR");
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
        // Target: encoder.compare_branch.encode_cset
        #[test]
        fn encode_cset_neg_arity(arity in 0u32..=1u32) {
            let ops: Vec<Operand> = match arity {
                0 => vec![],
                _ => vec![Operand::Reg("x0".into())],
            };
            prop_assert!(
                encode_cset(&ops).is_err(),
                "cset with {} operand(s) must Err (llvm-mc: too few operands)",
                arity
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_cset
        #[test]
        fn encode_cset_neg_extra_operand(
            rd in rd_strat(),
            cond in cond14_strat(),
            which in 0u32..=3u32,
        ) {
            let extra = extra_operand(which);
            let ops = [
                Operand::Reg(rd.clone()),
                Operand::Cond(cond.clone()),
                extra,
            ];
            prop_assert!(
                encode_cset(&ops).is_err(),
                "cset {}, {}, extra (which={}) must Err (llvm-mc: invalid operand)",
                rd, cond, which
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_cset
        #[test]
        fn encode_cset_neg_al_nv(
            rd in rd_strat(),
            which in 0u32..=1u32,
        ) {
            let cond = if which == 0 { "al" } else { "nv" };
            let ops = [Operand::Reg(rd.clone()), Operand::Cond(cond.to_string())];
            prop_assert!(
                encode_cset(&ops).is_err(),
                "cset {}, {} must Err (llvm-mc: AL and NV invalid for CSET)",
                rd, cond
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_cset
        #[test]
        fn encode_cset_neg_wrong_reg(kind in 0u32..=8u32, n in 0u32..=31u32) {
            let name = wrong_reg_name(kind, n);
            let ops = [Operand::Reg(name.clone()), Operand::Cond("eq".into())];
            prop_assert!(
                encode_cset(&ops).is_err(),
                "cset {}, eq must Err (llvm-mc rejects SP/FP/invalid)",
                name
            );
        }

        // Oracle: negative_error (coverage sweep: encode_cond None arm)
        // Target: encoder.compare_branch.encode_cset
        #[test]
        fn encode_cset_neg_unknown_cond(
            rd in rd_strat(),
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
            let ops = [Operand::Reg(rd), Operand::Cond(cond.to_string())];
            prop_assert!(
                encode_cset(&ops).is_err(),
                "cset with unknown cond '{}' must Err",
                cond
            );
        }

        // Oracle: negative_error (coverage sweep: get_reg parse_reg_num None arm)
        // Target: encoder.compare_branch.encode_cset
        #[test]
        fn encode_cset_neg_invalid_name(which in 0u32..=7u32) {
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
            let ops = [Operand::Reg(name.to_string()), Operand::Cond("eq".into())];
            prop_assert!(
                encode_cset(&ops).is_err(),
                "cset {} , eq must Err (not a valid register name)",
                name
            );
        }

        // Oracle: negative_error (coverage sweep: get_reg non-Reg / cond not Cond)
        // Target: encoder.compare_branch.encode_cset
        #[test]
        fn encode_cset_neg_bad_operand_kind(slot in 0u32..=1u32, which in 0u32..=4u32) {
            let bad = match which {
                0 => Operand::Imm(0),
                1 => Operand::Mem { base: "x0".into(), offset: 0 },
                2 => Operand::Symbol("foo".into()),
                3 => Operand::Shift { kind: "lsl".into(), amount: 0 },
                _ => Operand::Label("foo".into()),
            };
            let mut ops = vec![
                Operand::Reg("x0".into()),
                Operand::Cond("eq".into()),
            ];
            ops[slot as usize] = bad;
            prop_assert!(
                encode_cset(&ops).is_err(),
                "cset with non-Reg/non-Cond at slot {} which={} must Err",
                slot, which
            );
        }
    }

    #[test]
    fn test_encode_cset_regression_extra_operand() {
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Cond("eq".into()),
            Operand::Reg("x2".into()),
        ];
        assert!(
            encode_cset(&ops).is_err(),
            "cset x0, eq, x2 must Err; llvm-mc rejects a third operand"
        );
    }

    #[test]
    fn test_encode_cset_regression_al() {
        let ops = [Operand::Reg("x0".into()), Operand::Cond("al".into())];
        assert!(
            encode_cset(&ops).is_err(),
            "cset x0, al must Err; llvm-mc rejects AL/NV for CSET"
        );
    }

    #[test]
    fn test_encode_cset_regression_nv() {
        let ops = [Operand::Reg("x0".into()), Operand::Cond("nv".into())];
        assert!(
            encode_cset(&ops).is_err(),
            "cset x0, nv must Err; llvm-mc rejects AL/NV for CSET"
        );
    }

    #[test]
    fn test_encode_cset_regression_sp() {
        let ops = [Operand::Reg("sp".into()), Operand::Cond("eq".into())];
        assert!(
            encode_cset(&ops).is_err(),
            "cset sp, eq must Err; llvm-mc rejects SP (register 31 is XZR)"
        );
    }

    #[test]
    fn test_encode_cset_regression_fp_reg() {
        let ops = [Operand::Reg("d0".into()), Operand::Cond("eq".into())];
        assert!(
            encode_cset(&ops).is_err(),
            "cset d0, eq must Err; llvm-mc rejects FP/SIMD registers"
        );
    }
}

#[cfg(test)]
mod encode_csetm_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler (CSETM Rd, cond);
    //   algebraic.metamorphic (CSETM vs CSINV invert(cond) with Rm=Rn=ZR;
    //     CSETM vs CINV Rd, ZR, cond);
    //   algebraic.invariant (CSINV field layout with Rm=Rn=31);
    //   negative_error (arity / extra / AL-NV / SP-FP-invalid).
    // Evidence: src/backend/arm/assembler/README.md:5-14 gas-compat;
    //   encoder/mod.rs:1-7 32-bit AArch64 words; encoder/mod.rs:313 csetm dispatch;
    //   compare_branch.rs:156 CSETM Rd, cond -> CSINV Rd, XZR, XZR, invert(cond);
    //   ARM ARM Conditional Set Mask alias of CSINV (not valid for AL/NV);
    //   ARM ARM CSINV: sf 1 0 11010100 Rm cond 0 0 Rn Rd; invert(cond)=cond XOR 1;
    //   CSETM forces Rm=Rn=31 (XZR/WZR). Wt/Xt only; register 31 is XZR/WZR.
    // Stronger considered:
    //   - State machine: rejected — encode_csetm is a pure function with no lifecycle
    //   - Algebraic round-trip via in-tree decoder: rejected — no CSETM decoder
    //   - encode_csinv / encode_cinv as differential siblings: rejected — different
    //     jobs (4-operand CSINV / 3-operand CINV); used only as metamorphic alias
    // Weaker available: algebraic.invariant (opcode/Rm=31/op2/invert/Rn=31/Rd),
    //   algebraic.metamorphic (vs encode_csinv / encode_cinv), negative_error
    //   (arity/extra/AL-NV/SP/FP/invalid)
    // Differential: candidate=encode_csetm, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=[Reg(rd), Cond(c)] <-> asm text `csetm rd, c`

    use super::{encode_cinv, encode_csetm, encode_csinv};
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

    /// n=0..30 -> wN; 31 -> wzr. wsp is not in the valid CSETM domain.
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

    fn rd_strat() -> impl Strategy<Value = String> {
        prop_oneof![x_name_strat(), w_name_strat()]
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

    fn wrong_reg_name(kind: u32, n: u32) -> String {
        let n = n.min(31);
        match kind {
            0 => "sp".to_string(),
            1 => "wsp".to_string(),
            2 => format!("d{}", n),
            3 => format!("s{}", n),
            4 => format!("q{}", n),
            5 => format!("v{}", n),
            6 => format!("h{}", n),
            7 => format!("b{}", n),
            _ => match n % 8 {
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
    fn encode_csetm_kat_llvm_mc_x0_eq() {
        let want = 0xda9f13e0u32;
        let mc = llvm_mc_word("csetm x0, eq").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [Operand::Reg("x0".into()), Operand::Cond("eq".into())];
        match encode_csetm(&ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for csetm x0, eq, got {:?}",
                want, other
            ),
        }
    }

    #[test]
    fn encode_csetm_kat_llvm_mc_w0_ne() {
        let want = 0x5a9f03e0u32;
        let mc = llvm_mc_word("csetm w0, ne").expect("llvm-mc KAT w-form");
        assert_eq!(mc, want, "llvm-mc KAT w-form mapping broken");
        let ops = [Operand::Reg("w0".into()), Operand::Cond("ne".into())];
        match encode_csetm(&ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for csetm w0, ne, got {:?}",
                want, other
            ),
        }
    }

    #[test]
    fn encode_csetm_kat_csinv_alias() {
        let want = 0xda9f13e0u32;
        let mc = llvm_mc_word("csinv x0, xzr, xzr, ne").expect("llvm-mc KAT csinv alias");
        assert_eq!(mc, want, "llvm-mc KAT csinv-alias mapping broken");
        let csetm_ops = [Operand::Reg("x0".into()), Operand::Cond("eq".into())];
        let csinv_ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("xzr".into()),
            Operand::Reg("xzr".into()),
            Operand::Cond("ne".into()),
        ];
        match encode_csetm(&csetm_ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!("encode_csetm KAT mismatch: {:?}", other),
        }
        match encode_csinv(&csinv_ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!("encode_csinv KAT mismatch: {:?}", other),
        }
    }

    proptest! {
        #![proptest_config(cfg())]

        // Oracle: differential
        // Target: encoder.compare_branch.encode_csetm
        #[test]
        fn encode_csetm_diff_llvm_mc(
            rd in rd_strat(),
            cond in cond14_strat(),
        ) {
            let asm = format!("csetm {}, {}", rd, cond);
            let ops = [Operand::Reg(rd.clone()), Operand::Cond(cond.clone())];
            let sut = match encode_csetm(&ops) {
                Ok(EncodeResult::Word(w)) => w,
                other => {
                    return Err(TestCaseError::fail(format!(
                        "SUT rejected valid CSETM {}: {:?}",
                        asm, other
                    )));
                }
            };
            let mc = llvm_mc_word(&asm)
                .map_err(|e| TestCaseError::fail(format!(
                    "llvm-mc rejected valid CSETM {}: {}",
                    asm, e
                )))?;
            prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
        }

        // Oracle: algebraic.metamorphic
        // Target: encoder.compare_branch.encode_csetm
        #[test]
        fn encode_csetm_meta_vs_csinv(
            rd in rd_strat(),
            cond in cond14_strat(),
        ) {
            let inv = invert_cond_name(&cond).to_string();
            let zr = zr_of(&rd);
            let csetm_ops = [Operand::Reg(rd.clone()), Operand::Cond(cond.clone())];
            let csinv_ops = [
                Operand::Reg(rd.clone()),
                Operand::Reg(zr.clone()),
                Operand::Reg(zr.clone()),
                Operand::Cond(inv),
            ];
            let left = encode_csetm(&csetm_ops).map_err(|e| TestCaseError::fail(e))?;
            let right = encode_csinv(&csinv_ops).map_err(|e| TestCaseError::fail(e))?;
            match (left, right) {
                (EncodeResult::Word(w_csetm), EncodeResult::Word(w_csinv)) => {
                    prop_assert_eq!(
                        w_csetm, w_csinv,
                        "CSETM {}, {} must equal CSINV {}, {}, {}, {}",
                        rd, cond, rd, zr, zr, invert_cond_name(&cond)
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
        // Target: encoder.compare_branch.encode_csetm
        #[test]
        fn encode_csetm_meta_vs_cinv(
            rd in rd_strat(),
            cond in cond14_strat(),
        ) {
            let zr = zr_of(&rd);
            let csetm_ops = [Operand::Reg(rd.clone()), Operand::Cond(cond.clone())];
            let cinv_ops = [
                Operand::Reg(rd.clone()),
                Operand::Reg(zr.clone()),
                Operand::Cond(cond.clone()),
            ];
            let left = encode_csetm(&csetm_ops).map_err(|e| TestCaseError::fail(e))?;
            let right = encode_cinv(&cinv_ops).map_err(|e| TestCaseError::fail(e))?;
            match (left, right) {
                (EncodeResult::Word(w_csetm), EncodeResult::Word(w_cinv)) => {
                    prop_assert_eq!(
                        w_csetm, w_cinv,
                        "CSETM {}, {} must equal CINV {}, {}, {}",
                        rd, cond, rd, zr, cond
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
        // Target: encoder.compare_branch.encode_csetm
        #[test]
        fn encode_csetm_word_layout(
            rd_n in 0u32..=31u32,
            is_64 in any::<bool>(),
            cond_enc in 0u32..=13u32,
        ) {
            let rd = gpr_name(rd_n, is_64);
            let cond = COND14[cond_enc as usize];
            let ops = [Operand::Reg(rd), Operand::Cond(cond.to_string())];
            match encode_csetm(&ops) {
                Ok(EncodeResult::Word(w)) => {
                    let sf = if is_64 { 1u32 } else { 0u32 };
                    let inv = cond_enc ^ 1;
                    prop_assert_eq!((w >> 31) & 1, sf, "sf bit 31");
                    prop_assert_eq!((w >> 30) & 1, 1u32, "op bit 30");
                    prop_assert_eq!((w >> 29) & 1, 0u32, "S bit 29");
                    prop_assert_eq!((w >> 21) & 0xFF, 0b11010100u32, "bits[28:21]");
                    prop_assert_eq!((w >> 16) & 0x1F, 31u32, "Rm [20:16] == ZR");
                    prop_assert_eq!((w >> 12) & 0xF, inv, "cond [15:12] == invert");
                    prop_assert_eq!((w >> 10) & 0x3, 0b00u32, "op2 [11:10]");
                    prop_assert_eq!((w >> 5) & 0x1F, 31u32, "Rn [9:5] == ZR");
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
        // Target: encoder.compare_branch.encode_csetm
        #[test]
        fn encode_csetm_neg_arity(arity in 0u32..=1u32) {
            let ops: Vec<Operand> = match arity {
                0 => vec![],
                _ => vec![Operand::Reg("x0".into())],
            };
            prop_assert!(
                encode_csetm(&ops).is_err(),
                "csetm with {} operand(s) must Err (llvm-mc: too few operands)",
                arity
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_csetm
        #[test]
        fn encode_csetm_neg_extra_operand(
            rd in rd_strat(),
            cond in cond14_strat(),
            which in 0u32..=3u32,
        ) {
            let extra = extra_operand(which);
            let ops = [
                Operand::Reg(rd.clone()),
                Operand::Cond(cond.clone()),
                extra,
            ];
            prop_assert!(
                encode_csetm(&ops).is_err(),
                "csetm {}, {}, extra (which={}) must Err (llvm-mc: invalid operand)",
                rd, cond, which
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_csetm
        #[test]
        fn encode_csetm_neg_al_nv(
            rd in rd_strat(),
            which in 0u32..=1u32,
        ) {
            let cond = if which == 0 { "al" } else { "nv" };
            let ops = [Operand::Reg(rd.clone()), Operand::Cond(cond.to_string())];
            prop_assert!(
                encode_csetm(&ops).is_err(),
                "csetm {}, {} must Err (llvm-mc: AL and NV invalid for CSETM)",
                rd, cond
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_csetm
        #[test]
        fn encode_csetm_neg_wrong_reg(kind in 0u32..=8u32, n in 0u32..=31u32) {
            let name = wrong_reg_name(kind, n);
            let ops = [Operand::Reg(name.clone()), Operand::Cond("eq".into())];
            prop_assert!(
                encode_csetm(&ops).is_err(),
                "csetm {}, eq must Err (llvm-mc rejects SP/FP/invalid)",
                name
            );
        }

        // Oracle: negative_error (coverage sweep: encode_cond None arm)
        // Target: encoder.compare_branch.encode_csetm
        #[test]
        fn encode_csetm_neg_unknown_cond(
            rd in rd_strat(),
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
            let ops = [Operand::Reg(rd), Operand::Cond(cond.to_string())];
            prop_assert!(
                encode_csetm(&ops).is_err(),
                "csetm with unknown cond '{}' must Err",
                cond
            );
        }

        // Oracle: negative_error (coverage sweep: get_reg parse_reg_num None arm)
        // Target: encoder.compare_branch.encode_csetm
        #[test]
        fn encode_csetm_neg_invalid_name(which in 0u32..=7u32) {
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
            let ops = [Operand::Reg(name.to_string()), Operand::Cond("eq".into())];
            prop_assert!(
                encode_csetm(&ops).is_err(),
                "csetm {} , eq must Err (not a valid register name)",
                name
            );
        }

        // Oracle: negative_error (coverage sweep: get_reg non-Reg / cond not Cond)
        // Target: encoder.compare_branch.encode_csetm
        #[test]
        fn encode_csetm_neg_bad_operand_kind(slot in 0u32..=1u32, which in 0u32..=4u32) {
            let bad = match which {
                0 => Operand::Imm(0),
                1 => Operand::Mem { base: "x0".into(), offset: 0 },
                2 => Operand::Symbol("foo".into()),
                3 => Operand::Shift { kind: "lsl".into(), amount: 0 },
                _ => Operand::Label("foo".into()),
            };
            let mut ops = vec![
                Operand::Reg("x0".into()),
                Operand::Cond("eq".into()),
            ];
            ops[slot as usize] = bad;
            prop_assert!(
                encode_csetm(&ops).is_err(),
                "csetm with non-Reg/non-Cond at slot {} which={} must Err",
                slot, which
            );
        }
    }

    #[test]
    fn test_encode_csetm_regression_extra_operand() {
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Cond("eq".into()),
            Operand::Reg("x2".into()),
        ];
        assert!(
            encode_csetm(&ops).is_err(),
            "csetm x0, eq, x2 must Err; llvm-mc rejects a third operand"
        );
    }

    #[test]
    fn test_encode_csetm_regression_al() {
        let ops = [Operand::Reg("x0".into()), Operand::Cond("al".into())];
        assert!(
            encode_csetm(&ops).is_err(),
            "csetm x0, al must Err; llvm-mc rejects AL/NV for CSETM"
        );
    }

    #[test]
    fn test_encode_csetm_regression_nv() {
        let ops = [Operand::Reg("x0".into()), Operand::Cond("nv".into())];
        assert!(
            encode_csetm(&ops).is_err(),
            "csetm x0, nv must Err; llvm-mc rejects AL/NV for CSETM"
        );
    }

    #[test]
    fn test_encode_csetm_regression_sp() {
        let ops = [Operand::Reg("sp".into()), Operand::Cond("eq".into())];
        assert!(
            encode_csetm(&ops).is_err(),
            "csetm sp, eq must Err; llvm-mc rejects SP (register 31 is XZR)"
        );
    }

    #[test]
    fn test_encode_csetm_regression_fp_reg() {
        let ops = [Operand::Reg("d0".into()), Operand::Cond("eq".into())];
        assert!(
            encode_csetm(&ops).is_err(),
            "csetm d0, eq must Err; llvm-mc rejects FP/SIMD registers"
        );
    }
}

#[cfg(test)]
mod encode_csinc_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler (CSINC Rd, Rn, Rm, cond);
    //   algebraic.metamorphic (CSINC vs CSEL XOR bit 10; CSINC vs CINC alias);
    //   algebraic.invariant (CSINC field layout); negative_error (arity / extra /
    //   SP-FP-mixed-invalid / unknown cond).
    // Evidence: src/backend/arm/assembler/README.md:5-14 gas-compat;
    //   encoder/mod.rs:1-7 32-bit AArch64 words; encoder/mod.rs:309 csinc dispatch;
    //   compare_branch.rs:99-109 CSINC op2=01; compare_branch.rs:292 CINC alias;
    //   ARM ARM Conditional Select Increment (architectural):
    //   sf 0 0 11010100 Rm cond 01 Rn Rd; AL/NV valid (unlike CINC/CSET aliases);
    //   register 31 is XZR/WZR, Wt/Xt only.
    // Stronger considered:
    //   - State machine: rejected — encode_csinc is a pure function with no lifecycle
    //   - Algebraic round-trip via in-tree decoder: rejected — no CSINC decoder
    //   - encode_csel / encode_csneg as differential siblings: rejected — different
    //     jobs (CSEL op2=00 / CSNEG op=1); used only as metamorphic XOR transforms
    //   - encode_cinc / encode_cset as differential siblings: rejected — different
    //     mnemonic/arity (3-op / 2-op aliases); used as metamorphic alias transforms
    // Weaker available: algebraic.invariant (opcode/Rm/cond/op2/Rn/Rd fields),
    //   algebraic.metamorphic (vs encode_csel / encode_cinc), negative_error
    //   (arity/extra/SP/FP/mixed/unknown cond)
    // Differential: candidate=encode_csinc, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=[Reg(rd), Reg(rn), Reg(rm), Cond(c)] <-> asm text `csinc rd, rn, rm, c`

    use super::{encode_cinc, encode_csel, encode_csinc};
    use super::super::EncodeResult;
    use crate::backend::arm::assembler::parser::Operand;
    use proptest::prelude::*;
    use std::io::Write;
    use std::process::{Command, Stdio};

    const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
    const CASES: u32 = 1000;
    /// Canonical cond names whose encodings are 0..=15 (AL=14 / NV=15 included).
    const COND16: [&str; 16] = [
        "eq", "ne", "cs", "cc", "mi", "pl", "vs", "vc",
        "hi", "ls", "ge", "lt", "gt", "le", "al", "nv",
    ];
    const COND16_WITH_ALIASES: [&str; 18] = [
        "eq", "ne", "cs", "hs", "cc", "lo", "mi", "pl",
        "vs", "vc", "hi", "ls", "ge", "lt", "gt", "le", "al", "nv",
    ];
    /// Cond14: AL/NV excluded (CINC alias is not valid for those).
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

    /// Invert a condition name by flipping encoding bit 0 (ARM ARM invert(cond)).
    fn invert_cond_name(c: &str) -> &'static str {
        match c.to_lowercase().as_str() {
            "eq" => "ne",
            "ne" => "eq",
            "cs" | "hs" => "cc",
            "cc" | "lo" => "cs",
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
            _ => "eq",
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

    /// n=0..30 -> wN; 31 -> wzr. wsp is not in the valid CSINC domain.
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

    fn same_width_triple() -> impl Strategy<Value = (String, String, String)> {
        prop_oneof![
            (x_name_strat(), x_name_strat(), x_name_strat()),
            (w_name_strat(), w_name_strat(), w_name_strat()),
        ]
    }

    fn same_width_pair() -> impl Strategy<Value = (String, String)> {
        prop_oneof![
            (x_name_strat(), x_name_strat()),
            (w_name_strat(), w_name_strat()),
        ]
    }

    fn cond16_strat() -> impl Strategy<Value = String> {
        prop_oneof![
            Just("eq".to_string()),
            Just("al".to_string()),
            Just("nv".to_string()),
            Just("hs".to_string()),
            Just("lo".to_string()),
            Just("le".to_string()),
            prop::sample::select(
                COND16_WITH_ALIASES.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
            ),
        ]
    }

    fn cond14_strat() -> impl Strategy<Value = String> {
        prop_oneof![
            Just("eq".to_string()),
            Just("ne".to_string()),
            Just("hs".to_string()),
            Just("lo".to_string()),
            Just("le".to_string()),
            prop::sample::select(
                COND14_WITH_ALIASES.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
            ),
        ]
    }

    fn extra_operand(which: u32) -> Operand {
        match which {
            0 => Operand::Reg("x3".into()),
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

    fn bad_ops(kind: u32, n: u32) -> [Operand; 4] {
        let n = n.min(31);
        let eq = Operand::Cond("eq".into());
        let xn = Operand::Reg(format!("x{}", n.min(30)));
        let wn = Operand::Reg(format!("w{}", n.min(30)));
        match kind {
            0 => [Operand::Reg("sp".into()), xn.clone(), xn, eq],
            1 => [xn.clone(), Operand::Reg("sp".into()), xn, eq],
            2 => [xn.clone(), xn, Operand::Reg("sp".into()), eq],
            3 => [Operand::Reg("wsp".into()), wn.clone(), wn, eq],
            4 => [xn.clone(), wn, xn, eq],
            5 => [
                Operand::Reg(format!("d{}", n)),
                Operand::Reg(format!("d{}", n)),
                Operand::Reg(format!("d{}", n)),
                eq,
            ],
            6 => [
                Operand::Reg(format!("s{}", n)),
                Operand::Reg("w0".into()),
                Operand::Reg("w1".into()),
                eq,
            ],
            7 => [
                Operand::Reg(format!("q{}", n)),
                Operand::Reg("x0".into()),
                Operand::Reg("x1".into()),
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
                [
                    Operand::Reg(name),
                    Operand::Reg("x0".into()),
                    Operand::Reg("x1".into()),
                    eq,
                ]
            }
        }
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_csinc_kat_llvm_mc_x0_x1_x2_eq() {
        let want = 0x9a820420u32;
        let mc = llvm_mc_word("csinc x0, x1, x2, eq").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("x1".into()),
            Operand::Reg("x2".into()),
            Operand::Cond("eq".into()),
        ];
        match encode_csinc(&ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for csinc x0, x1, x2, eq, got {:?}",
                want, other
            ),
        }
    }

    #[test]
    fn encode_csinc_kat_llvm_mc_w0_w1_w2_ne() {
        let want = 0x1a821420u32;
        let mc = llvm_mc_word("csinc w0, w1, w2, ne").expect("llvm-mc KAT w-form");
        assert_eq!(mc, want, "llvm-mc KAT w-form mapping broken");
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Reg("w1".into()),
            Operand::Reg("w2".into()),
            Operand::Cond("ne".into()),
        ];
        match encode_csinc(&ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for csinc w0, w1, w2, ne, got {:?}",
                want, other
            ),
        }
    }

    #[test]
    fn encode_csinc_kat_llvm_mc_al() {
        let want = 0x9a82e420u32;
        let mc = llvm_mc_word("csinc x0, x1, x2, al").expect("llvm-mc KAT al");
        assert_eq!(mc, want, "llvm-mc KAT al mapping broken");
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("x1".into()),
            Operand::Reg("x2".into()),
            Operand::Cond("al".into()),
        ];
        match encode_csinc(&ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for csinc x0, x1, x2, al, got {:?}",
                want, other
            ),
        }
    }

    #[test]
    fn encode_csinc_kat_cinc_alias() {
        // ARM ARM: CINC x0, x1, eq == CSINC x0, x1, x1, ne
        let want = 0x9a811420u32;
        let mc = llvm_mc_word("csinc x0, x1, x1, ne").expect("llvm-mc KAT cinc alias");
        assert_eq!(mc, want, "llvm-mc KAT cinc-alias mapping broken");
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("x1".into()),
            Operand::Reg("x1".into()),
            Operand::Cond("ne".into()),
        ];
        match encode_csinc(&ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for csinc x0, x1, x1, ne, got {:?}",
                want, other
            ),
        }
    }

    proptest! {
        #![proptest_config(cfg())]

        // Oracle: differential
        // Target: encoder.compare_branch.encode_csinc
        #[test]
        fn encode_csinc_diff_llvm_mc(
            (rd, rn, rm) in same_width_triple(),
            cond in cond16_strat(),
        ) {
            let asm = format!("csinc {}, {}, {}, {}", rd, rn, rm, cond);
            let ops = [
                Operand::Reg(rd.clone()),
                Operand::Reg(rn.clone()),
                Operand::Reg(rm.clone()),
                Operand::Cond(cond.clone()),
            ];
            let sut = match encode_csinc(&ops) {
                Ok(EncodeResult::Word(w)) => w,
                other => {
                    return Err(TestCaseError::fail(format!(
                        "SUT rejected valid CSINC {}: {:?}",
                        asm, other
                    )));
                }
            };
            let mc = llvm_mc_word(&asm)
                .map_err(|e| TestCaseError::fail(format!(
                    "llvm-mc rejected valid CSINC {}: {}",
                    asm, e
                )))?;
            prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
        }

        // Oracle: algebraic.metamorphic
        // Target: encoder.compare_branch.encode_csinc
        #[test]
        fn encode_csinc_meta_vs_csel(
            (rd, rn, rm) in same_width_triple(),
            cond in cond16_strat(),
        ) {
            let ops = [
                Operand::Reg(rd.clone()),
                Operand::Reg(rn.clone()),
                Operand::Reg(rm.clone()),
                Operand::Cond(cond.clone()),
            ];
            let left = encode_csinc(&ops).map_err(|e| TestCaseError::fail(e))?;
            let right = encode_csel(&ops).map_err(|e| TestCaseError::fail(e))?;
            match (left, right) {
                (EncodeResult::Word(w_csinc), EncodeResult::Word(w_csel)) => {
                    prop_assert_eq!(
                        w_csinc ^ w_csel,
                        1u32 << 10,
                        "CSINC XOR CSEL must be bit 10 (csinc={:#010x} csel={:#010x}) for {}, {}, {}, {}",
                        w_csinc, w_csel, rd, rn, rm, cond
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
        // Target: encoder.compare_branch.encode_csinc
        #[test]
        fn encode_csinc_meta_vs_cinc(
            (rd, rn) in same_width_pair(),
            cond in cond14_strat(),
        ) {
            let inv = invert_cond_name(&cond);
            let csinc_ops = [
                Operand::Reg(rd.clone()),
                Operand::Reg(rn.clone()),
                Operand::Reg(rn.clone()),
                Operand::Cond(inv.to_string()),
            ];
            let cinc_ops = [
                Operand::Reg(rd.clone()),
                Operand::Reg(rn.clone()),
                Operand::Cond(cond.clone()),
            ];
            let left = encode_csinc(&csinc_ops).map_err(|e| TestCaseError::fail(e))?;
            let right = encode_cinc(&cinc_ops).map_err(|e| TestCaseError::fail(e))?;
            match (left, right) {
                (EncodeResult::Word(w_csinc), EncodeResult::Word(w_cinc)) => {
                    prop_assert_eq!(
                        w_csinc, w_cinc,
                        "CSINC {}, {}, {}, {} must equal CINC {}, {}, {}",
                        rd, rn, rn, inv, rd, rn, cond
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
        // Target: encoder.compare_branch.encode_csinc
        #[test]
        fn encode_csinc_word_layout(
            rd_n in 0u32..=31u32,
            rn_n in 0u32..=31u32,
            rm_n in 0u32..=31u32,
            is_64 in any::<bool>(),
            cond_enc in 0u32..=15u32,
        ) {
            let rd = gpr_name(rd_n, is_64);
            let rn = gpr_name(rn_n, is_64);
            let rm = gpr_name(rm_n, is_64);
            let cond = COND16[cond_enc as usize];
            let ops = [
                Operand::Reg(rd),
                Operand::Reg(rn),
                Operand::Reg(rm),
                Operand::Cond(cond.to_string()),
            ];
            match encode_csinc(&ops) {
                Ok(EncodeResult::Word(w)) => {
                    let sf = if is_64 { 1u32 } else { 0u32 };
                    prop_assert_eq!((w >> 31) & 1, sf, "sf bit 31");
                    prop_assert_eq!((w >> 30) & 1, 0u32, "op bit 30");
                    prop_assert_eq!((w >> 29) & 1, 0u32, "S bit 29");
                    prop_assert_eq!((w >> 21) & 0xFF, 0b11010100u32, "bits[28:21]");
                    prop_assert_eq!((w >> 16) & 0x1F, rm_n, "Rm [20:16]");
                    prop_assert_eq!((w >> 12) & 0xF, cond_enc, "cond [15:12]");
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
        // Target: encoder.compare_branch.encode_csinc
        #[test]
        fn encode_csinc_neg_arity(arity in 0u32..=3u32) {
            let ops: Vec<Operand> = match arity {
                0 => vec![],
                1 => vec![Operand::Reg("x0".into())],
                2 => vec![Operand::Reg("x0".into()), Operand::Reg("x1".into())],
                _ => vec![
                    Operand::Reg("x0".into()),
                    Operand::Reg("x1".into()),
                    Operand::Reg("x2".into()),
                ],
            };
            prop_assert!(
                encode_csinc(&ops).is_err(),
                "csinc with {} operand(s) must Err (llvm-mc: too few operands)",
                arity
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_csinc
        #[test]
        fn encode_csinc_neg_extra_operand(
            (rd, rn, rm) in same_width_triple(),
            cond in cond16_strat(),
            which in 0u32..=3u32,
        ) {
            let extra = extra_operand(which);
            let ops = [
                Operand::Reg(rd.clone()),
                Operand::Reg(rn.clone()),
                Operand::Reg(rm.clone()),
                Operand::Cond(cond.clone()),
                extra,
            ];
            prop_assert!(
                encode_csinc(&ops).is_err(),
                "csinc {}, {}, {}, {}, extra (which={}) must Err (llvm-mc: invalid operand)",
                rd, rn, rm, cond, which
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_csinc
        #[test]
        fn encode_csinc_neg_wrong_reg(kind in 0u32..=8u32, n in 0u32..=31u32) {
            let ops = bad_ops(kind, n);
            prop_assert!(
                encode_csinc(&ops).is_err(),
                "csinc with wrong-reg kind={} n={} must Err (llvm-mc rejects SP/FP/mixed/invalid)",
                kind, n
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_csinc
        #[test]
        fn encode_csinc_neg_unknown_cond(
            (rd, rn, rm) in same_width_triple(),
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
                Operand::Reg(rm),
                Operand::Cond(cond.to_string()),
            ];
            prop_assert!(
                encode_csinc(&ops).is_err(),
                "csinc with unknown cond '{}' must Err",
                cond
            );
        }

        // Oracle: negative_error (coverage sweep: get_reg parse_reg_num None arm)
        // Target: encoder.compare_branch.encode_csinc
        #[test]
        fn encode_csinc_neg_invalid_name(which in 0u32..=7u32) {
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
                Operand::Reg("x1".into()),
                Operand::Cond("eq".into()),
            ];
            prop_assert!(
                encode_csinc(&ops).is_err(),
                "csinc {} , x0, x1, eq must Err (not a valid register name)",
                name
            );
        }

        // Oracle: negative_error (coverage sweep: get_reg non-Reg / cond-not-Cond)
        // Target: encoder.compare_branch.encode_csinc
        #[test]
        fn encode_csinc_neg_bad_operand_kind(slot in 0u32..=3u32, which in 0u32..=4u32) {
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
                Operand::Reg("x2".into()),
                Operand::Cond("eq".into()),
            ];
            ops[slot as usize] = bad;
            prop_assert!(
                encode_csinc(&ops).is_err(),
                "csinc with non-Reg/non-Cond at slot {} which={} must Err",
                slot, which
            );
        }
    }

    #[test]
    fn test_encode_csinc_regression_extra_operand() {
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("x1".into()),
            Operand::Reg("x2".into()),
            Operand::Cond("eq".into()),
            Operand::Reg("x3".into()),
        ];
        assert!(
            encode_csinc(&ops).is_err(),
            "csinc x0, x1, x2, eq, x3 must Err; llvm-mc rejects a fifth operand"
        );
    }

    #[test]
    fn test_encode_csinc_regression_sp() {
        let ops = [
            Operand::Reg("sp".into()),
            Operand::Reg("x0".into()),
            Operand::Reg("x1".into()),
            Operand::Cond("eq".into()),
        ];
        assert!(
            encode_csinc(&ops).is_err(),
            "csinc sp, x0, x1, eq must Err; llvm-mc rejects SP (register 31 is XZR)"
        );
    }

    #[test]
    fn test_encode_csinc_regression_mixed_width() {
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("w1".into()),
            Operand::Reg("x2".into()),
            Operand::Cond("eq".into()),
        ];
        assert!(
            encode_csinc(&ops).is_err(),
            "csinc x0, w1, x2, eq must Err; llvm-mc rejects mixed x/w"
        );
    }

    #[test]
    fn test_encode_csinc_regression_fp_reg() {
        let ops = [
            Operand::Reg("d0".into()),
            Operand::Reg("d1".into()),
            Operand::Reg("d2".into()),
            Operand::Cond("eq".into()),
        ];
        assert!(
            encode_csinc(&ops).is_err(),
            "csinc d0, d1, d2, eq must Err; llvm-mc rejects FP/SIMD registers"
        );
    }
}

#[cfg(test)]
mod encode_csinv_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler (CSINV Rd, Rn, Rm, cond);
    //   algebraic.metamorphic (CSINV vs CSEL XOR bit 30; CSINV vs CINV alias);
    //   algebraic.invariant (CSINV field layout); negative_error (arity / extra /
    //   SP-FP-mixed-invalid / unknown cond).
    // Evidence: src/backend/arm/assembler/README.md:5-14 gas-compat;
    //   encoder/mod.rs:1-7 32-bit AArch64 words; encoder/mod.rs:310 csinv dispatch;
    //   compare_branch.rs:113-125 CSINV op=1 op2=00; compare_branch.rs:308 CINV alias;
    //   ARM ARM Conditional Select Invert (architectural):
    //   sf 1 0 11010100 Rm cond 00 Rn Rd; AL/NV valid (unlike CINV/CSETM aliases);
    //   register 31 is XZR/WZR, Wt/Xt only.
    // Stronger considered:
    //   - State machine: rejected — encode_csinv is a pure function with no lifecycle
    //   - Algebraic round-trip via in-tree decoder: rejected — no CSINV decoder
    //   - encode_csel / encode_csneg as differential siblings: rejected — different
    //     jobs (CSEL op=0 / CSNEG op2=01); used only as metamorphic XOR transforms
    //   - encode_cinv / encode_csetm as differential siblings: rejected — different
    //     mnemonic/arity (3-op / 2-op aliases); used as metamorphic alias transforms
    // Weaker available: algebraic.invariant (opcode/Rm/cond/op2/Rn/Rd fields),
    //   algebraic.metamorphic (vs encode_csel / encode_cinv), negative_error
    //   (arity/extra/SP/FP/mixed/unknown cond)
    // Differential: candidate=encode_csinv, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=[Reg(rd), Reg(rn), Reg(rm), Cond(c)] <-> asm text `csinv rd, rn, rm, c`

    use super::{encode_cinv, encode_csel, encode_csinv};
    use super::super::EncodeResult;
    use crate::backend::arm::assembler::parser::Operand;
    use proptest::prelude::*;
    use std::io::Write;
    use std::process::{Command, Stdio};

    const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
    const CASES: u32 = 1000;
    /// Canonical cond names whose encodings are 0..=15 (AL=14 / NV=15 included).
    const COND16: [&str; 16] = [
        "eq", "ne", "cs", "cc", "mi", "pl", "vs", "vc",
        "hi", "ls", "ge", "lt", "gt", "le", "al", "nv",
    ];
    const COND16_WITH_ALIASES: [&str; 18] = [
        "eq", "ne", "cs", "hs", "cc", "lo", "mi", "pl",
        "vs", "vc", "hi", "ls", "ge", "lt", "gt", "le", "al", "nv",
    ];
    /// Cond14: AL/NV excluded (CINV alias is not valid for those).
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

    /// Invert a condition name by flipping encoding bit 0 (ARM ARM invert(cond)).
    fn invert_cond_name(c: &str) -> &'static str {
        match c.to_lowercase().as_str() {
            "eq" => "ne",
            "ne" => "eq",
            "cs" | "hs" => "cc",
            "cc" | "lo" => "cs",
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
            _ => "eq",
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

    /// n=0..30 -> wN; 31 -> wzr. wsp is not in the valid CSINV domain.
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

    fn same_width_triple() -> impl Strategy<Value = (String, String, String)> {
        prop_oneof![
            (x_name_strat(), x_name_strat(), x_name_strat()),
            (w_name_strat(), w_name_strat(), w_name_strat()),
        ]
    }

    fn same_width_pair() -> impl Strategy<Value = (String, String)> {
        prop_oneof![
            (x_name_strat(), x_name_strat()),
            (w_name_strat(), w_name_strat()),
        ]
    }

    fn cond16_strat() -> impl Strategy<Value = String> {
        prop_oneof![
            Just("eq".to_string()),
            Just("al".to_string()),
            Just("nv".to_string()),
            Just("hs".to_string()),
            Just("lo".to_string()),
            Just("le".to_string()),
            prop::sample::select(
                COND16_WITH_ALIASES.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
            ),
        ]
    }

    fn cond14_strat() -> impl Strategy<Value = String> {
        prop_oneof![
            Just("eq".to_string()),
            Just("ne".to_string()),
            Just("hs".to_string()),
            Just("lo".to_string()),
            Just("le".to_string()),
            prop::sample::select(
                COND14_WITH_ALIASES.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
            ),
        ]
    }

    fn extra_operand(which: u32) -> Operand {
        match which {
            0 => Operand::Reg("x3".into()),
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

    fn bad_ops(kind: u32, n: u32) -> [Operand; 4] {
        let n = n.min(31);
        let eq = Operand::Cond("eq".into());
        let xn = Operand::Reg(format!("x{}", n.min(30)));
        let wn = Operand::Reg(format!("w{}", n.min(30)));
        match kind {
            0 => [Operand::Reg("sp".into()), xn.clone(), xn, eq],
            1 => [xn.clone(), Operand::Reg("sp".into()), xn, eq],
            2 => [xn.clone(), xn, Operand::Reg("sp".into()), eq],
            3 => [Operand::Reg("wsp".into()), wn.clone(), wn, eq],
            4 => [xn.clone(), wn, xn, eq],
            5 => [
                Operand::Reg(format!("d{}", n)),
                Operand::Reg(format!("d{}", n)),
                Operand::Reg(format!("d{}", n)),
                eq,
            ],
            6 => [
                Operand::Reg(format!("s{}", n)),
                Operand::Reg("w0".into()),
                Operand::Reg("w1".into()),
                eq,
            ],
            7 => [
                Operand::Reg(format!("q{}", n)),
                Operand::Reg("x0".into()),
                Operand::Reg("x1".into()),
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
                [
                    Operand::Reg(name),
                    Operand::Reg("x0".into()),
                    Operand::Reg("x1".into()),
                    eq,
                ]
            }
        }
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_csinv_kat_llvm_mc_x0_x1_x2_eq() {
        let want = 0xda820020u32;
        let mc = llvm_mc_word("csinv x0, x1, x2, eq").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("x1".into()),
            Operand::Reg("x2".into()),
            Operand::Cond("eq".into()),
        ];
        match encode_csinv(&ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for csinv x0, x1, x2, eq, got {:?}",
                want, other
            ),
        }
    }

    #[test]
    fn encode_csinv_kat_llvm_mc_w0_w1_w2_ne() {
        let want = 0x5a821020u32;
        let mc = llvm_mc_word("csinv w0, w1, w2, ne").expect("llvm-mc KAT w-form");
        assert_eq!(mc, want, "llvm-mc KAT w-form mapping broken");
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Reg("w1".into()),
            Operand::Reg("w2".into()),
            Operand::Cond("ne".into()),
        ];
        match encode_csinv(&ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for csinv w0, w1, w2, ne, got {:?}",
                want, other
            ),
        }
    }

    #[test]
    fn encode_csinv_kat_llvm_mc_al() {
        let want = 0xda82e020u32;
        let mc = llvm_mc_word("csinv x0, x1, x2, al").expect("llvm-mc KAT al");
        assert_eq!(mc, want, "llvm-mc KAT al mapping broken");
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("x1".into()),
            Operand::Reg("x2".into()),
            Operand::Cond("al".into()),
        ];
        match encode_csinv(&ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for csinv x0, x1, x2, al, got {:?}",
                want, other
            ),
        }
    }

    #[test]
    fn encode_csinv_kat_cinv_alias() {
        // ARM ARM: CINV x0, x1, eq == CSINV x0, x1, x1, ne
        let want = 0xda811020u32;
        let mc = llvm_mc_word("csinv x0, x1, x1, ne").expect("llvm-mc KAT cinv alias");
        assert_eq!(mc, want, "llvm-mc KAT cinv-alias mapping broken");
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("x1".into()),
            Operand::Reg("x1".into()),
            Operand::Cond("ne".into()),
        ];
        match encode_csinv(&ops) {
            Ok(EncodeResult::Word(w)) => assert_eq!(w, want),
            other => panic!(
                "SUT KAT: expected Word({:#010x}) for csinv x0, x1, x1, ne, got {:?}",
                want, other
            ),
        }
    }

    proptest! {
        #![proptest_config(cfg())]

        // Oracle: differential
        // Target: encoder.compare_branch.encode_csinv
        #[test]
        fn encode_csinv_diff_llvm_mc(
            (rd, rn, rm) in same_width_triple(),
            cond in cond16_strat(),
        ) {
            let asm = format!("csinv {}, {}, {}, {}", rd, rn, rm, cond);
            let ops = [
                Operand::Reg(rd.clone()),
                Operand::Reg(rn.clone()),
                Operand::Reg(rm.clone()),
                Operand::Cond(cond.clone()),
            ];
            let sut = match encode_csinv(&ops) {
                Ok(EncodeResult::Word(w)) => w,
                other => {
                    return Err(TestCaseError::fail(format!(
                        "SUT rejected valid CSINV {}: {:?}",
                        asm, other
                    )));
                }
            };
            let mc = llvm_mc_word(&asm)
                .map_err(|e| TestCaseError::fail(format!(
                    "llvm-mc rejected valid CSINV {}: {}",
                    asm, e
                )))?;
            prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
        }

        // Oracle: algebraic.metamorphic
        // Target: encoder.compare_branch.encode_csinv
        #[test]
        fn encode_csinv_meta_vs_csel(
            (rd, rn, rm) in same_width_triple(),
            cond in cond16_strat(),
        ) {
            let ops = [
                Operand::Reg(rd.clone()),
                Operand::Reg(rn.clone()),
                Operand::Reg(rm.clone()),
                Operand::Cond(cond.clone()),
            ];
            let left = encode_csinv(&ops).map_err(|e| TestCaseError::fail(e))?;
            let right = encode_csel(&ops).map_err(|e| TestCaseError::fail(e))?;
            match (left, right) {
                (EncodeResult::Word(w_csinv), EncodeResult::Word(w_csel)) => {
                    prop_assert_eq!(
                        w_csinv ^ w_csel,
                        1u32 << 30,
                        "CSINV XOR CSEL must be bit 30 (csinv={:#010x} csel={:#010x}) for {}, {}, {}, {}",
                        w_csinv, w_csel, rd, rn, rm, cond
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
        // Target: encoder.compare_branch.encode_csinv
        #[test]
        fn encode_csinv_meta_vs_cinv(
            (rd, rn) in same_width_pair(),
            cond in cond14_strat(),
        ) {
            let inv = invert_cond_name(&cond);
            let csinv_ops = [
                Operand::Reg(rd.clone()),
                Operand::Reg(rn.clone()),
                Operand::Reg(rn.clone()),
                Operand::Cond(inv.to_string()),
            ];
            let cinv_ops = [
                Operand::Reg(rd.clone()),
                Operand::Reg(rn.clone()),
                Operand::Cond(cond.clone()),
            ];
            let left = encode_csinv(&csinv_ops).map_err(|e| TestCaseError::fail(e))?;
            let right = encode_cinv(&cinv_ops).map_err(|e| TestCaseError::fail(e))?;
            match (left, right) {
                (EncodeResult::Word(w_csinv), EncodeResult::Word(w_cinv)) => {
                    prop_assert_eq!(
                        w_csinv, w_cinv,
                        "CSINV {}, {}, {}, {} must equal CINV {}, {}, {}",
                        rd, rn, rn, inv, rd, rn, cond
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
        // Target: encoder.compare_branch.encode_csinv
        #[test]
        fn encode_csinv_word_layout(
            rd_n in 0u32..=31u32,
            rn_n in 0u32..=31u32,
            rm_n in 0u32..=31u32,
            is_64 in any::<bool>(),
            cond_enc in 0u32..=15u32,
        ) {
            let rd = gpr_name(rd_n, is_64);
            let rn = gpr_name(rn_n, is_64);
            let rm = gpr_name(rm_n, is_64);
            let cond = COND16[cond_enc as usize];
            let ops = [
                Operand::Reg(rd),
                Operand::Reg(rn),
                Operand::Reg(rm),
                Operand::Cond(cond.to_string()),
            ];
            match encode_csinv(&ops) {
                Ok(EncodeResult::Word(w)) => {
                    let sf = if is_64 { 1u32 } else { 0u32 };
                    prop_assert_eq!((w >> 31) & 1, sf, "sf bit 31");
                    prop_assert_eq!((w >> 30) & 1, 1u32, "op bit 30");
                    prop_assert_eq!((w >> 29) & 1, 0u32, "S bit 29");
                    prop_assert_eq!((w >> 21) & 0xFF, 0b11010100u32, "bits[28:21]");
                    prop_assert_eq!((w >> 16) & 0x1F, rm_n, "Rm [20:16]");
                    prop_assert_eq!((w >> 12) & 0xF, cond_enc, "cond [15:12]");
                    prop_assert_eq!((w >> 10) & 0x3, 0b00u32, "op2 [11:10]");
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
        // Target: encoder.compare_branch.encode_csinv
        #[test]
        fn encode_csinv_neg_arity(arity in 0u32..=3u32) {
            let ops: Vec<Operand> = match arity {
                0 => vec![],
                1 => vec![Operand::Reg("x0".into())],
                2 => vec![Operand::Reg("x0".into()), Operand::Reg("x1".into())],
                _ => vec![
                    Operand::Reg("x0".into()),
                    Operand::Reg("x1".into()),
                    Operand::Reg("x2".into()),
                ],
            };
            prop_assert!(
                encode_csinv(&ops).is_err(),
                "csinv with {} operand(s) must Err (llvm-mc: too few operands)",
                arity
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_csinv
        #[test]
        fn encode_csinv_neg_extra_operand(
            (rd, rn, rm) in same_width_triple(),
            cond in cond16_strat(),
            which in 0u32..=3u32,
        ) {
            let extra = extra_operand(which);
            let ops = [
                Operand::Reg(rd.clone()),
                Operand::Reg(rn.clone()),
                Operand::Reg(rm.clone()),
                Operand::Cond(cond.clone()),
                extra,
            ];
            prop_assert!(
                encode_csinv(&ops).is_err(),
                "csinv {}, {}, {}, {}, extra (which={}) must Err (llvm-mc: invalid operand)",
                rd, rn, rm, cond, which
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_csinv
        #[test]
        fn encode_csinv_neg_wrong_reg(kind in 0u32..=8u32, n in 0u32..=31u32) {
            let ops = bad_ops(kind, n);
            prop_assert!(
                encode_csinv(&ops).is_err(),
                "csinv with wrong-reg kind={} n={} must Err (llvm-mc rejects SP/FP/mixed/invalid)",
                kind, n
            );
        }

        // Oracle: negative_error
        // Target: encoder.compare_branch.encode_csinv
        #[test]
        fn encode_csinv_neg_unknown_cond(
            (rd, rn, rm) in same_width_triple(),
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
                Operand::Reg(rm),
                Operand::Cond(cond.to_string()),
            ];
            prop_assert!(
                encode_csinv(&ops).is_err(),
                "csinv with unknown cond '{}' must Err",
                cond
            );
        }

        // Oracle: negative_error (coverage sweep: get_reg parse_reg_num None arm)
        // Target: encoder.compare_branch.encode_csinv
        #[test]
        fn encode_csinv_neg_invalid_name(which in 0u32..=7u32) {
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
                Operand::Reg("x1".into()),
                Operand::Cond("eq".into()),
            ];
            prop_assert!(
                encode_csinv(&ops).is_err(),
                "csinv {} , x0, x1, eq must Err (not a valid register name)",
                name
            );
        }

        // Oracle: negative_error (coverage sweep: get_reg non-Reg / cond-not-Cond)
        // Target: encoder.compare_branch.encode_csinv
        #[test]
        fn encode_csinv_neg_bad_operand_kind(slot in 0u32..=3u32, which in 0u32..=4u32) {
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
                Operand::Reg("x2".into()),
                Operand::Cond("eq".into()),
            ];
            ops[slot as usize] = bad;
            prop_assert!(
                encode_csinv(&ops).is_err(),
                "csinv with non-Reg/non-Cond at slot {} which={} must Err",
                slot, which
            );
        }
    }

    #[test]
    fn test_encode_csinv_regression_extra_operand() {
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("x1".into()),
            Operand::Reg("x2".into()),
            Operand::Cond("eq".into()),
            Operand::Reg("x3".into()),
        ];
        assert!(
            encode_csinv(&ops).is_err(),
            "csinv x0, x1, x2, eq, x3 must Err; llvm-mc rejects a fifth operand"
        );
    }

    #[test]
    fn test_encode_csinv_regression_sp() {
        let ops = [
            Operand::Reg("sp".into()),
            Operand::Reg("x0".into()),
            Operand::Reg("x1".into()),
            Operand::Cond("eq".into()),
        ];
        assert!(
            encode_csinv(&ops).is_err(),
            "csinv sp, x0, x1, eq must Err; llvm-mc rejects SP (register 31 is XZR)"
        );
    }

    #[test]
    fn test_encode_csinv_regression_mixed_width() {
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("w1".into()),
            Operand::Reg("x2".into()),
            Operand::Cond("eq".into()),
        ];
        assert!(
            encode_csinv(&ops).is_err(),
            "csinv x0, w1, x2, eq must Err; llvm-mc rejects mixed x/w"
        );
    }

    #[test]
    fn test_encode_csinv_regression_fp_reg() {
        let ops = [
            Operand::Reg("d0".into()),
            Operand::Reg("d1".into()),
            Operand::Reg("d2".into()),
            Operand::Cond("eq".into()),
        ];
        assert!(
            encode_csinv(&ops).is_err(),
            "csinv d0, d1, d2, eq must Err; llvm-mc rejects FP/SIMD registers"
        );
    }
}
