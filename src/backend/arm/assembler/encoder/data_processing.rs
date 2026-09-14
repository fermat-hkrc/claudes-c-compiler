use super::*;
use crate::backend::arm::assembler::parser::Operand;

// ── MOV ──────────────────────────────────────────────────────────────────

pub(crate) fn encode_mov(operands: &[Operand]) -> Result<EncodeResult, String> {
    if operands.len() < 2 {
        return Err("mov requires 2 operands".to_string());
    }

    // NEON register-to-register move: mov v1.16b, v0.16b -> ORR v1.16b, v0.16b, v0.16b
    if let (Some(Operand::RegArrangement { reg: rd_name, arrangement: arr_d }),
            Some(Operand::RegArrangement { reg: rm_name, arrangement: _arr_m })) =
        (operands.first(), operands.get(1))
    {
        let rd = parse_reg_num(rd_name).ok_or("invalid NEON rd")?;
        let rm = parse_reg_num(rm_name).ok_or("invalid NEON rm")?;
        let q: u32 = if arr_d == "16b" { 1 } else { 0 };
        // ORR Vd.T, Vm.T, Vm.T: 0 Q 0 01110 10 1 Rm 0 00111 Rn Rd
        let word = (q << 30) | (0b001110 << 24) | (0b10 << 22) | (1 << 21)
            | (rm << 16) | (0b000111 << 10) | (rm << 5) | rd;
        return Ok(EncodeResult::Word(word));
    }

    // NEON lane insert: mov v0.d[1], x1 -> INS Vd.D[index], Xn
    if let (Some(Operand::RegLane { reg: vd_name, elem_size, index }),
            Some(Operand::Reg(rn_name))) =
        (operands.first(), operands.get(1))
    {
        let vd = parse_reg_num(vd_name).ok_or("invalid NEON vd")?;
        let rn = parse_reg_num(rn_name).ok_or("invalid rn")?;
        // INS Vd.Ts[index], Rn
        // Encoding: 0 1 0 0 1110 000 imm5 0 0011 1 Rn Rd
        // imm5 encoding depends on element size and index
        let imm5 = match elem_size.as_str() {
            "b" => ((*index & 0xF) << 1) | 0b00001,
            "h" => ((*index & 0x7) << 2) | 0b00010,
            "s" => ((*index & 0x3) << 3) | 0b00100,
            "d" => ((*index & 0x1) << 4) | 0b01000,
            _ => return Err(format!("unsupported element size for ins: {}", elem_size)),
        };
        let word = (0b01001110000u32 << 21) | (imm5 << 16) | (0b000111 << 10) | (rn << 5) | vd;
        return Ok(EncodeResult::Word(word));
    }

    // NEON lane extract: mov x0, v0.d[1] -> UMOV Xd, Vn.D[index]
    if let (Some(Operand::Reg(rd_name)),
            Some(Operand::RegLane { reg: vn_name, elem_size, index })) =
        (operands.first(), operands.get(1))
    {
        let rd = parse_reg_num(rd_name).ok_or("invalid rd")?;
        let vn = parse_reg_num(vn_name).ok_or("invalid NEON vn")?;
        // UMOV Rd, Vn.Ts[index]
        // Encoding: 0 Q 0 0 1110 000 imm5 0 0111 1 Rn Rd
        let (q, imm5) = match elem_size.as_str() {
            "b" => (0u32, ((*index & 0xF) << 1) | 0b00001),
            "h" => (0, ((*index & 0x7) << 2) | 0b00010),
            "s" => (0, ((*index & 0x3) << 3) | 0b00100),
            "d" => (1, ((*index & 0x1) << 4) | 0b01000),
            _ => return Err(format!("unsupported element size for umov: {}", elem_size)),
        };
        let word = (q << 30) | (0b001110000u32 << 21) | (imm5 << 16) | (0b001111 << 10) | (vn << 5) | rd;
        return Ok(EncodeResult::Word(word));
    }

    // NEON element-to-element move: mov v0.s[3], v1.s[0] -> INS Vd.Ts[i1], Vn.Ts[i2]
    if let (Some(Operand::RegLane { reg: vd_name, elem_size: es_d, index: idx_d }),
            Some(Operand::RegLane { reg: vn_name, elem_size: _es_n, index: idx_n })) =
        (operands.first(), operands.get(1))
    {
        let vd = parse_reg_num(vd_name).ok_or("invalid NEON vd")?;
        let vn = parse_reg_num(vn_name).ok_or("invalid NEON vn")?;
        // INS Vd.Ts[i1], Vn.Ts[i2]
        // Encoding: 0 1 1 01110 000 imm5 0 imm4 1 Rn Rd
        let (imm5, imm4) = match es_d.as_str() {
            "b" => ((idx_d << 1) | 0b00001, *idx_n),
            "h" => ((idx_d << 2) | 0b00010, idx_n << 1),
            "s" => ((idx_d << 3) | 0b00100, idx_n << 2),
            "d" => ((idx_d << 4) | 0b01000, idx_n << 3),
            _ => return Err(format!("unsupported element size for ins: {}", es_d)),
        };
        let word = ((0b01101110000u32 << 21) | (imm5 << 16)) | (imm4 << 11) | (1 << 10) | (vn << 5) | vd;
        return Ok(EncodeResult::Word(word));
    }

    // mov Xd, #imm -> movz or movn
    if let Some(Operand::Imm(imm)) = operands.get(1) {
        let (rd, is_64) = get_reg(operands, 0)?;
        let imm = *imm;

        // Check if it can be a simple MOVZ
        if (0..=0xFFFF).contains(&imm) {
            let sf = sf_bit(is_64);
            let word = (sf << 31) | (0b10100101 << 23) | ((imm as u32 & 0xFFFF) << 5) | rd;
            return Ok(EncodeResult::Word(word));
        }

        // Negative: try MOVN
        if imm < 0 {
            let not_imm = !imm;
            if (0..=0xFFFF).contains(&not_imm) {
                let sf = sf_bit(is_64);
                let word = (sf << 31) | (0b00100101 << 23) | ((not_imm as u32 & 0xFFFF) << 5) | rd;
                return Ok(EncodeResult::Word(word));
            }
        }

        // Try encoding as ORR Rd, XZR, #imm (logical/bitmask immediate)
        // This handles patterns like 0x0101010101010101 in a single instruction
        if let Some((n, immr, imms)) = encode_bitmask_imm(imm as u64, is_64) {
            let sf = sf_bit(is_64);
            // ORR Rd, XZR, #imm: sf 01 100100 N immr imms 11111 Rd
            let word = (sf << 31) | (0b01 << 29) | (0b100100 << 23) | (n << 22) | (immr << 16) | (imms << 10) | (0b11111 << 5) | rd;
            return Ok(EncodeResult::Word(word));
        }

        // Need movz + movk sequence for large immediates
        return encode_mov_wide_imm(rd, is_64, imm as u64);
    }

    // mov Xd, Xm -> ORR Xd, XZR, Xm
    if let (Some(Operand::Reg(rd_name)), Some(Operand::Reg(rm_name))) = (operands.first(), operands.get(1)) {
        let rd = parse_reg_num(rd_name).ok_or("invalid rd")?;
        let rm = parse_reg_num(rm_name).ok_or("invalid rm")?;
        let is_64 = is_64bit_reg(rd_name);

        // Check for MOV to/from SP: uses ADD Xd, Xn, #0
        if rd_name.to_lowercase() == "sp" || rm_name.to_lowercase() == "sp" {
            let sf = sf_bit(is_64);
            // ADD Xd, Xn, #0: sf 0 0 10001 00 imm12=0 Rn Rd
            let word = ((sf << 31) | (0b10001 << 24)) | (rm << 5) | rd;
            return Ok(EncodeResult::Word(word));
        }

        let sf = sf_bit(is_64);
        // ORR Rd, XZR, Rm: sf 01 01010 00 0 Rm 000000 11111 Rd
        let word = ((sf << 31) | (0b01 << 29) | (0b01010 << 24)) | (rm << 16) | (0b11111 << 5) | rd;
        return Ok(EncodeResult::Word(word));
    }

    Err(format!("unsupported mov operands: {:?}", operands))
}

pub(crate) fn encode_mov_wide_imm(rd: u32, is_64: bool, imm: u64) -> Result<EncodeResult, String> {
    let sf = sf_bit(is_64);
    let mut words = Vec::new();
    let max_hw = if is_64 { 4 } else { 2 };
    let mut first = true;

    for hw in 0..max_hw {
        let chunk = ((imm >> (hw * 16)) & 0xFFFF) as u32;
        if chunk != 0 || (hw == 0 && imm == 0) {
            if first {
                // MOVZ
                let word = (sf << 31) | (0b10100101 << 23) | (hw << 21) | (chunk << 5) | rd;
                words.push(word);
                first = false;
            } else {
                // MOVK
                let word = (sf << 31) | (0b11100101 << 23) | (hw << 21) | (chunk << 5) | rd;
                words.push(word);
            }
        }
    }

    if words.is_empty() {
        // imm is 0
        let word = (sf << 31) | (0b10100101 << 23) | rd;
        words.push(word);
    }

    if words.len() == 1 {
        Ok(EncodeResult::Word(words[0]))
    } else {
        Ok(EncodeResult::Words(words))
    }
}

/// Resolve `:abs_g0:`, `:abs_g1:`, etc. modifiers for movz/movk.
/// If the expression is a pure constant, returns Some((imm16, hw)) where
/// imm16 is the relevant 16-bit chunk and hw is the halfword selector.
/// If the expression contains a symbol reference, returns None (needs relocation).
pub(crate) fn resolve_abs_g_modifier(kind: &str, symbol: &str) -> Result<Option<(u32, u32)>, String> {
    let shift = match kind {
        "abs_g0" | "abs_g0_nc" | "abs_g0_s" => 0,
        "abs_g1" | "abs_g1_nc" | "abs_g1_s" => 16,
        "abs_g2" | "abs_g2_nc" | "abs_g2_s" => 32,
        "abs_g3" => 48,
        _ => return Ok(None), // Not an abs_g modifier
    };
    let hw = shift / 16;
    // Try to evaluate the expression as a constant
    if let Ok(val) = crate::backend::asm_expr::parse_integer_expr(symbol) {
        let imm16 = ((val as u64) >> shift) as u32 & 0xFFFF;
        Ok(Some((imm16, hw)))
    } else {
        Ok(None) // Contains symbol reference - needs relocation
    }
}

pub(crate) fn encode_movz(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, is_64) = get_reg(operands, 0)?;
    let sf = sf_bit(is_64);

    // Handle :abs_g*: modifiers
    if let Some(Operand::Modifier { kind, symbol }) = operands.get(1) {
        if let Some((imm16, hw)) = resolve_abs_g_modifier(kind, symbol)? {
            let word = (sf << 31) | (0b10100101 << 23) | (hw << 21) | ((imm16 & 0xFFFF) << 5) | rd;
            return Ok(EncodeResult::Word(word));
        }
    }

    let imm = get_imm(operands, 1)?;

    // Check for lsl #N shift
    let hw = if operands.len() > 2 {
        if let Some(Operand::Shift { kind, amount }) = operands.get(2) {
            if kind == "lsl" {
                *amount / 16
            } else {
                0
            }
        } else {
            0
        }
    } else {
        0
    };

    let word = (sf << 31) | (0b10100101 << 23) | (hw << 21) | (((imm as u32) & 0xFFFF) << 5) | rd;
    Ok(EncodeResult::Word(word))
}

pub(crate) fn encode_movk(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, is_64) = get_reg(operands, 0)?;
    let sf = sf_bit(is_64);

    // Handle :abs_g*: modifiers
    if let Some(Operand::Modifier { kind, symbol }) = operands.get(1) {
        if let Some((imm16, hw)) = resolve_abs_g_modifier(kind, symbol)? {
            let word = (sf << 31) | (0b11100101 << 23) | (hw << 21) | ((imm16 & 0xFFFF) << 5) | rd;
            return Ok(EncodeResult::Word(word));
        }
    }

    let imm = get_imm(operands, 1)?;

    let hw = if operands.len() > 2 {
        if let Some(Operand::Shift { kind, amount }) = operands.get(2) {
            if kind == "lsl" {
                *amount / 16
            } else {
                0
            }
        } else {
            0
        }
    } else {
        0
    };

    let word = (sf << 31) | (0b11100101 << 23) | (hw << 21) | (((imm as u32) & 0xFFFF) << 5) | rd;
    Ok(EncodeResult::Word(word))
}

pub(crate) fn encode_movn(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, is_64) = get_reg(operands, 0)?;
    let imm = get_imm(operands, 1)?;
    let sf = sf_bit(is_64);

    let hw = if operands.len() > 2 {
        if let Some(Operand::Shift { kind, amount }) = operands.get(2) {
            if kind == "lsl" {
                *amount / 16
            } else {
                0
            }
        } else {
            0
        }
    } else {
        0
    };

    let word = (sf << 31) | (0b00100101 << 23) | (hw << 21) | (((imm as u32) & 0xFFFF) << 5) | rd;
    Ok(EncodeResult::Word(word))
}

// ── ADD/SUB ──────────────────────────────────────────────────────────────

pub(crate) fn encode_add_sub(operands: &[Operand], is_sub: bool, set_flags: bool) -> Result<EncodeResult, String> {
    if operands.len() < 3 {
        return Err(format!("add/sub requires 3 operands, got {}", operands.len()));
    }

    // NEON vector form: ADD/SUB Vd.T, Vn.T, Vm.T
    if let Some(Operand::RegArrangement { .. }) = operands.first() {
        if !set_flags {
            return encode_neon_add_sub(operands, is_sub);
        }
    }

    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let sf = sf_bit(is_64);
    let op = if is_sub { 1u32 } else { 0u32 };
    let s_bit = if set_flags { 1u32 } else { 0u32 };

    // ADD Rd, Rn, #imm
    if let Some(Operand::Imm(imm)) = operands.get(2) {
        let imm_signed = *imm;
        // Handle negative immediates: add #-N -> sub #N and vice versa
        let (imm_val, actual_op) = if imm_signed < 0 {
            ((-imm_signed) as u64, if is_sub { 0u32 } else { 1u32 })
        } else {
            (imm_signed as u64, op)
        };
        // Check for explicit lsl #12 shift
        let explicit_shift = if operands.len() > 3 {
            if let Some(Operand::Shift { kind, amount }) = operands.get(3) {
                kind == "lsl" && *amount == 12
            } else { false }
        } else { false };

        let (imm12, sh) = if explicit_shift {
            // Explicit lsl #12: use the immediate as-is (must fit in 12 bits)
            ((imm_val as u32) & 0xFFF, 1u32)
        } else if imm_val <= 0xFFF {
            // Fits in 12 bits unshifted
            (imm_val as u32, 0u32)
        } else if (imm_val & 0xFFF) == 0 && (imm_val >> 12) <= 0xFFF {
            // Low 12 bits are zero and shifted value fits: auto-shift
            // e.g., #4096 -> #1, lsl #12
            ((imm_val >> 12) as u32, 1u32)
        } else {
            return Err(format!("immediate {} does not fit in add/sub imm12 encoding", imm_val));
        };

        let word = (sf << 31) | (actual_op << 30) | (s_bit << 29) | (0b10001 << 24) | (sh << 22) | (imm12 << 10) | (rn << 5) | rd;
        return Ok(EncodeResult::Word(word));
    }

    // ADD Rd, Rn, :lo12:symbol
    if let Some(Operand::Modifier { kind, symbol }) = operands.get(2) {
        if kind == "lo12" {
            let word = ((sf << 31) | (op << 30) | (s_bit << 29) | (0b10001 << 24)) | (rn << 5) | rd;
            return Ok(EncodeResult::WordWithReloc {
                word,
                reloc: Relocation {
                    reloc_type: RelocType::AddAbsLo12,
                    symbol: symbol.clone(),
                    addend: 0,
                },
            });
        }
        if kind == "tprel_lo12_nc" {
            let word = ((sf << 31) | (op << 30) | (s_bit << 29) | (0b10001 << 24)) | (rn << 5) | rd;
            return Ok(EncodeResult::WordWithReloc {
                word,
                reloc: Relocation {
                    reloc_type: RelocType::TlsLeAddTprelLo12,
                    symbol: symbol.clone(),
                    addend: 0,
                },
            });
        }
        if kind == "tprel_hi12" {
            let word = ((sf << 31) | (op << 30) | (s_bit << 29) | (0b10001 << 24) | (1 << 22)) | (rn << 5) | rd;
            return Ok(EncodeResult::WordWithReloc {
                word,
                reloc: Relocation {
                    reloc_type: RelocType::TlsLeAddTprelHi12,
                    symbol: symbol.clone(),
                    addend: 0,
                },
            });
        }
    }
    if let Some(Operand::ModifierOffset { kind, symbol, offset }) = operands.get(2) {
        if kind == "lo12" {
            let word = ((sf << 31) | (op << 30) | (s_bit << 29) | (0b10001 << 24)) | (rn << 5) | rd;
            return Ok(EncodeResult::WordWithReloc {
                word,
                reloc: Relocation {
                    reloc_type: RelocType::AddAbsLo12,
                    symbol: symbol.clone(),
                    addend: *offset,
                },
            });
        }
    }

    // ADD Rd, Rn, Rm
    if let Some(Operand::Reg(rm_name)) = operands.get(2) {
        let rm = parse_reg_num(rm_name).ok_or("invalid rm")?;

        // Check for extended register: add Xd, Xn, Wm, sxtw [#N]
        if let Some(Operand::Extend { kind, amount }) = operands.get(3) {
            let option = match kind.as_str() {
                "uxtb" => 0b000u32,
                "uxth" => 0b001,
                "uxtw" => 0b010,
                "uxtx" => 0b011,
                "sxtb" => 0b100,
                "sxth" => 0b101,
                "sxtw" => 0b110,
                "sxtx" => 0b111,
                _ => 0b011, // default UXTX/LSL
            };
            let imm3 = *amount & 0x7;
            // Extended register form: sf op S 01011 00 1 Rm option imm3 Rn Rd
            let word = ((sf << 31) | (op << 30) | (s_bit << 29) | (0b01011 << 24)) | (1 << 21) | (rm << 16) | (option << 13) | (imm3 << 10) | (rn << 5) | rd;
            return Ok(EncodeResult::Word(word));
        }

        // When Rn or Rd is SP (register 31), the shifted register form encodes
        // register 31 as XZR, not SP. We must use the extended register form
        // with UXTX (option=0b011) to get SP semantics.
        let rn_is_sp = matches!(&operands[1], Operand::Reg(name) if {
            let n = name.to_lowercase(); n == "sp" || n == "wsp"
        });
        let rd_is_sp = matches!(&operands[0], Operand::Reg(name) if {
            let n = name.to_lowercase(); n == "sp" || n == "wsp"
        });

        if (rn_is_sp || rd_is_sp) && operands.len() <= 3 {
            // Extended register form with UXTX #0: sf op S 01011 00 1 Rm 011 000 Rn Rd
            let option = if is_64 { 0b011u32 } else { 0b010u32 }; // UXTX for 64-bit, UXTW for 32-bit
            let word = (((sf << 31) | (op << 30) | (s_bit << 29) | (0b01011 << 24)) | (1 << 21) | (rm << 16) | (option << 13)) | (rn << 5) | rd;
            return Ok(EncodeResult::Word(word));
        }

        // Check for shifted register: add Xd, Xn, Xm, lsl #N
        let (shift_type, shift_amount) = if let Some(Operand::Shift { kind, amount }) = operands.get(3) {
            let st = match kind.as_str() {
                "lsl" => 0b00u32,
                "lsr" => 0b01,
                "asr" => 0b10,
                _ => 0b00,
            };
            (st, *amount)
        } else {
            (0, 0)
        };

        let word = ((sf << 31) | (op << 30) | (s_bit << 29) | (0b01011 << 24) | (shift_type << 22)) | (rm << 16) | ((shift_amount & 0x3F) << 10) | (rn << 5) | rd;
        return Ok(EncodeResult::Word(word));
    }

    Err(format!("unsupported add/sub operands: {:?}", operands))
}

// ── Logical ──────────────────────────────────────────────────────────────

pub(crate) fn encode_logical(operands: &[Operand], opc: u32) -> Result<EncodeResult, String> {
    if operands.len() < 3 {
        return Err("logical op requires 3 operands".to_string());
    }

    // NEON vector form: ORR/AND/EOR Vd.T, Vn.T, Vm.T
    if let Some(Operand::RegArrangement { .. }) = operands.first() {
        return encode_neon_logical(operands, opc);
    }

    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let sf = sf_bit(is_64);

    // AND/ORR/EOR Rd, Rn, #imm (bitmask immediate)
    if let Some(Operand::Imm(imm)) = operands.get(2) {
        if let Some((n, immr, imms)) = encode_bitmask_imm(*imm as u64, is_64) {
            let word = (sf << 31) | (opc << 29) | (0b100100 << 23) | (n << 22) | (immr << 16) | (imms << 10) | (rn << 5) | rd;
            return Ok(EncodeResult::Word(word));
        }
        return Err(format!("cannot encode bitmask immediate: 0x{:x}", imm));
    }

    // AND/ORR/EOR Rd, Rn, Rm [, shift #amount]
    if let Some(Operand::Reg(rm_name)) = operands.get(2) {
        let rm = parse_reg_num(rm_name).ok_or("invalid rm")?;

        let (shift_type, shift_amount) = if let Some(Operand::Shift { kind, amount }) = operands.get(3) {
            let st = match kind.as_str() {
                "lsl" => 0b00u32,
                "lsr" => 0b01,
                "asr" => 0b10,
                "ror" => 0b11,
                _ => 0b00,
            };
            (st, *amount)
        } else {
            (0, 0)
        };

        let word = ((sf << 31) | (opc << 29) | (0b01010 << 24) | (shift_type << 22))
            | (rm << 16) | ((shift_amount & 0x3F) << 10) | (rn << 5) | rd;
        return Ok(EncodeResult::Word(word));
    }

    Err("unsupported logical operands".to_string())
}

/// Encode a bitmask immediate for AArch64.
/// Returns (N, immr, imms) if the value is a valid bitmask immediate.
pub(crate) fn encode_bitmask_imm(val: u64, is_64: bool) -> Option<(u32, u32, u32)> {
    if val == 0 || (!is_64 && val == 0xFFFFFFFF) || (is_64 && val == u64::MAX) {
        return None; // Not a valid bitmask immediate
    }

    let width = if is_64 { 64 } else { 32 };
    let val = if !is_64 { val & 0xFFFFFFFF } else { val };

    // Try each possible element size: 2, 4, 8, 16, 32, 64
    for size in [2u32, 4, 8, 16, 32, 64] {
        if size > width {
            continue;
        }

        let mask = if size == 64 { u64::MAX } else { (1u64 << size) - 1 };
        let elem = val & mask;

        // Check that the pattern repeats
        let mut repeats = true;
        let mut pos = size;
        while pos < width {
            if ((val >> pos) & mask) != elem {
                repeats = false;
                break;
            }
            pos += size;
        }
        if !repeats {
            continue;
        }

        // Check that elem is a contiguous run of 1s (possibly rotated)
        let ones = elem.count_ones();
        if ones == 0 || ones == size {
            continue; // All zeros or all ones in element
        }

        // Find rotation: rotate elem right until the least significant bit is 1
        // and the run of 1s starts at bit 0.
        // The `r` we find is the right-rotation from actual -> base.
        // immr is the right-rotation from base -> actual = size - r (mod size).
        let mut found_rotation = false;
        let mut rotation = 0u32;
        for r in 0..size {
            let rot = if r == 0 { elem } else { ((elem >> r) | (elem << (size - r))) & mask };
            // Check if this is a contiguous run from bit 0
            let run = rot.trailing_ones();
            if run == ones {
                // r rotates actual -> base, so immr = size - r (mod size) rotates base -> actual
                rotation = if r == 0 { 0 } else { size - r };
                found_rotation = true;
                break;
            }
        }
        if !found_rotation {
            continue;
        }

        // Encode the fields
        let n = if size == 64 { 1u32 } else { 0u32 };
        let immr = rotation;
        let imms = match size {
            2 => 0b111100 | (ones - 1),
            4 => 0b111000 | (ones - 1),
            8 => 0b110000 | (ones - 1),
            16 => 0b100000 | (ones - 1),
            32 => ones - 1,
            64 => ones - 1,
            _ => unreachable!(),
        };

        return Some((n, immr, imms));
    }

    None
}

// ── MUL/DIV ──────────────────────────────────────────────────────────────

pub(crate) fn encode_mul(operands: &[Operand]) -> Result<EncodeResult, String> {
    // NEON vector form: MUL Vd.T, Vn.T, Vm.T
    if let Some(Operand::RegArrangement { .. }) = operands.first() {
        return encode_neon_mul(operands);
    }
    // MUL Rd, Rn, Rm is MADD Rd, Rn, Rm, XZR
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
    let sf = sf_bit(is_64);
    let word = (sf << 31) | (0b0011011000 << 21) | (rm << 16) | (0b11111 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

pub(crate) fn encode_madd(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
    let (ra, _) = get_reg(operands, 3)?;
    let sf = sf_bit(is_64);
    let word = ((sf << 31) | (0b0011011000 << 21) | (rm << 16)) | (ra << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

pub(crate) fn encode_msub(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
    let (ra, _) = get_reg(operands, 3)?;
    let sf = sf_bit(is_64);
    let word = (sf << 31) | (0b0011011000 << 21) | (rm << 16) | (1 << 15) | (ra << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

pub(crate) fn encode_div(operands: &[Operand], unsigned: bool) -> Result<EncodeResult, String> {
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
    let sf = sf_bit(is_64);
    let o1 = if unsigned { 0u32 } else { 1u32 };
    // Data-processing (2 source): sf 0 S=0 11010110 Rm 00001 o1 Rn Rd
    let word = (sf << 31) | (0b0011010110 << 21) | (rm << 16)
        | (0b00001 << 11) | (o1 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode SMULL Xd, Wn, Wm -> SMADDL Xd, Wn, Wm, XZR
pub(crate) fn encode_smull(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, _) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
    // SMADDL: 1 00 11011 001 Rm 0 11111 Rn Rd (Ra=XZR makes it SMULL)
    let word = (1u32 << 31) | (0b0011011001 << 21) | (rm << 16)
        | (0b011111 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode UMULL Xd, Wn, Wm -> UMADDL Xd, Wn, Wm, XZR
pub(crate) fn encode_umull(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, _) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
    // UMADDL: 1 00 11011 101 Rm 0 11111 Rn Rd (Ra=XZR makes it UMULL)
    let word = (1u32 << 31) | (0b0011011101 << 21) | (rm << 16)
        | (0b011111 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode SMADDL Xd, Wn, Wm, Xa (signed multiply-add long)
pub(crate) fn encode_smaddl(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, _) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
    let (ra, _) = get_reg(operands, 3)?;
    // SMADDL: 1 00 11011 001 Rm 0 Ra Rn Rd
    let word = (1u32 << 31) | (0b0011011001 << 21) | (rm << 16)
        | (ra << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode UMADDL Xd, Wn, Wm, Xa (unsigned multiply-add long)
pub(crate) fn encode_umaddl(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, _) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
    let (ra, _) = get_reg(operands, 3)?;
    // UMADDL: 1 00 11011 101 Rm 0 Ra Rn Rd
    let word = (1u32 << 31) | (0b0011011101 << 21) | (rm << 16)
        | (ra << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode MNEG Xd, Xn, Xm -> MSUB Xd, Xn, Xm, XZR
pub(crate) fn encode_mneg(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
    let sf = sf_bit(is_64);
    // MSUB with Ra=XZR: sf 00 11011 000 Rm 1 11111 Rn Rd
    let word = (sf << 31) | (0b0011011000 << 21) | (rm << 16)
        | (1 << 15) | (0b11111 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

pub(crate) fn encode_umulh(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, _) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
    // UMULH: 1 00 11011 1 10 Rm 0 11111 Rn Rd
    let word = (1u32 << 31) | (0b0011011110 << 21) | (rm << 16) | (0b011111 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

pub(crate) fn encode_smulh(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, _) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
    // SMULH: 1 00 11011 0 10 Rm 0 11111 Rn Rd
    let word = (1u32 << 31) | (0b0011011010 << 21) | (rm << 16) | (0b011111 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

pub(crate) fn encode_neg(operands: &[Operand]) -> Result<EncodeResult, String> {
    // NEG Rd, Rm [, shift #amount] -> SUB Rd, XZR, Rm [, shift #amount]
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rm, _) = get_reg(operands, 1)?;
    let sf = sf_bit(is_64);
    let (shift_type, shift_amount) = if let Some(Operand::Shift { kind, amount }) = operands.get(2) {
        let st = match kind.as_str() {
            "lsl" => 0b00u32,
            "lsr" => 0b01,
            "asr" => 0b10,
            _ => 0b00,
        };
        (st, *amount)
    } else {
        (0, 0)
    };
    let word = (sf << 31) | (1 << 30) | (0b01011 << 24) | (shift_type << 22)
        | (rm << 16) | ((shift_amount & 0x3F) << 10) | (0b11111 << 5) | rd;
    Ok(EncodeResult::Word(word))
}

pub(crate) fn encode_negs(operands: &[Operand]) -> Result<EncodeResult, String> {
    // NEGS Rd, Rm [, shift #amount] -> SUBS Rd, XZR, Rm [, shift #amount]
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rm, _) = get_reg(operands, 1)?;
    let sf = sf_bit(is_64);
    let (shift_type, shift_amount) = if let Some(Operand::Shift { kind, amount }) = operands.get(2) {
        let st = match kind.as_str() {
            "lsl" => 0b00u32,
            "lsr" => 0b01,
            "asr" => 0b10,
            _ => 0b00,
        };
        (st, *amount)
    } else {
        (0, 0)
    };
    let word = (sf << 31) | (1 << 30) | (1 << 29) | (0b01011 << 24) | (shift_type << 22)
        | (rm << 16) | ((shift_amount & 0x3F) << 10) | (0b11111 << 5) | rd;
    Ok(EncodeResult::Word(word))
}

pub(crate) fn encode_mvn(operands: &[Operand]) -> Result<EncodeResult, String> {
    // NEON vector form: MVN Vd.T, Vn.T (alias of NOT)
    if let Some(Operand::RegArrangement { .. }) = operands.first() {
        return encode_neon_not(operands);
    }
    // MVN Rd, Rm [, shift #amount] -> ORN Rd, XZR, Rm [, shift #amount]
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rm, _) = get_reg(operands, 1)?;
    let sf = sf_bit(is_64);
    let (shift_type, shift_amount) = if let Some(Operand::Shift { kind, amount }) = operands.get(2) {
        let st = match kind.as_str() {
            "lsl" => 0b00u32,
            "lsr" => 0b01,
            "asr" => 0b10,
            "ror" => 0b11,
            _ => 0b00,
        };
        (st, *amount)
    } else {
        (0, 0)
    };
    let word = (sf << 31) | (0b01 << 29) | (0b01010 << 24) | (shift_type << 22) | (1 << 21)
        | (rm << 16) | ((shift_amount & 0x3F) << 10) | (0b11111 << 5) | rd;
    Ok(EncodeResult::Word(word))
}

pub(crate) fn encode_adc(operands: &[Operand], set_flags: bool) -> Result<EncodeResult, String> {
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
    let sf = sf_bit(is_64);
    let s = if set_flags { 1u32 } else { 0 };
    let word = ((sf << 31) | (s << 29) | (0b11010000 << 21) | (rm << 16)) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

pub(crate) fn encode_sbc(operands: &[Operand], set_flags: bool) -> Result<EncodeResult, String> {
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
    let sf = sf_bit(is_64);
    let s = if set_flags { 1u32 } else { 0 };
    let word = ((sf << 31) | (1 << 30) | (s << 29) | (0b11010000 << 21) | (rm << 16)) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

// ── Shifts ───────────────────────────────────────────────────────────────

pub(crate) fn encode_shift(operands: &[Operand], shift_type: u32) -> Result<EncodeResult, String> {
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;

    // LSL/LSR/ASR Rd, Rn, #imm (immediate form -> UBFM/SBFM)
    if let Some(Operand::Imm(imm)) = operands.get(2) {
        let sf = sf_bit(is_64);
        let imm = *imm as u32;
        let width = if is_64 { 64 } else { 32 };
        let n = if is_64 { 1u32 } else { 0u32 };

        match shift_type {
            0b00 => {
                // LSL #imm -> UBFM Rd, Rn, #(-imm mod width), #(width-1-imm)
                let immr = (width - imm) % width;
                let imms = width - 1 - imm;
                let word = (sf << 31) | (0b10 << 29) | (0b100110 << 23) | (n << 22) | (immr << 16) | (imms << 10) | (rn << 5) | rd;
                return Ok(EncodeResult::Word(word));
            }
            0b01 => {
                // LSR #imm -> UBFM Rd, Rn, #imm, #(width-1)
                let immr = imm;
                let imms = width - 1;
                let word = (sf << 31) | (0b10 << 29) | (0b100110 << 23) | (n << 22) | (immr << 16) | (imms << 10) | (rn << 5) | rd;
                return Ok(EncodeResult::Word(word));
            }
            0b10 => {
                // ASR #imm -> SBFM Rd, Rn, #imm, #(width-1)
                let immr = imm;
                let imms = width - 1;
                let word = (sf << 31) | (0b100110 << 23) | (n << 22) | (immr << 16) | (imms << 10) | (rn << 5) | rd;
                return Ok(EncodeResult::Word(word));
            }
            0b11 => {
                // ROR #imm -> EXTR Rd, Rn, Rn, #imm
                // EXTR: sf 0 0 100111 N 0 Rm imms Rn Rd
                let word = (sf << 31) | (0b00100111 << 23) | (n << 22) | (rn << 16)
                    | (imm << 10) | (rn << 5) | rd;
                return Ok(EncodeResult::Word(word));
            }
            _ => {}
        }
    }

    // LSL/LSR/ASR Rd, Rn, Rm (register form)
    if let Some(Operand::Reg(rm_name)) = operands.get(2) {
        let rm = parse_reg_num(rm_name).ok_or("invalid rm")?;
        let sf = sf_bit(is_64);
        // Data-processing (2 source): sf 0 S=0 11010110 Rm 0010 op2 Rn Rd
        let op2 = shift_type; // 00=LSL, 01=LSR, 10=ASR, 11=ROR
        let word = (sf << 31) | (0b0011010110 << 21) | (rm << 16) | (0b0010 << 12) | (op2 << 10) | (rn << 5) | rd;
        return Ok(EncodeResult::Word(word));
    }

    Err("unsupported shift operands".to_string())
}

// ── Extensions ───────────────────────────────────────────────────────────

pub(crate) fn encode_sxtw(operands: &[Operand]) -> Result<EncodeResult, String> {
    // SXTW Xd, Wn -> SBFM Xd, Xn, #0, #31
    let (rd, _) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let word = ((1u32 << 31) | (0b100110 << 23) | (1 << 22)) | (31 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

pub(crate) fn encode_sxth(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let sf = sf_bit(is_64);
    let n = if is_64 { 1u32 } else { 0 };
    let word = ((sf << 31) | (0b100110 << 23) | (n << 22)) | (15 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

pub(crate) fn encode_sxtb(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let sf = sf_bit(is_64);
    let n = if is_64 { 1u32 } else { 0 };
    let word = ((sf << 31) | (0b100110 << 23) | (n << 22)) | (7 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

pub(crate) fn encode_uxtw(operands: &[Operand]) -> Result<EncodeResult, String> {
    // UXTW is MOV Wd, Wn (the upper 32 bits are zeroed)
    // Or: UBFM Xd, Xn, #0, #31
    let (rd, _) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    // Use 32-bit ORR (MOV alias)
    let word = (0b001010100 << 23) | (rn << 16) | (0b11111 << 5) | rd;
    Ok(EncodeResult::Word(word))
}

pub(crate) fn encode_uxth(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let sf = sf_bit(is_64);
    let n = if is_64 { 1u32 } else { 0 };
    let word = ((sf << 31) | (0b10 << 29) | (0b100110 << 23) | (n << 22)) | (15 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

pub(crate) fn encode_uxtb(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let sf = sf_bit(is_64);
    let n = if is_64 { 1u32 } else { 0 };
    let word = ((sf << 31) | (0b10 << 29) | (0b100110 << 23) | (n << 22)) | (7 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode ORN (logical OR NOT): ORN Rd, Rn, Rm (scalar or vector)
pub(crate) fn encode_orn(operands: &[Operand]) -> Result<EncodeResult, String> {
    if operands.len() < 3 {
        return Err("orn requires 3 operands".to_string());
    }

    // NEON vector form: ORN Vd.T, Vn.T, Vm.T
    if let Some(Operand::RegArrangement { .. }) = operands.first() {
        let (rd, arr_d) = get_neon_reg(operands, 0)?;
        let (rn, _) = get_neon_reg(operands, 1)?;
        let (rm, _) = get_neon_reg(operands, 2)?;
        let q: u32 = if arr_d == "16b" { 1 } else { 0 };
        // ORN Vd.T, Vn.T, Vm.T: 0 Q 0 01110 11 1 Rm 000111 Rn Rd
        let word = (q << 30) | (0b001110 << 24) | (0b11 << 22) | (1 << 21)
            | (rm << 16) | (0b000111 << 10) | (rn << 5) | rd;
        return Ok(EncodeResult::Word(word));
    }

    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
    let sf = sf_bit(is_64);

    let (shift_type, shift_amount) = if let Some(Operand::Shift { kind, amount }) = operands.get(3) {
        let st = match kind.as_str() {
            "lsl" => 0b00u32,
            "lsr" => 0b01,
            "asr" => 0b10,
            "ror" => 0b11,
            _ => 0b00,
        };
        (st, *amount)
    } else {
        (0, 0)
    };

    // ORN Rd, Rn, Rm [, shift #amount]: sf 01 01010 shift 1 Rm imm6 Rn Rd
    let word = (sf << 31) | (0b01 << 29) | (0b01010 << 24) | (shift_type << 22) | (1 << 21)
        | (rm << 16) | ((shift_amount & 0x3F) << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode EON (exclusive OR NOT): EON Rd, Rn, Rm [, shift #amount]
pub(crate) fn encode_eon(operands: &[Operand]) -> Result<EncodeResult, String> {
    if operands.len() < 3 {
        return Err("eon requires 3 operands".to_string());
    }
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
    let sf = sf_bit(is_64);

    let (shift_type, shift_amount) = if let Some(Operand::Shift { kind, amount }) = operands.get(3) {
        let st = match kind.as_str() {
            "lsl" => 0b00u32,
            "lsr" => 0b01,
            "asr" => 0b10,
            "ror" => 0b11,
            _ => 0b00,
        };
        (st, *amount)
    } else {
        (0, 0)
    };

    // EON Rd, Rn, Rm [, shift #amount]: sf 10 01010 shift 1 Rm imm6 Rn Rd (opc=10, N=1)
    let word = (sf << 31) | (0b10 << 29) | (0b01010 << 24) | (shift_type << 22) | (1 << 21)
        | (rm << 16) | ((shift_amount & 0x3F) << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode BICS (bitwise clear, setting flags): BICS Rd, Rn, Rm [, shift #amount]
pub(crate) fn encode_bics(operands: &[Operand]) -> Result<EncodeResult, String> {
    if operands.len() < 3 {
        return Err("bics requires 3 operands".to_string());
    }
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
    let sf = sf_bit(is_64);

    let (shift_type, shift_amount) = if let Some(Operand::Shift { kind, amount }) = operands.get(3) {
        let st = match kind.as_str() {
            "lsl" => 0b00u32,
            "lsr" => 0b01,
            "asr" => 0b10,
            "ror" => 0b11,
            _ => 0b00,
        };
        (st, *amount)
    } else {
        (0, 0)
    };

    // BICS Rd, Rn, Rm [, shift #amount]: sf 11 01010 shift 1 Rm imm6 Rn Rd
    let word = (sf << 31) | (0b11 << 29) | (0b01010 << 24) | (shift_type << 22) | (1 << 21)
        | (rm << 16) | ((shift_amount & 0x3F) << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode BIC instruction - disambiguates between scalar and NEON forms.
/// Scalar register: BIC Xd, Xn, Xm [, shift #amount] -> AND NOT (opc=00, N=1)
/// Scalar immediate: BIC Xd, Xn, #imm -> AND Xd, Xn, #~imm
/// NEON vector: BIC Vd.T, Vn.T, Vm.T
pub(crate) fn encode_bic(operands: &[Operand]) -> Result<EncodeResult, String> {
    if operands.len() < 3 {
        return Err("bic requires 3 operands".to_string());
    }

    // NEON vector form: BIC Vd.T, Vn.T, Vm.T
    if let Some(Operand::RegArrangement { .. }) = operands.first() {
        return encode_neon_bic(operands);
    }

    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let sf = sf_bit(is_64);

    // BIC Xd, Xn, #imm -> AND Xd, Xn, #~imm (bitmask immediate, inverted)
    if let Some(Operand::Imm(imm)) = operands.get(2) {
        let inverted = if is_64 {
            !(*imm as u64)
        } else {
            (!(*imm as u32)) as u64
        };
        if let Some((n, immr, imms)) = encode_bitmask_imm(inverted, is_64) {
            // AND Rd, Rn, #~imm: sf 00 100100 N immr imms Rn Rd
            // AND Rd, Rn, #~imm encoding: sf=bit31, opc=00 (bits29:30), 100100 (bits23:28), N, immr, imms, Rn, Rd
            let word = (sf << 31) | (0b100100 << 23) | (n << 22) | (immr << 16) | (imms << 10) | (rn << 5) | rd;
            return Ok(EncodeResult::Word(word));
        }
        return Err(format!("cannot encode bitmask immediate for bic: 0x{:x} (inverted: 0x{:x})", imm, inverted));
    }

    // BIC Xd, Xn, Xm [, shift #amount]: sf 00 01010 shift 1 Rm imm6 Rn Rd (N=1)
    if let Some(Operand::Reg(rm_name)) = operands.get(2) {
        let rm = parse_reg_num(rm_name).ok_or("invalid rm register for bic")?;

        let (shift_type, shift_amount) = if let Some(Operand::Shift { kind, amount }) = operands.get(3) {
            let st = match kind.as_str() {
                "lsl" => 0b00u32,
                "lsr" => 0b01,
                "asr" => 0b10,
                "ror" => 0b11,
                _ => 0b00,
            };
            (st, *amount)
        } else {
            (0, 0)
        };

        // BIC is AND with N=1 (bit 21): sf opc=00(bits29:30) 01010 shift 1 Rm imm6 Rn Rd
        let word = (sf << 31) | (0b01010 << 24) | (shift_type << 22) | (1 << 21)
            | (rm << 16) | ((shift_amount & 0x3F) << 10) | (rn << 5) | rd;
        return Ok(EncodeResult::Word(word));
    }

    Err("unsupported bic operands".to_string())
}

#[cfg(test)]
mod encode_add_sub_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler
    // Evidence: src/backend/arm/assembler/README.md "accepts the same textual assembly that GCC's gas would consume";
    //   encoder/mod.rs "Encodes AArch64 instructions into 32-bit machine code words";
    //   DESIGN_DOC.md "AArch64 | ARM assembly syntax | Fixed 32-bit encoding | imm12 auto-shift"
    // Stronger considered:
    //   - State machine: rejected — encode_add_sub is a pure function with no lifecycle
    //   - Algebraic round-trip: rejected — no in-tree ADD/SUB decoder
    // Weaker available: algebraic.metamorphic (neg-imm swap), negative_error (operand count / range)
    // Differential: candidate=encode_add_sub, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler, mapping=(operands,is_sub,set_flags)<->asm text

    use super::encode_add_sub;
    use super::super::EncodeResult;
    use crate::backend::arm::assembler::parser::Operand;
    use proptest::prelude::*;
    use std::io::Write;
    use std::process::{Command, Stdio};

    const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
    const CASES: u32 = 1000;

    fn cfg() -> ProptestConfig {
        ProptestConfig::with_cases(CASES)
    }

    fn mnemonic(is_sub: bool, set_flags: bool) -> &'static str {
        match (is_sub, set_flags) {
            (false, false) => "add",
            (false, true) => "adds",
            (true, false) => "sub",
            (true, true) => "subs",
        }
    }

    fn gpr(is_64: bool, n: u32, as_sp: bool) -> String {
        if n == 31 {
            if as_sp {
                if is_64 { "sp".into() } else { "wsp".into() }
            } else if is_64 {
                "xzr".into()
            } else {
                "wzr".into()
            }
        } else {
            format!("{}{}", if is_64 { "x" } else { "w" }, n)
        }
    }

    fn sut_word(ops: &[Operand], is_sub: bool, set_flags: bool) -> Result<u32, String> {
        match encode_add_sub(ops, is_sub, set_flags)? {
            EncodeResult::Word(w) => Ok(w),
            other => Err(format!("expected Word, got {:?}", other)),
        }
    }

    fn parse_llvm_encoding(stdout: &str) -> Result<u32, String> {
        let marker = "encoding: [";
        let start = stdout
            .find(marker)
            .ok_or_else(|| format!("no encoding in stdout: {stdout}"))?;
        let rest = &stdout[start + marker.len()..];
        let end = rest.find(']').ok_or_else(|| format!("no closing bracket: {stdout}"))?;
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
            stdin.write_all(b"\n").map_err(|e| format!("write llvm-mc: {e}"))?;
        }
        let out = child.wait_with_output().map_err(|e| format!("wait llvm-mc: {e}"))?;
        let stdout = String::from_utf8_lossy(&out.stdout);
        let stderr = String::from_utf8_lossy(&out.stderr);
        if !out.status.success() || stderr.contains("error:") {
            return Err(format!("llvm-mc error: {stderr}"));
        }
        parse_llvm_encoding(&stdout)
    }

    fn is_valid_imm12_magnitude(mag: u64) -> bool {
        mag <= 0xFFF || ((mag & 0xFFF) == 0 && (mag >> 12) <= 0xFFF)
    }

    fn is_valid_signed_imm12(imm: i64) -> bool {
        if imm == i64::MIN {
            return false;
        }
        let mag = if imm < 0 { (-imm) as u64 } else { imm as u64 };
        is_valid_imm12_magnitude(mag)
    }

    fn imm12_unshifted() -> impl Strategy<Value = i64> {
        prop_oneof![
            Just(0i64),
            Just(1i64),
            Just(0xFFFi64),
            0i64..=0xFFF,
        ]
    }

    fn imm12_autoshift() -> impl Strategy<Value = i64> {
        prop_oneof![
            Just(4096i64),
            Just(0xFFF000i64),
            (1i64..=0xFFF).prop_map(|k| k << 12),
        ]
    }

    // (imm, explicit_lsl12)
    fn valid_imm_form() -> impl Strategy<Value = (i64, bool)> {
        prop_oneof![
            imm12_unshifted().prop_map(|i| (i, false)),
            imm12_unshifted().prop_map(|i| (-i, false)),
            imm12_autoshift().prop_map(|i| (i, false)),
            imm12_autoshift().prop_map(|i| (-i, false)),
            imm12_unshifted().prop_map(|i| (i, true)),
        ]
    }

    fn invalid_imm() -> impl Strategy<Value = i64> {
        prop_oneof![
            Just(4097i64),
            Just(-4097i64),
            Just(0x1001i64),
            Just(0xFFF001i64),
            Just(0x1000000i64),
            Just(i64::MIN),
            Just(i64::MAX),
            (0x1001i64..=0x1F_FFFFi64).prop_filter("not a valid auto-shift", |x| {
                !is_valid_signed_imm12(*x)
            }),
        ]
    }

    fn shift_kind() -> impl Strategy<Value = &'static str> {
        prop::sample::select(vec!["lsl", "lsr", "asr"])
    }

    fn extend_kind() -> impl Strategy<Value = &'static str> {
        prop::sample::select(vec![
            "uxtb", "uxth", "uxtw", "uxtx", "sxtb", "sxth", "sxtw", "sxtx",
        ])
    }

    fn neon_arr() -> impl Strategy<Value = &'static str> {
        prop::sample::select(vec!["8b", "16b", "4h", "8h", "2s", "4s", "2d"])
    }

    fn rm_width_64(dn_is_64: bool, ext: &str) -> bool {
        dn_is_64 && matches!(ext, "uxtx" | "sxtx" | "lsl")
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_add_sub_kat_llvm_mc_add_imm42() {
        let want = 0x9100a820u32;
        let mc = llvm_mc_word("add x0, x1, #42").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("x1".into()),
            Operand::Imm(42),
        ];
        let sut = sut_word(&ops, false, false).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    proptest! {
        #![proptest_config(cfg())]

        #[test]
        fn encode_add_sub_diff_imm(
            rd in 0u32..=31,
            rn in 0u32..=31,
            is_64 in any::<bool>(),
            is_sub in any::<bool>(),
            set_flags in any::<bool>(),
            (imm, sh12) in valid_imm_form(),
            prefer_sp in any::<bool>(),
        ) {
            // Immediate form: register 31 is SP/WSP, never XZR/WZR (llvm-mc rejects add Rd, XZR, #imm).
            let rd_sp = rd == 31 && !set_flags;
            let rn_sp = rn == 31;
            let _ = prefer_sp;
            let rd_n = gpr(is_64, rd, rd_sp);
            let rn_n = gpr(is_64, rn, rn_sp);
            let mut ops = vec![
                Operand::Reg(rd_n.clone()),
                Operand::Reg(rn_n.clone()),
                Operand::Imm(imm),
            ];
            let mut asm = format!("{} {}, {}, #{}", mnemonic(is_sub, set_flags), rd_n, rn_n, imm);
            if sh12 {
                ops.push(Operand::Shift { kind: "lsl".into(), amount: 12 });
                asm.push_str(", lsl #12");
            }
            let mc = llvm_mc_word(&asm).unwrap_or_else(|e| panic!("llvm-mc rejected valid {asm}: {e}"));
            let sut = sut_word(&ops, is_sub, set_flags)
                .unwrap_or_else(|e| panic!("SUT rejected valid {asm}: {e}"));
            prop_assert_eq!(sut, mc, "asm={}", asm);
        }

        #[test]
        fn encode_add_sub_diff_shifted_reg(
            rd in 0u32..=30,
            rn in 0u32..=30,
            rm in 0u32..=31,
            is_64 in any::<bool>(),
            is_sub in any::<bool>(),
            set_flags in any::<bool>(),
            kind in shift_kind(),
            amt in 0u32..=63,
        ) {
            let max_amt = if is_64 { 63u32 } else { 31u32 };
            prop_assume!(amt <= max_amt);
            let rd_n = gpr(is_64, rd, false);
            let rn_n = gpr(is_64, rn, false);
            let rm_n = gpr(is_64, rm, false);
            let ops = vec![
                Operand::Reg(rd_n.clone()),
                Operand::Reg(rn_n.clone()),
                Operand::Reg(rm_n.clone()),
                Operand::Shift { kind: kind.to_string(), amount: amt },
            ];
            let asm = format!(
                "{} {}, {}, {}, {} #{}",
                mnemonic(is_sub, set_flags), rd_n, rn_n, rm_n, kind, amt
            );
            let mc = llvm_mc_word(&asm).unwrap_or_else(|e| panic!("llvm-mc rejected valid {asm}: {e}"));
            let sut = sut_word(&ops, is_sub, set_flags)
                .unwrap_or_else(|e| panic!("SUT rejected valid {asm}: {e}"));
            prop_assert_eq!(sut, mc, "asm={}", asm);
        }

        #[test]
        fn encode_add_sub_diff_extended_and_sp(
            rd in 0u32..=31,
            rn in 0u32..=31,
            rm in 0u32..=30,
            is_64 in any::<bool>(),
            is_sub in any::<bool>(),
            set_flags in any::<bool>(),
            ext in extend_kind(),
            amt in 0u32..=4,
            prefer_sp_rd in any::<bool>(),
            prefer_sp_rn in any::<bool>(),
            use_lsl_alias in any::<bool>(),
        ) {
            let rd_sp = prefer_sp_rd && rd == 31 && !set_flags;
            let rn_sp = prefer_sp_rn && rn == 31;
            // Force at least one SP when using the LSL/UXTX alias path so we hit SP encoding.
            let (rd_sp, rn_sp, ext, rm_64) = if use_lsl_alias {
                let rn_sp = true;
                let rn = 31u32;
                let _ = rn;
                (false, true, "lsl", is_64)
            } else {
                (rd_sp, rn_sp, ext, rm_width_64(is_64, ext))
            };
            let rd_n = if use_lsl_alias {
                gpr(is_64, rd.min(30), false)
            } else {
                gpr(is_64, rd, rd_sp)
            };
            let rn_n = if use_lsl_alias {
                gpr(is_64, 31, true)
            } else {
                gpr(is_64, rn, rn_sp)
            };
            let rm_n = gpr(rm_64, rm, false);
            let mut ops = vec![
                Operand::Reg(rd_n.clone()),
                Operand::Reg(rn_n.clone()),
                Operand::Reg(rm_n.clone()),
            ];
            let mut asm = format!("{} {}, {}, {}", mnemonic(is_sub, set_flags), rd_n, rn_n, rm_n);
            if ext == "lsl" {
                if amt > 0 {
                    ops.push(Operand::Shift { kind: "lsl".into(), amount: amt });
                    asm.push_str(&format!(", lsl #{amt}"));
                }
            } else {
                ops.push(Operand::Extend { kind: ext.to_string(), amount: amt });
                if amt > 0 {
                    asm.push_str(&format!(", {ext} #{amt}"));
                } else {
                    asm.push_str(&format!(", {ext}"));
                }
            }
            let mc = match llvm_mc_word(&asm) {
                Ok(w) => w,
                Err(_) => return Ok(()), // skip combinations llvm-mc rejects (e.g. odd extend/width)
            };
            let sut = sut_word(&ops, is_sub, set_flags)
                .unwrap_or_else(|e| panic!("SUT rejected {asm} which llvm-mc accepted as {mc:#010x}: {e}"));
            prop_assert_eq!(sut, mc, "asm={}", asm);
        }

        #[test]
        fn encode_add_sub_diff_neon(
            vd in 0u32..=31,
            vn in 0u32..=31,
            vm in 0u32..=31,
            arr in neon_arr(),
            is_sub in any::<bool>(),
        ) {
            let rd = format!("v{vd}");
            let rn = format!("v{vn}");
            let rm = format!("v{vm}");
            let ops = vec![
                Operand::RegArrangement { reg: rd.clone(), arrangement: arr.to_string() },
                Operand::RegArrangement { reg: rn.clone(), arrangement: arr.to_string() },
                Operand::RegArrangement { reg: rm.clone(), arrangement: arr.to_string() },
            ];
            let mn = if is_sub { "sub" } else { "add" };
            let asm = format!("{mn} {rd}.{arr}, {rn}.{arr}, {rm}.{arr}");
            let mc = llvm_mc_word(&asm).unwrap_or_else(|e| panic!("llvm-mc rejected valid {asm}: {e}"));
            let sut = sut_word(&ops, is_sub, false)
                .unwrap_or_else(|e| panic!("SUT rejected valid {asm}: {e}"));
            prop_assert_eq!(sut, mc, "asm={}", asm);
        }

        #[test]
        fn encode_add_sub_neg_too_few_operands(
            n in 0usize..=2,
            is_sub in any::<bool>(),
            set_flags in any::<bool>(),
            r0 in 0u32..=30,
            r1 in 0u32..=30,
        ) {
            let mut ops = Vec::new();
            if n >= 1 {
                ops.push(Operand::Reg(gpr(true, r0, false)));
            }
            if n >= 2 {
                ops.push(Operand::Reg(gpr(true, r1, false)));
            }
            let err = encode_add_sub(&ops, is_sub, set_flags).expect_err("too few operands must Err");
            prop_assert!(
                err.contains("requires 3 operands"),
                "unexpected error for n={}: {}", n, err
            );
        }

        #[test]
        fn encode_add_sub_neg_imm_out_of_range(
            rd in 0u32..=30,
            rn in 0u32..=30,
            is_64 in any::<bool>(),
            is_sub in any::<bool>(),
            set_flags in any::<bool>(),
            imm in invalid_imm(),
            explicit_lsl12 in any::<bool>(),
        ) {
            if !explicit_lsl12 {
                prop_assume!(!is_valid_signed_imm12(imm));
            } else {
                // Explicit lsl #12: the unshifted field must fit in 12 bits; overflow is invalid.
                let mag = if imm == i64::MIN {
                    u64::MAX
                } else if imm < 0 {
                    (-imm) as u64
                } else {
                    imm as u64
                };
                prop_assume!(mag > 0xFFF);
            }
            let rd_n = gpr(is_64, rd, false);
            let rn_n = gpr(is_64, rn, false);
            let mut ops = vec![
                Operand::Reg(rd_n),
                Operand::Reg(rn_n),
                Operand::Imm(imm),
            ];
            if explicit_lsl12 {
                ops.push(Operand::Shift { kind: "lsl".into(), amount: 12 });
            }
            prop_assert!(
                encode_add_sub(&ops, is_sub, set_flags).is_err(),
                "out-of-range imm {} (lsl12={}) must Err, not encode", imm, explicit_lsl12
            );
        }

        #[test]
        fn encode_add_sub_neg_invalid_shift_extend(
            rd in 0u32..=30,
            rn in 0u32..=30,
            rm in 0u32..=30,
            is_64 in any::<bool>(),
            is_sub in any::<bool>(),
            set_flags in any::<bool>(),
            class in 0u8..=3,
            extra in 0u32..=64,
        ) {
            let rd_n = gpr(is_64, rd, false);
            let rn_n = gpr(is_64, rn, false);
            let rm_n = gpr(is_64, rm, false);
            let mut ops = vec![
                Operand::Reg(rd_n),
                Operand::Reg(rn_n),
                Operand::Reg(rm_n),
            ];
            match class {
                0 => {
                    // ROR is not a valid ADD/SUB shift.
                    ops.push(Operand::Shift { kind: "ror".into(), amount: extra % 64 });
                }
                1 => {
                    // Shift amount outside the sf-dependent range.
                    let bad = if is_64 { 64 + (extra % 16) } else { 32 + (extra % 16) };
                    let kind = ["lsl", "lsr", "asr"][(extra as usize) % 3];
                    ops.push(Operand::Shift { kind: kind.into(), amount: bad });
                }
                2 => {
                    // Extend amount > 4 is UNALLOCATED (ARM ARM).
                    let kind = ["uxtb", "uxth", "uxtw", "uxtx", "sxtb", "sxth", "sxtw", "sxtx"]
                        [(extra as usize) % 8];
                    ops.push(Operand::Extend { kind: kind.into(), amount: 5 + (extra % 4) });
                }
                _ => {
                    // 32-bit shifted-register with imm6 bit 5 set (UNALLOCATED).
                    prop_assume!(!is_64);
                    ops.push(Operand::Shift { kind: "lsl".into(), amount: 32 + (extra % 32) });
                }
            }
            prop_assert!(
                encode_add_sub(&ops, is_sub, set_flags).is_err(),
                "invalid shift/extend must Err, got Ok for class={} extra={} is_64={}", class, extra, is_64
            );
        }

        #[test]
        fn encode_add_sub_metamorphic_neg_imm(
            rd in 0u32..=30,
            rn in 0u32..=31,
            is_64 in any::<bool>(),
            is_sub in any::<bool>(),
            set_flags in any::<bool>(),
            n in prop_oneof![1i64..=0xFFF, (1i64..=0xFFF).prop_map(|k| k << 12)],
            prefer_sp in any::<bool>(),
        ) {
            prop_assume!(n > 0);
            prop_assume!(is_valid_signed_imm12(n));
            let rn_n = gpr(is_64, rn, prefer_sp && rn == 31);
            let rd_n = gpr(is_64, rd, false);
            let ops_neg = [
                Operand::Reg(rd_n.clone()),
                Operand::Reg(rn_n.clone()),
                Operand::Imm(-n),
            ];
            let ops_pos = [
                Operand::Reg(rd_n),
                Operand::Reg(rn_n),
                Operand::Imm(n),
            ];
            let a = sut_word(&ops_neg, is_sub, set_flags)
                .unwrap_or_else(|e| panic!("neg-imm form rejected n={n}: {e}"));
            let b = sut_word(&ops_pos, !is_sub, set_flags)
                .unwrap_or_else(|e| panic!("pos-imm form rejected n={n}: {e}"));
            prop_assert_eq!(a, b, "add #-N must match sub #N (n={} is_sub={})", n, is_sub);
        }

        #[test]
        fn encode_add_sub_neg_imm_bad_shift(
            rd in 0u32..=30,
            rn in 0u32..=30,
            is_64 in any::<bool>(),
            is_sub in any::<bool>(),
            set_flags in any::<bool>(),
            imm in 0i64..=0xFFF,
            class in 0u8..=3,
            amt in 0u32..=63,
        ) {
            // ARM ADD/SUB (immediate) allows only LSL #0 or LSL #12 after #imm.
            let (kind, amt) = match class {
                0 => ("lsr", amt),
                1 => ("asr", amt),
                2 => ("ror", amt),
                _ => {
                    let a = if amt == 0 || amt == 12 { 1 + (amt % 11) } else { amt };
                    prop_assume!(a != 0 && a != 12);
                    ("lsl", a)
                }
            };
            let rd_n = gpr(is_64, rd, false);
            let rn_n = gpr(is_64, rn, false);
            let ops = vec![
                Operand::Reg(rd_n),
                Operand::Reg(rn_n),
                Operand::Imm(imm),
                Operand::Shift { kind: kind.into(), amount: amt },
            ];
            prop_assert!(
                encode_add_sub(&ops, is_sub, set_flags).is_err(),
                "immediate form with {} #{} must Err, not ignore the shift", kind, amt
            );
        }

        #[test]
        fn encode_add_sub_neg_mixed_width(
            rd in 0u32..=30,
            rn in 0u32..=30,
            rm in 0u32..=30,
            rd64 in any::<bool>(),
            rn64 in any::<bool>(),
            rm64 in any::<bool>(),
            is_sub in any::<bool>(),
            set_flags in any::<bool>(),
            use_imm in any::<bool>(),
            imm in 0i64..=0xFFF,
        ) {
            if use_imm {
                prop_assume!(rd64 != rn64);
                let ops = vec![
                    Operand::Reg(gpr(rd64, rd, false)),
                    Operand::Reg(gpr(rn64, rn, false)),
                    Operand::Imm(imm),
                ];
                prop_assert!(
                    encode_add_sub(&ops, is_sub, set_flags).is_err(),
                    "mixed-width immediate form must Err (rd64={} rn64={})", rd64, rn64
                );
            } else {
                prop_assume!(!(rd64 == rn64 && rn64 == rm64));
                let ops = vec![
                    Operand::Reg(gpr(rd64, rd, false)),
                    Operand::Reg(gpr(rn64, rn, false)),
                    Operand::Reg(gpr(rm64, rm, false)),
                ];
                prop_assert!(
                    encode_add_sub(&ops, is_sub, set_flags).is_err(),
                    "mixed-width shifted-register form must Err"
                );
            }
        }

        #[test]
        fn encode_add_sub_reloc_lo12(
            rd in 0u32..=30,
            rn in 0u32..=30,
            is_64 in any::<bool>(),
            is_sub in any::<bool>(),
            set_flags in any::<bool>(),
            use_offset in any::<bool>(),
            off in -4096i64..=4096,
            suffix in 0u32..=1000,
        ) {
            let rd_n = gpr(is_64, rd, false);
            let rn_n = gpr(is_64, rn, false);
            let sym = format!("foo{suffix}");
            let ops = if use_offset {
                vec![
                    Operand::Reg(rd_n),
                    Operand::Reg(rn_n),
                    Operand::ModifierOffset {
                        kind: "lo12".into(),
                        symbol: sym.clone(),
                        offset: off,
                    },
                ]
            } else {
                vec![
                    Operand::Reg(rd_n),
                    Operand::Reg(rn_n),
                    Operand::Modifier {
                        kind: "lo12".into(),
                        symbol: sym.clone(),
                    },
                ]
            };
            let expected_addend = if use_offset { off } else { 0 };
            let (word, reloc) = match encode_add_sub(&ops, is_sub, set_flags) {
                Ok(EncodeResult::WordWithReloc { word, reloc }) => (word, reloc),
                other => panic!("expected WordWithReloc for :lo12:{sym}, got {other:?}"),
            };
            prop_assert!(
                matches!(reloc.reloc_type, super::super::RelocType::AddAbsLo12),
                "reloc_type must be AddAbsLo12"
            );
            prop_assert_eq!(&reloc.symbol, &sym);
            prop_assert_eq!(reloc.addend, expected_addend);
            let sf = if is_64 { 1u32 } else { 0 };
            let op = if is_sub { 1u32 } else { 0 };
            let s = if set_flags { 1u32 } else { 0 };
            prop_assert_eq!(word & 0x1F, rd);
            prop_assert_eq!((word >> 5) & 0x1F, rn);
            prop_assert_eq!((word >> 31) & 1, sf);
            prop_assert_eq!((word >> 30) & 1, op);
            prop_assert_eq!((word >> 29) & 1, s);
            prop_assert_eq!((word >> 24) & 0x1F, 0b10001);
            prop_assert_eq!((word >> 10) & 0xFFF, 0u32);
        }

        #[test]
        fn encode_add_sub_neg_fp_reg(
            which in 0u32..=2,
            is_sub in any::<bool>(),
            set_flags in any::<bool>(),
            prefix in prop::sample::select(vec!["d", "s", "q", "v", "h", "b"]),
            n in 0u32..=31,
        ) {
            let fp = format!("{prefix}{n}");
            let mut ops = vec![
                Operand::Reg("x1".into()),
                Operand::Reg("x2".into()),
                Operand::Reg("x3".into()),
            ];
            ops[which as usize] = Operand::Reg(fp.clone());
            prop_assert!(
                encode_add_sub(&ops, is_sub, set_flags).is_err(),
                "FP/SIMD register {fp} at operand {which} must Err"
            );
        }

        #[test]
        fn encode_add_sub_neg_adds_sp_rd(
            is_64 in any::<bool>(),
            is_sub in any::<bool>(),
            rn in 0u32..=30,
            imm in 0i64..=0xFFF,
        ) {
            let rd = if is_64 { "sp" } else { "wsp" };
            let rn_n = gpr(is_64, rn, false);
            let ops = [
                Operand::Reg(rd.into()),
                Operand::Reg(rn_n),
                Operand::Imm(imm),
            ];
            prop_assert!(
                encode_add_sub(&ops, is_sub, true).is_err(),
                "ADDS/SUBS with Rd=SP/WSP must Err (encodes as XZR/CMP otherwise)"
            );
        }

        #[test]
        fn encode_add_sub_reloc_tprel(
            rd in 0u32..=30,
            rn in 0u32..=30,
            is_64 in any::<bool>(),
            is_sub in any::<bool>(),
            set_flags in any::<bool>(),
            hi in any::<bool>(),
            suffix in 0u32..=1000,
        ) {
            let rd_n = gpr(is_64, rd, false);
            let rn_n = gpr(is_64, rn, false);
            let kind = if hi { "tprel_hi12" } else { "tprel_lo12_nc" };
            let sym = format!("tls{suffix}");
            let ops = vec![
                Operand::Reg(rd_n),
                Operand::Reg(rn_n),
                Operand::Modifier { kind: kind.into(), symbol: sym.clone() },
            ];
            let (word, reloc) = match encode_add_sub(&ops, is_sub, set_flags) {
                Ok(EncodeResult::WordWithReloc { word, reloc }) => (word, reloc),
                other => panic!("expected WordWithReloc for :{kind}:{sym}, got {other:?}"),
            };
            if hi {
                prop_assert!(matches!(reloc.reloc_type, super::super::RelocType::TlsLeAddTprelHi12));
                prop_assert_eq!((word >> 22) & 1, 1u32);
            } else {
                prop_assert!(matches!(reloc.reloc_type, super::super::RelocType::TlsLeAddTprelLo12));
                prop_assert_eq!((word >> 22) & 1, 0u32);
            }
            prop_assert_eq!(&reloc.symbol, &sym);
            prop_assert_eq!(reloc.addend, 0i64);
            prop_assert_eq!(word & 0x1F, rd);
            prop_assert_eq!((word >> 5) & 0x1F, rn);
        }
    }

    #[test]
    fn test_encode_add_sub_regression_ror_rejected() {
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Reg("w1".into()),
            Operand::Reg("w2".into()),
            Operand::Shift { kind: "ror".into(), amount: 0 },
        ];
        assert!(
            encode_add_sub(&ops, false, false).is_err(),
            "ROR is not a valid ADD/SUB shift; encoder must Err (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_add_sub_regression_imm12_lsl12_overflow() {
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Reg("w1".into()),
            Operand::Imm(4097),
            Operand::Shift { kind: "lsl".into(), amount: 12 },
        ];
        assert!(
            encode_add_sub(&ops, false, false).is_err(),
            "imm 4097 with explicit lsl #12 does not fit imm12; encoder must Err, not mask"
        );
    }

    #[test]
    fn test_encode_add_sub_regression_sp_lsl_extended() {
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Reg("wsp".into()),
            Operand::Reg("w0".into()),
            Operand::Shift { kind: "lsl".into(), amount: 1 },
        ];
        let sut = sut_word(&ops, false, false).expect("valid SP+LSL form");
        // llvm-mc: add w0, wsp, w0, lsl #1 => 0x0b2047e0 (extended UXTW #1)
        assert_eq!(
            sut, 0x0b2047e0,
            "SP/WSP as Rn with LSL #N (N<=4) must use extended-register form, not shifted-register (XZR)"
        );
    }

    #[test]
    fn test_encode_add_sub_regression_imm_lsr_ignored() {
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Imm(0),
            Operand::Shift { kind: "lsr".into(), amount: 0 },
        ];
        assert!(
            encode_add_sub(&ops, false, false).is_err(),
            "ADD/SUB immediate form allows only LSL #0/#12; lsr #0 must Err (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_add_sub_regression_mixed_width() {
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
        ];
        assert!(
            encode_add_sub(&ops, false, false).is_err(),
            "mixed x/w ADD/SUB shifted-register form must Err (llvm-mc rejects add x0, w0, w0)"
        );
    }

    #[test]
    fn test_encode_add_sub_regression_fp_reg() {
        let ops = [
            Operand::Reg("d0".into()),
            Operand::Reg("x1".into()),
            Operand::Reg("x2".into()),
        ];
        assert!(
            encode_add_sub(&ops, false, false).is_err(),
            "FP/SIMD register d0 is not a GPR ADD operand; encoder must Err"
        );
    }

    #[test]
    fn test_encode_add_sub_regression_adds_sp_rd() {
        let ops = [
            Operand::Reg("wsp".into()),
            Operand::Reg("w0".into()),
            Operand::Imm(0),
        ];
        assert!(
            encode_add_sub(&ops, false, true).is_err(),
            "ADDS wsp, w0, #0 must Err (Rd=SP with S=1 encodes as WZR)"
        );
    }
}

#[cfg(test)]
mod encode_adc_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler
    // Evidence: src/backend/arm/assembler/README.md "accepts the same textual assembly that GCC's gas would consume";
    //   encoder/mod.rs "Encodes AArch64 instructions into 32-bit machine code words";
    //   encoder/mod.rs:281-282 adc/adcs dispatch; ARM ARM ADC register form
    // Stronger considered:
    //   - State machine: rejected — encode_adc is a pure function with no lifecycle
    //   - Algebraic round-trip: rejected — no in-tree ADC decoder
    // Weaker available: algebraic.metamorphic (S bit), algebraic.invariant (ARM fields),
    //   negative_error (arity / non-register / extra shift / mixed width / SP)
    // Differential: candidate=encode_adc, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=(operands,set_flags)<->asm text `adc`/`adcs` Rd, Rn, Rm

    use super::encode_adc;
    use super::super::EncodeResult;
    use crate::backend::arm::assembler::parser::Operand;
    use proptest::prelude::*;
    use std::io::Write;
    use std::process::{Command, Stdio};

    const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
    const CASES: u32 = 1000;

    fn cfg() -> ProptestConfig {
        ProptestConfig::with_cases(CASES)
    }

    fn gpr(is_64: bool, n: u32) -> String {
        if n == 31 {
            if is_64 {
                "xzr".into()
            } else {
                "wzr".into()
            }
        } else {
            format!("{}{}", if is_64 { "x" } else { "w" }, n)
        }
    }

    fn mnemonic(set_flags: bool) -> &'static str {
        if set_flags {
            "adcs"
        } else {
            "adc"
        }
    }

    fn sut_word(ops: &[Operand], set_flags: bool) -> Result<u32, String> {
        match encode_adc(ops, set_flags)? {
            EncodeResult::Word(w) => Ok(w),
            other => Err(format!("expected Word, got {:?}", other)),
        }
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

    fn shift_kind() -> impl Strategy<Value = &'static str> {
        prop::sample::select(vec!["lsl", "lsr", "asr", "ror"])
    }

    fn bad_third() -> impl Strategy<Value = Operand> {
        prop_oneof![
            any::<i64>().prop_map(Operand::Imm),
            Just(Operand::Symbol("foo".into())),
            Just(Operand::Mem {
                base: "x0".into(),
                offset: 8,
            }),
            (shift_kind(), 0u32..=63u32).prop_map(|(k, a)| Operand::Shift {
                kind: k.into(),
                amount: a,
            }),
            Just(Operand::Cond("eq".into())),
        ]
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_adc_kat_llvm_mc_adc_x0_x1_x2() {
        let want = 0x9a020020u32;
        let mc = llvm_mc_word("adc x0, x1, x2").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("x1".into()),
            Operand::Reg("x2".into()),
        ];
        let sut = sut_word(&ops, false).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    proptest! {
        #![proptest_config(cfg())]

        #[test]
        fn encode_adc_diff_gpr_same_width(
            rd in 0u32..=31,
            rn in 0u32..=31,
            rm in 0u32..=31,
            is_64 in any::<bool>(),
            set_flags in any::<bool>(),
        ) {
            let rd_n = gpr(is_64, rd);
            let rn_n = gpr(is_64, rn);
            let rm_n = gpr(is_64, rm);
            let asm = format!(
                "{} {}, {}, {}",
                mnemonic(set_flags), rd_n, rn_n, rm_n
            );
            let ops = [
                Operand::Reg(rd_n),
                Operand::Reg(rn_n),
                Operand::Reg(rm_n),
            ];
            let sut = sut_word(&ops, set_flags)
                .unwrap_or_else(|e| panic!("SUT rejected valid ADC {}: {}", asm, e));
            let mc = llvm_mc_word(&asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid ADC {}: {}", asm, e));
            prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
        }

        #[test]
        fn encode_adc_metamorphic_s_bit(
            rd in 0u32..=31,
            rn in 0u32..=31,
            rm in 0u32..=31,
            is_64 in any::<bool>(),
        ) {
            let ops = [
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
                Operand::Reg(gpr(is_64, rm)),
            ];
            let adc = sut_word(&ops, false)
                .unwrap_or_else(|e| panic!("adc rejected: {}", e));
            let adcs = sut_word(&ops, true)
                .unwrap_or_else(|e| panic!("adcs rejected: {}", e));
            prop_assert_eq!(
                adc ^ adcs,
                1u32 << 29,
                "ADC vs ADCS must differ only by S bit 29 (adc={:#010x} adcs={:#010x})",
                adc,
                adcs
            );
        }

        #[test]
        fn encode_adc_invariant_arm_fields(
            rd in 0u32..=31,
            rn in 0u32..=31,
            rm in 0u32..=31,
            is_64 in any::<bool>(),
            set_flags in any::<bool>(),
        ) {
            let ops = [
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
                Operand::Reg(gpr(is_64, rm)),
            ];
            let w = sut_word(&ops, set_flags)
                .unwrap_or_else(|e| panic!("ADC rejected: {}", e));
            let sf = if is_64 { 1u32 } else { 0 };
            let s = if set_flags { 1u32 } else { 0 };
            prop_assert_eq!(w & 0x1F, rd, "Rd field");
            prop_assert_eq!((w >> 5) & 0x1F, rn, "Rn field");
            prop_assert_eq!((w >> 16) & 0x1F, rm, "Rm field");
            prop_assert_eq!((w >> 31) & 1, sf, "sf bit");
            prop_assert_eq!((w >> 29) & 1, s, "S bit");
            prop_assert_eq!((w >> 30) & 1, 0, "op bit must be 0 for ADC");
            prop_assert_eq!((w >> 21) & 0xFF, 0b11010000u32, "opcode bits 28:21");
            prop_assert_eq!((w >> 10) & 0x3F, 0, "bits 15:10 must be 000000");
        }

        #[test]
        fn encode_adc_neg_too_few_operands(
            n in 0usize..=2,
            set_flags in any::<bool>(),
            is_64 in any::<bool>(),
            r0 in 0u32..=31,
            r1 in 0u32..=31,
        ) {
            let all = [
                Operand::Reg(gpr(is_64, r0)),
                Operand::Reg(gpr(is_64, r1)),
            ];
            let ops = &all[..n.min(2)];
            prop_assert!(
                encode_adc(ops, set_flags).is_err(),
                "fewer than 3 operands must Err, n={}",
                n
            );
        }

        #[test]
        fn encode_adc_neg_non_register(
            rd in 0u32..=30,
            rn in 0u32..=30,
            is_64 in any::<bool>(),
            set_flags in any::<bool>(),
            which in 0u32..=2,
            bad in bad_third(),
        ) {
            let mut ops = vec![
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
                Operand::Reg(gpr(is_64, rd)),
            ];
            ops[which as usize] = bad;
            prop_assert!(
                encode_adc(&ops, set_flags).is_err(),
                "non-register operand at position {} must Err",
                which
            );
        }

        #[test]
        fn encode_adc_neg_invalid_reg_name(
            which in 0u32..=2,
            set_flags in any::<bool>(),
            bad in prop_oneof![
                Just("x32".to_string()),
                Just("w32".to_string()),
                Just("x99".to_string()),
                Just("w99".to_string()),
                Just("".to_string()),
                Just("foo".to_string()),
                Just("r0".to_string()),
                Just("x".to_string()),
                Just("x-1".to_string()),
            ],
        ) {
            let mut ops = vec![
                Operand::Reg("x0".into()),
                Operand::Reg("x1".into()),
                Operand::Reg("x2".into()),
            ];
            ops[which as usize] = Operand::Reg(bad.clone());
            prop_assert!(
                encode_adc(&ops, set_flags).is_err(),
                "invalid register name {:?} at {} must Err",
                bad,
                which
            );
        }

        #[test]
        fn encode_adc_neg_fp_reg(
            which in 0u32..=2,
            set_flags in any::<bool>(),
            prefix in prop::sample::select(vec!["d", "s", "q", "v", "h", "b"]),
            n in prop_oneof![Just(0u32), Just(31u32), 0u32..=31],
        ) {
            let fp = format!("{}{}", prefix, n);
            let mut ops = vec![
                Operand::Reg("x0".into()),
                Operand::Reg("x1".into()),
                Operand::Reg("x2".into()),
            ];
            ops[which as usize] = Operand::Reg(fp.clone());
            prop_assert!(
                encode_adc(&ops, set_flags).is_err(),
                "FP/SIMD register {} is not a valid ADC operand (which={})",
                fp,
                which
            );
        }

        #[test]
        fn encode_adc_neg_extra_shift(
            rd in 0u32..=30,
            rn in 0u32..=30,
            rm in 0u32..=30,
            is_64 in any::<bool>(),
            set_flags in any::<bool>(),
            kind in shift_kind(),
            amt in prop_oneof![Just(0u32), Just(1u32), Just(31u32), Just(32u32), Just(63u32), 0u32..=63],
        ) {
            let ops = [
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
                Operand::Reg(gpr(is_64, rm)),
                Operand::Shift {
                    kind: kind.into(),
                    amount: amt,
                },
            ];
            prop_assert!(
                encode_adc(&ops, set_flags).is_err(),
                "ADC has no shifted-register form; extra {} #{} must Err",
                kind,
                amt
            );
        }

        #[test]
        fn encode_adc_neg_mixed_width(
            rd in 0u32..=30,
            rn in 0u32..=30,
            rm in 0u32..=30,
            rd64 in any::<bool>(),
            rn64 in any::<bool>(),
            rm64 in any::<bool>(),
            set_flags in any::<bool>(),
        ) {
            prop_assume!(!(rd64 == rn64 && rn64 == rm64));
            let ops = [
                Operand::Reg(gpr(rd64, rd)),
                Operand::Reg(gpr(rn64, rn)),
                Operand::Reg(gpr(rm64, rm)),
            ];
            prop_assert!(
                encode_adc(&ops, set_flags).is_err(),
                "mixed-width ADC registers must Err (rd64={} rn64={} rm64={})",
                rd64,
                rn64,
                rm64
            );
        }

        #[test]
        fn encode_adc_neg_sp(
            which in 0u32..=2,
            is_64 in any::<bool>(),
            set_flags in any::<bool>(),
            a in 0u32..=30,
            b in 0u32..=30,
        ) {
            let sp = if is_64 { "sp" } else { "wsp" };
            let ra = gpr(is_64, a);
            let rb = gpr(is_64, b);
            let mut names = [ra, rb, sp.to_string()];
            // Place SP at operand `which`.
            names.swap(2, which as usize);
            let ops = [
                Operand::Reg(names[0].clone()),
                Operand::Reg(names[1].clone()),
                Operand::Reg(names[2].clone()),
            ];
            prop_assert!(
                encode_adc(&ops, set_flags).is_err(),
                "SP/WSP is not a valid ADC operand (which={} names={:?})",
                which,
                names
            );
        }
    }

    #[test]
    fn test_encode_adc_regression_extra_shift() {
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Shift {
                kind: "lsl".into(),
                amount: 0,
            },
        ];
        assert!(
            encode_adc(&ops, false).is_err(),
            "ADC has no shifted-register form; extra lsl #0 must Err (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_adc_regression_mixed_width() {
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("x0".into()),
        ];
        assert!(
            encode_adc(&ops, false).is_err(),
            "mixed-width ADC w0, w0, x0 must Err (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_adc_regression_sp() {
        let ops = [
            Operand::Reg("wsp".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
        ];
        assert!(
            encode_adc(&ops, false).is_err(),
            "ADC wsp, w0, w0 must Err; register 31 is WZR not WSP (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_adc_regression_fp_reg() {
        let ops = [
            Operand::Reg("d0".into()),
            Operand::Reg("x1".into()),
            Operand::Reg("x2".into()),
        ];
        assert!(
            encode_adc(&ops, false).is_err(),
            "ADC d0, x1, x2 must Err; FP/SIMD registers are not ADC operands (llvm-mc rejects it)"
        );
    }
}

#[cfg(test)]
mod encode_bic_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler
    // Evidence: src/backend/arm/assembler/README.md "accepts the same textual assembly that GCC's gas would consume";
    //   encoder/mod.rs:1-7 "Encodes AArch64 instructions into 32-bit machine code words";
    //   encoder/mod.rs:674 "bic" => encode_bic; ARM ARM BIC shifted-register / immediate-alias / vector
    // Stronger considered:
    //   - State machine: rejected — encode_bic is a pure function with no lifecycle
    //   - Algebraic round-trip: rejected — no in-tree BIC decoder
    // Weaker available: algebraic.metamorphic (BIC-imm = AND-~imm), negative_error (arity / mixed width / SP / FP / shift range / NEON T)
    // Differential: candidate=encode_bic, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=operands <-> asm text `bic Rd, Rn, Rm{, shift}` / `bic Rd, Rn, #imm` / `bic Vd.T, Vn.T, Vm.T`

    use super::{encode_bic, encode_logical};
    use super::super::EncodeResult;
    use crate::backend::arm::assembler::parser::Operand;
    use proptest::prelude::*;
    use std::io::Write;
    use std::process::{Command, Stdio};

    const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
    const CASES: u32 = 1000;

    fn cfg() -> ProptestConfig {
        ProptestConfig::with_cases(CASES)
    }

    fn gpr(is_64: bool, n: u32) -> String {
        if n == 31 {
            if is_64 {
                "xzr".into()
            } else {
                "wzr".into()
            }
        } else {
            format!("{}{}", if is_64 { "x" } else { "w" }, n)
        }
    }

    /// Immediate-form Rd: register 31 is SP/WSP (AND-immediate), never XZR.
    fn gpr_imm_rd(is_64: bool, n: u32) -> String {
        if n == 31 {
            if is_64 {
                "sp".into()
            } else {
                "wsp".into()
            }
        } else {
            gpr(is_64, n)
        }
    }

    fn sut_word(ops: &[Operand]) -> Result<u32, String> {
        match encode_bic(ops)? {
            EncodeResult::Word(w) => Ok(w),
            other => Err(format!("expected Word, got {:?}", other)),
        }
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
        let out = child
            .wait_with_output()
            .map_err(|e| format!("wait llvm-mc: {e}"))?;
        let stdout = String::from_utf8_lossy(&out.stdout);
        let stderr = String::from_utf8_lossy(&out.stderr);
        if !out.status.success() || stderr.contains("error:") {
            return Err(format!("llvm-mc error: {stderr}"));
        }
        parse_llvm_encoding(&stdout)
    }

    /// Independent AArch64 logical-immediate constructor (ARM ARM, not encode_bitmask_imm).
    /// `ones` consecutive 1s of element `size`, right-rotated by `immr`, tiled to register width.
    fn bitmask_from_fields(size: u32, ones: u32, immr: u32, is_64: bool) -> u64 {
        let width = if is_64 { 64u32 } else { 32 };
        let mask = if size == 64 {
            u64::MAX
        } else {
            (1u64 << size) - 1
        };
        let base = (1u64 << ones) - 1;
        let elem = if immr % size == 0 {
            base
        } else {
            let r = immr % size;
            ((base >> r) | (base << (size - r))) & mask
        };
        let mut val = 0u64;
        let mut pos = 0u32;
        while pos < width {
            val |= elem << pos;
            pos += size;
        }
        val
    }

    fn reg_num() -> impl Strategy<Value = u32> {
        prop_oneof![Just(0u32), Just(1u32), Just(30u32), Just(31u32), 0u32..=31]
    }

    fn shift_kind() -> impl Strategy<Value = &'static str> {
        prop::sample::select(vec!["lsl", "lsr", "asr", "ror"])
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_bic_kat_llvm_mc_bic_x0_x1_x2() {
        let want = 0x8a220020u32;
        let mc = llvm_mc_word("bic x0, x1, x2").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("x1".into()),
            Operand::Reg("x2".into()),
        ];
        let sut = sut_word(&ops).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_bic_kat_llvm_mc_bic_imm1() {
        let want = 0x927ff820u32;
        let mc = llvm_mc_word("bic x0, x1, #1").expect("llvm-mc imm KAT");
        assert_eq!(mc, want, "llvm-mc imm KAT mapping broken");
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("x1".into()),
            Operand::Imm(1),
        ];
        let sut = sut_word(&ops).expect("SUT imm KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_bic_kat_llvm_mc_bic_neon_16b() {
        let want = 0x4e621c20u32;
        let mc = llvm_mc_word("bic v0.16b, v1.16b, v2.16b").expect("llvm-mc neon KAT");
        assert_eq!(mc, want, "llvm-mc neon KAT mapping broken");
        let ops = [
            Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "16b".into(),
            },
            Operand::RegArrangement {
                reg: "v1".into(),
                arrangement: "16b".into(),
            },
            Operand::RegArrangement {
                reg: "v2".into(),
                arrangement: "16b".into(),
            },
        ];
        let sut = sut_word(&ops).expect("SUT neon KAT");
        assert_eq!(sut, want);
    }

    proptest! {
        #![proptest_config(cfg())]

        #[test]
        fn encode_bic_diff_reg_llvm_mc(
            rd in reg_num(),
            rn in reg_num(),
            rm in reg_num(),
            is_64 in any::<bool>(),
            kind in shift_kind(),
            use_shift in any::<bool>(),
            amt in 0u32..=63,
        ) {
            let max = if is_64 { 63u32 } else { 31 };
            let amt = if use_shift { amt % (max + 1) } else { 0 };
            let rd_n = gpr(is_64, rd);
            let rn_n = gpr(is_64, rn);
            let rm_n = gpr(is_64, rm);
            let mut ops = vec![
                Operand::Reg(rd_n.clone()),
                Operand::Reg(rn_n.clone()),
                Operand::Reg(rm_n.clone()),
            ];
            let mut asm = format!("bic {}, {}, {}", rd_n, rn_n, rm_n);
            if use_shift {
                ops.push(Operand::Shift {
                    kind: kind.to_string(),
                    amount: amt,
                });
                if !(kind == "lsl" && amt == 0) {
                    asm.push_str(&format!(", {} #{}", kind, amt));
                }
            }
            let mc = llvm_mc_word(&asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
            let sut = sut_word(&ops)
                .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
            prop_assert_eq!(sut, mc, "mismatch for {}", asm);
        }

        #[test]
        fn encode_bic_diff_imm_llvm_mc(
            rd in reg_num(),
            rn in reg_num(),
            is_64 in any::<bool>(),
            seed in 0u32..10000,
        ) {
            // Draw a valid BIC immediate whose inverted value is an AArch64 bitmask.
            let sizes: [u32; 6] = if is_64 {
                [2, 4, 8, 16, 32, 64]
            } else {
                [2, 4, 8, 16, 32, 32]
            };
            let size = sizes[(seed as usize) % sizes.len()];
            let ones = 1 + (seed / 6) % (size - 1);
            let rot = (seed / 6 / (size - 1).max(1)) % size;
            let m = bitmask_from_fields(size, ones, rot, is_64);
            let bic_u = if is_64 { !m } else { (!(m as u32)) as u64 };
            let imm = bic_u as i64;
            let rd_n = gpr_imm_rd(is_64, rd);
            let rn_n = gpr(is_64, rn);
            let hex = if is_64 {
                format!("#0x{:x}", bic_u)
            } else {
                format!("#0x{:x}", bic_u as u32)
            };
            let asm = format!("bic {}, {}, {}", rd_n, rn_n, hex);
            let ops = [
                Operand::Reg(rd_n),
                Operand::Reg(rn_n),
                Operand::Imm(imm),
            ];
            let mc = llvm_mc_word(&asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
            let sut = sut_word(&ops)
                .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
            prop_assert_eq!(sut, mc, "mismatch for {}", asm);
        }

        #[test]
        fn encode_bic_diff_neon_llvm_mc(
            d in reg_num(),
            n in reg_num(),
            m in reg_num(),
            q16 in any::<bool>(),
        ) {
            let arr = if q16 { "16b" } else { "8b" };
            let asm = format!("bic v{}.{}, v{}.{}, v{}.{}", d, arr, n, arr, m, arr);
            let ops = [
                Operand::RegArrangement {
                    reg: format!("v{}", d),
                    arrangement: arr.into(),
                },
                Operand::RegArrangement {
                    reg: format!("v{}", n),
                    arrangement: arr.into(),
                },
                Operand::RegArrangement {
                    reg: format!("v{}", m),
                    arrangement: arr.into(),
                },
            ];
            let mc = llvm_mc_word(&asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
            let sut = sut_word(&ops)
                .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
            prop_assert_eq!(sut, mc, "mismatch for {}", asm);
        }

        #[test]
        fn encode_bic_meta_imm_and_alias(
            rd in 0u32..=30,
            rn in reg_num(),
            is_64 in any::<bool>(),
            seed in 0u32..10000,
        ) {
            let sizes: [u32; 6] = if is_64 {
                [2, 4, 8, 16, 32, 64]
            } else {
                [2, 4, 8, 16, 32, 32]
            };
            let size = sizes[(seed as usize) % sizes.len()];
            let ones = 1 + (seed / 6) % (size - 1);
            let rot = (seed / 6 / (size - 1).max(1)) % size;
            let m = bitmask_from_fields(size, ones, rot, is_64);
            let bic_u = if is_64 { !m } else { (!(m as u32)) as u64 };
            let imm = bic_u as i64;
            let inv = if is_64 {
                (!bic_u) as i64
            } else {
                ((!(bic_u as u32)) as u64) as i64
            };
            let rd_n = gpr(is_64, rd);
            let rn_n = gpr(is_64, rn);
            let bic_ops = [
                Operand::Reg(rd_n.clone()),
                Operand::Reg(rn_n.clone()),
                Operand::Imm(imm),
            ];
            let and_ops = [
                Operand::Reg(rd_n),
                Operand::Reg(rn_n),
                Operand::Imm(inv),
            ];
            let bic_w = match encode_bic(&bic_ops) {
                Ok(EncodeResult::Word(w)) => w,
                other => panic!("BIC imm should encode, got {:?}", other),
            };
            let and_w = match encode_logical(&and_ops, 0b00) {
                Ok(EncodeResult::Word(w)) => w,
                other => panic!("AND ~imm should encode, got {:?}", other),
            };
            prop_assert_eq!(bic_w, and_w, "BIC #imm must equal AND #~imm");
        }

        #[test]
        fn encode_bic_neg_arity(n in 0usize..=2, is_64 in any::<bool>(), r in 0u32..=30) {
            let ops: Vec<Operand> = (0..n)
                .map(|_| Operand::Reg(gpr(is_64, r)))
                .collect();
            prop_assert!(
                encode_bic(&ops).is_err(),
                "fewer than 3 operands must Err, n={}",
                n
            );
        }

        #[test]
        fn encode_bic_neg_mixed_width(
            rd in 0u32..=30,
            rn in 0u32..=30,
            rm in 0u32..=30,
            rd64 in any::<bool>(),
            rn64 in any::<bool>(),
            rm64 in any::<bool>(),
        ) {
            prop_assume!(!(rd64 == rn64 && rn64 == rm64));
            let ops = [
                Operand::Reg(gpr(rd64, rd)),
                Operand::Reg(gpr(rn64, rn)),
                Operand::Reg(gpr(rm64, rm)),
            ];
            prop_assert!(
                encode_bic(&ops).is_err(),
                "mixed-width BIC registers must Err (rd64={} rn64={} rm64={})",
                rd64,
                rn64,
                rm64
            );
        }

        #[test]
        fn encode_bic_neg_sp_fp_regform(
            which in 0u32..=2,
            is_64 in any::<bool>(),
            a in 0u32..=30,
            b in 0u32..=30,
            kind in 0u32..=8,
            fp_n in prop_oneof![Just(0u32), Just(31u32), 0u32..=31],
        ) {
            let ra = gpr(is_64, a);
            let rb = gpr(is_64, b);
            let bad = match kind {
                0 => if is_64 { "sp".to_string() } else { "wsp".to_string() },
                1 => format!("d{}", fp_n),
                2 => format!("s{}", fp_n),
                3 => format!("q{}", fp_n),
                4 => format!("v{}", fp_n),
                5 => format!("h{}", fp_n),
                6 => format!("b{}", fp_n),
                7 => if is_64 { "sp".to_string() } else { "wsp".to_string() },
                _ => format!("d{}", fp_n),
            };
            let mut names = [ra, rb, bad.clone()];
            names.swap(2, which as usize);
            let ops = [
                Operand::Reg(names[0].clone()),
                Operand::Reg(names[1].clone()),
                Operand::Reg(names[2].clone()),
            ];
            prop_assert!(
                encode_bic(&ops).is_err(),
                "SP/FP name {} at operand {} must Err (names={:?})",
                bad,
                which,
                names
            );
        }

        #[test]
        fn encode_bic_neg_shift_range_neon_arr(
            rd in 0u32..=30,
            rn in 0u32..=30,
            rm in 0u32..=30,
            is_64 in any::<bool>(),
            kind in shift_kind(),
            which in 0u32..=1,
            amt_w in prop_oneof![Just(32u32), Just(33u32), Just(63u32), Just(64u32)],
            amt_x in prop_oneof![Just(64u32), Just(65u32), Just(128u32)],
            arr in prop::sample::select(vec!["8h", "4h", "4s", "2s", "2d"]),
            d in 0u32..=31,
            n in 0u32..=31,
            m in 0u32..=31,
        ) {
            if which == 0 {
                let amt = if is_64 { amt_x } else { amt_w };
                let ops = [
                    Operand::Reg(gpr(is_64, rd)),
                    Operand::Reg(gpr(is_64, rn)),
                    Operand::Reg(gpr(is_64, rm)),
                    Operand::Shift {
                        kind: kind.to_string(),
                        amount: amt,
                    },
                ];
                prop_assert!(
                    encode_bic(&ops).is_err(),
                    "out-of-range {} #{} on {}-bit BIC must Err",
                    kind,
                    amt,
                    if is_64 { 64 } else { 32 }
                );
            } else {
                let ops = [
                    Operand::RegArrangement {
                        reg: format!("v{}", d),
                        arrangement: arr.to_string(),
                    },
                    Operand::RegArrangement {
                        reg: format!("v{}", n),
                        arrangement: arr.to_string(),
                    },
                    Operand::RegArrangement {
                        reg: format!("v{}", m),
                        arrangement: arr.to_string(),
                    },
                ];
                prop_assert!(
                    encode_bic(&ops).is_err(),
                    "NEON BIC T={} is not 8b/16b and must Err",
                    arr
                );
            }
        }
    }

    proptest! {
        #![proptest_config(cfg())]

        #[test]
        fn encode_bic_neg_fp_reg(
            which in 0u32..=2,
            prefix in prop::sample::select(vec!["d", "s", "q", "v", "h", "b"]),
            n in prop_oneof![Just(0u32), Just(31u32), 0u32..=31],
        ) {
            let fp = format!("{}{}", prefix, n);
            let mut ops = vec![
                Operand::Reg("x0".into()),
                Operand::Reg("x1".into()),
                Operand::Reg("x2".into()),
            ];
            ops[which as usize] = Operand::Reg(fp.clone());
            prop_assert!(
                encode_bic(&ops).is_err(),
                "FP/SIMD register {} is not a valid BIC GPR (which={})",
                fp,
                which
            );
        }

        #[test]
        fn encode_bic_neg_invalid_neon_arr(
            d in 0u32..=31,
            n in 0u32..=31,
            m in 0u32..=31,
            arr in prop::sample::select(vec!["8h", "4h", "4s", "2s", "2d"]),
        ) {
            let ops = [
                Operand::RegArrangement {
                    reg: format!("v{}", d),
                    arrangement: arr.to_string(),
                },
                Operand::RegArrangement {
                    reg: format!("v{}", n),
                    arrangement: arr.to_string(),
                },
                Operand::RegArrangement {
                    reg: format!("v{}", m),
                    arrangement: arr.to_string(),
                },
            ];
            prop_assert!(
                encode_bic(&ops).is_err(),
                "NEON BIC T={} is not 8b/16b and must Err",
                arr
            );
        }

        #[test]
        fn encode_bic_neg_imm_xzr_rd(is_64 in any::<bool>(), rn in 0u32..=30) {
            let rd = if is_64 { "xzr" } else { "wzr" };
            let rn_n = gpr(is_64, rn);
            let ops = [
                Operand::Reg(rd.into()),
                Operand::Reg(rn_n),
                Operand::Imm(1),
            ];
            prop_assert!(
                encode_bic(&ops).is_err(),
                "BIC immediate Rd={} is XZR not SP and must Err (llvm-mc rejects it)",
                rd
            );
        }

        #[test]
        fn encode_bic_neg_invalid_imm(
            is_64 in any::<bool>(),
            rd in 0u32..=30,
            rn in 0u32..=30,
            imm in prop_oneof![
                Just(0i64),
                Just(-1i64),
                Just(5i64),
                Just(9i64),
                Just(0x11i64),
                Just(i64::MIN),
            ],
        ) {
            let rd_n = gpr(is_64, rd);
            let rn_n = gpr(is_64, rn);
            let hex = if is_64 {
                format!("#0x{:x}", imm as u64)
            } else {
                format!("#0x{:x}", imm as u32)
            };
            let asm = format!("bic {}, {}, {}", rd_n, rn_n, hex);
            let ops = [
                Operand::Reg(rd_n),
                Operand::Reg(rn_n),
                Operand::Imm(imm),
            ];
            match llvm_mc_word(&asm) {
                Ok(mc) => {
                    let sut = sut_word(&ops)
                        .unwrap_or_else(|e| panic!("SUT rejected llvm-mc-valid {}: {}", asm, e));
                    prop_assert_eq!(sut, mc, "mismatch for {}", asm);
                }
                Err(_) => {
                    prop_assert!(
                        encode_bic(&ops).is_err(),
                        "invalid bitmask BIC {} must Err (llvm-mc rejects it)",
                        asm
                    );
                }
            }
        }

        #[test]
        fn encode_bic_neg_unsupported_third(
            is_64 in any::<bool>(),
            rd in 0u32..=30,
            rn in 0u32..=30,
            which in 0u32..=4,
        ) {
            let bad = match which {
                0 => Operand::Mem {
                    base: "x0".into(),
                    offset: 0,
                },
                1 => Operand::Symbol("foo".into()),
                2 => Operand::Cond("eq".into()),
                3 => Operand::Label(".L0".into()),
                _ => Operand::Barrier("sy".into()),
            };
            let ops = [
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
                bad,
            ];
            prop_assert!(
                encode_bic(&ops).is_err(),
                "non-Reg/Imm third operand must Err (which={})",
                which
            );
        }

        #[test]
        fn encode_bic_neg_invalid_rm(
            is_64 in any::<bool>(),
            rd in 0u32..=30,
            rn in 0u32..=30,
            bad in prop_oneof![
                Just("x32".to_string()),
                Just("w32".to_string()),
                Just("x99".to_string()),
                Just("".to_string()),
                Just("foo".to_string()),
                Just("r0".to_string()),
                Just("x".to_string()),
            ],
        ) {
            let ops = [
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
                Operand::Reg(bad.clone()),
            ];
            prop_assert!(
                encode_bic(&ops).is_err(),
                "invalid rm name {:?} must Err",
                bad
            );
        }

        #[test]
        fn encode_bic_neg_unknown_shift_kind(
            rd in 0u32..=30,
            rn in 0u32..=30,
            rm in 0u32..=30,
            is_64 in any::<bool>(),
            kind in prop_oneof![
                Just("lslx".to_string()),
                Just("rrx".to_string()),
                Just("rol".to_string()),
                Just("".to_string()),
                Just("asr ".to_string()),
            ],
            amt in prop_oneof![Just(0u32), Just(1u32), Just(31u32)],
        ) {
            let ops = [
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
                Operand::Reg(gpr(is_64, rm)),
                Operand::Shift {
                    kind: kind.clone(),
                    amount: amt,
                },
            ];
            prop_assert!(
                encode_bic(&ops).is_err(),
                "unknown shift kind {:?} must Err",
                kind
            );
        }
    }

    #[test]
    fn test_encode_bic_regression_mixed_width() {
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("x0".into()),
        ];
        assert!(
            encode_bic(&ops).is_err(),
            "mixed-width BIC w0, w0, x0 must Err (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_bic_regression_sp() {
        let ops = [
            Operand::Reg("wsp".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
        ];
        assert!(
            encode_bic(&ops).is_err(),
            "BIC wsp, w0, w0 must Err; register 31 is WZR not WSP (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_bic_regression_shift32() {
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Shift {
                kind: "lsl".into(),
                amount: 32,
            },
        ];
        assert!(
            encode_bic(&ops).is_err(),
            "BIC w0, w0, w0, lsl #32 must Err; 32-bit shift amount range is [0, 31]"
        );
    }

    #[test]
    fn test_encode_bic_regression_fp_reg() {
        let ops = [
            Operand::Reg("d0".into()),
            Operand::Reg("x1".into()),
            Operand::Reg("x2".into()),
        ];
        assert!(
            encode_bic(&ops).is_err(),
            "BIC d0, x1, x2 must Err; FP/SIMD registers are not BIC GPRs (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_bic_regression_neon_8h() {
        let ops = [
            Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "8h".into(),
            },
            Operand::RegArrangement {
                reg: "v1".into(),
                arrangement: "8h".into(),
            },
            Operand::RegArrangement {
                reg: "v2".into(),
                arrangement: "8h".into(),
            },
        ];
        assert!(
            encode_bic(&ops).is_err(),
            "BIC v0.8h, v1.8h, v2.8h must Err; ARM ARM BIC vector T is 8B|16B only"
        );
    }

    #[test]
    fn test_encode_bic_regression_imm_xzr_rd() {
        let ops = [
            Operand::Reg("xzr".into()),
            Operand::Reg("x0".into()),
            Operand::Imm(1),
        ];
        assert!(
            encode_bic(&ops).is_err(),
            "BIC xzr, x0, #1 must Err; AND-immediate Rd of 31 is SP not XZR (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_bic_regression_unknown_shift_kind() {
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Shift {
                kind: "lslx".into(),
                amount: 0,
            },
        ];
        assert!(
            encode_bic(&ops).is_err(),
            "BIC w0, w0, w0, lslx #0 must Err; only lsl/lsr/asr/ror are valid (unknown kinds default to lsl)"
        );
    }
}

#[cfg(test)]
mod encode_bics_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler
    // Evidence: src/backend/arm/assembler/README.md "accepts the same textual assembly that GCC's gas would consume";
    //   encoder/mod.rs:1-7 "Encodes AArch64 instructions into 32-bit machine code words";
    //   encoder/mod.rs:237 "bics" => encode_bics; ARM ARM Logical (shifted register) BICS opc=11 N=1
    // Stronger considered:
    //   - State machine: rejected — encode_bics is a pure function with no lifecycle
    //   - Algebraic round-trip: rejected — no in-tree BICS decoder
    // Weaker available: algebraic.metamorphic (opc XOR vs BIC), algebraic.invariant (word layout),
    //   negative_error (arity / mixed width / SP / FP / shift range)
    // Differential: candidate=encode_bics, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=operands <-> asm text `bics Rd, Rn, Rm{, shift}` / `bics Rd, Rn, #imm` (ANDS #~imm alias)

    use super::{encode_bic, encode_bics};
    use super::super::EncodeResult;
    use crate::backend::arm::assembler::parser::Operand;
    use proptest::prelude::*;
    use std::io::Write;
    use std::process::{Command, Stdio};

    const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
    const CASES: u32 = 1000;

    fn cfg() -> ProptestConfig {
        ProptestConfig::with_cases(CASES)
    }

    fn gpr(is_64: bool, n: u32) -> String {
        if n == 31 {
            if is_64 {
                "xzr".into()
            } else {
                "wzr".into()
            }
        } else {
            format!("{}{}", if is_64 { "x" } else { "w" }, n)
        }
    }

    fn sut_word(ops: &[Operand]) -> Result<u32, String> {
        match encode_bics(ops)? {
            EncodeResult::Word(w) => Ok(w),
            other => Err(format!("expected Word, got {:?}", other)),
        }
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
        let out = child
            .wait_with_output()
            .map_err(|e| format!("wait llvm-mc: {e}"))?;
        let stdout = String::from_utf8_lossy(&out.stdout);
        let stderr = String::from_utf8_lossy(&out.stderr);
        if !out.status.success() || stderr.contains("error:") {
            return Err(format!("llvm-mc error: {stderr}"));
        }
        parse_llvm_encoding(&stdout)
    }

    /// Independent AArch64 logical-immediate constructor (ARM ARM, not encode_bitmask_imm).
    fn bitmask_from_fields(size: u32, ones: u32, immr: u32, is_64: bool) -> u64 {
        let width = if is_64 { 64u32 } else { 32 };
        let mask = if size == 64 {
            u64::MAX
        } else {
            (1u64 << size) - 1
        };
        let base = (1u64 << ones) - 1;
        let elem = if immr % size == 0 {
            base
        } else {
            let r = immr % size;
            ((base >> r) | (base << (size - r))) & mask
        };
        let mut val = 0u64;
        let mut pos = 0u32;
        while pos < width {
            val |= elem << pos;
            pos += size;
        }
        val
    }

    fn reg_num() -> impl Strategy<Value = u32> {
        prop_oneof![Just(0u32), Just(1u32), Just(30u32), Just(31u32), 0u32..=31]
    }

    fn shift_kind() -> impl Strategy<Value = &'static str> {
        prop::sample::select(vec!["lsl", "lsr", "asr", "ror"])
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_bics_kat_llvm_mc_bics_x0_x1_x2() {
        let want = 0xea220020u32;
        let mc = llvm_mc_word("bics x0, x1, x2").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("x1".into()),
            Operand::Reg("x2".into()),
        ];
        let sut = sut_word(&ops).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_bics_kat_llvm_mc_bics_w0_w1_w2() {
        let want = 0x6a220020u32;
        let mc = llvm_mc_word("bics w0, w1, w2").expect("llvm-mc W KAT");
        assert_eq!(mc, want, "llvm-mc W KAT mapping broken");
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Reg("w1".into()),
            Operand::Reg("w2".into()),
        ];
        let sut = sut_word(&ops).expect("SUT W KAT");
        assert_eq!(sut, want);
    }

    proptest! {
        #![proptest_config(cfg())]

        #[test]
        fn encode_bics_diff_reg_llvm_mc(
            rd in reg_num(),
            rn in reg_num(),
            rm in reg_num(),
            is_64 in any::<bool>(),
            kind in shift_kind(),
            use_shift in any::<bool>(),
            amt in 0u32..=63,
        ) {
            let max = if is_64 { 63u32 } else { 31 };
            let amt = if use_shift { amt % (max + 1) } else { 0 };
            let rd_n = gpr(is_64, rd);
            let rn_n = gpr(is_64, rn);
            let rm_n = gpr(is_64, rm);
            let mut ops = vec![
                Operand::Reg(rd_n.clone()),
                Operand::Reg(rn_n.clone()),
                Operand::Reg(rm_n.clone()),
            ];
            let mut asm = format!("bics {}, {}, {}", rd_n, rn_n, rm_n);
            if use_shift {
                ops.push(Operand::Shift {
                    kind: kind.to_string(),
                    amount: amt,
                });
                if !(kind == "lsl" && amt == 0) {
                    asm.push_str(&format!(", {} #{}", kind, amt));
                }
            }
            let mc = llvm_mc_word(&asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
            let sut = sut_word(&ops)
                .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
            prop_assert_eq!(sut, mc, "mismatch for {}", asm);
        }

        #[test]
        fn encode_bics_diff_imm_llvm_mc(
            rd in reg_num(),
            rn in reg_num(),
            is_64 in any::<bool>(),
            seed in 0u32..10000,
        ) {
            let sizes: [u32; 6] = if is_64 {
                [2, 4, 8, 16, 32, 64]
            } else {
                [2, 4, 8, 16, 32, 32]
            };
            let size = sizes[(seed as usize) % sizes.len()];
            let ones = 1 + (seed / 6) % (size - 1);
            let rot = (seed / 6 / (size - 1).max(1)) % size;
            let m = bitmask_from_fields(size, ones, rot, is_64);
            let bics_u = if is_64 { !m } else { (!(m as u32)) as u64 };
            let imm = bics_u as i64;
            let rd_n = gpr(is_64, rd);
            let rn_n = gpr(is_64, rn);
            let hex = if is_64 {
                format!("#0x{:x}", bics_u)
            } else {
                format!("#0x{:x}", bics_u as u32)
            };
            let asm = format!("bics {}, {}, {}", rd_n, rn_n, hex);
            let ops = [
                Operand::Reg(rd_n),
                Operand::Reg(rn_n),
                Operand::Imm(imm),
            ];
            let mc = llvm_mc_word(&asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
            let sut = sut_word(&ops)
                .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
            prop_assert_eq!(sut, mc, "mismatch for {}", asm);
        }

        #[test]
        fn encode_bics_meta_opc_vs_bic(
            rd in reg_num(),
            rn in reg_num(),
            rm in reg_num(),
            is_64 in any::<bool>(),
            kind in shift_kind(),
            use_shift in any::<bool>(),
            amt in 0u32..=63,
        ) {
            let max = if is_64 { 63u32 } else { 31 };
            let amt = if use_shift { amt % (max + 1) } else { 0 };
            let mut ops = vec![
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
                Operand::Reg(gpr(is_64, rm)),
            ];
            if use_shift {
                ops.push(Operand::Shift {
                    kind: kind.to_string(),
                    amount: amt,
                });
            }
            let bics_w = match encode_bics(&ops) {
                Ok(EncodeResult::Word(w)) => w,
                other => panic!("BICS should encode, got {:?}", other),
            };
            let bic_w = match encode_bic(&ops) {
                Ok(EncodeResult::Word(w)) => w,
                other => panic!("BIC should encode, got {:?}", other),
            };
            prop_assert_eq!(
                bics_w ^ bic_w,
                0b11 << 29,
                "BICS opc must be 11 vs BIC opc 00 (bics={:#010x} bic={:#010x})",
                bics_w,
                bic_w
            );
        }

        #[test]
        fn encode_bics_word_layout(
            rd in reg_num(),
            rn in reg_num(),
            rm in reg_num(),
            is_64 in any::<bool>(),
            kind in shift_kind(),
            amt in 0u32..=63,
        ) {
            let max = if is_64 { 63u32 } else { 31 };
            let amt = amt % (max + 1);
            let st = match kind {
                "lsl" => 0u32,
                "lsr" => 1,
                "asr" => 2,
                "ror" => 3,
                _ => 0,
            };
            let ops = [
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
                Operand::Reg(gpr(is_64, rm)),
                Operand::Shift {
                    kind: kind.to_string(),
                    amount: amt,
                },
            ];
            let w = sut_word(&ops).expect("valid BICS must encode");
            let sf = if is_64 { 1u32 } else { 0 };
            prop_assert_eq!((w >> 31) & 1, sf, "sf");
            prop_assert_eq!((w >> 29) & 3, 0b11, "opc");
            prop_assert_eq!((w >> 24) & 0x1f, 0b01010, "opcode 01010");
            prop_assert_eq!((w >> 22) & 3, st, "shift");
            prop_assert_eq!((w >> 21) & 1, 1, "N");
            prop_assert_eq!((w >> 16) & 0x1f, rm, "Rm");
            prop_assert_eq!((w >> 10) & 0x3f, amt, "imm6");
            prop_assert_eq!((w >> 5) & 0x1f, rn, "Rn");
            prop_assert_eq!(w & 0x1f, rd, "Rd");
        }

        #[test]
        fn encode_bics_neg_arity(n in 0usize..=2, is_64 in any::<bool>(), r in 0u32..=30) {
            let ops: Vec<Operand> = (0..n)
                .map(|_| Operand::Reg(gpr(is_64, r)))
                .collect();
            prop_assert!(
                encode_bics(&ops).is_err(),
                "fewer than 3 operands must Err, n={}",
                n
            );
        }

        #[test]
        fn encode_bics_neg_mixed_width(
            rd in 0u32..=30,
            rn in 0u32..=30,
            rm in 0u32..=30,
            rd64 in any::<bool>(),
            rn64 in any::<bool>(),
            rm64 in any::<bool>(),
        ) {
            prop_assume!(!(rd64 == rn64 && rn64 == rm64));
            let ops = [
                Operand::Reg(gpr(rd64, rd)),
                Operand::Reg(gpr(rn64, rn)),
                Operand::Reg(gpr(rm64, rm)),
            ];
            prop_assert!(
                encode_bics(&ops).is_err(),
                "mixed-width BICS registers must Err (rd64={} rn64={} rm64={})",
                rd64,
                rn64,
                rm64
            );
        }

        #[test]
        fn encode_bics_neg_sp_fp(
            which in 0u32..=2,
            is_64 in any::<bool>(),
            a in 0u32..=30,
            b in 0u32..=30,
            kind in 0u32..=8,
            fp_n in prop_oneof![Just(0u32), Just(31u32), 0u32..=31],
        ) {
            let ra = gpr(is_64, a);
            let rb = gpr(is_64, b);
            let bad = match kind {
                0 => if is_64 { "sp".to_string() } else { "wsp".to_string() },
                1 => format!("d{}", fp_n),
                2 => format!("s{}", fp_n),
                3 => format!("q{}", fp_n),
                4 => format!("v{}", fp_n),
                5 => format!("h{}", fp_n),
                6 => format!("b{}", fp_n),
                7 => if is_64 { "sp".to_string() } else { "wsp".to_string() },
                _ => format!("d{}", fp_n),
            };
            let mut names = [ra, rb, bad.clone()];
            names.swap(2, which as usize);
            let ops = [
                Operand::Reg(names[0].clone()),
                Operand::Reg(names[1].clone()),
                Operand::Reg(names[2].clone()),
            ];
            prop_assert!(
                encode_bics(&ops).is_err(),
                "SP/FP name {} at operand {} must Err (names={:?})",
                bad,
                which,
                names
            );
        }

        #[test]
        fn encode_bics_neg_shift_range(
            rd in 0u32..=30,
            rn in 0u32..=30,
            rm in 0u32..=30,
            is_64 in any::<bool>(),
            kind in shift_kind(),
            amt_w in prop_oneof![Just(32u32), Just(33u32), Just(63u32), Just(64u32)],
            amt_x in prop_oneof![Just(64u32), Just(65u32), Just(128u32)],
        ) {
            let amt = if is_64 { amt_x } else { amt_w };
            let ops = [
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
                Operand::Reg(gpr(is_64, rm)),
                Operand::Shift {
                    kind: kind.to_string(),
                    amount: amt,
                },
            ];
            prop_assert!(
                encode_bics(&ops).is_err(),
                "out-of-range {} #{} on {}-bit BICS must Err",
                kind,
                amt,
                if is_64 { 64 } else { 32 }
            );
        }

        #[test]
        fn encode_bics_neg_unknown_shift(
            rd in 0u32..=30,
            rn in 0u32..=30,
            rm in 0u32..=30,
            is_64 in any::<bool>(),
            unknown in prop_oneof![
                Just("lslx".to_string()),
                Just("rrx".to_string()),
                Just("rol".to_string()),
                Just("".to_string()),
                Just("asr ".to_string()),
            ],
            amt_ok in prop_oneof![Just(0u32), Just(1u32), Just(31u32)],
        ) {
            let ops = [
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
                Operand::Reg(gpr(is_64, rm)),
                Operand::Shift {
                    kind: unknown.clone(),
                    amount: amt_ok,
                },
            ];
            prop_assert!(
                encode_bics(&ops).is_err(),
                "unknown shift kind {:?} must Err",
                unknown
            );
        }

        #[test]
        fn encode_bics_neg_invalid_rm(
            is_64 in any::<bool>(),
            rd in 0u32..=30,
            rn in 0u32..=30,
            bad in prop_oneof![
                Just("x32".to_string()),
                Just("w32".to_string()),
                Just("x99".to_string()),
                Just("".to_string()),
                Just("foo".to_string()),
                Just("r0".to_string()),
                Just("x".to_string()),
            ],
        ) {
            let ops = [
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
                Operand::Reg(bad.clone()),
            ];
            prop_assert!(
                encode_bics(&ops).is_err(),
                "invalid rm name {:?} must Err",
                bad
            );
        }

        #[test]
        fn encode_bics_neg_extra_operand(
            is_64 in any::<bool>(),
            rd in 0u32..=30,
            rn in 0u32..=30,
            rm in 0u32..=30,
            which in 0u32..=3,
        ) {
            let extra = match which {
                0 => Operand::Reg(gpr(is_64, 0)),
                1 => Operand::Imm(0),
                2 => Operand::Mem {
                    base: "x0".into(),
                    offset: 0,
                },
                _ => Operand::Symbol("foo".into()),
            };
            let ops = [
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
                Operand::Reg(gpr(is_64, rm)),
                extra,
            ];
            prop_assert!(
                encode_bics(&ops).is_err(),
                "trailing non-shift 4th operand must Err (which={})",
                which
            );
        }
    }

    #[test]
    fn test_encode_bics_regression_mixed_width() {
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("x0".into()),
        ];
        assert!(
            encode_bics(&ops).is_err(),
            "mixed-width BICS w0, w0, x0 must Err (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_bics_regression_sp() {
        let ops = [
            Operand::Reg("wsp".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
        ];
        assert!(
            encode_bics(&ops).is_err(),
            "BICS wsp, w0, w0 must Err; register 31 is WZR not WSP (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_bics_regression_shift32() {
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Shift {
                kind: "lsl".into(),
                amount: 32,
            },
        ];
        assert!(
            encode_bics(&ops).is_err(),
            "BICS w0, w0, w0, lsl #32 must Err; 32-bit shift amount range is [0, 31]"
        );
    }

    #[test]
    fn test_encode_bics_regression_fp_reg() {
        let ops = [
            Operand::Reg("d0".into()),
            Operand::Reg("x1".into()),
            Operand::Reg("x2".into()),
        ];
        assert!(
            encode_bics(&ops).is_err(),
            "BICS d0, x1, x2 must Err; FP/SIMD registers are not BICS GPRs (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_bics_regression_imm() {
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Imm(0xaaaaaaaau32 as i64),
        ];
        match encode_bics(&ops) {
            Ok(EncodeResult::Word(w)) => {
                assert_eq!(
                    w, 0x7200f000u32,
                    "BICS w0, w0, #0xaaaaaaaa must encode as ANDS w0, w0, #0x55555555"
                );
            }
            other => panic!(
                "BICS w0, w0, #0xaaaaaaaa must encode as ANDS #~imm (llvm-mc 0x7200f000), got {:?}",
                other
            ),
        }
    }

    #[test]
    fn test_encode_bics_regression_unknown_shift_kind() {
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Shift {
                kind: "lslx".into(),
                amount: 0,
            },
        ];
        assert!(
            encode_bics(&ops).is_err(),
            "BICS w0, w0, w0, lslx #0 must Err; only lsl/lsr/asr/ror are valid (unknown kinds default to lsl)"
        );
    }

    #[test]
    fn test_encode_bics_regression_extra_operand() {
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("x1".into()),
            Operand::Reg("x2".into()),
            Operand::Reg("x3".into()),
        ];
        assert!(
            encode_bics(&ops).is_err(),
            "BICS x0, x1, x2, x3 must Err; a 4th non-shift operand is not valid (llvm-mc rejects it)"
        );
    }
}

#[cfg(test)]
mod encode_div_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler
    // Evidence: src/backend/arm/assembler/README.md "accepts the same textual assembly that GCC's gas would consume";
    //   encoder/mod.rs "Encodes AArch64 instructions into 32-bit machine code words";
    //   encoder/mod.rs:272-273 udiv/sdiv dispatch; ARM ARM UDIV/SDIV register form
    // Stronger considered:
    //   - State machine: rejected — encode_div is a pure function with no lifecycle
    //   - Algebraic round-trip: rejected — no in-tree UDIV/SDIV decoder
    // Weaker available: algebraic.metamorphic (o1 bit), algebraic.invariant (ARM fields),
    //   negative_error (arity / non-register / extra operand / mixed width / SP)
    // Differential: candidate=encode_div, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=(operands,unsigned)<->asm text `udiv`/`sdiv` Rd, Rn, Rm

    use super::encode_div;
    use super::super::EncodeResult;
    use crate::backend::arm::assembler::parser::Operand;
    use proptest::prelude::*;
    use std::io::Write;
    use std::process::{Command, Stdio};

    const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
    const CASES: u32 = 1000;

    fn cfg() -> ProptestConfig {
        ProptestConfig::with_cases(CASES)
    }

    fn gpr(is_64: bool, n: u32) -> String {
        if n == 31 {
            if is_64 {
                "xzr".into()
            } else {
                "wzr".into()
            }
        } else {
            format!("{}{}", if is_64 { "x" } else { "w" }, n)
        }
    }

    fn mnemonic(unsigned: bool) -> &'static str {
        if unsigned {
            "udiv"
        } else {
            "sdiv"
        }
    }

    fn sut_word(ops: &[Operand], unsigned: bool) -> Result<u32, String> {
        match encode_div(ops, unsigned)? {
            EncodeResult::Word(w) => Ok(w),
            other => Err(format!("expected Word, got {:?}", other)),
        }
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

    fn shift_kind() -> impl Strategy<Value = &'static str> {
        prop::sample::select(vec!["lsl", "lsr", "asr", "ror"])
    }

    fn bad_third() -> impl Strategy<Value = Operand> {
        prop_oneof![
            any::<i64>().prop_map(Operand::Imm),
            Just(Operand::Symbol("foo".into())),
            Just(Operand::Mem {
                base: "x0".into(),
                offset: 8,
            }),
            (shift_kind(), 0u32..=63u32).prop_map(|(k, a)| Operand::Shift {
                kind: k.into(),
                amount: a,
            }),
            Just(Operand::Cond("eq".into())),
        ]
    }

    fn extra_operand() -> impl Strategy<Value = Operand> {
        prop_oneof![
            (0u32..=31).prop_map(|n| Operand::Reg(format!("x{n}"))),
            (shift_kind(), 0u32..=63u32).prop_map(|(k, a)| Operand::Shift {
                kind: k.into(),
                amount: a,
            }),
            any::<i64>().prop_map(Operand::Imm),
        ]
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_div_kat_llvm_mc_udiv_x0_x1_x2() {
        let want = 0x9ac20820u32;
        let mc = llvm_mc_word("udiv x0, x1, x2").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("x1".into()),
            Operand::Reg("x2".into()),
        ];
        let sut = sut_word(&ops, true).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_div_kat_llvm_mc_sdiv_w0_w1_w2() {
        let want = 0x1ac20c20u32;
        let mc = llvm_mc_word("sdiv w0, w1, w2").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Reg("w1".into()),
            Operand::Reg("w2".into()),
        ];
        let sut = sut_word(&ops, false).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    proptest! {
        #![proptest_config(cfg())]

        #[test]
        fn encode_div_diff_gpr_same_width(
            rd in 0u32..=31,
            rn in 0u32..=31,
            rm in 0u32..=31,
            is_64 in any::<bool>(),
            unsigned in any::<bool>(),
        ) {
            let rd_n = gpr(is_64, rd);
            let rn_n = gpr(is_64, rn);
            let rm_n = gpr(is_64, rm);
            let asm = format!(
                "{} {}, {}, {}",
                mnemonic(unsigned), rd_n, rn_n, rm_n
            );
            let ops = [
                Operand::Reg(rd_n),
                Operand::Reg(rn_n),
                Operand::Reg(rm_n),
            ];
            let sut = sut_word(&ops, unsigned)
                .unwrap_or_else(|e| panic!("SUT rejected valid DIV {}: {}", asm, e));
            let mc = llvm_mc_word(&asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid DIV {}: {}", asm, e));
            prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
        }

        #[test]
        fn encode_div_metamorphic_o1_bit(
            rd in 0u32..=31,
            rn in 0u32..=31,
            rm in 0u32..=31,
            is_64 in any::<bool>(),
        ) {
            let ops = [
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
                Operand::Reg(gpr(is_64, rm)),
            ];
            let udiv = sut_word(&ops, true)
                .unwrap_or_else(|e| panic!("udiv rejected: {}", e));
            let sdiv = sut_word(&ops, false)
                .unwrap_or_else(|e| panic!("sdiv rejected: {}", e));
            prop_assert_eq!(
                udiv ^ sdiv,
                1u32 << 10,
                "UDIV vs SDIV must differ only by o1 bit 10 (udiv={:#010x} sdiv={:#010x})",
                udiv,
                sdiv
            );
        }

        #[test]
        fn encode_div_invariant_arm_fields(
            rd in 0u32..=31,
            rn in 0u32..=31,
            rm in 0u32..=31,
            is_64 in any::<bool>(),
            unsigned in any::<bool>(),
        ) {
            let ops = [
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
                Operand::Reg(gpr(is_64, rm)),
            ];
            let w = sut_word(&ops, unsigned)
                .unwrap_or_else(|e| panic!("DIV rejected: {}", e));
            let sf = if is_64 { 1u32 } else { 0 };
            let o1 = if unsigned { 0u32 } else { 1 };
            prop_assert_eq!(w & 0x1F, rd, "Rd field");
            prop_assert_eq!((w >> 5) & 0x1F, rn, "Rn field");
            prop_assert_eq!((w >> 16) & 0x1F, rm, "Rm field");
            prop_assert_eq!((w >> 31) & 1, sf, "sf bit");
            prop_assert_eq!((w >> 30) & 1, 0, "bit 30 must be 0");
            prop_assert_eq!((w >> 29) & 1, 0, "S bit must be 0");
            prop_assert_eq!((w >> 21) & 0xFF, 0b11010110u32, "opcode bits 28:21");
            prop_assert_eq!((w >> 11) & 0x1F, 0b00001u32, "bits 15:11 must be 00001");
            prop_assert_eq!((w >> 10) & 1, o1, "o1 bit 10");
        }

        #[test]
        fn encode_div_neg_too_few_operands(
            n in 0usize..=2,
            unsigned in any::<bool>(),
            is_64 in any::<bool>(),
            r0 in 0u32..=31,
            r1 in 0u32..=31,
        ) {
            let all = [
                Operand::Reg(gpr(is_64, r0)),
                Operand::Reg(gpr(is_64, r1)),
            ];
            let ops = &all[..n.min(2)];
            prop_assert!(
                encode_div(ops, unsigned).is_err(),
                "fewer than 3 operands must Err, n={}",
                n
            );
        }

        #[test]
        fn encode_div_neg_non_register(
            rd in 0u32..=30,
            rn in 0u32..=30,
            is_64 in any::<bool>(),
            unsigned in any::<bool>(),
            which in 0u32..=2,
            bad in bad_third(),
        ) {
            let mut ops = vec![
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
                Operand::Reg(gpr(is_64, rd)),
            ];
            ops[which as usize] = bad;
            prop_assert!(
                encode_div(&ops, unsigned).is_err(),
                "non-register operand at position {} must Err",
                which
            );
        }

        #[test]
        fn encode_div_neg_extra_operand(
            rd in 0u32..=30,
            rn in 0u32..=30,
            rm in 0u32..=30,
            is_64 in any::<bool>(),
            unsigned in any::<bool>(),
            extra in extra_operand(),
        ) {
            let ops = vec![
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
                Operand::Reg(gpr(is_64, rm)),
                extra,
            ];
            prop_assert!(
                encode_div(&ops, unsigned).is_err(),
                "UDIV/SDIV have no 4th operand; extra operand must Err"
            );
        }

        #[test]
        fn encode_div_neg_mixed_width(
            rd in 0u32..=30,
            rn in 0u32..=30,
            rm in 0u32..=30,
            rd64 in any::<bool>(),
            rn64 in any::<bool>(),
            rm64 in any::<bool>(),
            unsigned in any::<bool>(),
        ) {
            prop_assume!(!(rd64 == rn64 && rn64 == rm64));
            let ops = [
                Operand::Reg(gpr(rd64, rd)),
                Operand::Reg(gpr(rn64, rn)),
                Operand::Reg(gpr(rm64, rm)),
            ];
            prop_assert!(
                encode_div(&ops, unsigned).is_err(),
                "mixed-width DIV registers must Err (rd64={} rn64={} rm64={})",
                rd64,
                rn64,
                rm64
            );
        }

        #[test]
        fn encode_div_neg_invalid_reg_name(
            which in 0u32..=2,
            unsigned in any::<bool>(),
            bad in prop_oneof![
                Just("x32".to_string()),
                Just("w32".to_string()),
                Just("x99".to_string()),
                Just("w99".to_string()),
                Just("".to_string()),
                Just("foo".to_string()),
                Just("r0".to_string()),
                Just("x".to_string()),
                Just("x-1".to_string()),
            ],
        ) {
            let mut ops = vec![
                Operand::Reg("x0".into()),
                Operand::Reg("x1".into()),
                Operand::Reg("x2".into()),
            ];
            ops[which as usize] = Operand::Reg(bad.clone());
            prop_assert!(
                encode_div(&ops, unsigned).is_err(),
                "invalid register name {:?} at {} must Err",
                bad,
                which
            );
        }

        #[test]
        fn encode_div_neg_fp(
            which in 0u32..=2,
            unsigned in any::<bool>(),
            prefix in prop::sample::select(vec!["d", "s", "q", "v", "h", "b"]),
            n in prop_oneof![Just(0u32), Just(31u32), 0u32..=31],
        ) {
            let fp = format!("{}{}", prefix, n);
            let mut ops = vec![
                Operand::Reg("x0".into()),
                Operand::Reg("x1".into()),
                Operand::Reg("x2".into()),
            ];
            ops[which as usize] = Operand::Reg(fp.clone());
            prop_assert!(
                encode_div(&ops, unsigned).is_err(),
                "FP/SIMD register {} is not a valid DIV operand (which={})",
                fp,
                which
            );
        }

        #[test]
        fn encode_div_neg_sp(
            which in 0u32..=2,
            is_64 in any::<bool>(),
            unsigned in any::<bool>(),
            a in 0u32..=30,
            b in 0u32..=30,
        ) {
            let sp = if is_64 { "sp" } else { "wsp" };
            let ra = gpr(is_64, a);
            let rb = gpr(is_64, b);
            let mut names = [ra, rb, sp.to_string()];
            names.swap(2, which as usize);
            let ops = [
                Operand::Reg(names[0].clone()),
                Operand::Reg(names[1].clone()),
                Operand::Reg(names[2].clone()),
            ];
            prop_assert!(
                encode_div(&ops, unsigned).is_err(),
                "SP/WSP is not a valid DIV operand (which={} names={:?})",
                which,
                names
            );
        }
    }

    #[test]
    fn test_encode_div_regression_extra_operand() {
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("x0".into()),
        ];
        assert!(
            encode_div(&ops, false).is_err(),
            "SDIV w0, w0, w0, x0 must Err; a 4th operand is not valid (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_div_regression_mixed_width() {
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("x0".into()),
        ];
        assert!(
            encode_div(&ops, false).is_err(),
            "mixed-width SDIV w0, w0, x0 must Err (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_div_regression_sp() {
        let ops = [
            Operand::Reg("wsp".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
        ];
        assert!(
            encode_div(&ops, false).is_err(),
            "SDIV wsp, w0, w0 must Err; register 31 is WZR not WSP (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_div_regression_fp_reg() {
        let ops = [
            Operand::Reg("d0".into()),
            Operand::Reg("x1".into()),
            Operand::Reg("x2".into()),
        ];
        assert!(
            encode_div(&ops, true).is_err(),
            "UDIV d0, x1, x2 must Err; FP/SIMD registers are not DIV operands (llvm-mc rejects it)"
        );
    }
}

#[cfg(test)]
mod encode_eon_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler
    // Evidence: src/backend/arm/assembler/README.md "accepts the same textual assembly that GCC's gas would consume";
    //   encoder/mod.rs:1-7 "Encodes AArch64 instructions into 32-bit machine code words";
    //   encoder/mod.rs:236 "eon" => encode_eon; ARM ARM Logical (shifted register) EON opc=10 N=1
    // Stronger considered:
    //   - State machine: rejected — encode_eon is a pure function with no lifecycle
    //   - Algebraic round-trip: rejected — no in-tree EON decoder
    // Weaker available: algebraic.metamorphic (N XOR vs EOR), algebraic.invariant (word layout),
    //   negative_error (arity / mixed width / SP / FP / shift range / extra operand)
    // Differential: candidate=encode_eon, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=operands <-> asm text `eon Rd, Rn, Rm{, shift}` / `eon Rd, Rn, #imm` (EOR #~imm alias)

    use super::{encode_eon, encode_logical};
    use super::super::EncodeResult;
    use crate::backend::arm::assembler::parser::Operand;
    use proptest::prelude::*;
    use std::io::Write;
    use std::process::{Command, Stdio};

    const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
    const CASES: u32 = 1000;

    fn cfg() -> ProptestConfig {
        ProptestConfig::with_cases(CASES)
    }

    fn gpr(is_64: bool, n: u32) -> String {
        if n == 31 {
            if is_64 {
                "xzr".into()
            } else {
                "wzr".into()
            }
        } else {
            format!("{}{}", if is_64 { "x" } else { "w" }, n)
        }
    }

    fn sut_word(ops: &[Operand]) -> Result<u32, String> {
        match encode_eon(ops)? {
            EncodeResult::Word(w) => Ok(w),
            other => Err(format!("expected Word, got {:?}", other)),
        }
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
        let out = child
            .wait_with_output()
            .map_err(|e| format!("wait llvm-mc: {e}"))?;
        let stdout = String::from_utf8_lossy(&out.stdout);
        let stderr = String::from_utf8_lossy(&out.stderr);
        if !out.status.success() || stderr.contains("error:") {
            return Err(format!("llvm-mc error: {stderr}"));
        }
        parse_llvm_encoding(&stdout)
    }

    /// Independent AArch64 logical-immediate constructor (ARM ARM, not encode_bitmask_imm).
    fn bitmask_from_fields(size: u32, ones: u32, immr: u32, is_64: bool) -> u64 {
        let width = if is_64 { 64u32 } else { 32 };
        let mask = if size == 64 {
            u64::MAX
        } else {
            (1u64 << size) - 1
        };
        let base = (1u64 << ones) - 1;
        let elem = if immr % size == 0 {
            base
        } else {
            let r = immr % size;
            ((base >> r) | (base << (size - r))) & mask
        };
        let mut val = 0u64;
        let mut pos = 0u32;
        while pos < width {
            val |= elem << pos;
            pos += size;
        }
        val
    }

    fn reg_num() -> impl Strategy<Value = u32> {
        prop_oneof![Just(0u32), Just(1u32), Just(30u32), Just(31u32), 0u32..=31]
    }

    fn shift_kind() -> impl Strategy<Value = &'static str> {
        prop::sample::select(vec!["lsl", "lsr", "asr", "ror"])
    }

    fn amt_bound() -> impl Strategy<Value = u32> {
        prop_oneof![
            Just(0u32),
            Just(1u32),
            Just(31u32),
            Just(32u32),
            Just(63u32),
            0u32..=63,
        ]
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_eon_kat_llvm_mc_eon_x0_x1_x2() {
        let want = 0xca220020u32;
        let mc = llvm_mc_word("eon x0, x1, x2").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("x1".into()),
            Operand::Reg("x2".into()),
        ];
        let sut = sut_word(&ops).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_eon_kat_llvm_mc_eon_w0_w1_w2() {
        let want = 0x4a220020u32;
        let mc = llvm_mc_word("eon w0, w1, w2").expect("llvm-mc W KAT");
        assert_eq!(mc, want, "llvm-mc W KAT mapping broken");
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Reg("w1".into()),
            Operand::Reg("w2".into()),
        ];
        let sut = sut_word(&ops).expect("SUT W KAT");
        assert_eq!(sut, want);
    }

    proptest! {
        #![proptest_config(cfg())]

        #[test]
        fn encode_eon_diff_reg_llvm_mc(
            rd in reg_num(),
            rn in reg_num(),
            rm in reg_num(),
            is_64 in any::<bool>(),
            kind in shift_kind(),
            use_shift in any::<bool>(),
            amt in amt_bound(),
        ) {
            let max = if is_64 { 63u32 } else { 31 };
            let amt = if use_shift { amt % (max + 1) } else { 0 };
            let rd_n = gpr(is_64, rd);
            let rn_n = gpr(is_64, rn);
            let rm_n = gpr(is_64, rm);
            let mut ops = vec![
                Operand::Reg(rd_n.clone()),
                Operand::Reg(rn_n.clone()),
                Operand::Reg(rm_n.clone()),
            ];
            let mut asm = format!("eon {}, {}, {}", rd_n, rn_n, rm_n);
            if use_shift {
                ops.push(Operand::Shift {
                    kind: kind.to_string(),
                    amount: amt,
                });
                if !(kind == "lsl" && amt == 0) {
                    asm.push_str(&format!(", {} #{}", kind, amt));
                }
            }
            let mc = llvm_mc_word(&asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
            let sut = sut_word(&ops)
                .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
            prop_assert_eq!(sut, mc, "mismatch for {}", asm);
        }

        #[test]
        fn encode_eon_diff_imm_llvm_mc(
            rd in 0u32..=30,
            rn in reg_num(),
            is_64 in any::<bool>(),
            seed in 0u32..10000,
        ) {
            let sizes: [u32; 6] = if is_64 {
                [2, 4, 8, 16, 32, 64]
            } else {
                [2, 4, 8, 16, 32, 32]
            };
            let size = sizes[(seed as usize) % sizes.len()];
            let ones = 1 + (seed / 6) % (size - 1);
            let rot = (seed / 6 / (size - 1).max(1)) % size;
            let m = bitmask_from_fields(size, ones, rot, is_64);
            let eon_u = if is_64 { !m } else { (!(m as u32)) as u64 };
            let imm = eon_u as i64;
            let rd_n = gpr(is_64, rd);
            let rn_n = gpr(is_64, rn);
            let hex = if is_64 {
                format!("#0x{:x}", eon_u)
            } else {
                format!("#0x{:x}", eon_u as u32)
            };
            let asm = format!("eon {}, {}, {}", rd_n, rn_n, hex);
            let ops = [
                Operand::Reg(rd_n),
                Operand::Reg(rn_n),
                Operand::Imm(imm),
            ];
            let mc = llvm_mc_word(&asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
            let sut = sut_word(&ops)
                .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
            prop_assert_eq!(sut, mc, "mismatch for {}", asm);
        }

        #[test]
        fn encode_eon_metamorphic_n_bit_vs_eor(
            rd in reg_num(),
            rn in reg_num(),
            rm in reg_num(),
            is_64 in any::<bool>(),
            kind in shift_kind(),
            use_shift in any::<bool>(),
            amt in amt_bound(),
        ) {
            let max = if is_64 { 63u32 } else { 31 };
            let amt = if use_shift { amt % (max + 1) } else { 0 };
            let mut ops = vec![
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
                Operand::Reg(gpr(is_64, rm)),
            ];
            if use_shift {
                ops.push(Operand::Shift {
                    kind: kind.to_string(),
                    amount: amt,
                });
            }
            let eon_w = match encode_eon(&ops) {
                Ok(EncodeResult::Word(w)) => w,
                other => panic!("EON should encode, got {:?}", other),
            };
            let eor_w = match encode_logical(&ops, 0b10) {
                Ok(EncodeResult::Word(w)) => w,
                other => panic!("EOR should encode, got {:?}", other),
            };
            prop_assert_eq!(
                eon_w ^ eor_w,
                1u32 << 21,
                "EON N must be 1 vs EOR N=0 (eon={:#010x} eor={:#010x})",
                eon_w,
                eor_w
            );
        }

        #[test]
        fn encode_eon_invariant_arm_fields(
            rd in reg_num(),
            rn in reg_num(),
            rm in reg_num(),
            is_64 in any::<bool>(),
            kind in shift_kind(),
            amt in amt_bound(),
        ) {
            let max = if is_64 { 63u32 } else { 31 };
            let amt = amt % (max + 1);
            let st = match kind {
                "lsl" => 0u32,
                "lsr" => 1,
                "asr" => 2,
                "ror" => 3,
                _ => 0,
            };
            let ops = [
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
                Operand::Reg(gpr(is_64, rm)),
                Operand::Shift {
                    kind: kind.to_string(),
                    amount: amt,
                },
            ];
            let w = sut_word(&ops).expect("valid EON must encode");
            let sf = if is_64 { 1u32 } else { 0 };
            prop_assert_eq!((w >> 31) & 1, sf, "sf");
            prop_assert_eq!((w >> 29) & 3, 0b10, "opc");
            prop_assert_eq!((w >> 24) & 0x1f, 0b01010, "opcode 01010");
            prop_assert_eq!((w >> 22) & 3, st, "shift");
            prop_assert_eq!((w >> 21) & 1, 1, "N");
            prop_assert_eq!((w >> 16) & 0x1f, rm, "Rm");
            prop_assert_eq!((w >> 10) & 0x3f, amt, "imm6");
            prop_assert_eq!((w >> 5) & 0x1f, rn, "Rn");
            prop_assert_eq!(w & 0x1f, rd, "Rd");
        }

        #[test]
        fn encode_eon_neg_arity(n in 0usize..=2, is_64 in any::<bool>(), r in 0u32..=30) {
            let ops: Vec<Operand> = (0..n)
                .map(|_| Operand::Reg(gpr(is_64, r)))
                .collect();
            prop_assert!(
                encode_eon(&ops).is_err(),
                "fewer than 3 operands must Err, n={}",
                n
            );
        }

        #[test]
        fn encode_eon_neg_extra_operand(
            is_64 in any::<bool>(),
            rd in 0u32..=30,
            rn in 0u32..=30,
            rm in 0u32..=30,
            which in 0u32..=3,
        ) {
            let extra = match which {
                0 => Operand::Reg(gpr(is_64, 0)),
                1 => Operand::Imm(0),
                2 => Operand::Mem {
                    base: "x0".into(),
                    offset: 0,
                },
                _ => Operand::Symbol("foo".into()),
            };
            let ops = [
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
                Operand::Reg(gpr(is_64, rm)),
                extra,
            ];
            prop_assert!(
                encode_eon(&ops).is_err(),
                "trailing non-shift 4th operand must Err (which={})",
                which
            );
        }

        #[test]
        fn encode_eon_neg_mixed_width(
            rd in 0u32..=30,
            rn in 0u32..=30,
            rm in 0u32..=30,
            rd64 in any::<bool>(),
            rn64 in any::<bool>(),
            rm64 in any::<bool>(),
        ) {
            prop_assume!(!(rd64 == rn64 && rn64 == rm64));
            let ops = [
                Operand::Reg(gpr(rd64, rd)),
                Operand::Reg(gpr(rn64, rn)),
                Operand::Reg(gpr(rm64, rm)),
            ];
            prop_assert!(
                encode_eon(&ops).is_err(),
                "mixed-width EON registers must Err (rd64={} rn64={} rm64={})",
                rd64,
                rn64,
                rm64
            );
        }

        #[test]
        fn encode_eon_neg_sp_fp(
            which in 0u32..=2,
            is_64 in any::<bool>(),
            a in 0u32..=30,
            b in 0u32..=30,
            kind in 0u32..=8,
            fp_n in prop_oneof![Just(0u32), Just(31u32), 0u32..=31],
        ) {
            let ra = gpr(is_64, a);
            let rb = gpr(is_64, b);
            let bad = match kind {
                0 => if is_64 { "sp".to_string() } else { "wsp".to_string() },
                1 => format!("d{}", fp_n),
                2 => format!("s{}", fp_n),
                3 => format!("q{}", fp_n),
                4 => format!("v{}", fp_n),
                5 => format!("h{}", fp_n),
                6 => format!("b{}", fp_n),
                7 => if is_64 { "sp".to_string() } else { "wsp".to_string() },
                _ => format!("d{}", fp_n),
            };
            let mut names = [ra, rb, bad.clone()];
            names.swap(2, which as usize);
            let ops = [
                Operand::Reg(names[0].clone()),
                Operand::Reg(names[1].clone()),
                Operand::Reg(names[2].clone()),
            ];
            prop_assert!(
                encode_eon(&ops).is_err(),
                "SP/FP name {} at operand {} must Err (names={:?})",
                bad,
                which,
                names
            );
        }

        #[test]
        fn encode_eon_neg_shift_range(
            rd in 0u32..=30,
            rn in 0u32..=30,
            rm in 0u32..=30,
            is_64 in any::<bool>(),
            kind in shift_kind(),
            amt_w in prop_oneof![Just(32u32), Just(33u32), Just(63u32), Just(64u32)],
            amt_x in prop_oneof![Just(64u32), Just(65u32), Just(128u32)],
        ) {
            let amt = if is_64 { amt_x } else { amt_w };
            let ops = [
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
                Operand::Reg(gpr(is_64, rm)),
                Operand::Shift {
                    kind: kind.to_string(),
                    amount: amt,
                },
            ];
            prop_assert!(
                encode_eon(&ops).is_err(),
                "out-of-range {} #{} on {}-bit EON must Err",
                kind,
                amt,
                if is_64 { 64 } else { 32 }
            );
        }

        #[test]
        fn encode_eon_neg_unknown_shift(
            rd in 0u32..=30,
            rn in 0u32..=30,
            rm in 0u32..=30,
            is_64 in any::<bool>(),
            unknown in prop_oneof![
                Just("lslx".to_string()),
                Just("rrx".to_string()),
                Just("rol".to_string()),
                Just("".to_string()),
                Just("asr ".to_string()),
            ],
            amt_ok in prop_oneof![Just(0u32), Just(1u32), Just(31u32)],
        ) {
            let ops = [
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
                Operand::Reg(gpr(is_64, rm)),
                Operand::Shift {
                    kind: unknown.clone(),
                    amount: amt_ok,
                },
            ];
            prop_assert!(
                encode_eon(&ops).is_err(),
                "unknown shift kind {:?} must Err",
                unknown
            );
        }

        #[test]
        fn encode_eon_neg_invalid_name(
            is_64 in any::<bool>(),
            rd in 0u32..=30,
            rn in 0u32..=30,
            bad in prop_oneof![
                Just("x32".to_string()),
                Just("w32".to_string()),
                Just("x99".to_string()),
                Just("".to_string()),
                Just("foo".to_string()),
                Just("r0".to_string()),
                Just("x".to_string()),
            ],
        ) {
            let ops = [
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
                Operand::Reg(bad.clone()),
            ];
            prop_assert!(
                encode_eon(&ops).is_err(),
                "invalid rm name {:?} must Err",
                bad
            );
        }
    }

    #[test]
    fn test_encode_eon_regression_imm() {
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Imm(0xaaaaaaaau32 as i64),
        ];
        match encode_eon(&ops) {
            Ok(EncodeResult::Word(w)) => {
                assert_eq!(
                    w, 0x5200f000u32,
                    "EON w0, w0, #0xaaaaaaaa must encode as EOR w0, w0, #0x55555555"
                );
            }
            other => panic!(
                "EON w0, w0, #0xaaaaaaaa must encode as EOR #~imm (llvm-mc 0x5200f000), got {:?}",
                other
            ),
        }
    }

    #[test]
    fn test_encode_eon_regression_extra_operand() {
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("x1".into()),
            Operand::Reg("x2".into()),
            Operand::Reg("x3".into()),
        ];
        assert!(
            encode_eon(&ops).is_err(),
            "EON x0, x1, x2, x3 must Err; a 4th non-shift operand is not valid (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_eon_regression_mixed_width() {
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("x0".into()),
        ];
        assert!(
            encode_eon(&ops).is_err(),
            "mixed-width EON w0, w0, x0 must Err (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_eon_regression_sp() {
        let ops = [
            Operand::Reg("wsp".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
        ];
        assert!(
            encode_eon(&ops).is_err(),
            "EON wsp, w0, w0 must Err; register 31 is WZR not WSP (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_eon_regression_fp_reg() {
        let ops = [
            Operand::Reg("d0".into()),
            Operand::Reg("x1".into()),
            Operand::Reg("x2".into()),
        ];
        assert!(
            encode_eon(&ops).is_err(),
            "EON d0, x1, x2 must Err; FP/SIMD registers are not EON GPRs (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_eon_regression_shift32() {
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Shift {
                kind: "lsl".into(),
                amount: 32,
            },
        ];
        assert!(
            encode_eon(&ops).is_err(),
            "EON w0, w0, w0, lsl #32 must Err; 32-bit shift amount range is [0, 31]"
        );
    }

    #[test]
    fn test_encode_eon_regression_unknown_shift_kind() {
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Shift {
                kind: "lslx".into(),
                amount: 0,
            },
        ];
        assert!(
            encode_eon(&ops).is_err(),
            "EON w0, w0, w0, lslx #0 must Err; only lsl/lsr/asr/ror are valid (unknown kinds default to lsl)"
        );
    }
}

#[cfg(test)]
mod encode_logical_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler
    // Evidence: src/backend/arm/assembler/README.md "accepts the same textual assembly that GCC's gas would consume";
    //   encoder/mod.rs:1-7 "Encodes AArch64 instructions into 32-bit machine code words";
    //   encoder/mod.rs:231-234 and/orr/eor/ands => encode_logical(opc);
    //   ARM ARM Logical (shifted register) sf opc 01010 shift N=0 Rm imm6 Rn Rd;
    //   ARM ARM Logical (immediate) sf opc 100100 N immr imms Rn Rd;
    //   ARM ARM Advanced SIMD logical 0 Q U 01110 size 1 Rm 000111 Rn Rd
    // Stronger considered:
    //   - State machine: rejected — encode_logical is a pure function with no lifecycle
    //   - Algebraic round-trip: rejected — no in-tree AND/ORR/EOR/ANDS decoder
    //   - Same-job sibling encode_bic/orn/eon/bics: rejected — those are N=1 (different job)
    // Weaker available: algebraic.metamorphic (opc bits), algebraic.invariant (word layout),
    //   negative_error (arity / invalid imm / SP / mixed width / FP / shift range / NEON T / ANDS NEON)
    // Differential: candidate=encode_logical, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=(operands,opc) <-> {and|orr|eor|ands} Rd, Rn, Rm{, shift} / #imm / Vd.T,Vn.T,Vm.T

    use super::encode_logical;
    use super::super::EncodeResult;
    use crate::backend::arm::assembler::parser::Operand;
    use proptest::prelude::*;
    use std::io::Write;
    use std::process::{Command, Stdio};

    const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
    const CASES: u32 = 1000;

    fn cfg() -> ProptestConfig {
        ProptestConfig::with_cases(CASES)
    }

    fn mnemonic(opc: u32) -> &'static str {
        match opc & 3 {
            0 => "and",
            1 => "orr",
            2 => "eor",
            _ => "ands",
        }
    }

    fn gpr(is_64: bool, n: u32) -> String {
        if n == 31 {
            if is_64 {
                "xzr".into()
            } else {
                "wzr".into()
            }
        } else {
            format!("{}{}", if is_64 { "x" } else { "w" }, n)
        }
    }

    fn dest_imm(is_64: bool, n: u32, opc: u32) -> String {
        if n == 31 && (opc & 3) != 0b11 {
            if is_64 {
                "sp".into()
            } else {
                "wsp".into()
            }
        } else {
            gpr(is_64, n)
        }
    }

    fn sut_word(ops: &[Operand], opc: u32) -> Result<u32, String> {
        match encode_logical(ops, opc)? {
            EncodeResult::Word(w) => Ok(w),
            other => Err(format!("expected Word, got {:?}", other)),
        }
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
        let out = child
            .wait_with_output()
            .map_err(|e| format!("wait llvm-mc: {e}"))?;
        let stdout = String::from_utf8_lossy(&out.stdout);
        let stderr = String::from_utf8_lossy(&out.stderr);
        if !out.status.success() || stderr.contains("error:") {
            return Err(format!("llvm-mc error: {stderr}"));
        }
        parse_llvm_encoding(&stdout)
    }

    /// Independent AArch64 logical-immediate constructor (ARM ARM, not encode_bitmask_imm).
    fn bitmask_from_fields(size: u32, ones: u32, immr: u32, is_64: bool) -> u64 {
        let width = if is_64 { 64u32 } else { 32 };
        let mask = if size == 64 {
            u64::MAX
        } else {
            (1u64 << size) - 1
        };
        let base = (1u64 << ones) - 1;
        let elem = if immr % size == 0 {
            base
        } else {
            let r = immr % size;
            ((base >> r) | (base << (size - r))) & mask
        };
        let mut val = 0u64;
        let mut pos = 0u32;
        while pos < width {
            val |= elem << pos;
            pos += size;
        }
        val
    }

    fn reg_num() -> impl Strategy<Value = u32> {
        prop_oneof![Just(0u32), Just(1u32), Just(30u32), Just(31u32), 0u32..=31]
    }

    fn opc_all() -> impl Strategy<Value = u32> {
        prop_oneof![Just(0u32), Just(1u32), Just(2u32), Just(3u32)]
    }

    fn opc_neon() -> impl Strategy<Value = u32> {
        prop_oneof![Just(0u32), Just(1u32), Just(2u32)]
    }

    fn shift_kind() -> impl Strategy<Value = &'static str> {
        prop::sample::select(vec!["lsl", "lsr", "asr", "ror"])
    }

    fn neon_arr() -> impl Strategy<Value = &'static str> {
        prop::sample::select(vec!["8b", "16b"])
    }

    fn invalid_arr() -> impl Strategy<Value = &'static str> {
        prop::sample::select(vec!["4s", "8h", "4h", "2s", "2d", "1d", "8s"])
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_logical_kat_llvm_mc_and_x0_x1_x2() {
        let want = 0x8a020020u32;
        let mc = llvm_mc_word("and x0, x1, x2").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("x1".into()),
            Operand::Reg("x2".into()),
        ];
        let sut = sut_word(&ops, 0b00).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_logical_kat_llvm_mc_orr_eor_ands_and_w_imm_neon() {
        let cases: &[(&str, u32)] = &[
            ("orr x0, x1, x2", 0xaa020020),
            ("eor x0, x1, x2", 0xca020020),
            ("ands x0, x1, x2", 0xea020020),
            ("and w0, w1, w2", 0x0a020020),
            ("and x0, x1, #0x1", 0x92400020),
            ("and v0.16b, v1.16b, v2.16b", 0x4e221c20),
            ("and v0.8b, v1.8b, v2.8b", 0x0e221c20),
            ("orr v0.16b, v1.16b, v2.16b", 0x4ea21c20),
            ("eor v0.16b, v1.16b, v2.16b", 0x6e221c20),
            ("and sp, x0, #0x1", 0x9240001f),
        ];
        for (asm, want) in cases {
            let mc = llvm_mc_word(asm).unwrap_or_else(|e| panic!("llvm-mc KAT {asm}: {e}"));
            assert_eq!(mc, *want, "llvm-mc KAT mapping broken for {asm}");
        }
        let and_imm = [
            Operand::Reg("x0".into()),
            Operand::Reg("x1".into()),
            Operand::Imm(1),
        ];
        assert_eq!(sut_word(&and_imm, 0b00).expect("SUT imm KAT"), 0x92400020);
        let neon = [
            Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "16b".into(),
            },
            Operand::RegArrangement {
                reg: "v1".into(),
                arrangement: "16b".into(),
            },
            Operand::RegArrangement {
                reg: "v2".into(),
                arrangement: "16b".into(),
            },
        ];
        assert_eq!(sut_word(&neon, 0b00).expect("SUT neon KAT"), 0x4e221c20);
        let sp_imm = [
            Operand::Reg("sp".into()),
            Operand::Reg("x0".into()),
            Operand::Imm(1),
        ];
        assert_eq!(sut_word(&sp_imm, 0b00).expect("SUT sp imm KAT"), 0x9240001f);
    }

    proptest! {
        #![proptest_config(cfg())]

        #[test]
        fn encode_logical_diff_reg(
            rd in reg_num(),
            rn in reg_num(),
            rm in reg_num(),
            is_64 in any::<bool>(),
            opc in opc_all(),
            kind in shift_kind(),
            use_shift in any::<bool>(),
            amt in 0u32..=63,
        ) {
            let max = if is_64 { 63u32 } else { 31 };
            let amt = if use_shift { amt % (max + 1) } else { 0 };
            let rd_n = gpr(is_64, rd);
            let rn_n = gpr(is_64, rn);
            let rm_n = gpr(is_64, rm);
            let mut ops = vec![
                Operand::Reg(rd_n.clone()),
                Operand::Reg(rn_n.clone()),
                Operand::Reg(rm_n.clone()),
            ];
            let mut asm = format!("{} {}, {}, {}", mnemonic(opc), rd_n, rn_n, rm_n);
            if use_shift {
                ops.push(Operand::Shift {
                    kind: kind.to_string(),
                    amount: amt,
                });
                if !(kind == "lsl" && amt == 0) {
                    asm.push_str(&format!(", {} #{}", kind, amt));
                }
            }
            let mc = llvm_mc_word(&asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid {asm}: {e}"));
            let sut = sut_word(&ops, opc)
                .unwrap_or_else(|e| panic!("SUT rejected valid {asm}: {e}"));
            prop_assert_eq!(sut, mc, "mismatch for {}", asm);
        }

        #[test]
        fn encode_logical_diff_imm(
            rd in reg_num(),
            rn in reg_num(),
            is_64 in any::<bool>(),
            opc in opc_all(),
            seed in 0u32..10000,
        ) {
            let sizes: [u32; 6] = if is_64 {
                [2, 4, 8, 16, 32, 64]
            } else {
                [2, 4, 8, 16, 32, 32]
            };
            let size = sizes[(seed as usize) % sizes.len()];
            let ones = 1 + (seed / 6) % (size - 1);
            let rot = (seed / 6 / (size - 1).max(1)) % size;
            let m = bitmask_from_fields(size, ones, rot, is_64);
            let rd_n = dest_imm(is_64, rd, opc);
            let rn_n = gpr(is_64, rn);
            let hex = if is_64 {
                format!("#0x{:x}", m)
            } else {
                format!("#0x{:x}", m as u32)
            };
            let asm = format!("{} {}, {}, {}", mnemonic(opc), rd_n, rn_n, hex);
            let ops = [
                Operand::Reg(rd_n),
                Operand::Reg(rn_n),
                Operand::Imm(m as i64),
            ];
            let mc = llvm_mc_word(&asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid {asm}: {e}"));
            let sut = sut_word(&ops, opc)
                .unwrap_or_else(|e| panic!("SUT rejected valid {asm}: {e}"));
            prop_assert_eq!(sut, mc, "mismatch for {}", asm);
        }

        #[test]
        fn encode_logical_diff_neon(
            vd in 0u32..=31,
            vn in 0u32..=31,
            vm in 0u32..=31,
            arr in neon_arr(),
            opc in opc_neon(),
        ) {
            let ops = [
                Operand::RegArrangement {
                    reg: format!("v{vd}"),
                    arrangement: arr.to_string(),
                },
                Operand::RegArrangement {
                    reg: format!("v{vn}"),
                    arrangement: arr.to_string(),
                },
                Operand::RegArrangement {
                    reg: format!("v{vm}"),
                    arrangement: arr.to_string(),
                },
            ];
            let asm = format!(
                "{} v{vd}.{arr}, v{vn}.{arr}, v{vm}.{arr}",
                mnemonic(opc)
            );
            let mc = llvm_mc_word(&asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid {asm}: {e}"));
            let sut = sut_word(&ops, opc)
                .unwrap_or_else(|e| panic!("SUT rejected valid {asm}: {e}"));
            prop_assert_eq!(sut, mc, "mismatch for {}", asm);
        }

        #[test]
        fn encode_logical_metamorphic_opc(
            rd in reg_num(),
            rn in reg_num(),
            rm in reg_num(),
            is_64 in any::<bool>(),
            opc1 in opc_all(),
            opc2 in opc_all(),
            kind in shift_kind(),
            use_shift in any::<bool>(),
            amt in 0u32..=63,
        ) {
            let max = if is_64 { 63u32 } else { 31 };
            let amt = if use_shift { amt % (max + 1) } else { 0 };
            let mut ops = vec![
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
                Operand::Reg(gpr(is_64, rm)),
            ];
            if use_shift {
                ops.push(Operand::Shift {
                    kind: kind.to_string(),
                    amount: amt,
                });
            }
            let w1 = match encode_logical(&ops, opc1) {
                Ok(EncodeResult::Word(w)) => w,
                other => panic!("opc1 should encode, got {other:?}"),
            };
            let w2 = match encode_logical(&ops, opc2) {
                Ok(EncodeResult::Word(w)) => w,
                other => panic!("opc2 should encode, got {other:?}"),
            };
            prop_assert_eq!(
                w1 ^ w2,
                (opc1 ^ opc2) << 29,
                "opc bits [30:29] (w1={:#010x} w2={:#010x})", w1, w2
            );
        }

        #[test]
        fn encode_logical_invariant_arm_fields(
            rd in reg_num(),
            rn in reg_num(),
            rm in reg_num(),
            is_64 in any::<bool>(),
            opc in opc_all(),
            kind in shift_kind(),
            use_shift in any::<bool>(),
            amt in 0u32..=63,
        ) {
            let max = if is_64 { 63u32 } else { 31 };
            let amt = if use_shift { amt % (max + 1) } else { 0 };
            let st = match kind {
                "lsl" => 0u32,
                "lsr" => 1,
                "asr" => 2,
                "ror" => 3,
                _ => 0,
            };
            let mut ops = vec![
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
                Operand::Reg(gpr(is_64, rm)),
            ];
            if use_shift {
                ops.push(Operand::Shift {
                    kind: kind.to_string(),
                    amount: amt,
                });
            }
            let w = match encode_logical(&ops, opc) {
                Ok(EncodeResult::Word(w)) => w,
                other => panic!("should encode, got {other:?}"),
            };
            let sf = if is_64 { 1u32 } else { 0 };
            prop_assert_eq!(w >> 31, sf, "sf");
            prop_assert_eq!((w >> 29) & 3, opc, "opc");
            prop_assert_eq!((w >> 24) & 0x1F, 0b01010, "opcode 01010");
            if use_shift {
                prop_assert_eq!((w >> 22) & 3, st, "shift type");
                prop_assert_eq!((w >> 10) & 0x3F, amt, "imm6");
            } else {
                prop_assert_eq!((w >> 22) & 3, 0, "default lsl");
                prop_assert_eq!((w >> 10) & 0x3F, 0, "imm6=0");
            }
            prop_assert_eq!((w >> 21) & 1, 0, "N=0");
            prop_assert_eq!((w >> 16) & 0x1F, rm, "Rm");
            prop_assert_eq!((w >> 5) & 0x1F, rn, "Rn");
            prop_assert_eq!(w & 0x1F, rd, "Rd");
        }

        #[test]
        fn encode_logical_neg_arity(
            opc in opc_all(),
            n in 0usize..3,
            is_64 in any::<bool>(),
            r in reg_num(),
        ) {
            let name = gpr(is_64, r);
            let ops: Vec<Operand> = (0..n).map(|_| Operand::Reg(name.clone())).collect();
            prop_assert!(
                encode_logical(&ops, opc).is_err(),
                "len={} must Err", n
            );
        }

        #[test]
        fn encode_logical_neg_invalid_imm(
            rd in 0u32..=30,
            rn in 0u32..=30,
            is_64 in any::<bool>(),
            opc in opc_all(),
            which in 0u32..6,
        ) {
            let imm: i64 = match which {
                0 => 0,
                1 => -1,
                2 => 0x1234,
                3 => 5,
                4 => 0x1001,
                _ => 0x12345678,
            };
            let rd_n = gpr(is_64, rd);
            let rn_n = gpr(is_64, rn);
            let ops = [
                Operand::Reg(rd_n),
                Operand::Reg(rn_n),
                Operand::Imm(imm),
            ];
            prop_assert!(
                encode_logical(&ops, opc).is_err(),
                "imm={:#x} must Err as non-bitmask", imm
            );
        }

        #[test]
        fn encode_logical_neg_extra_operand(
            rd in 0u32..=30,
            rn in 0u32..=30,
            rm in 0u32..=30,
            extra in 0u32..=30,
            is_64 in any::<bool>(),
            opc in opc_all(),
        ) {
            let ops = [
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
                Operand::Reg(gpr(is_64, rm)),
                Operand::Reg(gpr(is_64, extra)),
            ];
            prop_assert!(
                encode_logical(&ops, opc).is_err(),
                "4th GPR operand must Err (llvm-mc rejects extra operand)"
            );
        }

        #[test]
        fn encode_logical_neg_sp_shifted(
            other in 0u32..=30,
            is_64 in any::<bool>(),
            opc in opc_all(),
            pos in 0u32..3,
        ) {
            let sp = if is_64 { "sp" } else { "wsp" };
            let g = gpr(is_64, other);
            let mut names = [g.clone(), g.clone(), g.clone()];
            names[pos as usize] = sp.to_string();
            let ops = [
                Operand::Reg(names[0].clone()),
                Operand::Reg(names[1].clone()),
                Operand::Reg(names[2].clone()),
            ];
            prop_assert!(
                encode_logical(&ops, opc).is_err(),
                "SP in shifted-register form must Err (llvm-mc rejects)"
            );
        }

        #[test]
        fn encode_logical_neg_mixed_width(
            rd in 0u32..=30,
            rn in 0u32..=30,
            rm in 0u32..=30,
            opc in opc_all(),
            rd64 in any::<bool>(),
            rn64 in any::<bool>(),
            rm64 in any::<bool>(),
        ) {
            prop_assume!(rd64 != rn64 || rd64 != rm64);
            let ops = [
                Operand::Reg(gpr(rd64, rd)),
                Operand::Reg(gpr(rn64, rn)),
                Operand::Reg(gpr(rm64, rm)),
            ];
            prop_assert!(
                encode_logical(&ops, opc).is_err(),
                "mixed X/W must Err"
            );
        }

        #[test]
        fn encode_logical_neg_fp_as_gpr(
            n in 0u32..=31,
            opc in opc_all(),
            prefix in prop::sample::select(vec!["d", "s", "q", "v", "h", "b"]),
            pos in 0u32..3,
        ) {
            let fp = format!("{prefix}{n}");
            let g = gpr(true, n.min(30));
            let mut names = [g.clone(), g.clone(), g.clone()];
            names[pos as usize] = fp;
            let ops = [
                Operand::Reg(names[0].clone()),
                Operand::Reg(names[1].clone()),
                Operand::Reg(names[2].clone()),
            ];
            prop_assert!(
                encode_logical(&ops, opc).is_err(),
                "FP/SIMD name as GPR must Err"
            );
        }

        #[test]
        fn encode_logical_neg_shift_oob(
            rd in 0u32..=30,
            rn in 0u32..=30,
            rm in 0u32..=30,
            is_64 in any::<bool>(),
            opc in opc_all(),
            kind in shift_kind(),
            amt in prop_oneof![Just(32u32), Just(63u32), Just(64u32), Just(65u32), 32u32..=128],
        ) {
            let max = if is_64 { 63u32 } else { 31 };
            prop_assume!(amt > max);
            let ops = [
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
                Operand::Reg(gpr(is_64, rm)),
                Operand::Shift {
                    kind: kind.to_string(),
                    amount: amt,
                },
            ];
            prop_assert!(
                encode_logical(&ops, opc).is_err(),
                "shift amount {} > {} must Err", amt, max
            );
        }

        #[test]
        fn encode_logical_neg_unknown_shift(
            rd in 0u32..=30,
            rn in 0u32..=30,
            rm in 0u32..=30,
            is_64 in any::<bool>(),
            opc in opc_all(),
            kind in prop::sample::select(vec!["lslx", "rol", "uxtw", "", "lsr "]),
        ) {
            let ops = [
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
                Operand::Reg(gpr(is_64, rm)),
                Operand::Shift {
                    kind: kind.to_string(),
                    amount: 0,
                },
            ];
            prop_assert!(
                encode_logical(&ops, opc).is_err(),
                "unknown shift kind {:?} must Err", kind
            );
        }

        #[test]
        fn encode_logical_neg_neon_bad_arr(
            vd in 0u32..=31,
            vn in 0u32..=31,
            vm in 0u32..=31,
            arr in invalid_arr(),
            opc in opc_neon(),
        ) {
            let ops = [
                Operand::RegArrangement {
                    reg: format!("v{vd}"),
                    arrangement: arr.to_string(),
                },
                Operand::RegArrangement {
                    reg: format!("v{vn}"),
                    arrangement: arr.to_string(),
                },
                Operand::RegArrangement {
                    reg: format!("v{vm}"),
                    arrangement: arr.to_string(),
                },
            ];
            prop_assert!(
                encode_logical(&ops, opc).is_err(),
                "NEON T={} must Err (only 8b/16b)", arr
            );
        }

        #[test]
        fn encode_logical_neg_neon_mismatch(
            vd in 0u32..=31,
            vn in 0u32..=31,
            vm in 0u32..=31,
            opc in opc_neon(),
        ) {
            let ops = [
                Operand::RegArrangement {
                    reg: format!("v{vd}"),
                    arrangement: "16b".into(),
                },
                Operand::RegArrangement {
                    reg: format!("v{vn}"),
                    arrangement: "8b".into(),
                },
                Operand::RegArrangement {
                    reg: format!("v{vm}"),
                    arrangement: "16b".into(),
                },
            ];
            prop_assert!(
                encode_logical(&ops, opc).is_err(),
                "mismatched NEON arrangements must Err"
            );
        }

        #[test]
        fn encode_logical_neg_ands_neon(
            vd in 0u32..=31,
            vn in 0u32..=31,
            vm in 0u32..=31,
            arr in neon_arr(),
        ) {
            let ops = [
                Operand::RegArrangement {
                    reg: format!("v{vd}"),
                    arrangement: arr.to_string(),
                },
                Operand::RegArrangement {
                    reg: format!("v{vn}"),
                    arrangement: arr.to_string(),
                },
                Operand::RegArrangement {
                    reg: format!("v{vm}"),
                    arrangement: arr.to_string(),
                },
            ];
            prop_assert!(
                encode_logical(&ops, 0b11).is_err(),
                "ANDS is not a NEON instruction"
            );
        }

        #[test]
        fn encode_logical_metamorphic_sf(
            rd in 0u32..=30,
            rn in 0u32..=30,
            rm in 0u32..=30,
            opc in opc_all(),
        ) {
            let ops64 = [
                Operand::Reg(gpr(true, rd)),
                Operand::Reg(gpr(true, rn)),
                Operand::Reg(gpr(true, rm)),
            ];
            let ops32 = [
                Operand::Reg(gpr(false, rd)),
                Operand::Reg(gpr(false, rn)),
                Operand::Reg(gpr(false, rm)),
            ];
            let w64 = match encode_logical(&ops64, opc) {
                Ok(EncodeResult::Word(w)) => w,
                other => panic!("x form should encode, got {other:?}"),
            };
            let w32 = match encode_logical(&ops32, opc) {
                Ok(EncodeResult::Word(w)) => w,
                other => panic!("w form should encode, got {other:?}"),
            };
            prop_assert_eq!(w64 ^ w32, 1u32 << 31, "sf must be the only differing bit");
        }

        #[test]
        fn encode_logical_neg_unsupported_third(
            opc in opc_all(),
            which in 0u32..4,
        ) {
            let third = match which {
                0 => Operand::Symbol("foo".into()),
                1 => Operand::Mem { base: "x0".into(), offset: 0 },
                2 => Operand::Label("L1".into()),
                _ => Operand::Cond("eq".into()),
            };
            let ops = [
                Operand::Reg("x0".into()),
                Operand::Reg("x1".into()),
                third,
            ];
            prop_assert!(
                encode_logical(&ops, opc).is_err(),
                "third operand not Imm/Reg must Err"
            );
        }

        #[test]
        fn encode_logical_neg_invalid_reg(
            opc in opc_all(),
            pos in 0u32..3,
            name in prop::sample::select(vec!["foo", "x32", "w32", "x", "r0", ""]),
        ) {
            let g = "x0".to_string();
            let mut names = [g.clone(), g.clone(), g.clone()];
            names[pos as usize] = name.to_string();
            let ops = [
                Operand::Reg(names[0].clone()),
                Operand::Reg(names[1].clone()),
                Operand::Reg(names[2].clone()),
            ];
            prop_assert!(
                encode_logical(&ops, opc).is_err(),
                "invalid register name must Err"
            );
        }
    }

    #[test]
    fn test_encode_logical_regression_extra_operand() {
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
        ];
        assert!(
            encode_logical(&ops, 0b00).is_err(),
            "AND w0, w0, w0, w0 must Err; llvm-mc rejects a 4th GPR operand"
        );
    }

    #[test]
    fn test_encode_logical_regression_sp_shifted() {
        let ops = [
            Operand::Reg("wsp".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
        ];
        assert!(
            encode_logical(&ops, 0b00).is_err(),
            "AND wsp, w0, w0 must Err; shifted-register form uses WZR not WSP for 31"
        );
    }

    #[test]
    fn test_encode_logical_regression_mixed_width() {
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("x0".into()),
        ];
        assert!(
            encode_logical(&ops, 0b00).is_err(),
            "AND w0, w0, x0 must Err; mixed X/W is invalid"
        );
    }

    #[test]
    fn test_encode_logical_regression_fp_as_gpr() {
        let ops = [
            Operand::Reg("d0".into()),
            Operand::Reg("x0".into()),
            Operand::Reg("x0".into()),
        ];
        assert!(
            encode_logical(&ops, 0b00).is_err(),
            "AND d0, x0, x0 must Err; FP/SIMD names are not GPRs"
        );
    }

    #[test]
    fn test_encode_logical_regression_shift_oob() {
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Shift {
                kind: "lsl".into(),
                amount: 32,
            },
        ];
        assert!(
            encode_logical(&ops, 0b00).is_err(),
            "AND w0, w0, w0, lsl #32 must Err; 32-bit shift amount range is [0, 31]"
        );
    }

    #[test]
    fn test_encode_logical_regression_unknown_shift() {
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Shift {
                kind: "lslx".into(),
                amount: 0,
            },
        ];
        assert!(
            encode_logical(&ops, 0b00).is_err(),
            "AND w0, w0, w0, lslx #0 must Err; only lsl/lsr/asr/ror are valid"
        );
    }

    #[test]
    fn test_encode_logical_regression_neon_bad_arr() {
        let ops = [
            Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "4s".into(),
            },
            Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "4s".into(),
            },
            Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "4s".into(),
            },
        ];
        assert!(
            encode_logical(&ops, 0b00).is_err(),
            "AND v0.4s, v0.4s, v0.4s must Err; NEON logical T is 8b/16b only"
        );
    }

    #[test]
    fn test_encode_logical_regression_neon_mismatch() {
        let ops = [
            Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "16b".into(),
            },
            Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "8b".into(),
            },
            Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "16b".into(),
            },
        ];
        assert!(
            encode_logical(&ops, 0b00).is_err(),
            "AND v0.16b, v0.8b, v0.16b must Err; arrangements must match"
        );
    }

    #[test]
    fn test_encode_logical_regression_ands_neon() {
        let ops = [
            Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "8b".into(),
            },
            Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "8b".into(),
            },
            Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "8b".into(),
            },
        ];
        assert!(
            encode_logical(&ops, 0b11).is_err(),
            "ANDS v0.8b, v0.8b, v0.8b must Err; ANDS is not a NEON instruction"
        );
    }
}

#[cfg(test)]
mod encode_madd_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler
    // Evidence: src/backend/arm/assembler/README.md "accepts the same textual assembly that GCC's gas would consume";
    //   encoder/mod.rs "Encodes AArch64 instructions into 32-bit machine code words";
    //   encoder/mod.rs:245 madd dispatch; ARM ARM Data-processing (3 source) MADD
    //   sf 00 11011 000 Rm 0 Ra Rn Rd; data_processing.rs:589 MUL is MADD with Ra=XZR
    // Stronger considered:
    //   - State machine: rejected — encode_madd is a pure function with no lifecycle
    //   - Algebraic round-trip: rejected — no in-tree MADD decoder
    //   - encode_msub as differential sibling: rejected — same-job gate fails (o0=1 vs o0=0)
    // Weaker available: algebraic.metamorphic (sf bit), algebraic.invariant (ARM fields),
    //   negative_error (arity / extra operand / mixed width / SP / FP / invalid name)
    // Differential: candidate=encode_madd, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=operands <-> asm text `madd Rd, Rn, Rm, Ra`

    use super::encode_madd;
    use super::super::EncodeResult;
    use crate::backend::arm::assembler::parser::Operand;
    use proptest::prelude::*;
    use std::io::Write;
    use std::process::{Command, Stdio};

    const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
    const CASES: u32 = 1000;

    fn cfg() -> ProptestConfig {
        ProptestConfig::with_cases(CASES)
    }

    fn gpr(is_64: bool, n: u32) -> String {
        if n == 31 {
            if is_64 {
                "xzr".into()
            } else {
                "wzr".into()
            }
        } else {
            format!("{}{}", if is_64 { "x" } else { "w" }, n)
        }
    }

    fn sut_word(ops: &[Operand]) -> Result<u32, String> {
        match encode_madd(ops)? {
            EncodeResult::Word(w) => Ok(w),
            other => Err(format!("expected Word, got {:?}", other)),
        }
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

    fn extra_operand() -> impl Strategy<Value = Operand> {
        prop_oneof![
            (0u32..=31).prop_map(|n| Operand::Reg(format!("x{n}"))),
            Just(Operand::Imm(0)),
            Just(Operand::Shift {
                kind: "lsl".into(),
                amount: 0,
            }),
        ]
    }

    fn non_register() -> impl Strategy<Value = Operand> {
        prop_oneof![
            any::<i64>().prop_map(Operand::Imm),
            Just(Operand::Symbol("foo".into())),
            Just(Operand::Mem {
                base: "x0".into(),
                offset: 8,
            }),
            Just(Operand::Shift {
                kind: "lsl".into(),
                amount: 0,
            }),
            Just(Operand::Cond("eq".into())),
            Just(Operand::Label("L0".into())),
        ]
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_madd_kat_llvm_mc_x0_x1_x2_x3() {
        let want = 0x9b020c20u32;
        let mc = llvm_mc_word("madd x0, x1, x2, x3").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("x1".into()),
            Operand::Reg("x2".into()),
            Operand::Reg("x3".into()),
        ];
        let sut = sut_word(&ops).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_madd_kat_llvm_mc_w0_w1_w2_w3() {
        let want = 0x1b020c20u32;
        let mc = llvm_mc_word("madd w0, w1, w2, w3").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Reg("w1".into()),
            Operand::Reg("w2".into()),
            Operand::Reg("w3".into()),
        ];
        let sut = sut_word(&ops).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_madd_kat_llvm_mc_ra_zr_is_mul() {
        let want = 0x9b027c20u32;
        let mc_mul = llvm_mc_word("mul x0, x1, x2").expect("llvm-mc MUL KAT");
        let mc_madd = llvm_mc_word("madd x0, x1, x2, xzr").expect("llvm-mc MADD ZR KAT");
        assert_eq!(mc_mul, want, "llvm-mc MUL KAT mapping broken");
        assert_eq!(mc_madd, want, "llvm-mc MADD ZR KAT mapping broken");
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("x1".into()),
            Operand::Reg("x2".into()),
            Operand::Reg("xzr".into()),
        ];
        let sut = sut_word(&ops).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    proptest! {
        #![proptest_config(cfg())]

        #[test]
        fn encode_madd_diff_gpr(
            rd in 0u32..=31,
            rn in 0u32..=31,
            rm in 0u32..=31,
            ra in 0u32..=31,
            is_64 in any::<bool>(),
        ) {
            let rd_n = gpr(is_64, rd);
            let rn_n = gpr(is_64, rn);
            let rm_n = gpr(is_64, rm);
            let ra_n = gpr(is_64, ra);
            let asm = format!("madd {}, {}, {}, {}", rd_n, rn_n, rm_n, ra_n);
            let ops = [
                Operand::Reg(rd_n),
                Operand::Reg(rn_n),
                Operand::Reg(rm_n),
                Operand::Reg(ra_n),
            ];
            let sut = sut_word(&ops)
                .unwrap_or_else(|e| panic!("SUT rejected valid MADD {}: {}", asm, e));
            let mc = llvm_mc_word(&asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid MADD {}: {}", asm, e));
            prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
        }

        #[test]
        fn encode_madd_diff_ra_zr_is_mul(
            rd in 0u32..=31,
            rn in 0u32..=31,
            rm in 0u32..=31,
            is_64 in any::<bool>(),
        ) {
            let rd_n = gpr(is_64, rd);
            let rn_n = gpr(is_64, rn);
            let rm_n = gpr(is_64, rm);
            let zr = gpr(is_64, 31);
            let mul_asm = format!("mul {}, {}, {}", rd_n, rn_n, rm_n);
            let madd_asm = format!("madd {}, {}, {}, {}", rd_n, rn_n, rm_n, zr);
            let ops = [
                Operand::Reg(rd_n),
                Operand::Reg(rn_n),
                Operand::Reg(rm_n),
                Operand::Reg(zr),
            ];
            let sut = sut_word(&ops)
                .unwrap_or_else(|e| panic!("SUT rejected valid MADD {}: {}", madd_asm, e));
            let mc_mul = llvm_mc_word(&mul_asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid MUL {}: {}", mul_asm, e));
            let mc_madd = llvm_mc_word(&madd_asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid MADD {}: {}", madd_asm, e));
            prop_assert_eq!(mc_mul, mc_madd, "llvm-mc MUL vs MADD ZR mismatch");
            prop_assert_eq!(sut, mc_mul, "SUT vs llvm-mc MUL mismatch for {}", mul_asm);
        }

        #[test]
        fn encode_madd_metamorphic_sf_bit(
            rd in 0u32..=31,
            rn in 0u32..=31,
            rm in 0u32..=31,
            ra in 0u32..=31,
        ) {
            let x_ops = [
                Operand::Reg(gpr(true, rd)),
                Operand::Reg(gpr(true, rn)),
                Operand::Reg(gpr(true, rm)),
                Operand::Reg(gpr(true, ra)),
            ];
            let w_ops = [
                Operand::Reg(gpr(false, rd)),
                Operand::Reg(gpr(false, rn)),
                Operand::Reg(gpr(false, rm)),
                Operand::Reg(gpr(false, ra)),
            ];
            let xw = sut_word(&x_ops)
                .unwrap_or_else(|e| panic!("64-bit MADD rejected: {}", e));
            let ww = sut_word(&w_ops)
                .unwrap_or_else(|e| panic!("32-bit MADD rejected: {}", e));
            prop_assert_eq!(
                xw ^ ww,
                1u32 << 31,
                "X vs W MADD must differ only by sf bit 31 (x={:#010x} w={:#010x})",
                xw,
                ww
            );
        }

        #[test]
        fn encode_madd_invariant_arm_fields(
            rd in 0u32..=31,
            rn in 0u32..=31,
            rm in 0u32..=31,
            ra in 0u32..=31,
            is_64 in any::<bool>(),
        ) {
            let ops = [
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
                Operand::Reg(gpr(is_64, rm)),
                Operand::Reg(gpr(is_64, ra)),
            ];
            let w = sut_word(&ops)
                .unwrap_or_else(|e| panic!("MADD rejected: {}", e));
            let sf = if is_64 { 1u32 } else { 0 };
            prop_assert_eq!(w & 0x1F, rd, "Rd field");
            prop_assert_eq!((w >> 5) & 0x1F, rn, "Rn field");
            prop_assert_eq!((w >> 10) & 0x1F, ra, "Ra field");
            prop_assert_eq!((w >> 15) & 1, 0, "o0 bit 15 must be 0 (MADD not MSUB)");
            prop_assert_eq!((w >> 16) & 0x1F, rm, "Rm field");
            prop_assert_eq!((w >> 21) & 0x3FF, 0b0011011000u32, "bits 30:21 must be 0011011000");
            prop_assert_eq!((w >> 31) & 1, sf, "sf bit");
        }

        #[test]
        fn encode_madd_diff_lr(
            which in 0u32..=3,
            a in 0u32..=30,
            b in 0u32..=30,
            c in 0u32..=30,
        ) {
            let mut names = [
                gpr(true, a),
                gpr(true, b),
                gpr(true, c),
                "lr".to_string(),
            ];
            names.swap(3, which as usize);
            let asm = format!(
                "madd {}, {}, {}, {}",
                names[0], names[1], names[2], names[3]
            );
            let ops = [
                Operand::Reg(names[0].clone()),
                Operand::Reg(names[1].clone()),
                Operand::Reg(names[2].clone()),
                Operand::Reg(names[3].clone()),
            ];
            let sut = sut_word(&ops)
                .unwrap_or_else(|e| panic!("SUT rejected valid MADD {}: {}", asm, e));
            let mc = llvm_mc_word(&asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid MADD {}: {}", asm, e));
            prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {} (lr alias x30)", asm);
        }

        #[test]
        fn encode_madd_neg_too_few(
            n in 0usize..=3,
            is_64 in any::<bool>(),
            r0 in 0u32..=31,
            r1 in 0u32..=31,
            r2 in 0u32..=31,
        ) {
            let all = [
                Operand::Reg(gpr(is_64, r0)),
                Operand::Reg(gpr(is_64, r1)),
                Operand::Reg(gpr(is_64, r2)),
            ];
            let ops = &all[..n.min(3)];
            prop_assert!(
                encode_madd(ops).is_err(),
                "fewer than 4 operands must Err, n={}",
                n
            );
        }

        #[test]
        fn encode_madd_neg_extra_operand(
            rd in 0u32..=30,
            rn in 0u32..=30,
            rm in 0u32..=30,
            ra in 0u32..=30,
            is_64 in any::<bool>(),
            extra in extra_operand(),
        ) {
            let ops = vec![
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
                Operand::Reg(gpr(is_64, rm)),
                Operand::Reg(gpr(is_64, ra)),
                extra,
            ];
            prop_assert!(
                encode_madd(&ops).is_err(),
                "MADD has no 5th operand; extra operand must Err"
            );
        }

        #[test]
        fn encode_madd_neg_mixed_width(
            rd in 0u32..=30,
            rn in 0u32..=30,
            rm in 0u32..=30,
            ra in 0u32..=30,
            rd64 in any::<bool>(),
            rn64 in any::<bool>(),
            rm64 in any::<bool>(),
            ra64 in any::<bool>(),
        ) {
            prop_assume!(!(rd64 == rn64 && rn64 == rm64 && rm64 == ra64));
            let ops = [
                Operand::Reg(gpr(rd64, rd)),
                Operand::Reg(gpr(rn64, rn)),
                Operand::Reg(gpr(rm64, rm)),
                Operand::Reg(gpr(ra64, ra)),
            ];
            prop_assert!(
                encode_madd(&ops).is_err(),
                "mixed-width MADD registers must Err (rd64={} rn64={} rm64={} ra64={})",
                rd64,
                rn64,
                rm64,
                ra64
            );
        }

        #[test]
        fn encode_madd_neg_sp(
            which in 0u32..=3,
            is_64 in any::<bool>(),
            a in 0u32..=30,
            b in 0u32..=30,
            c in 0u32..=30,
        ) {
            let sp = if is_64 { "sp" } else { "wsp" };
            let mut names = [
                gpr(is_64, a),
                gpr(is_64, b),
                gpr(is_64, c),
                sp.to_string(),
            ];
            names.swap(3, which as usize);
            let ops = [
                Operand::Reg(names[0].clone()),
                Operand::Reg(names[1].clone()),
                Operand::Reg(names[2].clone()),
                Operand::Reg(names[3].clone()),
            ];
            prop_assert!(
                encode_madd(&ops).is_err(),
                "SP/WSP is not a valid MADD operand (which={} names={:?})",
                which,
                names
            );
        }

        #[test]
        fn encode_madd_neg_fp(
            which in 0u32..=3,
            prefix in prop::sample::select(vec!["d", "s", "q", "v", "h", "b"]),
            n in prop_oneof![Just(0u32), Just(31u32), 0u32..=31],
        ) {
            let fp = format!("{}{}", prefix, n);
            let mut ops = vec![
                Operand::Reg("x0".into()),
                Operand::Reg("x1".into()),
                Operand::Reg("x2".into()),
                Operand::Reg("x3".into()),
            ];
            ops[which as usize] = Operand::Reg(fp.clone());
            prop_assert!(
                encode_madd(&ops).is_err(),
                "FP/SIMD register {} is not a valid MADD operand (which={})",
                fp,
                which
            );
        }

        #[test]
        fn encode_madd_neg_invalid_reg(
            which in 0u32..=3,
            bad in prop_oneof![
                Just("x32".to_string()),
                Just("w32".to_string()),
                Just("x99".to_string()),
                Just("w99".to_string()),
                Just("".to_string()),
                Just("foo".to_string()),
                Just("r0".to_string()),
                Just("x".to_string()),
                Just("x-1".to_string()),
            ],
        ) {
            let mut ops = vec![
                Operand::Reg("x0".into()),
                Operand::Reg("x1".into()),
                Operand::Reg("x2".into()),
                Operand::Reg("x3".into()),
            ];
            ops[which as usize] = Operand::Reg(bad.clone());
            prop_assert!(
                encode_madd(&ops).is_err(),
                "invalid register name {:?} at {} must Err",
                bad,
                which
            );
        }

        #[test]
        fn encode_madd_neg_non_register(
            which in 0u32..=3,
            bad in non_register(),
        ) {
            let mut ops = vec![
                Operand::Reg("x0".into()),
                Operand::Reg("x1".into()),
                Operand::Reg("x2".into()),
                Operand::Reg("x3".into()),
            ];
            ops[which as usize] = bad;
            prop_assert!(
                encode_madd(&ops).is_err(),
                "non-register operand at position {} must Err",
                which
            );
        }
    }

    #[test]
    fn test_encode_madd_regression_extra_operand() {
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("x0".into()),
        ];
        assert!(
            encode_madd(&ops).is_err(),
            "MADD w0, w0, w0, w0, x0 must Err; a 5th operand is not valid (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_madd_regression_mixed_width() {
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("x0".into()),
        ];
        assert!(
            encode_madd(&ops).is_err(),
            "mixed-width MADD w0, w0, w0, x0 must Err (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_madd_regression_sp() {
        let ops = [
            Operand::Reg("wsp".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
        ];
        assert!(
            encode_madd(&ops).is_err(),
            "MADD wsp, w0, w0, w0 must Err; register 31 is WZR not WSP (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_madd_regression_fp_reg() {
        let ops = [
            Operand::Reg("d0".into()),
            Operand::Reg("x1".into()),
            Operand::Reg("x2".into()),
            Operand::Reg("x3".into()),
        ];
        assert!(
            encode_madd(&ops).is_err(),
            "MADD d0, x1, x2, x3 must Err; FP/SIMD registers are not MADD operands (llvm-mc rejects it)"
        );
    }
}

#[cfg(test)]
mod encode_movk_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler
    // Evidence: src/backend/arm/assembler/README.md "accepts the same textual assembly that GCC's gas would consume";
    //   encoder/mod.rs "Encodes AArch64 instructions into 32-bit machine code words";
    //   encoder/mod.rs:221 movk dispatch; ARM ARM Move wide (immediate) MOVK
    //   sf 11 100101 hw imm16 Rd; codegen emit.rs:863-928 movk Rd, #imm16 [, lsl #N]
    // Stronger considered:
    //   - State machine: rejected — encode_movk is a pure function with no lifecycle
    //   - Algebraic round-trip: rejected — no in-tree MOVK decoder
    //   - encode_movz / encode_movn as differential sibling: rejected — same-job gate fails
    //     (opc 10/00 vs 11; MOVZ zeros other halfwords, MOVN inverts)
    // Weaker available: algebraic.metamorphic (sf bit), algebraic.invariant (ARM fields),
    //   negative_error (imm16 range / shift / extra operand / SP / FP / arity)
    // Differential: candidate=encode_movk, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=operands <-> asm text `movk Rd, #imm16 [, lsl #N]`

    use super::encode_movk;
    use super::super::EncodeResult;
    use crate::backend::arm::assembler::parser::Operand;
    use proptest::prelude::*;
    use std::io::Write;
    use std::process::{Command, Stdio};

    const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
    const CASES: u32 = 1000;

    fn cfg() -> ProptestConfig {
        ProptestConfig::with_cases(CASES)
    }

    fn gpr(is_64: bool, n: u32) -> String {
        if n == 31 {
            if is_64 {
                "xzr".into()
            } else {
                "wzr".into()
            }
        } else {
            format!("{}{}", if is_64 { "x" } else { "w" }, n)
        }
    }

    fn sut_word(ops: &[Operand]) -> Result<u32, String> {
        match encode_movk(ops)? {
            EncodeResult::Word(w) => Ok(w),
            other => Err(format!("expected Word, got {:?}", other)),
        }
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

    fn movk_asm(rd: &str, imm: i64, hw: u32) -> String {
        if hw == 0 {
            format!("movk {}, #{}", rd, imm)
        } else {
            format!("movk {}, #{}, lsl #{}", rd, imm, hw * 16)
        }
    }

    fn ops_imm(rd: &str, imm: i64, hw: u32) -> Vec<Operand> {
        let mut ops = vec![Operand::Reg(rd.to_string()), Operand::Imm(imm)];
        if hw != 0 {
            ops.push(Operand::Shift {
                kind: "lsl".into(),
                amount: hw * 16,
            });
        }
        ops
    }

    fn imm16() -> impl Strategy<Value = i64> {
        prop_oneof![Just(0i64), Just(1i64), Just(65535i64), 0i64..=65535]
    }

    fn valid_width_hw() -> impl Strategy<Value = (bool, u32)> {
        prop_oneof![
            (Just(true), prop_oneof![Just(0u32), Just(1u32), Just(2u32), Just(3u32)]),
            (Just(false), prop_oneof![Just(0u32), Just(1u32)]),
        ]
    }

    fn oob_imm() -> impl Strategy<Value = i64> {
        prop_oneof![
            Just(-1i64),
            Just(65536i64),
            Just(65537i64),
            Just(-65535i64),
            Just(i64::MIN),
            Just(i64::MAX),
            (65536i64..=0x1_0000_0),
            (i64::MIN..=-1),
        ]
    }

    fn extra_operand() -> impl Strategy<Value = Operand> {
        prop_oneof![
            (0u32..=31).prop_map(|n| Operand::Reg(format!("x{n}"))),
            Just(Operand::Imm(0)),
            Just(Operand::Shift {
                kind: "lsl".into(),
                amount: 16,
            }),
        ]
    }

    fn invalid_name() -> impl Strategy<Value = String> {
        prop::sample::select(vec![
            "foo".into(),
            "x32".into(),
            "w32".into(),
            "x".into(),
            "r0".into(),
            "".into(),
        ])
    }

    fn fp_name() -> impl Strategy<Value = String> {
        prop_oneof![
            (0u32..=31).prop_map(|n| format!("d{n}")),
            (0u32..=31).prop_map(|n| format!("s{n}")),
            (0u32..=31).prop_map(|n| format!("q{n}")),
            (0u32..=31).prop_map(|n| format!("v{n}")),
            (0u32..=31).prop_map(|n| format!("h{n}")),
            (0u32..=31).prop_map(|n| format!("b{n}")),
        ]
    }

    fn abs_g_kind(is_64: bool) -> impl Strategy<Value = &'static str> {
        if is_64 {
            prop::sample::select(vec![
                "abs_g0", "abs_g0_nc", "abs_g1", "abs_g1_nc",
                "abs_g2", "abs_g2_nc", "abs_g3",
            ])
            .boxed()
        } else {
            prop::sample::select(vec!["abs_g0", "abs_g0_nc", "abs_g1", "abs_g1_nc"]).boxed()
        }
    }

    fn abs_g_shift(kind: &str) -> u32 {
        match kind {
            "abs_g0" | "abs_g0_nc" => 0,
            "abs_g1" | "abs_g1_nc" => 16,
            "abs_g2" | "abs_g2_nc" => 32,
            "abs_g3" => 48,
            _ => 0,
        }
    }

    fn invalid_shift_case() -> impl Strategy<Value = (bool, String, u32)> {
        prop_oneof![
            (
                any::<bool>(),
                prop::sample::select(vec!["lsr", "asr", "ror", "lslx", ""]),
                prop_oneof![Just(0u32), Just(16u32), Just(32u32), 0u32..=64],
            )
                .prop_map(|(b, k, a)| (b, k.to_string(), a)),
            (
                any::<bool>(),
                prop::sample::select(vec![1u32, 8, 15, 17, 31, 33, 47, 49, 63, 64]),
            )
                .prop_map(|(b, a)| (b, "lsl".into(), a)),
            prop::sample::select(vec![32u32, 48]).prop_map(|a| (false, "lsl".into(), a)),
        ]
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_movk_kat_llvm_mc() {
        let want_x = 0xf2800540u32;
        let mc_x = llvm_mc_word("movk x0, #42").expect("llvm-mc KAT x");
        assert_eq!(mc_x, want_x, "llvm-mc KAT mapping broken for 64-bit");
        let ops_x = [Operand::Reg("x0".into()), Operand::Imm(42)];
        let sut_x = sut_word(&ops_x).expect("SUT KAT x");
        assert_eq!(sut_x, want_x);

        let want_w = 0x72800540u32;
        let mc_w = llvm_mc_word("movk w0, #42").expect("llvm-mc KAT w");
        assert_eq!(mc_w, want_w, "llvm-mc KAT mapping broken for 32-bit");
        let ops_w = [Operand::Reg("w0".into()), Operand::Imm(42)];
        let sut_w = sut_word(&ops_w).expect("SUT KAT w");
        assert_eq!(sut_w, want_w);

        let want_lsl = 0xf2a00540u32;
        let mc_lsl = llvm_mc_word("movk x0, #42, lsl #16").expect("llvm-mc KAT lsl");
        assert_eq!(mc_lsl, want_lsl, "llvm-mc KAT mapping broken for lsl #16");
        let ops_lsl = [
            Operand::Reg("x0".into()),
            Operand::Imm(42),
            Operand::Shift {
                kind: "lsl".into(),
                amount: 16,
            },
        ];
        let sut_lsl = sut_word(&ops_lsl).expect("SUT KAT lsl");
        assert_eq!(sut_lsl, want_lsl);
    }

    proptest! {
        #![proptest_config(cfg())]

        #[test]
        fn encode_movk_diff_imm_shift(
            rd in 0u32..=31,
            (is_64, hw) in valid_width_hw(),
            imm in imm16(),
            explicit_lsl0 in any::<bool>(),
        ) {
            let rd_n = gpr(is_64, rd);
            let mut ops = vec![Operand::Reg(rd_n.clone()), Operand::Imm(imm)];
            let asm = if hw == 0 && !explicit_lsl0 {
                format!("movk {}, #{}", rd_n, imm)
            } else {
                ops.push(Operand::Shift {
                    kind: "lsl".into(),
                    amount: hw * 16,
                });
                format!("movk {}, #{}, lsl #{}", rd_n, imm, hw * 16)
            };
            let mc = llvm_mc_word(&asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid {asm}: {e}"));
            let sut = sut_word(&ops)
                .unwrap_or_else(|e| panic!("SUT rejected valid {asm}: {e}"));
            prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
        }

        #[test]
        fn encode_movk_metamorphic_sf(
            rd in 0u32..=31,
            imm in imm16(),
            hw in 0u32..=1,
        ) {
            let x_ops = ops_imm(&gpr(true, rd), imm, hw);
            let w_ops = ops_imm(&gpr(false, rd), imm, hw);
            let xw = sut_word(&x_ops)
                .unwrap_or_else(|e| panic!("64-bit MOVK rejected: {e}"));
            let ww = sut_word(&w_ops)
                .unwrap_or_else(|e| panic!("32-bit MOVK rejected: {e}"));
            prop_assert_eq!(
                xw ^ ww,
                1u32 << 31,
                "X vs W MOVK must differ only by sf bit 31 (x={:#010x} w={:#010x})",
                xw,
                ww
            );
        }

        #[test]
        fn encode_movk_invariant_arm_fields(
            rd in 0u32..=31,
            (is_64, hw) in valid_width_hw(),
            imm in imm16(),
        ) {
            let ops = ops_imm(&gpr(is_64, rd), imm, hw);
            let w = sut_word(&ops).unwrap_or_else(|e| panic!("MOVK rejected: {e}"));
            let sf = if is_64 { 1u32 } else { 0 };
            prop_assert_eq!(w & 0x1F, rd, "Rd field");
            prop_assert_eq!((w >> 5) & 0xFFFF, imm as u32, "imm16 field");
            prop_assert_eq!((w >> 21) & 0x3, hw, "hw field");
            prop_assert_eq!((w >> 23) & 0xFF, 0b11100101u32, "bits 30:23 must be 11100101 (opc=11, 100101)");
            prop_assert_eq!((w >> 31) & 1, sf, "sf bit");
        }

        #[test]
        fn encode_movk_diff_abs_g(
            rd in 0u32..=31,
            is_64 in any::<bool>(),
            val in any::<i64>(),
            kind in abs_g_kind(true),
        ) {
            // Co-generate kind valid for width: drop g2/g3 on W.
            let kind = if !is_64 && abs_g_shift(kind) >= 32 {
                if kind.contains('1') { "abs_g1" } else { "abs_g0" }
            } else {
                kind
            };
            let shift = abs_g_shift(kind);
            let hw = shift / 16;
            let chunk = ((val as u64) >> shift) as i64 & 0xFFFF;
            let rd_n = gpr(is_64, rd);
            let ops = [
                Operand::Reg(rd_n.clone()),
                Operand::Modifier {
                    kind: kind.to_string(),
                    symbol: val.to_string(),
                },
            ];
            let asm = movk_asm(&rd_n, chunk, hw);
            let mc = llvm_mc_word(&asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected resolved {asm}: {e}"));
            let sut = sut_word(&ops).unwrap_or_else(|e| {
                panic!("SUT rejected abs_g {kind} val={val} -> {asm}: {e}")
            });
            prop_assert_eq!(
                sut, mc,
                "abs_g {} val={} must match resolved {}",
                kind,
                val,
                asm
            );
        }

        #[test]
        fn encode_movk_neg_imm_oob(
            rd in 0u32..=31,
            is_64 in any::<bool>(),
            imm in oob_imm(),
        ) {
            let ops = [Operand::Reg(gpr(is_64, rd)), Operand::Imm(imm)];
            prop_assert!(
                encode_movk(&ops).is_err(),
                "imm {} outside [0, 65535] must Err (llvm-mc rejects it)",
                imm
            );
        }

        #[test]
        fn encode_movk_neg_invalid_shift(
            rd in 0u32..=31,
            imm in imm16(),
            (is_64, kind, amount) in invalid_shift_case(),
        ) {
            let ops = [
                Operand::Reg(gpr(is_64, rd)),
                Operand::Imm(imm),
                Operand::Shift {
                    kind: kind.clone(),
                    amount,
                },
            ];
            prop_assert!(
                encode_movk(&ops).is_err(),
                "movk with shift {} #{} on {} must Err",
                kind,
                amount,
                gpr(is_64, rd)
            );
        }

        #[test]
        fn encode_movk_neg_too_few(
            n in 0usize..=1,
            rd in 0u32..=31,
            is_64 in any::<bool>(),
            imm in imm16(),
        ) {
            let all = [Operand::Reg(gpr(is_64, rd)), Operand::Imm(imm)];
            let ops = &all[..n];
            prop_assert!(
                encode_movk(ops).is_err(),
                "fewer than 2 operands must Err, n={}",
                n
            );
        }

        #[test]
        fn encode_movk_neg_extra_operand(
            rd in 0u32..=31,
            (is_64, hw) in valid_width_hw(),
            imm in imm16(),
            extra in extra_operand(),
        ) {
            let mut ops = ops_imm(&gpr(is_64, rd), imm, hw);
            ops.push(extra);
            prop_assert!(
                encode_movk(&ops).is_err(),
                "MOVK has no operand after optional lsl; extra operand must Err"
            );
        }

        #[test]
        fn encode_movk_neg_sp(
            is_64 in any::<bool>(),
            imm in imm16(),
            hw in 0u32..=1,
        ) {
            let name = if is_64 { "sp" } else { "wsp" };
            let ops = ops_imm(name, imm, hw);
            prop_assert!(
                encode_movk(&ops).is_err(),
                "MOVK {} must Err; register 31 is ZR not SP (llvm-mc rejects it)",
                name
            );
        }

        #[test]
        fn encode_movk_neg_fp(
            fp in fp_name(),
            imm in imm16(),
        ) {
            let ops = [Operand::Reg(fp.clone()), Operand::Imm(imm)];
            prop_assert!(
                encode_movk(&ops).is_err(),
                "MOVK {} must Err; FP/SIMD names are not GPRs (llvm-mc rejects it)",
                fp
            );
        }

        #[test]
        fn encode_movk_neg_invalid_name(
            name in invalid_name(),
            imm in imm16(),
        ) {
            let ops = [Operand::Reg(name.clone()), Operand::Imm(imm)];
            prop_assert!(
                encode_movk(&ops).is_err(),
                "invalid register name {:?} must Err",
                name
            );
        }

        #[test]
        fn encode_movk_neg_bad_second(
            rd in 0u32..=31,
            is_64 in any::<bool>(),
            second in prop_oneof![
                Just(Operand::Modifier {
                    kind: "lo12".into(),
                    symbol: "0".into(),
                }),
                Just(Operand::Modifier {
                    kind: "abs_g0".into(),
                    symbol: "foo".into(),
                }),
                Just(Operand::Symbol("sym".into())),
                Just(Operand::Label("L0".into())),
                Just(Operand::Mem {
                    base: "x0".into(),
                    offset: 0,
                }),
                Just(Operand::Reg("x1".into())),
            ],
        ) {
            let ops = [Operand::Reg(gpr(is_64, rd)), second];
            prop_assert!(
                encode_movk(&ops).is_err(),
                "second operand must be imm16 or a resolvable abs_g modifier"
            );
        }

        #[test]
        fn encode_movk_diff_lr(
            imm in imm16(),
            hw in 0u32..=3,
        ) {
            let ops = ops_imm("lr", imm, hw);
            let asm = movk_asm("lr", imm, hw);
            let sut = sut_word(&ops)
                .unwrap_or_else(|e| panic!("SUT rejected valid {asm}: {e}"));
            let mc = llvm_mc_word(&asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid {asm}: {e}"));
            prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {} (lr alias x30)", asm);
        }
    }

    #[test]
    fn test_encode_movk_regression_imm_oob() {
        let ops = [Operand::Reg("w0".into()), Operand::Imm(-1)];
        assert!(
            encode_movk(&ops).is_err(),
            "MOVK w0, #-1 must Err; imm16 range is [0, 65535] (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_movk_regression_invalid_shift() {
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Imm(0),
            Operand::Shift {
                kind: "lsr".into(),
                amount: 0,
            },
        ];
        assert!(
            encode_movk(&ops).is_err(),
            "MOVK w0, #0, lsr #0 must Err; only lsl with 0/16 (W) or 0/16/32/48 (X) is valid"
        );
    }

    #[test]
    fn test_encode_movk_regression_extra_operand() {
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Imm(0),
            Operand::Reg("x0".into()),
        ];
        assert!(
            encode_movk(&ops).is_err(),
            "MOVK x0, #0, x0 must Err; extra operand is invalid (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_movk_regression_sp() {
        let ops = [Operand::Reg("wsp".into()), Operand::Imm(0)];
        assert!(
            encode_movk(&ops).is_err(),
            "MOVK wsp, #0 must Err; register 31 is WZR not WSP (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_movk_regression_fp() {
        let ops = [Operand::Reg("d0".into()), Operand::Imm(0)];
        assert!(
            encode_movk(&ops).is_err(),
            "MOVK d0, #0 must Err; FP/SIMD registers are not MOVK operands (llvm-mc rejects it)"
        );
    }
}

#[cfg(test)]
mod encode_movn_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler
    // Evidence: src/backend/arm/assembler/README.md "accepts the same textual assembly that GCC's gas would consume";
    //   encoder/mod.rs "Encodes AArch64 instructions into 32-bit machine code words";
    //   encoder/mod.rs:222 movn dispatch; ARM ARM Move wide (immediate) MOVN
    //   sf 00 100101 hw imm16 Rd; codegen emit.rs:873-902 movn Rd, #imm16 [, lsl #N]
    // Stronger considered:
    //   - State machine: rejected — encode_movn is a pure function with no lifecycle
    //   - Algebraic round-trip: rejected — no in-tree MOVN decoder
    //   - encode_movz / encode_movk as differential sibling: rejected — same-job gate fails
    //     (opc 10/11 vs 00; MOVZ zeros other halfwords, MOVK keeps them)
    // Weaker available: algebraic.metamorphic (sf bit), algebraic.invariant (ARM fields),
    //   negative_error (imm16 range / shift / extra operand / SP)
    // Differential: candidate=encode_movn, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=operands <-> asm text `movn Rd, #imm16 [, lsl #N]`

    use super::encode_movn;
    use super::super::EncodeResult;
    use crate::backend::arm::assembler::parser::Operand;
    use proptest::prelude::*;
    use std::io::Write;
    use std::process::{Command, Stdio};

    const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
    const CASES: u32 = 1000;

    fn cfg() -> ProptestConfig {
        ProptestConfig::with_cases(CASES)
    }

    fn gpr(is_64: bool, n: u32) -> String {
        if n == 31 {
            if is_64 {
                "xzr".into()
            } else {
                "wzr".into()
            }
        } else {
            format!("{}{}", if is_64 { "x" } else { "w" }, n)
        }
    }

    fn sut_word(ops: &[Operand]) -> Result<u32, String> {
        match encode_movn(ops)? {
            EncodeResult::Word(w) => Ok(w),
            other => Err(format!("expected Word, got {:?}", other)),
        }
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

    fn movn_asm(rd: &str, imm: i64, hw: u32) -> String {
        if hw == 0 {
            format!("movn {}, #{}", rd, imm)
        } else {
            format!("movn {}, #{}, lsl #{}", rd, imm, hw * 16)
        }
    }

    fn ops_imm(rd: &str, imm: i64, hw: u32) -> Vec<Operand> {
        let mut ops = vec![Operand::Reg(rd.to_string()), Operand::Imm(imm)];
        if hw != 0 {
            ops.push(Operand::Shift {
                kind: "lsl".into(),
                amount: hw * 16,
            });
        }
        ops
    }

    fn imm16() -> impl Strategy<Value = i64> {
        prop_oneof![Just(0i64), Just(1i64), Just(65535i64), 0i64..=65535]
    }

    fn valid_width_hw() -> impl Strategy<Value = (bool, u32)> {
        prop_oneof![
            (Just(true), prop_oneof![Just(0u32), Just(1u32), Just(2u32), Just(3u32)]),
            (Just(false), prop_oneof![Just(0u32), Just(1u32)]),
        ]
    }

    fn oob_imm() -> impl Strategy<Value = i64> {
        prop_oneof![
            Just(-1i64),
            Just(65536i64),
            Just(65537i64),
            Just(-65535i64),
            Just(i64::MIN),
            Just(i64::MAX),
            (65536i64..=0x1_0000_0),
            (i64::MIN..=-1),
        ]
    }

    fn extra_operand() -> impl Strategy<Value = Operand> {
        prop_oneof![
            (0u32..=31).prop_map(|n| Operand::Reg(format!("x{n}"))),
            Just(Operand::Imm(0)),
            Just(Operand::Shift {
                kind: "lsl".into(),
                amount: 16,
            }),
        ]
    }

    fn invalid_shift_case() -> impl Strategy<Value = (bool, String, u32)> {
        prop_oneof![
            (
                any::<bool>(),
                prop::sample::select(vec!["lsr", "asr", "ror", "lslx", ""]),
                prop_oneof![Just(0u32), Just(16u32), Just(32u32), 0u32..=64],
            )
                .prop_map(|(b, k, a)| (b, k.to_string(), a)),
            (
                any::<bool>(),
                prop::sample::select(vec![1u32, 8, 15, 17, 31, 33, 47, 49, 63, 64]),
            )
                .prop_map(|(b, a)| (b, "lsl".into(), a)),
            prop::sample::select(vec![32u32, 48]).prop_map(|a| (false, "lsl".into(), a)),
        ]
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_movn_kat_llvm_mc() {
        let want_x = 0x92800540u32;
        let mc_x = llvm_mc_word("movn x0, #42").expect("llvm-mc KAT x");
        assert_eq!(mc_x, want_x, "llvm-mc KAT mapping broken for 64-bit");
        let ops_x = [Operand::Reg("x0".into()), Operand::Imm(42)];
        let sut_x = sut_word(&ops_x).expect("SUT KAT x");
        assert_eq!(sut_x, want_x);

        let want_w = 0x12800540u32;
        let mc_w = llvm_mc_word("movn w0, #42").expect("llvm-mc KAT w");
        assert_eq!(mc_w, want_w, "llvm-mc KAT mapping broken for 32-bit");
        let ops_w = [Operand::Reg("w0".into()), Operand::Imm(42)];
        let sut_w = sut_word(&ops_w).expect("SUT KAT w");
        assert_eq!(sut_w, want_w);

        let want_lsl = 0x92a00540u32;
        let mc_lsl = llvm_mc_word("movn x0, #42, lsl #16").expect("llvm-mc KAT lsl");
        assert_eq!(mc_lsl, want_lsl, "llvm-mc KAT mapping broken for lsl #16");
        let ops_lsl = [
            Operand::Reg("x0".into()),
            Operand::Imm(42),
            Operand::Shift {
                kind: "lsl".into(),
                amount: 16,
            },
        ];
        let sut_lsl = sut_word(&ops_lsl).expect("SUT KAT lsl");
        assert_eq!(sut_lsl, want_lsl);
    }

    proptest! {
        #![proptest_config(cfg())]

        #[test]
        fn encode_movn_diff_imm_shift(
            rd in 0u32..=31,
            (is_64, hw) in valid_width_hw(),
            imm in imm16(),
            explicit_lsl0 in any::<bool>(),
        ) {
            let rd_n = gpr(is_64, rd);
            let mut ops = vec![Operand::Reg(rd_n.clone()), Operand::Imm(imm)];
            let asm = if hw == 0 && !explicit_lsl0 {
                format!("movn {}, #{}", rd_n, imm)
            } else {
                ops.push(Operand::Shift {
                    kind: "lsl".into(),
                    amount: hw * 16,
                });
                format!("movn {}, #{}, lsl #{}", rd_n, imm, hw * 16)
            };
            let mc = llvm_mc_word(&asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid {asm}: {e}"));
            let sut = sut_word(&ops)
                .unwrap_or_else(|e| panic!("SUT rejected valid {asm}: {e}"));
            prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
        }

        #[test]
        fn encode_movn_metamorphic_sf(
            rd in 0u32..=31,
            imm in imm16(),
            hw in 0u32..=1,
        ) {
            let x_ops = ops_imm(&gpr(true, rd), imm, hw);
            let w_ops = ops_imm(&gpr(false, rd), imm, hw);
            let xw = sut_word(&x_ops)
                .unwrap_or_else(|e| panic!("64-bit MOVN rejected: {e}"));
            let ww = sut_word(&w_ops)
                .unwrap_or_else(|e| panic!("32-bit MOVN rejected: {e}"));
            prop_assert_eq!(
                xw ^ ww,
                1u32 << 31,
                "X vs W MOVN must differ only by sf bit 31 (x={:#010x} w={:#010x})",
                xw,
                ww
            );
        }

        #[test]
        fn encode_movn_invariant_arm_fields(
            rd in 0u32..=31,
            (is_64, hw) in valid_width_hw(),
            imm in imm16(),
        ) {
            let ops = ops_imm(&gpr(is_64, rd), imm, hw);
            let w = sut_word(&ops).unwrap_or_else(|e| panic!("MOVN rejected: {e}"));
            let sf = if is_64 { 1u32 } else { 0 };
            prop_assert_eq!(w & 0x1F, rd, "Rd field");
            prop_assert_eq!((w >> 5) & 0xFFFF, imm as u32, "imm16 field");
            prop_assert_eq!((w >> 21) & 0x3, hw, "hw field");
            prop_assert_eq!((w >> 23) & 0xFF, 0b00100101u32, "bits 30:23 must be 00100101 (opc=00, 100101)");
            prop_assert_eq!((w >> 31) & 1, sf, "sf bit");
        }

        #[test]
        fn encode_movn_diff_lr(
            imm in imm16(),
            hw in 0u32..=3,
        ) {
            let ops = ops_imm("lr", imm, hw);
            let asm = movn_asm("lr", imm, hw);
            let sut = sut_word(&ops)
                .unwrap_or_else(|e| panic!("SUT rejected valid {asm}: {e}"));
            let mc = llvm_mc_word(&asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid {asm}: {e}"));
            prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {} (lr alias x30)", asm);
        }

        #[test]
        fn encode_movn_neg_imm_oob(
            rd in 0u32..=31,
            is_64 in any::<bool>(),
            imm in oob_imm(),
        ) {
            let ops = [Operand::Reg(gpr(is_64, rd)), Operand::Imm(imm)];
            prop_assert!(
                encode_movn(&ops).is_err(),
                "imm {} outside [0, 65535] must Err (llvm-mc rejects it)",
                imm
            );
        }

        #[test]
        fn encode_movn_neg_invalid_shift(
            rd in 0u32..=31,
            imm in imm16(),
            (is_64, kind, amount) in invalid_shift_case(),
        ) {
            let ops = [
                Operand::Reg(gpr(is_64, rd)),
                Operand::Imm(imm),
                Operand::Shift {
                    kind: kind.clone(),
                    amount,
                },
            ];
            prop_assert!(
                encode_movn(&ops).is_err(),
                "movn with shift {} #{} on {} must Err",
                kind,
                amount,
                gpr(is_64, rd)
            );
        }

        #[test]
        fn encode_movn_neg_extra_operand(
            rd in 0u32..=31,
            (is_64, hw) in valid_width_hw(),
            imm in imm16(),
            extra in extra_operand(),
        ) {
            let mut ops = ops_imm(&gpr(is_64, rd), imm, hw);
            ops.push(extra);
            prop_assert!(
                encode_movn(&ops).is_err(),
                "MOVN has no operand after optional lsl; extra operand must Err"
            );
        }

        #[test]
        fn encode_movn_neg_sp(
            is_64 in any::<bool>(),
            imm in imm16(),
            hw in 0u32..=1,
        ) {
            let name = if is_64 { "sp" } else { "wsp" };
            let ops = ops_imm(name, imm, hw);
            prop_assert!(
                encode_movn(&ops).is_err(),
                "MOVN {} must Err; register 31 is ZR not SP (llvm-mc rejects it)",
                name
            );
        }

        #[test]
        fn encode_movn_neg_too_few(
            n in 0usize..=1,
            rd in 0u32..=31,
            is_64 in any::<bool>(),
            imm in imm16(),
        ) {
            let all = [Operand::Reg(gpr(is_64, rd)), Operand::Imm(imm)];
            let ops = &all[..n];
            prop_assert!(
                encode_movn(ops).is_err(),
                "fewer than 2 operands must Err, n={}",
                n
            );
        }

        #[test]
        fn encode_movn_neg_fp(
            fp in fp_name(),
            imm in imm16(),
        ) {
            let ops = [Operand::Reg(fp.clone()), Operand::Imm(imm)];
            prop_assert!(
                encode_movn(&ops).is_err(),
                "MOVN {} must Err; FP/SIMD names are not GPRs (llvm-mc rejects it)",
                fp
            );
        }

        #[test]
        fn encode_movn_neg_invalid_name(
            name in invalid_name(),
            imm in imm16(),
        ) {
            let ops = [Operand::Reg(name.clone()), Operand::Imm(imm)];
            prop_assert!(
                encode_movn(&ops).is_err(),
                "invalid register name {:?} must Err",
                name
            );
        }

        #[test]
        fn encode_movn_neg_bad_second(
            rd in 0u32..=31,
            is_64 in any::<bool>(),
            second in prop_oneof![
                Just(Operand::Modifier {
                    kind: "lo12".into(),
                    symbol: "0".into(),
                }),
                Just(Operand::Modifier {
                    kind: "abs_g0".into(),
                    symbol: "foo".into(),
                }),
                Just(Operand::Symbol("sym".into())),
                Just(Operand::Label("L0".into())),
                Just(Operand::Mem {
                    base: "x0".into(),
                    offset: 0,
                }),
                Just(Operand::Reg("x1".into())),
            ],
        ) {
            let ops = [Operand::Reg(gpr(is_64, rd)), second];
            prop_assert!(
                encode_movn(&ops).is_err(),
                "second operand must be imm16"
            );
        }
    }

    fn invalid_name() -> impl Strategy<Value = String> {
        prop::sample::select(vec![
            "foo".into(),
            "x32".into(),
            "w32".into(),
            "x".into(),
            "r0".into(),
            "".into(),
        ])
    }

    fn fp_name() -> impl Strategy<Value = String> {
        prop_oneof![
            (0u32..=31).prop_map(|n| format!("d{n}")),
            (0u32..=31).prop_map(|n| format!("s{n}")),
            (0u32..=31).prop_map(|n| format!("q{n}")),
            (0u32..=31).prop_map(|n| format!("v{n}")),
            (0u32..=31).prop_map(|n| format!("h{n}")),
            (0u32..=31).prop_map(|n| format!("b{n}")),
        ]
    }

    #[test]
    fn test_encode_movn_regression_imm_oob() {
        let ops = [Operand::Reg("w0".into()), Operand::Imm(-1)];
        assert!(
            encode_movn(&ops).is_err(),
            "MOVN w0, #-1 must Err; imm16 range is [0, 65535] (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_movn_regression_invalid_shift() {
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Imm(0),
            Operand::Shift {
                kind: "lsr".into(),
                amount: 0,
            },
        ];
        assert!(
            encode_movn(&ops).is_err(),
            "MOVN w0, #0, lsr #0 must Err; only lsl with 0/16 (W) or 0/16/32/48 (X) is valid"
        );
    }

    #[test]
    fn test_encode_movn_regression_extra_operand() {
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Imm(0),
            Operand::Reg("x0".into()),
        ];
        assert!(
            encode_movn(&ops).is_err(),
            "MOVN x0, #0, x0 must Err; extra operand is invalid (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_movn_regression_sp() {
        let ops = [Operand::Reg("wsp".into()), Operand::Imm(0)];
        assert!(
            encode_movn(&ops).is_err(),
            "MOVN wsp, #0 must Err; register 31 is WZR not WSP (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_movn_regression_fp() {
        let ops = [Operand::Reg("d0".into()), Operand::Imm(0)];
        assert!(
            encode_movn(&ops).is_err(),
            "MOVN d0, #0 must Err; FP/SIMD registers are not MOVN operands (llvm-mc rejects it)"
        );
    }
}

#[cfg(test)]
mod encode_movz_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler
    // Evidence: src/backend/arm/assembler/README.md "accepts the same textual assembly that GCC's gas would consume";
    //   encoder/mod.rs "Encodes AArch64 instructions into 32-bit machine code words";
    //   encoder/mod.rs:220 movz dispatch; ARM ARM Move wide (immediate) MOVZ
    //   sf 10 100101 hw imm16 Rd; codegen emit.rs:911-922 movz Rd, #imm16 [, lsl #N]
    // Stronger considered:
    //   - State machine: rejected — encode_movz is a pure function with no lifecycle
    //   - Algebraic round-trip: rejected — no in-tree MOVZ decoder
    //   - encode_movk / encode_movn as differential sibling: rejected — same-job gate fails
    //     (opc 11/00 vs 10; MOVK keeps other halfwords, MOVN inverts)
    // Weaker available: algebraic.metamorphic (sf bit), algebraic.invariant (ARM fields),
    //   negative_error (imm16 range / shift / extra operand / SP / FP / arity)
    // Differential: candidate=encode_movz, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=operands <-> asm text `movz Rd, #imm16 [, lsl #N]`

    use super::encode_movz;
    use super::super::EncodeResult;
    use crate::backend::arm::assembler::parser::Operand;
    use proptest::prelude::*;
    use std::io::Write;
    use std::process::{Command, Stdio};

    const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
    const CASES: u32 = 1000;

    fn cfg() -> ProptestConfig {
        ProptestConfig::with_cases(CASES)
    }

    fn gpr(is_64: bool, n: u32) -> String {
        if n == 31 {
            if is_64 {
                "xzr".into()
            } else {
                "wzr".into()
            }
        } else {
            format!("{}{}", if is_64 { "x" } else { "w" }, n)
        }
    }

    fn sut_word(ops: &[Operand]) -> Result<u32, String> {
        match encode_movz(ops)? {
            EncodeResult::Word(w) => Ok(w),
            other => Err(format!("expected Word, got {:?}", other)),
        }
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

    fn movz_asm(rd: &str, imm: i64, hw: u32) -> String {
        if hw == 0 {
            format!("movz {}, #{}", rd, imm)
        } else {
            format!("movz {}, #{}, lsl #{}", rd, imm, hw * 16)
        }
    }

    fn ops_imm(rd: &str, imm: i64, hw: u32) -> Vec<Operand> {
        let mut ops = vec![Operand::Reg(rd.to_string()), Operand::Imm(imm)];
        if hw != 0 {
            ops.push(Operand::Shift {
                kind: "lsl".into(),
                amount: hw * 16,
            });
        }
        ops
    }

    fn imm16() -> impl Strategy<Value = i64> {
        prop_oneof![Just(0i64), Just(1i64), Just(65535i64), 0i64..=65535]
    }

    fn valid_width_hw() -> impl Strategy<Value = (bool, u32)> {
        prop_oneof![
            (Just(true), prop_oneof![Just(0u32), Just(1u32), Just(2u32), Just(3u32)]),
            (Just(false), prop_oneof![Just(0u32), Just(1u32)]),
        ]
    }

    fn oob_imm() -> impl Strategy<Value = i64> {
        prop_oneof![
            Just(-1i64),
            Just(65536i64),
            Just(65537i64),
            Just(-65535i64),
            Just(i64::MIN),
            Just(i64::MAX),
            (65536i64..=0x1_0000_0),
            (i64::MIN..=-1),
        ]
    }

    fn extra_operand() -> impl Strategy<Value = Operand> {
        prop_oneof![
            (0u32..=31).prop_map(|n| Operand::Reg(format!("x{n}"))),
            Just(Operand::Imm(0)),
            Just(Operand::Shift {
                kind: "lsl".into(),
                amount: 16,
            }),
        ]
    }

    fn abs_g_shift(kind: &str) -> u32 {
        match kind {
            "abs_g0" | "abs_g0_nc" => 0,
            "abs_g1" | "abs_g1_nc" => 16,
            "abs_g2" | "abs_g2_nc" => 32,
            "abs_g3" => 48,
            _ => 0,
        }
    }

    fn invalid_shift_case() -> impl Strategy<Value = (bool, String, u32)> {
        prop_oneof![
            (
                any::<bool>(),
                prop::sample::select(vec!["lsr", "asr", "ror", "lslx", ""]),
                prop_oneof![Just(0u32), Just(16u32), Just(32u32), 0u32..=64],
            )
                .prop_map(|(b, k, a)| (b, k.to_string(), a)),
            (
                any::<bool>(),
                prop::sample::select(vec![1u32, 8, 15, 17, 31, 33, 47, 49, 63, 64]),
            )
                .prop_map(|(b, a)| (b, "lsl".into(), a)),
            prop::sample::select(vec![32u32, 48]).prop_map(|a| (false, "lsl".into(), a)),
        ]
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_movz_kat_llvm_mc() {
        let want_x = 0xd2800540u32;
        let mc_x = llvm_mc_word("movz x0, #42").expect("llvm-mc KAT x");
        assert_eq!(mc_x, want_x, "llvm-mc KAT mapping broken for 64-bit");
        let ops_x = [Operand::Reg("x0".into()), Operand::Imm(42)];
        let sut_x = sut_word(&ops_x).expect("SUT KAT x");
        assert_eq!(sut_x, want_x);

        let want_w = 0x52800540u32;
        let mc_w = llvm_mc_word("movz w0, #42").expect("llvm-mc KAT w");
        assert_eq!(mc_w, want_w, "llvm-mc KAT mapping broken for 32-bit");
        let ops_w = [Operand::Reg("w0".into()), Operand::Imm(42)];
        let sut_w = sut_word(&ops_w).expect("SUT KAT w");
        assert_eq!(sut_w, want_w);

        let want_lsl = 0xd2a00540u32;
        let mc_lsl = llvm_mc_word("movz x0, #42, lsl #16").expect("llvm-mc KAT lsl");
        assert_eq!(mc_lsl, want_lsl, "llvm-mc KAT mapping broken for lsl #16");
        let ops_lsl = [
            Operand::Reg("x0".into()),
            Operand::Imm(42),
            Operand::Shift {
                kind: "lsl".into(),
                amount: 16,
            },
        ];
        let sut_lsl = sut_word(&ops_lsl).expect("SUT KAT lsl");
        assert_eq!(sut_lsl, want_lsl);
    }

    proptest! {
        #![proptest_config(cfg())]

        #[test]
        fn encode_movz_diff_imm_shift(
            rd in 0u32..=31,
            (is_64, hw) in valid_width_hw(),
            imm in imm16(),
            explicit_lsl0 in any::<bool>(),
        ) {
            let rd_n = gpr(is_64, rd);
            let mut ops = vec![Operand::Reg(rd_n.clone()), Operand::Imm(imm)];
            let asm = if hw == 0 && !explicit_lsl0 {
                format!("movz {}, #{}", rd_n, imm)
            } else {
                ops.push(Operand::Shift {
                    kind: "lsl".into(),
                    amount: hw * 16,
                });
                format!("movz {}, #{}, lsl #{}", rd_n, imm, hw * 16)
            };
            let mc = llvm_mc_word(&asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid {asm}: {e}"));
            let sut = sut_word(&ops)
                .unwrap_or_else(|e| panic!("SUT rejected valid {asm}: {e}"));
            prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
        }

        #[test]
        fn encode_movz_metamorphic_sf(
            rd in 0u32..=31,
            imm in imm16(),
            hw in 0u32..=1,
        ) {
            let x_ops = ops_imm(&gpr(true, rd), imm, hw);
            let w_ops = ops_imm(&gpr(false, rd), imm, hw);
            let xw = sut_word(&x_ops)
                .unwrap_or_else(|e| panic!("64-bit MOVZ rejected: {e}"));
            let ww = sut_word(&w_ops)
                .unwrap_or_else(|e| panic!("32-bit MOVZ rejected: {e}"));
            prop_assert_eq!(
                xw ^ ww,
                1u32 << 31,
                "X vs W MOVZ must differ only by sf bit 31 (x={:#010x} w={:#010x})",
                xw,
                ww
            );
        }

        #[test]
        fn encode_movz_invariant_arm_fields(
            rd in 0u32..=31,
            (is_64, hw) in valid_width_hw(),
            imm in imm16(),
        ) {
            let ops = ops_imm(&gpr(is_64, rd), imm, hw);
            let w = sut_word(&ops).unwrap_or_else(|e| panic!("MOVZ rejected: {e}"));
            let sf = if is_64 { 1u32 } else { 0 };
            prop_assert_eq!(w & 0x1F, rd, "Rd field");
            prop_assert_eq!((w >> 5) & 0xFFFF, imm as u32, "imm16 field");
            prop_assert_eq!((w >> 21) & 0x3, hw, "hw field");
            prop_assert_eq!((w >> 23) & 0xFF, 0b10100101u32, "bits 30:23 must be 10100101 (opc=10, 100101)");
            prop_assert_eq!((w >> 31) & 1, sf, "sf bit");
        }

        #[test]
        fn encode_movz_diff_abs_g(
            rd in 0u32..=31,
            is_64 in any::<bool>(),
            val in any::<i64>(),
            kind in prop::sample::select(vec![
                "abs_g0", "abs_g0_nc", "abs_g1", "abs_g1_nc",
                "abs_g2", "abs_g2_nc", "abs_g3",
            ]),
        ) {
            let kind = if !is_64 && abs_g_shift(kind) >= 32 {
                if kind.contains('1') { "abs_g1" } else { "abs_g0" }
            } else {
                kind
            };
            let shift = abs_g_shift(kind);
            let hw = shift / 16;
            let chunk = ((val as u64) >> shift) as i64 & 0xFFFF;
            let rd_n = gpr(is_64, rd);
            let ops = [
                Operand::Reg(rd_n.clone()),
                Operand::Modifier {
                    kind: kind.to_string(),
                    symbol: val.to_string(),
                },
            ];
            let asm = movz_asm(&rd_n, chunk, hw);
            let mc = llvm_mc_word(&asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected resolved {asm}: {e}"));
            let sut = sut_word(&ops).unwrap_or_else(|e| {
                panic!("SUT rejected abs_g {kind} val={val} -> {asm}: {e}")
            });
            prop_assert_eq!(
                sut, mc,
                "abs_g {} val={} must match resolved {}",
                kind,
                val,
                asm
            );
        }

        #[test]
        fn encode_movz_neg_imm_oob(
            rd in 0u32..=31,
            is_64 in any::<bool>(),
            imm in oob_imm(),
        ) {
            let ops = [Operand::Reg(gpr(is_64, rd)), Operand::Imm(imm)];
            prop_assert!(
                encode_movz(&ops).is_err(),
                "imm {} outside [0, 65535] must Err (llvm-mc rejects it)",
                imm
            );
        }

        #[test]
        fn encode_movz_neg_invalid_shift(
            rd in 0u32..=31,
            imm in imm16(),
            (is_64, kind, amount) in invalid_shift_case(),
        ) {
            let ops = [
                Operand::Reg(gpr(is_64, rd)),
                Operand::Imm(imm),
                Operand::Shift {
                    kind: kind.clone(),
                    amount,
                },
            ];
            prop_assert!(
                encode_movz(&ops).is_err(),
                "movz with shift {} #{} on {} must Err",
                kind,
                amount,
                gpr(is_64, rd)
            );
        }

        #[test]
        fn encode_movz_neg_extra_operand(
            rd in 0u32..=31,
            (is_64, hw) in valid_width_hw(),
            imm in imm16(),
            extra in extra_operand(),
        ) {
            let mut ops = ops_imm(&gpr(is_64, rd), imm, hw);
            ops.push(extra);
            prop_assert!(
                encode_movz(&ops).is_err(),
                "MOVZ has no operand after optional lsl; extra operand must Err"
            );
        }

        #[test]
        fn encode_movz_neg_sp(
            is_64 in any::<bool>(),
            imm in imm16(),
            hw in 0u32..=1,
        ) {
            let name = if is_64 { "sp" } else { "wsp" };
            let ops = ops_imm(name, imm, hw);
            prop_assert!(
                encode_movz(&ops).is_err(),
                "MOVZ {} must Err; register 31 is ZR not SP (llvm-mc rejects it)",
                name
            );
        }

        #[test]
        fn encode_movz_diff_lr(
            imm in imm16(),
            hw in 0u32..=3,
        ) {
            let ops = ops_imm("lr", imm, hw);
            let asm = movz_asm("lr", imm, hw);
            let sut = sut_word(&ops)
                .unwrap_or_else(|e| panic!("SUT rejected valid {asm}: {e}"));
            let mc = llvm_mc_word(&asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid {asm}: {e}"));
            prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {} (lr alias x30)", asm);
        }

        #[test]
        fn encode_movz_neg_too_few(
            n in 0usize..=1,
            rd in 0u32..=31,
            is_64 in any::<bool>(),
            imm in imm16(),
        ) {
            let all = [Operand::Reg(gpr(is_64, rd)), Operand::Imm(imm)];
            let ops = &all[..n];
            prop_assert!(
                encode_movz(ops).is_err(),
                "fewer than 2 operands must Err, n={}",
                n
            );
        }

        #[test]
        fn encode_movz_neg_fp(
            fp in prop_oneof![
                (0u32..=31).prop_map(|n| format!("d{n}")),
                (0u32..=31).prop_map(|n| format!("s{n}")),
                (0u32..=31).prop_map(|n| format!("q{n}")),
                (0u32..=31).prop_map(|n| format!("v{n}")),
                (0u32..=31).prop_map(|n| format!("h{n}")),
                (0u32..=31).prop_map(|n| format!("b{n}")),
            ],
            imm in imm16(),
        ) {
            let ops = [Operand::Reg(fp.clone()), Operand::Imm(imm)];
            prop_assert!(
                encode_movz(&ops).is_err(),
                "MOVZ {} must Err; FP/SIMD names are not GPRs (llvm-mc rejects it)",
                fp
            );
        }

        #[test]
        fn encode_movz_neg_invalid_name(
            name in prop::sample::select(vec![
                "foo".to_string(),
                "x32".into(),
                "w32".into(),
                "x".into(),
                "r0".into(),
                "".into(),
            ]),
            imm in imm16(),
        ) {
            let ops = [Operand::Reg(name.clone()), Operand::Imm(imm)];
            prop_assert!(
                encode_movz(&ops).is_err(),
                "invalid register name {:?} must Err",
                name
            );
        }

        #[test]
        fn encode_movz_neg_bad_second(
            rd in 0u32..=31,
            is_64 in any::<bool>(),
            second in prop_oneof![
                Just(Operand::Modifier {
                    kind: "lo12".into(),
                    symbol: "0".into(),
                }),
                Just(Operand::Modifier {
                    kind: "abs_g0".into(),
                    symbol: "foo".into(),
                }),
                Just(Operand::Symbol("sym".into())),
                Just(Operand::Label("L0".into())),
                Just(Operand::Mem {
                    base: "x0".into(),
                    offset: 0,
                }),
                Just(Operand::Reg("x1".into())),
            ],
        ) {
            let ops = [Operand::Reg(gpr(is_64, rd)), second];
            prop_assert!(
                encode_movz(&ops).is_err(),
                "second operand must be imm16 or a resolvable abs_g modifier"
            );
        }
    }

    #[test]
    fn test_encode_movz_regression_imm_oob() {
        let ops = [Operand::Reg("w0".into()), Operand::Imm(-1)];
        assert!(
            encode_movz(&ops).is_err(),
            "MOVZ w0, #-1 must Err; imm16 range is [0, 65535] (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_movz_regression_invalid_shift() {
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Imm(0),
            Operand::Shift {
                kind: "lsr".into(),
                amount: 0,
            },
        ];
        assert!(
            encode_movz(&ops).is_err(),
            "MOVZ w0, #0, lsr #0 must Err; only lsl with 0/16 (W) or 0/16/32/48 (X) is valid"
        );
    }

    #[test]
    fn test_encode_movz_regression_extra_operand() {
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Imm(0),
            Operand::Reg("x0".into()),
        ];
        assert!(
            encode_movz(&ops).is_err(),
            "MOVZ x0, #0, x0 must Err; extra operand is invalid (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_movz_regression_sp() {
        let ops = [Operand::Reg("wsp".into()), Operand::Imm(0)];
        assert!(
            encode_movz(&ops).is_err(),
            "MOVZ wsp, #0 must Err; register 31 is WZR not WSP (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_movz_regression_fp() {
        let ops = [Operand::Reg("d0".into()), Operand::Imm(0)];
        assert!(
            encode_movz(&ops).is_err(),
            "MOVZ d0, #0 must Err; FP/SIMD registers are not MOVZ operands (llvm-mc rejects it)"
        );
    }
}

#[cfg(test)]
mod encode_msub_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler
    // Evidence: src/backend/arm/assembler/README.md "accepts the same textual assembly that GCC's gas would consume";
    //   encoder/mod.rs "Encodes AArch64 instructions into 32-bit machine code words";
    //   encoder/mod.rs:246 msub dispatch; ARM ARM Data-processing (3 source) MSUB
    //   sf 00 11011 000 Rm 1 Ra Rn Rd; data_processing.rs:677 MNEG is MSUB with Ra=XZR
    // Stronger considered:
    //   - State machine: rejected — encode_msub is a pure function with no lifecycle
    //   - Algebraic round-trip: rejected — no in-tree MSUB decoder
    //   - encode_madd as differential sibling: rejected — same-job gate fails (o0=0 vs o0=1)
    //   - encode_mneg as differential sibling: rejected — same-job gate fails (3-operand alias vs 4-operand MSUB)
    // Weaker available: algebraic.metamorphic (sf bit), algebraic.invariant (ARM fields),
    //   negative_error (arity / extra operand / mixed width / SP / FP / invalid name)
    // Differential: candidate=encode_msub, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=operands <-> asm text `msub Rd, Rn, Rm, Ra`

    use super::encode_msub;
    use super::super::EncodeResult;
    use crate::backend::arm::assembler::parser::Operand;
    use proptest::prelude::*;
    use std::io::Write;
    use std::process::{Command, Stdio};

    const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
    const CASES: u32 = 1000;

    fn cfg() -> ProptestConfig {
        ProptestConfig::with_cases(CASES)
    }

    fn gpr(is_64: bool, n: u32) -> String {
        if n == 31 {
            if is_64 {
                "xzr".into()
            } else {
                "wzr".into()
            }
        } else {
            format!("{}{}", if is_64 { "x" } else { "w" }, n)
        }
    }

    fn sut_word(ops: &[Operand]) -> Result<u32, String> {
        match encode_msub(ops)? {
            EncodeResult::Word(w) => Ok(w),
            other => Err(format!("expected Word, got {:?}", other)),
        }
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

    fn extra_operand() -> impl Strategy<Value = Operand> {
        prop_oneof![
            (0u32..=31).prop_map(|n| Operand::Reg(format!("x{n}"))),
            Just(Operand::Imm(0)),
            Just(Operand::Shift {
                kind: "lsl".into(),
                amount: 0,
            }),
        ]
    }

    fn non_register() -> impl Strategy<Value = Operand> {
        prop_oneof![
            any::<i64>().prop_map(Operand::Imm),
            Just(Operand::Symbol("foo".into())),
            Just(Operand::Mem {
                base: "x0".into(),
                offset: 8,
            }),
            Just(Operand::Shift {
                kind: "lsl".into(),
                amount: 0,
            }),
            Just(Operand::Cond("eq".into())),
            Just(Operand::Label("L0".into())),
        ]
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_msub_kat_llvm_mc_x0_x1_x2_x3() {
        let want = 0x9b028c20u32;
        let mc = llvm_mc_word("msub x0, x1, x2, x3").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("x1".into()),
            Operand::Reg("x2".into()),
            Operand::Reg("x3".into()),
        ];
        let sut = sut_word(&ops).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_msub_kat_llvm_mc_w0_w1_w2_w3() {
        let want = 0x1b028c20u32;
        let mc = llvm_mc_word("msub w0, w1, w2, w3").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Reg("w1".into()),
            Operand::Reg("w2".into()),
            Operand::Reg("w3".into()),
        ];
        let sut = sut_word(&ops).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_msub_kat_llvm_mc_ra_zr_is_mneg() {
        let want = 0x9b02fc20u32;
        let mc_mneg = llvm_mc_word("mneg x0, x1, x2").expect("llvm-mc MNEG KAT");
        let mc_msub = llvm_mc_word("msub x0, x1, x2, xzr").expect("llvm-mc MSUB ZR KAT");
        assert_eq!(mc_mneg, want, "llvm-mc MNEG KAT mapping broken");
        assert_eq!(mc_msub, want, "llvm-mc MSUB ZR KAT mapping broken");
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("x1".into()),
            Operand::Reg("x2".into()),
            Operand::Reg("xzr".into()),
        ];
        let sut = sut_word(&ops).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    proptest! {
        #![proptest_config(cfg())]

        #[test]
        fn encode_msub_diff_gpr(
            rd in 0u32..=31,
            rn in 0u32..=31,
            rm in 0u32..=31,
            ra in 0u32..=31,
            is_64 in any::<bool>(),
        ) {
            let rd_n = gpr(is_64, rd);
            let rn_n = gpr(is_64, rn);
            let rm_n = gpr(is_64, rm);
            let ra_n = gpr(is_64, ra);
            let asm = format!("msub {}, {}, {}, {}", rd_n, rn_n, rm_n, ra_n);
            let ops = [
                Operand::Reg(rd_n),
                Operand::Reg(rn_n),
                Operand::Reg(rm_n),
                Operand::Reg(ra_n),
            ];
            let sut = sut_word(&ops)
                .unwrap_or_else(|e| panic!("SUT rejected valid MSUB {}: {}", asm, e));
            let mc = llvm_mc_word(&asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid MSUB {}: {}", asm, e));
            prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
        }

        #[test]
        fn encode_msub_diff_ra_zr_is_mneg(
            rd in 0u32..=31,
            rn in 0u32..=31,
            rm in 0u32..=31,
            is_64 in any::<bool>(),
        ) {
            let rd_n = gpr(is_64, rd);
            let rn_n = gpr(is_64, rn);
            let rm_n = gpr(is_64, rm);
            let zr = gpr(is_64, 31);
            let mneg_asm = format!("mneg {}, {}, {}", rd_n, rn_n, rm_n);
            let msub_asm = format!("msub {}, {}, {}, {}", rd_n, rn_n, rm_n, zr);
            let ops = [
                Operand::Reg(rd_n),
                Operand::Reg(rn_n),
                Operand::Reg(rm_n),
                Operand::Reg(zr),
            ];
            let sut = sut_word(&ops)
                .unwrap_or_else(|e| panic!("SUT rejected valid MSUB {}: {}", msub_asm, e));
            let mc_mneg = llvm_mc_word(&mneg_asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid MNEG {}: {}", mneg_asm, e));
            let mc_msub = llvm_mc_word(&msub_asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid MSUB {}: {}", msub_asm, e));
            prop_assert_eq!(mc_mneg, mc_msub, "llvm-mc MNEG vs MSUB ZR mismatch");
            prop_assert_eq!(sut, mc_mneg, "SUT vs llvm-mc MNEG mismatch for {}", mneg_asm);
        }

        #[test]
        fn encode_msub_metamorphic_sf_bit(
            rd in 0u32..=31,
            rn in 0u32..=31,
            rm in 0u32..=31,
            ra in 0u32..=31,
        ) {
            let x_ops = [
                Operand::Reg(gpr(true, rd)),
                Operand::Reg(gpr(true, rn)),
                Operand::Reg(gpr(true, rm)),
                Operand::Reg(gpr(true, ra)),
            ];
            let w_ops = [
                Operand::Reg(gpr(false, rd)),
                Operand::Reg(gpr(false, rn)),
                Operand::Reg(gpr(false, rm)),
                Operand::Reg(gpr(false, ra)),
            ];
            let xw = sut_word(&x_ops)
                .unwrap_or_else(|e| panic!("64-bit MSUB rejected: {}", e));
            let ww = sut_word(&w_ops)
                .unwrap_or_else(|e| panic!("32-bit MSUB rejected: {}", e));
            prop_assert_eq!(
                xw ^ ww,
                1u32 << 31,
                "X vs W MSUB must differ only by sf bit 31 (x={:#010x} w={:#010x})",
                xw,
                ww
            );
        }

        #[test]
        fn encode_msub_invariant_arm_fields(
            rd in 0u32..=31,
            rn in 0u32..=31,
            rm in 0u32..=31,
            ra in 0u32..=31,
            is_64 in any::<bool>(),
        ) {
            let ops = [
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
                Operand::Reg(gpr(is_64, rm)),
                Operand::Reg(gpr(is_64, ra)),
            ];
            let w = sut_word(&ops)
                .unwrap_or_else(|e| panic!("MSUB rejected: {}", e));
            let sf = if is_64 { 1u32 } else { 0 };
            prop_assert_eq!(w & 0x1F, rd, "Rd field");
            prop_assert_eq!((w >> 5) & 0x1F, rn, "Rn field");
            prop_assert_eq!((w >> 10) & 0x1F, ra, "Ra field");
            prop_assert_eq!((w >> 15) & 1, 1, "o0 bit 15 must be 1 (MSUB not MADD)");
            prop_assert_eq!((w >> 16) & 0x1F, rm, "Rm field");
            prop_assert_eq!((w >> 21) & 0x3FF, 0b0011011000u32, "bits 30:21 must be 0011011000");
            prop_assert_eq!((w >> 31) & 1, sf, "sf bit");
        }

        #[test]
        fn encode_msub_diff_lr(
            which in 0u32..=3,
            a in 0u32..=30,
            b in 0u32..=30,
            c in 0u32..=30,
        ) {
            let mut names = [
                gpr(true, a),
                gpr(true, b),
                gpr(true, c),
                "lr".to_string(),
            ];
            names.swap(3, which as usize);
            let asm = format!(
                "msub {}, {}, {}, {}",
                names[0], names[1], names[2], names[3]
            );
            let ops = [
                Operand::Reg(names[0].clone()),
                Operand::Reg(names[1].clone()),
                Operand::Reg(names[2].clone()),
                Operand::Reg(names[3].clone()),
            ];
            let sut = sut_word(&ops)
                .unwrap_or_else(|e| panic!("SUT rejected valid MSUB {}: {}", asm, e));
            let mc = llvm_mc_word(&asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid MSUB {}: {}", asm, e));
            prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {} (lr alias x30)", asm);
        }

        #[test]
        fn encode_msub_neg_too_few(
            n in 0usize..=3,
            is_64 in any::<bool>(),
            r0 in 0u32..=31,
            r1 in 0u32..=31,
            r2 in 0u32..=31,
        ) {
            let all = [
                Operand::Reg(gpr(is_64, r0)),
                Operand::Reg(gpr(is_64, r1)),
                Operand::Reg(gpr(is_64, r2)),
            ];
            let ops = &all[..n.min(3)];
            prop_assert!(
                encode_msub(ops).is_err(),
                "fewer than 4 operands must Err, n={}",
                n
            );
        }

        #[test]
        fn encode_msub_neg_extra_operand(
            rd in 0u32..=30,
            rn in 0u32..=30,
            rm in 0u32..=30,
            ra in 0u32..=30,
            is_64 in any::<bool>(),
            extra in extra_operand(),
        ) {
            let ops = vec![
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
                Operand::Reg(gpr(is_64, rm)),
                Operand::Reg(gpr(is_64, ra)),
                extra,
            ];
            prop_assert!(
                encode_msub(&ops).is_err(),
                "MSUB has no 5th operand; extra operand must Err"
            );
        }

        #[test]
        fn encode_msub_neg_mixed_width(
            rd in 0u32..=30,
            rn in 0u32..=30,
            rm in 0u32..=30,
            ra in 0u32..=30,
            rd64 in any::<bool>(),
            rn64 in any::<bool>(),
            rm64 in any::<bool>(),
            ra64 in any::<bool>(),
        ) {
            prop_assume!(!(rd64 == rn64 && rn64 == rm64 && rm64 == ra64));
            let ops = [
                Operand::Reg(gpr(rd64, rd)),
                Operand::Reg(gpr(rn64, rn)),
                Operand::Reg(gpr(rm64, rm)),
                Operand::Reg(gpr(ra64, ra)),
            ];
            prop_assert!(
                encode_msub(&ops).is_err(),
                "mixed-width MSUB registers must Err (rd64={} rn64={} rm64={} ra64={})",
                rd64,
                rn64,
                rm64,
                ra64
            );
        }

        #[test]
        fn encode_msub_neg_sp(
            which in 0u32..=3,
            is_64 in any::<bool>(),
            a in 0u32..=30,
            b in 0u32..=30,
            c in 0u32..=30,
        ) {
            let sp = if is_64 { "sp" } else { "wsp" };
            let mut names = [
                gpr(is_64, a),
                gpr(is_64, b),
                gpr(is_64, c),
                sp.to_string(),
            ];
            names.swap(3, which as usize);
            let ops = [
                Operand::Reg(names[0].clone()),
                Operand::Reg(names[1].clone()),
                Operand::Reg(names[2].clone()),
                Operand::Reg(names[3].clone()),
            ];
            prop_assert!(
                encode_msub(&ops).is_err(),
                "SP/WSP is not a valid MSUB operand (which={} names={:?})",
                which,
                names
            );
        }

        #[test]
        fn encode_msub_neg_fp(
            which in 0u32..=3,
            prefix in prop::sample::select(vec!["d", "s", "q", "v", "h", "b"]),
            n in prop_oneof![Just(0u32), Just(31u32), 0u32..=31],
        ) {
            let fp = format!("{}{}", prefix, n);
            let mut ops = vec![
                Operand::Reg("x0".into()),
                Operand::Reg("x1".into()),
                Operand::Reg("x2".into()),
                Operand::Reg("x3".into()),
            ];
            ops[which as usize] = Operand::Reg(fp.clone());
            prop_assert!(
                encode_msub(&ops).is_err(),
                "FP/SIMD register {} is not a valid MSUB operand (which={})",
                fp,
                which
            );
        }

        #[test]
        fn encode_msub_neg_invalid_reg(
            which in 0u32..=3,
            bad in prop_oneof![
                Just("x32".to_string()),
                Just("w32".to_string()),
                Just("x99".to_string()),
                Just("w99".to_string()),
                Just("".to_string()),
                Just("foo".to_string()),
                Just("r0".to_string()),
                Just("x".to_string()),
                Just("x-1".to_string()),
            ],
        ) {
            let mut ops = vec![
                Operand::Reg("x0".into()),
                Operand::Reg("x1".into()),
                Operand::Reg("x2".into()),
                Operand::Reg("x3".into()),
            ];
            ops[which as usize] = Operand::Reg(bad.clone());
            prop_assert!(
                encode_msub(&ops).is_err(),
                "invalid register name {:?} at {} must Err",
                bad,
                which
            );
        }

        #[test]
        fn encode_msub_neg_non_register(
            which in 0u32..=3,
            bad in non_register(),
        ) {
            let mut ops = vec![
                Operand::Reg("x0".into()),
                Operand::Reg("x1".into()),
                Operand::Reg("x2".into()),
                Operand::Reg("x3".into()),
            ];
            ops[which as usize] = bad;
            prop_assert!(
                encode_msub(&ops).is_err(),
                "non-register operand at position {} must Err",
                which
            );
        }
    }

    #[test]
    fn test_encode_msub_regression_extra_operand() {
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("x0".into()),
        ];
        assert!(
            encode_msub(&ops).is_err(),
            "MSUB w0, w0, w0, w0, x0 must Err; a 5th operand is not valid (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_msub_regression_mixed_width() {
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("x0".into()),
        ];
        assert!(
            encode_msub(&ops).is_err(),
            "mixed-width MSUB w0, w0, w0, x0 must Err (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_msub_regression_sp() {
        let ops = [
            Operand::Reg("wsp".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
        ];
        assert!(
            encode_msub(&ops).is_err(),
            "MSUB wsp, w0, w0, w0 must Err; register 31 is WZR not WSP (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_msub_regression_fp_reg() {
        let ops = [
            Operand::Reg("d0".into()),
            Operand::Reg("x1".into()),
            Operand::Reg("x2".into()),
            Operand::Reg("x3".into()),
        ];
        assert!(
            encode_msub(&ops).is_err(),
            "MSUB d0, x1, x2, x3 must Err; FP/SIMD registers are not MSUB operands (llvm-mc rejects it)"
        );
    }
}

#[cfg(test)]
mod encode_mul_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler
    // Evidence: src/backend/arm/assembler/README.md "accepts the same textual assembly that GCC's gas would consume";
    //   encoder/mod.rs "Encodes AArch64 instructions into 32-bit machine code words";
    //   encoder/mod.rs:238-244 mul dispatch; ARM ARM Data-processing (3 source) MUL
    //   sf 00 11011 000 Rm 0 11111 Rn Rd; data_processing.rs:589 MUL is MADD with Ra=XZR;
    //   ARM ARM Advanced SIMD MUL 0 Q 0 01110 size 1 Rm 10011 1 Rn Rd, T in {8B,16B,4H,8H,2S,4S}
    // Stronger considered:
    //   - State machine: rejected — encode_mul is a pure function with no lifecycle
    //   - Algebraic round-trip: rejected — no in-tree MUL decoder
    //   - encode_madd as differential sibling: rejected — same-job gate fails (4-operand MADD vs 3-operand MUL)
    // Weaker available: algebraic.metamorphic (sf bit), algebraic.invariant (ARM fields),
    //   negative_error (arity / extra operand / mixed width / SP / FP / invalid name / NEON 2d)
    // Differential: candidate=encode_mul, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=operands <-> asm text `mul Rd, Rn, Rm` or `mul Vd.T, Vn.T, Vm.T`

    use super::encode_mul;
    use super::super::EncodeResult;
    use crate::backend::arm::assembler::parser::Operand;
    use proptest::prelude::*;
    use std::io::Write;
    use std::process::{Command, Stdio};

    const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
    const CASES: u32 = 1000;

    fn cfg() -> ProptestConfig {
        ProptestConfig::with_cases(CASES)
    }

    fn gpr(is_64: bool, n: u32) -> String {
        if n == 31 {
            if is_64 {
                "xzr".into()
            } else {
                "wzr".into()
            }
        } else {
            format!("{}{}", if is_64 { "x" } else { "w" }, n)
        }
    }

    fn sut_word(ops: &[Operand]) -> Result<u32, String> {
        match encode_mul(ops)? {
            EncodeResult::Word(w) => Ok(w),
            other => Err(format!("expected Word, got {:?}", other)),
        }
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

    fn extra_operand() -> impl Strategy<Value = Operand> {
        prop_oneof![
            (0u32..=31).prop_map(|n| Operand::Reg(format!("x{n}"))),
            Just(Operand::Imm(0)),
            Just(Operand::Shift {
                kind: "lsl".into(),
                amount: 0,
            }),
        ]
    }

    fn neon_t() -> impl Strategy<Value = &'static str> {
        prop::sample::select(vec!["8b", "16b", "4h", "8h", "2s", "4s"])
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_mul_kat_llvm_mc_x0_x1_x2() {
        let want = 0x9b027c20u32;
        let mc = llvm_mc_word("mul x0, x1, x2").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("x1".into()),
            Operand::Reg("x2".into()),
        ];
        let sut = sut_word(&ops).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_mul_kat_llvm_mc_w0_w1_w2() {
        let want = 0x1b027c20u32;
        let mc = llvm_mc_word("mul w0, w1, w2").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Reg("w1".into()),
            Operand::Reg("w2".into()),
        ];
        let sut = sut_word(&ops).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_mul_kat_llvm_mc_alias_madd_zr() {
        let want = 0x9b027c20u32;
        let mc_mul = llvm_mc_word("mul x0, x1, x2").expect("llvm-mc MUL KAT");
        let mc_madd = llvm_mc_word("madd x0, x1, x2, xzr").expect("llvm-mc MADD ZR KAT");
        assert_eq!(mc_mul, want, "llvm-mc MUL KAT mapping broken");
        assert_eq!(mc_madd, want, "llvm-mc MADD ZR KAT mapping broken");
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("x1".into()),
            Operand::Reg("x2".into()),
        ];
        let sut = sut_word(&ops).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_mul_kat_llvm_mc_neon_v0_16b() {
        let want = 0x4e229c20u32;
        let mc = llvm_mc_word("mul v0.16b, v1.16b, v2.16b").expect("llvm-mc NEON KAT");
        assert_eq!(mc, want, "llvm-mc NEON KAT mapping broken");
        let ops = [
            Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "16b".into(),
            },
            Operand::RegArrangement {
                reg: "v1".into(),
                arrangement: "16b".into(),
            },
            Operand::RegArrangement {
                reg: "v2".into(),
                arrangement: "16b".into(),
            },
        ];
        let sut = sut_word(&ops).expect("SUT NEON KAT");
        assert_eq!(sut, want);
    }

    proptest! {
        #![proptest_config(cfg())]

        #[test]
        fn encode_mul_diff_gpr(
            rd in 0u32..=31,
            rn in 0u32..=31,
            rm in 0u32..=31,
            is_64 in any::<bool>(),
        ) {
            let rd_n = gpr(is_64, rd);
            let rn_n = gpr(is_64, rn);
            let rm_n = gpr(is_64, rm);
            let asm = format!("mul {}, {}, {}", rd_n, rn_n, rm_n);
            let ops = [
                Operand::Reg(rd_n),
                Operand::Reg(rn_n),
                Operand::Reg(rm_n),
            ];
            let sut = sut_word(&ops)
                .unwrap_or_else(|e| panic!("SUT rejected valid MUL {}: {}", asm, e));
            let mc = llvm_mc_word(&asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid MUL {}: {}", asm, e));
            prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
        }

        #[test]
        fn encode_mul_diff_alias_madd_zr(
            rd in 0u32..=31,
            rn in 0u32..=31,
            rm in 0u32..=31,
            is_64 in any::<bool>(),
        ) {
            let rd_n = gpr(is_64, rd);
            let rn_n = gpr(is_64, rn);
            let rm_n = gpr(is_64, rm);
            let zr = gpr(is_64, 31);
            let mul_asm = format!("mul {}, {}, {}", rd_n, rn_n, rm_n);
            let madd_asm = format!("madd {}, {}, {}, {}", rd_n, rn_n, rm_n, zr);
            let ops = [
                Operand::Reg(rd_n),
                Operand::Reg(rn_n),
                Operand::Reg(rm_n),
            ];
            let sut = sut_word(&ops)
                .unwrap_or_else(|e| panic!("SUT rejected valid MUL {}: {}", mul_asm, e));
            let mc_mul = llvm_mc_word(&mul_asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid MUL {}: {}", mul_asm, e));
            let mc_madd = llvm_mc_word(&madd_asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid MADD ZR {}: {}", madd_asm, e));
            prop_assert_eq!(mc_mul, mc_madd, "llvm-mc MUL vs MADD ZR mismatch");
            prop_assert_eq!(sut, mc_mul, "SUT vs llvm-mc MUL mismatch for {}", mul_asm);
        }

        #[test]
        fn encode_mul_metamorphic_sf_bit(
            rd in 0u32..=31,
            rn in 0u32..=31,
            rm in 0u32..=31,
        ) {
            let x_ops = [
                Operand::Reg(gpr(true, rd)),
                Operand::Reg(gpr(true, rn)),
                Operand::Reg(gpr(true, rm)),
            ];
            let w_ops = [
                Operand::Reg(gpr(false, rd)),
                Operand::Reg(gpr(false, rn)),
                Operand::Reg(gpr(false, rm)),
            ];
            let xw = sut_word(&x_ops)
                .unwrap_or_else(|e| panic!("64-bit MUL rejected: {}", e));
            let ww = sut_word(&w_ops)
                .unwrap_or_else(|e| panic!("32-bit MUL rejected: {}", e));
            prop_assert_eq!(
                xw ^ ww,
                1u32 << 31,
                "X vs W MUL must differ only by sf bit 31 (x={:#010x} w={:#010x})",
                xw,
                ww
            );
        }

        #[test]
        fn encode_mul_invariant_arm_fields(
            rd in 0u32..=31,
            rn in 0u32..=31,
            rm in 0u32..=31,
            is_64 in any::<bool>(),
        ) {
            let ops = [
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
                Operand::Reg(gpr(is_64, rm)),
            ];
            let w = sut_word(&ops)
                .unwrap_or_else(|e| panic!("MUL rejected: {}", e));
            let sf = if is_64 { 1u32 } else { 0 };
            prop_assert_eq!(w & 0x1F, rd, "Rd field");
            prop_assert_eq!((w >> 5) & 0x1F, rn, "Rn field");
            prop_assert_eq!((w >> 10) & 0x1F, 31, "Ra field must be XZR/WZR (31)");
            prop_assert_eq!((w >> 15) & 1, 0, "o0 bit 15 must be 0 (MUL/MADD not MSUB)");
            prop_assert_eq!((w >> 16) & 0x1F, rm, "Rm field");
            prop_assert_eq!((w >> 21) & 0x3FF, 0b0011011000u32, "bits 30:21 must be 0011011000");
            prop_assert_eq!((w >> 31) & 1, sf, "sf bit");
        }

        #[test]
        fn encode_mul_diff_neon(
            vd in 0u32..=31,
            vn in 0u32..=31,
            vm in 0u32..=31,
            t in neon_t(),
        ) {
            let asm = format!("mul v{}.{t}, v{}.{t}, v{}.{t}", vd, vn, vm);
            let ops = [
                Operand::RegArrangement {
                    reg: format!("v{vd}"),
                    arrangement: t.to_string(),
                },
                Operand::RegArrangement {
                    reg: format!("v{vn}"),
                    arrangement: t.to_string(),
                },
                Operand::RegArrangement {
                    reg: format!("v{vm}"),
                    arrangement: t.to_string(),
                },
            ];
            let sut = sut_word(&ops)
                .unwrap_or_else(|e| panic!("SUT rejected valid NEON MUL {}: {}", asm, e));
            let mc = llvm_mc_word(&asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid NEON MUL {}: {}", asm, e));
            prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
        }

        #[test]
        fn encode_mul_neg_too_few(
            n in 0usize..=2,
            is_64 in any::<bool>(),
            r0 in 0u32..=31,
            r1 in 0u32..=31,
        ) {
            let all = [
                Operand::Reg(gpr(is_64, r0)),
                Operand::Reg(gpr(is_64, r1)),
            ];
            let ops = &all[..n.min(2)];
            prop_assert!(
                encode_mul(ops).is_err(),
                "fewer than 3 operands must Err, n={}",
                n
            );
        }

        #[test]
        fn encode_mul_neg_extra_operand(
            rd in 0u32..=30,
            rn in 0u32..=30,
            rm in 0u32..=30,
            is_64 in any::<bool>(),
            extra in extra_operand(),
        ) {
            let ops = vec![
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
                Operand::Reg(gpr(is_64, rm)),
                extra,
            ];
            prop_assert!(
                encode_mul(&ops).is_err(),
                "MUL has no 4th operand; extra operand must Err"
            );
        }

        #[test]
        fn encode_mul_neg_mixed_width(
            rd in 0u32..=30,
            rn in 0u32..=30,
            rm in 0u32..=30,
            rd64 in any::<bool>(),
            rn64 in any::<bool>(),
            rm64 in any::<bool>(),
        ) {
            prop_assume!(!(rd64 == rn64 && rn64 == rm64));
            let ops = [
                Operand::Reg(gpr(rd64, rd)),
                Operand::Reg(gpr(rn64, rn)),
                Operand::Reg(gpr(rm64, rm)),
            ];
            prop_assert!(
                encode_mul(&ops).is_err(),
                "mixed-width MUL registers must Err (rd64={} rn64={} rm64={})",
                rd64,
                rn64,
                rm64
            );
        }

        #[test]
        fn encode_mul_diff_lr(
            which in 0u32..=2,
            a in 0u32..=30,
            b in 0u32..=30,
        ) {
            let mut names = [
                gpr(true, a),
                gpr(true, b),
                "lr".to_string(),
            ];
            names.swap(2, which as usize);
            let asm = format!("mul {}, {}, {}", names[0], names[1], names[2]);
            let ops = [
                Operand::Reg(names[0].clone()),
                Operand::Reg(names[1].clone()),
                Operand::Reg(names[2].clone()),
            ];
            let sut = sut_word(&ops)
                .unwrap_or_else(|e| panic!("SUT rejected valid MUL {}: {}", asm, e));
            let mc = llvm_mc_word(&asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid MUL {}: {}", asm, e));
            prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {} (lr alias x30)", asm);
        }

        #[test]
        fn encode_mul_neg_sp(
            which in 0u32..=2,
            is_64 in any::<bool>(),
            a in 0u32..=30,
            b in 0u32..=30,
        ) {
            let sp = if is_64 { "sp" } else { "wsp" };
            let mut names = [
                gpr(is_64, a),
                gpr(is_64, b),
                sp.to_string(),
            ];
            names.swap(2, which as usize);
            let ops = [
                Operand::Reg(names[0].clone()),
                Operand::Reg(names[1].clone()),
                Operand::Reg(names[2].clone()),
            ];
            prop_assert!(
                encode_mul(&ops).is_err(),
                "SP/WSP is not a valid MUL operand (which={} names={:?})",
                which,
                names
            );
        }

        #[test]
        fn encode_mul_neg_fp(
            which in 0u32..=2,
            prefix in prop::sample::select(vec!["d", "s", "q", "v", "h", "b"]),
            n in prop_oneof![Just(0u32), Just(31u32), 0u32..=31],
        ) {
            let fp = format!("{}{}", prefix, n);
            let mut ops = vec![
                Operand::Reg("x0".into()),
                Operand::Reg("x1".into()),
                Operand::Reg("x2".into()),
            ];
            ops[which as usize] = Operand::Reg(fp.clone());
            prop_assert!(
                encode_mul(&ops).is_err(),
                "FP/SIMD register {} is not a valid MUL operand (which={})",
                fp,
                which
            );
        }

        #[test]
        fn encode_mul_neg_neon_d(
            vd in 0u32..=31,
            vn in 0u32..=31,
            vm in 0u32..=31,
            t in prop::sample::select(vec!["1d", "2d"]),
        ) {
            let ops = [
                Operand::RegArrangement {
                    reg: format!("v{vd}"),
                    arrangement: t.to_string(),
                },
                Operand::RegArrangement {
                    reg: format!("v{vn}"),
                    arrangement: t.to_string(),
                },
                Operand::RegArrangement {
                    reg: format!("v{vm}"),
                    arrangement: t.to_string(),
                },
            ];
            prop_assert!(
                encode_mul(&ops).is_err(),
                "NEON MUL size==11 (T={}) is UNDEFINED; must Err",
                t
            );
        }

        #[test]
        fn encode_mul_neg_invalid_reg(
            which in 0u32..=2,
            bad in prop_oneof![
                Just("x32".to_string()),
                Just("w32".to_string()),
                Just("x99".to_string()),
                Just("w99".to_string()),
                Just("".to_string()),
                Just("foo".to_string()),
                Just("r0".to_string()),
                Just("x".to_string()),
                Just("x-1".to_string()),
            ],
        ) {
            let mut ops = vec![
                Operand::Reg("x0".into()),
                Operand::Reg("x1".into()),
                Operand::Reg("x2".into()),
            ];
            ops[which as usize] = Operand::Reg(bad.clone());
            prop_assert!(
                encode_mul(&ops).is_err(),
                "invalid register name {:?} at {} must Err",
                bad,
                which
            );
        }

        #[test]
        fn encode_mul_neg_non_register(
            which in 0u32..=2,
            bad in prop_oneof![
                any::<i64>().prop_map(Operand::Imm),
                Just(Operand::Symbol("foo".into())),
                Just(Operand::Mem {
                    base: "x0".into(),
                    offset: 8,
                }),
                Just(Operand::Shift {
                    kind: "lsl".into(),
                    amount: 0,
                }),
                Just(Operand::Cond("eq".into())),
                Just(Operand::Label("L0".into())),
            ],
        ) {
            let mut ops = vec![
                Operand::Reg("x0".into()),
                Operand::Reg("x1".into()),
                Operand::Reg("x2".into()),
            ];
            ops[which as usize] = bad;
            prop_assert!(
                encode_mul(&ops).is_err(),
                "non-register operand at position {} must Err",
                which
            );
        }

        #[test]
        fn encode_mul_neg_neon_mismatch_t(
            vd in 0u32..=31,
            vn in 0u32..=31,
            vm in 0u32..=31,
            td in neon_t(),
            tn in neon_t(),
            tm in neon_t(),
        ) {
            prop_assume!(td != tn || tn != tm);
            let ops = [
                Operand::RegArrangement {
                    reg: format!("v{vd}"),
                    arrangement: td.to_string(),
                },
                Operand::RegArrangement {
                    reg: format!("v{vn}"),
                    arrangement: tn.to_string(),
                },
                Operand::RegArrangement {
                    reg: format!("v{vm}"),
                    arrangement: tm.to_string(),
                },
            ];
            prop_assert!(
                encode_mul(&ops).is_err(),
                "NEON MUL requires matching T (td={} tn={} tm={})",
                td,
                tn,
                tm
            );
        }
    }

    #[test]
    fn test_encode_mul_regression_extra_operand() {
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("x0".into()),
        ];
        assert!(
            encode_mul(&ops).is_err(),
            "MUL w0, w0, w0, x0 must Err; a 4th operand is not valid (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_mul_regression_mixed_width() {
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("x0".into()),
        ];
        assert!(
            encode_mul(&ops).is_err(),
            "mixed-width MUL w0, w0, x0 must Err (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_mul_regression_sp() {
        let ops = [
            Operand::Reg("wsp".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
        ];
        assert!(
            encode_mul(&ops).is_err(),
            "MUL wsp, w0, w0 must Err; register 31 is WZR not WSP (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_mul_regression_fp_reg() {
        let ops = [
            Operand::Reg("d0".into()),
            Operand::Reg("x1".into()),
            Operand::Reg("x2".into()),
        ];
        assert!(
            encode_mul(&ops).is_err(),
            "MUL d0, x1, x2 must Err; FP/SIMD registers are not MUL operands (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_mul_regression_neon_d() {
        let ops = [
            Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "1d".into(),
            },
            Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "1d".into(),
            },
            Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "1d".into(),
            },
        ];
        assert!(
            encode_mul(&ops).is_err(),
            "MUL v0.1d, v0.1d, v0.1d must Err; size==11 is UNDEFINED (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_mul_regression_neon_mismatch_t() {
        let ops = [
            Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "8b".into(),
            },
            Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "8b".into(),
            },
            Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "16b".into(),
            },
        ];
        assert!(
            encode_mul(&ops).is_err(),
            "MUL v0.8b, v0.8b, v0.16b must Err; NEON MUL requires matching T (llvm-mc rejects it)"
        );
    }
}