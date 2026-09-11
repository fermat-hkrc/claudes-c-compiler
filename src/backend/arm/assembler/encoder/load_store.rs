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
