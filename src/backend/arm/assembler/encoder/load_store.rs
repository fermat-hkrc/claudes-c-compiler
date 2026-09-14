use super::*;
use crate::backend::arm::assembler::parser::Operand;

// ── Loads/Stores ─────────────────────────────────────────────────────────

/// Auto-detect LDR/STR size from the first register operand.
pub(crate) fn encode_ldr_str_auto(operands: &[Operand], is_load: bool) -> Result<EncodeResult, String> {
    // Determine size from register: Wn -> 32-bit (size=10), Xn -> 64-bit (size=11)
    // FP: Sn -> 32-bit, Dn -> 64-bit, Qn -> 128-bit
    let reg_name = match operands.first() {
        Some(Operand::Reg(r)) => r.to_lowercase(),
        _ => return Err("ldr/str needs register operand".to_string()),
    };

    let size = if reg_name.starts_with('w') {
        0b10 // 32-bit
    } else if reg_name.starts_with('x') || reg_name == "sp" || reg_name == "xzr" || reg_name == "lr" {
        0b11 // 64-bit
    } else if reg_name.starts_with('s') {
        0b10 // 32-bit float
    } else if reg_name.starts_with('d') {
        0b11 // 64-bit float
    } else if reg_name.starts_with('q') {
        0b00 // 128-bit: size=00 with opc adjustment in encode_ldr_str
    } else {
        0b11 // default 64-bit
    };

    let is_128bit = reg_name.starts_with('q');
    encode_ldr_str(operands, is_load, size, false, is_128bit)
}

pub(crate) fn encode_ldr_str(operands: &[Operand], is_load: bool, size: u32, is_signed: bool, is_128bit: bool) -> Result<EncodeResult, String> {
    if operands.len() < 2 {
        return Err("ldr/str requires at least 2 operands".to_string());
    }

    let (rt, _) = get_reg(operands, 0)?;
    let fp = is_fp_reg(operands.first().map(|o| match o { Operand::Reg(r) => r.as_str(), _ => "" }).unwrap_or(""));

    // Use the size parameter as-is (auto-detection happens in encode_ldr_str_auto)
    let actual_size = size;

    let v = if fp { 1u32 } else { 0u32 };

    match operands.get(1) {
        // [base, #offset]
        Some(Operand::Mem { base, offset }) => {
            let rn = parse_reg_num(base).ok_or("invalid base reg")?;

            // Unsigned offset encoding
            // Size determines the shift for offset alignment
            // For 128-bit Q registers: shift=4, opc=11 (load) or 10 (store)
            let shift = if is_128bit { 4 } else { actual_size };
            let opc = if is_128bit {
                if is_load { 0b11 } else { 0b10 }
            } else if is_load {
                if is_signed { 0b10 } else { 0b01 }
            } else {
                0b00
            };

            // Check if offset is aligned and fits in 12-bit unsigned field
            let abs_offset = *offset as u64;
            let align = 1u64 << shift;
            if *offset >= 0 && abs_offset.is_multiple_of(align) {
                let imm12 = (abs_offset / align) as u32;
                if imm12 < 4096 {
                    // Unsigned offset form: size 111 V 01 opc imm12 Rn Rt
                    let word = (actual_size << 30) | (0b111 << 27) | (v << 26) | (0b01 << 24)
                        | (opc << 22) | (imm12 << 10) | (rn << 5) | rt;
                    return Ok(EncodeResult::Word(word));
                }
            }

            // Unscaled offset (LDUR/STUR form)
            let imm9 = (*offset as i32) & 0x1FF;
            let opc = if is_128bit {
                if is_load { 0b11 } else { 0b10 }
            } else if is_load {
                if is_signed { 0b10 } else { 0b01 }
            } else {
                0b00
            };
            let word = (((actual_size << 30) | (0b111 << 27) | (v << 26)) | (opc << 22)
                | ((imm9 as u32 & 0x1FF) << 12)) | (rn << 5) | rt;
            return Ok(EncodeResult::Word(word));
        }

        // [base, #offset]! (pre-index)
        Some(Operand::MemPreIndex { base, offset }) => {
            let rn = parse_reg_num(base).ok_or("invalid base reg")?;
            let imm9 = (*offset as i32) & 0x1FF;
            let opc = if is_128bit {
                if is_load { 0b11 } else { 0b10 }
            } else if is_load { 0b01 } else { 0b00 };
            let word = ((actual_size << 30) | (0b111 << 27) | (v << 26)) | (opc << 22)
                | ((imm9 as u32 & 0x1FF) << 12) | (0b11 << 10) | (rn << 5) | rt;
            return Ok(EncodeResult::Word(word));
        }

        // [base], #offset (post-index)
        Some(Operand::MemPostIndex { base, offset }) => {
            let rn = parse_reg_num(base).ok_or("invalid base reg")?;
            let imm9 = (*offset as i32) & 0x1FF;
            let opc = if is_128bit {
                if is_load { 0b11 } else { 0b10 }
            } else if is_load { 0b01 } else { 0b00 };
            let word = ((actual_size << 30) | (0b111 << 27) | (v << 26)) | (opc << 22)
                | ((imm9 as u32 & 0x1FF) << 12) | (0b01 << 10) | (rn << 5) | rt;
            return Ok(EncodeResult::Word(word));
        }

        // [base, Xm] register offset
        Some(Operand::MemRegOffset { base, index, extend, shift }) => {
            // Check if index is a :lo12: modifier
            if index.starts_with(':') {
                // Parse modifier from the index string
                let rn = parse_reg_num(base).ok_or("invalid base reg")?;
                let mod_str = index.trim_start_matches(':');
                let (kind, sym) = if let Some(colon_pos) = mod_str.find(':') {
                    (&mod_str[..colon_pos], &mod_str[colon_pos + 1..])
                } else {
                    return Err(format!("malformed modifier in memory operand: {}", index));
                };

                let (symbol, addend) = if let Some(plus_pos) = sym.find('+') {
                    let s = &sym[..plus_pos];
                    let off: i64 = sym[plus_pos + 1..].parse().unwrap_or(0);
                    (s.to_string(), off)
                } else {
                    (sym.to_string(), 0i64)
                };

                let opc = if is_128bit {
                    if is_load { 0b11 } else { 0b10 }
                } else if is_load { 0b01 } else { 0b00 };

                let reloc_type = match kind {
                    "lo12" => {
                        if is_128bit {
                            RelocType::Ldst128AbsLo12
                        } else {
                            match actual_size {
                                0b00 => RelocType::Ldst8AbsLo12,
                                0b01 => RelocType::Ldst16AbsLo12,
                                0b10 => RelocType::Ldst32AbsLo12,
                                0b11 => RelocType::Ldst64AbsLo12,
                                _ => RelocType::Ldst64AbsLo12,
                            }
                        }
                    }
                    "got_lo12" => RelocType::Ld64GotLo12,
                    _ => return Err(format!("unsupported modifier in load/store: {}", kind)),
                };

                let word = ((actual_size << 30) | (0b111 << 27) | (v << 26) | (0b01 << 24) | (opc << 22)) | (rn << 5) | rt;
                return Ok(EncodeResult::WordWithReloc {
                    word,
                    reloc: Relocation {
                        reloc_type,
                        symbol,
                        addend,
                    },
                });
            }

            let rn = parse_reg_num(base).ok_or("invalid base reg")?;
            let rm = parse_reg_num(index).ok_or("invalid index reg")?;
            let opc = if is_128bit {
                if is_load { 0b11 } else { 0b10 }
            } else if is_load { 0b01 } else { 0b00 };
            // Register offset: size 111 V opc 1 Rm option S 10 Rn Rt
            // Determine option and S from extend/shift specifiers
            let is_w_index = index.starts_with('w') || index.starts_with('W');
            let shift_amount: u8 = match shift { Some(s) => *s, None => 0 };
            let (option, s_bit) = match extend.as_deref() {
                Some("lsl") => {
                    // LSL with shift: S=1 if shift amount > 0
                    let s_val = if shift_amount > 0 { 1u32 } else { 0u32 };
                    (0b011u32, s_val)
                }
                Some("sxtw") => {
                    let s_val = if shift_amount > 0 { 1u32 } else { 0u32 };
                    (0b110u32, s_val)
                }
                Some("sxtx") => {
                    let s_val = if shift_amount > 0 { 1u32 } else { 0u32 };
                    (0b111u32, s_val)
                }
                Some("uxtw") => {
                    let s_val = if shift_amount > 0 { 1u32 } else { 0u32 };
                    (0b010u32, s_val)
                }
                Some("uxtx") => {
                    let s_val = if shift_amount > 0 { 1u32 } else { 0u32 };
                    (0b011u32, s_val)
                }
                None => {
                    // Default: if W register index, use UXTW; if X register, use LSL
                    if is_w_index {
                        (0b010u32, 0u32) // UXTW, no shift
                    } else {
                        (0b011u32, 0u32) // LSL, no shift
                    }
                }
                _ => (0b011u32, 0u32), // default LSL
            };
            let word = (actual_size << 30) | (0b111 << 27) | (v << 26) | (opc << 22)
                | (1 << 21) | (rm << 16) | (option << 13) | (s_bit << 12) | (0b10 << 10) | (rn << 5) | rt;
            return Ok(EncodeResult::Word(word));
        }

        // LDR (literal): ldr Rt, label — PC-relative load
        Some(Operand::Symbol(sym)) if is_load => {
            // opc V 011 00 imm19 Rt
            // For GP registers: opc=00 → 32-bit (W), opc=01 → 64-bit (X), opc=11 → PRFM
            // For FP/SIMD:      opc=00 → 32-bit (S), opc=01 → 64-bit (D), opc=10 → 128-bit (Q)
            // Note: actual_size uses 10=32-bit, 11=64-bit but LDR literal uses 00=32-bit, 01=64-bit
            let opc = if is_128bit {
                0b10u32
            } else if fp {
                // FP: S=00, D=01 (same mapping as GP)
                if actual_size == 0b11 { 0b01 } else { 0b00 }
            } else {
                // GP: W=00, X=01
                if actual_size == 0b11 { 0b01 } else { 0b00 }
            };
            let word = (opc << 30) | (v << 26) | (0b011 << 27) | rt;
            return Ok(EncodeResult::WordWithReloc {
                word,
                reloc: Relocation {
                    reloc_type: RelocType::Ldr19,
                    symbol: sym.clone(),
                    addend: 0,
                },
            });
        }

        _ => {}
    }

    Err(format!("unsupported ldr/str operands: {:?}", operands))
}

/// Encode LDUR/STUR (unscaled immediate offset load/store)
/// Format: size 111 V 00 opc 0 imm9 00 Rn Rt
pub(crate) fn encode_ldur_stur(operands: &[Operand], is_load: bool, op2_bits: u32) -> Result<EncodeResult, String> {
    if operands.len() < 2 {
        return Err("ldur/stur requires 2 operands".to_string());
    }
    let (rt, _) = get_reg(operands, 0)?;
    let reg_name = match &operands[0] { Operand::Reg(r) => r.to_lowercase(), _ => String::new() };
    let fp = is_fp_reg(&reg_name);
    let v = if fp { 1u32 } else { 0u32 };

    let (size, opc) = if fp {
        if reg_name.starts_with('q') {
            (0b00u32, if is_load { 0b11u32 } else { 0b10 })
        } else if reg_name.starts_with('d') {
            (0b11, if is_load { 0b01 } else { 0b00 })
        } else if reg_name.starts_with('s') {
            (0b10, if is_load { 0b01 } else { 0b00 })
        } else if reg_name.starts_with('h') {
            (0b01, if is_load { 0b01 } else { 0b00 })
        } else if reg_name.starts_with('b') {
            (0b00, if is_load { 0b01 } else { 0b00 })
        } else {
            (0b11, if is_load { 0b01 } else { 0b00 })
        }
    } else {
        let is_64 = reg_name.starts_with('x');
        let sz = if is_64 { 0b11u32 } else { 0b10 };
        (sz, if is_load { 0b01u32 } else { 0b00 })
    };

    let (rn, imm9) = match &operands[1] {
        Operand::Mem { base, offset } => {
            let rn = parse_reg_num(base).ok_or("invalid base reg")?;
            (rn, *offset as i32)
        }
        _ => return Err(format!("ldur/stur: expected memory operand, got {:?}", operands[1])),
    };

    let imm9_enc = (imm9 as u32) & 0x1FF;
    let word = (size << 30) | (0b111 << 27) | (v << 26) | (opc << 22)
        | (imm9_enc << 12) | (op2_bits << 10) | (rn << 5) | rt;
    Ok(EncodeResult::Word(word))
}

/// Encode LDTR/STTR with explicit size (for ldtrh, ldtrb, etc.)
pub(crate) fn encode_ldtr_sized(operands: &[Operand], is_load: bool, size: u32) -> Result<EncodeResult, String> {
    if operands.len() < 2 {
        return Err("ldtr/sttr requires 2 operands".to_string());
    }
    let (rt, _) = get_reg(operands, 0)?;
    let opc = if is_load { 0b01u32 } else { 0b00 };
    let (rn, imm9) = match &operands[1] {
        Operand::Mem { base, offset } => {
            let rn = parse_reg_num(base).ok_or("invalid base reg")?;
            (rn, *offset as i32)
        }
        _ => return Err("ldtr/sttr: expected memory operand".to_string()),
    };
    let imm9_enc = (imm9 as u32) & 0x1FF;
    let word = (size << 30) | (0b111 << 27) | (opc << 22)
        | (imm9_enc << 12) | (0b10 << 10) | (rn << 5) | rt;
    Ok(EncodeResult::Word(word))
}

pub(crate) fn encode_ldrsw(operands: &[Operand]) -> Result<EncodeResult, String> {
    if operands.len() < 2 {
        return Err("ldrsw requires 2 operands".to_string());
    }

    let (rt, _) = get_reg(operands, 0)?;

    match operands.get(1) {
        Some(Operand::Mem { base, offset }) => {
            let rn = parse_reg_num(base).ok_or("invalid base reg")?;
            // LDRSW: size=10 111 V=0 01 opc=10 -> unsigned offset
            // Actually: 10 111 0 01 10 imm12 Rn Rt
            let abs_offset = *offset as u64;
            if *offset >= 0 && abs_offset.is_multiple_of(4) {
                let imm12 = (abs_offset / 4) as u32;
                if imm12 < 4096 {
                    let word = ((0b10 << 30) | (0b111 << 27)) | (0b01 << 24) | (0b10 << 22)
                        | (imm12 << 10) | (rn << 5) | rt;
                    return Ok(EncodeResult::Word(word));
                }
            }
            // Unscaled: LDURSW
            let imm9 = (*offset as i32) & 0x1FF;
            let word = (((0b10 << 30) | (0b111 << 27)) | (0b10 << 22)
                | ((imm9 as u32 & 0x1FF) << 12)) | (rn << 5) | rt;
            return Ok(EncodeResult::Word(word));
        }

        Some(Operand::MemPostIndex { base, offset }) => {
            let rn = parse_reg_num(base).ok_or("invalid base reg")?;
            let imm9 = (*offset as i32) & 0x1FF;
            let word = ((0b10 << 30) | (0b111 << 27)) | (0b10 << 22)
                | ((imm9 as u32 & 0x1FF) << 12) | (0b01 << 10) | (rn << 5) | rt;
            return Ok(EncodeResult::Word(word));
        }

        Some(Operand::MemPreIndex { base, offset }) => {
            let rn = parse_reg_num(base).ok_or("invalid base reg")?;
            let imm9 = (*offset as i32) & 0x1FF;
            let word = ((0b10 << 30) | (0b111 << 27)) | (0b10 << 22)
                | ((imm9 as u32 & 0x1FF) << 12) | (0b11 << 10) | (rn << 5) | rt;
            return Ok(EncodeResult::Word(word));
        }

        Some(Operand::MemRegOffset { base, index, extend, shift }) => {
            let rn = parse_reg_num(base).ok_or("invalid base reg")?;
            let rm = parse_reg_num(index).ok_or("invalid index reg")?;
            let (option, s_bit) = match (extend.as_deref(), shift) {
                (Some("lsl"), Some(2)) => (0b011u32, 1u32),
                (Some("lsl"), Some(0)) | (Some("lsl"), None) => (0b011, 0),
                (None, None) | (None, Some(0)) => (0b011, 0),
                (Some("sxtw"), Some(2)) => (0b110, 1),
                (Some("sxtw"), Some(0)) | (Some("sxtw"), None) => (0b110, 0),
                (Some("uxtw"), Some(2)) => (0b010, 1),
                (Some("uxtw"), Some(0)) | (Some("uxtw"), None) => (0b010, 0),
                (Some("sxtx"), Some(2)) => (0b111, 1),
                (Some("sxtx"), Some(0)) | (Some("sxtx"), None) => (0b111, 0),
                _ => return Err(format!("unsupported ldrsw extend/shift: {:?}/{:?}", extend, shift)),
            };
            // LDRSW reg: 10 111 0 00 10 1 Rm option S 10 Rn Rt
            let word = (0b10 << 30) | (0b111 << 27) | (0b10 << 22) | (1 << 21)
                | (rm << 16) | (option << 13) | (s_bit << 12) | (0b10 << 10) | (rn << 5) | rt;
            return Ok(EncodeResult::Word(word));
        }

        _ => {}
    }

    Err(format!("unsupported ldrsw operands: {:?}", operands))
}

pub(crate) fn encode_ldrs(operands: &[Operand], size: u32) -> Result<EncodeResult, String> {
    // LDRSB/LDRSH: sign-extending byte/halfword loads
    if operands.len() < 2 {
        return Err("ldrsb/ldrsh requires 2 operands".to_string());
    }

    let (rt, is_64) = get_reg(operands, 0)?;
    let opc = if is_64 { 0b10 } else { 0b11 }; // 64-bit target: opc=10, 32-bit: opc=11

    if let Some(Operand::Mem { base, offset }) = operands.get(1) {
        let rn = parse_reg_num(base).ok_or("invalid base reg")?;
        let shift = size;
        let abs_offset = *offset as u64;
        let align = 1u64 << shift;
        if *offset >= 0 && abs_offset.is_multiple_of(align) {
            let imm12 = (abs_offset / align) as u32;
            if imm12 < 4096 {
                let word = ((size << 30) | (0b111 << 27)) | (0b01 << 24) | (opc << 22)
                    | (imm12 << 10) | (rn << 5) | rt;
                return Ok(EncodeResult::Word(word));
            }
        }
        // Unscaled
        let imm9 = (*offset as i32) & 0x1FF;
        let word = (((size << 30) | (0b111 << 27)) | (opc << 22)
            | ((imm9 as u32 & 0x1FF) << 12)) | (rn << 5) | rt;
        return Ok(EncodeResult::Word(word));
    }

    // Post-index: ldrsb/ldrsh Rt, [Xn], #imm
    if let Some(Operand::MemPostIndex { base, offset }) = operands.get(1) {
        let rn = parse_reg_num(base).ok_or("invalid base reg")?;
        let imm9 = (*offset as i32) & 0x1FF;
        let word = (size << 30) | (0b111 << 27) | (opc << 22)
            | ((imm9 as u32 & 0x1FF) << 12) | (0b01 << 10) | (rn << 5) | rt;
        return Ok(EncodeResult::Word(word));
    }

    // Pre-index: ldrsb/ldrsh Rt, [Xn, #imm]!
    if let Some(Operand::MemPreIndex { base, offset }) = operands.get(1) {
        let rn = parse_reg_num(base).ok_or("invalid base reg")?;
        let imm9 = (*offset as i32) & 0x1FF;
        let word = (size << 30) | (0b111 << 27) | (opc << 22)
            | ((imm9 as u32 & 0x1FF) << 12) | (0b11 << 10) | (rn << 5) | rt;
        return Ok(EncodeResult::Word(word));
    }

    // Register offset: ldrsb/ldrsh Rt, [Xn, Xm{, extend {#amount}}]
    if let Some(Operand::MemRegOffset { base, index, extend, shift }) = operands.get(1) {
        let rn = parse_reg_num(base).ok_or("invalid base reg")?;
        let rm = parse_reg_num(index).ok_or("invalid index reg")?;
        let is_w_index = index.starts_with('w') || index.starts_with('W');
        let shift_amount: u8 = match shift { Some(s) => *s, None => 0 };
        let (option, s_bit) = match extend.as_deref() {
            Some("lsl") => (0b011u32, if shift_amount > 0 { 1u32 } else { 0 }),
            Some("sxtw") => (0b110u32, if shift_amount > 0 { 1u32 } else { 0 }),
            Some("sxtx") => (0b111u32, if shift_amount > 0 { 1u32 } else { 0 }),
            Some("uxtw") => (0b010u32, if shift_amount > 0 { 1u32 } else { 0 }),
            Some("uxtx") => (0b011u32, if shift_amount > 0 { 1u32 } else { 0 }),
            None => if is_w_index { (0b010u32, 0u32) } else { (0b011u32, 0u32) },
            _ => (0b011u32, 0u32),
        };
        let word = (size << 30) | (0b111 << 27) | (opc << 22) | (1 << 21)
            | (rm << 16) | (option << 13) | (s_bit << 12) | (0b10 << 10) | (rn << 5) | rt;
        return Ok(EncodeResult::Word(word));
    }

    Err(format!("unsupported ldrsb/ldrsh operands: {:?}", operands))
}

pub(crate) fn encode_ldp_stp(operands: &[Operand], is_load: bool) -> Result<EncodeResult, String> {
    if operands.len() < 3 {
        return Err("ldp/stp requires 3 operands".to_string());
    }

    let (rt1, is_64) = get_reg(operands, 0)?;
    let (rt2, _) = get_reg(operands, 1)?;
    let fp = is_fp_reg(match &operands[0] { Operand::Reg(r) => r.as_str(), _ => "" });

    let opc = if fp {
        let r = match &operands[0] { Operand::Reg(r) => r.to_lowercase(), _ => String::new() };
        if r.starts_with('s') { 0b00 }
        else if r.starts_with('d') { 0b01 }
        else if r.starts_with('q') || is_64 { 0b10 }
        else { 0b00 }
    } else if is_64 { 0b10 } else { 0b00 };

    let v = if fp { 1u32 } else { 0u32 };
    let l = if is_load { 1u32 } else { 0u32 };

    // Shift depends on register size
    let shift = if fp {
        let r = match &operands[0] { Operand::Reg(r) => r.to_lowercase(), _ => String::new() };
        if r.starts_with('s') { 2 }
        else if r.starts_with('d') { 3 }
        else if r.starts_with('q') { 4 }
        else if is_64 { 3 } else { 2 }
    } else if is_64 { 3 } else { 2 };

    match operands.get(2) {
        // STP rt1, rt2, [base, #offset]! (pre-index)
        Some(Operand::MemPreIndex { base, offset }) => {
            let rn = parse_reg_num(base).ok_or("invalid base reg")?;
            let imm7 = ((*offset >> shift) as i32) & 0x7F;
            let word = (opc << 30) | (0b101 << 27) | (v << 26) | (0b011 << 23) | (l << 22)
                | ((imm7 as u32 & 0x7F) << 15) | (rt2 << 10) | (rn << 5) | rt1;
            return Ok(EncodeResult::Word(word));
        }

        // LDP/STP rt1, rt2, [base], #offset (post-index)
        Some(Operand::MemPostIndex { base, offset }) => {
            let rn = parse_reg_num(base).ok_or("invalid base reg")?;
            let imm7 = ((*offset >> shift) as i32) & 0x7F;
            let word = (opc << 30) | (0b101 << 27) | (v << 26) | (0b001 << 23) | (l << 22)
                | ((imm7 as u32 & 0x7F) << 15) | (rt2 << 10) | (rn << 5) | rt1;
            return Ok(EncodeResult::Word(word));
        }

        // LDP/STP rt1, rt2, [base, #offset] (signed offset)
        Some(Operand::Mem { base, offset }) => {
            let rn = parse_reg_num(base).ok_or("invalid base reg")?;
            let imm7 = ((*offset >> shift) as i32) & 0x7F;
            let word = (opc << 30) | (0b101 << 27) | (v << 26) | (0b010 << 23) | (l << 22)
                | ((imm7 as u32 & 0x7F) << 15) | (rt2 << 10) | (rn << 5) | rt1;
            return Ok(EncodeResult::Word(word));
        }

        _ => {}
    }

    Err(format!("unsupported ldp/stp operands: {:?}", operands))
}

/// Encode LDNP/STNP (load/store pair non-temporal)
/// Encoding: opc 101 V 000 L imm7 Rt2 Rn Rt
/// TODO: Only handles integer registers (V=0). FP/SIMD register support needed for V=1.
pub(crate) fn encode_ldnp_stnp(operands: &[Operand], is_load: bool) -> Result<EncodeResult, String> {
    if operands.len() < 3 {
        return Err("ldnp/stnp requires 3 operands".to_string());
    }

    let (rt1, is_64) = get_reg(operands, 0)?;
    let (rt2, _) = get_reg(operands, 1)?;

    let opc: u32 = if is_64 { 0b10 } else { 0b00 };
    let l: u32 = if is_load { 1 } else { 0 };
    let shift = if is_64 { 3 } else { 2 }; // scale factor: 8 for 64-bit, 4 for 32-bit

    match operands.get(2) {
        Some(Operand::Mem { base, offset }) => {
            let rn = parse_reg_num(base).ok_or("invalid base reg")?;
            let imm7 = ((*offset >> shift) as i32) & 0x7F;
            // LDNP/STNP: opc 101 V=0 000 L imm7 Rt2 Rn Rt
            let word = (opc << 30) | (0b101 << 27) | (l << 22)
                | ((imm7 as u32 & 0x7F) << 15) | (rt2 << 10) | (rn << 5) | rt1;
            Ok(EncodeResult::Word(word))
        }
        _ => Err(format!("unsupported ldnp/stnp operands: {:?}", operands)),
    }
}

// ── Exclusive loads/stores ───────────────────────────────────────────────

/// Encode LDXR/STXR and byte/halfword variants.
/// `forced_size`: None = auto-detect from register width, Some(0b00) = byte, Some(0b01) = halfword
pub(crate) fn encode_ldxr_stxr(operands: &[Operand], is_load: bool, forced_size: Option<u32>) -> Result<EncodeResult, String> {
    if is_load {
        let (rt, is_64) = get_reg(operands, 0)?;
        let rn = match operands.get(1) {
            Some(Operand::Mem { base, .. }) => parse_reg_num(base).ok_or("invalid base")?,
            _ => return Err("ldxr needs memory operand".to_string()),
        };
        let size = forced_size.unwrap_or(if is_64 { 0b11 } else { 0b10 });
        let word = ((size << 30) | (0b001000010 << 21) | (0b11111 << 16))
            | (0b11111 << 10) | (rn << 5) | rt;
        Ok(EncodeResult::Word(word))
    } else {
        let (ws, _) = get_reg(operands, 0)?;
        let (rt, is_64) = get_reg(operands, 1)?;
        let rn = match operands.get(2) {
            Some(Operand::Mem { base, .. }) => parse_reg_num(base).ok_or("invalid base")?,
            _ => return Err("stxr needs memory operand".to_string()),
        };
        let size = forced_size.unwrap_or(if is_64 { 0b11 } else { 0b10 });
        let word = ((size << 30) | (0b001000000 << 21) | (ws << 16))
            | (0b11111 << 10) | (rn << 5) | rt;
        Ok(EncodeResult::Word(word))
    }
}

/// Encode LDAXR/STLXR and byte/halfword variants.
pub(crate) fn encode_ldaxr_stlxr(operands: &[Operand], is_load: bool, forced_size: Option<u32>) -> Result<EncodeResult, String> {
    if is_load {
        let (rt, is_64) = get_reg(operands, 0)?;
        let rn = match operands.get(1) {
            Some(Operand::Mem { base, .. }) => parse_reg_num(base).ok_or("invalid base")?,
            _ => return Err("ldaxr needs memory operand".to_string()),
        };
        let size = forced_size.unwrap_or(if is_64 { 0b11 } else { 0b10 });
        let word = (size << 30) | (0b001000010 << 21) | (0b11111 << 16) | (1 << 15)
            | (0b11111 << 10) | (rn << 5) | rt;
        Ok(EncodeResult::Word(word))
    } else {
        let (ws, _) = get_reg(operands, 0)?;
        let (rt, is_64) = get_reg(operands, 1)?;
        let rn = match operands.get(2) {
            Some(Operand::Mem { base, .. }) => parse_reg_num(base).ok_or("invalid base")?,
            _ => return Err("stlxr needs memory operand".to_string()),
        };
        let size = forced_size.unwrap_or(if is_64 { 0b11 } else { 0b10 });
        let word = (size << 30) | (0b001000000 << 21) | (ws << 16) | (1 << 15)
            | (0b11111 << 10) | (rn << 5) | rt;
        Ok(EncodeResult::Word(word))
    }
}

/// Encode LDXP/STXP/LDAXP/STLXP (exclusive pair) instructions.
///
/// LDXP  Xt1, Xt2, [Xn]  : sz 001000 0 1 1 11111 0 Rt2 Rn Rt
/// LDAXP Xt1, Xt2, [Xn]  : sz 001000 0 1 1 11111 1 Rt2 Rn Rt
/// STXP  Ws, Xt1, Xt2, [Xn] : sz 001000 0 0 1 Rs 0 Rt2 Rn Rt
/// STLXP Ws, Xt1, Xt2, [Xn] : sz 001000 0 0 1 Rs 1 Rt2 Rn Rt
pub(crate) fn encode_ldxp_stxp(operands: &[Operand], is_load: bool, acquire_release: bool) -> Result<EncodeResult, String> {
    let o0 = if acquire_release { 1u32 } else { 0 };
    if is_load {
        // LDXP/LDAXP Rt, Rt2, [Rn]
        let (rt, is_64) = get_reg(operands, 0)?;
        let (rt2, _) = get_reg(operands, 1)?;
        let rn = match operands.get(2) {
            Some(Operand::Mem { base, .. }) => parse_reg_num(base).ok_or("ldxp needs memory operand")?,
            _ => return Err("ldxp needs memory operand".to_string()),
        };
        let sz = if is_64 { 1u32 } else { 0 };
        // 1 sz 001000 0 1 1 11111 o0 Rt2 Rn Rt  (bit23=0)
        let word = (1u32 << 31) | (sz << 30) | (0b001000 << 24) | (1 << 22)
            | (1 << 21) | (0b11111 << 16) | (o0 << 15) | (rt2 << 10) | (rn << 5) | rt;
        Ok(EncodeResult::Word(word))
    } else {
        // STXP/STLXP Ws, Rt, Rt2, [Rn]
        let (ws, _) = get_reg(operands, 0)?;  // status register (always W)
        let (rt, is_64) = get_reg(operands, 1)?;
        let (rt2, _) = get_reg(operands, 2)?;
        let rn = match operands.get(3) {
            Some(Operand::Mem { base, .. }) => parse_reg_num(base).ok_or("stxp needs memory operand")?,
            _ => return Err("stxp needs memory operand".to_string()),
        };
        let sz = if is_64 { 1u32 } else { 0 };
        // 1 sz 001000 0 0 1 Rs o0 Rt2 Rn Rt  (bit23=0, bit22=0)
        let word = (1u32 << 31) | (sz << 30) | (0b001000 << 24)
            | (1 << 21) | (ws << 16) | (o0 << 15) | (rt2 << 10) | (rn << 5) | rt;
        Ok(EncodeResult::Word(word))
    }
}

/// Encode LDAR/STLR and byte/halfword variants.
pub(crate) fn encode_ldar_stlr(operands: &[Operand], is_load: bool, forced_size: Option<u32>) -> Result<EncodeResult, String> {
    let (rt, is_64) = get_reg(operands, 0)?;
    let rn = match operands.get(1) {
        Some(Operand::Mem { base, .. }) => parse_reg_num(base).ok_or("invalid base")?,
        _ => return Err("ldar/stlr needs memory operand".to_string()),
    };
    let size = forced_size.unwrap_or(if is_64 { 0b11 } else { 0b10 });
    let l = if is_load { 1u32 } else { 0 };
    // LDAR/STLR: size 001000 1 L 0 11111 1 11111 Rn Rt
    let word = ((size << 30) | (0b001000 << 24) | (1 << 23) | (l << 22))
        | (0b11111 << 16) | (1 << 15) | (0b11111 << 10) | (rn << 5) | rt;
    Ok(EncodeResult::Word(word))
}

// ── Address computation ──────────────────────────────────────────────────

pub(crate) fn encode_adrp(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, _) = get_reg(operands, 0)?;

    let (sym, addend) = match operands.get(1) {
        Some(Operand::Symbol(s)) => (s.clone(), 0i64),
        Some(Operand::Modifier { kind, symbol }) if kind == "got" => {
            // adrp x0, :got:symbol
            let word = (1u32 << 31) | (0b10000 << 24) | rd;
            return Ok(EncodeResult::WordWithReloc {
                word,
                reloc: Relocation {
                    reloc_type: RelocType::AdrGotPage21,
                    symbol: symbol.clone(),
                    addend: 0,
                },
            });
        }
        Some(Operand::SymbolOffset(s, off)) => (s.clone(), *off),
        Some(Operand::Label(s)) => (s.clone(), 0i64),
        // Parser misclassifies symbol names that collide with register names (s1, v0, d1, etc.),
        // condition codes (cc, lt, le), or barrier names (st, ld).
        // ADRP never takes these as actual operand types, so treat them as symbols.
        Some(Operand::Reg(name)) => (name.clone(), 0i64),
        Some(Operand::Cond(name)) => (name.clone(), 0i64),
        Some(Operand::Barrier(name)) => (name.clone(), 0i64),
        _ => return Err(format!("adrp needs symbol operand, got {:?}", operands.get(1))),
    };

    // ADRP: 1 immlo[1:0] 10000 immhi[18:0] Rd
    let word = (1u32 << 31) | (0b10000 << 24) | rd;
    Ok(EncodeResult::WordWithReloc {
        word,
        reloc: Relocation {
            reloc_type: RelocType::AdrpPage21,
            symbol: sym,
            addend,
        },
    })
}

pub(crate) fn encode_adr(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, _) = get_reg(operands, 0)?;

    // Check for immediate offset form: adr Rd, #imm
    // TODO: validate 21-bit signed immediate range
    if let Some(Operand::Imm(imm)) = operands.get(1) {
        let imm = *imm;
        // ADR: 0 immlo[1:0] 10000 immhi[18:0] Rd
        let immlo = ((imm as u32) & 3) << 29;
        let immhi = (((imm as u32) >> 2) & 0x7FFFF) << 5;
        let word = immlo | (0b10000 << 24) | immhi | rd;
        return Ok(EncodeResult::Word(word));
    }

    let (sym, addend) = get_symbol(operands, 1)?;
    // ADR: 0 immlo[1:0] 10000 immhi[18:0] Rd
    let word = (0b10000 << 24) | rd;
    Ok(EncodeResult::WordWithReloc {
        word,
        reloc: Relocation {
            reloc_type: RelocType::AdrPrelLo21,
            symbol: sym,
            addend,
        },
    })
}

// ── Prefetch ─────────────────────────────────────────────────────────────

/// Encode the PRFM (prefetch memory) instruction.
/// Format: PRFM <prfop>, [<Xn|SP>{, #<pimm>}]
/// Encoding: 1111 1001 10 imm12 Rn Rt
/// where Rt is the 5-bit prefetch operation type.
pub(crate) fn encode_prfm(operands: &[Operand]) -> Result<EncodeResult, String> {
    if operands.len() < 2 {
        return Err("prfm requires 2 operands".to_string());
    }

    // First operand: prefetch operation type (parsed as Symbol)
    let prfop = match &operands[0] {
        Operand::Symbol(s) => encode_prfop(s)?,
        Operand::Imm(v) => {
            if *v < 0 || *v > 31 {
                return Err(format!("prfm: immediate prefetch type out of range: {}", v));
            }
            *v as u32
        }
        _ => return Err(format!("prfm: expected prefetch operation name, got {:?}", operands[0])),
    };

    // Second operand: memory address [Xn{, #imm}]
    match &operands[1] {
        Operand::Mem { base, offset } => {
            let rn = parse_reg_num(base).ok_or_else(|| format!("prfm: invalid base register: {}", base))?;
            let imm = *offset;
            if imm < 0 || imm % 8 != 0 {
                return Err(format!("prfm: offset must be non-negative and 8-byte aligned, got {}", imm));
            }
            let imm12 = (imm / 8) as u32;
            if imm12 > 0xFFF {
                return Err(format!("prfm: offset too large: {}", imm));
            }
            // PRFM (imm): 1111 1001 10 imm12(12) Rn(5) Rt(5)
            let word = 0xF9800000 | (imm12 << 10) | (rn << 5) | prfop;
            Ok(EncodeResult::Word(word))
        }
        Operand::Symbol(_sym) => {
            // PRFM (literal) with symbol reference is not yet supported
            Err("prfm with symbol/label operand not yet supported".to_string())
        }
        Operand::MemRegOffset { base, index, extend, shift } => {
            // PRFM (register): 11 111 0 00 10 1 Rm option S 10 Rn Rt
            let rn = parse_reg_num(base).ok_or_else(|| format!("prfm: invalid base register: {}", base))?;
            let rm = parse_reg_num(index).ok_or_else(|| format!("prfm: invalid index register: {}", index))?;
            let is_w_index = index.starts_with('w') || index.starts_with('W');
            let shift_amount: u8 = match shift { Some(s) => *s, None => 0 };
            let (option, s_bit) = match extend.as_deref() {
                Some("lsl") => (0b011u32, if shift_amount > 0 { 1u32 } else { 0 }),
                Some("sxtw") => (0b110u32, if shift_amount > 0 { 1u32 } else { 0 }),
                Some("sxtx") => (0b111u32, if shift_amount > 0 { 1u32 } else { 0 }),
                Some("uxtw") => (0b010u32, if shift_amount > 0 { 1u32 } else { 0 }),
                None => if is_w_index { (0b010u32, 0u32) } else { (0b011u32, 0u32) },
                _ => (0b011u32, 0u32),
            };
            let word = (0b11 << 30) | (0b111 << 27) | (0b10 << 23) | (1 << 21)
                | (rm << 16) | (option << 13) | (s_bit << 12) | (0b10 << 10) | (rn << 5) | prfop;
            Ok(EncodeResult::Word(word))
        }
        _ => Err(format!("prfm: expected memory operand, got {:?}", operands[1])),
    }
}

/// Map prefetch operation name to its 5-bit encoding.
pub(crate) fn encode_prfop(name: &str) -> Result<u32, String> {
    match name.to_lowercase().as_str() {
        "pldl1keep" => Ok(0b00000),
        "pldl1strm" => Ok(0b00001),
        "pldl2keep" => Ok(0b00010),
        "pldl2strm" => Ok(0b00011),
        "pldl3keep" => Ok(0b00100),
        "pldl3strm" => Ok(0b00101),
        "plil1keep" => Ok(0b01000),
        "plil1strm" => Ok(0b01001),
        "plil2keep" => Ok(0b01010),
        "plil2strm" => Ok(0b01011),
        "plil3keep" => Ok(0b01100),
        "plil3strm" => Ok(0b01101),
        "pstl1keep" => Ok(0b10000),
        "pstl1strm" => Ok(0b10001),
        "pstl2keep" => Ok(0b10010),
        "pstl2strm" => Ok(0b10011),
        "pstl3keep" => Ok(0b10100),
        "pstl3strm" => Ok(0b10101),
        _ => Err(format!("prfm: unknown prefetch operation: {}", name)),
    }
}

// ── LSE Atomics ──────────────────────────────────────────────────────────

/// Encode CAS/CASA/CASAL/CASL and byte/halfword variants (Compare and Swap).
/// CAS Xs, Xt, [Xn]: size 001000 1 L 1 Rs o0 11111 Rn Rt
pub(crate) fn encode_cas(mnemonic: &str, operands: &[Operand]) -> Result<EncodeResult, String> {
    if operands.len() < 3 {
        return Err(format!("{} requires 3 operands", mnemonic));
    }
    let (rs, is_64) = get_reg(operands, 0)?;
    let (rt, _) = get_reg(operands, 1)?;
    let rn = match operands.get(2) {
        Some(Operand::Mem { base, .. }) => parse_reg_num(base).ok_or("cas: invalid base")?,
        _ => return Err("cas requires memory operand [Xn]".to_string()),
    };
    let mn = mnemonic.to_lowercase();
    let suffix = mn.strip_prefix("cas").unwrap_or("");
    // Determine size: 'b' suffix = byte (00), 'h' suffix = half (01), else register-based
    let size = if suffix.contains('b') {
        0b00u32
    } else if suffix.contains('h') {
        0b01u32
    } else if is_64 {
        0b11u32
    } else {
        0b10u32
    };
    // L bit (acquire): set for casa, casal
    let l = if suffix.contains('a') { 1u32 } else { 0u32 };
    // o0 bit (release): set for casl, casal
    let o0 = if suffix.contains('l') { 1u32 } else { 0u32 };
    // size 001000 1 L 1 Rs o0 11111 Rn Rt
    let word = (size << 30) | (0b001000 << 24) | (1 << 23) | (l << 22) | (1 << 21)
        | (rs << 16) | (o0 << 15) | (0b11111 << 10) | (rn << 5) | rt;
    Ok(EncodeResult::Word(word))
}

/// Encode SWP/SWPA/SWPAL/SWPL and byte/halfword variants (Swap).
/// SWP Xs, Xt, [Xn]: size 111000 AR 1 Rs 1 000 00 Rn Rt
/// Variants: swp, swpa, swpal, swpl, swpb, swpab, swpalb, swplb, swph, swpah, swpalh, swplh
pub(crate) fn encode_swp(mnemonic: &str, operands: &[Operand]) -> Result<EncodeResult, String> {
    if operands.len() < 3 {
        return Err(format!("{} requires 3 operands", mnemonic));
    }
    let (rs, is_64) = get_reg(operands, 0)?;
    let (rt, _) = get_reg(operands, 1)?;
    let rn = match operands.get(2) {
        Some(Operand::Mem { base, .. }) => parse_reg_num(base).ok_or("swp: invalid base")?,
        _ => return Err("swp requires memory operand [Xn]".to_string()),
    };
    let mn = mnemonic.to_lowercase();
    let suffix = mn.strip_prefix("swp").unwrap_or("");
    // Determine size: 'b' suffix = byte (00), 'h' suffix = half (01), else register-based
    let size = if suffix.contains('b') {
        0b00u32
    } else if suffix.contains('h') {
        0b01u32
    } else if is_64 {
        0b11u32
    } else {
        0b10u32
    };
    let a = if suffix.contains('a') { 1u32 } else { 0u32 };
    let r = if suffix.contains('l') { 1u32 } else { 0u32 };
    // size 111000 A R 1 Rs 1 000 00 Rn Rt
    let word = (size << 30) | (0b111000 << 24) | (a << 23) | (r << 22) | (1 << 21)
        | (rs << 16) | (1 << 15) | (rn << 5) | rt;
    Ok(EncodeResult::Word(word))
}

/// Encode LDADD/LDCLR/LDEOR/LDSET and their acquire/release/byte/halfword variants (LSE atomics).
/// LDADD Rs, Rt, [Xn]: size 111000 A R 1 Rs 0 opc 00 Rn Rt
/// opc: LDADD=000, LDCLR=001, LDEOR=010, LDSET=011
pub(crate) fn encode_ldop(mnemonic: &str, operands: &[Operand]) -> Result<EncodeResult, String> {
    if operands.len() < 3 {
        return Err(format!("{} requires 3 operands", mnemonic));
    }
    let (rs, is_64) = get_reg(operands, 0)?;
    let (rt, _) = get_reg(operands, 1)?;
    let rn = match operands.get(2) {
        Some(Operand::Mem { base, .. }) => parse_reg_num(base).ok_or("ldop: invalid base")?,
        _ => return Err(format!("{} requires memory operand [Xn]", mnemonic)),
    };
    let mn = mnemonic.to_lowercase();
    // Determine base op and suffix
    let (base, suffix) = if let Some(s) = mn.strip_prefix("ldadd") {
        (0b000u32, s)
    } else if let Some(s) = mn.strip_prefix("ldclr") {
        (0b001u32, s)
    } else if let Some(s) = mn.strip_prefix("ldeor") {
        (0b010u32, s)
    } else if let Some(s) = mn.strip_prefix("ldset") {
        (0b011u32, s)
    } else {
        return Err(format!("unknown ld atomic op: {}", mnemonic));
    };
    // Determine size: 'b' suffix = byte (00), 'h' suffix = half (01), else register-based
    let size = if suffix.contains('b') {
        0b00u32
    } else if suffix.contains('h') {
        0b01u32
    } else if is_64 {
        0b11u32
    } else {
        0b10u32
    };
    let a = if suffix.contains('a') { 1u32 } else { 0u32 };
    let r = if suffix.contains('l') { 1u32 } else { 0u32 };
    // size 111000 A R 1 Rs 0 opc 00 Rn Rt
    let word = (size << 30) | (0b111000 << 24) | (a << 23) | (r << 22) | (1 << 21)
        | (rs << 16) | (base << 12) | (rn << 5) | rt;
    Ok(EncodeResult::Word(word))
}

/// Encode STADD/STCLR/STEOR/STSET and their release/byte/halfword variants.
/// These are aliases for LDADD/LDCLR/LDEOR/LDSET with Rt=XZR (register 31).
/// STADD Ws, [Xn] encodes as LDADD Ws, WZR, [Xn]
/// Variants: stadd/stclr/steor/stset, plus 'l' (release), 'b' (byte), 'h' (half).
pub(crate) fn encode_stop(mnemonic: &str, operands: &[Operand]) -> Result<EncodeResult, String> {
    if operands.len() < 2 {
        return Err(format!("{} requires 2 operands", mnemonic));
    }
    let (rs, is_64) = get_reg(operands, 0)?;
    let rn = match operands.get(1) {
        Some(Operand::Mem { base, .. }) => parse_reg_num(base).ok_or_else(|| format!("{}: invalid base", mnemonic))?,
        _ => return Err(format!("{} requires memory operand [Xn]", mnemonic)),
    };
    let mn = mnemonic.to_lowercase();
    // Determine base op from the prefix
    let (opc, suffix) = if let Some(s) = mn.strip_prefix("stadd") {
        (0b000u32, s)
    } else if let Some(s) = mn.strip_prefix("stclr") {
        (0b001u32, s)
    } else if let Some(s) = mn.strip_prefix("steor") {
        (0b010u32, s)
    } else if let Some(s) = mn.strip_prefix("stset") {
        (0b011u32, s)
    } else {
        return Err(format!("unknown st atomic op: {}", mnemonic));
    };
    // Determine size: 'b' suffix = byte (00), 'h' suffix = half (01), else register-based
    let size = if suffix.contains('b') {
        0b00u32
    } else if suffix.contains('h') {
        0b01u32
    } else if is_64 {
        0b11u32
    } else {
        0b10u32
    };
    // A=0 (no acquire for store aliases), R from 'l' suffix (release)
    let r = if suffix.contains('l') { 1u32 } else { 0u32 };
    let rt = 31u32; // XZR/WZR - discard result
    // size 111000 A R 1 Rs 0 opc 00 Rn Rt
    let word = (size << 30) | (0b111000 << 24) | (r << 22) | (1 << 21)
        | (rs << 16) | (opc << 12) | (rn << 5) | rt;
    Ok(EncodeResult::Word(word))
}

#[cfg(test)]
mod encode_adr_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler
    // Evidence: src/backend/arm/assembler/README.md "accepts the same textual assembly that GCC's gas would consume";
    //   encoder/mod.rs:1-7 32-bit AArch64 words; encoder/mod.rs:373 adr dispatch;
    //   ARM ARM ADR (PC-relative): op=0 immlo 10000 immhi Rd, 21-bit signed offset.
    // Stronger considered:
    //   - State machine: rejected — encode_adr is a pure function with no lifecycle
    //   - Algebraic round-trip via in-tree decoder: rejected — no ADR decoder
    //   - Linker reloc::encode_adr as differential sibling: rejected — different job (patches imm)
    // Weaker available: algebraic.round_trip (ARM field unpack), algebraic.metamorphic (Rd/imm independence),
    //   algebraic.invariant (AdrPrelLo21 reloc), negative_error (W/SP/range/arity/FP/modifier)
    // Differential: candidate=encode_adr, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=(Reg(Xd), Imm(imm)) <-> asm text `adr Xd, #imm`

    use super::encode_adr;
    use super::super::{EncodeResult, RelocType};
    use crate::backend::arm::assembler::parser::Operand;
    use proptest::prelude::*;
    use std::io::Write;
    use std::process::{Command, Stdio};

    const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
    const CASES: u32 = 1000;
    const IMM_MIN: i64 = -1_048_576; // -2^20
    const IMM_MAX: i64 = 1_048_575; // 2^20 - 1

    fn cfg() -> ProptestConfig {
        ProptestConfig::with_cases(CASES)
    }

    fn xreg(n: u32) -> String {
        if n == 31 {
            "xzr".into()
        } else {
            format!("x{}", n)
        }
    }

    fn wreg(n: u32) -> String {
        if n == 31 {
            "wzr".into()
        } else {
            format!("w{}", n)
        }
    }

    fn sut_word(ops: &[Operand]) -> Result<u32, String> {
        match encode_adr(ops)? {
            EncodeResult::Word(w) => Ok(w),
            other => Err(format!("expected Word, got {:?}", other)),
        }
    }

    /// Unpack ADR fields per ARM ARM (not a copy of the SUT packer).
    fn unpack_adr(word: u32) -> (u32 /*rd*/, i64 /*imm21*/, u32 /*op*/, u32 /*opc*/ ) {
        let rd = word & 0x1f;
        let immlo = (word >> 29) & 0x3;
        let immhi = (word >> 5) & 0x7ffff;
        let imm21 = (immhi << 2) | immlo;
        let imm = if (imm21 & (1 << 20)) != 0 {
            (imm21 as i64) | !0x1f_ffffi64
        } else {
            imm21 as i64
        };
        let op = (word >> 31) & 1;
        let opc = (word >> 24) & 0x1f;
        (rd, imm, op, opc)
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

    fn imm_in_range() -> impl Strategy<Value = i64> {
        prop_oneof![
            Just(IMM_MIN),
            Just(IMM_MIN + 1),
            Just(-1i64),
            Just(0i64),
            Just(1i64),
            Just(3i64),
            Just(4i64),
            Just(IMM_MAX - 1),
            Just(IMM_MAX),
            IMM_MIN..=IMM_MAX,
        ]
    }

    fn imm_out_of_range() -> impl Strategy<Value = i64> {
        prop_oneof![
            Just(IMM_MIN - 1),
            Just(IMM_MAX + 1),
            Just(i64::MIN),
            Just(i64::MAX),
            Just(1i64 << 21),
            Just(1i64 << 40),
            Just(-(1i64 << 21)),
            (i64::MIN..=IMM_MIN - 1),
            (IMM_MAX + 1..=i64::MAX),
        ]
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_adr_kat_llvm_mc_adr_x0_imm0() {
        let want = 0x10000000u32;
        let mc = llvm_mc_word("adr x0, #0").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [Operand::Reg("x0".into()), Operand::Imm(0)];
        let sut = sut_word(&ops).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_adr_kat_llvm_mc_adr_x0_imm1() {
        let want = 0x30000000u32;
        let mc = llvm_mc_word("adr x0, #1").expect("llvm-mc KAT #1");
        assert_eq!(mc, want, "llvm-mc KAT #1 mapping broken");
        let ops = [Operand::Reg("x0".into()), Operand::Imm(1)];
        let sut = sut_word(&ops).expect("SUT KAT #1");
        assert_eq!(sut, want);
    }

    proptest! {
        #![proptest_config(cfg())]

        // Oracle: differential
        // Target: encoder.load_store.encode_adr
        #[test]
        fn encode_adr_diff_imm_llvm_mc(
            rd in 0u32..=31,
            imm in imm_in_range(),
        ) {
            let rd_n = xreg(rd);
            let asm = format!("adr {}, #{}", rd_n, imm);
            let ops = [Operand::Reg(rd_n.clone()), Operand::Imm(imm)];
            let sut = sut_word(&ops)
                .unwrap_or_else(|e| panic!("SUT rejected valid ADR {}: {}", asm, e));
            let mc = llvm_mc_word(&asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid ADR {}: {}", asm, e));
            prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
        }

        // Oracle: algebraic.round_trip
        // Target: encoder.load_store.encode_adr
        #[test]
        fn encode_adr_roundtrip_arm_fields(
            rd in 0u32..=31,
            imm in imm_in_range(),
        ) {
            let ops = [Operand::Reg(xreg(rd)), Operand::Imm(imm)];
            let word = sut_word(&ops)
                .unwrap_or_else(|e| panic!("SUT rejected in-range ADR: {}", e));
            let (got_rd, got_imm, op, opc) = unpack_adr(word);
            prop_assert_eq!(op, 0, "ADR op bit 31 must be 0 (not ADRP), word={:#010x}", word);
            prop_assert_eq!(opc, 0b10000, "ADR bits [28:24] must be 10000, word={:#010x}", word);
            prop_assert_eq!(got_rd, rd, "Rd field mismatch word={:#010x}", word);
            prop_assert_eq!(got_imm, imm, "unpacked 21-bit imm mismatch word={:#010x}", word);
        }

        // Oracle: algebraic.metamorphic
        // Target: encoder.load_store.encode_adr
        #[test]
        fn encode_adr_metamorphic_rd_imm_independent(
            rd in 0u32..=31,
            rd2 in 0u32..=31,
            imm in imm_in_range(),
            imm2 in imm_in_range(),
        ) {
            let w_rd_imm = sut_word(&[Operand::Reg(xreg(rd)), Operand::Imm(imm)])
                .unwrap_or_else(|e| panic!("encode rd,imm: {}", e));
            let w_rd_imm2 = sut_word(&[Operand::Reg(xreg(rd)), Operand::Imm(imm2)])
                .unwrap_or_else(|e| panic!("encode rd,imm2: {}", e));
            let w_rd2_imm = sut_word(&[Operand::Reg(xreg(rd2)), Operand::Imm(imm)])
                .unwrap_or_else(|e| panic!("encode rd2,imm: {}", e));
            prop_assert_eq!(
                (w_rd_imm ^ w_rd_imm2) & 0x1f,
                0,
                "changing imm must not change Rd (w1={:#010x} w2={:#010x})",
                w_rd_imm,
                w_rd_imm2
            );
            prop_assert_eq!(
                (w_rd_imm ^ w_rd2_imm) & !0x1fu32,
                0,
                "changing Rd must not change opcode/imm fields (w1={:#010x} w2={:#010x})",
                w_rd_imm,
                w_rd2_imm
            );
        }

        // Oracle: algebraic.invariant
        // Target: encoder.load_store.encode_adr
        #[test]
        fn encode_adr_symbol_reloc(
            rd in 0u32..=31,
            suffix in 0u32..=1000,
            addend in prop_oneof![
                Just(0i64),
                Just(-1i64),
                Just(8i64),
                Just(-8i64),
                -4096i64..=4096i64,
            ],
        ) {
            let sym = format!("labl{}", suffix);
            let rd_n = xreg(rd);
            let expected_word = 0x10000000u32 | rd;

            let check = |ops: &[Operand], expect_addend: i64, tag: &str| {
                match encode_adr(ops) {
                    Ok(EncodeResult::WordWithReloc { word, reloc }) => {
                        prop_assert_eq!(word, expected_word, "{} word", tag);
                        match reloc.reloc_type {
                            RelocType::AdrPrelLo21 => {}
                            other => {
                                return Err(TestCaseError::fail(format!(
                                    "{} expected AdrPrelLo21, got {:?}",
                                    tag, other
                                )));
                            }
                        }
                        prop_assert_eq!(&reloc.symbol, &sym, "{} symbol", tag);
                        prop_assert_eq!(reloc.addend, expect_addend, "{} addend", tag);
                        let (got_rd, got_imm, op, opc) = unpack_adr(word);
                        prop_assert_eq!(op, 0, "{} op bit", tag);
                        prop_assert_eq!(opc, 0b10000, "{} opc", tag);
                        prop_assert_eq!(got_rd, rd, "{} rd", tag);
                        prop_assert_eq!(got_imm, 0, "{} reloc imm fields must be 0", tag);
                        Ok(())
                    }
                    other => Err(TestCaseError::fail(format!(
                        "{} expected WordWithReloc, got {:?}",
                        tag, other
                    ))),
                }
            };

            check(
                &[Operand::Reg(rd_n.clone()), Operand::Symbol(sym.clone())],
                0,
                "Symbol",
            )?;
            check(
                &[Operand::Reg(rd_n.clone()), Operand::Label(sym.clone())],
                0,
                "Label",
            )?;
            check(
                &[
                    Operand::Reg(rd_n),
                    Operand::SymbolOffset(sym.clone(), addend),
                ],
                addend,
                "SymbolOffset",
            )?;
        }

        // Oracle: negative_error
        // Target: encoder.load_store.encode_adr
        #[test]
        fn encode_adr_neg_w_reg(
            wd in 0u32..=31,
            imm in imm_in_range(),
            use_wsp in any::<bool>(),
        ) {
            let name = if use_wsp { "wsp".to_string() } else { wreg(wd) };
            let ops = [Operand::Reg(name.clone()), Operand::Imm(imm)];
            prop_assert!(
                encode_adr(&ops).is_err(),
                "ADR takes Xd only; {} must Err (imm={})",
                name,
                imm
            );
        }

        // Oracle: negative_error
        // Target: encoder.load_store.encode_adr
        #[test]
        fn encode_adr_neg_sp(imm in imm_in_range()) {
            let ops = [Operand::Reg("sp".into()), Operand::Imm(imm)];
            prop_assert!(
                encode_adr(&ops).is_err(),
                "ADR Rd is Xd (X31=XZR, not SP); adr sp, #{} must Err",
                imm
            );
        }

        // Oracle: negative_error
        // Target: encoder.load_store.encode_adr
        #[test]
        fn encode_adr_neg_imm_range(
            rd in 0u32..=31,
            imm in imm_out_of_range(),
        ) {
            let ops = [Operand::Reg(xreg(rd)), Operand::Imm(imm)];
            prop_assert!(
                encode_adr(&ops).is_err(),
                "21-bit signed ADR offset {} is out of range [-1048576, 1048575] and must Err",
                imm
            );
        }

        // Oracle: negative_error
        // Target: encoder.load_store.encode_adr
        #[test]
        fn encode_adr_neg_bad_operands(
            kind in 0u32..=4,
            rd in 0u32..=30,
            imm in imm_in_range(),
        ) {
            let x = xreg(rd);
            let ops: Vec<Operand> = match kind {
                0 => vec![],
                1 => vec![Operand::Imm(imm)],
                2 => vec![Operand::Reg(x.clone())],
                3 => vec![Operand::Reg(x.clone()), Operand::Mem { base: "x1".into(), offset: 0 }],
                _ => vec![Operand::Reg("x32".into()), Operand::Imm(imm)],
            };
            prop_assert!(
                encode_adr(&ops).is_err(),
                "invalid ADR arity/kind={} must Err, got {:?}",
                kind,
                encode_adr(&ops)
            );
        }

        // Oracle: negative_error
        // Target: encoder.load_store.encode_adr
        #[test]
        fn encode_adr_neg_fp_reg(
            prefix in prop::sample::select(vec!["d", "s", "q", "v", "h", "b"]),
            n in 0u32..=31,
            imm in imm_in_range(),
        ) {
            let fp = format!("{}{}", prefix, n);
            let ops = [Operand::Reg(fp.clone()), Operand::Imm(imm)];
            prop_assert!(
                encode_adr(&ops).is_err(),
                "ADR takes Xd only; FP/SIMD {} must Err (imm={})",
                fp,
                imm
            );
        }

        // Oracle: negative_error
        // Target: encoder.load_store.encode_adr
        #[test]
        fn encode_adr_neg_modifier(
            rd in 0u32..=31,
            mod_kind in prop::sample::select(vec!["lo12", "got", "got_lo12"]),
            suffix in 0u32..=1000,
        ) {
            let sym = format!("labl{}", suffix);
            let ops = [
                Operand::Reg(xreg(rd)),
                Operand::Modifier {
                    kind: mod_kind.to_string(),
                    symbol: sym,
                },
            ];
            prop_assert!(
                encode_adr(&ops).is_err(),
                "ADR does not take :{}: modifiers (llvm-mc: unexpected adr label)",
                mod_kind
            );
        }

        // Oracle: algebraic.invariant (coverage sweep: get_symbol parser-misclassification arms)
        // Target: encoder.load_store.encode_adr
        #[test]
        fn encode_adr_symbol_misclassified(
            rd in 0u32..=31,
            which in 0u32..=2,
            name in prop::sample::select(vec!["s1", "v0", "d1", "cc", "lt", "le", "st", "ld"]),
        ) {
            let second = match which {
                0 => Operand::Reg(name.to_string()),
                1 => Operand::Cond(name.to_string()),
                _ => Operand::Barrier(name.to_string()),
            };
            let ops = [Operand::Reg(xreg(rd)), second];
            match encode_adr(&ops) {
                Ok(EncodeResult::WordWithReloc { word, reloc }) => {
                    prop_assert_eq!(word, 0x10000000u32 | rd);
                    match reloc.reloc_type {
                        RelocType::AdrPrelLo21 => {}
                        other => {
                            return Err(TestCaseError::fail(format!(
                                "expected AdrPrelLo21, got {:?}",
                                other
                            )));
                        }
                    }
                    prop_assert_eq!(&reloc.symbol, name);
                    prop_assert_eq!(reloc.addend, 0);
                }
                other => {
                    return Err(TestCaseError::fail(format!(
                        "parser-misclassified {} as operand 1 must be a symbol reloc, got {:?}",
                        name, other
                    )));
                }
            }
        }

        // Oracle: negative_error (coverage sweep: ModifierOffset)
        // Target: encoder.load_store.encode_adr
        #[test]
        fn encode_adr_neg_modifier_offset(
            rd in 0u32..=31,
            mod_kind in prop::sample::select(vec!["lo12", "got", "got_lo12"]),
            offset in prop_oneof![Just(0i64), Just(8i64), Just(-8i64), -4096i64..=4096],
        ) {
            let ops = [
                Operand::Reg(xreg(rd)),
                Operand::ModifierOffset {
                    kind: mod_kind.to_string(),
                    symbol: "foo".into(),
                    offset,
                },
            ];
            prop_assert!(
                encode_adr(&ops).is_err(),
                "ADR does not take :{}:symbol+offset modifiers",
                mod_kind
            );
        }
    }

    #[test]
    fn test_encode_adr_regression_w_reg() {
        let ops = [Operand::Reg("w0".into()), Operand::Imm(-1_048_576)];
        assert!(
            encode_adr(&ops).is_err(),
            "adr w0, #-1048576 must Err; ADR takes Xd only (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_adr_regression_sp() {
        let ops = [Operand::Reg("sp".into()), Operand::Imm(-1_048_576)];
        assert!(
            encode_adr(&ops).is_err(),
            "adr sp, #-1048576 must Err; ADR Rd is Xd (X31=XZR, not SP)"
        );
    }

    #[test]
    fn test_encode_adr_regression_imm_range() {
        let ops = [Operand::Reg("x0".into()), Operand::Imm(-1_048_577)];
        assert!(
            encode_adr(&ops).is_err(),
            "adr x0, #-1048577 must Err; 21-bit signed offset is out of range"
        );
    }

    #[test]
    fn test_encode_adr_regression_fp_reg() {
        let ops = [Operand::Reg("d0".into()), Operand::Imm(0)];
        assert!(
            encode_adr(&ops).is_err(),
            "adr d0, #0 must Err; FP/SIMD registers are not ADR operands (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_adr_regression_modifier() {
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Modifier {
                kind: "lo12".into(),
                symbol: "foo".into(),
            },
        ];
        assert!(
            encode_adr(&ops).is_err(),
            "adr x0, :lo12:foo must Err; llvm-mc reports unexpected adr label"
        );
    }

    #[test]
    fn test_encode_adr_regression_modifier_offset() {
        let ops = [
            Operand::Reg("x0".into()),
            Operand::ModifierOffset {
                kind: "lo12".into(),
                symbol: "foo".into(),
                offset: 0,
            },
        ];
        assert!(
            encode_adr(&ops).is_err(),
            "adr x0, :lo12:foo+0 must Err; ADR does not take :lo12: modifiers"
        );
    }
}

#[cfg(test)]
mod encode_ldar_stlr_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler
    // Evidence: src/backend/arm/assembler/README.md "accepts the same textual assembly that GCC's gas would consume";
    //   encoder/mod.rs:1-7 32-bit AArch64 words; encoder/mod.rs:360-365 ldar/stlr/ldarb/stlrb/ldarh/stlrh dispatch;
    //   ARM ARM LDAR/STLR: size 001000 1 L 0 11111 1 11111 Rn Rt; Rt is Wt/Xt (31=ZR); Rn is Xn|SP; offset {,#0}.
    // Stronger considered:
    //   - State machine: rejected — encode_ldar_stlr is a pure function with no lifecycle
    //   - Algebraic round-trip via in-tree decoder: rejected — no LDAR/STLR decoder
    //   - encode_ldaxr_stlxr / encode_ldxr_stxr as differential siblings: rejected — exclusive forms, different job
    // Weaker available: algebraic.round_trip (ARM field unpack), algebraic.metamorphic (L bit),
    //   algebraic.invariant (fixed opcode bits), negative_error (arity / extra / SP / FP / W-base / offset)
    // Differential: candidate=encode_ldar_stlr, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=(Reg(Rt), Mem{Rn, 0}, is_load, forced_size) <-> `{ldar|stlr|ldarb|stlrb|ldarh|stlrh} Rt, [Rn]`

    use super::encode_ldar_stlr;
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

    fn rt_name(n: u32, is_64: bool) -> String {
        if is_64 {
            if n == 31 {
                "xzr".into()
            } else {
                format!("x{}", n)
            }
        } else if n == 31 {
            "wzr".into()
        } else {
            format!("w{}", n)
        }
    }

    fn rn_name(n: u32) -> String {
        if n == 31 {
            "sp".into()
        } else {
            format!("x{}", n)
        }
    }

    fn forced_size(variant: u32) -> Option<u32> {
        match variant {
            0 => None,
            1 => Some(0b00),
            _ => Some(0b01),
        }
    }

    fn mnemonic(is_load: bool, variant: u32) -> &'static str {
        match (is_load, variant) {
            (true, 0) => "ldar",
            (false, 0) => "stlr",
            (true, 1) => "ldarb",
            (false, 1) => "stlrb",
            (true, _) => "ldarh",
            (false, _) => "stlrh",
        }
    }

    fn expected_size(variant: u32, data_is_64: bool) -> u32 {
        match variant {
            1 => 0b00,
            2 => 0b01,
            _ if data_is_64 => 0b11,
            _ => 0b10,
        }
    }

    fn data_is_64(variant: u32, is_64: bool) -> bool {
        variant == 0 && is_64
    }

    fn valid_ops(rt: u32, rn: u32, variant: u32, is_64: bool) -> Vec<Operand> {
        let wide = data_is_64(variant, is_64);
        vec![
            Operand::Reg(rt_name(rt, wide)),
            Operand::Mem {
                base: rn_name(rn),
                offset: 0,
            },
        ]
    }

    fn sut_word(ops: &[Operand], is_load: bool, variant: u32) -> Result<u32, String> {
        match encode_ldar_stlr(ops, is_load, forced_size(variant))? {
            EncodeResult::Word(w) => Ok(w),
            other => Err(format!("expected Word, got {:?}", other)),
        }
    }

    /// Unpack LDAR/STLR fields per ARM ARM (not a copy of the SUT packer).
    fn unpack_ldar_stlr(word: u32) -> (u32, u32, u32, u32, u32, u32, u32) {
        let size = (word >> 30) & 0b11;
        let l = (word >> 22) & 1;
        let rs = (word >> 16) & 0x1f;
        let o0 = (word >> 15) & 1;
        let rt2 = (word >> 10) & 0x1f;
        let rn = (word >> 5) & 0x1f;
        let rt = word & 0x1f;
        (size, l, rs, o0, rt2, rn, rt)
    }

    fn fixed_ldar_stlr_bits(word: u32) -> bool {
        ((word >> 24) & 0x3f) == 0b001000
            && ((word >> 23) & 1) == 1
            && ((word >> 21) & 1) == 0
            && ((word >> 16) & 0x1f) == 31
            && ((word >> 15) & 1) == 1
            && ((word >> 10) & 0x1f) == 31
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

    fn reg_edge() -> impl Strategy<Value = u32> {
        prop_oneof![Just(0u32), Just(1u32), Just(30u32), Just(31u32), 0u32..=31]
    }

    fn variant_strat() -> impl Strategy<Value = u32> {
        prop_oneof![Just(0u32), Just(1u32), Just(2u32)]
    }

    fn extra_operand() -> impl Strategy<Value = Operand> {
        prop_oneof![
            Just(Operand::Reg("x2".into())),
            Just(Operand::Imm(0)),
            Just(Operand::Imm(1)),
            Just(Operand::Symbol("foo".into())),
            Just(Operand::Mem {
                base: "x3".into(),
                offset: 0
            }),
        ]
    }

    fn nonzero_offset() -> impl Strategy<Value = i64> {
        prop_oneof![
            Just(-1i64),
            Just(1i64),
            Just(8i64),
            Just(-8i64),
            Just(256i64),
            Just(i64::MIN),
            Just(i64::MAX),
            1i64..=4096,
            -4096i64..=-1,
        ]
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_ldar_stlr_kat_llvm_mc_ldar_x0_x1() {
        let want = 0xc8dffc20u32;
        let mc = llvm_mc_word("ldar x0, [x1]").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = valid_ops(0, 1, 0, true);
        let sut = sut_word(&ops, true, 0).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_ldar_stlr_kat_llvm_mc_stlr_x0_x1() {
        let want = 0xc89ffc20u32;
        let mc = llvm_mc_word("stlr x0, [x1]").expect("llvm-mc KAT stlr");
        assert_eq!(mc, want, "llvm-mc KAT stlr mapping broken");
        let ops = valid_ops(0, 1, 0, true);
        let sut = sut_word(&ops, false, 0).expect("SUT KAT stlr");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_ldar_stlr_kat_llvm_mc_ldar_w0_x1() {
        let want = 0x88dffc20u32;
        let mc = llvm_mc_word("ldar w0, [x1]").expect("llvm-mc KAT w");
        assert_eq!(mc, want, "llvm-mc KAT w mapping broken");
        let ops = valid_ops(0, 1, 0, false);
        let sut = sut_word(&ops, true, 0).expect("SUT KAT w");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_ldar_stlr_kat_llvm_mc_ldarb_w0_x1() {
        let want = 0x08dffc20u32;
        let mc = llvm_mc_word("ldarb w0, [x1]").expect("llvm-mc KAT b");
        assert_eq!(mc, want, "llvm-mc KAT ldarb mapping broken");
        let ops = valid_ops(0, 1, 1, false);
        let sut = sut_word(&ops, true, 1).expect("SUT KAT b");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_ldar_stlr_kat_llvm_mc_ldarh_w0_x1() {
        let want = 0x48dffc20u32;
        let mc = llvm_mc_word("ldarh w0, [x1]").expect("llvm-mc KAT h");
        assert_eq!(mc, want, "llvm-mc KAT ldarh mapping broken");
        let ops = valid_ops(0, 1, 2, false);
        let sut = sut_word(&ops, true, 2).expect("SUT KAT h");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_ldar_stlr_kat_llvm_mc_ldar_xzr_sp() {
        let want = 0xc8dfffffu32;
        let mc = llvm_mc_word("ldar xzr, [sp]").expect("llvm-mc KAT xzr/sp");
        assert_eq!(mc, want, "llvm-mc KAT xzr/sp mapping broken");
        let ops = valid_ops(31, 31, 0, true);
        let sut = sut_word(&ops, true, 0).expect("SUT KAT xzr/sp");
        assert_eq!(sut, want);
    }

    #[test]
    fn test_encode_ldar_stlr_regression_extra_operand() {
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Mem {
                base: "x0".into(),
                offset: 0,
            },
            Operand::Reg("x2".into()),
        ];
        assert!(
            encode_ldar_stlr(&ops, false, None).is_err(),
            "stlr w0, [x0], x2 must Err; llvm-mc rejects a 3rd operand"
        );
    }

    #[test]
    fn test_encode_ldar_stlr_regression_sp_as_rt() {
        let ops = [
            Operand::Reg("sp".into()),
            Operand::Mem {
                base: "x0".into(),
                offset: 0,
            },
        ];
        assert!(
            encode_ldar_stlr(&ops, false, None).is_err(),
            "stlr sp, [x0] must Err; llvm-mc rejects SP as Rt"
        );
    }

    #[test]
    fn test_encode_ldar_stlr_regression_w_base() {
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Mem {
                base: "w0".into(),
                offset: 0,
            },
        ];
        assert!(
            encode_ldar_stlr(&ops, false, None).is_err(),
            "stlr w0, [w0] must Err; llvm-mc rejects a W register as base"
        );
    }

    proptest! {
        #![proptest_config(cfg())]

        // Oracle: differential
        // Target: encoder.load_store.encode_ldar_stlr
        #[test]
        fn encode_ldar_stlr_diff_llvm_mc(
            rt in reg_edge(),
            rn in reg_edge(),
            is_load in any::<bool>(),
            variant in variant_strat(),
            is_64 in any::<bool>(),
        ) {
            let wide = data_is_64(variant, is_64);
            let rt_n = rt_name(rt, wide);
            let rn_n = rn_name(rn);
            let mnem = mnemonic(is_load, variant);
            let asm = format!("{} {}, [{}]", mnem, rt_n, rn_n);
            let ops = valid_ops(rt, rn, variant, is_64);
            let sut = sut_word(&ops, is_load, variant)
                .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
            let mc = llvm_mc_word(&asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
            prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
        }

        // Oracle: algebraic.round_trip
        // Target: encoder.load_store.encode_ldar_stlr
        #[test]
        fn encode_ldar_stlr_roundtrip_arm_fields(
            rt in reg_edge(),
            rn in reg_edge(),
            is_load in any::<bool>(),
            variant in variant_strat(),
            is_64 in any::<bool>(),
        ) {
            let ops = valid_ops(rt, rn, variant, is_64);
            let word = sut_word(&ops, is_load, variant)
                .unwrap_or_else(|e| panic!("SUT rejected valid LDAR/STLR: {}", e));
            let (size, l, rs, o0, rt2, got_rn, got_rt) = unpack_ldar_stlr(word);
            let want_size = expected_size(variant, data_is_64(variant, is_64));
            prop_assert_eq!(size, want_size, "size field mismatch word={:#010x}", word);
            prop_assert_eq!(l, if is_load { 1 } else { 0 }, "L bit mismatch word={:#010x}", word);
            prop_assert_eq!(rs, 31, "Rs must be 11111 word={:#010x}", word);
            prop_assert_eq!(o0, 1, "o0 must be 1 (acquire/release) word={:#010x}", word);
            prop_assert_eq!(rt2, 31, "Rt2 must be 11111 word={:#010x}", word);
            prop_assert_eq!(got_rn, rn, "Rn field mismatch word={:#010x}", word);
            prop_assert_eq!(got_rt, rt, "Rt field mismatch word={:#010x}", word);
            prop_assert!(
                ((word >> 24) & 0x3f) == 0b001000,
                "bits [29:24] must be 001000 word={:#010x}",
                word
            );
            prop_assert!(((word >> 23) & 1) == 1, "bit 23 must be 1 word={:#010x}", word);
            prop_assert!(((word >> 21) & 1) == 0, "bit 21 must be 0 word={:#010x}", word);
        }

        // Oracle: algebraic.metamorphic
        // Target: encoder.load_store.encode_ldar_stlr
        #[test]
        fn encode_ldar_stlr_metamorphic_l_bit(
            rt in reg_edge(),
            rn in reg_edge(),
            variant in variant_strat(),
            is_64 in any::<bool>(),
        ) {
            let ops = valid_ops(rt, rn, variant, is_64);
            let load = sut_word(&ops, true, variant)
                .unwrap_or_else(|e| panic!("SUT rejected load: {}", e));
            let store = sut_word(&ops, false, variant)
                .unwrap_or_else(|e| panic!("SUT rejected store: {}", e));
            prop_assert_eq!(
                load ^ store,
                1u32 << 22,
                "LDAR vs STLR must differ only by L at bit 22 load={:#010x} store={:#010x}",
                load,
                store
            );
        }

        // Oracle: algebraic.invariant
        // Target: encoder.load_store.encode_ldar_stlr
        #[test]
        fn encode_ldar_stlr_invariant_fixed_bits(
            rt in reg_edge(),
            rn in reg_edge(),
            is_load in any::<bool>(),
            variant in variant_strat(),
            is_64 in any::<bool>(),
        ) {
            let ops = valid_ops(rt, rn, variant, is_64);
            let word = sut_word(&ops, is_load, variant)
                .unwrap_or_else(|e| panic!("SUT rejected valid LDAR/STLR: {}", e));
            prop_assert!(
                fixed_ldar_stlr_bits(word),
                "fixed LDAR/STLR bits violated word={:#010x}",
                word
            );
        }

        // Oracle: negative_error
        // Target: encoder.load_store.encode_ldar_stlr
        // Invalid domain: fewer than 2 operands, or second operand not Mem.
        #[test]
        fn encode_ldar_stlr_neg_arity_and_shape(
            is_load in any::<bool>(),
            variant in variant_strat(),
            shape in 0u32..=8,
            rt in reg_edge(),
            off in -8i64..=8,
        ) {
            let sz = forced_size(variant);
            let rt_n = rt_name(rt, data_is_64(variant, true));
            let ops: Vec<Operand> = match shape {
                0 => vec![],
                1 => vec![Operand::Reg(rt_n)],
                2 => vec![Operand::Reg(rt_n.clone()), Operand::Imm(0)],
                3 => vec![Operand::Reg(rt_n.clone()), Operand::Symbol("foo".into())],
                4 => vec![
                    Operand::Reg(rt_n.clone()),
                    Operand::MemPreIndex {
                        base: "x0".into(),
                        offset: off,
                    },
                ],
                5 => vec![
                    Operand::Reg(rt_n.clone()),
                    Operand::MemPostIndex {
                        base: "x0".into(),
                        offset: off,
                    },
                ],
                6 => vec![
                    Operand::Reg(rt_n),
                    Operand::MemRegOffset {
                        base: "x0".into(),
                        index: "x1".into(),
                        extend: None,
                        shift: None,
                    },
                ],
                7 => vec![
                    Operand::Imm(0),
                    Operand::Mem {
                        base: "x0".into(),
                        offset: 0,
                    },
                ],
                _ => vec![
                    Operand::Reg(rt_n),
                    Operand::Mem {
                        base: if off >= 0 { "foo".into() } else { "x32".into() },
                        offset: 0,
                    },
                ],
            };
            prop_assert!(
                encode_ldar_stlr(&ops, is_load, sz).is_err(),
                "expected Err for arity/shape {} got {:?}",
                shape,
                encode_ldar_stlr(&ops, is_load, sz)
            );
        }

        // Oracle: negative_error
        // Target: encoder.load_store.encode_ldar_stlr
        // Invalid domain: a third operand.
        #[test]
        fn encode_ldar_stlr_neg_extra_operands(
            rt in reg_edge(),
            rn in reg_edge(),
            is_load in any::<bool>(),
            variant in variant_strat(),
            is_64 in any::<bool>(),
            extra in extra_operand(),
        ) {
            let mut ops = valid_ops(rt, rn, variant, is_64);
            ops.push(extra);
            prop_assert!(
                encode_ldar_stlr(&ops, is_load, forced_size(variant)).is_err(),
                "extra operand must Err; llvm-mc rejects a 3rd operand"
            );
        }

        // Oracle: negative_error
        // Target: encoder.load_store.encode_ldar_stlr
        // Invalid domain: SP/WSP/FP as Rt, or Xt for byte/halfword forms.
        #[test]
        fn encode_ldar_stlr_neg_invalid_rt(
            is_load in any::<bool>(),
            kind in 0u32..=3,
            n in 0u32..=31,
        ) {
            let (bad_rt, variant): (String, u32) = match kind {
                0 => ("sp".into(), 0),
                1 => ("wsp".into(), 0),
                2 => {
                    let prefixes = ["d", "s", "q", "v", "h", "b"];
                    (format!("{}{}", prefixes[(n as usize) % 6], n % 32), 0)
                }
                _ => (rt_name(n, true), if n % 2 == 0 { 1 } else { 2 }),
            };
            let ops = [
                Operand::Reg(bad_rt.clone()),
                Operand::Mem {
                    base: "x0".into(),
                    offset: 0,
                },
            ];
            prop_assert!(
                encode_ldar_stlr(&ops, is_load, forced_size(variant)).is_err(),
                "invalid Rt {} for variant {} must Err",
                bad_rt,
                variant
            );
        }

        // Oracle: negative_error
        // Target: encoder.load_store.encode_ldar_stlr
        // Invalid domain: W/WSP/XZR/WZR as base, or nonzero offset.
        #[test]
        fn encode_ldar_stlr_neg_invalid_base_offset(
            rt in reg_edge(),
            is_load in any::<bool>(),
            variant in variant_strat(),
            is_64 in any::<bool>(),
            kind in 0u32..=4,
            wn in 0u32..=30,
            off in nonzero_offset(),
        ) {
            let wide = data_is_64(variant, is_64);
            let (base, offset) = match kind {
                0 => (format!("w{}", wn), 0i64),
                1 => ("wsp".into(), 0),
                2 => ("wzr".into(), 0),
                3 => ("xzr".into(), 0),
                _ => (rn_name(wn), off),
            };
            let ops = [
                Operand::Reg(rt_name(rt, wide)),
                Operand::Mem { base: base.clone(), offset },
            ];
            prop_assert!(
                encode_ldar_stlr(&ops, is_load, forced_size(variant)).is_err(),
                "invalid base/offset [{}], #{} must Err",
                base,
                offset
            );
        }
    }
}

#[cfg(test)]
mod encode_ldur_stur_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler
    // Evidence: src/backend/arm/assembler/README.md "accepts the same textual assembly that GCC's gas would consume";
    //   encoder/mod.rs:1-7 32-bit AArch64 words; encoder/mod.rs:336-339 ldur/stur/ldtr/sttr dispatch;
    //   ARM ARM LDUR/STUR: size 111 V 00 opc 0 imm9 00 Rn Rt; LDTR/STTR: bits [11:10]=10;
    //   Rt is Wt/Xt (31=ZR) or Bt/Ht/St/Dt/Qt; Rn is Xn|SP; simm9 in [-256, 255].
    // Stronger considered:
    //   - State machine: rejected — encode_ldur_stur is a pure function with no lifecycle
    //   - Algebraic round-trip via in-tree decoder: rejected — no LDUR decoder
    //   - encode_ldr_str / encode_ldtr_sized as differential siblings: rejected — different job
    // Weaker available: algebraic.invariant (ARM field unpack), algebraic.metamorphic (opc / op2),
    //   negative_error (extra / range / SP / W-base / XZR-base / SIMD-LDTR / V-Rt)
    // Differential: candidate=encode_ldur_stur, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=(Reg(Rt), Mem{Rn, imm9}, is_load, op2) <-> `{ldur|stur|ldtr|sttr} Rt, [Rn{, #imm}]`

    use super::encode_ldur_stur;
    use super::super::EncodeResult;
    use crate::backend::arm::assembler::parser::Operand;
    use proptest::prelude::*;
    use std::io::Write;
    use std::process::{Command, Stdio};

    const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
    const CASES: u32 = 1000;
    const IMM_MIN: i64 = -256;
    const IMM_MAX: i64 = 255;

    fn cfg() -> ProptestConfig {
        ProptestConfig::with_cases(CASES)
    }

    fn rt_gpr(n: u32, is_64: bool) -> String {
        if is_64 {
            if n == 31 {
                "xzr".into()
            } else {
                format!("x{}", n)
            }
        } else if n == 31 {
            "wzr".into()
        } else {
            format!("w{}", n)
        }
    }

    fn rt_simd(n: u32, kind: char) -> String {
        format!("{}{}", kind, n)
    }

    fn rn_name(n: u32) -> String {
        if n == 31 {
            "sp".into()
        } else {
            format!("x{}", n)
        }
    }

    fn op2_bits(unpriv: bool) -> u32 {
        if unpriv {
            0b10
        } else {
            0b00
        }
    }

    fn mnemonic(is_load: bool, unpriv: bool) -> &'static str {
        match (is_load, unpriv) {
            (true, false) => "ldur",
            (false, false) => "stur",
            (true, true) => "ldtr",
            (false, true) => "sttr",
        }
    }

    fn asm_mem(rn: &str, offset: i64) -> String {
        if offset == 0 {
            format!("[{}]", rn)
        } else {
            format!("[{}, #{}]", rn, offset)
        }
    }

    fn valid_gpr_ops(rt: u32, rn: u32, offset: i64, is_64: bool) -> Vec<Operand> {
        vec![
            Operand::Reg(rt_gpr(rt, is_64)),
            Operand::Mem {
                base: rn_name(rn),
                offset,
            },
        ]
    }

    fn valid_simd_ops(rt: u32, rn: u32, offset: i64, kind: char) -> Vec<Operand> {
        vec![
            Operand::Reg(rt_simd(rt, kind)),
            Operand::Mem {
                base: rn_name(rn),
                offset,
            },
        ]
    }

    fn sut_word(ops: &[Operand], is_load: bool, op2: u32) -> Result<u32, String> {
        match encode_ldur_stur(ops, is_load, op2)? {
            EncodeResult::Word(w) => Ok(w),
            other => Err(format!("expected Word, got {:?}", other)),
        }
    }

    /// Unpack LDUR/STUR/LDTR/STTR fields per ARM ARM (not a copy of the SUT packer).
    fn unpack_ldur(word: u32) -> (u32, u32, u32, i64, u32, u32, u32) {
        let size = (word >> 30) & 0b11;
        let v = (word >> 26) & 1;
        let opc = (word >> 22) & 0b11;
        let imm9_raw = (word >> 12) & 0x1ff;
        let imm9 = if (imm9_raw & 0x100) != 0 {
            (imm9_raw as i64) | !0x1ffi64
        } else {
            imm9_raw as i64
        };
        let op2 = (word >> 10) & 0b11;
        let rn = (word >> 5) & 0x1f;
        let rt = word & 0x1f;
        (size, v, opc, imm9, op2, rn, rt)
    }

    fn fixed_ldur_bits(word: u32) -> bool {
        ((word >> 27) & 0b111) == 0b111
            && ((word >> 24) & 0b11) == 0b00
            && ((word >> 21) & 1) == 0
    }

    fn expected_gpr_size_opc(is_64: bool, is_load: bool) -> (u32, u32) {
        let size = if is_64 { 0b11 } else { 0b10 };
        let opc = if is_load { 0b01 } else { 0b00 };
        (size, opc)
    }

    fn expected_simd_size_opc(kind: char, is_load: bool) -> (u32, u32) {
        match kind {
            'q' => (0b00, if is_load { 0b11 } else { 0b10 }),
            'd' => (0b11, if is_load { 0b01 } else { 0b00 }),
            's' => (0b10, if is_load { 0b01 } else { 0b00 }),
            'h' => (0b01, if is_load { 0b01 } else { 0b00 }),
            _ => (0b00, if is_load { 0b01 } else { 0b00 }), // b
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

    fn reg_edge() -> impl Strategy<Value = u32> {
        prop_oneof![Just(0u32), Just(1u32), Just(30u32), Just(31u32), 0u32..=31]
    }

    fn imm9_in_range() -> impl Strategy<Value = i64> {
        prop_oneof![
            Just(IMM_MIN),
            Just(IMM_MIN + 1),
            Just(-1i64),
            Just(0i64),
            Just(1i64),
            Just(IMM_MAX - 1),
            Just(IMM_MAX),
            IMM_MIN..=IMM_MAX,
        ]
    }

    fn imm9_out_of_range() -> impl Strategy<Value = i64> {
        prop_oneof![
            Just(IMM_MIN - 1),
            Just(IMM_MAX + 1),
            Just(i64::MIN),
            Just(i64::MAX),
            Just(512i64),
            Just(-512i64),
            Just(1i64 << 40),
            (i64::MIN..=IMM_MIN - 1),
            (IMM_MAX + 1..=i64::MAX),
        ]
    }

    fn simd_kind() -> impl Strategy<Value = char> {
        prop_oneof![
            Just('b'),
            Just('h'),
            Just('s'),
            Just('d'),
            Just('q'),
        ]
    }

    fn extra_operand() -> impl Strategy<Value = Operand> {
        prop_oneof![
            Just(Operand::Reg("x2".into())),
            Just(Operand::Imm(0)),
            Just(Operand::Imm(1)),
            Just(Operand::Symbol("foo".into())),
            Just(Operand::Mem {
                base: "x3".into(),
                offset: 0
            }),
        ]
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_ldur_stur_kat_llvm_mc_ldur_x0_x1() {
        let want = 0xf8400020u32;
        let mc = llvm_mc_word("ldur x0, [x1]").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Mem {
                base: "x1".into(),
                offset: 0,
            },
        ];
        let sut = sut_word(&ops, true, 0b00).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_ldur_stur_kat_llvm_mc_ldur_q0_x1() {
        let want = 0x3cc00020u32;
        let mc = llvm_mc_word("ldur q0, [x1]").expect("llvm-mc KAT Q");
        assert_eq!(mc, want, "llvm-mc KAT Q mapping broken");
        let ops = [
            Operand::Reg("q0".into()),
            Operand::Mem {
                base: "x1".into(),
                offset: 0,
            },
        ];
        let sut = sut_word(&ops, true, 0b00).expect("SUT KAT Q");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_ldur_stur_kat_llvm_mc_ldtr_x0_x1() {
        let want = 0xf8400820u32;
        let mc = llvm_mc_word("ldtr x0, [x1]").expect("llvm-mc KAT LDTR");
        assert_eq!(mc, want, "llvm-mc KAT LDTR mapping broken");
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Mem {
                base: "x1".into(),
                offset: 0,
            },
        ];
        let sut = sut_word(&ops, true, 0b10).expect("SUT KAT LDTR");
        assert_eq!(sut, want);
    }

    #[test]
    fn test_encode_ldur_stur_regression_extra_operand() {
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Mem {
                base: "x0".into(),
                offset: -256,
            },
            Operand::Reg("x2".into()),
        ];
        assert!(
            encode_ldur_stur(&ops, false, 0b00).is_err(),
            "stur w0, [x0, #-256], x2 must Err; llvm-mc rejects a 3rd operand"
        );
    }

    #[test]
    fn test_encode_ldur_stur_regression_imm9_range() {
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Mem {
                base: "x0".into(),
                offset: -257,
            },
        ];
        assert!(
            encode_ldur_stur(&ops, false, 0b00).is_err(),
            "stur w0, [x0, #-257] must Err; llvm-mc requires simm9 in [-256, 255]"
        );
    }

    #[test]
    fn test_encode_ldur_stur_regression_sp_as_rt() {
        let ops = [
            Operand::Reg("sp".into()),
            Operand::Mem {
                base: "x0".into(),
                offset: -256,
            },
        ];
        assert!(
            encode_ldur_stur(&ops, false, 0b00).is_err(),
            "stur sp, [x0, #-256] must Err; llvm-mc rejects SP as Rt"
        );
    }

    #[test]
    fn test_encode_ldur_stur_regression_w_base() {
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Mem {
                base: "w0".into(),
                offset: 0,
            },
        ];
        assert!(
            encode_ldur_stur(&ops, true, 0b00).is_err(),
            "ldur x0, [w0] must Err; llvm-mc rejects a W register as base"
        );
    }

    #[test]
    fn test_encode_ldur_stur_regression_xzr_base() {
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Mem {
                base: "xzr".into(),
                offset: 0,
            },
        ];
        assert!(
            encode_ldur_stur(&ops, true, 0b00).is_err(),
            "ldur x0, [xzr] must Err; llvm-mc rejects XZR as base (Rn=31 is SP)"
        );
    }

    #[test]
    fn test_encode_ldur_stur_regression_simd_ldtr() {
        let ops = [
            Operand::Reg("d0".into()),
            Operand::Mem {
                base: "x0".into(),
                offset: 0,
            },
        ];
        assert!(
            encode_ldur_stur(&ops, true, 0b10).is_err(),
            "ldtr d0, [x0] must Err; llvm-mc rejects SIMD Rt on LDTR"
        );
    }

    #[test]
    fn test_encode_ldur_stur_regression_v_reg() {
        let ops = [
            Operand::Reg("v0".into()),
            Operand::Mem {
                base: "x0".into(),
                offset: 0,
            },
        ];
        assert!(
            encode_ldur_stur(&ops, true, 0b00).is_err(),
            "ldur v0, [x0] must Err; llvm-mc rejects V-register Rt without arrangement"
        );
    }

    #[test]
    fn test_encode_ldur_stur_regression_lr_as_x30() {
        let ops_lr = [
            Operand::Reg("lr".into()),
            Operand::Mem {
                base: "x0".into(),
                offset: 0,
            },
        ];
        let ops_x30 = [
            Operand::Reg("x30".into()),
            Operand::Mem {
                base: "x0".into(),
                offset: 0,
            },
        ];
        let lr = sut_word(&ops_lr, true, 0b00).expect("ldur lr is a valid X30 alias");
        let x30 = sut_word(&ops_x30, true, 0b00).expect("ldur x30");
        assert_eq!(
            lr, x30,
            "ldur lr, [x0] must encode as ldur x30, [x0]; llvm-mc accepts lr as X30"
        );
    }

    proptest! {
        #![proptest_config(cfg())]

        // Oracle: differential
        // Target: encoder.load_store.encode_ldur_stur
        #[test]
        fn encode_ldur_stur_diff_gpr_llvm_mc(
            rt in reg_edge(),
            rn in reg_edge(),
            offset in imm9_in_range(),
            is_load in any::<bool>(),
            is_64 in any::<bool>(),
            unpriv in any::<bool>(),
        ) {
            let mn = mnemonic(is_load, unpriv);
            let rt_n = rt_gpr(rt, is_64);
            let rn_n = rn_name(rn);
            let asm = format!("{} {}, {}", mn, rt_n, asm_mem(&rn_n, offset));
            let ops = valid_gpr_ops(rt, rn, offset, is_64);
            let sut = sut_word(&ops, is_load, op2_bits(unpriv))
                .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
            let mc = llvm_mc_word(&asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
            prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
        }

        // Oracle: differential
        // Target: encoder.load_store.encode_ldur_stur
        #[test]
        fn encode_ldur_stur_diff_simd_llvm_mc(
            rt in reg_edge(),
            rn in reg_edge(),
            offset in imm9_in_range(),
            is_load in any::<bool>(),
            kind in simd_kind(),
        ) {
            let mn = mnemonic(is_load, false);
            let rt_n = rt_simd(rt, kind);
            let rn_n = rn_name(rn);
            let asm = format!("{} {}, {}", mn, rt_n, asm_mem(&rn_n, offset));
            let ops = valid_simd_ops(rt, rn, offset, kind);
            let sut = sut_word(&ops, is_load, 0b00)
                .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
            let mc = llvm_mc_word(&asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
            prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
        }

        // Oracle: algebraic.invariant
        // Target: encoder.load_store.encode_ldur_stur
        #[test]
        fn encode_ldur_stur_roundtrip_arm_fields(
            rt in reg_edge(),
            rn in reg_edge(),
            offset in imm9_in_range(),
            is_load in any::<bool>(),
            is_64 in any::<bool>(),
            unpriv in any::<bool>(),
            kind in simd_kind(),
            use_simd in any::<bool>(),
        ) {
            let (ops, exp_size, exp_v, exp_opc) = if use_simd && !unpriv {
                let (sz, opc) = expected_simd_size_opc(kind, is_load);
                (valid_simd_ops(rt, rn, offset, kind), sz, 1u32, opc)
            } else {
                let (sz, opc) = expected_gpr_size_opc(is_64, is_load);
                (valid_gpr_ops(rt, rn, offset, is_64), sz, 0u32, opc)
            };
            let op2 = op2_bits(unpriv && !(use_simd && !unpriv));
            let word = sut_word(&ops, is_load, op2)
                .unwrap_or_else(|e| panic!("SUT rejected in-range LDUR/STUR: {}", e));
            let (got_size, got_v, got_opc, got_imm, got_op2, got_rn, got_rt) = unpack_ldur(word);
            prop_assert!(fixed_ldur_bits(word), "fixed bits violated word={:#010x}", word);
            prop_assert_eq!(got_size, exp_size, "size mismatch word={:#010x}", word);
            prop_assert_eq!(got_v, exp_v, "V mismatch word={:#010x}", word);
            prop_assert_eq!(got_opc, exp_opc, "opc mismatch word={:#010x}", word);
            prop_assert_eq!(got_imm, offset, "imm9 mismatch word={:#010x}", word);
            prop_assert_eq!(got_op2, op2, "op2 mismatch word={:#010x}", word);
            prop_assert_eq!(got_rn, rn, "Rn mismatch word={:#010x}", word);
            prop_assert_eq!(got_rt, rt, "Rt mismatch word={:#010x}", word);
        }

        // Oracle: algebraic.metamorphic
        // Target: encoder.load_store.encode_ldur_stur
        #[test]
        fn encode_ldur_stur_metamorphic_load_xor_store(
            rt in reg_edge(),
            rn in reg_edge(),
            offset in imm9_in_range(),
            is_64 in any::<bool>(),
            unpriv in any::<bool>(),
            kind in simd_kind(),
            use_simd in any::<bool>(),
        ) {
            let (ops, op2) = if use_simd {
                (valid_simd_ops(rt, rn, offset, kind), 0b00u32)
            } else {
                (valid_gpr_ops(rt, rn, offset, is_64), op2_bits(unpriv))
            };
            let load = sut_word(&ops, true, op2)
                .unwrap_or_else(|e| panic!("SUT rejected load: {}", e));
            let store = sut_word(&ops, false, op2)
                .unwrap_or_else(|e| panic!("SUT rejected store: {}", e));
            prop_assert_eq!(
                load ^ store,
                1u32 << 22,
                "load vs store must differ only by opc bit 22 load={:#010x} store={:#010x}",
                load,
                store
            );
        }

        // Oracle: algebraic.metamorphic
        // Target: encoder.load_store.encode_ldur_stur
        #[test]
        fn encode_ldur_stur_metamorphic_unscaled_xor_unpriv(
            rt in reg_edge(),
            rn in reg_edge(),
            offset in imm9_in_range(),
            is_load in any::<bool>(),
            is_64 in any::<bool>(),
        ) {
            let ops = valid_gpr_ops(rt, rn, offset, is_64);
            let unscaled = sut_word(&ops, is_load, 0b00)
                .unwrap_or_else(|e| panic!("SUT rejected LDUR/STUR: {}", e));
            let unpriv = sut_word(&ops, is_load, 0b10)
                .unwrap_or_else(|e| panic!("SUT rejected LDTR/STTR: {}", e));
            prop_assert_eq!(
                unscaled ^ unpriv,
                1u32 << 11,
                "unscaled vs unpriv must differ only by bit 11 u={:#010x} p={:#010x}",
                unscaled,
                unpriv
            );
        }

        // Oracle: negative_error
        // Target: encoder.load_store.encode_ldur_stur
        // Invalid domain: a third operand.
        #[test]
        fn encode_ldur_stur_neg_extra_operands(
            rt in reg_edge(),
            rn in reg_edge(),
            offset in imm9_in_range(),
            is_load in any::<bool>(),
            is_64 in any::<bool>(),
            unpriv in any::<bool>(),
            extra in extra_operand(),
        ) {
            let mut ops = valid_gpr_ops(rt, rn, offset, is_64);
            ops.push(extra);
            prop_assert!(
                encode_ldur_stur(&ops, is_load, op2_bits(unpriv)).is_err(),
                "extra operand must Err; llvm-mc rejects a 3rd operand"
            );
        }

        // Oracle: negative_error
        // Target: encoder.load_store.encode_ldur_stur
        // Invalid domain: simm9 outside [-256, 255].
        #[test]
        fn encode_ldur_stur_neg_imm9_range(
            rt in reg_edge(),
            rn in reg_edge(),
            offset in imm9_out_of_range(),
            is_load in any::<bool>(),
            is_64 in any::<bool>(),
            unpriv in any::<bool>(),
        ) {
            let ops = valid_gpr_ops(rt, rn, offset, is_64);
            prop_assert!(
                encode_ldur_stur(&ops, is_load, op2_bits(unpriv)).is_err(),
                "offset {} outside [-256, 255] must Err",
                offset
            );
        }

        // Oracle: negative_error
        // Target: encoder.load_store.encode_ldur_stur
        // Invalid domain: SP/WSP as Rt; W/WSP/XZR/WZR as base; SIMD Rt on LDTR/STTR; V-register Rt.
        #[test]
        fn encode_ldur_stur_neg_invalid_rt_rn(
            is_load in any::<bool>(),
            kind in 0u32..=7,
            n in 0u32..=31,
            offset in imm9_in_range(),
        ) {
            let (ops, op2): (Vec<Operand>, u32) = match kind {
                0 => (
                    vec![
                        Operand::Reg("sp".into()),
                        Operand::Mem {
                            base: "x0".into(),
                            offset,
                        },
                    ],
                    0b00,
                ),
                1 => (
                    vec![
                        Operand::Reg("wsp".into()),
                        Operand::Mem {
                            base: "x0".into(),
                            offset,
                        },
                    ],
                    0b00,
                ),
                2 => (
                    vec![
                        Operand::Reg(rt_gpr(n, true)),
                        Operand::Mem {
                            base: format!("w{}", n % 31),
                            offset,
                        },
                    ],
                    0b00,
                ),
                3 => (
                    vec![
                        Operand::Reg(rt_gpr(n, true)),
                        Operand::Mem {
                            base: "wsp".into(),
                            offset,
                        },
                    ],
                    0b00,
                ),
                4 => (
                    vec![
                        Operand::Reg(rt_gpr(n, true)),
                        Operand::Mem {
                            base: "xzr".into(),
                            offset,
                        },
                    ],
                    0b00,
                ),
                5 => (
                    vec![
                        Operand::Reg(rt_gpr(n, true)),
                        Operand::Mem {
                            base: "wzr".into(),
                            offset,
                        },
                    ],
                    0b00,
                ),
                6 => (
                    vec![
                        Operand::Reg(rt_simd(n, ['b', 'h', 's', 'd', 'q'][(n as usize) % 5])),
                        Operand::Mem {
                            base: "x0".into(),
                            offset,
                        },
                    ],
                    0b10,
                ),
                _ => (
                    vec![
                        Operand::Reg(format!("v{}", n)),
                        Operand::Mem {
                            base: "x0".into(),
                            offset,
                        },
                    ],
                    0b00,
                ),
            };
            prop_assert!(
                encode_ldur_stur(&ops, is_load, op2).is_err(),
                "invalid Rt/Rn kind {} must Err; got {:?}",
                kind,
                encode_ldur_stur(&ops, is_load, op2)
            );
        }

        // Oracle: negative_error
        // Target: encoder.load_store.encode_ldur_stur
        // Invalid domain: fewer than 2 operands, non-Reg Rt, non-Mem addressing, invalid base name.
        #[test]
        fn encode_ldur_stur_neg_arity_and_shape(
            is_load in any::<bool>(),
            unpriv in any::<bool>(),
            shape in 0u32..=8,
            rt in reg_edge(),
            off in imm9_in_range(),
        ) {
            let op2 = op2_bits(unpriv);
            let rt_n = rt_gpr(rt, true);
            let ops: Vec<Operand> = match shape {
                0 => vec![],
                1 => vec![Operand::Reg(rt_n)],
                2 => vec![Operand::Reg(rt_n.clone()), Operand::Imm(0)],
                3 => vec![Operand::Reg(rt_n.clone()), Operand::Symbol("foo".into())],
                4 => vec![
                    Operand::Reg(rt_n.clone()),
                    Operand::MemPreIndex {
                        base: "x0".into(),
                        offset: off,
                    },
                ],
                5 => vec![
                    Operand::Reg(rt_n.clone()),
                    Operand::MemPostIndex {
                        base: "x0".into(),
                        offset: off,
                    },
                ],
                6 => vec![
                    Operand::Imm(0),
                    Operand::Mem {
                        base: "x0".into(),
                        offset: 0,
                    },
                ],
                7 => vec![
                    Operand::Reg(rt_n),
                    Operand::Mem {
                        base: "foo".into(),
                        offset: 0,
                    },
                ],
                _ => vec![
                    Operand::Reg(rt_gpr(rt, true)),
                    Operand::Mem {
                        base: "x32".into(),
                        offset: 0,
                    },
                ],
            };
            prop_assert!(
                encode_ldur_stur(&ops, is_load, op2).is_err(),
                "expected Err for arity/shape {} got {:?}",
                shape,
                encode_ldur_stur(&ops, is_load, op2)
            );
        }

        // Oracle: differential
        // Target: encoder.load_store.encode_ldur_stur
        // lr is the architectural alias of X30 (is_64bit_reg / encode_ldr_str_auto / llvm-mc).
        #[test]
        fn encode_ldur_stur_diff_lr_alias(
            rn in reg_edge(),
            offset in imm9_in_range(),
            is_load in any::<bool>(),
            unpriv in any::<bool>(),
        ) {
            let mn = mnemonic(is_load, unpriv);
            let rn_n = rn_name(rn);
            let asm = format!("{} lr, {}", mn, asm_mem(&rn_n, offset));
            let ops = [
                Operand::Reg("lr".into()),
                Operand::Mem {
                    base: rn_n,
                    offset,
                },
            ];
            let sut = sut_word(&ops, is_load, op2_bits(unpriv))
                .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
            let mc = llvm_mc_word(&asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
            prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
        }
    }
}

#[cfg(test)]
mod encode_ldxp_stxp_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler
    // Evidence: src/backend/arm/assembler/README.md "accepts the same textual assembly that GCC's gas would consume";
    //   encoder/mod.rs:1-7 32-bit AArch64 words; encoder/mod.rs:366-369 ldxp/ldaxp/stxp/stlxp dispatch;
    //   ARM ARM Load/Store Exclusive Pair: size 001000 0 L 1 Rs o0 Rt2 Rn Rt;
    //   Rt/Rt2 are Wt/Xt (31=ZR); Rn is Xn|SP; Ws is Wt (31=WZR); offset {,#0}.
    // Stronger considered:
    //   - State machine: rejected — encode_ldxp_stxp is a pure function with no lifecycle
    //   - Algebraic round-trip via in-tree decoder: rejected — no exclusive-pair decoder
    //   - encode_ldxr_stxr / encode_ldaxr_stlxr / encode_ldp_stp as differential siblings:
    //     rejected — exclusive-single / non-exclusive pair, different job
    // Weaker available: algebraic.invariant (ARM field unpack), algebraic.metamorphic (o0 / sz),
    //   negative_error (arity / extra / SP / FP / W-base / XZR-base / mixed width / X-Ws / offset)
    // Differential: candidate=encode_ldxp_stxp, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=load (Reg(Rt), Reg(Rt2), Mem{Rn, 0}, is_load=true, acqrel)
    //           <-> `{ldxp|ldaxp} Rt, Rt2, [Rn]`;
    //           store (Reg(Ws), Reg(Rt), Reg(Rt2), Mem{Rn, 0}, is_load=false, acqrel)
    //           <-> `{stxp|stlxp} Ws, Rt, Rt2, [Rn]`

    use super::encode_ldxp_stxp;
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

    fn data_name(n: u32, is_64: bool) -> String {
        if is_64 {
            if n == 31 {
                "xzr".into()
            } else {
                format!("x{}", n)
            }
        } else if n == 31 {
            "wzr".into()
        } else {
            format!("w{}", n)
        }
    }

    fn rn_name(n: u32) -> String {
        if n == 31 {
            "sp".into()
        } else {
            format!("x{}", n)
        }
    }

    fn ws_name(n: u32) -> String {
        if n == 31 {
            "wzr".into()
        } else {
            format!("w{}", n)
        }
    }

    /// llvm-mc / ARM: STXP status must not also be a source (Rt, Rt2, or Xn).
    /// WZR vs SP (both encode as 31) is allowed: `stxp wzr, x1, x2, [sp]`.
    fn stxp_ws_aliases_source(ws: u32, rt: u32, rt2: u32, rn: u32) -> bool {
        ws == rt || ws == rt2 || (rn != 31 && ws == rn)
    }

    fn mnemonic(is_load: bool, acqrel: bool) -> &'static str {
        match (is_load, acqrel) {
            (true, false) => "ldxp",
            (true, true) => "ldaxp",
            (false, false) => "stxp",
            (false, true) => "stlxp",
        }
    }

    fn load_ops(rt: u32, rt2: u32, rn: u32, is_64: bool) -> Vec<Operand> {
        vec![
            Operand::Reg(data_name(rt, is_64)),
            Operand::Reg(data_name(rt2, is_64)),
            Operand::Mem {
                base: rn_name(rn),
                offset: 0,
            },
        ]
    }

    fn store_ops(ws: u32, rt: u32, rt2: u32, rn: u32, is_64: bool) -> Vec<Operand> {
        vec![
            Operand::Reg(ws_name(ws)),
            Operand::Reg(data_name(rt, is_64)),
            Operand::Reg(data_name(rt2, is_64)),
            Operand::Mem {
                base: rn_name(rn),
                offset: 0,
            },
        ]
    }

    fn valid_ops(
        rt: u32,
        rt2: u32,
        rn: u32,
        ws: u32,
        is_load: bool,
        is_64: bool,
    ) -> Vec<Operand> {
        if is_load {
            load_ops(rt, rt2, rn, is_64)
        } else {
            store_ops(ws, rt, rt2, rn, is_64)
        }
    }

    fn sut_word(ops: &[Operand], is_load: bool, acqrel: bool) -> Result<u32, String> {
        match encode_ldxp_stxp(ops, is_load, acqrel)? {
            EncodeResult::Word(w) => Ok(w),
            other => Err(format!("expected Word, got {:?}", other)),
        }
    }

    /// Unpack exclusive-pair fields per ARM ARM (not a copy of the SUT packer).
    fn unpack_ldxp_stxp(word: u32) -> (u32, u32, u32, u32, u32, u32, u32, u32) {
        let size = (word >> 30) & 0b11;
        let l = (word >> 22) & 1;
        let o1 = (word >> 21) & 1;
        let rs = (word >> 16) & 0x1f;
        let o0 = (word >> 15) & 1;
        let rt2 = (word >> 10) & 0x1f;
        let rn = (word >> 5) & 0x1f;
        let rt = word & 0x1f;
        (size, l, o1, rs, o0, rt2, rn, rt)
    }

    fn fixed_ldxp_stxp_bits(word: u32) -> bool {
        ((word >> 31) & 1) == 1
            && ((word >> 24) & 0x3f) == 0b001000
            && ((word >> 23) & 1) == 0
            && ((word >> 21) & 1) == 1
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

    fn reg_edge() -> impl Strategy<Value = u32> {
        prop_oneof![Just(0u32), Just(1u32), Just(30u32), Just(31u32), 0u32..=31]
    }

    fn extra_operand() -> impl Strategy<Value = Operand> {
        prop_oneof![
            Just(Operand::Reg("x2".into())),
            Just(Operand::Imm(0)),
            Just(Operand::Imm(1)),
            Just(Operand::Symbol("foo".into())),
            Just(Operand::Mem {
                base: "x3".into(),
                offset: 0
            }),
        ]
    }

    fn nonzero_offset() -> impl Strategy<Value = i64> {
        prop_oneof![
            Just(-1i64),
            Just(1i64),
            Just(8i64),
            Just(-8i64),
            Just(256i64),
            Just(i64::MIN),
            Just(i64::MAX),
            1i64..=4096,
            -4096i64..=-1,
        ]
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_ldxp_stxp_kat_llvm_mc_ldxp_x0_x1_x2() {
        let want = 0xc87f0440u32;
        let mc = llvm_mc_word("ldxp x0, x1, [x2]").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = load_ops(0, 1, 2, true);
        let sut = sut_word(&ops, true, false).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_ldxp_stxp_kat_llvm_mc_ldxp_w0_w1_x2() {
        let want = 0x887f0440u32;
        let mc = llvm_mc_word("ldxp w0, w1, [x2]").expect("llvm-mc KAT w");
        assert_eq!(mc, want, "llvm-mc KAT w mapping broken");
        let ops = load_ops(0, 1, 2, false);
        let sut = sut_word(&ops, true, false).expect("SUT KAT w");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_ldxp_stxp_kat_llvm_mc_ldaxp_x0_x1_x2() {
        let want = 0xc87f8440u32;
        let mc = llvm_mc_word("ldaxp x0, x1, [x2]").expect("llvm-mc KAT ldaxp");
        assert_eq!(mc, want, "llvm-mc KAT ldaxp mapping broken");
        let ops = load_ops(0, 1, 2, true);
        let sut = sut_word(&ops, true, true).expect("SUT KAT ldaxp");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_ldxp_stxp_kat_llvm_mc_stxp_w0_x1_x2_x3() {
        let want = 0xc8200861u32;
        let mc = llvm_mc_word("stxp w0, x1, x2, [x3]").expect("llvm-mc KAT stxp");
        assert_eq!(mc, want, "llvm-mc KAT stxp mapping broken");
        let ops = store_ops(0, 1, 2, 3, true);
        let sut = sut_word(&ops, false, false).expect("SUT KAT stxp");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_ldxp_stxp_kat_llvm_mc_stlxp_w0_x1_x2_x3() {
        let want = 0xc8208861u32;
        let mc = llvm_mc_word("stlxp w0, x1, x2, [x3]").expect("llvm-mc KAT stlxp");
        assert_eq!(mc, want, "llvm-mc KAT stlxp mapping broken");
        let ops = store_ops(0, 1, 2, 3, true);
        let sut = sut_word(&ops, false, true).expect("SUT KAT stlxp");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_ldxp_stxp_kat_llvm_mc_ldxp_xzr_sp() {
        let want = 0xc87f07e0u32;
        let mc = llvm_mc_word("ldxp x0, x1, [sp]").expect("llvm-mc KAT sp");
        assert_eq!(mc, want, "llvm-mc KAT sp mapping broken");
        let ops = load_ops(0, 1, 31, true);
        let sut = sut_word(&ops, true, false).expect("SUT KAT sp");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_ldxp_stxp_kat_llvm_mc_ldxp_lr() {
        let want = 0xc87f045eu32;
        let mc = llvm_mc_word("ldxp lr, x1, [x2]").expect("llvm-mc KAT lr");
        assert_eq!(mc, want, "llvm-mc KAT lr mapping broken");
        let ops = vec![
            Operand::Reg("lr".into()),
            Operand::Reg("x1".into()),
            Operand::Mem {
                base: "x2".into(),
                offset: 0,
            },
        ];
        let sut = sut_word(&ops, true, false).expect("SUT KAT lr");
        assert_eq!(sut, want);
    }

    #[test]
    fn test_encode_ldxp_stxp_regression_extra_operand() {
        let mut ops = store_ops(0, 1, 2, 3, true);
        ops.push(Operand::Reg("x2".into()));
        assert!(
            encode_ldxp_stxp(&ops, false, false).is_err(),
            "stxp w0, x1, x2, [x3], x2 must Err; llvm-mc rejects a 5th operand"
        );
    }

    #[test]
    fn test_encode_ldxp_stxp_regression_sp_as_rt() {
        let ops = [
            Operand::Reg(ws_name(0)),
            Operand::Reg("sp".into()),
            Operand::Reg("w0".into()),
            Operand::Mem {
                base: "x0".into(),
                offset: 0,
            },
        ];
        assert!(
            encode_ldxp_stxp(&ops, false, false).is_err(),
            "stxp w0, sp, w0, [x0] must Err; llvm-mc rejects SP as Rt"
        );
    }

    #[test]
    fn test_encode_ldxp_stxp_regression_w_base() {
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Reg("w1".into()),
            Operand::Mem {
                base: "w0".into(),
                offset: 0,
            },
        ];
        assert!(
            encode_ldxp_stxp(&ops, true, false).is_err(),
            "ldxp w0, w1, [w0] must Err; llvm-mc rejects a W register as base"
        );
    }

    #[test]
    fn test_encode_ldxp_stxp_regression_xzr_as_base() {
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("x1".into()),
            Operand::Mem {
                base: "xzr".into(),
                offset: 0,
            },
        ];
        assert!(
            encode_ldxp_stxp(&ops, true, false).is_err(),
            "ldxp x0, x1, [xzr] must Err; llvm-mc rejects XZR as base"
        );
    }

    #[test]
    fn test_encode_ldxp_stxp_regression_fp_as_rt() {
        let ops = [
            Operand::Reg("d0".into()),
            Operand::Reg("x1".into()),
            Operand::Mem {
                base: "x2".into(),
                offset: 0,
            },
        ];
        assert!(
            encode_ldxp_stxp(&ops, true, false).is_err(),
            "ldxp d0, x1, [x2] must Err; llvm-mc rejects SIMD/FP as Rt"
        );
    }

    #[test]
    fn test_encode_ldxp_stxp_regression_mixed_width() {
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("w1".into()),
            Operand::Mem {
                base: "x2".into(),
                offset: 0,
            },
        ];
        assert!(
            encode_ldxp_stxp(&ops, true, false).is_err(),
            "ldxp x0, w1, [x2] must Err; llvm-mc rejects mixed X/W pair"
        );
    }

    #[test]
    fn test_encode_ldxp_stxp_regression_x_as_ws() {
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("x1".into()),
            Operand::Reg("x2".into()),
            Operand::Mem {
                base: "x3".into(),
                offset: 0,
            },
        ];
        assert!(
            encode_ldxp_stxp(&ops, false, false).is_err(),
            "stxp x0, x1, x2, [x3] must Err; llvm-mc rejects X as STXP status"
        );
    }

    #[test]
    fn test_encode_ldxp_stxp_regression_nonzero_offset() {
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Mem {
                base: "x0".into(),
                offset: -1,
            },
        ];
        assert!(
            encode_ldxp_stxp(&ops, false, false).is_err(),
            "stxp w0, w0, w0, [x0, #-1] must Err; llvm-mc: index must be absent or #0"
        );
    }

    #[test]
    fn test_encode_ldxp_stxp_regression_ws_overlap() {
        let ops = store_ops(0, 0, 1, 2, false);
        assert!(
            encode_ldxp_stxp(&ops, false, false).is_err(),
            "stxp w0, w0, w1, [x2] must Err; llvm-mc: unpredictable STXP, status is also a source"
        );
    }

    proptest! {
        #![proptest_config(cfg())]

        // Oracle: differential
        // Target: encoder.load_store.encode_ldxp_stxp
        #[test]
        fn encode_ldxp_stxp_diff_llvm_mc(
            rt in reg_edge(),
            rt2 in reg_edge(),
            rn in reg_edge(),
            ws in reg_edge(),
            is_load in any::<bool>(),
            acqrel in any::<bool>(),
            is_64 in any::<bool>(),
        ) {
            // ARM CONSTRAINED UNPREDICTABLE / llvm-mc: "status is also a source".
            if !is_load {
                prop_assume!(!stxp_ws_aliases_source(ws, rt, rt2, rn));
            }
            let ops = valid_ops(rt, rt2, rn, ws, is_load, is_64);
            let mnem = mnemonic(is_load, acqrel);
            let asm = if is_load {
                format!(
                    "{} {}, {}, [{}]",
                    mnem,
                    data_name(rt, is_64),
                    data_name(rt2, is_64),
                    rn_name(rn)
                )
            } else {
                format!(
                    "{} {}, {}, {}, [{}]",
                    mnem,
                    ws_name(ws),
                    data_name(rt, is_64),
                    data_name(rt2, is_64),
                    rn_name(rn)
                )
            };
            let sut = sut_word(&ops, is_load, acqrel)
                .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
            let mc = llvm_mc_word(&asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
            prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
        }

        // Oracle: algebraic.invariant
        // Target: encoder.load_store.encode_ldxp_stxp
        #[test]
        fn encode_ldxp_stxp_invariant_arm_fields(
            rt in reg_edge(),
            rt2 in reg_edge(),
            rn in reg_edge(),
            ws in reg_edge(),
            is_load in any::<bool>(),
            acqrel in any::<bool>(),
            is_64 in any::<bool>(),
        ) {
            let ops = valid_ops(rt, rt2, rn, ws, is_load, is_64);
            let w = sut_word(&ops, is_load, acqrel)
                .unwrap_or_else(|e| panic!("SUT rejected valid exclusive pair: {}", e));
            prop_assert!(fixed_ldxp_stxp_bits(w), "fixed bits wrong: {w:#010x}");
            let (size, l, o1, rs, o0, got_rt2, got_rn, got_rt) = unpack_ldxp_stxp(w);
            let expect_size = if is_64 { 0b11 } else { 0b10 };
            prop_assert_eq!(size, expect_size);
            prop_assert_eq!(l, if is_load { 1 } else { 0 });
            prop_assert_eq!(o1, 1);
            prop_assert_eq!(rs, if is_load { 31 } else { ws });
            prop_assert_eq!(o0, if acqrel { 1 } else { 0 });
            prop_assert_eq!(got_rt2, rt2);
            prop_assert_eq!(got_rn, rn);
            prop_assert_eq!(got_rt, rt);
        }

        // Oracle: algebraic.metamorphic
        // Target: encoder.load_store.encode_ldxp_stxp
        #[test]
        fn encode_ldxp_stxp_metamorphic_o0(
            rt in reg_edge(),
            rt2 in reg_edge(),
            rn in reg_edge(),
            ws in reg_edge(),
            is_load in any::<bool>(),
            is_64 in any::<bool>(),
        ) {
            let ops = valid_ops(rt, rt2, rn, ws, is_load, is_64);
            let w_rel = sut_word(&ops, is_load, false)
                .unwrap_or_else(|e| panic!("SUT rejected o0=0: {}", e));
            let w_acq = sut_word(&ops, is_load, true)
                .unwrap_or_else(|e| panic!("SUT rejected o0=1: {}", e));
            prop_assert_eq!(w_acq ^ w_rel, 1u32 << 15, "o0 bit is not the sole difference");
        }

        // Oracle: algebraic.metamorphic
        // Target: encoder.load_store.encode_ldxp_stxp
        #[test]
        fn encode_ldxp_stxp_metamorphic_sz(
            rt in reg_edge(),
            rt2 in reg_edge(),
            rn in reg_edge(),
            ws in reg_edge(),
            is_load in any::<bool>(),
            acqrel in any::<bool>(),
        ) {
            let ops_x = valid_ops(rt, rt2, rn, ws, is_load, true);
            let ops_w = valid_ops(rt, rt2, rn, ws, is_load, false);
            let w_x = sut_word(&ops_x, is_load, acqrel)
                .unwrap_or_else(|e| panic!("SUT rejected X pair: {}", e));
            let w_w = sut_word(&ops_w, is_load, acqrel)
                .unwrap_or_else(|e| panic!("SUT rejected W pair: {}", e));
            prop_assert_eq!(w_x ^ w_w, 1u32 << 30, "sz bit is not the sole difference");
        }

        // Oracle: negative_error
        // Target: encoder.load_store.encode_ldxp_stxp
        // llvm-mc: "unpredictable STXP instruction, status is also a source"
        #[test]
        fn encode_ldxp_stxp_neg_ws_overlap(
            rt in reg_edge(),
            rt2 in reg_edge(),
            rn in reg_edge(),
            acqrel in any::<bool>(),
            is_64 in any::<bool>(),
            kind in 0u32..=2,
        ) {
            let rn = if kind == 2 { rn % 31 } else { rn }; // Xn, never SP, so Wn==Xn aliases
            let ws = match kind {
                0 => rt,
                1 => rt2,
                _ => rn,
            };
            prop_assume!(stxp_ws_aliases_source(ws, rt, rt2, rn));
            let ops = store_ops(ws, rt, rt2, rn, is_64);
            prop_assert!(
                encode_ldxp_stxp(&ops, false, acqrel).is_err(),
                "stxp Ws overlapping Rt/Rt2/Rn must Err; llvm-mc rejects it; got {:?}",
                encode_ldxp_stxp(&ops, false, acqrel)
            );
        }

        // Oracle: negative_error
        // Target: encoder.load_store.encode_ldxp_stxp
        #[test]
        fn encode_ldxp_stxp_neg_arity_extra(
            rt in reg_edge(),
            rt2 in reg_edge(),
            rn in reg_edge(),
            ws in reg_edge(),
            is_load in any::<bool>(),
            acqrel in any::<bool>(),
            is_64 in any::<bool>(),
            extra in extra_operand(),
            short_len in 0usize..=2,
        ) {
            let mut ops = valid_ops(rt, rt2, rn, ws, is_load, is_64);
            ops.push(extra);
            prop_assert!(
                encode_ldxp_stxp(&ops, is_load, acqrel).is_err(),
                "extra operand must Err; llvm-mc rejects a surplus operand"
            );
            let short: Vec<Operand> = valid_ops(rt, rt2, rn, ws, is_load, is_64)
                .into_iter()
                .take(short_len)
                .collect();
            prop_assert!(
                encode_ldxp_stxp(&short, is_load, acqrel).is_err(),
                "too few operands must Err"
            );
        }

        // Oracle: negative_error
        // Target: encoder.load_store.encode_ldxp_stxp
        #[test]
        fn encode_ldxp_stxp_neg_invalid_rt_base(
            n in reg_edge(),
            rn in 0u32..=30,
            is_load in any::<bool>(),
            acqrel in any::<bool>(),
            is_64 in any::<bool>(),
            kind in 0u32..=9,
        ) {
            let data = data_name(n, is_64);
            let base_x = rn_name(rn);
            let ops: Vec<Operand> = match kind {
                0 => {
                    // SP as Rt (register 31 is ZR, never SP)
                    if is_load {
                        vec![
                            Operand::Reg("sp".into()),
                            Operand::Reg(data.clone()),
                            Operand::Mem {
                                base: base_x,
                                offset: 0,
                            },
                        ]
                    } else {
                        vec![
                            Operand::Reg(ws_name(0)),
                            Operand::Reg("sp".into()),
                            Operand::Reg(data),
                            Operand::Mem {
                                base: base_x,
                                offset: 0,
                            },
                        ]
                    }
                }
                1 => {
                    // SP as Rt2
                    if is_load {
                        vec![
                            Operand::Reg(data),
                            Operand::Reg("sp".into()),
                            Operand::Mem {
                                base: base_x,
                                offset: 0,
                            },
                        ]
                    } else {
                        vec![
                            Operand::Reg(ws_name(0)),
                            Operand::Reg(data),
                            Operand::Reg("sp".into()),
                            Operand::Mem {
                                base: base_x,
                                offset: 0,
                            },
                        ]
                    }
                }
                2 => {
                    // W-register base
                    if is_load {
                        vec![
                            Operand::Reg(data.clone()),
                            Operand::Reg(data_name((n + 1) & 31, is_64)),
                            Operand::Mem {
                                base: format!("w{}", rn),
                                offset: 0,
                            },
                        ]
                    } else {
                        vec![
                            Operand::Reg(ws_name(0)),
                            Operand::Reg(data.clone()),
                            Operand::Reg(data_name((n + 1) & 31, is_64)),
                            Operand::Mem {
                                base: format!("w{}", rn),
                                offset: 0,
                            },
                        ]
                    }
                }
                3 => {
                    // XZR as base (Rn is Xn|SP, never XZR)
                    if is_load {
                        vec![
                            Operand::Reg(data.clone()),
                            Operand::Reg(data_name((n + 1) & 31, is_64)),
                            Operand::Mem {
                                base: "xzr".into(),
                                offset: 0,
                            },
                        ]
                    } else {
                        vec![
                            Operand::Reg(ws_name(0)),
                            Operand::Reg(data.clone()),
                            Operand::Reg(data_name((n + 1) & 31, is_64)),
                            Operand::Mem {
                                base: "xzr".into(),
                                offset: 0,
                            },
                        ]
                    }
                }
                4 => {
                    // SIMD/FP as Rt
                    if is_load {
                        vec![
                            Operand::Reg(format!("d{}", n)),
                            Operand::Reg(data),
                            Operand::Mem {
                                base: base_x,
                                offset: 0,
                            },
                        ]
                    } else {
                        vec![
                            Operand::Reg(ws_name(0)),
                            Operand::Reg(format!("d{}", n)),
                            Operand::Reg(data),
                            Operand::Mem {
                                base: base_x,
                                offset: 0,
                            },
                        ]
                    }
                }
                5 => {
                    // mixed X/W data pair
                    if is_load {
                        vec![
                            Operand::Reg(data_name(n, true)),
                            Operand::Reg(data_name((n + 1) & 31, false)),
                            Operand::Mem {
                                base: base_x,
                                offset: 0,
                            },
                        ]
                    } else {
                        vec![
                            Operand::Reg(ws_name(0)),
                            Operand::Reg(data_name(n, true)),
                            Operand::Reg(data_name((n + 1) & 31, false)),
                            Operand::Mem {
                                base: base_x,
                                offset: 0,
                            },
                        ]
                    }
                }
                6 => {
                    // X as STXP status (Ws must be W); for load, wsp as Rt
                    if is_load {
                        vec![
                            Operand::Reg("wsp".into()),
                            Operand::Reg(data),
                            Operand::Mem {
                                base: base_x,
                                offset: 0,
                            },
                        ]
                    } else {
                        vec![
                            Operand::Reg(format!("x{}", n.min(30))),
                            Operand::Reg(data.clone()),
                            Operand::Reg(data_name((n + 1) & 31, is_64)),
                            Operand::Mem {
                                base: base_x,
                                offset: 0,
                            },
                        ]
                    }
                }
                7 => {
                    // v-register as Rt
                    if is_load {
                        vec![
                            Operand::Reg(format!("v{}", n)),
                            Operand::Reg(data),
                            Operand::Mem {
                                base: base_x,
                                offset: 0,
                            },
                        ]
                    } else {
                        vec![
                            Operand::Reg(ws_name(0)),
                            Operand::Reg(format!("v{}", n)),
                            Operand::Reg(data),
                            Operand::Mem {
                                base: base_x,
                                offset: 0,
                            },
                        ]
                    }
                }
                8 => {
                    // WZR as base
                    if is_load {
                        vec![
                            Operand::Reg(data.clone()),
                            Operand::Reg(data_name((n + 1) & 31, is_64)),
                            Operand::Mem {
                                base: "wzr".into(),
                                offset: 0,
                            },
                        ]
                    } else {
                        vec![
                            Operand::Reg(ws_name(0)),
                            Operand::Reg(data.clone()),
                            Operand::Reg(data_name((n + 1) & 31, is_64)),
                            Operand::Mem {
                                base: "wzr".into(),
                                offset: 0,
                            },
                        ]
                    }
                }
                _ => {
                    // wsp as base
                    if is_load {
                        vec![
                            Operand::Reg(data.clone()),
                            Operand::Reg(data_name((n + 1) & 31, is_64)),
                            Operand::Mem {
                                base: "wsp".into(),
                                offset: 0,
                            },
                        ]
                    } else {
                        vec![
                            Operand::Reg(ws_name(0)),
                            Operand::Reg(data.clone()),
                            Operand::Reg(data_name((n + 1) & 31, is_64)),
                            Operand::Mem {
                                base: "wsp".into(),
                                offset: 0,
                            },
                        ]
                    }
                }
            };
            prop_assert!(
                encode_ldxp_stxp(&ops, is_load, acqrel).is_err(),
                "invalid Rt/base/width must Err (kind {}); llvm-mc rejects it; got {:?}",
                kind,
                encode_ldxp_stxp(&ops, is_load, acqrel)
            );
        }

        // Oracle: negative_error
        // Target: encoder.load_store.encode_ldxp_stxp
        #[test]
        fn encode_ldxp_stxp_neg_offset_nonmem(
            rt in reg_edge(),
            rt2 in reg_edge(),
            rn in 0u32..=30,
            ws in reg_edge(),
            is_load in any::<bool>(),
            acqrel in any::<bool>(),
            is_64 in any::<bool>(),
            offset in nonzero_offset(),
            shape in 0u32..=7,
        ) {
            let data_rt = data_name(rt, is_64);
            let data_rt2 = data_name(rt2, is_64);
            let base = rn_name(rn);
            let mem_slot = match shape {
                0 => Operand::Mem {
                    base: base.clone(),
                    offset,
                },
                1 => Operand::MemPreIndex {
                    base: base.clone(),
                    offset,
                },
                2 => Operand::MemPostIndex {
                    base: base.clone(),
                    offset,
                },
                3 => Operand::MemRegOffset {
                    base: base.clone(),
                    index: "x4".into(),
                    extend: None,
                    shift: None,
                },
                4 => Operand::Imm(offset),
                5 => Operand::Symbol("foo".into()),
                6 => Operand::Mem {
                    base: "foo".into(),
                    offset: 0,
                },
                _ => Operand::Mem {
                    base: "x32".into(),
                    offset: 0,
                },
            };
            let ops = if is_load {
                vec![
                    Operand::Reg(data_rt),
                    Operand::Reg(data_rt2),
                    mem_slot,
                ]
            } else {
                vec![
                    Operand::Reg(ws_name(ws)),
                    Operand::Reg(data_rt),
                    Operand::Reg(data_rt2),
                    mem_slot,
                ]
            };
            prop_assert!(
                encode_ldxp_stxp(&ops, is_load, acqrel).is_err(),
                "nonzero offset / non-Mem / invalid base name must Err (shape {}); got {:?}",
                shape,
                encode_ldxp_stxp(&ops, is_load, acqrel)
            );
        }

        // Oracle: negative_error (coverage sweep — arms the extra/offset properties never reach)
        // Target: encoder.load_store.encode_ldxp_stxp
        #[test]
        fn encode_ldxp_stxp_neg_too_short_nonmem_badname(
            rt in reg_edge(),
            rt2 in reg_edge(),
            rn in 0u32..=30,
            ws in reg_edge(),
            is_load in any::<bool>(),
            acqrel in any::<bool>(),
            is_64 in any::<bool>(),
            shape in 0u32..=6,
        ) {
            let data_rt = data_name(rt, is_64);
            let data_rt2 = data_name(rt2, is_64);
            let ops: Vec<Operand> = match shape {
                0 => vec![],
                1 => vec![Operand::Reg(data_rt.clone())],
                2 => vec![
                    Operand::Reg(data_rt.clone()),
                    Operand::Reg(data_rt2.clone()),
                ],
                3 => vec![
                    Operand::Imm(0),
                    Operand::Reg(data_rt2.clone()),
                    Operand::Mem {
                        base: rn_name(rn),
                        offset: 0,
                    },
                ],
                4 => {
                    let mut v = valid_ops(rt, rt2, rn, ws, is_load, is_64);
                    let last = v.len() - 1;
                    v[last] = Operand::Imm(0);
                    v
                }
                5 => {
                    let mut v = valid_ops(rt, rt2, rn, ws, is_load, is_64);
                    let last = v.len() - 1;
                    v[last] = Operand::MemPreIndex {
                        base: rn_name(rn),
                        offset: 0,
                    };
                    v
                }
                _ => {
                    let mut v = valid_ops(rt, rt2, rn, ws, is_load, is_64);
                    let last = v.len() - 1;
                    v[last] = Operand::Mem {
                        base: "foo".into(),
                        offset: 0,
                    };
                    v
                }
            };
            prop_assert!(
                encode_ldxp_stxp(&ops, is_load, acqrel).is_err(),
                "too-few / non-Reg / non-Mem / invalid base must Err (shape {}); got {:?}",
                shape,
                encode_ldxp_stxp(&ops, is_load, acqrel)
            );
        }
    }
}

#[cfg(test)]
mod encode_ldxr_stxr_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler
    // Evidence: src/backend/arm/assembler/README.md "accepts the same textual assembly that GCC's gas would consume";
    //   encoder/mod.rs:1-7 32-bit AArch64 words; encoder/mod.rs:348-353 ldxr/stxr/ldxrb/stxrb/ldxrh/stxrh dispatch;
    //   ARM ARM Load/Store Exclusive: size 001000 0 L 0 Rs o0 Rt2 Rn Rt;
    //   Rt is Wt/Xt (31=ZR); Rn is Xn|SP; Ws is Wt (31=WZR); offset {,#0}; o0=0; Rt2=11111.
    // Stronger considered:
    //   - State machine: rejected — encode_ldxr_stxr is a pure function with no lifecycle
    //   - Algebraic round-trip via in-tree decoder: rejected — no exclusive-single decoder
    //   - encode_ldaxr_stlxr / encode_ldxp_stxp / encode_ldar_stlr as differential siblings:
    //     rejected — acquire-release exclusive / exclusive-pair / ordered non-exclusive, different job
    // Weaker available: algebraic.invariant (ARM field unpack), algebraic.metamorphic (L / size),
    //   negative_error (arity / extra / SP / FP / W-base / XZR-base / X-Ws / offset / Ws-overlap)
    // Differential: candidate=encode_ldxr_stxr, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=load (Reg(Rt), Mem{Rn, 0}, is_load=true, forced_size)
    //           <-> `{ldxr|ldxrb|ldxrh} Rt, [Rn]`;
    //           store (Reg(Ws), Reg(Rt), Mem{Rn, 0}, is_load=false, forced_size)
    //           <-> `{stxr|stxrb|stxrh} Ws, Rt, [Rn]`

    use super::encode_ldxr_stxr;
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

    fn data_name(n: u32, is_64: bool) -> String {
        if is_64 {
            if n == 31 {
                "xzr".into()
            } else {
                format!("x{}", n)
            }
        } else if n == 31 {
            "wzr".into()
        } else {
            format!("w{}", n)
        }
    }

    fn rn_name(n: u32) -> String {
        if n == 31 {
            "sp".into()
        } else {
            format!("x{}", n)
        }
    }

    fn ws_name(n: u32) -> String {
        if n == 31 {
            "wzr".into()
        } else {
            format!("w{}", n)
        }
    }

    /// llvm-mc / ARM: STXR status must not also be a source (Rt or Xn).
    /// WZR vs SP (both encode as 31) is allowed: `stxr wzr, x0, [sp]`.
    fn stxr_ws_aliases_source(ws: u32, rt: u32, rn: u32) -> bool {
        ws == rt || (rn != 31 && ws == rn)
    }

    fn forced_size(variant: u32) -> Option<u32> {
        match variant {
            1 => Some(0b00),
            2 => Some(0b01),
            _ => None,
        }
    }

    fn mnemonic(is_load: bool, variant: u32) -> &'static str {
        match (is_load, variant) {
            (true, 1) => "ldxrb",
            (false, 1) => "stxrb",
            (true, 2) => "ldxrh",
            (false, 2) => "stxrh",
            (true, _) => "ldxr",
            (false, _) => "stxr",
        }
    }

    /// Byte/half exclusive forms take Wt, not Xt.
    fn data_is_64(variant: u32, is_64: bool) -> bool {
        variant == 0 && is_64
    }

    fn expected_size(variant: u32, is_64: bool) -> u32 {
        match variant {
            1 => 0b00,
            2 => 0b01,
            _ if is_64 => 0b11,
            _ => 0b10,
        }
    }

    fn load_ops(rt: u32, rn: u32, is_64: bool) -> Vec<Operand> {
        vec![
            Operand::Reg(data_name(rt, is_64)),
            Operand::Mem {
                base: rn_name(rn),
                offset: 0,
            },
        ]
    }

    fn store_ops(ws: u32, rt: u32, rn: u32, is_64: bool) -> Vec<Operand> {
        vec![
            Operand::Reg(ws_name(ws)),
            Operand::Reg(data_name(rt, is_64)),
            Operand::Mem {
                base: rn_name(rn),
                offset: 0,
            },
        ]
    }

    fn valid_ops(rt: u32, rn: u32, ws: u32, is_load: bool, is_64: bool) -> Vec<Operand> {
        if is_load {
            load_ops(rt, rn, is_64)
        } else {
            store_ops(ws, rt, rn, is_64)
        }
    }

    fn sut_word(ops: &[Operand], is_load: bool, forced: Option<u32>) -> Result<u32, String> {
        match encode_ldxr_stxr(ops, is_load, forced)? {
            EncodeResult::Word(w) => Ok(w),
            other => Err(format!("expected Word, got {:?}", other)),
        }
    }

    /// Unpack exclusive-single fields per ARM ARM (not a copy of the SUT packer).
    fn unpack_ldxr_stxr(word: u32) -> (u32, u32, u32, u32, u32, u32, u32, u32) {
        let size = (word >> 30) & 0b11;
        let l = (word >> 22) & 1;
        let o1 = (word >> 21) & 1;
        let rs = (word >> 16) & 0x1f;
        let o0 = (word >> 15) & 1;
        let rt2 = (word >> 10) & 0x1f;
        let rn = (word >> 5) & 0x1f;
        let rt = word & 0x1f;
        (size, l, o1, rs, o0, rt2, rn, rt)
    }

    fn fixed_ldxr_stxr_bits(word: u32) -> bool {
        ((word >> 24) & 0x3f) == 0b001000
            && ((word >> 23) & 1) == 0
            && ((word >> 21) & 1) == 0
            && ((word >> 15) & 1) == 0
            && ((word >> 10) & 0x1f) == 0b11111
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

    fn reg_edge() -> impl Strategy<Value = u32> {
        prop_oneof![Just(0u32), Just(1u32), Just(30u32), Just(31u32), 0u32..=31]
    }

    fn extra_operand() -> impl Strategy<Value = Operand> {
        prop_oneof![
            Just(Operand::Reg("x2".into())),
            Just(Operand::Imm(0)),
            Just(Operand::Imm(1)),
            Just(Operand::Symbol("foo".into())),
            Just(Operand::Mem {
                base: "x3".into(),
                offset: 0
            }),
        ]
    }

    fn nonzero_offset() -> impl Strategy<Value = i64> {
        prop_oneof![
            Just(-1i64),
            Just(1i64),
            Just(8i64),
            Just(-8i64),
            Just(256i64),
            Just(i64::MIN),
            Just(i64::MAX),
            1i64..=4096,
            -4096i64..=-1,
        ]
    }

    fn fp_name(n: u32, kind: u32) -> String {
        let p = match kind % 6 {
            0 => "d",
            1 => "s",
            2 => "q",
            3 => "v",
            4 => "h",
            _ => "b",
        };
        format!("{}{}", p, n % 32)
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_ldxr_stxr_kat_llvm_mc_ldxr_x0_x1() {
        let want = 0xc85f7c20u32;
        let mc = llvm_mc_word("ldxr x0, [x1]").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = load_ops(0, 1, true);
        let sut = sut_word(&ops, true, None).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_ldxr_stxr_kat_llvm_mc_ldxr_w0_x1() {
        let want = 0x885f7c20u32;
        let mc = llvm_mc_word("ldxr w0, [x1]").expect("llvm-mc KAT w");
        assert_eq!(mc, want, "llvm-mc KAT w mapping broken");
        let ops = load_ops(0, 1, false);
        let sut = sut_word(&ops, true, None).expect("SUT KAT w");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_ldxr_stxr_kat_llvm_mc_ldxrb_w0_x1() {
        let want = 0x085f7c20u32;
        let mc = llvm_mc_word("ldxrb w0, [x1]").expect("llvm-mc KAT b");
        assert_eq!(mc, want, "llvm-mc KAT b mapping broken");
        let ops = load_ops(0, 1, false);
        let sut = sut_word(&ops, true, Some(0b00)).expect("SUT KAT b");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_ldxr_stxr_kat_llvm_mc_ldxrh_w0_x1() {
        let want = 0x485f7c20u32;
        let mc = llvm_mc_word("ldxrh w0, [x1]").expect("llvm-mc KAT h");
        assert_eq!(mc, want, "llvm-mc KAT h mapping broken");
        let ops = load_ops(0, 1, false);
        let sut = sut_word(&ops, true, Some(0b01)).expect("SUT KAT h");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_ldxr_stxr_kat_llvm_mc_stxr_w0_x1_x2() {
        let want = 0xc8007c41u32;
        let mc = llvm_mc_word("stxr w0, x1, [x2]").expect("llvm-mc KAT stxr");
        assert_eq!(mc, want, "llvm-mc KAT stxr mapping broken");
        let ops = store_ops(0, 1, 2, true);
        let sut = sut_word(&ops, false, None).expect("SUT KAT stxr");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_ldxr_stxr_kat_llvm_mc_stxr_w0_w1_x2() {
        let want = 0x88007c41u32;
        let mc = llvm_mc_word("stxr w0, w1, [x2]").expect("llvm-mc KAT stxr w");
        assert_eq!(mc, want, "llvm-mc KAT stxr w mapping broken");
        let ops = store_ops(0, 1, 2, false);
        let sut = sut_word(&ops, false, None).expect("SUT KAT stxr w");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_ldxr_stxr_kat_llvm_mc_ldxr_sp() {
        let want = 0xc85f7fe0u32;
        let mc = llvm_mc_word("ldxr x0, [sp]").expect("llvm-mc KAT sp");
        assert_eq!(mc, want, "llvm-mc KAT sp mapping broken");
        let ops = load_ops(0, 31, true);
        let sut = sut_word(&ops, true, None).expect("SUT KAT sp");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_ldxr_stxr_kat_llvm_mc_ldxr_lr() {
        let want = 0xc85f7c5eu32;
        let mc = llvm_mc_word("ldxr lr, [x2]").expect("llvm-mc KAT lr");
        assert_eq!(mc, want, "llvm-mc KAT lr mapping broken");
        let ops = vec![
            Operand::Reg("lr".into()),
            Operand::Mem {
                base: "x2".into(),
                offset: 0,
            },
        ];
        let sut = sut_word(&ops, true, None).expect("SUT KAT lr");
        assert_eq!(sut, want);
    }

    #[test]
    fn test_encode_ldxr_stxr_regression_extra_operand() {
        let mut ops = store_ops(0, 1, 2, true);
        ops.push(Operand::Reg("x2".into()));
        assert!(
            encode_ldxr_stxr(&ops, false, None).is_err(),
            "stxr w0, x1, [x2], x2 must Err; llvm-mc rejects a 4th operand"
        );
    }

    #[test]
    fn test_encode_ldxr_stxr_regression_sp_as_rt() {
        let ops = load_ops_named("sp", "x0");
        assert!(
            encode_ldxr_stxr(&ops, true, None).is_err(),
            "ldxr sp, [x0] must Err; llvm-mc rejects SP as Rt"
        );
    }

    #[test]
    fn test_encode_ldxr_stxr_regression_w_base() {
        let ops = load_ops_named("x0", "w1");
        assert!(
            encode_ldxr_stxr(&ops, true, None).is_err(),
            "ldxr x0, [w1] must Err; llvm-mc rejects a W register as base"
        );
    }

    #[test]
    fn test_encode_ldxr_stxr_regression_xzr_as_base() {
        let ops = load_ops_named("x0", "xzr");
        assert!(
            encode_ldxr_stxr(&ops, true, None).is_err(),
            "ldxr x0, [xzr] must Err; llvm-mc rejects XZR as base"
        );
    }

    #[test]
    fn test_encode_ldxr_stxr_regression_fp_as_rt() {
        let ops = load_ops_named("d0", "x1");
        assert!(
            encode_ldxr_stxr(&ops, true, None).is_err(),
            "ldxr d0, [x1] must Err; llvm-mc rejects SIMD/FP as Rt"
        );
    }

    #[test]
    fn test_encode_ldxr_stxr_regression_x_as_ws() {
        let ops = vec![
            Operand::Reg("x0".into()),
            Operand::Reg("x1".into()),
            Operand::Mem {
                base: "x2".into(),
                offset: 0,
            },
        ];
        assert!(
            encode_ldxr_stxr(&ops, false, None).is_err(),
            "stxr x0, x1, [x2] must Err; llvm-mc rejects X as STXR status"
        );
    }

    #[test]
    fn test_encode_ldxr_stxr_regression_x_data_byte() {
        let ops = load_ops_named("x0", "x1");
        assert!(
            encode_ldxr_stxr(&ops, true, Some(0b00)).is_err(),
            "ldxrb x0, [x1] must Err; llvm-mc requires Wt for byte exclusive"
        );
    }

    #[test]
    fn test_encode_ldxr_stxr_regression_nonzero_offset() {
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Mem {
                base: "x1".into(),
                offset: -1,
            },
        ];
        assert!(
            encode_ldxr_stxr(&ops, true, None).is_err(),
            "ldxr x0, [x1, #-1] must Err; llvm-mc: index must be absent or #0"
        );
    }

    #[test]
    fn test_encode_ldxr_stxr_regression_ws_overlap() {
        let ops = store_ops(0, 0, 1, false);
        assert!(
            encode_ldxr_stxr(&ops, false, None).is_err(),
            "stxr w0, w0, [x1] must Err; llvm-mc: unpredictable STXR, status is also a source"
        );
    }

    proptest! {
        #![proptest_config(cfg())]

        // Oracle: differential
        // Target: encoder.load_store.encode_ldxr_stxr
        #[test]
        fn encode_ldxr_stxr_diff_llvm_mc(
            rt in reg_edge(),
            rn in reg_edge(),
            ws in reg_edge(),
            is_load in any::<bool>(),
            variant in 0u32..=2,
            is_64 in any::<bool>(),
        ) {
            let is_64 = data_is_64(variant, is_64);
            if !is_load {
                prop_assume!(!stxr_ws_aliases_source(ws, rt, rn));
            }
            let ops = valid_ops(rt, rn, ws, is_load, is_64);
            let mnem = mnemonic(is_load, variant);
            let asm = if is_load {
                format!("{} {}, [{}]", mnem, data_name(rt, is_64), rn_name(rn))
            } else {
                format!(
                    "{} {}, {}, [{}]",
                    mnem,
                    ws_name(ws),
                    data_name(rt, is_64),
                    rn_name(rn)
                )
            };
            let sut = sut_word(&ops, is_load, forced_size(variant))
                .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
            let mc = llvm_mc_word(&asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
            prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
        }

        // Oracle: algebraic.invariant
        // Target: encoder.load_store.encode_ldxr_stxr
        #[test]
        fn encode_ldxr_stxr_arm_fields(
            rt in reg_edge(),
            rn in reg_edge(),
            ws in reg_edge(),
            is_load in any::<bool>(),
            variant in 0u32..=2,
            is_64 in any::<bool>(),
        ) {
            let is_64 = data_is_64(variant, is_64);
            let ops = valid_ops(rt, rn, ws, is_load, is_64);
            let word = sut_word(&ops, is_load, forced_size(variant))
                .unwrap_or_else(|e| panic!("SUT rejected valid exclusive: {}", e));
            prop_assert!(
                fixed_ldxr_stxr_bits(word),
                "fixed exclusive-single bits wrong: {:#010x}",
                word
            );
            let (size, l, o1, rs, o0, rt2, got_rn, got_rt) = unpack_ldxr_stxr(word);
            prop_assert_eq!(size, expected_size(variant, is_64), "size");
            prop_assert_eq!(l, if is_load { 1 } else { 0 }, "L");
            prop_assert_eq!(o1, 0, "o1 must be 0 (not pair)");
            prop_assert_eq!(rs, if is_load { 31 } else { ws }, "Rs");
            prop_assert_eq!(o0, 0, "o0 must be 0 (not acquire/release)");
            prop_assert_eq!(rt2, 0b11111, "Rt2");
            prop_assert_eq!(got_rn, rn, "Rn");
            prop_assert_eq!(got_rt, rt, "Rt");
        }

        // Oracle: algebraic.metamorphic
        // Target: encoder.load_store.encode_ldxr_stxr
        #[test]
        fn encode_ldxr_stxr_metamorphic_l_size(
            rt in 0u32..=30,
            rn in reg_edge(),
            ws in reg_edge(),
        ) {
            let load = sut_word(&load_ops(rt, rn, true), true, None)
                .unwrap_or_else(|e| panic!("load: {}", e));
            let store = sut_word(&store_ops(31, rt, rn, true), false, None)
                .unwrap_or_else(|e| panic!("store wzr: {}", e));
            prop_assert_eq!(
                load ^ store,
                1u32 << 22,
                "L bit: ldxr XOR stxr(wzr) must be 1<<22; load={:#010x} store={:#010x}",
                load,
                store
            );

            let xw_x = sut_word(&load_ops(rt, rn, true), true, None)
                .unwrap_or_else(|e| panic!("X: {}", e));
            let xw_w = sut_word(&load_ops(rt, rn, false), true, None)
                .unwrap_or_else(|e| panic!("W: {}", e));
            prop_assert_eq!(
                xw_x ^ xw_w,
                1u32 << 30,
                "size X vs W must XOR 1<<30; x={:#010x} w={:#010x}",
                xw_x,
                xw_w
            );

            if !stxr_ws_aliases_source(ws, rt, rn) {
                let b = sut_word(&store_ops(ws, rt, rn, false), false, Some(0b00))
                    .unwrap_or_else(|e| panic!("byte: {}", e));
                let h = sut_word(&store_ops(ws, rt, rn, false), false, Some(0b01))
                    .unwrap_or_else(|e| panic!("half: {}", e));
                prop_assert_eq!(
                    b ^ h,
                    1u32 << 30,
                    "size byte vs half must XOR 1<<30; b={:#010x} h={:#010x}",
                    b,
                    h
                );
            }
        }

        // Oracle: negative_error
        // Target: encoder.load_store.encode_ldxr_stxr
        #[test]
        fn encode_ldxr_stxr_neg_extra_operand(
            rt in reg_edge(),
            rn in reg_edge(),
            ws in reg_edge(),
            is_load in any::<bool>(),
            variant in 0u32..=2,
            is_64 in any::<bool>(),
            extra in extra_operand(),
        ) {
            let is_64 = data_is_64(variant, is_64);
            let mut ops = valid_ops(rt, rn, ws, is_load, is_64);
            ops.push(extra);
            prop_assert!(
                encode_ldxr_stxr(&ops, is_load, forced_size(variant)).is_err(),
                "extra operand must Err (llvm-mc rejects a trailing operand); got {:?}",
                encode_ldxr_stxr(&ops, is_load, forced_size(variant))
            );
        }

        // Oracle: negative_error
        // Target: encoder.load_store.encode_ldxr_stxr
        #[test]
        fn encode_ldxr_stxr_neg_invalid_regs(
            kind in 0u32..=5,
            n in reg_edge(),
            other in reg_edge(),
            fp_kind in 0u32..=5,
        ) {
            let (ops, is_load, forced, why) = match kind {
                0 => {
                    // SP as Rt — register 31 is ZR, not SP
                    (
                        load_ops_named("sp", &rn_name(n)),
                        true,
                        None,
                        "ldxr sp, [Xn] must Err; llvm-mc rejects SP as Rt",
                    )
                }
                1 => {
                    // W register as base
                    let base = if n == 31 {
                        "wsp".to_string()
                    } else {
                        format!("w{}", n)
                    };
                    (
                        load_ops_named(&data_name(other, true), &base),
                        true,
                        None,
                        "ldxr Xt, [Wn] must Err; llvm-mc rejects a W register as base",
                    )
                }
                2 => {
                    // XZR as base (encoding 31 is SP in the address register)
                    (
                        load_ops_named(&data_name(other, true), "xzr"),
                        true,
                        None,
                        "ldxr Xt, [xzr] must Err; llvm-mc rejects XZR as base",
                    )
                }
                3 => {
                    // FP/SIMD as Rt
                    (
                        load_ops_named(&fp_name(n, fp_kind), &rn_name(other)),
                        true,
                        None,
                        "ldxr <FP>, [Xn] must Err; llvm-mc rejects SIMD/FP as Rt",
                    )
                }
                4 => {
                    // X register as STXR status (must be Ws)
                    (
                        vec![
                            Operand::Reg(data_name(n, true)),
                            Operand::Reg(data_name(other, true)),
                            Operand::Mem {
                                base: rn_name((other + 1) % 31),
                                offset: 0,
                            },
                        ],
                        false,
                        None,
                        "stxr Xt, Xt, [Xn] must Err; llvm-mc rejects X as STXR status",
                    )
                }
                _ => {
                    // X data register on byte/half exclusive
                    (
                        load_ops_named(&data_name(n, true), &rn_name(other)),
                        true,
                        Some(0b00),
                        "ldxrb Xt, [Xn] must Err; llvm-mc requires Wt for byte/half",
                    )
                }
            };
            prop_assert!(
                encode_ldxr_stxr(&ops, is_load, forced).is_err(),
                "{}; got {:?}",
                why,
                encode_ldxr_stxr(&ops, is_load, forced)
            );
        }

        // Oracle: negative_error
        // Target: encoder.load_store.encode_ldxr_stxr
        #[test]
        fn encode_ldxr_stxr_neg_arity_shape(
            is_load in any::<bool>(),
            shape in 0u32..=8,
            rt in reg_edge(),
            offset in nonzero_offset(),
        ) {
            let rt_n = data_name(rt, true);
            let ops: Vec<Operand> = match shape {
                0 => vec![],
                1 => vec![Operand::Reg(rt_n)],
                2 => vec![Operand::Reg(rt_n.clone()), Operand::Imm(0)],
                3 => vec![Operand::Reg(rt_n.clone()), Operand::Symbol("foo".into())],
                4 => vec![
                    Operand::Reg(rt_n.clone()),
                    Operand::MemPreIndex {
                        base: "x0".into(),
                        offset: 0,
                    },
                ],
                5 => vec![
                    Operand::Reg(rt_n.clone()),
                    Operand::MemPostIndex {
                        base: "x0".into(),
                        offset: 0,
                    },
                ],
                6 => {
                    if is_load {
                        vec![
                            Operand::Reg(rt_n),
                            Operand::Mem {
                                base: "foo".into(),
                                offset: 0,
                            },
                        ]
                    } else {
                        vec![
                            Operand::Reg(ws_name(0)),
                            Operand::Reg(rt_n),
                            Operand::Mem {
                                base: "foo".into(),
                                offset: 0,
                            },
                        ]
                    }
                }
                7 => {
                    if is_load {
                        vec![
                            Operand::Reg(rt_n),
                            Operand::Mem {
                                base: "x32".into(),
                                offset: 0,
                            },
                        ]
                    } else {
                        vec![
                            Operand::Reg(ws_name(0)),
                            Operand::Reg(rt_n),
                            Operand::Mem {
                                base: "x32".into(),
                                offset: 0,
                            },
                        ]
                    }
                }
                _ => {
                    if is_load {
                        vec![
                            Operand::Reg(rt_n),
                            Operand::Mem {
                                base: "x0".into(),
                                offset,
                            },
                        ]
                    } else {
                        vec![
                            Operand::Reg(ws_name(1)),
                            Operand::Reg(data_name(0, true)),
                            Operand::Mem {
                                base: "x2".into(),
                                offset,
                            },
                        ]
                    }
                }
            };
            prop_assert!(
                encode_ldxr_stxr(&ops, is_load, None).is_err(),
                "too-few / non-Mem / pre-post / invalid base / nonzero offset must Err (shape {}); got {:?}",
                shape,
                encode_ldxr_stxr(&ops, is_load, None)
            );
        }

        // Oracle: negative_error
        // Target: encoder.load_store.encode_ldxr_stxr
        #[test]
        fn encode_ldxr_stxr_neg_ws_overlap(
            rt in reg_edge(),
            rn in reg_edge(),
            variant in 0u32..=2,
            is_64 in any::<bool>(),
            overlap_rt in any::<bool>(),
        ) {
            let is_64 = data_is_64(variant, is_64);
            // Force a CONSTRAINED UNPREDICTABLE overlap: Ws==Rt, or Ws==Rn (Rn not SP).
            let ws = if overlap_rt {
                rt
            } else if rn != 31 {
                rn
            } else {
                rt // rn==31 cannot overlap via Xn; fall back to Rt alias
            };
            prop_assume!(stxr_ws_aliases_source(ws, rt, rn));
            let ops = store_ops(ws, rt, rn, is_64);
            prop_assert!(
                encode_ldxr_stxr(&ops, false, forced_size(variant)).is_err(),
                "stxr with Ws aliasing Rt/Xn must Err (llvm-mc: status is also a source); got {:?}",
                encode_ldxr_stxr(&ops, false, forced_size(variant))
            );
        }
    }

    fn load_ops_named(rt: &str, rn: &str) -> Vec<Operand> {
        vec![
            Operand::Reg(rt.to_string()),
            Operand::Mem {
                base: rn.to_string(),
                offset: 0,
            },
        ]
    }
}
