use super::*;

// ── Pseudo-instruction encoders ──────────────────────────────────────

pub(crate) fn encode_li(operands: &[Operand]) -> Result<EncodeResult, String> {
    let rd = get_reg(operands, 0)?;
    let imm = get_imm(operands, 1)?;

    let words = encode_li_immediate(rd, imm);
    if words.len() == 1 {
        Ok(EncodeResult::Word(words[0]))
    } else {
        Ok(EncodeResult::Words(words))
    }
}

/// Sign-extend a value from `bits` width to i64.
pub(crate) fn sign_extend_li(val: i64, bits: u32) -> i64 {
    let shift = 64 - bits;
    (val << shift) >> shift
}

/// Emit lui + addiw (or just addi) for a 32-bit signed value into register `rd`.
///
/// On RV64, the `li` pseudo-instruction uses `lui + addiw` (not `lui + addi`)
/// to ensure proper 32-bit sign extension. GAS always uses `addiw` after `lui`
/// for `li` on RV64. For small values (fits in 12 bits), `addi rd, x0, imm`
/// is sufficient since the result is the same.
pub(crate) fn encode_li_32bit(rd: u32, imm: i32) -> Vec<u32> {
    if (-2048..=2047).contains(&imm) {
        return vec![encode_i(OP_OP_IMM, rd, 0, 0, imm)]; // addi rd, x0, imm
    }
    let lo = (imm << 20) >> 20; // sign-extend low 12 bits
    let hi = ((imm as u32).wrapping_add(if lo < 0 { 0x1000 } else { 0 })) & 0xFFFFF000;
    let mut words = vec![encode_u(OP_LUI, rd, hi)];
    if lo != 0 {
        // Use addiw (OP_OP_IMM_32) to match GAS behavior on RV64.
        // lui sign-extends the 20-bit immediate to 64 bits, and addiw
        // ensures the final 32-bit result is properly sign-extended.
        words.push(encode_i(OP_OP_IMM_32, rd, 0, rd, lo)); // addiw rd, rd, lo
    }
    words
}

/// Encode `li` pseudo-instruction for an arbitrary 64-bit immediate.
///
/// Decomposes the value into a sequence of lui/addiw/slli/addi instructions.
/// For 64-bit values that don't fit in 32 bits, finds optimal shift amounts
/// such that the value = ((upper << shift1) + lo1) << shift2 + lo2 ...
/// where upper fits in 32 bits and each lo fits in 12 signed bits.
pub(crate) fn encode_li_immediate(rd: u32, imm: i64) -> Vec<u32> {
    // Case 1: fits in 12 bits (addi rd, x0, imm)
    if (-2048..=2047).contains(&imm) {
        return vec![encode_i(OP_OP_IMM, rd, 0, 0, imm as i32)];
    }

    // Case 2: fits in 32 bits (lui + addi)
    if (-0x80000000..=0x7FFFFFFF).contains(&imm) {
        return encode_li_32bit(rd, imm as i32);
    }

    // Case 3: 64-bit — try single shift: imm = (upper << shift) + lo12
    let lo12 = sign_extend_li(imm & 0xFFF, 12);
    let mut best: Option<Vec<u32>> = None;

    for shift in 12..45 {
        let remainder = imm.wrapping_sub(lo12);
        if remainder & ((1i64 << shift) - 1) != 0 {
            continue;
        }
        let upper = remainder >> shift;
        if !(-0x80000000..=0x7FFFFFFF).contains(&upper) {
            continue;
        }

        let mut words = encode_li_32bit(rd, upper as i32);
        // Convert addi to addiw after lui for proper 64-bit sign extension
        if words.len() == 2 {
            let first_opcode = words[0] & 0x7F;
            let second_opcode = words[1] & 0x7F;
            if first_opcode == OP_LUI && second_opcode == OP_OP_IMM {
                words[1] = (words[1] & !0x7F) | OP_OP_IMM_32;
            }
        }
        words.push(encode_i(OP_OP_IMM, rd, 0b001, rd, shift)); // slli
        if lo12 != 0 {
            words.push(encode_i(OP_OP_IMM, rd, 0, rd, lo12 as i32)); // addi
        }

        if best.is_none() || words.len() < best.as_ref().unwrap().len() {
            best = Some(words);
        }
    }

    if let Some(words) = best {
        return words;
    }

    // Case 4: two-level shift — imm = ((A << shift1) + lo_b) << shift2 + lo_c
    for shift2 in 12..33 {
        let remainder_c = imm.wrapping_sub(lo12);
        if remainder_c & ((1i64 << shift2) - 1) != 0 {
            continue;
        }
        let inner = remainder_c >> shift2;
        let lo12_b = sign_extend_li(inner & 0xFFF, 12);

        for shift1 in 12..33 {
            let remainder_b = inner.wrapping_sub(lo12_b);
            if remainder_b & ((1i64 << shift1) - 1) != 0 {
                continue;
            }
            let upper = remainder_b >> shift1;
            if !(-0x80000000..=0x7FFFFFFF).contains(&upper) {
                continue;
            }

            let mut words = encode_li_32bit(rd, upper as i32);
            if words.len() == 2 {
                let first_opcode = words[0] & 0x7F;
                let second_opcode = words[1] & 0x7F;
                if first_opcode == OP_LUI && second_opcode == OP_OP_IMM {
                    words[1] = (words[1] & !0x7F) | OP_OP_IMM_32;
                }
            }
            words.push(encode_i(OP_OP_IMM, rd, 0b001, rd, shift1)); // slli
            if lo12_b != 0 {
                words.push(encode_i(OP_OP_IMM, rd, 0, rd, lo12_b as i32)); // addi
            }
            words.push(encode_i(OP_OP_IMM, rd, 0b001, rd, shift2)); // slli
            if lo12 != 0 {
                words.push(encode_i(OP_OP_IMM, rd, 0, rd, lo12 as i32)); // addi
            }

            if best.is_none() || words.len() < best.as_ref().unwrap().len() {
                best = Some(words);
            }
        }
    }

    if let Some(words) = best {
        return words;
    }

    // Case 5: three-level shift (needed for dense bit patterns across all 64 bits)
    for shift3 in 12..23 {
        let rem_c = imm.wrapping_sub(lo12);
        if rem_c & ((1i64 << shift3) - 1) != 0 {
            continue;
        }
        let v2 = rem_c >> shift3;
        let lo12_b = sign_extend_li(v2 & 0xFFF, 12);

        for shift2 in 12..23 {
            let rem_b = v2.wrapping_sub(lo12_b);
            if rem_b & ((1i64 << shift2) - 1) != 0 {
                continue;
            }
            let v1 = rem_b >> shift2;
            let lo12_a = sign_extend_li(v1 & 0xFFF, 12);

            for shift1 in 12..23 {
                let rem_a = v1.wrapping_sub(lo12_a);
                if rem_a & ((1i64 << shift1) - 1) != 0 {
                    continue;
                }
                let upper = rem_a >> shift1;
                if !(-0x80000000..=0x7FFFFFFF).contains(&upper) {
                    continue;
                }

                let mut words = encode_li_32bit(rd, upper as i32);
                if words.len() == 2 {
                    let first_opcode = words[0] & 0x7F;
                    let second_opcode = words[1] & 0x7F;
                    if first_opcode == OP_LUI && second_opcode == OP_OP_IMM {
                        words[1] = (words[1] & !0x7F) | OP_OP_IMM_32;
                    }
                }
                words.push(encode_i(OP_OP_IMM, rd, 0b001, rd, shift1));
                if lo12_a != 0 {
                    words.push(encode_i(OP_OP_IMM, rd, 0, rd, lo12_a as i32));
                }
                words.push(encode_i(OP_OP_IMM, rd, 0b001, rd, shift2));
                if lo12_b != 0 {
                    words.push(encode_i(OP_OP_IMM, rd, 0, rd, lo12_b as i32));
                }
                words.push(encode_i(OP_OP_IMM, rd, 0b001, rd, shift3));
                if lo12 != 0 {
                    words.push(encode_i(OP_OP_IMM, rd, 0, rd, lo12 as i32));
                }

                if best.is_none() || words.len() < best.as_ref().unwrap().len() {
                    best = Some(words);
                }
            }
        }
    }

    if let Some(words) = best {
        return words;
    }

    // Fallback: lui + addiw + slli 32, then add lower bits via addi chain
    eprintln!("warning: li fallback for 0x{:x}", imm as u64);
    let upper = (imm >> 32) as i32;
    let mut words = encode_li_32bit(rd, upper);
    if words.len() == 2 {
        let first_opcode = words[0] & 0x7F;
        let second_opcode = words[1] & 0x7F;
        if first_opcode == OP_LUI && second_opcode == OP_OP_IMM {
            words[1] = (words[1] & !0x7F) | OP_OP_IMM_32;
        }
    }
    words.push(encode_i(OP_OP_IMM, rd, 0b001, rd, 32)); // slli rd, rd, 32
    let mut remaining = imm as i32 as i64;
    while remaining != 0 {
        let chunk = remaining.clamp(-2048, 2047);
        words.push(encode_i(OP_OP_IMM, rd, 0, rd, chunk as i32));
        remaining -= chunk;
    }
    words
}

pub(crate) fn encode_mv(operands: &[Operand]) -> Result<EncodeResult, String> {
    let rd = get_reg(operands, 0)?;
    let rs = get_reg(operands, 1)?;
    // Use `add rd, x0, rs` instead of `addi rd, rs, 0` so the instruction
    // is eligible for RV64C compression to C.MV (which requires the ADD form).
    Ok(EncodeResult::Word(encode_r(OP_OP, rd, 0b000, 0, rs, 0b0000000))) // add rd, x0, rs
}

pub(crate) fn encode_not(operands: &[Operand]) -> Result<EncodeResult, String> {
    let rd = get_reg(operands, 0)?;
    let rs1 = get_reg(operands, 1)?;
    Ok(EncodeResult::Word(encode_i(OP_OP_IMM, rd, 0b100, rs1, -1))) // xori rd, rs1, -1
}

pub(crate) fn encode_neg(operands: &[Operand]) -> Result<EncodeResult, String> {
    let rd = get_reg(operands, 0)?;
    let rs2 = get_reg(operands, 1)?;
    Ok(EncodeResult::Word(encode_r(OP_OP, rd, 0b000, 0, rs2, 0b0100000))) // sub rd, x0, rs2
}

pub(crate) fn encode_negw(operands: &[Operand]) -> Result<EncodeResult, String> {
    let rd = get_reg(operands, 0)?;
    let rs2 = get_reg(operands, 1)?;
    Ok(EncodeResult::Word(encode_r(OP_OP_32, rd, 0b000, 0, rs2, 0b0100000)))
}

pub(crate) fn encode_sext_w(operands: &[Operand]) -> Result<EncodeResult, String> {
    let rd = get_reg(operands, 0)?;
    let rs1 = get_reg(operands, 1)?;
    Ok(EncodeResult::Word(encode_i(OP_OP_IMM_32, rd, 0, rs1, 0))) // addiw rd, rs1, 0
}

pub(crate) fn encode_seqz(operands: &[Operand]) -> Result<EncodeResult, String> {
    let rd = get_reg(operands, 0)?;
    let rs1 = get_reg(operands, 1)?;
    Ok(EncodeResult::Word(encode_i(OP_OP_IMM, rd, 0b011, rs1, 1))) // sltiu rd, rs1, 1
}

pub(crate) fn encode_snez(operands: &[Operand]) -> Result<EncodeResult, String> {
    let rd = get_reg(operands, 0)?;
    let rs2 = get_reg(operands, 1)?;
    Ok(EncodeResult::Word(encode_r(OP_OP, rd, 0b011, 0, rs2, 0b0000000))) // sltu rd, x0, rs2
}

pub(crate) fn encode_sltz(operands: &[Operand]) -> Result<EncodeResult, String> {
    let rd = get_reg(operands, 0)?;
    let rs1 = get_reg(operands, 1)?;
    Ok(EncodeResult::Word(encode_r(OP_OP, rd, 0b010, rs1, 0, 0b0000000))) // slt rd, rs1, x0
}

pub(crate) fn encode_sgtz(operands: &[Operand]) -> Result<EncodeResult, String> {
    let rd = get_reg(operands, 0)?;
    let rs2 = get_reg(operands, 1)?;
    Ok(EncodeResult::Word(encode_r(OP_OP, rd, 0b010, 0, rs2, 0b0000000))) // slt rd, x0, rs2
}

// Branch pseudo-instructions
pub(crate) fn encode_beqz(operands: &[Operand]) -> Result<EncodeResult, String> {
    let rs1 = get_reg(operands, 0)?;
    let label = get_branch_target(operands, 1)?;
    Ok(EncodeResult::WordWithReloc {
        word: encode_b(OP_BRANCH, 0b000, rs1, 0, 0),
        reloc: Relocation { reloc_type: RelocType::Branch, symbol: label, addend: 0 },
    })
}

pub(crate) fn encode_bnez(operands: &[Operand]) -> Result<EncodeResult, String> {
    let rs1 = get_reg(operands, 0)?;
    let label = get_branch_target(operands, 1)?;
    Ok(EncodeResult::WordWithReloc {
        word: encode_b(OP_BRANCH, 0b001, rs1, 0, 0),
        reloc: Relocation { reloc_type: RelocType::Branch, symbol: label, addend: 0 },
    })
}

pub(crate) fn encode_blez(operands: &[Operand]) -> Result<EncodeResult, String> {
    let rs2 = get_reg(operands, 0)?;
    let label = get_branch_target(operands, 1)?;
    Ok(EncodeResult::WordWithReloc {
        word: encode_b(OP_BRANCH, 0b101, 0, rs2, 0), // bge x0, rs
        reloc: Relocation { reloc_type: RelocType::Branch, symbol: label, addend: 0 },
    })
}

pub(crate) fn encode_bgez(operands: &[Operand]) -> Result<EncodeResult, String> {
    let rs1 = get_reg(operands, 0)?;
    let label = get_branch_target(operands, 1)?;
    Ok(EncodeResult::WordWithReloc {
        word: encode_b(OP_BRANCH, 0b101, rs1, 0, 0), // bge rs, x0
        reloc: Relocation { reloc_type: RelocType::Branch, symbol: label, addend: 0 },
    })
}

pub(crate) fn encode_bltz(operands: &[Operand]) -> Result<EncodeResult, String> {
    let rs1 = get_reg(operands, 0)?;
    let label = get_branch_target(operands, 1)?;
    Ok(EncodeResult::WordWithReloc {
        word: encode_b(OP_BRANCH, 0b100, rs1, 0, 0), // blt rs, x0
        reloc: Relocation { reloc_type: RelocType::Branch, symbol: label, addend: 0 },
    })
}

pub(crate) fn encode_bgtz(operands: &[Operand]) -> Result<EncodeResult, String> {
    let rs2 = get_reg(operands, 0)?;
    let label = get_branch_target(operands, 1)?;
    Ok(EncodeResult::WordWithReloc {
        word: encode_b(OP_BRANCH, 0b100, 0, rs2, 0), // blt x0, rs
        reloc: Relocation { reloc_type: RelocType::Branch, symbol: label, addend: 0 },
    })
}

pub(crate) fn encode_bgt(operands: &[Operand]) -> Result<EncodeResult, String> {
    let rs1 = get_reg(operands, 0)?;
    let rs2 = get_reg(operands, 1)?;
    let label = get_branch_target(operands, 2)?;
    Ok(EncodeResult::WordWithReloc {
        word: encode_b(OP_BRANCH, 0b100, rs2, rs1, 0), // blt rs2, rs1
        reloc: Relocation { reloc_type: RelocType::Branch, symbol: label, addend: 0 },
    })
}

pub(crate) fn encode_ble(operands: &[Operand]) -> Result<EncodeResult, String> {
    let rs1 = get_reg(operands, 0)?;
    let rs2 = get_reg(operands, 1)?;
    let label = get_branch_target(operands, 2)?;
    Ok(EncodeResult::WordWithReloc {
        word: encode_b(OP_BRANCH, 0b101, rs2, rs1, 0), // bge rs2, rs1
        reloc: Relocation { reloc_type: RelocType::Branch, symbol: label, addend: 0 },
    })
}

pub(crate) fn encode_bgtu(operands: &[Operand]) -> Result<EncodeResult, String> {
    let rs1 = get_reg(operands, 0)?;
    let rs2 = get_reg(operands, 1)?;
    let label = get_branch_target(operands, 2)?;
    Ok(EncodeResult::WordWithReloc {
        word: encode_b(OP_BRANCH, 0b110, rs2, rs1, 0), // bltu rs2, rs1
        reloc: Relocation { reloc_type: RelocType::Branch, symbol: label, addend: 0 },
    })
}

pub(crate) fn encode_bleu(operands: &[Operand]) -> Result<EncodeResult, String> {
    let rs1 = get_reg(operands, 0)?;
    let rs2 = get_reg(operands, 1)?;
    let label = get_branch_target(operands, 2)?;
    Ok(EncodeResult::WordWithReloc {
        word: encode_b(OP_BRANCH, 0b111, rs2, rs1, 0), // bgeu rs2, rs1
        reloc: Relocation { reloc_type: RelocType::Branch, symbol: label, addend: 0 },
    })
}

pub(crate) fn get_branch_target(operands: &[Operand], idx: usize) -> Result<String, String> {
    match operands.get(idx) {
        Some(Operand::Symbol(s)) | Some(Operand::Label(s)) => Ok(s.clone()),
        Some(Operand::Imm(v)) => Ok(format!("{}", v)),
        // A register name can also be a symbol/label name (e.g. `beqz a0, t1`
        // where t1 is a label). Treat Reg as symbol in branch target context.
        Some(Operand::Reg(s)) => Ok(s.clone()),
        _ => Err(format!("expected branch target at operand {}", idx)),
    }
}

pub(crate) fn encode_j_pseudo(operands: &[Operand]) -> Result<EncodeResult, String> {
    // j offset -> jal x0, offset
    match &operands[0] {
        Operand::Symbol(s) | Operand::Label(s) | Operand::Reg(s) => {
            Ok(EncodeResult::WordWithReloc {
                word: encode_j(OP_JAL, 0, 0),
                reloc: Relocation {
                    reloc_type: RelocType::Jal,
                    symbol: s.clone(),
                    addend: 0,
                },
            })
        }
        Operand::Imm(imm) => {
            Ok(EncodeResult::Word(encode_j(OP_JAL, 0, *imm as i32)))
        }
        _ => Err("j: expected offset or label".to_string()),
    }
}

pub(crate) fn encode_jr(operands: &[Operand]) -> Result<EncodeResult, String> {
    let rs1 = get_reg(operands, 0)?;
    Ok(EncodeResult::Word(encode_i(OP_JALR, 0, 0, rs1, 0)))
}

pub(crate) fn encode_call(operands: &[Operand]) -> Result<EncodeResult, String> {
    // call symbol -> auipc ra, %pcrel_hi(symbol) ; jalr ra, %pcrel_lo(symbol)(ra)
    let (symbol, addend) = get_symbol(operands, 0)?;
    Ok(EncodeResult::WordsWithRelocs(vec![
        (encode_u(OP_AUIPC, 1, 0), Some(Relocation {
            reloc_type: RelocType::CallPlt,
            symbol: symbol.clone(),
            addend,
        })),
        (encode_i(OP_JALR, 1, 0, 1, 0), None), // jalr ra, 0(ra)
    ]))
}

pub(crate) fn encode_tail(operands: &[Operand]) -> Result<EncodeResult, String> {
    // tail symbol -> auipc t1, %pcrel_hi(symbol) ; jalr x0, %pcrel_lo(symbol)(t1)
    let (symbol, addend) = get_symbol(operands, 0)?;
    Ok(EncodeResult::WordsWithRelocs(vec![
        (encode_u(OP_AUIPC, 6, 0), Some(Relocation { // t1 = x6
            reloc_type: RelocType::CallPlt,
            symbol: symbol.clone(),
            addend,
        })),
        (encode_i(OP_JALR, 0, 0, 6, 0), None),
    ]))
}

pub(crate) fn encode_jump(operands: &[Operand]) -> Result<EncodeResult, String> {
    // jump label, temp_reg -> auipc temp, %pcrel_hi(label) ; jalr x0, %pcrel_lo(label)(temp)
    // Our codegen emits: jump .LBB42, t6
    let (symbol, addend) = get_symbol(operands, 0)?;
    let temp = if operands.len() > 1 {
        get_reg(operands, 1)?
    } else {
        31 // t6
    };

    Ok(EncodeResult::WordsWithRelocs(vec![
        (encode_u(OP_AUIPC, temp, 0), Some(Relocation {
            reloc_type: RelocType::CallPlt,
            symbol: symbol.clone(),
            addend,
        })),
        (encode_i(OP_JALR, 0, 0, temp, 0), None),
    ]))
}

pub(crate) fn encode_la(operands: &[Operand]) -> Result<EncodeResult, String> {
    // la rd, symbol -> auipc rd, %pcrel_hi(symbol) ; addi rd, rd, %pcrel_lo(symbol)
    // TODO: For PIC, this should use GOT
    encode_lla(operands) // for now, same as lla
}

pub(crate) fn encode_lla(operands: &[Operand]) -> Result<EncodeResult, String> {
    // lla rd, symbol -> auipc rd, %pcrel_hi(symbol) ; addi rd, rd, %pcrel_lo(symbol)
    let rd = get_reg(operands, 0)?;
    let (symbol, addend) = get_symbol(operands, 1)?;

    Ok(EncodeResult::WordsWithRelocs(vec![
        (encode_u(OP_AUIPC, rd, 0), Some(Relocation {
            reloc_type: RelocType::PcrelHi20,
            symbol: symbol.clone(),
            addend,
        })),
        (encode_i(OP_OP_IMM, rd, 0, rd, 0), Some(Relocation {
            reloc_type: RelocType::PcrelLo12I,
            symbol, // TODO: This should reference the auipc label, not the symbol directly
            addend,
        })),
    ]))
}

pub(crate) fn encode_rdcsr(mnemonic: &str, operands: &[Operand]) -> Result<EncodeResult, String> {
    let rd = get_reg(operands, 0)?;
    let csr = match mnemonic {
        "rdcycle" => 0xC00,
        "rdtime" => 0xC01,
        "rdinstret" => 0xC02,
        _ => return Err(format!("unknown CSR pseudo: {}", mnemonic)),
    };
    Ok(EncodeResult::Word(encode_i(OP_SYSTEM, rd, 0b010, 0, csr))) // csrrs rd, csr, x0
}

pub(crate) fn encode_csrr(operands: &[Operand]) -> Result<EncodeResult, String> {
    let rd = get_reg(operands, 0)?;
    let csr = get_csr_num(operands, 1)?;
    Ok(EncodeResult::Word(encode_i(OP_SYSTEM, rd, 0b010, 0, csr as i32)))
}

pub(crate) fn encode_csrw(operands: &[Operand]) -> Result<EncodeResult, String> {
    let csr = get_csr_num(operands, 0)?;
    if matches!(operands.get(1), Some(Operand::Imm(_))) {
        let zimm = get_imm(operands, 1)? as u32 & 0x1F;
        return Ok(EncodeResult::Word(encode_i(OP_SYSTEM, 0, 0b101, zimm, csr as i32)));
    }
    let rs1 = get_reg(operands, 1)?;
    Ok(EncodeResult::Word(encode_i(OP_SYSTEM, 0, 0b001, rs1, csr as i32)))
}

pub(crate) fn encode_csrs(operands: &[Operand]) -> Result<EncodeResult, String> {
    let csr = get_csr_num(operands, 0)?;
    if matches!(operands.get(1), Some(Operand::Imm(_))) {
        let zimm = get_imm(operands, 1)? as u32 & 0x1F;
        return Ok(EncodeResult::Word(encode_i(OP_SYSTEM, 0, 0b110, zimm, csr as i32)));
    }
    let rs1 = get_reg(operands, 1)?;
    Ok(EncodeResult::Word(encode_i(OP_SYSTEM, 0, 0b010, rs1, csr as i32)))
}

pub(crate) fn encode_csrc(operands: &[Operand]) -> Result<EncodeResult, String> {
    let csr = get_csr_num(operands, 0)?;
    if matches!(operands.get(1), Some(Operand::Imm(_))) {
        let zimm = get_imm(operands, 1)? as u32 & 0x1F;
        return Ok(EncodeResult::Word(encode_i(OP_SYSTEM, 0, 0b111, zimm, csr as i32)));
    }
    let rs1 = get_reg(operands, 1)?;
    Ok(EncodeResult::Word(encode_i(OP_SYSTEM, 0, 0b011, rs1, csr as i32)))
}

// Float pseudo-instructions
pub(crate) fn encode_fmv_s(operands: &[Operand]) -> Result<EncodeResult, String> {
    let rd = get_freg(operands, 0)?;
    let rs1 = get_freg(operands, 1)?;
    // fsgnj.s rd, rs, rs
    Ok(EncodeResult::Word(encode_r(OP_OP_FP, rd, 0b000, rs1, rs1, 0b0010000)))
}

pub(crate) fn encode_fmv_d(operands: &[Operand]) -> Result<EncodeResult, String> {
    let rd = get_freg(operands, 0)?;
    let rs1 = get_freg(operands, 1)?;
    Ok(EncodeResult::Word(encode_r(OP_OP_FP, rd, 0b000, rs1, rs1, 0b0010001)))
}

pub(crate) fn encode_fabs_s(operands: &[Operand]) -> Result<EncodeResult, String> {
    let rd = get_freg(operands, 0)?;
    let rs1 = get_freg(operands, 1)?;
    // fsgnjx.s rd, rs, rs
    Ok(EncodeResult::Word(encode_r(OP_OP_FP, rd, 0b010, rs1, rs1, 0b0010000)))
}

pub(crate) fn encode_fabs_d(operands: &[Operand]) -> Result<EncodeResult, String> {
    let rd = get_freg(operands, 0)?;
    let rs1 = get_freg(operands, 1)?;
    Ok(EncodeResult::Word(encode_r(OP_OP_FP, rd, 0b010, rs1, rs1, 0b0010001)))
}

pub(crate) fn encode_fneg_s(operands: &[Operand]) -> Result<EncodeResult, String> {
    let rd = get_freg(operands, 0)?;
    let rs1 = get_freg(operands, 1)?;
    // fsgnjn.s rd, rs, rs
    Ok(EncodeResult::Word(encode_r(OP_OP_FP, rd, 0b001, rs1, rs1, 0b0010000)))
}

pub(crate) fn encode_fneg_d(operands: &[Operand]) -> Result<EncodeResult, String> {
    let rd = get_freg(operands, 0)?;
    let rs1 = get_freg(operands, 1)?;
    Ok(EncodeResult::Word(encode_r(OP_OP_FP, rd, 0b001, rs1, rs1, 0b0010001)))
}

// ── Relocation modifier parsing ──────────────────────────────────────

/// Extract symbol name from %modifier(symbol) expressions
pub(crate) fn extract_modifier_symbol(s: &str) -> String {
    if let Some(start) = s.find('(') {
        if let Some(end) = s.rfind(')') {
            return s[start + 1..end].to_string();
        }
    }
    s.to_string()
}

/// Parse a relocation modifier like %pcrel_hi(symbol) and return (RelocType, symbol)
pub(crate) fn parse_reloc_modifier(s: &str) -> (RelocType, String) {
    if s.starts_with("%pcrel_hi(") {
        (RelocType::PcrelHi20, extract_modifier_symbol(s))
    } else if s.starts_with("%pcrel_lo(") {
        (RelocType::PcrelLo12I, extract_modifier_symbol(s))
    } else if s.starts_with("%hi(") {
        (RelocType::Hi20, extract_modifier_symbol(s))
    } else if s.starts_with("%lo(") {
        (RelocType::Lo12I, extract_modifier_symbol(s))
    } else if s.starts_with("%tprel_hi(") {
        (RelocType::TprelHi20, extract_modifier_symbol(s))
    } else if s.starts_with("%tprel_lo(") {
        (RelocType::TprelLo12I, extract_modifier_symbol(s))
    } else if s.starts_with("%tprel_add(") {
        (RelocType::TprelAdd, extract_modifier_symbol(s))
    } else if s.starts_with("%got_pcrel_hi(") {
        (RelocType::GotHi20, extract_modifier_symbol(s))
    } else if s.starts_with("%tls_ie_pcrel_hi(") {
        (RelocType::TlsGotHi20, extract_modifier_symbol(s))
    } else if s.starts_with("%tls_gd_pcrel_hi(") {
        (RelocType::TlsGdHi20, extract_modifier_symbol(s))
    } else {
        // Plain symbol - use as PC-relative
        (RelocType::PcrelHi20, s.to_string())
    }
}

#[cfg(test)]
mod encode_neg_pbt {
    // Oracle: differential — llvm-mc RISC-V assembler
    // Evidence: src/backend/riscv/assembler/README.md:321 `neg rd, rs` → `sub rd, x0, rs`;
    //   encoder/mod.rs:1-7 Encodes RISC-V instructions into 32-bit machine code words;
    //   encoder/mod.rs:749 "neg" => encode_neg;
    //   pseudo.rs:239-242 encode_r(OP_OP, rd, 000, x0, rs2, 0100000);
    //   RISC-V Unprivileged ISA pseudoinstruction NEG rd, rs = SUB rd, x0, rs.
    // Stronger considered:
    //   - State machine: rejected — encode_neg is a pure function with no lifecycle
    //   - Algebraic round-trip: rejected — no in-tree NEG/SUB decoder
    //   - Differential vs encode_negw: rejected — different job (SUBW / OP-32)
    //   - Differential vs encode_alu_reg(sub): rejected as primary — shared encode_r/get_reg
    //     (same-job expansion used as a weaker metamorphic instead)
    // Weaker available: algebraic.metamorphic (SUB x0 expansion, ABI/xN alias, Imm 0..31),
    //   algebraic.invariant (R-type field layout), negative_error (arity / invalid / extra)
    // Differential: candidate=encode_neg, reference=llvm-mc -triple=riscv64 -show-encoding,
    //   SUT-boundary=internal-helper of RISC-V assembler,
    //   mapping=[Reg(rd), Reg(rs)] <-> `neg rd, rs`

    use super::encode_neg;
    use super::super::{encode_alu_reg, EncodeResult};
    use crate::backend::riscv::assembler::parser::Operand;
    use proptest::prelude::*;
    use std::io::Write;
    use std::process::{Command, Stdio};

    const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
    const CASES: u32 = 1000;

    const ABI: [&str; 32] = [
        "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2",
        "s0", "s1", "a0", "a1", "a2", "a3", "a4", "a5",
        "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7",
        "s8", "s9", "s10", "s11", "t3", "t4", "t5", "t6",
    ];

    fn cfg() -> ProptestConfig {
        ProptestConfig::with_cases(CASES)
    }

    fn abi_name(n: u32) -> &'static str {
        ABI[n as usize]
    }

    fn xn(n: u32) -> String {
        format!("x{}", n)
    }

    fn reg(name: &str) -> Operand {
        Operand::Reg(name.to_string())
    }

    fn sut_word(ops: &[Operand]) -> Result<u32, String> {
        match encode_neg(ops)? {
            EncodeResult::Word(w) => Ok(w),
            other => Err(format!("expected Word, got {:?}", other)),
        }
    }

    fn sub_word(ops: &[Operand]) -> Result<u32, String> {
        match encode_alu_reg(ops, 0b000, 0b0100000)? {
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
            .args(["-triple=riscv64", "-show-encoding"])
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

    fn llvm_mc_rejects(asm: &str) -> bool {
        llvm_mc_word(asm).is_err()
    }

    /// Pin 0, 8 (s0/fp), 31 plus uniform 0..=31.
    fn reg_num() -> impl Strategy<Value = u32> {
        prop_oneof![Just(0u32), Just(8u32), Just(31u32), 0u32..=31]
    }

    /// Canonical names llvm-mc accepts: ABI or xN (not x00, not uppercase).
    fn gpr_name() -> impl Strategy<Value = String> {
        prop_oneof![
            reg_num().prop_map(|n| abi_name(n).to_string()),
            reg_num().prop_map(xn),
            Just("fp".to_string()),
            Just("zero".to_string()),
            Just("x0".to_string()),
            Just("x31".to_string()),
            Just("t6".to_string()),
        ]
    }

    fn extra_operand() -> impl Strategy<Value = Operand> {
        prop_oneof![
            gpr_name().prop_map(Operand::Reg),
            (-8i64..=40).prop_map(Operand::Imm),
            Just(Operand::Symbol("foo".into())),
            Just(Operand::Label(".L1".into())),
            Just(Operand::Mem {
                base: "sp".into(),
                offset: 8,
            }),
        ]
    }

    fn invalid_operand() -> impl Strategy<Value = Operand> {
        prop_oneof![
            prop::sample::select(vec![
                "fa0", "ft0", "f0", "fs0", "fa1", "f31", "v0", "v31", "x32", "x",
                "foo", "", "spx", "x-1", "r0", "w0",
            ])
            .prop_map(|s| Operand::Reg(s.to_string())),
            prop_oneof![Just(-1i64), Just(32i64), Just(100i64), Just(i64::MIN), Just(i64::MAX)]
                .prop_map(Operand::Imm),
            Just(Operand::Symbol("sym".into())),
            Just(Operand::Label("lbl".into())),
            Just(Operand::Mem {
                base: "sp".into(),
                offset: 0,
            }),
            Just(Operand::Csr("mstatus".into())),
            Just(Operand::FenceArg("iorw".into())),
            Just(Operand::RoundingMode("rne".into())),
            Just(Operand::SymbolOffset("sym".into(), 4)),
            Just(Operand::MemSymbol {
                base: "sp".into(),
                symbol: "foo".into(),
                modifier: "%lo".into(),
            }),
        ]
    }

    fn short_ops() -> impl Strategy<Value = Vec<Operand>> {
        prop_oneof![
            Just(vec![]),
            gpr_name().prop_map(|n| vec![reg(&n)]),
            invalid_operand().prop_map(|o| vec![o]),
            (-4i64..=40).prop_map(|i| vec![Operand::Imm(i)]),
        ]
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_neg_kat_llvm_mc() {
        let want = 0x40b0_0533u32;
        let mc = llvm_mc_word("neg a0, a1").expect("llvm-mc KAT neg a0, a1");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken for neg a0, a1");
        assert_eq!(
            sut_word(&[reg("a0"), reg("a1")]).expect("SUT KAT neg a0, a1"),
            want
        );

        let want_z = 0x4000_0033u32;
        let mc_z = llvm_mc_word("neg zero, zero").expect("llvm-mc KAT neg zero, zero");
        assert_eq!(mc_z, want_z, "llvm-mc KAT mapping broken for neg zero, zero");
        assert_eq!(
            sut_word(&[reg("zero"), reg("zero")]).expect("SUT KAT neg zero, zero"),
            want_z
        );
        assert_eq!(
            sut_word(&[reg("x0"), reg("x0")]).expect("SUT KAT neg x0, x0"),
            want_z
        );

        let want_t6 = 0x4010_0fb3u32;
        let mc_t6 = llvm_mc_word("neg t6, ra").expect("llvm-mc KAT neg t6, ra");
        assert_eq!(mc_t6, want_t6, "llvm-mc KAT mapping broken for neg t6, ra");
        assert_eq!(
            sut_word(&[reg("t6"), reg("ra")]).expect("SUT KAT neg t6, ra"),
            want_t6
        );
        assert_eq!(
            sut_word(&[reg("x31"), reg("x1")]).expect("SUT KAT neg x31, x1"),
            want_t6
        );

        let want_fp = 0x4080_0433u32;
        let mc_fp = llvm_mc_word("neg fp, s0").expect("llvm-mc KAT neg fp, s0");
        assert_eq!(mc_fp, want_fp, "llvm-mc KAT mapping broken for neg fp, s0");
        assert_eq!(
            sut_word(&[reg("fp"), reg("s0")]).expect("SUT KAT neg fp, s0"),
            want_fp
        );
        assert_eq!(
            sut_word(&[reg("x8"), reg("x8")]).expect("SUT KAT neg x8, x8"),
            want_fp
        );

        let mc_sub = llvm_mc_word("sub a0, x0, a1").expect("llvm-mc KAT sub a0, x0, a1");
        assert_eq!(mc_sub, want, "sub a0, x0, a1 must encode as neg a0, a1");
        assert_eq!(
            sub_word(&[reg("a0"), reg("x0"), reg("a1")]).expect("SUB KAT"),
            want
        );
    }

    /// Regression: extra operand must be rejected (README two-operand form; llvm-mc errors).
    #[test]
    fn test_encode_neg_regression_extra_operand() {
        let ops = [reg("zero"), reg("zero"), reg("zero")];
        assert!(
            encode_neg(&ops).is_err(),
            "neg zero, zero with a third operand must be rejected (llvm-mc rejects; README documents `neg rd, rs`)"
        );
    }

    proptest! {
        #![proptest_config(cfg())]

        #[test]
        fn encode_neg_diff_llvm_mc(rd in gpr_name(), rs in gpr_name()) {
            let asm = format!("neg {}, {}", rd, rs);
            let mc = llvm_mc_word(&asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
            let sut = sut_word(&[reg(&rd), reg(&rs)])
                .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
            prop_assert_eq!(sut, mc, "SUT {:08x} != llvm-mc {:08x} for {}", sut, mc, asm);
        }

        /// Sweep: documented expansion vs independent assembler (not in-tree encode_alu_reg).
        #[test]
        fn encode_neg_diff_llvm_mc_sub(rd in gpr_name(), rs in gpr_name()) {
            let asm_sub = format!("sub {}, x0, {}", rd, rs);
            let mc = llvm_mc_word(&asm_sub)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm_sub, e));
            let sut = sut_word(&[reg(&rd), reg(&rs)])
                .unwrap_or_else(|e| panic!("SUT rejected valid neg {}, {}: {}", rd, rs, e));
            prop_assert_eq!(sut, mc, "SUT {:08x} != llvm-mc sub {:08x} for {}", sut, mc, asm_sub);
        }

        #[test]
        fn encode_neg_eq_sub_x0(rd in reg_num(), rs in reg_num()) {
            let ops_neg = [reg(&xn(rd)), reg(&xn(rs))];
            let ops_sub = [reg(&xn(rd)), reg("x0"), reg(&xn(rs))];
            let neg = sut_word(&ops_neg)
                .unwrap_or_else(|e| panic!("SUT rejected neg x{}, x{}: {}", rd, rs, e));
            let sub = sub_word(&ops_sub)
                .unwrap_or_else(|e| panic!("SUT rejected sub x{}, x0, x{}: {}", rd, rs, e));
            prop_assert_eq!(neg, sub);
            let ops_zero = [reg(&xn(rd)), reg("zero"), reg(&xn(rs))];
            let sub_z = sub_word(&ops_zero)
                .unwrap_or_else(|e| panic!("SUT rejected sub x{}, zero, x{}: {}", rd, rs, e));
            prop_assert_eq!(neg, sub_z);
        }

        #[test]
        fn encode_neg_isa_fields(rd in reg_num(), rs in reg_num()) {
            let w = sut_word(&[reg(&xn(rd)), reg(&xn(rs))])
                .unwrap_or_else(|e| panic!("SUT rejected x{}, x{}: {}", rd, rs, e));
            prop_assert_eq!(w & 0x7F, 0b0110011u32, "opcode");
            prop_assert_eq!((w >> 7) & 0x1F, rd, "rd");
            prop_assert_eq!((w >> 12) & 7, 0u32, "funct3");
            prop_assert_eq!((w >> 15) & 0x1F, 0u32, "rs1 must be x0");
            prop_assert_eq!((w >> 20) & 0x1F, rs, "rs2");
            prop_assert_eq!((w >> 25) & 0x7F, 0b0100000u32, "funct7");
        }

        #[test]
        fn encode_neg_abi_xn_alias(n in reg_num(), m in reg_num()) {
            let via_x = sut_word(&[reg(&xn(n)), reg(&xn(m))])
                .unwrap_or_else(|e| panic!("xN rejected: {}", e));
            let via_abi = sut_word(&[reg(abi_name(n)), reg(abi_name(m))])
                .unwrap_or_else(|e| panic!("ABI rejected: {}", e));
            prop_assert_eq!(via_abi, via_x);
            if n == 8 {
                let via_fp = sut_word(&[reg("fp"), reg(&xn(m))])
                    .unwrap_or_else(|e| panic!("fp rejected: {}", e));
                prop_assert_eq!(via_fp, via_x);
            }
            if m == 8 {
                let via_fp = sut_word(&[reg(&xn(n)), reg("fp")])
                    .unwrap_or_else(|e| panic!("fp rs rejected: {}", e));
                prop_assert_eq!(via_fp, via_x);
            }
        }

        #[test]
        fn encode_neg_neg_arity(ops in short_ops()) {
            prop_assume!(ops.len() < 2);
            prop_assert!(encode_neg(&ops).is_err(), "arity {} must Err", ops.len());
        }

        #[test]
        fn encode_neg_neg_invalid(
            bad in invalid_operand(),
            good in gpr_name(),
            which in 0u8..=2u8,
        ) {
            let ops: Vec<Operand> = match which {
                0 => vec![bad.clone(), bad.clone()],
                1 => vec![bad, reg(&good)],
                _ => vec![reg(&good), bad],
            };
            prop_assert!(encode_neg(&ops).is_err(), "invalid operand must Err, got {:?}", encode_neg(&ops));
        }

        #[test]
        fn encode_neg_neg_extra(
            rd in gpr_name(),
            rs in gpr_name(),
            extra in extra_operand(),
        ) {
            let asm = format!("neg {}, {}", rd, rs);
            // Contract: exactly two operands (README + llvm-mc).
            let ops = vec![reg(&rd), reg(&rs), extra];
            prop_assert!(
                encode_neg(&ops).is_err(),
                "extra operand must Err for {} (llvm-mc rejects: {})",
                asm,
                llvm_mc_rejects(&format!("{}, a2", asm))
            );
        }

        #[test]
        fn encode_neg_imm_regnum(n in 0i64..=31, m in 0i64..=31) {
            let via_imm = sut_word(&[Operand::Imm(n), Operand::Imm(m)])
                .unwrap_or_else(|e| panic!("Imm({}, {}) rejected: {}", n, m, e));
            let via_x = sut_word(&[reg(&xn(n as u32)), reg(&xn(m as u32))])
                .unwrap_or_else(|e| panic!("xN rejected: {}", e));
            prop_assert_eq!(via_imm, via_x);
            let mixed = sut_word(&[Operand::Imm(n), reg(&xn(m as u32))])
                .unwrap_or_else(|e| panic!("mixed Imm/Reg rejected: {}", e));
            prop_assert_eq!(mixed, via_x);
        }
    }
}
