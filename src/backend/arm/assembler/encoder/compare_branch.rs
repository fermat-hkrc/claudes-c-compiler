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
mod scratch {
    use super::*;
    use crate::backend::arm::assembler::parser::Operand;

    /// Issue #37: BL operand is a label or encodable offset; `bl :lo12:foo` must be Err.
    /// clang: llvm-mc rejects modifiers on BL.
    #[test]
    fn manual_bl_modifier() {
        let r = encode_bl(&[Operand::Modifier { kind: "lo12".to_string(), symbol: "foo".to_string() }]);
        match r {
            Ok(EncodeResult::WordWithReloc { word, reloc }) => {
                println!("bl :lo12:foo -> WordWithReloc {{ 0x{:08x}, {:?} }}  (should be Err)", word, reloc.reloc_type);
                panic!("modifier must be Err for bl, got WordWithReloc (Call26 applied to illegal modifier)");
            }
            other => println!("bl :lo12:foo -> {:?}  (Err = correct)", other),
        }
    }
}

#[cfg(test)]
mod scratch_blr {
    use super::*;
    use crate::backend::arm::assembler::parser::Operand;

    /// Issue #42: BLR takes a single Xn; `blr x0, x1` must be Err.
    #[test]
    fn manual_blr_extra_operand() {
        let r = encode_blr(&[Operand::Reg("x0".to_string()), Operand::Reg("x1".to_string())]);
        match r {
            Ok(EncodeResult::Word(w)) => {
                println!("blr x0,x1 -> 0x{:08x}  (should be Err; extra operand dropped)", w);
                panic!("extra operand must be Err for blr, got Ok(0x{:08x})", w);
            }
            other => println!("blr x0,x1 -> {:?}  (Err = correct)", other),
        }
    }
}

#[cfg(test)]
mod scratch_br {
    use super::*;
    use crate::backend::arm::assembler::parser::Operand;

    /// Issue #46: BR takes a single Xn; `br x0, x1` must be Err.
    /// Issue #49: BR takes Xn only; `br w0` must be Err (not silently br x0).
    #[test]
    fn manual_br_two_defects() {
        let r  = encode_br(&[Operand::Reg("x0".to_string()), Operand::Reg("x1".to_string())]);
        let r2 = encode_br(&[Operand::Reg("w0".to_string())]);
        println!("br x0,x1 -> {:?}  [#46]", r);
        println!("br w0    -> {:?}  [#49]", r2);
        assert!(r.is_err(), "#46: extra operand must be Err for br");
        assert!(r2.is_err(), "#49: W register must be Err for br (not silently br x0)");
    }

    /// Issue #51: `b #imm` (explicit PC offset) is valid gas syntax — llvm-mc emits
    /// 0x14000000 for `b #0`; encode_branch must accept the immediate form.
    #[test]
    fn manual_branch_imm() {
        let r = encode_branch(&[Operand::Imm(0)]);
        match r {
            Ok(EncodeResult::Word(w)) => {
                println!("b #0 -> 0x{:08x}  (llvm-mc: 0x14000000)", w);
                assert_eq!(w, 0x14000000);
            }
            Err(e) => {
                println!("b #0 -> Err({:?})  (llvm-mc assembles it as 0x14000000)", e);
                panic!("immediate PC-offset form must be accepted (llvm-mc differential)");
            }
            other => panic!("expected Word, got {:?}", other),
        }
    }

}

#[cfg(test)]
mod scratch_cbz {
    use super::*;
    use crate::backend::arm::assembler::parser::Operand;

    /// Issue #53: CBZ takes (reg, target); `cbz x0, L, x1` must be Err.
    /// Issue #54: CBZ Rt is a GPR; `cbz d0, L` must be Err (not silently w0).
    #[test]
    fn manual_cbz_two_defects() {
        let r  = encode_cbz(&[Operand::Reg("x0".to_string()), Operand::Symbol("L".to_string()),
                              Operand::Reg("x1".to_string())], false);
        let r2 = encode_cbz(&[Operand::Reg("d0".to_string()), Operand::Symbol("L".to_string())], false);
        println!("cbz x0, L, x1 -> {:?}  [#53]", r);
        println!("cbz d0, L      -> {:?}  [#54]", r2);
        assert!(r.is_err(), "#53: third operand must be Err for cbz");
        assert!(r2.is_err(), "#54: FP/SIMD register must be Err for cbz (not silently w0)");
    }
}

#[cfg(test)]
mod scratch_ccmp {
    use super::*;
    use crate::backend::arm::assembler::parser::Operand;

    /// Issue #59: imm5 ∈ [0,31], nzcv ∈ [0,15]; out-of-range must be Err, not masked.
    #[test]
    fn manual_ccmp_imm_ranges() {
        let r  = encode_ccmp_ccmn(&[Operand::Reg("x0".to_string()), Operand::Imm(-1),
                                     Operand::Imm(0), Operand::Cond("eq".to_string())], false);
        let r2 = encode_ccmp_ccmn(&[Operand::Reg("x0".to_string()), Operand::Imm(0),
                                     Operand::Imm(16), Operand::Cond("eq".to_string())], true);
        println!("ccmn x0,#-1,#0,eq -> {:?}  (imm5 -1 & 0x1F = 31, silent)  [#59]", r);
        println!("ccmp x0,#0,#16,eq -> {:?}  (nzcv 16 & 0xF  = 0,  silent)  [#59]", r2);
        assert!(r.is_err(), "#59: imm5 = -1 must be Err");
        assert!(r2.is_err(), "#59: nzcv = 16 must be Err");
    }
}

#[cfg(test)]
mod scratch_width {
    use super::*;
    use crate::backend::arm::assembler::parser::Operand;

    /// #65: cinc x0, w0, eq — mixed width must be Err.
    /// #73: cmn d0, #0 — FP/SIMD as GPR must be Err.
    /// #81: cmp x0, w0 — mixed width without extend must be Err.
    #[test]
    fn manual_width_defects() {
        let r65 = encode_cinc(&[Operand::Reg("x0".to_string()), Operand::Reg("w0".to_string()),
                                 Operand::Cond("eq".to_string())]);
        let r73 = encode_cmn(&[Operand::Reg("d0".to_string()), Operand::Imm(0)]);
        let r81 = encode_cmp(&[Operand::Reg("x0".to_string()), Operand::Reg("w0".to_string())]);
        println!("cinc x0,w0,eq -> {:?}  [#65]", r65);
        println!("cmn d0,#0     -> {:?}  [#73]", r73);
        println!("cmp x0,w0     -> {:?}  [#81]", r81);
        assert!(r65.is_err(), "#65: mixed x/w must be Err for cinc");
        assert!(r73.is_err(), "#73: FP/SIMD register must be Err for cmn");
        assert!(r81.is_err(), "#81: mixed x/w without extend must be Err for cmp");
    }
}

#[cfg(test)]
mod scratch_cmp_zr {
    use super::*;
    use crate::backend::arm::assembler::parser::Operand;

    /// Issue #83: CMP immediate-form Rn is Xn|SP; XZR/WZR must be Err
    /// (reg 31 in that form means SP, so `cmp xzr,#0` would silently become `cmp sp,#0`).
    #[test]
    fn manual_cmp_zr_imm() {
        let r  = encode_cmp(&[Operand::Reg("xzr".to_string()), Operand::Imm(0)]);
        let r2 = encode_cmp(&[Operand::Reg("wzr".to_string()), Operand::Imm(0)]);
        println!("cmp xzr,#0 -> {:?}  [#83]", r);
        println!("cmp wzr,#0 -> {:?}  [#83]", r2);
        assert!(r.is_err(), "#83: xzr must be Err in cmp immediate form");
        assert!(r2.is_err(), "#83: wzr must be Err in cmp immediate form");
    }
}

#[cfg(test)]
mod scratch_cneg {
    use super::*;
    use crate::backend::arm::assembler::parser::Operand;

    /// Issue #85: CNEG takes (Rd, Rn, cond); a 4th operand must be Err.
    #[test]
    fn manual_cneg_extra_operand() {
        let r = encode_cneg(&[Operand::Reg("x0".to_string()), Operand::Reg("x0".to_string()),
                               Operand::Cond("eq".to_string()), Operand::Reg("x2".to_string())]);
        println!("cneg x0,x0,eq,x2 -> {:?}  [#85]", r);
        assert!(r.is_err(), "#85: fourth operand must be Err for cneg");
    }
}

#[cfg(test)]
mod scratch_cond_ops {
    use super::*;
    use crate::backend::arm::assembler::parser::Operand;

    /// #100: CSETM Rd is never SP; `csetm sp, eq` must be Err (not silently xzr).
    /// #103: CSINC all GPRs same width; `csinc x0, w1, x2, eq` must be Err.
    /// #109: CSNEG takes 4 operands; a 5th must be Err.
    #[test]
    fn manual_cond_ops_defects() {
        let r100 = encode_csetm(&[Operand::Reg("sp".to_string()), Operand::Cond("eq".to_string())]);
        let r103 = encode_csinc(&[Operand::Reg("x0".to_string()), Operand::Reg("w1".to_string()),
                                   Operand::Reg("x2".to_string()), Operand::Cond("eq".to_string())]);
        let r109 = encode_csneg(&[Operand::Reg("x0".to_string()), Operand::Reg("x0".to_string()),
                                   Operand::Reg("x0".to_string()), Operand::Cond("eq".to_string()),
                                   Operand::Reg("x3".to_string())]);
        println!("csetm sp,eq        -> {:?}  [#100]", r100);
        println!("csinc x0,w1,x2,eq  -> {:?}  [#103]", r103);
        println!("csneg x0,x0,x0,eq,x3 -> {:?}  [#109]", r109);
        assert!(r100.is_err(), "#100: SP must be Err for csetm (reg 31 = XZR here)");
        assert!(r103.is_err(), "#103: mixed x/w must be Err for csinc");
        assert!(r109.is_err(), "#109: fifth operand must be Err for csneg");
    }
}

#[cfg(test)]
mod scratch_csneg_sp {
    use super::*;
    use crate::backend::arm::assembler::parser::Operand;

    /// Issue #112: CSNEG register 31 is XZR/WZR; SP as Rd must be Err.
    #[test]
    fn manual_csneg_sp() {
        let r = encode_csneg(&[Operand::Reg("sp".to_string()), Operand::Reg("x0".to_string()),
                                Operand::Reg("x0".to_string()), Operand::Cond("eq".to_string())]);
        println!("csneg sp,x0,x0,eq -> {:?}  [#112]", r);
        assert!(r.is_err(), "#112: SP must be Err for csneg (reg 31 = XZR here)");
    }
}
