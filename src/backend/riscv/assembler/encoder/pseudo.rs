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
mod pbt_li_exec {
    use super::*;
    use proptest::prelude::*;

    const OP_LUI_T: u32 = 0x37;
    const OP_OP_IMM_T: u32 = 0x13;
    const OP_OP_IMM_32_T: u32 = 0x1B;

    /// 解码 I-type: (rd, funct3, rs1, imm)
    fn dec_i(w: u32) -> (u32, u32, u32, i32) {
        let rd = (w >> 7) & 0x1F;  // I-type rd at bits 11-7
        let funct3 = (w >> 12) & 0x7;
        let rs1 = (w >> 15) & 0x1F;
        let imm = ((w as i32) >> 20) as i32;
        (rd, funct3, rs1, imm)
    }

    /// 模拟执行 encode_li_immediate 生成的指令序列
    fn exec_li_words(words: &[u32]) -> u64 {
        let mut x = [0u64; 32];
        for &w in words {
            let op = w & 0x7F;
            let (rd, funct3, rs1, imm) = dec_i(w);
            let rd = rd as usize;
            let rs1 = rs1 as usize;
            let result: u64 = match (op, funct3) {
                (OP_LUI_T, _) => {
                    // LUI imm20 field is bits 31:12 (NOT the I-type bits 31:20)
                    let imm20 = (w >> 12) as i32; // 20-bit field
                    (imm20.wrapping_shl(12)) as i64 as u64 // sign-extended 32-bit → 64
                }
                (OP_OP_IMM_T, 0b000) => x[rs1].wrapping_add(imm as i64 as u64), // addi
                (OP_OP_IMM_T, 0b001) => {
                    let sh = (imm & 0x3F) as u32;
                    x[rs1] << sh // slli
                }
                (OP_OP_IMM_32_T, 0b000) => (x[rs1] as u32).wrapping_add(imm as u32) as i32 as i64 as u64, // addiw
                (OP_OP_IMM_32_T, 0b001) => {
                    let sh = (imm & 0x3F) as u32;
                    let v = (x[rs1] << sh) as u32; // slliw (w-reg)
                    v as i32 as i64 as u64
                }
                _ => panic!("unexpected instruction {:#010x} (op={:#04x}, f3={:#03x})", w, op, funct3),
            };
            if rd != 0 { x[rd] = result; }
        }
        x[10] // a0 假设 rd=10 — caller 控制
    }

    /// P-LI-EXEC: encode_li_immediate 生成的序列执行后必须精确重建 imm
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(2048))]
        #[test]
        fn p_li_exec_roundtrip(
            imm in proptest::strategy::Union::new(vec![
                any::<i64>().boxed(),
                (0i64..64).prop_map(|k| (1i64 << k) | 1).boxed(),
                (0i64..64).prop_map(|k| ((1u64 << k) - 1) as i64).boxed(),
                (0i64..64).prop_map(|k| (-1i64) << k).boxed(),
                any::<i32>().prop_map(|v| v as i64).boxed(),
                (0i64..65536).prop_map(|hi| ((hi as i64) << 48) | 0x1234).boxed(),
            ]),
        ) {
            let words = encode_li_immediate(10, imm);
            prop_assert!(!words.is_empty(), "li {imm}: empty sequence");
            prop_assert!(words.len() <= 8, "li {imm}: {} words (expected <= 8)", words.len());
            let got = exec_li_words(&words);
            let want = imm as u64;
            prop_assert_eq!(got, want,
                "li rd, {} (0x{:016x}): sequence produced 0x{:016x}\nwords: {:?}",
                imm, want, got, words.iter().map(|w| format!("{w:#010x}")).collect::<Vec<_>>());
        }
    }
}

#[cfg(test)]
mod pbt_pseudo_diff {
    use super::*;
    use crate::backend::riscv::assembler::parser::Operand;

    fn r(s: &str) -> Operand { Operand::Reg(s.to_string()) }
    fn i(v: i64) -> Operand { Operand::Imm(v) }

    /// P-PSEUDO-DIFF: 寄存器类 pseudo 与 llvm-mc 参考编码逐字节比对
    #[test]
    fn p_pseudo_diff_reg_only() {
        // (name, result, llvm-mc word, note)
        let cases: Vec<(&str, Result<EncodeResult, String>, u32)> = vec![
            ("not x1,x2",   encode_not(&[r("x1"), r("x2")]),           0xfff14093),
            ("neg x1,x2",   encode_neg(&[r("x1"), r("x2")]),           0x402000b3),
            ("negw x1,x2",  encode_negw(&[r("x1"), r("x2")]),          0x402000bb),
            ("sext.w x1,x2",encode_sext_w(&[r("x1"), r("x2")]),        0x0001009b),
            ("seqz x1,x2",  encode_seqz(&[r("x1"), r("x2")]),          0x00113093),
            ("snez x1,x2",  encode_snez(&[r("x1"), r("x2")]),          0x002030b3),
            ("sltz x1,x2",  encode_sltz(&[r("x1"), r("x2")]),          0x000120b3),
            ("sgtz x1,x2",  encode_sgtz(&[r("x1"), r("x2")]),          0x002020b3),
            // repo uses `add rd, x0, rs2` (R-type) instead of canonical `addi rd, rs2, 0`
            // — functionally identical, valid mv expansion. llvm-mc: 0x00010093.
            ("mv x1,x2",    encode_mv(&[r("x1"), r("x2")]),            0x002000b3),
            ("csrr x1,0x300", encode_csrr(&[r("x1"), i(0x300)]),       0x300020f3),
            ("csrw 0x300,x1", encode_csrw(&[i(0x300), r("x1")]),       0x30009073),
            ("csrs 0x300,x1", encode_csrs(&[i(0x300), r("x1")]),       0x3000a073),
            ("csrc 0x300,x1", encode_csrc(&[i(0x300), r("x1")]),       0x3000b073),
        ];
        let mut fails = 0;
        for (name, res, expect) in cases {
            match res {
                Ok(EncodeResult::Word(w)) if w == expect => {}
                Ok(EncodeResult::Word(w)) => {
                    println!("  MISMATCH {}: got {:#010x}, want {:#010x}", name, w, expect);
                    fails += 1;
                }
                Ok(_) => { println!("  NON-WORD {}: unexpected result variant", name); fails += 1; }
                Err(e) => { println!("  ERR {}: {}", name, &e[..e.len().min(50)]); fails += 1; }
            }
        }
        assert_eq!(fails, 0, "{} pseudo encodings mismatched llvm-mc", fails);
    }

    /// P-FP-PSEUDO-DIFF: FP pseudo（fmv/fabs/fneg）
    #[test]
    fn p_fp_pseudo_diff() {
        let cases: Vec<(&str, Result<EncodeResult, String>, u32)> = vec![
            ("fmv.s f1,f2",  encode_fmv_s(&[r("f1"), r("f2")]),  0x202100d3),
            ("fmv.d f1,f2",  encode_fmv_d(&[r("f1"), r("f2")]),  0x222100d3),
            ("fabs.s f1,f2", encode_fabs_s(&[r("f1"), r("f2")]), 0x202120d3),
            ("fabs.d f1,f2", encode_fabs_d(&[r("f1"), r("f2")]), 0x222120d3),
            ("fneg.s f1,f2", encode_fneg_s(&[r("f1"), r("f2")]), 0x202110d3),
            ("fneg.d f1,f2", encode_fneg_d(&[r("f1"), r("f2")]), 0x222110d3),
        ];
        let mut fails = 0;
        for (name, res, expect) in cases {
            match res {
                Ok(EncodeResult::Word(w)) if w == expect => {}
                Ok(EncodeResult::Word(w)) => {
                    println!("  MISMATCH {}: got {:#010x}, want {:#010x}", name, w, expect);
                    fails += 1;
                }
                Ok(_) => { println!("  NON-WORD {}", name); fails += 1; }
                Err(e) => { println!("  ERR {}: {}", name, &e[..e.len().min(50)]); fails += 1; }
            }
        }
        // fneg.d 的正确值从 llvm-mc 取；此测试若 fail 先打印再判断
        if fails > 0 { println!("(verify against: llvm-mc --triple=riscv64 -mattr=+f,+d)"); }
        assert_eq!(fails, 0, "{} FP pseudo encodings mismatched", fails);
    }
}

#[cfg(test)]
mod pbt_extra_unclaimed {
    //! Round-37: extra-operand sweep over NOT-yet-claimed RISC-V encoders.
    //! Claimed set (137 fns) from dedup DB — this sweep only reports unclaimed.
    use super::*;
    use crate::backend::riscv::assembler::parser::Operand;

    fn r(s: &str) -> Operand { Operand::Reg(s.to_string()) }
    fn i(v: i64) -> Operand { Operand::Imm(v) }
    fn sym(s: &str) -> Operand { Operand::Symbol(s.to_string()) }

    #[test]
    fn p_extra_unclaimed_sweep() {
        // (fn name, correct form, +1 extra operand form)
        let cases: Vec<(&str, Vec<Operand>, Vec<Operand>)> = vec![
            ("encode_bgez", vec![r("a0"), sym("L")], vec![r("a0"), sym("L"), r("a1")]),
            ("encode_bltz", vec![r("a0"), sym("L")], vec![r("a0"), sym("L"), r("a1")]),
            ("encode_bgtz", vec![r("a0"), sym("L")], vec![r("a0"), sym("L"), r("a1")]),
            ("encode_bgt",  vec![r("a0"), r("a1"), sym("L")], vec![r("a0"), r("a1"), sym("L"), r("a2")]),
            ("encode_ble",  vec![r("a0"), r("a1"), sym("L")], vec![r("a0"), r("a1"), sym("L"), r("a2")]),
            ("encode_bgtu", vec![r("a0"), r("a1"), sym("L")], vec![r("a0"), r("a1"), sym("L"), r("a2")]),
            ("encode_bleu", vec![r("a0"), r("a1"), sym("L")], vec![r("a0"), r("a1"), sym("L"), r("a2")]),
            ("encode_csrr", vec![r("a0"), i(0x300)], vec![r("a0"), i(0x300), i(1)]),
            ("encode_csrw", vec![i(0x300), r("a0")], vec![i(0x300), r("a0"), r("a1")]),
            ("encode_csrs", vec![i(0x300), r("a0")], vec![i(0x300), r("a0"), r("a1")]),
            ("encode_csrc", vec![i(0x300), r("a0")], vec![i(0x300), r("a0"), r("a1")]),
            ("encode_fmv_s",  vec![r("f0"), r("f1")], vec![r("f0"), r("f1"), r("f2")]),
            ("encode_fmv_d",  vec![r("f0"), r("f1")], vec![r("f0"), r("f1"), r("f2")]),
            ("encode_fabs_s", vec![r("f0"), r("f1")], vec![r("f0"), r("f1"), r("f2")]),
            ("encode_fabs_d", vec![r("f0"), r("f1")], vec![r("f0"), r("f1"), r("f2")]),
            ("encode_fneg_s", vec![r("f0"), r("f1")], vec![r("f0"), r("f1"), r("f2")]),
            ("encode_fneg_d", vec![r("f0"), r("f1")], vec![r("f0"), r("f1"), r("f2")]),
            ("encode_la",  vec![r("a0"), sym("sym")], vec![r("a0"), sym("sym"), r("a1")]),
            ("encode_lla", vec![r("a0"), sym("sym")], vec![r("a0"), sym("sym"), r("a1")]),
        ];
        let mut ignored = Vec::new();
        for (name, base, extra) in &cases {
            let rb = dispatch(name, base);
            let re = dispatch(name, extra);
            match (rb, re) {
                (Ok(_), Ok(_)) => { println!("  {}: extra operand IGNORED", name); ignored.push(*name); }
                (Ok(_), Err(_)) => println!("  {}: extra operand REJECTED (correct)", name),
                (Err(e), _) => println!("  {}: base form fails: {}", name, &e[..e.len().min(40)]),
            }
        }
        println!("\\nUNCLAIMED + IGNORED: {:?}", ignored);
        assert!(ignored.is_empty(), "unclaimed fns silently ignore extra operands: {:?}", ignored);
    }

    fn dispatch(name: &str, ops: &[Operand]) -> Result<EncodeResult, String> {
        match name {
            "encode_bgez" => encode_bgez(ops),
            "encode_bltz" => encode_bltz(ops),
            "encode_bgtz" => encode_bgtz(ops),
            "encode_bgt" => encode_bgt(ops),
            "encode_ble" => encode_ble(ops),
            "encode_bgtu" => encode_bgtu(ops),
            "encode_bleu" => encode_bleu(ops),
            "encode_csrr" => encode_csrr(ops),
            "encode_csrw" => encode_csrw(ops),
            "encode_csrs" => encode_csrs(ops),
            "encode_csrc" => encode_csrc(ops),
            "encode_fmv_s" => encode_fmv_s(ops),
            "encode_fmv_d" => encode_fmv_d(ops),
            "encode_fabs_s" => encode_fabs_s(ops),
            "encode_fabs_d" => encode_fabs_d(ops),
            "encode_fneg_s" => encode_fneg_s(ops),
            "encode_fneg_d" => encode_fneg_d(ops),
            "encode_la" => encode_la(ops),
            "encode_lla" => encode_lla(ops),
            _ => Err("unknown".into()),
        }
    }
}
