use super::*;
use crate::backend::arm::assembler::parser::Operand;

// ── NEON/SIMD ────────────────────────────────────────────────────────────

/// Helper to extract register number from a RegArrangement operand
pub(crate) fn get_neon_reg(operands: &[Operand], idx: usize) -> Result<(u32, String), String> {
    match operands.get(idx) {
        Some(Operand::RegArrangement { reg, arrangement }) => {
            let num = parse_reg_num(reg)
                .ok_or_else(|| format!("invalid NEON register: {}", reg))?;
            Ok((num, arrangement.clone()))
        }
        Some(Operand::Reg(name)) => {
            let num = parse_reg_num(name)
                .ok_or_else(|| format!("invalid register: {}", name))?;
            Ok((num, String::new()))
        }
        other => Err(format!("expected NEON register at operand {}, got {:?}", idx, other)),
    }
}

pub(crate) fn encode_cnt(operands: &[Operand]) -> Result<EncodeResult, String> {
    // CNT Vd.<T>, Vn.<T>
    // Encoding: 0 Q 00 1110 size 10 0000 0101 10 Rn Rd
    // Only valid for .8b (Q=0) and .16b (Q=1)
    if operands.len() < 2 {
        return Err("cnt requires 2 operands".to_string());
    }
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _arr_n) = get_neon_reg(operands, 1)?;

    let q: u32 = if arr_d == "16b" { 1 } else { 0 }; // .8b -> Q=0, .16b -> Q=1

    // 0 Q 00 1110 00 10 0000 0101 10 Rn Rd
    let word = ((q << 30) | (0b001110 << 24)) | (0b100000 << 16)
        | (0b010110 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

// ── NEON three-same register operations ──────────────────────────────────

/// Get Q bit and size from arrangement specifier.
pub(crate) fn neon_arr_to_q_size(arr: &str) -> Result<(u32, u32), String> {
    match arr {
        "8b" => Ok((0, 0b00)),
        "16b" => Ok((1, 0b00)),
        "4h" => Ok((0, 0b01)),
        "8h" => Ok((1, 0b01)),
        "2s" => Ok((0, 0b10)),
        "4s" => Ok((1, 0b10)),
        "1d" => Ok((0, 0b11)),
        "2d" => Ok((1, 0b11)),
        _ => Err(format!("unsupported NEON arrangement: {}", arr)),
    }
}

/// Encode NEON three-same-register instructions: CMEQ, UQSUB, SQSUB, CMHI, etc.
///
/// Layout: 0 Q U 01110 size 1 Rm opcode 1 Rn Rd
///         31 30 29 28-24 23-22 21 20-16 15-11 10 9-5 4-0
///
/// `u_bit`: U field (bit 29) - 0 for signed, 1 for unsigned
/// `opcode`: instruction opcode (bits 15-11)
pub(crate) fn encode_neon_three_same(operands: &[Operand], u_bit: u32, opcode: u32) -> Result<EncodeResult, String> {
    if operands.len() < 3 {
        return Err("NEON three-same requires 3 operands".to_string());
    }
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _arr_n) = get_neon_reg(operands, 1)?;
    let (rm, _arr_m) = get_neon_reg(operands, 2)?;

    let (q, size) = neon_arr_to_q_size(&arr_d)?;

    // 0 Q U 01110 size 1 Rm opcode 1 Rn Rd
    let word = (q << 30) | (u_bit << 29) | (0b01110 << 24) | (size << 22) | (1 << 21)
        | (rm << 16) | (opcode << 11) | (1 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode NEON three-different instructions: USUBL, SSUBL, UADDL, SADDL, etc.
///
/// These instructions have wider destination than source operands.
/// Format: 0 Q U 01110 size 1 Rm opcode 00 Rn Rd
///
/// `u_bit`: 0 for signed, 1 for unsigned
/// `opcode`: 4-bit opcode (bits 15-12)
/// `is_high`: true for the "2" variant (upper half, Q=1)
pub(crate) fn encode_neon_three_diff(operands: &[Operand], u_bit: u32, opcode: u32, is_high: bool) -> Result<EncodeResult, String> {
    if operands.len() < 3 {
        return Err("NEON three-different requires 3 operands".to_string());
    }
    let (rd, _arr_d) = get_neon_reg(operands, 0)?;
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    let (rm, _arr_m) = get_neon_reg(operands, 2)?;

    // Size is determined from the source (narrow) arrangement
    let (q, size) = match arr_n.as_str() {
        "8b" => (0u32, 0b00u32),   // base
        "16b" => (1, 0b00),         // "2" variant
        "4h" => (0, 0b01),
        "8h" => (1, 0b01),
        "2s" => (0, 0b10),
        "4s" => (1, 0b10),
        _ => return Err(format!("unsupported source arrangement for three-diff: {}", arr_n)),
    };

    // For the "2" variant, override Q
    let q = if is_high { 1 } else { q };

    // 0 Q U 01110 size 1 Rm opcode 00 Rn Rd
    let word = (q << 30) | (u_bit << 29) | (0b01110 << 24) | (size << 22)
        | (1 << 21) | (rm << 16) | (opcode << 12) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode NEON SQSHRUN/SQSHRUN2: Signed saturating shift right unsigned narrow
/// Format: 0 Q 1 011110 immh immb 100011 Rn Rd
pub(crate) fn encode_neon_sqshrun(operands: &[Operand], is_rounding: bool, is_high: bool) -> Result<EncodeResult, String> {
    if operands.len() < 3 {
        return Err("sqshrun requires 3 operands".to_string());
    }
    let (rd, _arr_d) = get_neon_reg(operands, 0)?;
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    let shift = match &operands[2] {
        Operand::Imm(v) => *v as u32,
        _ => return Err("sqshrun: expected immediate shift".to_string()),
    };

    // immh:immb encode element size and shift amount
    // For source .4s (dest .4h or .8h): immh=001x, shift_amount = 32 - (immh:immb)
    // For source .8h (dest .8b or .16b): immh=0001, shift_amount = 16 - (immh:immb)
    // For source .2d (dest .2s or .4s): immh=01xx, shift_amount = 64 - (immh:immb)
    let (element_bits, immh_base) = match arr_n.as_str() {
        "8h" => (16u32, 0b0001u32),
        "4s" => (32, 0b0010),
        "2d" => (64, 0b0100),
        _ => return Err(format!("sqshrun: unsupported source arrangement: {}", arr_n)),
    };

    if shift == 0 || shift > element_bits {
        return Err(format!("sqshrun: shift {} out of range for {}-bit elements", shift, element_bits));
    }

    let immhb = (element_bits - shift) & 0x7F; // immh:immb combined
    let immh = (immhb >> 3) | immh_base;
    let immb = immhb & 0x7;

    let q = if is_high { 1u32 } else { 0 };

    // 0 Q 1 011110 immh immb opcode 1 Rn Rd
    // SQSHRUN: opcode = 100001, SQRSHRUN: opcode = 100011
    let opcode_bits: u32 = if is_rounding { 0b100011 } else { 0b100001 };
    let word = (q << 30) | (1 << 29) | (0b011110 << 23) | (immh << 19) | (immb << 16)
        | (opcode_bits << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode NEON UXTL/SXTL (unsigned/signed extend long).
/// These are aliases for USHLL/SSHLL with shift #0.
///
/// Format: 0 Q U 011110 immh immb 10100 1 Rn Rd
pub(crate) fn encode_neon_xtl(operands: &[Operand], u_bit: u32, is_high: bool) -> Result<EncodeResult, String> {
    if operands.len() < 2 {
        return Err("NEON uxtl/sxtl requires 2 operands".to_string());
    }
    let (rd, _arr_d) = get_neon_reg(operands, 0)?;
    let (rn, arr_n) = get_neon_reg(operands, 1)?;

    // immh encodes the source element size, immb=0 (shift=0)
    let immh = match arr_n.as_str() {
        "8b" | "16b" => 0b0001u32,
        "4h" | "8h" => 0b0010,
        "2s" | "4s" => 0b0100,
        _ => return Err(format!("uxtl/sxtl: unsupported source arrangement: {}", arr_n)),
    };

    let q = if is_high { 1u32 } else { 0 };

    // 0 Q U 011110 immh immb 10100 1 Rn Rd
    let word = (q << 30) | (u_bit << 29) | (0b011110 << 23) | (immh << 19)
        | (0b101001 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode NEON compare-to-zero: CMEQ Vd, Vn, #0, CMGE Vd, Vn, #0, etc.
///
/// Format: 0 Q U 01110 size 10000 opcode 10 Rn Rd
pub(crate) fn encode_neon_cmp_zero(operands: &[Operand], u_bit: u32, opcode: u32) -> Result<EncodeResult, String> {
    if operands.len() < 2 {
        return Err("NEON compare-zero requires at least 2 operands".to_string());
    }
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;
    let (q, size) = neon_arr_to_q_size(&arr_d)?;

    // 0 Q U 01110 size 10000 opcode 10 Rn Rd
    let word = (q << 30) | (u_bit << 29) | (0b01110 << 24) | (size << 22)
        | (0b10000 << 17) | (opcode << 12) | (0b10 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode NEON two-register miscellaneous narrowing: UQXTN, SQXTN, XTN
///
/// Format: 0 Q U 01110 size 10000 opcode 10 Rn Rd
pub(crate) fn encode_neon_two_misc_narrow(operands: &[Operand], u_bit: u32, opcode: u32, is_high: bool) -> Result<EncodeResult, String> {
    if operands.len() < 2 {
        return Err("NEON two-reg narrow requires 2 operands".to_string());
    }
    let (rd, _arr_d) = get_neon_reg(operands, 0)?;
    let (rn, arr_n) = get_neon_reg(operands, 1)?;

    // Size from source (wider) arrangement
    let size = match arr_n.as_str() {
        "8h" => 0b00u32,
        "4s" => 0b01,
        "2d" => 0b10,
        _ => return Err(format!("unsupported source arrangement for narrow: {}", arr_n)),
    };

    let q = if is_high { 1u32 } else { 0 };

    // 0 Q U 01110 size 10000 opcode 10 Rn Rd
    let word = (q << 30) | (u_bit << 29) | (0b01110 << 24) | (size << 22)
        | (0b10000 << 17) | (opcode << 12) | (0b10 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode NEON vector-by-element long instructions: SMULL/UMULL/SMLAL/UMLAL/SMLSL/UMLSL (elem)
///
/// Format: 0 Q U 01111 size L M Rm opcode H 0 Rn Rd
///
/// These are the widening multiply-by-element forms where the third operand
/// is a register lane (e.g., v0.h[2]).
pub(crate) fn encode_neon_elem_long(operands: &[Operand], u_bit: u32, opcode: u32, is_high: bool) -> Result<EncodeResult, String> {
    if operands.len() < 3 {
        return Err("NEON elem-long requires 3 operands".to_string());
    }
    let (rd, _arr_d) = get_neon_reg(operands, 0)?;
    let (rn, arr_n) = get_neon_reg(operands, 1)?;

    // Third operand is RegLane: v0.h[2]
    let (rm, index) = match &operands[2] {
        Operand::RegLane { reg, elem_size: _, index } => {
            let rm = parse_reg_num(reg).ok_or("invalid NEON register")?;
            (rm, *index)
        }
        _ => return Err(format!("expected register lane operand, got {:?}", operands[2])),
    };

    // Determine size and Q from source arrangement
    let (q, size) = match arr_n.as_str() {
        "4h" => (0u32, 0b01u32),
        "8h" => (1, 0b01),
        "2s" => (0, 0b10),
        "4s" => (1, 0b10),
        _ => return Err(format!("unsupported source arrangement for elem-long: {}", arr_n)),
    };
    let q = if is_high { 1 } else { q };

    // Encode index into H:L:M bits depending on element size
    let (h, l, m) = match size {
        0b01 => {
            // Half-word: index = H:L:M (3 bits), Rm limited to v0-v15
            if index > 7 {
                return Err(format!("element index {} out of range for .h", index));
            }
            let h = (index >> 2) & 1;
            let l = (index >> 1) & 1;
            let m = index & 1;
            (h, l, m)
        }
        0b10 => {
            // Word: index = H:L (2 bits), M=Rm[4]
            if index > 3 {
                return Err(format!("element index {} out of range for .s", index));
            }
            let h = (index >> 1) & 1;
            let l = index & 1;
            let m = (rm >> 4) & 1; // M bit from Rm[4]
            (h, l, m)
        }
        _ => return Err("unsupported element size for by-element".to_string()),
    };

    // Limit Rm for half-word indexing (only v0-v15)
    let rm_enc = if size == 0b01 { rm & 0xF } else { rm & 0x1F };

    // 0 Q U 01111 size L M Rm opcode H 0 Rn Rd
    let word = (q << 30) | (u_bit << 29) | (0b01111 << 24) | (size << 22)
        | (l << 21) | (m << 20) | (rm_enc << 16) | (opcode << 12)
        | (h << 11) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode NEON logical operations: ORR/AND/EOR Vd.T, Vn.T, Vm.T
pub(crate) fn encode_neon_logical(operands: &[Operand], opc: u32) -> Result<EncodeResult, String> {
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _arr_n) = get_neon_reg(operands, 1)?;
    let (rm, _arr_m) = get_neon_reg(operands, 2)?;

    let q: u32 = if arr_d == "16b" { 1 } else { 0 };

    // NEON logical three-same:
    // ORR: 0 Q 0 01110 10 1 Rm 000111 Rn Rd  (opc=0b01 -> size=10)
    // AND: 0 Q 0 01110 00 1 Rm 000111 Rn Rd  (opc=0b00 -> size=00)
    // EOR: 0 Q 1 01110 00 1 Rm 000111 Rn Rd  (opc=0b10 -> size=00, U=1)
    // BIC: 0 Q 0 01110 01 1 Rm 000111 Rn Rd  (would be opc=0b01 with N=1... but not needed)
    let (u_bit, size_bits): (u32, u32) = match opc {
        0b00 => (0, 0b00),  // AND
        0b01 => (0, 0b10),  // ORR
        0b10 => (1, 0b00),  // EOR
        0b11 => (1, 0b00),  // ANDS - not valid for NEON, fall back
        _ => return Err("unsupported NEON logical opc".to_string()),
    };

    let word = (q << 30) | (u_bit << 29) | (0b01110 << 24) | (size_bits << 22)
        | (1 << 21) | (rm << 16) | (0b000111 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode NEON MUL Vd.T, Vn.T, Vm.T
pub(crate) fn encode_neon_mul(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;
    let (rm, _) = get_neon_reg(operands, 2)?;
    let (q, size) = neon_arr_to_q_size(&arr_d)?;

    // MUL (vector): 0 Q 0 01110 size 1 Rm 10011 1 Rn Rd
    let word = (q << 30) | (0b001110 << 24) | (size << 22) | (1 << 21)
        | (rm << 16) | (0b100111 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode NEON PMUL Vd.T, Vn.T, Vm.T (polynomial multiply, bytes only)
pub(crate) fn encode_neon_pmul(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;
    let (rm, _) = get_neon_reg(operands, 2)?;
    let q: u32 = if arr_d == "16b" { 1 } else { 0 };
    // PMUL: 0 Q 1 01110 00 1 Rm 10011 1 Rn Rd (size=00 for bytes, U=1)
    // PMUL encoding: size=00 (bytes) is implicit (zero bits at [23:22])
    let word = (q << 30) | (1 << 29) | (0b01110 << 24) | (1 << 21)
        | (rm << 16) | (0b100111 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode NEON MLA Vd.T, Vn.T, Vm.T (multiply-accumulate)
pub(crate) fn encode_neon_mla(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;
    let (rm, _) = get_neon_reg(operands, 2)?;
    let (q, size) = neon_arr_to_q_size(&arr_d)?;
    // MLA: 0 Q 0 01110 size 1 Rm 10010 1 Rn Rd
    let word = (q << 30) | (0b001110 << 24) | (size << 22) | (1 << 21)
        | (rm << 16) | (0b100101 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode NEON MLS Vd.T, Vn.T, Vm.T (multiply-subtract)
pub(crate) fn encode_neon_mls(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;
    let (rm, _) = get_neon_reg(operands, 2)?;
    let (q, size) = neon_arr_to_q_size(&arr_d)?;
    // MLS: 0 Q 1 01110 size 1 Rm 10010 1 Rn Rd (U=1)
    let word = (q << 30) | (1 << 29) | (0b01110 << 24) | (size << 22) | (1 << 21)
        | (rm << 16) | (0b100101 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode NEON USHR Vd.T, Vn.T, #shift (unsigned shift right immediate)
pub(crate) fn encode_neon_shift_imm(operands: &[Operand], _is_unsigned: bool) -> Result<EncodeResult, String> {
    if operands.len() < 3 {
        return Err("ushr requires 3 operands".to_string());
    }
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;
    let shift = get_imm(operands, 2)?;

    let (q, _size) = neon_arr_to_q_size(&arr_d)?;

    // USHR: 0 Q 1 011110 immh:immb 00000 1 Rn Rd
    // For .16b (bytes, size=8): immh = 0001, immb = 8-shift (3 bits)
    // For .8h (halfwords, size=16): immh = 001x
    // For .4s (words, size=32): immh = 01xx
    // For .2d (doublewords, size=64): immh = 1xxx
    // immh:immb = (element_size * 2 - shift)
    let (elem_bits, immh_immb) = match arr_d.as_str() {
        "8b" | "16b" => (8u32, (16 - shift as u32) & 0xF),   // immh:immb is 4 bits for 8-bit elems
        "4h" | "8h" => (16, (32 - shift as u32) & 0x1F),
        "2s" | "4s" => (32, (64 - shift as u32) & 0x3F),
        "2d" => (64, (128 - shift as u32) & 0x7F),
        _ => return Err(format!("unsupported USHR arrangement: {}", arr_d)),
    };
    let _ = elem_bits;

    // Full encoding: 0 Q 1 011110 immh:immb 000001 Rn Rd
    let word = (q << 30) | (1 << 29) | (0b011110 << 23) | (immh_immb << 16)
        | (0b000001 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode NEON EXT Vd.T, Vn.T, Vm.T, #index
pub(crate) fn encode_neon_ext(operands: &[Operand]) -> Result<EncodeResult, String> {
    if operands.len() < 4 {
        return Err("ext requires 4 operands".to_string());
    }
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;
    let (rm, _) = get_neon_reg(operands, 2)?;
    let index = get_imm(operands, 3)? as u32;

    let q: u32 = if arr_d == "16b" { 1 } else { 0 };

    // EXT Vd.T, Vn.T, Vm.T, #index
    // Encoding: 0 Q 10 1110 00 0 Rm 0 imm4 0 Rn Rd
    let word = ((((q << 30) | (0b101110 << 24))
        | (rm << 16)) | ((index & 0xF) << 11)) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode NEON ADDV: add across vector lanes
pub(crate) fn encode_neon_addv(operands: &[Operand]) -> Result<EncodeResult, String> {
    if operands.len() < 2 {
        return Err("addv requires 2 operands".to_string());
    }
    let (rd, _) = get_neon_reg(operands, 0)?;
    let (rn, arr_n) = get_neon_reg(operands, 1)?;

    let (q, size) = neon_arr_to_q_size(&arr_n)?;

    // ADDV: 0 Q 0 01110 size 11000 11011 10 Rn Rd
    let word = (q << 30) | (0b001110 << 24) | (size << 22) | (0b11000 << 17)
        | (0b110111 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode NEON across-vector instructions: UMAXV, UMINV, SMAXV, SMINV
///
/// Format: 0 Q U 01110 size 11000 opcode 10 Rn Rd
///
/// `u_bit`: 0 for signed, 1 for unsigned
/// `opcode`: 5-bit opcode (bits 16-12)
pub(crate) fn encode_neon_across(operands: &[Operand], u_bit: u32, opcode: u32) -> Result<EncodeResult, String> {
    if operands.len() < 2 {
        return Err("NEON across-vector requires 2 operands".to_string());
    }
    let (rd, _) = get_neon_reg(operands, 0)?;
    let (rn, arr_n) = get_neon_reg(operands, 1)?;

    let (q, size) = neon_arr_to_q_size(&arr_n)?;

    // 0 Q U 01110 size 11000 opcode 10 Rn Rd
    let word = (q << 30) | (u_bit << 29) | (0b01110 << 24) | (size << 22) | (0b11000 << 17)
        | (opcode << 12) | (0b10 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode NEON UMOV: move element to GP register
pub(crate) fn encode_neon_umov(operands: &[Operand]) -> Result<EncodeResult, String> {
    if operands.len() < 2 {
        return Err("umov requires 2 operands".to_string());
    }
    let (rd, is_64) = get_reg(operands, 0)?;

    // Second operand should be a RegLane (v0.b[0])
    match operands.get(1) {
        Some(Operand::RegLane { reg, elem_size, index }) => {
            let rn = parse_reg_num(reg).ok_or("invalid NEON register")?;
            let q = if is_64 { 1u32 } else { 0 };

            let imm5 = match elem_size.as_str() {
                "b" => ((*index & 0xF) << 1) | 0b00001,
                "h" => ((*index & 0x7) << 2) | 0b00010,
                "s" => ((*index & 0x3) << 3) | 0b00100,
                "d" => ((*index & 0x1) << 4) | 0b01000,
                _ => return Err(format!("unsupported umov element size: {}", elem_size)),
            };

            // UMOV Rd, Vn.Ts[index]: 0 Q 0 01110 000 imm5 0 0111 1 Rn Rd
            let word = (q << 30) | (0b001110000u32 << 21) | (imm5 << 16)
                | (0b001111 << 10) | (rn << 5) | rd;
            Ok(EncodeResult::Word(word))
        }
        _ => Err("umov: expected register lane operand".to_string()),
    }
}

/// Encode NEON DUP: broadcast GP register to all vector lanes
pub(crate) fn encode_neon_dup(operands: &[Operand]) -> Result<EncodeResult, String> {
    if operands.len() < 2 {
        return Err("dup requires 2 operands".to_string());
    }
    let (rd, arr_d) = get_neon_reg(operands, 0)?;

    // DUP Vd.T, Rn (general form - broadcast GP reg to vector)
    if let Some(Operand::Reg(rn_name)) = operands.get(1) {
        let rn = parse_reg_num(rn_name).ok_or("invalid rn")?;
        let (q, _) = neon_arr_to_q_size(&arr_d)?;

        // imm5 encoding for element size:
        // .8b/.16b: imm5 = 00001
        // .4h/.8h:  imm5 = 00010
        // .2s/.4s:  imm5 = 00100
        // .2d:      imm5 = 01000
        let imm5 = match arr_d.as_str() {
            "8b" | "16b" => 0b00001u32,
            "4h" | "8h" => 0b00010,
            "2s" | "4s" => 0b00100,
            "2d" => 0b01000,
            _ => return Err(format!("unsupported dup arrangement: {}", arr_d)),
        };

        // DUP Vd.T, Rn: 0 Q 0 01110 000 imm5 0 0001 1 Rn Rd
        let word = (q << 30) | (0b001110000u32 << 21) | (imm5 << 16)
            | (0b000011 << 10) | (rn << 5) | rd;
        return Ok(EncodeResult::Word(word));
    }

    // DUP Vd.T, Vn.Ts[index] (broadcast element to all lanes)
    if let Some(Operand::RegLane { reg, elem_size, index }) = operands.get(1) {
        let rn = parse_reg_num(reg).ok_or("invalid NEON register")?;
        let (q, _) = neon_arr_to_q_size(&arr_d)?;

        // imm5 encodes both element size and index:
        // .b[i]: imm5 = (i << 1) | 0b00001
        // .h[i]: imm5 = (i << 2) | 0b00010
        // .s[i]: imm5 = (i << 3) | 0b00100
        // .d[i]: imm5 = (i << 4) | 0b01000
        let imm5 = match elem_size.as_str() {
            "b" => ((*index & 0xF) << 1) | 0b00001,
            "h" => ((*index & 0x7) << 2) | 0b00010,
            "s" => ((*index & 0x3) << 3) | 0b00100,
            "d" => ((*index & 0x1) << 4) | 0b01000,
            _ => return Err(format!("unsupported dup element size: {}", elem_size)),
        };

        // DUP Vd.T, Vn.Ts[i]: 0 Q 0 01110 000 imm5 0 0000 1 Rn Rd
        let word = (q << 30) | (0b001110000u32 << 21) | (imm5 << 16)
            | (0b000001 << 10) | (rn << 5) | rd;
        return Ok(EncodeResult::Word(word));
    }

    Err("unsupported dup operands".to_string())
}

/// Encode NEON INS (insert element from GP register): INS Vd.Ts[index], Xn
pub(crate) fn encode_neon_ins(operands: &[Operand]) -> Result<EncodeResult, String> {
    if operands.len() < 2 {
        return Err("ins requires 2 operands".to_string());
    }
    match (&operands[0], &operands[1]) {
        // INS Vd.Ts[dst_idx], Xn (general register to element)
        (Operand::RegLane { reg, elem_size, index }, Operand::Reg(rn_name)) => {
            let rd = parse_reg_num(reg).ok_or("invalid NEON register")?;
            let rn = parse_reg_num(rn_name).ok_or("invalid register")?;

            let imm5 = match elem_size.as_str() {
                "b" => ((*index & 0xF) << 1) | 0b00001,
                "h" => ((*index & 0x7) << 2) | 0b00010,
                "s" => ((*index & 0x3) << 3) | 0b00100,
                "d" => ((*index & 0x1) << 4) | 0b01000,
                _ => return Err(format!("unsupported ins element size: {}", elem_size)),
            };

            // INS Vd.Ts[i], Xn: 0 1 0 01110 000 imm5 0 0011 1 Rn Rd
            let word = (0b01001110000u32 << 21) | (imm5 << 16)
                | (0b000111 << 10) | (rn << 5) | rd;
            Ok(EncodeResult::Word(word))
        }
        // INS Vd.Ts[dst_idx], Vn.Ts[src_idx] (element to element)
        (Operand::RegLane { reg: rd_name, elem_size: dst_size, index: dst_idx },
         Operand::RegLane { reg: rn_name, elem_size: _src_size, index: src_idx }) => {
            let rd = parse_reg_num(rd_name).ok_or("invalid NEON rd")?;
            let rn = parse_reg_num(rn_name).ok_or("invalid NEON rn")?;

            let (imm5, imm4) = match dst_size.as_str() {
                "b" => (
                    ((*dst_idx & 0xF) << 1) | 0b00001,
                    *src_idx & 0xF,
                ),
                "h" => (
                    ((*dst_idx & 0x7) << 2) | 0b00010,
                    (*src_idx & 0x7) << 1,
                ),
                "s" => (
                    ((*dst_idx & 0x3) << 3) | 0b00100,
                    (*src_idx & 0x3) << 2,
                ),
                "d" => (
                    ((*dst_idx & 0x1) << 4) | 0b01000,
                    (*src_idx & 0x1) << 3,
                ),
                _ => return Err(format!("unsupported ins element size: {}", dst_size)),
            };

            // INS Vd.Ts[dst], Vn.Ts[src]: 0 1 1 01110 000 imm5 0 imm4 1 Rn Rd
            let word = (0b01101110000u32 << 21) | (imm5 << 16)
                | (imm4 << 11) | (1 << 10) | (rn << 5) | rd;
            Ok(EncodeResult::Word(word))
        }
        _ => Err("ins: expected (RegLane, Reg) or (RegLane, RegLane) operands".to_string()),
    }
}

/// Encode NEON NOT (bitwise NOT): NOT Vd.T, Vn.T
pub(crate) fn encode_neon_not(operands: &[Operand]) -> Result<EncodeResult, String> {
    if operands.len() < 2 {
        return Err("not requires 2 operands".to_string());
    }
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;

    let q: u32 = if arr_d == "16b" { 1 } else { 0 };

    // NOT Vd.T, Vn.T (alias of MVN): 0 Q 1 01110 00 10000 00101 10 Rn Rd
    let word = ((q << 30) | (1 << 29) | (0b01110 << 24))
        | (0b10000 << 17) | (0b00101 << 12) | (0b10 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode NEON MOVI (move immediate to vector)
pub(crate) fn encode_neon_movi(operands: &[Operand]) -> Result<EncodeResult, String> {
    if operands.len() < 2 {
        return Err("movi requires 2 operands".to_string());
    }
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let imm = get_imm(operands, 1)?;

    match arr_d.as_str() {
        "16b" | "8b" => {
            // MOVI Vd.16b, #imm8
            // Encoding: 0 Q 00 1111 00000 abc 1110 01 defgh Rd
            // where imm8 = abc:defgh
            let q: u32 = if arr_d == "16b" { 1 } else { 0 };
            let imm8 = imm as u32 & 0xFF;
            let abc = (imm8 >> 5) & 0x7;
            let defgh = imm8 & 0x1F;
            // 0 Q op 0 1111 0 a b c cmode(1110) o2(0) 1 defgh Rd
            let word = (q << 30) | (0b0011110 << 23) | ((abc >> 2) << 18) | (((abc >> 1) & 1) << 17)
                | ((abc & 1) << 16) | (0b1110 << 12) | (0b01 << 10) | (defgh << 5) | rd;
            Ok(EncodeResult::Word(word))
        }
        "2d" => {
            // MOVI Vd.2d, #imm
            // The 64-bit immediate is encoded as 8 bits, where each bit expands
            // to 8 bits (0x00 or 0xFF) in the result.
            // Convert the 64-bit value to the 8-bit encoding.
            let imm64 = imm as u64;
            let mut imm8 = 0u32;
            for i in 0..8 {
                let byte_val = (imm64 >> (i * 8)) & 0xFF;
                if byte_val == 0xFF {
                    imm8 |= 1 << i;
                } else if byte_val != 0 {
                    return Err(format!("movi .2d: each byte of immediate must be 0x00 or 0xFF, got 0x{:02x} at byte {}", byte_val, i));
                }
            }
            let abc = (imm8 >> 5) & 0x7;
            let defgh = imm8 & 0x1F;
            // MOVI Vd.2d, #imm: 0 1 1 0 1111 00 abc 1110 01 defgh Rd  (op=1, Q=1)
            let word = (0b01101111 << 24) | ((abc >> 2) << 18) | (((abc >> 1) & 1) << 17)
                | ((abc & 1) << 16) | (0b111001 << 10) | (defgh << 5) | rd;
            Ok(EncodeResult::Word(word))
        }
        "2s" | "4s" => {
            // MOVI Vd.2s/4s, #imm8 (32-bit element, no shift)
            // Encoding: 0 Q op(0) 0 1111 0 abc cmode(0000) o2(0) 1 defgh Rd
            let q: u32 = if arr_d == "4s" { 1 } else { 0 };
            let imm8 = imm as u32 & 0xFF;
            let abc = (imm8 >> 5) & 0x7;
            let defgh = imm8 & 0x1F;

            // Check for optional LSL shift operand
            let (cmode, shift_val) = if operands.len() > 2 {
                if let Some(Operand::Shift { kind, amount }) = operands.get(2) {
                    if kind == "lsl" {
                        match amount {
                            0 => (0b0000u32, 0),
                            8 => (0b0010, 8),
                            16 => (0b0100, 16),
                            24 => (0b0110, 24),
                            _ => return Err(format!("movi: unsupported shift amount: {}", amount)),
                        }
                    } else {
                        (0b0000, 0)
                    }
                } else {
                    (0b0000, 0)
                }
            } else {
                (0b0000, 0)
            };
            let _ = shift_val;

            let word = (q << 30) | (0b0011110 << 23) | ((abc >> 2) << 18) | (((abc >> 1) & 1) << 17)
                | ((abc & 1) << 16) | (cmode << 12) | (0b01 << 10) | (defgh << 5) | rd;
            Ok(EncodeResult::Word(word))
        }
        "4h" | "8h" => {
            // MOVI Vd.4h/8h, #imm8
            let q: u32 = if arr_d == "8h" { 1 } else { 0 };
            let imm8 = imm as u32 & 0xFF;
            let abc = (imm8 >> 5) & 0x7;
            let defgh = imm8 & 0x1F;
            // cmode=1000 for .4h/.8h with no shift
            let word = (q << 30) | (0b0011110 << 23) | ((abc >> 2) << 18) | (((abc >> 1) & 1) << 17)
                | ((abc & 1) << 16) | (0b1000 << 12) | (0b01 << 10) | (defgh << 5) | rd;
            Ok(EncodeResult::Word(word))
        }
        _ => Err(format!("movi: unsupported arrangement: {}", arr_d)),
    }
}


/// Encode NEON BIC (bitwise clear vector): BIC Vd.T, Vn.T, Vm.T
pub(crate) fn encode_neon_bic(operands: &[Operand]) -> Result<EncodeResult, String> {
    if operands.len() < 3 {
        return Err("bic requires 3 operands".to_string());
    }
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;
    let (rm, _) = get_neon_reg(operands, 2)?;

    let q: u32 = if arr_d == "16b" { 1 } else { 0 };

    // BIC Vd.T, Vn.T, Vm.T: 0 Q 0 01110 01 1 Rm 000111 Rn Rd
    let word = (q << 30) | (0b001110 << 24) | (0b01 << 22) | (1 << 21)
        | (rm << 16) | (0b000111 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode NEON BSL (bitwise select): BSL Vd.T, Vn.T, Vm.T
pub(crate) fn encode_neon_bsl(operands: &[Operand]) -> Result<EncodeResult, String> {
    if operands.len() < 3 {
        return Err("bsl requires 3 operands".to_string());
    }
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;
    let (rm, _) = get_neon_reg(operands, 2)?;

    let q: u32 = if arr_d == "16b" { 1 } else { 0 };

    // BSL Vd.T, Vn.T, Vm.T: 0 Q 1 01110 01 1 Rm 000111 Rn Rd
    let word = (q << 30) | (1 << 29) | (0b01110 << 24) | (0b01 << 22) | (1 << 21)
        | (rm << 16) | (0b000111 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode NEON REV64: reverse elements within 64-bit doublewords
pub(crate) fn encode_neon_rev64(operands: &[Operand]) -> Result<EncodeResult, String> {
    if operands.len() < 2 {
        return Err("rev64 requires 2 operands".to_string());
    }
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;

    let (q, size) = neon_arr_to_q_size(&arr_d)?;

    // REV64 Vd.T, Vn.T: 0 Q 0 01110 size 10 0000 0000 10 Rn Rd
    let word = (q << 30) | (0b001110 << 24) | (size << 22)
        | (0b100000 << 16) | (0b000010 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode NEON TBL: table vector lookup
pub(crate) fn encode_neon_tbl(operands: &[Operand]) -> Result<EncodeResult, String> {
    // TBL Vd.T, {Vn.T}, Vm.T  (single register table)
    // TBL Vd.T, {Vn.T, Vn+1.T}, Vm.T  (two register table)
    // etc.
    if operands.len() < 3 {
        return Err("tbl requires 3 operands".to_string());
    }
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let q: u32 = if arr_d == "16b" { 1 } else { 0 };

    // The second operand is a register list
    let (rn, num_regs) = match &operands[1] {
        Operand::RegList(regs) => {
            let first_reg = match &regs[0] {
                Operand::RegArrangement { reg, .. } => parse_reg_num(reg).ok_or("invalid reg")?,
                Operand::Reg(name) => parse_reg_num(name).ok_or("invalid reg")?,
                _ => return Err("tbl: expected register in list".to_string()),
            };
            (first_reg, regs.len() as u32)
        }
        _ => return Err("tbl: expected register list as second operand".to_string()),
    };

    let (rm, _) = get_neon_reg(operands, 2)?;

    // len field: 1 reg -> 00, 2 -> 01, 3 -> 10, 4 -> 11
    let len = (num_regs - 1) & 0x3;

    // TBL: 0 Q 00 1110 000 Rm 0 len 0 00 Rn Rd
    let word = ((((q << 30) | (0b001110 << 24))
        | (rm << 16)) | (len << 13)) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode NEON TBX: table vector lookup with insert (preserves out-of-range lanes)
pub(crate) fn encode_neon_tbx(operands: &[Operand]) -> Result<EncodeResult, String> {
    if operands.len() < 3 {
        return Err("tbx requires 3 operands".to_string());
    }
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let q: u32 = if arr_d == "16b" { 1 } else { 0 };

    let (rn, num_regs) = match &operands[1] {
        Operand::RegList(regs) => {
            let first_reg = match &regs[0] {
                Operand::RegArrangement { reg, .. } => parse_reg_num(reg).ok_or("invalid reg")?,
                Operand::Reg(name) => parse_reg_num(name).ok_or("invalid reg")?,
                _ => return Err("tbx: expected register in list".to_string()),
            };
            (first_reg, regs.len() as u32)
        }
        _ => return Err("tbx: expected register list as second operand".to_string()),
    };

    let (rm, _) = get_neon_reg(operands, 2)?;
    let len = (num_regs - 1) & 0x3;

    // TBX: 0 Q 00 1110 000 Rm 0 len 1 00 Rn Rd (op=1 for TBX vs op=0 for TBL)
    let word = (q << 30) | (0b001110 << 24) | (rm << 16) | (len << 13)
        | (1 << 12) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode NEON LD1R: load single structure and replicate to all lanes
pub(crate) fn encode_neon_ld1r(operands: &[Operand]) -> Result<EncodeResult, String> {
    // LD1R {Vt.T}, [Xn]
    if operands.len() < 2 {
        return Err("ld1r requires 2 operands".to_string());
    }

    let (rt, arr) = match &operands[0] {
        Operand::RegList(regs) => {
            if regs.len() != 1 {
                return Err("ld1r expects exactly one register in list".to_string());
            }
            match &regs[0] {
                Operand::RegArrangement { reg, arrangement } => {
                    let num = parse_reg_num(reg).ok_or("invalid reg")?;
                    (num, arrangement.clone())
                }
                _ => return Err("ld1r: expected register with arrangement".to_string()),
            }
        }
        _ => return Err("ld1r: expected register list as first operand".to_string()),
    };

    let (q, size) = match arr.as_str() {
        "8b"  => (0u32, 0b00u32),
        "16b" => (1, 0b00),
        "4h"  => (0, 0b01),
        "8h"  => (1, 0b01),
        "2s"  => (0, 0b10),
        "4s"  => (1, 0b10),
        "1d"  => (0, 0b11),
        "2d"  => (1, 0b11),
        _ => return Err(format!("ld1r: unsupported arrangement: {}", arr)),
    };

    match &operands[1] {
        Operand::Mem { base, offset: 0 } => {
            let rn = parse_reg_num(base).ok_or("invalid base reg")?;
            // LD1R: 0 Q 0 01101 0 1 0 00000 110 0 size Rn Rt (no post-index)
            let word = (q << 30) | (0b001101 << 24) | (1 << 22) | (0b110 << 13)
                | (size << 10) | (rn << 5) | rt;
            Ok(EncodeResult::Word(word))
        }
        Operand::MemPostIndex { base, offset } => {
            let rn = parse_reg_num(base).ok_or("invalid base reg")?;
            // LD1R post-index (immediate): 0 Q 0 01101 1 1 0 11111 110 0 size Rn Rt
            // Rm=11111 means post-index by element size
            let _ = offset; // offset must match element size, not encoded separately
            let word = (q << 30) | (0b001101 << 24) | (1 << 23) | (1 << 22)
                | (0b11111 << 16) | (0b110 << 13) | (size << 10) | (rn << 5) | rt;
            Ok(EncodeResult::Word(word))
        }
        _ => Err("ld1r: expected [Xn] or [Xn], #imm memory operand".to_string()),
    }
}

/// Encode NEON LD1 (vector load, multiple structures)
/// Dispatch LD/ST1-4: choose between "multiple structures" and "single structure (element)" encoding.
pub(crate) fn encode_neon_ld_st_dispatch(operands: &[Operand], is_load: bool, num_structs: u32) -> Result<EncodeResult, String> {
    // If the first operand is a RegListIndexed, use single-element encoding
    if let Some(Operand::RegListIndexed { .. }) = operands.first() {
        return encode_neon_ld_st_single(operands, is_load, num_structs);
    }
    // Multiple-structures encoding for ld1-4/st1-4
    encode_neon_ld_st_multi(operands, is_load, num_structs)
}

/// Encode NEON LD/ST single structure (element):
/// st1 {v0.s}[0], [x3]
/// st2 {v0.s, v1.s}[0], [x3]
/// st4 {v0.s, v1.s, v2.s, v3.s}[0], [x3]
/// ld2 {v0.s, v1.s}[0], [x3]
// TODO: add post-index form [Xn], #imm
pub(crate) fn encode_neon_ld_st_single(operands: &[Operand], is_load: bool, num_structs: u32) -> Result<EncodeResult, String> {
    if operands.len() < 2 {
        return Err(format!("ld/st{} single element requires at least 2 operands", num_structs));
    }

    let (regs, index) = match &operands[0] {
        Operand::RegListIndexed { regs, index } => (regs, *index),
        _ => return Err("expected register list with index".to_string()),
    };

    if regs.len() as u32 != num_structs {
        return Err(format!("expected {} registers in list, got {}", num_structs, regs.len()));
    }
    // TODO: validate that registers in the list are consecutive (ARM ISA requirement)

    // Get element size and first register from the list
    let (rt, elem_size) = match &regs[0] {
        Operand::RegArrangement { reg, arrangement } => {
            (parse_reg_num(reg).ok_or("invalid register in list")?, arrangement.clone())
        }
        _ => return Err("expected register with arrangement in list".to_string()),
    };

    // Get base register and check for post-index
    let (rn, post_index) = match &operands[1] {
        Operand::Mem { base, offset: 0 } => {
            let rn = parse_reg_num(base).ok_or_else(|| format!("invalid base register: {}", base))?;
            // Check for post-index immediate: operands[2] is the post-index offset
            let pi = if operands.len() > 2 {
                match &operands[2] {
                    Operand::Imm(off) => Some(*off),
                    _ => None,
                }
            } else {
                None
            };
            (rn, pi)
        }
        Operand::MemPostIndex { base, offset } => {
            let rn = parse_reg_num(base).ok_or_else(|| format!("invalid base register: {}", base))?;
            (rn, Some(*offset))
        }
        _ => return Err("expected [Xn] memory operand".to_string()),
    };

    let l_bit = if is_load { 1u32 } else { 0u32 };

    // R bit: 0 for 1,3 registers; 1 for 2,4 registers
    let r_bit = match num_structs {
        1 | 3 => 0u32,
        2 | 4 => 1u32,
        _ => return Err(format!("unsupported struct count: {}", num_structs)),
    };

    // Compute opcode, S, Q, size based on element size and index
    let (opcode, s_bit, q_bit, size_field) = match elem_size.as_str() {
        "b" => {
            // opcode = 000 (1,2 regs) or 001 (3,4 regs)
            let base_opc = if num_structs <= 2 { 0b000u32 } else { 0b001u32 };
            // index bits: Q:S:size[1]:size[0] = 4 bits for 0-15
            let q = (index >> 3) & 1;
            let s = (index >> 2) & 1;
            let sz = index & 3;
            (base_opc, s, q, sz)
        }
        "h" => {
            let base_opc = if num_structs <= 2 { 0b010u32 } else { 0b011u32 };
            // index bits: Q:S:size[1] = 3 bits for 0-7, size[0]=0
            let q = (index >> 2) & 1;
            let s = (index >> 1) & 1;
            let sz = (index & 1) << 1;
            (base_opc, s, q, sz)
        }
        "s" => {
            let base_opc = if num_structs <= 2 { 0b100u32 } else { 0b101u32 };
            // index bits: Q:S = 2 bits for 0-3, size=00
            let q = (index >> 1) & 1;
            let s = index & 1;
            (base_opc, s, q, 0b00u32)
        }
        "d" => {
            let base_opc = if num_structs <= 2 { 0b100u32 } else { 0b101u32 };
            // index bits: Q = 1 bit for 0-1, S=0, size=01
            let q = index & 1;
            (base_opc, 0u32, q, 0b01u32)
        }
        _ => return Err(format!("unsupported element size for ld/st single: {}", elem_size)),
    };

    if let Some(_offset) = post_index {
        // Post-index form: Q 0011011 L R 11111 opcode S size Rn Rt
        // (Rm=11111 means immediate post-index, the amount is implicit from element size)
        let word = (q_bit << 30) | (0b0011011 << 23) | (l_bit << 22) | (r_bit << 21)
            | (0b11111 << 16) | (opcode << 13) | (s_bit << 12) | (size_field << 10) | (rn << 5) | rt;
        Ok(EncodeResult::Word(word))
    } else {
        // No post-index: Q 0011010 L R 00000 opcode S size Rn Rt
        let word = (q_bit << 30) | (0b0011010 << 23) | (l_bit << 22) | (r_bit << 21)
            | (opcode << 13) | (s_bit << 12) | (size_field << 10) | (rn << 5) | rt;
        Ok(EncodeResult::Word(word))
    }
}

/// Common encoder for LD1/ST1 (multiple structures)
pub(crate) fn encode_neon_ld_st_multi(operands: &[Operand], is_load: bool, num_structs: u32) -> Result<EncodeResult, String> {
    if operands.len() < 2 {
        return Err(format!("ld{}/st{} requires at least 2 operands", num_structs, num_structs));
    }

    // First operand: register list {Vt.T} or {Vt.T, Vt+1.T, ...}
    let (rt, arr, num_regs) = match &operands[0] {
        Operand::RegList(regs) => {
            let (first_reg, arrangement) = match &regs[0] {
                Operand::RegArrangement { reg, arrangement } => {
                    (parse_reg_num(reg).ok_or("invalid reg")?, arrangement.clone())
                }
                _ => return Err(format!("ld{}/st{}: expected RegArrangement in list", num_structs, num_structs)),
            };
            (first_reg, arrangement, regs.len() as u32)
        }
        _ => return Err(format!("ld{}/st{}: expected register list", num_structs, num_structs)),
    };

    let (q, size) = neon_arr_to_q_size(&arr)?;

    // Second operand: [Xn] memory base or [Xn], #imm (post-index, merged by parser)
    let (rn, post_index) = match &operands[1] {
        Operand::Mem { base, offset: 0 } => {
            let r = parse_reg_num(base).ok_or_else(|| format!("invalid base register: {}", base))?;
            (r, None)
        }
        Operand::MemPostIndex { base, offset } => {
            let r = parse_reg_num(base).ok_or_else(|| format!("invalid base register: {}", base))?;
            (r, Some(*offset))
        }
        _ => return Err(format!("ld{}/st{}: expected [Xn] memory operand", num_structs, num_structs)),
    };

    // opcode field based on structure count and number of registers:
    // LD1/ST1: 1 reg=0111, 2 reg=1010, 3 reg=0110, 4 reg=0010
    // LD2/ST2: 2 reg=1000
    // LD3/ST3: 3 reg=0100
    // LD4/ST4: 4 reg=0000
    let opcode = match num_structs {
        1 => match num_regs {
            1 => 0b0111u32,
            2 => 0b1010,
            3 => 0b0110,
            4 => 0b0010,
            _ => return Err(format!("ld1/st1: unsupported register count: {}", num_regs)),
        },
        2 => 0b1000u32,
        3 => 0b0100,
        4 => 0b0000,
        _ => return Err(format!("unsupported structure count: {}", num_structs)),
    };

    let l_bit = if is_load { 1u32 } else { 0u32 };

    // Handle post-index form from merged MemPostIndex
    if let Some(_imm) = post_index {
        // Post-index with immediate: use Rm=11111 (0x1F)
        let word = ((q << 30) | (0b001100 << 24) | (1 << 23) | (l_bit << 22)) | (0b11111 << 16) | (opcode << 12) | (size << 10) | (rn << 5) | rt;
        return Ok(EncodeResult::Word(word));
    }

    // Check for post-index form via separate operands: [Xn], Xm
    if operands.len() > 2 {
        match &operands[2] {
            Operand::Imm(_) => {
                // Post-index with immediate: use Rm=11111
                let word = ((q << 30) | (0b001100 << 24) | (1 << 23) | (l_bit << 22)) | (0b11111 << 16) | (opcode << 12) | (size << 10) | (rn << 5) | rt;
                return Ok(EncodeResult::Word(word));
            }
            Operand::Reg(rm_name) => {
                let rm = parse_reg_num(rm_name).ok_or("invalid rm")?;
                let word = ((q << 30) | (0b001100 << 24) | (1 << 23) | (l_bit << 22)) | (rm << 16) | (opcode << 12) | (size << 10) | (rn << 5) | rt;
                return Ok(EncodeResult::Word(word));
            }
            _ => {}
        }
    }

    // No post-index: LD1/ST1 {Vt.T...}, [Xn]
    // 0 Q 001100 0 L 0 00000 opcode size Rn Rt
    let word = (((q << 30) | (0b001100 << 24)) | (l_bit << 22)) | (opcode << 12) | (size << 10) | (rn << 5) | rt;
    Ok(EncodeResult::Word(word))
}

/// Encode NEON UZP1/UZP2/ZIP1/ZIP2
pub(crate) fn encode_neon_zip_uzp(operands: &[Operand], op_bits: u32, _is_zip: bool) -> Result<EncodeResult, String> {
    if operands.len() < 3 {
        return Err("uzp/zip requires 3 operands".to_string());
    }
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;
    let (rm, _) = get_neon_reg(operands, 2)?;
    let (q, size) = neon_arr_to_q_size(&arr_d)?;

    // UZP1: 0 Q 0 01110 size 0 Rm 0 001 10 Rn Rd  (op_bits=001)
    // UZP2: 0 Q 0 01110 size 0 Rm 0 101 10 Rn Rd  (op_bits=101)
    // ZIP1: 0 Q 0 01110 size 0 Rm 0 011 10 Rn Rd  (op_bits=011)
    // ZIP2: 0 Q 0 01110 size 0 Rm 0 111 10 Rn Rd  (op_bits=111)
    let word = (((q << 30) | (0b001110 << 24) | (size << 22)) | (rm << 16)) | (op_bits << 12) | (0b10 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode NEON EOR3 (three-way XOR, SHA3 extension): EOR3 Vd.16b, Vn.16b, Vm.16b, Vk.16b
pub(crate) fn encode_neon_eor3(operands: &[Operand]) -> Result<EncodeResult, String> {
    if operands.len() < 4 {
        return Err("eor3 requires 4 operands".to_string());
    }
    let (rd, _) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;
    let (rm, _) = get_neon_reg(operands, 2)?;
    let (rk, _) = get_neon_reg(operands, 3)?;

    // EOR3 Vd.16b, Vn.16b, Vm.16b, Vk.16b
    // Encoding: 11001110 000 Rm 0 Rk(4:0) 00 Rn Rd
    let word = ((0b11001110u32 << 24) | (rm << 16)) | (rk << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode NEON PMULL/PMULL2 (polynomial multiply long)
pub(crate) fn encode_neon_pmull(operands: &[Operand], is_pmull2: bool) -> Result<EncodeResult, String> {
    if operands.len() < 3 {
        return Err("pmull requires 3 operands".to_string());
    }
    let (rd, _) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;
    let (rm, _) = get_neon_reg(operands, 2)?;

    let q = if is_pmull2 { 1u32 } else { 0 };

    // PMULL  Vd.1q, Vn.1d, Vm.1d: 0 0 00 1110 11 1 Rm 11100 0 Rn Rd  (size=11)
    // PMULL2 Vd.1q, Vn.2d, Vm.2d: 0 1 00 1110 11 1 Rm 11100 0 Rn Rd
    let word = ((q << 30) | (0b001110 << 24) | (0b11 << 22) | (1 << 21)
        | (rm << 16) | (0b11100 << 11)) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode NEON AES instructions (AESE, AESD, AESMC, AESIMC)
pub(crate) fn encode_neon_aes(operands: &[Operand], opcode: u32) -> Result<EncodeResult, String> {
    if operands.len() < 2 {
        return Err("aes instruction requires 2 operands".to_string());
    }
    let (rd, _) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;

    // AES instructions: 0100 1110 0010 1000 opcode 10 Rn Rd
    // AESE:  opcode = 00100 (0x4)
    // AESD:  opcode = 00101 (0x5)
    // AESMC: opcode = 00110 (0x6)
    // AESIMC:opcode = 00111 (0x7)
    let word = (0b01001110 << 24) | (0b0010100 << 17) | (opcode << 12)
        | (0b10 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode NEON ADD/SUB (vector integer): ADD/SUB Vd.T, Vn.T, Vm.T
pub(crate) fn encode_neon_add_sub(operands: &[Operand], is_sub: bool) -> Result<EncodeResult, String> {
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;
    let (rm, _) = get_neon_reg(operands, 2)?;
    let (q, size) = neon_arr_to_q_size(&arr_d)?;
    let u = if is_sub { 1u32 } else { 0u32 };

    // ADD: 0 Q 0 01110 size 1 Rm 10000 1 Rn Rd
    // SUB: 0 Q 1 01110 size 1 Rm 10000 1 Rn Rd
    let word = (q << 30) | (u << 29) | (0b01110 << 24) | (size << 22) | (1 << 21)
        | (rm << 16) | (0b10000 << 11) | (1 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode NEON USHR (unsigned shift right immediate)
pub(crate) fn encode_neon_ushr(operands: &[Operand]) -> Result<EncodeResult, String> {
    if operands.len() < 3 {
        return Err("ushr requires 3 operands".to_string());
    }
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;
    let shift = get_imm(operands, 2)? as u32;

    let (q, _) = neon_arr_to_q_size(&arr_d)?;

    // USHR Vd.T, Vn.T, #shift
    // 0 Q 1 0 11110 immh:immb 000001 Rn Rd
    let immh_immb = match arr_d.as_str() {
        "8b" | "16b" => (16 - shift) & 0xF,
        "4h" | "8h" => (32 - shift) & 0x1F,
        "2s" | "4s" => (64 - shift) & 0x3F,
        "2d" => (128 - shift) & 0x7F,
        _ => return Err(format!("unsupported ushr arrangement: {}", arr_d)),
    };

    let word = (q << 30) | (1 << 29) | (0b011110 << 23) | (immh_immb << 16)
        | (0b000001 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode NEON SSHR (signed shift right immediate)
pub(crate) fn encode_neon_sshr(operands: &[Operand]) -> Result<EncodeResult, String> {
    if operands.len() < 3 {
        return Err("sshr requires 3 operands".to_string());
    }
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;
    let shift = get_imm(operands, 2)? as u32;

    let (q, _) = neon_arr_to_q_size(&arr_d)?;

    // SSHR Vd.T, Vn.T, #shift
    // 0 Q 0 0 11110 immh:immb 000001 Rn Rd  (U=0)
    let immh_immb = match arr_d.as_str() {
        "8b" | "16b" => (16 - shift) & 0xF,
        "4h" | "8h" => (32 - shift) & 0x1F,
        "2s" | "4s" => (64 - shift) & 0x3F,
        "2d" => (128 - shift) & 0x7F,
        _ => return Err(format!("unsupported sshr arrangement: {}", arr_d)),
    };

    let word = (q << 30) | (0b011110 << 23) | (immh_immb << 16)
        | (0b000001 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode NEON SHL (shift left immediate)
pub(crate) fn encode_neon_shl(operands: &[Operand]) -> Result<EncodeResult, String> {
    if operands.len() < 3 {
        return Err("shl requires 3 operands".to_string());
    }
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;
    let shift = get_imm(operands, 2)? as u32;

    let (q, _) = neon_arr_to_q_size(&arr_d)?;

    // SHL Vd.T, Vn.T, #shift
    // 0 Q 0 0 11110 immh:immb 010101 Rn Rd
    // immh:immb = element_size + shift
    let immh_immb = match arr_d.as_str() {
        "8b" | "16b" => (8 + shift) & 0xF,
        "4h" | "8h" => (16 + shift) & 0x1F,
        "2s" | "4s" => (32 + shift) & 0x3F,
        "2d" => (64 + shift) & 0x7F,
        _ => return Err(format!("unsupported shl arrangement: {}", arr_d)),
    };

    let word = (q << 30) | (0b011110 << 23) | (immh_immb << 16)
        | (0b010101 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode NEON SLI (shift left and insert)
pub(crate) fn encode_neon_sli(operands: &[Operand]) -> Result<EncodeResult, String> {
    if operands.len() < 3 {
        return Err("sli requires 3 operands".to_string());
    }
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;
    let shift = get_imm(operands, 2)? as u32;

    let (q, _) = neon_arr_to_q_size(&arr_d)?;

    // SLI Vd.T, Vn.T, #shift
    // 0 Q 1 0 11110 immh:immb 010101 Rn Rd  (U=1)
    let immh_immb = match arr_d.as_str() {
        "8b" | "16b" => (8 + shift) & 0xF,
        "4h" | "8h" => (16 + shift) & 0x1F,
        "2s" | "4s" => (32 + shift) & 0x3F,
        "2d" => (64 + shift) & 0x7F,
        _ => return Err(format!("unsupported sli arrangement: {}", arr_d)),
    };

    let word = (q << 30) | (1 << 29) | (0b011110 << 23) | (immh_immb << 16)
        | (0b010101 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode SRI (Shift Right and Insert) immediate.
/// SRI Vd.T, Vn.T, #shift: 0 Q 1 0 11110 immh:immb 010001 Rn Rd  (U=1)
pub(crate) fn encode_neon_sri(operands: &[Operand]) -> Result<EncodeResult, String> {
    if operands.len() < 3 {
        return Err("sri requires 3 operands".to_string());
    }
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;
    let shift = get_imm(operands, 2)? as u32;

    let (q, _) = neon_arr_to_q_size(&arr_d)?;

    // immh:immb = (2*esize - shift) for right shift
    let immh_immb = match arr_d.as_str() {
        "8b" | "16b" => (16 - shift) & 0xF,
        "4h" | "8h" => (32 - shift) & 0x1F,
        "2s" | "4s" => (64 - shift) & 0x3F,
        "2d" => (128 - shift) & 0x7F,
        _ => return Err(format!("unsupported sri arrangement: {}", arr_d)),
    };

    let word = (q << 30) | (1 << 29) | (0b011110 << 23) | (immh_immb << 16)
        | (0b010001 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

// ── NEON RBIT (vector bit reverse) ───────────────────────────────────────

/// Encode NEON RBIT Vd.T, Vn.T (per-byte bit reversal in each element).
pub(crate) fn encode_neon_rbit(operands: &[Operand]) -> Result<EncodeResult, String> {
    if operands.len() < 2 {
        return Err("neon rbit requires 2 operands".to_string());
    }
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;

    // Only .8b and .16b arrangements are valid for NEON RBIT
    if arr_d != "8b" && arr_d != "16b" {
        return Err(format!("neon rbit: unsupported arrangement .{}, expected .8b or .16b", arr_d));
    }
    let q: u32 = if arr_d == "16b" { 1 } else { 0 };
    // RBIT Vd.T, Vn.T: 0 Q 1 01110 01 10000 00101 10 Rn Rd
    let word = (q << 30) | (1 << 29) | (0b01110 << 24) | (0b01 << 22)
        | (0b10000 << 17) | (0b00101 << 12) | (0b10 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

// ── NEON MVNI (move NOT immediate) ───────────────────────────────────────

/// Encode NEON MVNI Vd.T, #imm (move bitwise NOT immediate to vector).
pub(crate) fn encode_neon_mvni(operands: &[Operand]) -> Result<EncodeResult, String> {
    if operands.len() < 2 {
        return Err("mvni requires 2 operands".to_string());
    }
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let imm = get_imm(operands, 1)?;
    let imm8 = imm as u32 & 0xFF;

    // Extract abc:defgh for encoding
    let abc = (imm8 >> 5) & 0x7;
    let defgh = imm8 & 0x1f;

    match arr_d.as_str() {
        "2s" | "4s" => {
            let q: u32 = if arr_d == "4s" { 1 } else { 0 };
            // Check for optional shift
            let cmode = if let Some(Operand::Shift { kind, amount }) = operands.get(2) {
                if kind.to_lowercase() == "lsl" {
                    match *amount {
                        0 => 0b0000u32,
                        8 => 0b0010,
                        16 => 0b0100,
                        24 => 0b0110,
                        _ => return Err(format!("mvni: unsupported shift amount: {}", amount)),
                    }
                } else if kind.to_lowercase() == "msl" {
                    match *amount {
                        8 => 0b1100u32,
                        16 => 0b1101,
                        _ => return Err(format!("mvni: unsupported MSL shift: {}", amount)),
                    }
                } else {
                    0b0000
                }
            } else {
                0b0000
            };
            // MVNI: 0 Q 1 0 1111 00 abc cmode 01 defgh Rd  (op=1)
            let word = (q << 30) | (1 << 29) | (0b0111100 << 22)
                | (abc << 16) | (cmode << 12) | (0b01 << 10) | (defgh << 5) | rd;
            Ok(EncodeResult::Word(word))
        }
        "4h" | "8h" => {
            let q: u32 = if arr_d == "8h" { 1 } else { 0 };
            // MVNI 16-bit: cmode=1000, op=1
            let word = (q << 30) | (1 << 29) | (0b0111100 << 22)
                | (abc << 16) | (0b1000 << 12) | (0b01 << 10) | (defgh << 5) | rd;
            Ok(EncodeResult::Word(word))
        }
        _ => Err(format!("mvni: unsupported arrangement: {}", arr_d)),
    }
}

// ── NEON float three-same ────────────────────────────────────────────────
/// Encode NEON float three-same: FADD, FSUB, FMUL, FDIV, FMLA, FMLS, etc.
/// Format: 0 Q U 01110 size 1 Rm opcode 1 Rn Rd
/// size[1]=size_hi (0 or 1), size[0]=sz (0=single, 1=double)
pub(crate) fn encode_neon_float_three_same(operands: &[Operand], u_bit: u32, size_hi: u32, opcode: u32) -> Result<EncodeResult, String> {
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;
    let (rm, _) = get_neon_reg(operands, 2)?;
    let (q, sz) = match arr_d.as_str() {
        "2s" => (0u32, 0u32), "4s" => (1, 0), "2d" => (1, 1),
        _ => return Err(format!("float three-same: unsupported arrangement: {}", arr_d)),
    };
    let size = (size_hi << 1) | sz;
    let word = (q << 30) | (u_bit << 29) | (0b01110 << 24) | (size << 22) | (1 << 21)
        | (rm << 16) | (opcode << 11) | (1 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

// ── NEON two-register misc (integer) ─────────────────────────────────────
/// Encode NEON two-reg misc: ABS, NEG, CLS, CLZ, etc.
/// Format: 0 Q U 01110 size 10000 opcode 10 Rn Rd
pub(crate) fn encode_neon_two_misc(operands: &[Operand], u_bit: u32, opcode: u32) -> Result<EncodeResult, String> {
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;
    let (q, size) = neon_arr_to_q_size(&arr_d)?;
    let word = (q << 30) | (u_bit << 29) | (0b01110 << 24) | (size << 22)
        | (0b10000 << 17) | (opcode << 12) | (0b10 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

// ── NEON float two-register misc ─────────────────────────────────────────
/// Encode NEON float two-reg misc: UCVTF, SCVTF, FCVTZS, FCVTZU, FNEG, FABS, etc. (vector)
/// Format: 0 Q U 01110 size 10000 opcode 10 Rn Rd
/// size[1]=size_hi, size[0]=sz (0=single, 1=double)
pub(crate) fn encode_neon_float_two_misc(operands: &[Operand], u_bit: u32, size_hi: u32, opcode: u32) -> Result<EncodeResult, String> {
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;
    let (q, sz) = match arr_d.as_str() {
        "2s" => (0u32, 0u32), "4s" => (1, 0), "2d" => (1, 1),
        _ => return Err(format!("float two-misc: unsupported arrangement: {}", arr_d)),
    };
    let size = (size_hi << 1) | sz;
    let word = (q << 30) | (u_bit << 29) | (0b01110 << 24) | (size << 22)
        | (0b10000 << 17) | (opcode << 12) | (0b10 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

// ── NEON shift right narrow (SHRN/RSHRN) ─────────────────────────────────
/// Format: 0 Q 0 01111 0 immh immb opcode 1 Rn Rd
/// SHRN opcode=10000, RSHRN opcode=10001
pub(crate) fn encode_neon_shrn(operands: &[Operand], opcode: u32, is_high: bool) -> Result<EncodeResult, String> {
    if operands.len() < 3 { return Err("shrn/rshrn requires 3 operands".to_string()); }
    let (rd, _) = get_neon_reg(operands, 0)?;
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    let shift = get_imm(operands, 2)? as u32;
    let element_bits = match arr_n.as_str() { "8h" => 16u32, "4s" => 32, "2d" => 64,
        _ => return Err(format!("shrn: unsupported source: {}", arr_n)), };
    let half_bits = element_bits / 2;
    if shift == 0 || shift > half_bits { return Err(format!("shrn: shift {} out of range", shift)); }
    let immhb = element_bits - shift;
    let q = if is_high { 1u32 } else { 0 };
    let word = (q << 30) | (0b011110 << 23) | ((immhb >> 3) << 19) | ((immhb & 7) << 16)
        | (opcode << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

// ── NEON shift right accumulate (SSRA/USRA/SRSHR/URSHR) ─────────────────
/// Format: 0 Q U 01111 0 immh immb opcode 1 Rn Rd
pub(crate) fn encode_neon_shift_right(operands: &[Operand], u_bit: u32, opcode: u32) -> Result<EncodeResult, String> {
    if operands.len() < 3 { return Err("shift-right requires 3 operands".to_string()); }
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;
    let shift = get_imm(operands, 2)? as u32;
    let (q, _) = neon_arr_to_q_size(&arr_d)?;
    let element_bits: u32 = match arr_d.as_str() {
        "8b" | "16b" => 8, "4h" | "8h" => 16, "2s" | "4s" => 32, "2d" => 64,
        _ => return Err(format!("shift-right: unsupported: {}", arr_d)), };
    if shift == 0 || shift > element_bits { return Err(format!("shift {} out of range", shift)); }
    let immhb = (element_bits * 2) - shift;
    let word = (q << 30) | (u_bit << 29) | (0b011110 << 23) | ((immhb >> 3) << 19) | ((immhb & 7) << 16)
        | (opcode << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

// ── NEON SSHLL/USHLL (shift left long) ───────────────────────────────────
/// Format: 0 Q U 011110 immh immb 10100 1 Rn Rd
pub(crate) fn encode_neon_shll(operands: &[Operand], u_bit: u32, is_high: bool) -> Result<EncodeResult, String> {
    if operands.len() < 3 { return Err("sshll/ushll requires 3 operands".to_string()); }
    let (rd, _) = get_neon_reg(operands, 0)?;
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    let shift = get_imm(operands, 2)? as u32;
    let base_val = match arr_n.as_str() {
        "8b" | "16b" => 8u32, "4h" | "8h" => 16, "2s" | "4s" => 32,
        _ => return Err(format!("sshll/ushll: unsupported source: {}", arr_n)), };
    let immhb = base_val + shift;
    let q = if is_high { 1u32 } else { 0 };
    let word = (q << 30) | (u_bit << 29) | (0b011110 << 23) | ((immhb >> 3) << 19) | ((immhb & 7) << 16)
        | (0b101001 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

// ── NEON pairwise add (UADDLP/SADDLP/UADALP/SADALP) ────────────────────

// ── NEON three-different extras: UABAL/SABAL/ADDHN/RADDHN/SUBHN/RSUBHN ──
// Already have encode_neon_three_diff which handles these opcodes.

// ── NEON SQXTUN ──────────────────────────────────────────────────────────
// Two-reg misc with U=1, opcode=10010. Reuse encode_neon_two_misc_narrow.

// ── NEON shift right narrow saturating (SQSHRN/UQSHRN/SQRSHRN/UQRSHRN) ─
pub(crate) fn encode_neon_qshrn(operands: &[Operand], u_bit: u32, is_rounding: bool, is_high: bool) -> Result<EncodeResult, String> {
    if operands.len() < 3 { return Err("qshrn requires 3 operands".to_string()); }
    let (rd, _) = get_neon_reg(operands, 0)?;
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    let shift = get_imm(operands, 2)? as u32;
    let element_bits = match arr_n.as_str() { "8h" => 16u32, "4s" => 32, "2d" => 64,
        _ => return Err(format!("qshrn: unsupported source: {}", arr_n)), };
    if shift == 0 || shift > element_bits { return Err(format!("qshrn: shift {} out of range for {}-bit elements", shift, element_bits)); }
    let immhb = element_bits - shift;
    let q = if is_high { 1u32 } else { 0 };
    let opcode_bits: u32 = if is_rounding { 0b100111 } else { 0b100101 };
    let word = (q << 30) | (u_bit << 29) | (0b011110 << 23) | ((immhb >> 3) << 19) | ((immhb & 7) << 16)
        | (opcode_bits << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

// ── NEON ADDHN/RADDHN/SUBHN/RSUBHN ──────────────────────────────────────
/// Three-different narrowing high: Format: 0 Q U 01110 size 1 Rm opcode 00 Rn Rd
pub(crate) fn encode_neon_three_diff_narrow(operands: &[Operand], u_bit: u32, opcode: u32, is_high: bool) -> Result<EncodeResult, String> {
    if operands.len() < 3 { return Err("addhn/subhn requires 3 operands".to_string()); }
    let (rd, _) = get_neon_reg(operands, 0)?;
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    let (rm, _) = get_neon_reg(operands, 2)?;
    let size = match arr_n.as_str() { "8h" => 0b00u32, "4s" => 0b01, "2d" => 0b10,
        _ => return Err(format!("addhn: unsupported source: {}", arr_n)), };
    let q = if is_high { 1u32 } else { 0 };
    let word = (q << 30) | (u_bit << 29) | (0b01110 << 24) | (size << 22) | (1 << 21)
        | (rm << 16) | (opcode << 12) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

// ── NEON LD2R/LD3R/LD4R ──────────────────────────────────────────────────
pub(crate) fn encode_neon_ldnr(operands: &[Operand], num_structs: u32) -> Result<EncodeResult, String> {
    if operands.len() < 2 { return Err(format!("ld{}r requires 2 operands", num_structs)); }
    let (rt, arr, num_regs) = match &operands[0] {
        Operand::RegList(regs) => {
            let (first_reg, arrangement) = match &regs[0] {
                Operand::RegArrangement { reg, arrangement } =>
                    (parse_reg_num(reg).ok_or("invalid reg")?, arrangement.clone()),
                _ => return Err("expected RegArrangement in list".to_string()),
            };
            (first_reg, arrangement, regs.len() as u32)
        }
        _ => return Err("expected register list".to_string()),
    };
    if num_regs != num_structs { return Err(format!("ld{}r: expected {} regs, got {}", num_structs, num_structs, num_regs)); }
    let (q, size) = match arr.as_str() {
        "8b" => (0u32, 0b00u32), "16b" => (1, 0b00),
        "4h" => (0, 0b01), "8h" => (1, 0b01),
        "2s" => (0, 0b10), "4s" => (1, 0b10),
        "1d" => (0, 0b11), "2d" => (1, 0b11),
        _ => return Err(format!("ld{}r: unsupported arrangement: {}", num_structs, arr)),
    };
    // opcode: ld1r=110, ld2r=110(S=1), ld3r=111, ld4r=111(S=1)
    let (opcode, s_bit) = match num_structs {
        1 => (0b110u32, 0u32),
        2 => (0b110, 1),
        3 => (0b111, 0),
        4 => (0b111, 1),
        _ => return Err(format!("unsupported: ld{}r", num_structs)),
    };
    let base = match &operands[1] {
        Operand::Mem { base, .. } => parse_reg_num(base).ok_or("invalid base")?,
        Operand::MemPostIndex { base, .. } => parse_reg_num(base).ok_or("invalid base")?,
        _ => return Err("expected memory operand".to_string()),
    };
    // check for post-index
    let rm = match &operands[1] {
        Operand::MemPostIndex { .. } => 0b11111u32, // immediate post-index
        _ => 0u32,
    };
    let has_post = rm != 0;
    let word = (q << 30) | (0b001101 << 24) | (if has_post { 1u32 } else { 0 } << 23)
        | (1 << 22) | (if has_post { rm } else { 0 } << 16) | (opcode << 13) | (s_bit << 12) | (size << 10) | (base << 5) | rt;
    Ok(EncodeResult::Word(word))
}

// ── NEON float compare-to-zero ───────────────────────────────────────────
/// FCMEQ/FCMLE/FCMLT/FCMGE/FCMGT to zero
/// Format: 0 Q U 01110 size 10000 opcode 10 Rn Rd (float, size = 0sz)
pub(crate) fn encode_neon_float_cmp_zero(operands: &[Operand], u_bit: u32, size_hi: u32, opcode: u32) -> Result<EncodeResult, String> {
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;
    let (q, sz) = match arr_d.as_str() {
        "2s" => (0u32, 0u32), "4s" => (1, 0), "2d" => (1, 1),
        _ => return Err(format!("float cmp zero: unsupported: {}", arr_d)),
    };
    let size = (size_hi << 1) | sz;
    let word = (q << 30) | (u_bit << 29) | (0b01110 << 24) | (size << 22)
        | (0b10000 << 17) | (opcode << 12) | (0b10 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

// ── NEON by-element (non-long) ───────────────────────────────────────────
/// MUL/MLA/MLS by element: 0 Q U 01111 size L M Rm opcode H 0 Rn Rd
pub(crate) fn encode_neon_elem(operands: &[Operand], u_bit: u32, opcode: u32) -> Result<EncodeResult, String> {
    if operands.len() < 3 { return Err("NEON by-element requires 3 operands".to_string()); }
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;
    let (rm, index) = match &operands[2] {
        Operand::RegLane { reg, index, .. } => (parse_reg_num(reg).ok_or("invalid reg")?, *index),
        _ => return Err(format!("expected register lane, got {:?}", operands[2])),
    };
    let (q, size) = neon_arr_to_q_size(&arr_d)?;
    let (h, l, m_bit) = match size {
        0b01 => ((index >> 2) & 1, (index >> 1) & 1, index & 1),
        0b10 => ((index >> 1) & 1, index & 1, (rm >> 4) & 1),
        _ => return Err("unsupported element size for by-element".to_string()),
    };
    let rm_enc = if size == 0b01 { rm & 0xF } else { rm & 0x1F };
    let word = (q << 30) | (u_bit << 29) | (0b01111 << 24) | (size << 22)
        | (l << 21) | (m_bit << 20) | (rm_enc << 16) | (opcode << 12)
        | (h << 11) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

// ── NEON float by-element ────────────────────────────────────────────────
pub(crate) fn encode_neon_float_elem(operands: &[Operand], u_bit: u32, opcode: u32) -> Result<EncodeResult, String> {
    if operands.len() < 3 { return Err("NEON float by-element requires 3 operands".to_string()); }
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;
    let (rm, index) = match &operands[2] {
        Operand::RegLane { reg, index, .. } => (parse_reg_num(reg).ok_or("invalid reg")?, *index),
        _ => return Err(format!("expected register lane, got {:?}", operands[2])),
    };
    let (q, sz) = match arr_d.as_str() {
        "2s" => (0u32, 0u32), "4s" => (1, 0), "2d" => (1, 1),
        _ => return Err(format!("float by-element: unsupported: {}", arr_d)),
    };
    let (h, l, m_bit) = if sz == 0 {
        ((index >> 1) & 1, index & 1, (rm >> 4) & 1)
    } else {
        (index & 1, 0u32, (rm >> 4) & 1)
    };
    let rm_enc = rm & 0x1F;
    let word = (q << 30) | (u_bit << 29) | (0b01111 << 24) | (sz << 22)
        | (l << 21) | (m_bit << 20) | (rm_enc << 16) | (opcode << 12)
        | (h << 11) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

// ── NEON FCVTL/FCVTN ────────────────────────────────────────────────────
/// FCVTL: half→single or single→double widening float convert
/// Format: 0 Q 0 01110 0 sz 10000 10111 10 Rn Rd
pub(crate) fn encode_neon_fcvtl(operands: &[Operand], is_high: bool) -> Result<EncodeResult, String> {
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;
    let sz = match arr_d.as_str() { "4s" | "2s" => 0u32, "2d" => 1,
        _ => return Err(format!("fcvtl: unsupported dest: {}", arr_d)), };
    let q = if is_high { 1u32 } else { 0 };
    let word = (q << 30) | (0b01110 << 24) | (sz << 22) | (0b10000 << 17)
        | (0b10111 << 12) | (0b10 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// FCVTN: single→half or double→single narrowing float convert
pub(crate) fn encode_neon_fcvtn(operands: &[Operand], is_high: bool) -> Result<EncodeResult, String> {
    let (rd, _) = get_neon_reg(operands, 0)?;
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    let sz = match arr_n.as_str() { "4s" | "2s" => 0u32, "2d" => 1,
        _ => return Err(format!("fcvtn: unsupported source: {}", arr_n)), };
    let q = if is_high { 1u32 } else { 0 };
    let word = (q << 30) | (0b01110 << 24) | (sz << 22) | (0b10000 << 17)
        | (0b10110 << 12) | (0b10 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

// ── BIT/BIF (bitwise insert if true/false) ──────────────────────────────
/// Encodes BIT (size=10) and BIF (size=11) instructions.
/// Same format as BSL but with different size field.
/// Format: 0 Q 1 01110 ss 1 Rm 000111 Rn Rd
pub(crate) fn encode_neon_bitwise_insert(operands: &[Operand], size: u32) -> Result<EncodeResult, String> {
    if operands.len() < 3 {
        return Err("bit/bif requires 3 operands".to_string());
    }
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;
    let (rm, _) = get_neon_reg(operands, 2)?;
    let q: u32 = if arr_d == "16b" { 1 } else { 0 };
    let word = (q << 30) | (1 << 29) | (0b01110 << 24) | (size << 22) | (1 << 21)
        | (rm << 16) | (0b000111 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

// ── FADDP (float pairwise add) ──────────────────────────────────────────
/// FADDP — float add pairwise
/// Vector form: FADDP Vd.T, Vn.T, Vm.T
///   Format: 0 Q 1 01110 0 sz 1 Rm 110101 Rn Rd
/// Scalar form: FADDP Sd, Vn.2S  or FADDP Dd, Vn.2D
///   Format: 01 1 11110 0 sz 11000 01101 10 Rn Rd
pub(crate) fn encode_neon_faddp(operands: &[Operand]) -> Result<EncodeResult, String> {
    if operands.len() >= 3 {
        // Vector form: 3 operands
        let (rd, arr_d) = get_neon_reg(operands, 0)?;
        let (rn, _) = get_neon_reg(operands, 1)?;
        let (rm, _) = get_neon_reg(operands, 2)?;
        let (q, sz) = match arr_d.as_str() {
            "2s" => (0u32, 0u32),
            "4s" => (1, 0),
            "2d" => (1, 1),
            _ => return Err(format!("faddp: unsupported arrangement: {}", arr_d)),
        };
        let word = (q << 30) | (1 << 29) | (0b01110 << 24) | (sz << 22) | (1 << 21)
            | (rm << 16) | (0b110101 << 10) | (rn << 5) | rd;
        Ok(EncodeResult::Word(word))
    } else if operands.len() == 2 {
        // Scalar form: FADDP Sd, Vn.2S or FADDP Dd, Vn.2D
        let rd = match &operands[0] {
            Operand::Reg(r) => parse_reg_num(r).ok_or("invalid dest reg")?,
            _ => return Err("faddp scalar: expected register".to_string()),
        };
        let (rn, arr_n) = get_neon_reg(operands, 1)?;
        let sz = match arr_n.as_str() {
            "2s" => 0u32,
            "2d" => 1,
            _ => return Err(format!("faddp scalar: unsupported source: {}", arr_n)),
        };
        // 01 1 11110 0 sz 11000 01101 10 Rn Rd
        let word = (0b01 << 30) | (1 << 29) | (0b11110 << 24) | (sz << 22)
            | (0b11000 << 17) | (0b01101 << 12) | (0b10 << 10) | (rn << 5) | rd;
        Ok(EncodeResult::Word(word))
    } else {
        Err("faddp requires 2 or 3 operands".to_string())
    }
}

// ── SADDLV/UADDLV (signed/unsigned add long across vector) ─────────────
/// Format: 0 Q U 01110 size 11000 00011 10 Rn Rd
pub(crate) fn encode_neon_across_long(operands: &[Operand], u: u32, opcode: u32) -> Result<EncodeResult, String> {
    if operands.len() < 2 {
        return Err("saddlv/uaddlv requires 2 operands".to_string());
    }
    // Destination is a scalar register (e.g., s16), source is a vector arrangement
    let rd = match &operands[0] {
        Operand::Reg(r) => parse_reg_num(r).ok_or("invalid dest reg")?,
        Operand::RegArrangement { reg, .. } => parse_reg_num(reg).ok_or("invalid dest reg")?,
        _ => return Err("saddlv: expected register".to_string()),
    };
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    let (q, size) = neon_arr_to_q_size(&arr_n)?;
    let word = (q << 30) | (u << 29) | (0b01110 << 24) | (size << 22)
        | (0b11000 << 17) | (opcode << 12) | (0b10 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

// ── NEON shift left by immediate (SQSHL, UQSHL, SHL, etc.) ─────────────
/// Format: 0 Q U 011110 immh:immb opcode 1 Rn Rd
/// immh:immb encodes both the element size and the shift amount.
pub(crate) fn encode_neon_shift_left_imm(operands: &[Operand], u: u32, opcode: u32) -> Result<EncodeResult, String> {
    if operands.len() < 3 {
        return Err("shift left immediate requires 3 operands".to_string());
    }
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;
    let shift = get_imm(operands, 2)? as u32;

    let (q, _immh_base, esize) = match arr_d.as_str() {
        "8b" => (0u32, 0b0001u32, 8u32),
        "16b" => (1, 0b0001, 8),
        "4h" => (0, 0b0010, 16),
        "8h" => (1, 0b0010, 16),
        "2s" => (0, 0b0100, 32),
        "4s" => (1, 0b0100, 32),
        "2d" => (1, 0b1000, 64),
        _ => return Err(format!("shift left imm: unsupported arrangement: {}", arr_d)),
    };

    // immh:immb = esize + shift_amount
    // For 8-bit: immh=0001, shift in 0..7 => immh:immb = 8 + shift
    // For 16-bit: immh=001x, shift in 0..15 => immh:immb = 16 + shift
    // For 32-bit: immh=01xx, shift in 0..31 => immh:immb = 32 + shift
    // For 64-bit: immh=1xxx, shift in 0..63 => immh:immb = 64 + shift
    let immhb = esize + shift;
    let immh = (immhb >> 3) & 0xF;
    let immb = immhb & 0x7;

    let word = (q << 30) | (u << 29) | (0b011110 << 23) | (immh << 19) | (immb << 16)
        | (opcode << 11) | (1 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

// ── Helper: detect scalar d-register 3-operand NEON operations ──────────────
pub(crate) fn is_neon_scalar_d_reg_op(operands: &[Operand]) -> bool {
    if operands.len() < 3 { return false; }
    match &operands[0] {
        Operand::Reg(r) => {
            let r = r.to_lowercase();
            r.starts_with('d') && r[1..].parse::<u32>().is_ok()
        }
        _ => false,
    }
}

// ── NEON scalar three-same: ADD/SUB Dd, Dn, Dm ────────────────────────────
/// Encode scalar NEON three-same: 01 U 11110 size 1 Rm opcode 1 Rn Rd
pub(crate) fn encode_neon_scalar_three_same(operands: &[Operand], u_bit: u32, opcode: u32, size: u32) -> Result<EncodeResult, String> {
    if operands.len() < 3 { return Err("scalar three-same requires 3 operands".to_string()); }
    let rd = match &operands[0] { Operand::Reg(r) => parse_reg_num(r).ok_or("invalid reg")?, _ => return Err("expected register".to_string()) };
    let rn = match &operands[1] { Operand::Reg(r) => parse_reg_num(r).ok_or("invalid reg")?, _ => return Err("expected register".to_string()) };
    let rm = match &operands[2] { Operand::Reg(r) => parse_reg_num(r).ok_or("invalid reg")?, _ => return Err("expected register".to_string()) };
    let word = (0b01 << 30) | (u_bit << 29) | (0b11110 << 24) | (size << 22) | (1 << 21)
        | (rm << 16) | (opcode << 11) | (1 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

// ── NEON scalar ADDP: addp Dd, Vn.2d ──────────────────────────────────────
pub(crate) fn encode_neon_scalar_addp(operands: &[Operand]) -> Result<EncodeResult, String> {
    if operands.len() < 2 { return Err("scalar addp requires 2 operands".to_string()); }
    let rd = match &operands[0] { Operand::Reg(r) => parse_reg_num(r).ok_or("invalid reg")?, _ => return Err("expected d register".to_string()) };
    let rn = match &operands[1] {
        Operand::RegArrangement { reg, arrangement } => {
            if arrangement != "2d" { return Err(format!("scalar addp requires .2d source, got .{}", arrangement)); }
            parse_reg_num(reg).ok_or("invalid reg")?
        }
        _ => return Err("scalar addp: expected Vn.2d source".to_string()),
    };
    // Scalar ADDP: 01 0 11110 11 11000 11011 10 Rn Rd
    let word = (0b01 << 30) | (0b011110 << 24) | (0b11 << 22) | (0b11000 << 17)
        | (0b11011 << 12) | (0b10 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

// ── NEON scalar two-reg misc: SQABS/SQNEG Hd,Hn / Sd,Sn / Dd,Dn ──────────
pub(crate) fn encode_neon_scalar_two_misc(operands: &[Operand], u_bit: u32, opcode: u32) -> Result<EncodeResult, String> {
    if operands.len() < 2 { return Err("scalar two-misc requires 2 operands".to_string()); }
    let (rd, rd_name) = match &operands[0] { Operand::Reg(r) => (parse_reg_num(r).ok_or("invalid reg")?, r.to_lowercase()), _ => return Err("expected register".to_string()) };
    let rn = match &operands[1] { Operand::Reg(r) => parse_reg_num(r).ok_or("invalid reg")?, _ => return Err("expected register".to_string()) };
    let size = if rd_name.starts_with('b') { 0b00u32 }
        else if rd_name.starts_with('h') { 0b01 }
        else if rd_name.starts_with('s') { 0b10 }
        else if rd_name.starts_with('d') { 0b11 }
        else { return Err(format!("scalar two-misc: unsupported register type: {}", rd_name)); };
    // 01 U 11110 size 10000 opcode 10 Rn Rd
    let word = (0b01 << 30) | (u_bit << 29) | (0b11110 << 24) | (size << 22)
        | (0b10000 << 17) | (opcode << 12) | (0b10 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

// ── NEON scalar SQSHRN: sqshrn Hd,Sn,#shift / sqshrn Sd,Dn,#shift ────────
pub(crate) fn encode_neon_scalar_qshrn(operands: &[Operand], u_bit: u32, is_rounding: bool) -> Result<EncodeResult, String> {
    if operands.len() < 3 { return Err("scalar qshrn requires 3 operands".to_string()); }
    let (rd, rd_name) = match &operands[0] { Operand::Reg(r) => (parse_reg_num(r).ok_or("invalid reg")?, r.to_lowercase()), _ => return Err("expected register".to_string()) };
    let rn = match &operands[1] { Operand::Reg(r) => parse_reg_num(r).ok_or("invalid reg")?, _ => return Err("expected register".to_string()) };
    let shift = get_imm(operands, 2)? as u32;
    // Determine element bits from destination register type
    let element_bits = if rd_name.starts_with('b') { 8u32 }  // b <- h (narrow from 16-bit)
        else if rd_name.starts_with('h') { 16 }  // h <- s (narrow from 32-bit), immh base = 16
        else if rd_name.starts_with('s') { 32 }  // s <- d (narrow from 64-bit), immh base = 32
        else { return Err(format!("scalar qshrn: unsupported dest: {}", rd_name)); };
    if shift == 0 || shift > element_bits { return Err(format!("scalar qshrn: shift {} out of range", shift)); }
    let immhb = (element_bits * 2) - shift;  // source element bits - shift
    let opcode_bits: u32 = if is_rounding { 0b100111 } else { 0b100101 };
    // 01 U 11110 immh:immb opcode 1 Rn Rd
    let word = (0b01 << 30) | (u_bit << 29) | (0b011110 << 23) | ((immhb >> 3) << 19) | ((immhb & 7) << 16)
        | (opcode_bits << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

// ── NEON addp (integer pairwise add) — already handled in three-same as addp ──

#[cfg(test)]
mod encode_neon_three_diff_narrow_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler
    // Evidence: src/backend/arm/assembler/README.md "accepts the same textual assembly that GCC's gas would consume";
    //   encoder/mod.rs:1-7 "Encodes AArch64 instructions into 32-bit machine code words";
    //   encoder/mod.rs:628-635 addhn/raddhn/subhn/rsubhn (+2) => encode_neon_three_diff_narrow;
    //   neon.rs:1513 Format 0 Q U 01110 size 1 Rm opcode 00 Rn Rd; ARM ARM Advanced SIMD three-different
    // Stronger considered:
    //   - State machine: rejected — encode_neon_three_diff_narrow is a pure function with no lifecycle
    //   - Algebraic round-trip: rejected — no in-tree ADDHN decoder
    //   - Differential vs encode_neon_three_diff: rejected — widening/long sibling, different Ta map (same-job gate)
    // Weaker available: algebraic.metamorphic (Q/U bits), algebraic.invariant (word layout), negative_error (arity / Ta / Tb / non-reg)
    // Differential: candidate=encode_neon_three_diff_narrow, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=[RegArrangement(Vd,Tb), RegArrangement(Vn,Ta), RegArrangement(Vm,Ta)] <-> `{mnem} Vd.Tb, Vn.Ta, Vm.Ta`

    use super::encode_neon_three_diff_narrow;
    use super::EncodeResult;
    use crate::backend::arm::assembler::parser::Operand;
    use proptest::prelude::*;
    use std::io::Write;
    use std::process::{Command, Stdio};

    const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
    const CASES: u32 = 1000;

    fn cfg() -> ProptestConfig {
        ProptestConfig::with_cases(CASES)
    }

    fn vreg(n: u32) -> String {
        format!("v{}", n)
    }

    fn neon_arr(reg: u32, arr: &str) -> Operand {
        Operand::RegArrangement {
            reg: vreg(reg),
            arrangement: arr.to_string(),
        }
    }

    fn mandated_tb(ta: &str, is_high: bool) -> &'static str {
        match (ta, is_high) {
            ("8h", false) => "8b",
            ("8h", true) => "16b",
            ("4s", false) => "4h",
            ("4s", true) => "8h",
            ("2d", false) => "2s",
            ("2d", true) => "4s",
            _ => "8b",
        }
    }

    fn size_of_ta(ta: &str) -> u32 {
        match ta {
            "8h" => 0b00,
            "4s" => 0b01,
            "2d" => 0b10,
            _ => 0xff,
        }
    }

    fn mnemonic(u_bit: u32, opcode: u32, is_high: bool) -> &'static str {
        match (u_bit, opcode, is_high) {
            (0, 0b0100, false) => "addhn",
            (0, 0b0100, true) => "addhn2",
            (1, 0b0100, false) => "raddhn",
            (1, 0b0100, true) => "raddhn2",
            (0, 0b0110, false) => "subhn",
            (0, 0b0110, true) => "subhn2",
            (1, 0b0110, false) => "rsubhn",
            (1, 0b0110, true) => "rsubhn2",
            _ => "addhn",
        }
    }

    fn sut_word(ops: &[Operand], u_bit: u32, opcode: u32, is_high: bool) -> Result<u32, String> {
        match encode_neon_three_diff_narrow(ops, u_bit, opcode, is_high)? {
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

    fn reg_num() -> impl Strategy<Value = u32> {
        prop_oneof![Just(0u32), Just(1u32), Just(30u32), Just(31u32), 0u32..=31]
    }

    fn ta_arr() -> impl Strategy<Value = &'static str> {
        prop::sample::select(vec!["8h", "4s", "2d"])
    }

    fn opcode_bits() -> impl Strategy<Value = u32> {
        prop_oneof![Just(0b0100u32), Just(0b0110u32)]
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_neon_three_diff_narrow_kat_llvm_mc_addhn_v0_v1_v2() {
        let want = 0x0e224020u32;
        let mc = llvm_mc_word("addhn v0.8b, v1.8h, v2.8h").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [
            neon_arr(0, "8b"),
            neon_arr(1, "8h"),
            neon_arr(2, "8h"),
        ];
        let sut = sut_word(&ops, 0, 0b0100, false).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    proptest! {
        #![proptest_config(cfg())]

        #[test]
        fn encode_neon_three_diff_narrow_diff_llvm_mc(
            rd in reg_num(),
            rn in reg_num(),
            rm in reg_num(),
            ta in ta_arr(),
            is_high in any::<bool>(),
            u_bit in 0u32..=1u32,
            opcode in opcode_bits(),
        ) {
            let tb = mandated_tb(ta, is_high);
            let mnem = mnemonic(u_bit, opcode, is_high);
            let asm = format!(
                "{} {}.{}, {}.{}, {}.{} ",
                mnem, vreg(rd), tb, vreg(rn), ta, vreg(rm), ta
            ).trim_end().to_string();
            let ops = [
                neon_arr(rd, tb),
                neon_arr(rn, ta),
                neon_arr(rm, ta),
            ];
            let mc = llvm_mc_word(&asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
            let sut = sut_word(&ops, u_bit, opcode, is_high)
                .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
            prop_assert_eq!(sut, mc, "mismatch for {}", asm);
        }

        #[test]
        fn encode_neon_three_diff_narrow_q_bit_is_high(
            rd in reg_num(),
            rn in reg_num(),
            rm in reg_num(),
            ta in ta_arr(),
            u_bit in 0u32..=1u32,
            opcode in opcode_bits(),
        ) {
            let tb = mandated_tb(ta, false);
            let ops = [
                neon_arr(rd, tb),
                neon_arr(rn, ta),
                neon_arr(rm, ta),
            ];
            let lo = sut_word(&ops, u_bit, opcode, false)
                .unwrap_or_else(|e| panic!("SUT Q=0 rejected: {}", e));
            let hi = sut_word(&ops, u_bit, opcode, true)
                .unwrap_or_else(|e| panic!("SUT Q=1 rejected: {}", e));
            prop_assert_eq!(lo ^ hi, 1u32 << 30, "is_high must toggle only Q (bit 30)");
        }

        #[test]
        fn encode_neon_three_diff_narrow_u_bit(
            rd in reg_num(),
            rn in reg_num(),
            rm in reg_num(),
            ta in ta_arr(),
            is_high in any::<bool>(),
            opcode in opcode_bits(),
        ) {
            let tb = mandated_tb(ta, is_high);
            let ops = [
                neon_arr(rd, tb),
                neon_arr(rn, ta),
                neon_arr(rm, ta),
            ];
            let u0 = sut_word(&ops, 0, opcode, is_high)
                .unwrap_or_else(|e| panic!("SUT U=0 rejected: {}", e));
            let u1 = sut_word(&ops, 1, opcode, is_high)
                .unwrap_or_else(|e| panic!("SUT U=1 rejected: {}", e));
            prop_assert_eq!(u0 ^ u1, 1u32 << 29, "u_bit must toggle only U (bit 29)");
        }

        #[test]
        fn encode_neon_three_diff_narrow_word_layout(
            rd in reg_num(),
            rn in reg_num(),
            rm in reg_num(),
            ta in ta_arr(),
            u_bit in 0u32..=1u32,
            opcode in opcode_bits(),
            is_high in any::<bool>(),
        ) {
            let tb = mandated_tb(ta, is_high);
            let ops = [
                neon_arr(rd, tb),
                neon_arr(rn, ta),
                neon_arr(rm, ta),
            ];
            let w = sut_word(&ops, u_bit, opcode, is_high)
                .unwrap_or_else(|e| panic!("SUT rejected valid layout: {}", e));
            let q = if is_high { 1u32 } else { 0 };
            let size = size_of_ta(ta);
            prop_assert_eq!((w >> 31) & 1, 0u32, "bit 31 must be 0");
            prop_assert_eq!((w >> 30) & 1, q, "Q bit");
            prop_assert_eq!((w >> 29) & 1, u_bit, "U bit");
            prop_assert_eq!((w >> 24) & 0b11111, 0b01110u32, "bits[28:24]=01110");
            prop_assert_eq!((w >> 22) & 0b11, size, "size from Ta");
            prop_assert_eq!((w >> 21) & 1, 1u32, "bit 21 must be 1");
            prop_assert_eq!((w >> 16) & 0b11111, rm, "Rm");
            prop_assert_eq!((w >> 12) & 0b1111, opcode, "opcode[15:12]");
            prop_assert_eq!((w >> 10) & 0b11, 0u32, "bits[11:10]=00");
            prop_assert_eq!((w >> 5) & 0b11111, rn, "Rn");
            prop_assert_eq!(w & 0b11111, rd, "Rd");
        }

        #[test]
        fn encode_neon_three_diff_narrow_arity_err(
            n in 0usize..=2,
            u_bit in 0u32..=1u32,
            opcode in opcode_bits(),
            is_high in any::<bool>(),
            rd in reg_num(),
            rn in reg_num(),
        ) {
            let all = [
                neon_arr(rd, "8b"),
                neon_arr(rn, "8h"),
            ];
            let ops: Vec<Operand> = all.iter().take(n).cloned().collect();
            prop_assert!(
                encode_neon_three_diff_narrow(&ops, u_bit, opcode, is_high).is_err(),
                "len={} must Err (requires 3 operands)",
                n
            );
        }

        #[test]
        fn encode_neon_three_diff_narrow_unsupported_src(
            rd in reg_num(),
            rn in reg_num(),
            rm in reg_num(),
            ta in prop::sample::select(vec!["8b", "16b", "4h", "2s", "1d", "16h", "8s", "4d", "", "b", "h"]),
            u_bit in 0u32..=1u32,
            opcode in opcode_bits(),
            is_high in any::<bool>(),
        ) {
            let ops = [
                neon_arr(rd, "8b"),
                neon_arr(rn, ta),
                neon_arr(rm, ta),
            ];
            prop_assert!(
                encode_neon_three_diff_narrow(&ops, u_bit, opcode, is_high).is_err(),
                "source Ta={} is not 8h/4s/2d and must Err",
                ta
            );
        }

        #[test]
        fn encode_neon_three_diff_narrow_dest_tb_must_match(
            rd in reg_num(),
            rn in reg_num(),
            rm in reg_num(),
            ta in ta_arr(),
            tb in prop::sample::select(vec!["8b", "16b", "4h", "8h", "2s", "4s", "1d", "2d"]),
            is_high in any::<bool>(),
            u_bit in 0u32..=1u32,
            opcode in opcode_bits(),
        ) {
            prop_assume!(tb != mandated_tb(ta, is_high));
            let mnem = mnemonic(u_bit, opcode, is_high);
            let asm = format!(
                "{} {}.{}, {}.{}, {}.{} ",
                mnem, vreg(rd), tb, vreg(rn), ta, vreg(rm), ta
            ).trim_end().to_string();
            prop_assert!(
                llvm_mc_word(&asm).is_err(),
                "llvm-mc unexpectedly accepted mismatched Tb {}",
                asm
            );
            let ops = [
                neon_arr(rd, tb),
                neon_arr(rn, ta),
                neon_arr(rm, ta),
            ];
            prop_assert!(
                encode_neon_three_diff_narrow(&ops, u_bit, opcode, is_high).is_err(),
                "mismatched dest Tb={} for Ta={} is_high={} must Err (llvm-mc rejects {})",
                tb,
                ta,
                is_high,
                asm
            );
        }

        #[test]
        fn encode_neon_three_diff_narrow_non_reg_err(
            slot in 0usize..=2,
            which in 0u32..=5,
            u_bit in 0u32..=1u32,
            opcode in opcode_bits(),
            is_high in any::<bool>(),
            rd in reg_num(),
        ) {
            let bad = match which {
                0 => Operand::Imm(0),
                1 => Operand::Mem {
                    base: "x0".into(),
                    offset: 0,
                },
                2 => Operand::Symbol("foo".into()),
                3 => Operand::Shift {
                    kind: "lsl".into(),
                    amount: 0,
                },
                4 => Operand::Cond("eq".into()),
                _ => Operand::Label(".L0".into()),
            };
            let mut ops = vec![
                neon_arr(rd, "8b"),
                neon_arr(rd, "8h"),
                neon_arr(rd, "8h"),
            ];
            ops[slot] = bad;
            prop_assert!(
                encode_neon_three_diff_narrow(&ops, u_bit, opcode, is_high).is_err(),
                "non-register at slot {} which={} must Err",
                slot,
                which
            );
        }

        #[test]
        fn encode_neon_three_diff_narrow_rm_ta_must_match(
            rd in reg_num(),
            rn in reg_num(),
            rm in reg_num(),
            ta_n in ta_arr(),
            ta_m in ta_arr(),
            is_high in any::<bool>(),
            u_bit in 0u32..=1u32,
            opcode in opcode_bits(),
        ) {
            prop_assume!(ta_n != ta_m);
            let tb = mandated_tb(ta_n, is_high);
            let mnem = mnemonic(u_bit, opcode, is_high);
            let asm = format!(
                "{} {}.{}, {}.{}, {}.{} ",
                mnem, vreg(rd), tb, vreg(rn), ta_n, vreg(rm), ta_m
            ).trim_end().to_string();
            prop_assert!(
                llvm_mc_word(&asm).is_err(),
                "llvm-mc unexpectedly accepted mismatched Rm Ta {}",
                asm
            );
            let ops = [
                neon_arr(rd, tb),
                neon_arr(rn, ta_n),
                neon_arr(rm, ta_m),
            ];
            prop_assert!(
                encode_neon_three_diff_narrow(&ops, u_bit, opcode, is_high).is_err(),
                "mismatched Rm Ta={} vs Rn Ta={} must Err (llvm-mc rejects {})",
                ta_m,
                ta_n,
                asm
            );
        }

        #[test]
        fn encode_neon_three_diff_narrow_extra_operand_err(
            rd in reg_num(),
            rn in reg_num(),
            rm in reg_num(),
            extra in reg_num(),
            ta in ta_arr(),
            is_high in any::<bool>(),
            u_bit in 0u32..=1u32,
            opcode in opcode_bits(),
        ) {
            let tb = mandated_tb(ta, is_high);
            let mnem = mnemonic(u_bit, opcode, is_high);
            let asm = format!(
                "{} {}.{}, {}.{}, {}.{}, {}.{} ",
                mnem, vreg(rd), tb, vreg(rn), ta, vreg(rm), ta, vreg(extra), ta
            ).trim_end().to_string();
            prop_assert!(
                llvm_mc_word(&asm).is_err(),
                "llvm-mc unexpectedly accepted 4-operand {}",
                asm
            );
            let ops = [
                neon_arr(rd, tb),
                neon_arr(rn, ta),
                neon_arr(rm, ta),
                neon_arr(extra, ta),
            ];
            prop_assert!(
                encode_neon_three_diff_narrow(&ops, u_bit, opcode, is_high).is_err(),
                "4 operands must Err (llvm-mc rejects {})",
                asm
            );
        }

        #[test]
        fn encode_neon_three_diff_narrow_gpr_dest_err(
            prefix in prop::sample::select(vec!["x", "w", "d", "s", "q", "h", "b"]),
            n in prop_oneof![Just(0u32), Just(31u32), 0u32..=31],
            rn in reg_num(),
            rm in reg_num(),
            ta in ta_arr(),
            is_high in any::<bool>(),
            u_bit in 0u32..=1u32,
            opcode in opcode_bits(),
        ) {
            let dest = format!("{}{}", prefix, n);
            let tb = mandated_tb(ta, is_high);
            let mnem = mnemonic(u_bit, opcode, is_high);
            let asm = format!(
                "{} {}, {}.{}, {}.{} ",
                mnem, dest, vreg(rn), ta, vreg(rm), ta
            ).trim_end().to_string();
            prop_assert!(
                llvm_mc_word(&asm).is_err(),
                "llvm-mc unexpectedly accepted GPR/FP dest {}",
                asm
            );
            let ops = [
                Operand::Reg(dest.clone()),
                neon_arr(rn, ta),
                neon_arr(rm, ta),
            ];
            prop_assert!(
                encode_neon_three_diff_narrow(&ops, u_bit, opcode, is_high).is_err(),
                "GPR/FP dest {} is not a NEON Vd.Tb and must Err (llvm-mc rejects {})",
                dest,
                asm
            );
        }

        #[test]
        fn encode_neon_three_diff_narrow_invalid_reg_err(
            slot in 0usize..=2,
            bad in prop_oneof![
                Just("v32".to_string()),
                Just("v99".to_string()),
                Just("foo".to_string()),
                Just("".to_string()),
                Just("v".to_string()),
                Just("v-1".to_string()),
            ],
            u_bit in 0u32..=1u32,
            opcode in opcode_bits(),
            is_high in any::<bool>(),
            rd in reg_num(),
        ) {
            let mut ops = vec![
                neon_arr(rd, "8b"),
                neon_arr(rd, "8h"),
                neon_arr(rd, "8h"),
            ];
            ops[slot] = Operand::RegArrangement {
                reg: bad.clone(),
                arrangement: if slot == 1 { "8h".into() } else { "8b".into() },
            };
            prop_assert!(
                encode_neon_three_diff_narrow(&ops, u_bit, opcode, is_high).is_err(),
                "invalid NEON register {} at slot {} must Err",
                bad,
                slot
            );
        }
    }

    /// Deterministic regression: ADDHN2 dest must be 16B when Ta=8H (shrunk from dest_tb_must_match).
    #[test]
    fn test_encode_neon_three_diff_narrow_regression_mismatched_dest_tb() {
        let ops = [
            neon_arr(0, "8b"),
            neon_arr(0, "8h"),
            neon_arr(0, "8h"),
        ];
        assert!(
            encode_neon_three_diff_narrow(&ops, 0, 0b0100, true).is_err(),
            "addhn2 v0.8b, v0.8h, v0.8h must Err (dest Tb=8b is not 16b)"
        );
    }

    /// Deterministic regression: Vm.Ta must equal Vn.Ta (shrunk from rm_ta_must_match).
    #[test]
    fn test_encode_neon_three_diff_narrow_regression_rm_ta_mismatch() {
        let ops = [
            neon_arr(0, "4h"),
            neon_arr(0, "4s"),
            neon_arr(0, "8h"),
        ];
        assert!(
            encode_neon_three_diff_narrow(&ops, 0, 0b0100, false).is_err(),
            "addhn v0.4h, v0.4s, v0.8h must Err (Rm Ta != Rn Ta)"
        );
    }

    /// Deterministic regression: exactly 3 operands (shrunk from extra_operand_err).
    #[test]
    fn test_encode_neon_three_diff_narrow_regression_extra_operand() {
        let ops = [
            neon_arr(0, "8b"),
            neon_arr(0, "8h"),
            neon_arr(0, "8h"),
            neon_arr(0, "8h"),
        ];
        assert!(
            encode_neon_three_diff_narrow(&ops, 0, 0b0100, false).is_err(),
            "addhn v0.8b, v0.8h, v0.8h, v0.8h must Err (exactly 3 operands)"
        );
    }

    /// Deterministic regression: dest must be Vd.Tb, not a GPR (shrunk from gpr_dest_err).
    #[test]
    fn test_encode_neon_three_diff_narrow_regression_gpr_dest() {
        let ops = [
            Operand::Reg("x0".into()),
            neon_arr(0, "8h"),
            neon_arr(0, "8h"),
        ];
        assert!(
            encode_neon_three_diff_narrow(&ops, 0, 0b0100, false).is_err(),
            "addhn x0, v0.8h, v0.8h must Err (GPR dest is not Vd.Tb)"
        );
    }
}

#[cfg(test)]
mod encode_neon_across_long_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler
    // Evidence: src/backend/arm/assembler/README.md "accepts the same textual assembly that GCC's gas would consume";
    //   encoder/mod.rs:1-7 "Encodes AArch64 instructions into 32-bit machine code words";
    //   encoder/mod.rs:679-680 saddlv/uaddlv => encode_neon_across_long(operands, U, 0b00011);
    //   neon.rs:1722 Format 0 Q U 01110 size 11000 00011 10 Rn Rd; ARM ARM SADDLV/UADDLV
    // Stronger considered:
    //   - State machine: rejected — encode_neon_across_long is a pure function with no lifecycle
    //   - Algebraic round-trip: rejected as primary — no in-tree SADDLV decoder (field unpack kept as weaker algebraic)
    //   - Differential vs encode_neon_across / encode_neon_addv: rejected — same-width reduce, different opcode/dest width (same-job gate)
    // Weaker available: algebraic.metamorphic (U bit), algebraic.invariant (word layout), negative_error (arity / T / dest V / extra)
    // Differential: candidate=encode_neon_across_long, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=[Reg(Vd), RegArrangement(Vn, T)] + (u, opcode=0b00011) <-> `{saddlv|uaddlv} Vd, Vn.T`

    use super::encode_neon_across_long;
    use super::EncodeResult;
    use crate::backend::arm::assembler::parser::Operand;
    use proptest::prelude::*;
    use std::io::Write;
    use std::process::{Command, Stdio};

    const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
    const CASES: u32 = 1000;
    const OPCODE: u32 = 0b00011;

    fn cfg() -> ProptestConfig {
        ProptestConfig::with_cases(CASES)
    }

    fn vreg(n: u32) -> String {
        format!("v{}", n)
    }

    fn neon_arr(reg: u32, arr: &str) -> Operand {
        Operand::RegArrangement {
            reg: vreg(reg),
            arrangement: arr.to_string(),
        }
    }

    fn mandated_v(t: &str) -> &'static str {
        match t {
            "8b" | "16b" => "h",
            "4h" | "8h" => "s",
            "4s" => "d",
            _ => "h",
        }
    }

    fn q_size_of(t: &str) -> (u32, u32) {
        match t {
            "8b" => (0, 0b00),
            "16b" => (1, 0b00),
            "4h" => (0, 0b01),
            "8h" => (1, 0b01),
            "4s" => (1, 0b10),
            _ => (0xff, 0xff),
        }
    }

    fn dest_reg(v: &str, n: u32) -> Operand {
        Operand::Reg(format!("{}{}", v, n))
    }

    fn mnemonic(u: u32) -> &'static str {
        if u == 0 {
            "saddlv"
        } else {
            "uaddlv"
        }
    }

    fn sut_word(ops: &[Operand], u: u32) -> Result<u32, String> {
        match encode_neon_across_long(ops, u, OPCODE)? {
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

    fn reg_num() -> impl Strategy<Value = u32> {
        prop_oneof![Just(0u32), Just(1u32), Just(30u32), Just(31u32), 0u32..=31]
    }

    fn valid_t() -> impl Strategy<Value = &'static str> {
        prop::sample::select(vec!["8b", "16b", "4h", "8h", "4s"])
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_neon_across_long_kat_llvm_mc_saddlv_h0_v1_8b() {
        let want = 0x0e303820u32;
        let mc = llvm_mc_word("saddlv h0, v1.8b").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [dest_reg("h", 0), neon_arr(1, "8b")];
        let sut = sut_word(&ops, 0).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    /// Known-answer: codegen popcount emits `uaddlv h0, v0.8b`.
    #[test]
    fn encode_neon_across_long_kat_llvm_mc_uaddlv_h0_v0_8b() {
        let want = 0x2e303800u32;
        let mc = llvm_mc_word("uaddlv h0, v0.8b").expect("llvm-mc KAT uaddlv");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken for uaddlv");
        let ops = [dest_reg("h", 0), neon_arr(0, "8b")];
        let sut = sut_word(&ops, 1).expect("SUT KAT uaddlv");
        assert_eq!(sut, want);
    }

    /// Known-answer: 4S form uses dest D, Q=1, size=10.
    #[test]
    fn encode_neon_across_long_kat_llvm_mc_saddlv_d0_v1_4s() {
        let want = 0x4eb03820u32;
        let mc = llvm_mc_word("saddlv d0, v1.4s").expect("llvm-mc KAT 4s");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken for 4s");
        let ops = [dest_reg("d", 0), neon_arr(1, "4s")];
        let sut = sut_word(&ops, 0).expect("SUT KAT 4s");
        assert_eq!(sut, want);
    }

    proptest! {
        #![proptest_config(cfg())]

        #[test]
        fn encode_neon_across_long_diff_llvm_mc(
            rd in reg_num(),
            rn in reg_num(),
            u in 0u32..=1u32,
            t in valid_t(),
        ) {
            let v = mandated_v(t);
            let mnem = mnemonic(u);
            let asm = format!("{} {}{}, {}.{} ", mnem, v, rd, vreg(rn), t)
                .trim_end()
                .to_string();
            let ops = [dest_reg(v, rd), neon_arr(rn, t)];
            let mc = llvm_mc_word(&asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
            let sut = sut_word(&ops, u)
                .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
            prop_assert_eq!(sut, mc, "mismatch for {}", asm);
        }

        #[test]
        fn encode_neon_across_long_roundtrip_arm_fields(
            rd in reg_num(),
            rn in reg_num(),
            u in 0u32..=1u32,
            t in valid_t(),
        ) {
            let v = mandated_v(t);
            let ops = [dest_reg(v, rd), neon_arr(rn, t)];
            let w = sut_word(&ops, u)
                .unwrap_or_else(|e| panic!("SUT rejected valid field unpack: {}", e));
            let (q, size) = q_size_of(t);
            prop_assert_eq!((w >> 31) & 1, 0u32, "bit 31 must be 0");
            prop_assert_eq!((w >> 30) & 1, q, "Q bit");
            prop_assert_eq!((w >> 29) & 1, u, "U bit");
            prop_assert_eq!((w >> 24) & 0b11111, 0b01110u32, "bits[28:24]=01110");
            prop_assert_eq!((w >> 22) & 0b11, size, "size from T");
            prop_assert_eq!((w >> 17) & 0b11111, 0b11000u32, "bits[21:17]=11000");
            prop_assert_eq!((w >> 12) & 0b11111, OPCODE, "opcode[16:12]=00011");
            prop_assert_eq!((w >> 10) & 0b11, 0b10u32, "bits[11:10]=10");
            prop_assert_eq!((w >> 5) & 0b11111, rn, "Rn");
            prop_assert_eq!(w & 0b11111, rd, "Rd");
        }

        #[test]
        fn encode_neon_across_long_metamorphic_u_bit(
            rd in reg_num(),
            rn in reg_num(),
            t in valid_t(),
        ) {
            let v = mandated_v(t);
            let ops = [dest_reg(v, rd), neon_arr(rn, t)];
            let s = sut_word(&ops, 0)
                .unwrap_or_else(|e| panic!("SUT U=0 rejected: {}", e));
            let uns = sut_word(&ops, 1)
                .unwrap_or_else(|e| panic!("SUT U=1 rejected: {}", e));
            prop_assert_eq!(s ^ uns, 1u32 << 29, "u must toggle only U (bit 29)");
        }

        #[test]
        fn encode_neon_across_long_invariant_fixed_bits(
            rd in reg_num(),
            rn in reg_num(),
            u in 0u32..=1u32,
            t in valid_t(),
        ) {
            let v = mandated_v(t);
            let ops = [dest_reg(v, rd), neon_arr(rn, t)];
            let w = sut_word(&ops, u)
                .unwrap_or_else(|e| panic!("SUT rejected valid invariant: {}", e));
            prop_assert_eq!((w >> 31) & 1, 0u32, "bit 31 must be 0");
            prop_assert_eq!((w >> 24) & 0b11111, 0b01110u32, "bits[28:24]=01110");
            prop_assert_eq!((w >> 17) & 0b11111, 0b11000u32, "bits[21:17]=11000");
            prop_assert_eq!((w >> 12) & 0b11111, OPCODE, "opcode[16:12]=00011");
            prop_assert_eq!((w >> 10) & 0b11, 0b10u32, "bits[11:10]=10");
        }

        #[test]
        fn encode_neon_across_long_neg_arity_and_shape(
            n in 0usize..=1,
            which in 0u32..=5,
            u in 0u32..=1u32,
            rd in reg_num(),
            bad in prop_oneof![
                Just("v32".to_string()),
                Just("h32".to_string()),
                Just("foo".to_string()),
                Just("".to_string()),
                Just("v".to_string()),
                Just("v-1".to_string()),
            ],
        ) {
            let dest = dest_reg("h", rd);
            let src = neon_arr(rd, "8b");
            // Too few operands.
            let short: Vec<Operand> = [dest.clone(), src.clone()].iter().take(n).cloned().collect();
            prop_assert!(
                encode_neon_across_long(&short, u, OPCODE).is_err(),
                "len={} must Err (requires 2 operands)",
                n
            );
            // Non-register second operand.
            let bad_src = match which {
                0 => Operand::Imm(0),
                1 => Operand::Mem {
                    base: "x0".into(),
                    offset: 0,
                },
                2 => Operand::Symbol("foo".into()),
                3 => Operand::Shift {
                    kind: "lsl".into(),
                    amount: 0,
                },
                4 => Operand::Cond("eq".into()),
                _ => Operand::Label(".L0".into()),
            };
            let ops_bad_src = [dest.clone(), bad_src];
            prop_assert!(
                encode_neon_across_long(&ops_bad_src, u, OPCODE).is_err(),
                "non-register source which={} must Err",
                which
            );
            // Invalid dest / src names.
            let ops_bad_dest = [
                Operand::Reg(bad.clone()),
                neon_arr(rd, "8b"),
            ];
            prop_assert!(
                encode_neon_across_long(&ops_bad_dest, u, OPCODE).is_err(),
                "invalid dest name {} must Err",
                bad
            );
            let ops_bad_rn = [
                dest_reg("h", rd),
                Operand::RegArrangement {
                    reg: bad.clone(),
                    arrangement: "8b".into(),
                },
            ];
            prop_assert!(
                encode_neon_across_long(&ops_bad_rn, u, OPCODE).is_err(),
                "invalid src name {} must Err",
                bad
            );
            // Coverage sweep: dest `_` arm (non-Reg / non-RegArrangement).
            let bad_dest_shape = match which {
                0 => Operand::Imm(0),
                1 => Operand::Mem {
                    base: "x0".into(),
                    offset: 0,
                },
                2 => Operand::Symbol("foo".into()),
                3 => Operand::Shift {
                    kind: "lsl".into(),
                    amount: 0,
                },
                4 => Operand::Cond("eq".into()),
                _ => Operand::Label(".L0".into()),
            };
            let ops_bad_dest_shape = [bad_dest_shape, neon_arr(rd, "8b")];
            prop_assert!(
                encode_neon_across_long(&ops_bad_dest_shape, u, OPCODE).is_err(),
                "non-register dest which={} must Err",
                which
            );
            // Coverage sweep: parse_reg_num None on dest RegArrangement.
            let ops_bad_arr_dest = [
                Operand::RegArrangement {
                    reg: bad.clone(),
                    arrangement: "8b".into(),
                },
                neon_arr(rd, "8b"),
            ];
            prop_assert!(
                encode_neon_across_long(&ops_bad_arr_dest, u, OPCODE).is_err(),
                "invalid dest RegArrangement name {} must Err",
                bad
            );
        }

        #[test]
        fn encode_neon_across_long_neg_extra_operands(
            rd in reg_num(),
            rn in reg_num(),
            extra in reg_num(),
            u in 0u32..=1u32,
            t in valid_t(),
            extra_kind in 0u32..=3u32,
        ) {
            let v = mandated_v(t);
            let mnem = mnemonic(u);
            let extra_op = match extra_kind {
                0 => dest_reg(v, extra),
                1 => neon_arr(extra, t),
                2 => Operand::Imm(0),
                _ => Operand::Mem {
                    base: "x0".into(),
                    offset: 0,
                },
            };
            let extra_asm = match extra_kind {
                0 => format!("{}{} ", v, extra),
                1 => format!("{}.{} ", vreg(extra), t),
                2 => "#0".to_string(),
                _ => "[x0]".to_string(),
            };
            let asm = format!(
                "{} {}{}, {}.{}, {} ",
                mnem, v, rd, vreg(rn), t, extra_asm.trim_end()
            )
            .trim_end()
            .to_string();
            prop_assert!(
                llvm_mc_word(&asm).is_err(),
                "llvm-mc unexpectedly accepted 3-operand {}",
                asm
            );
            let ops = [dest_reg(v, rd), neon_arr(rn, t), extra_op];
            prop_assert!(
                encode_neon_across_long(&ops, u, OPCODE).is_err(),
                "3 operands must Err (llvm-mc rejects {})",
                asm
            );
        }

        #[test]
        fn encode_neon_across_long_neg_invalid_arrangement(
            rd in reg_num(),
            rn in reg_num(),
            u in 0u32..=1u32,
            t in prop::sample::select(vec!["2s", "1d", "2d", "8s", "16h", "4d", "", "b", "h"]),
        ) {
            let asm = format!("{} h{}, {}.{} ", mnemonic(u), rd, vreg(rn), t)
                .trim_end()
                .to_string();
            if !t.is_empty() {
                prop_assert!(
                    llvm_mc_word(&asm).is_err(),
                    "llvm-mc unexpectedly accepted reserved/invalid T {}",
                    asm
                );
            }
            let ops = [dest_reg("h", rd), neon_arr(rn, t)];
            prop_assert!(
                encode_neon_across_long(&ops, u, OPCODE).is_err(),
                "T={} is not 8b/16b/4h/8h/4s and must Err (llvm-mc rejects {})",
                t,
                asm
            );
        }

        #[test]
        fn encode_neon_across_long_neg_dest_type(
            rd in reg_num(),
            rn in reg_num(),
            u in 0u32..=1u32,
            t in valid_t(),
            prefix in prop::sample::select(vec!["b", "h", "s", "d", "q", "x", "w", "v"]),
            as_arr in any::<bool>(),
        ) {
            let mandated = mandated_v(t);
            let dest_op;
            let dest_asm;
            if as_arr {
                dest_op = neon_arr(rd, t);
                dest_asm = format!("{}.{}", vreg(rd), t);
            } else {
                prop_assume!(prefix != mandated);
                dest_op = Operand::Reg(format!("{}{}", prefix, rd));
                dest_asm = format!("{}{}", prefix, rd);
            }
            let mnem = mnemonic(u);
            let asm = format!("{} {}, {}.{} ", mnem, dest_asm, vreg(rn), t)
                .trim_end()
                .to_string();
            prop_assert!(
                llvm_mc_word(&asm).is_err(),
                "llvm-mc unexpectedly accepted dest {}",
                asm
            );
            let ops = [dest_op, neon_arr(rn, t)];
            prop_assert!(
                encode_neon_across_long(&ops, u, OPCODE).is_err(),
                "dest {} is not mandated V={} for T={} and must Err (llvm-mc rejects {})",
                dest_asm,
                mandated,
                t,
                asm
            );
        }
    }

    /// Deterministic regression: exactly 2 operands (shrunk from neg_extra_operands).
    #[test]
    fn test_encode_neon_across_long_regression_extra_operand() {
        let ops = [
            dest_reg("h", 0),
            neon_arr(0, "8b"),
            dest_reg("h", 0),
        ];
        assert!(
            encode_neon_across_long(&ops, 0, OPCODE).is_err(),
            "saddlv h0, v0.8b, h0 must Err (exactly 2 operands)"
        );
    }

    /// Deterministic regression: reserved T=2S (shrunk from neg_invalid_arrangement).
    #[test]
    fn test_encode_neon_across_long_regression_reserved_2s() {
        let ops = [dest_reg("h", 0), neon_arr(0, "2s")];
        assert!(
            encode_neon_across_long(&ops, 0, OPCODE).is_err(),
            "saddlv h0, v0.2s must Err (T=2S is reserved)"
        );
    }

    /// Deterministic regression: dest V must match T (shrunk from neg_dest_type).
    #[test]
    fn test_encode_neon_across_long_regression_dest_type() {
        let ops = [dest_reg("b", 0), neon_arr(0, "8b")];
        assert!(
            encode_neon_across_long(&ops, 0, OPCODE).is_err(),
            "saddlv b0, v0.8b must Err (dest V must be H for T=8B)"
        );
    }
}

#[cfg(test)]
mod encode_neon_float_cmp_zero_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler
    // Evidence: src/backend/arm/assembler/README.md "accepts the same textual assembly that GCC's gas would consume";
    //   encoder/mod.rs:1-7 "Encodes AArch64 instructions into 32-bit machine code words";
    //   encoder/mod.rs:478-494 fcmeq/fcmge/fcmgt #0 and fcmle/fcmlt => encode_neon_float_cmp_zero;
    //   neon.rs:1574 Format 0 Q U 01110 size 10000 opcode 10 Rn Rd (float);
    //   ARM ARM Advanced SIMD two-register miscellaneous FP compare-with-zero (size=1sz)
    // Stronger considered:
    //   - State machine: rejected — encode_neon_float_cmp_zero is a pure function with no lifecycle
    //   - Algebraic round-trip: rejected as primary — no in-tree FCMEQ-zero decoder (field unpack kept as weaker algebraic)
    //   - Differential vs encode_neon_cmp_zero / encode_neon_float_two_misc: rejected — integer compare-zero / FP two-misc, different size map and opcodes (same-job gate)
    // Weaker available: algebraic.metamorphic (U bit, size_hi bit), algebraic.invariant (word layout), negative_error (arity / T / extra / mismatch)
    // Differential: candidate=encode_neon_float_cmp_zero, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=[RegArrangement(Vd, T), RegArrangement(Vn, T)] + (U, size_hi=1, opcode) <-> `{fcmeq|fcmge|fcmgt|fcmle|fcmlt} Vd.T, Vn.T, #0.0`

    use super::encode_neon_float_cmp_zero;
    use super::EncodeResult;
    use crate::backend::arm::assembler::parser::Operand;
    use proptest::prelude::*;
    use std::io::Write;
    use std::process::{Command, Stdio};

    const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
    const CASES: u32 = 1000;
    const SIZE_HI_FP: u32 = 1; // ARM ARM size = 1sz for FP compare-to-zero

    fn cfg() -> ProptestConfig {
        ProptestConfig::with_cases(CASES)
    }

    fn vreg(n: u32) -> String {
        format!("v{}", n)
    }

    fn neon_arr(reg: u32, arr: &str) -> Operand {
        Operand::RegArrangement {
            reg: vreg(reg),
            arrangement: arr.to_string(),
        }
    }

    fn q_sz_of(t: &str) -> (u32, u32) {
        match t {
            "2s" => (0, 0),
            "4s" => (1, 0),
            "2d" => (1, 1),
            _ => (0xff, 0xff),
        }
    }

    /// ARM-correct (U, opcode, mnemonic) for vector FCM* #0.0.
    fn insn_table() -> impl Strategy<Value = (u32, u32, &'static str)> {
        prop::sample::select(vec![
            (0u32, 0b01101u32, "fcmeq"),
            (1u32, 0b01100u32, "fcmge"),
            (0u32, 0b01100u32, "fcmgt"),
            (1u32, 0b01101u32, "fcmle"),
            (0u32, 0b01110u32, "fcmlt"),
        ])
    }

    fn sut_word(ops: &[Operand], u: u32, size_hi: u32, opcode: u32) -> Result<u32, String> {
        match encode_neon_float_cmp_zero(ops, u, size_hi, opcode)? {
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

    fn reg_num() -> impl Strategy<Value = u32> {
        prop_oneof![Just(0u32), Just(1u32), Just(30u32), Just(31u32), 0u32..=31]
    }

    fn valid_t() -> impl Strategy<Value = &'static str> {
        prop::sample::select(vec!["2s", "4s", "2d"])
    }

    fn opcode_bits() -> impl Strategy<Value = u32> {
        prop::sample::select(vec![0b01100u32, 0b01101u32, 0b01110u32])
    }

    /// Known-answer gate for the llvm-mc differential connection (FCMEQ 4S).
    #[test]
    fn encode_neon_float_cmp_zero_kat_llvm_mc_fcmeq_v0_4s_v1_4s() {
        let want = 0x4ea0d820u32;
        let mc = llvm_mc_word("fcmeq v0.4s, v1.4s, #0.0").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [neon_arr(0, "4s"), neon_arr(1, "4s")];
        let sut = sut_word(&ops, 0, SIZE_HI_FP, 0b01101).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    /// Known-answer: FCMGE 4S (U=1, opcode=01100).
    #[test]
    fn encode_neon_float_cmp_zero_kat_llvm_mc_fcmge_v0_4s_v1_4s() {
        let want = 0x6ea0c820u32;
        let mc = llvm_mc_word("fcmge v0.4s, v1.4s, #0.0").expect("llvm-mc KAT fcmge");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken for fcmge");
        let ops = [neon_arr(0, "4s"), neon_arr(1, "4s")];
        let sut = sut_word(&ops, 1, SIZE_HI_FP, 0b01100).expect("SUT KAT fcmge");
        assert_eq!(sut, want);
    }

    /// Known-answer: FCMLT 4S (U=0, opcode=01110) and FCMEQ 2S / 2D (Q/sz bounds).
    #[test]
    fn encode_neon_float_cmp_zero_kat_llvm_mc_fcmlt_and_width_bounds() {
        let want_lt = 0x4ea0e820u32;
        let mc_lt = llvm_mc_word("fcmlt v0.4s, v1.4s, #0.0").expect("llvm-mc KAT fcmlt");
        assert_eq!(mc_lt, want_lt, "llvm-mc KAT mapping broken for fcmlt");
        let ops4 = [neon_arr(0, "4s"), neon_arr(1, "4s")];
        let sut_lt = sut_word(&ops4, 0, SIZE_HI_FP, 0b01110).expect("SUT KAT fcmlt");
        assert_eq!(sut_lt, want_lt);

        let want_2s = 0x0ea0d820u32;
        let mc_2s = llvm_mc_word("fcmeq v0.2s, v1.2s, #0.0").expect("llvm-mc KAT 2s");
        assert_eq!(mc_2s, want_2s, "llvm-mc KAT mapping broken for 2s");
        let ops2s = [neon_arr(0, "2s"), neon_arr(1, "2s")];
        let sut_2s = sut_word(&ops2s, 0, SIZE_HI_FP, 0b01101).expect("SUT KAT 2s");
        assert_eq!(sut_2s, want_2s);

        let want_2d = 0x4ee0d820u32;
        let mc_2d = llvm_mc_word("fcmeq v0.2d, v1.2d, #0.0").expect("llvm-mc KAT 2d");
        assert_eq!(mc_2d, want_2d, "llvm-mc KAT mapping broken for 2d");
        let ops2d = [neon_arr(0, "2d"), neon_arr(1, "2d")];
        let sut_2d = sut_word(&ops2d, 0, SIZE_HI_FP, 0b01101).expect("SUT KAT 2d");
        assert_eq!(sut_2d, want_2d);
    }

    proptest! {
        #![proptest_config(cfg())]

        #[test]
        fn encode_neon_float_cmp_zero_diff_llvm_mc(
            rd in reg_num(),
            rn in reg_num(),
            t in valid_t(),
            insn in insn_table(),
        ) {
            let (u, opcode, mnem) = insn;
            let asm = format!("{} {}.{}, {}.{}, #0.0", mnem, vreg(rd), t, vreg(rn), t);
            let ops = [neon_arr(rd, t), neon_arr(rn, t)];
            let mc = llvm_mc_word(&asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
            let sut = sut_word(&ops, u, SIZE_HI_FP, opcode)
                .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
            prop_assert_eq!(sut, mc, "mismatch for {}", asm);
        }

        #[test]
        fn encode_neon_float_cmp_zero_roundtrip_arm_fields(
            rd in reg_num(),
            rn in reg_num(),
            t in valid_t(),
            u in 0u32..=1u32,
            size_hi in 0u32..=1u32,
            opcode in opcode_bits(),
        ) {
            let ops = [neon_arr(rd, t), neon_arr(rn, t)];
            let w = sut_word(&ops, u, size_hi, opcode)
                .unwrap_or_else(|e| panic!("SUT rejected valid field unpack: {}", e));
            let (q, sz) = q_sz_of(t);
            let size = (size_hi << 1) | sz;
            prop_assert_eq!((w >> 31) & 1, 0u32, "bit 31 must be 0");
            prop_assert_eq!((w >> 30) & 1, q, "Q bit");
            prop_assert_eq!((w >> 29) & 1, u, "U bit");
            prop_assert_eq!((w >> 24) & 0b11111, 0b01110u32, "bits[28:24]=01110");
            prop_assert_eq!((w >> 22) & 0b11, size, "size = (size_hi<<1)|sz");
            prop_assert_eq!((w >> 17) & 0b11111, 0b10000u32, "bits[21:17]=10000");
            prop_assert_eq!((w >> 12) & 0b11111, opcode, "opcode[16:12]");
            prop_assert_eq!((w >> 10) & 0b11, 0b10u32, "bits[11:10]=10");
            prop_assert_eq!((w >> 5) & 0b11111, rn, "Rn");
            prop_assert_eq!(w & 0b11111, rd, "Rd");
        }

        #[test]
        fn encode_neon_float_cmp_zero_metamorphic_u_bit(
            rd in reg_num(),
            rn in reg_num(),
            t in valid_t(),
            size_hi in 0u32..=1u32,
            opcode in opcode_bits(),
        ) {
            let ops = [neon_arr(rd, t), neon_arr(rn, t)];
            let a = sut_word(&ops, 0, size_hi, opcode)
                .unwrap_or_else(|e| panic!("SUT U=0 rejected: {}", e));
            let b = sut_word(&ops, 1, size_hi, opcode)
                .unwrap_or_else(|e| panic!("SUT U=1 rejected: {}", e));
            prop_assert_eq!(a ^ b, 1u32 << 29, "u must toggle only U (bit 29)");
        }

        #[test]
        fn encode_neon_float_cmp_zero_metamorphic_size_hi(
            rd in reg_num(),
            rn in reg_num(),
            t in valid_t(),
            u in 0u32..=1u32,
            opcode in opcode_bits(),
        ) {
            let ops = [neon_arr(rd, t), neon_arr(rn, t)];
            let a = sut_word(&ops, u, 0, opcode)
                .unwrap_or_else(|e| panic!("SUT size_hi=0 rejected: {}", e));
            let b = sut_word(&ops, u, 1, opcode)
                .unwrap_or_else(|e| panic!("SUT size_hi=1 rejected: {}", e));
            prop_assert_eq!(a ^ b, 1u32 << 23, "size_hi must toggle only bit 23");
        }

        #[test]
        fn encode_neon_float_cmp_zero_neg_unsupported_arrangement(
            rd in reg_num(),
            rn in reg_num(),
            u in 0u32..=1u32,
            size_hi in 0u32..=1u32,
            opcode in opcode_bits(),
            t in prop::sample::select(vec!["8b", "16b", "4h", "8h", "1d", "1s", "3s", "8s", "", "b", "h"]),
        ) {
            let asm = format!("fcmeq {}.{}, {}.{}, #0.0", vreg(rd), t, vreg(rn), t);
            if !t.is_empty() {
                prop_assert!(
                    llvm_mc_word(&asm).is_err(),
                    "llvm-mc unexpectedly accepted unsupported T {}",
                    asm
                );
            }
            let ops = [neon_arr(rd, t), neon_arr(rn, t)];
            prop_assert!(
                encode_neon_float_cmp_zero(&ops, u, size_hi, opcode).is_err(),
                "T={} is not 2s/4s/2d and must Err (llvm-mc rejects {})",
                t,
                asm
            );
        }

        #[test]
        fn encode_neon_float_cmp_zero_neg_extra_operands(
            rd in reg_num(),
            rn in reg_num(),
            extra in reg_num(),
            t in valid_t(),
            insn in insn_table(),
            extra_kind in 0u32..=3u32,
        ) {
            let (u, opcode, mnem) = insn;
            // Dispatch passes [Vd, Vn, Imm(0)] for fcmeq/fcmge/fcmgt #0.0.
            // A fourth operand is the extra that llvm-mc rejects.
            let extra_op = match extra_kind {
                0 => neon_arr(extra, t),
                1 => Operand::Imm(1),
                2 => Operand::Reg(vreg(extra)),
                _ => Operand::Mem {
                    base: "x0".into(),
                    offset: 0,
                },
            };
            let extra_asm = match extra_kind {
                0 => format!("{}.{}", vreg(extra), t),
                1 => "#1".to_string(),
                2 => vreg(extra),
                _ => "[x0]".to_string(),
            };
            let asm = format!(
                "{} {}.{}, {}.{}, #0.0, {}",
                mnem, vreg(rd), t, vreg(rn), t, extra_asm
            );
            prop_assert!(
                llvm_mc_word(&asm).is_err(),
                "llvm-mc unexpectedly accepted extra operand {}",
                asm
            );
            let ops = [
                neon_arr(rd, t),
                neon_arr(rn, t),
                Operand::Imm(0),
                extra_op,
            ];
            prop_assert!(
                encode_neon_float_cmp_zero(&ops, u, SIZE_HI_FP, opcode).is_err(),
                "4th operand must Err (llvm-mc rejects {})",
                asm
            );
        }

        #[test]
        fn encode_neon_float_cmp_zero_neg_arity_and_shape(
            n in 0usize..=1,
            which in 0u32..=5,
            u in 0u32..=1u32,
            rd in reg_num(),
            bad in prop_oneof![
                Just("v32".to_string()),
                Just("foo".to_string()),
                Just("".to_string()),
                Just("v".to_string()),
                Just("v-1".to_string()),
                Just("v99".to_string()),
            ],
        ) {
            let dest = neon_arr(rd, "4s");
            let src = neon_arr(rd, "4s");
            let short: Vec<Operand> = [dest.clone(), src.clone()].iter().take(n).cloned().collect();
            prop_assert!(
                encode_neon_float_cmp_zero(&short, u, SIZE_HI_FP, 0b01101).is_err(),
                "len={} must Err (requires 2 NEON registers)",
                n
            );
            let bad_src = match which {
                0 => Operand::Imm(0),
                1 => Operand::Mem {
                    base: "x0".into(),
                    offset: 0,
                },
                2 => Operand::Symbol("foo".into()),
                3 => Operand::Shift {
                    kind: "lsl".into(),
                    amount: 0,
                },
                4 => Operand::Cond("eq".into()),
                _ => Operand::Label(".L0".into()),
            };
            let ops_bad_src = [dest.clone(), bad_src];
            prop_assert!(
                encode_neon_float_cmp_zero(&ops_bad_src, u, SIZE_HI_FP, 0b01101).is_err(),
                "non-register source which={} must Err",
                which
            );
            let ops_bad_dest = [
                Operand::RegArrangement {
                    reg: bad.clone(),
                    arrangement: "4s".into(),
                },
                neon_arr(rd, "4s"),
            ];
            prop_assert!(
                encode_neon_float_cmp_zero(&ops_bad_dest, u, SIZE_HI_FP, 0b01101).is_err(),
                "invalid dest name {} must Err",
                bad
            );
            let ops_bad_rn = [
                neon_arr(rd, "4s"),
                Operand::RegArrangement {
                    reg: bad.clone(),
                    arrangement: "4s".into(),
                },
            ];
            prop_assert!(
                encode_neon_float_cmp_zero(&ops_bad_rn, u, SIZE_HI_FP, 0b01101).is_err(),
                "invalid src name {} must Err",
                bad
            );
            let bad_dest_shape = match which {
                0 => Operand::Imm(0),
                1 => Operand::Mem {
                    base: "x0".into(),
                    offset: 0,
                },
                2 => Operand::Symbol("foo".into()),
                3 => Operand::Shift {
                    kind: "lsl".into(),
                    amount: 0,
                },
                4 => Operand::Cond("eq".into()),
                _ => Operand::Label(".L0".into()),
            };
            let ops_bad_dest_shape = [bad_dest_shape, neon_arr(rd, "4s")];
            prop_assert!(
                encode_neon_float_cmp_zero(&ops_bad_dest_shape, u, SIZE_HI_FP, 0b01101).is_err(),
                "non-register dest which={} must Err",
                which
            );
            // Coverage sweep: get_neon_reg Operand::Reg arm (empty arrangement → dest match `_`).
            let ops_reg_dest = [Operand::Reg(vreg(rd)), neon_arr(rd, "4s")];
            prop_assert!(
                encode_neon_float_cmp_zero(&ops_reg_dest, u, SIZE_HI_FP, 0b01101).is_err(),
                "dest Operand::Reg (no arrangement) must Err"
            );
        }

        #[test]
        fn encode_neon_float_cmp_zero_neg_non_v_prefix(
            rd in reg_num(),
            rn in reg_num(),
            t in valid_t(),
            insn in insn_table(),
            prefix in prop::sample::select(vec!["x", "w", "d", "s", "q", "h", "b"]),
            which in 0u32..=1u32,
        ) {
            let (u, opcode, mnem) = insn;
            let bad_name = format!("{}{}", prefix, rd);
            let bad_op = Operand::RegArrangement {
                reg: bad_name.clone(),
                arrangement: t.to_string(),
            };
            let (ops, asm) = if which == 0 {
                (
                    [bad_op, neon_arr(rn, t)],
                    format!(
                        "{} {}.{}, {}.{}, #0.0",
                        mnem, bad_name, t, vreg(rn), t
                    ),
                )
            } else {
                (
                    [neon_arr(rd, t), bad_op],
                    format!(
                        "{} {}.{}, {}.{}, #0.0",
                        mnem, vreg(rd), t, bad_name, t
                    ),
                )
            };
            prop_assert!(
                llvm_mc_word(&asm).is_err(),
                "llvm-mc unexpectedly accepted non-v prefix {}",
                asm
            );
            prop_assert!(
                encode_neon_float_cmp_zero(&ops, u, SIZE_HI_FP, opcode).is_err(),
                "non-v prefix {} must Err (llvm-mc rejects {})",
                bad_name,
                asm
            );
        }

        #[test]
        fn encode_neon_float_cmp_zero_neg_arrangement_mismatch(
            rd in reg_num(),
            rn in reg_num(),
            td in valid_t(),
            tn in valid_t(),
            insn in insn_table(),
        ) {
            prop_assume!(td != tn);
            let (u, opcode, mnem) = insn;
            let asm = format!("{} {}.{}, {}.{}, #0.0", mnem, vreg(rd), td, vreg(rn), tn);
            prop_assert!(
                llvm_mc_word(&asm).is_err(),
                "llvm-mc unexpectedly accepted mismatched T {}",
                asm
            );
            let ops = [neon_arr(rd, td), neon_arr(rn, tn)];
            prop_assert!(
                encode_neon_float_cmp_zero(&ops, u, SIZE_HI_FP, opcode).is_err(),
                "dest T={} src T={} must Err (llvm-mc rejects {})",
                td,
                tn,
                asm
            );
        }
    }

    /// Deterministic regression: extra 4th operand (shrunk from neg_extra_operands).
    #[test]
    fn test_encode_neon_float_cmp_zero_regression_extra_operand() {
        let ops = [
            neon_arr(0, "2s"),
            neon_arr(0, "2s"),
            Operand::Imm(0),
            neon_arr(0, "2s"),
        ];
        assert!(
            encode_neon_float_cmp_zero(&ops, 0, SIZE_HI_FP, 0b01101).is_err(),
            "fcmeq v0.2s, v0.2s, #0.0, v0.2s must Err (no 4th operand)"
        );
    }

    /// Deterministic regression: dest T != src T (shrunk from neg_arrangement_mismatch).
    #[test]
    fn test_encode_neon_float_cmp_zero_regression_arrangement_mismatch() {
        let ops = [neon_arr(0, "4s"), neon_arr(0, "2s")];
        assert!(
            encode_neon_float_cmp_zero(&ops, 0, SIZE_HI_FP, 0b01101).is_err(),
            "fcmeq v0.4s, v0.2s, #0.0 must Err (dest T must equal src T)"
        );
    }

    /// Deterministic regression: non-V prefix (shrunk from neg_non_v_prefix).
    #[test]
    fn test_encode_neon_float_cmp_zero_regression_non_v_prefix() {
        let ops = [
            Operand::RegArrangement {
                reg: "x0".into(),
                arrangement: "2s".into(),
            },
            neon_arr(0, "2s"),
        ];
        assert!(
            encode_neon_float_cmp_zero(&ops, 0, SIZE_HI_FP, 0b01101).is_err(),
            "fcmeq x0.2s, v0.2s, #0.0 must Err (V register required)"
        );
    }
}

#[cfg(test)]
mod encode_neon_sli_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler
    // Evidence: src/backend/arm/assembler/README.md "accepts the same textual assembly that GCC's gas would consume";
    //   encoder/mod.rs:1-7 "Encodes AArch64 instructions into 32-bit machine code words";
    //   encoder/mod.rs:661 "sli" => encode_neon_sli;
    //   neon.rs:1257-1267 SLI Vd.T, Vn.T, #shift; 0 Q 1 0 11110 immh:immb 010101 Rn Rd (U=1);
    //   ARM ARM Advanced SIMD shift by immediate SLI (U=1, immh:immb = esize + shift)
    // Stronger considered:
    //   - State machine: rejected — encode_neon_sli is a pure function with no lifecycle
    //   - Algebraic round-trip: rejected as primary — no in-tree SLI decoder (field unpack kept as weaker algebraic)
    //   - Differential vs encode_neon_shl / encode_neon_shift_left_imm: rejected — SHL vs SLI / SQSHL helper (same-job gate)
    // Weaker available: algebraic.metamorphic (Q bit, shift+1), algebraic.invariant (word layout), negative_error (arity / T / extra / shift range / mismatch)
    // Differential: candidate=encode_neon_sli, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=[RegArrangement(Vd, T), RegArrangement(Vn, T), Imm(shift)] <-> `sli Vd.T, Vn.T, #shift`

    use super::encode_neon_sli;
    use super::EncodeResult;
    use crate::backend::arm::assembler::parser::Operand;
    use proptest::prelude::*;
    use std::io::Write;
    use std::process::{Command, Stdio};

    const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
    const CASES: u32 = 1000;

    fn cfg() -> ProptestConfig {
        ProptestConfig::with_cases(CASES)
    }

    fn vreg(n: u32) -> String {
        format!("v{}", n)
    }

    fn neon_arr(reg: u32, arr: &str) -> Operand {
        Operand::RegArrangement {
            reg: vreg(reg),
            arrangement: arr.to_string(),
        }
    }

    fn esize(t: &str) -> u32 {
        match t {
            "8b" | "16b" => 8,
            "4h" | "8h" => 16,
            "2s" | "4s" => 32,
            "2d" => 64,
            _ => 0,
        }
    }

    fn q_of(t: &str) -> u32 {
        match t {
            "8b" | "4h" | "2s" => 0,
            "16b" | "8h" | "4s" | "2d" => 1,
            _ => 0xff,
        }
    }

    fn sut_word(ops: &[Operand]) -> Result<u32, String> {
        match encode_neon_sli(ops)? {
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

    fn reg_num() -> impl Strategy<Value = u32> {
        prop_oneof![Just(0u32), Just(1u32), Just(30u32), Just(31u32), 0u32..=31]
    }

    fn valid_t() -> impl Strategy<Value = &'static str> {
        prop::sample::select(vec!["8b", "16b", "4h", "8h", "2s", "4s", "2d"])
    }

    /// Co-generate T with a shift in [0, esize-1], pinning 0 and esize-1.
    fn valid_t_shift() -> impl Strategy<Value = (&'static str, i64)> {
        valid_t().prop_flat_map(|t| {
            let e = esize(t) as i64;
            prop_oneof![
                Just(0i64),
                Just(e - 1),
                0i64..=(e - 1),
            ]
            .prop_map(move |s| (t, s))
        })
    }

    /// Shift in [0, esize-2] so shift+1 is still valid (bounds 0 and esize-2).
    fn valid_t_shift_inc() -> impl Strategy<Value = (&'static str, i64)> {
        valid_t().prop_flat_map(|t| {
            let e = esize(t) as i64;
            prop_oneof![
                Just(0i64),
                Just(e - 2),
                0i64..=(e - 2),
            ]
            .prop_map(move |s| (t, s))
        })
    }

    /// Out-of-range shift, pinning -1, esize, esize+1.
    fn oob_t_shift() -> impl Strategy<Value = (&'static str, i64)> {
        valid_t().prop_flat_map(|t| {
            let e = esize(t) as i64;
            prop_oneof![
                Just(-1i64),
                Just(e),
                Just(e + 1),
                Just(-2i64),
                Just(e * 2),
                Just(256i64),
                (-16i64..=-1),
                e..=(e + 16),
            ]
            .prop_map(move |s| (t, s))
        })
    }

    fn q_pair() -> impl Strategy<Value = (&'static str, &'static str)> {
        prop::sample::select(vec![("8b", "16b"), ("4h", "8h"), ("2s", "4s")])
    }

    /// Known-answer gate for the llvm-mc differential connection (8B shift 0 and 7).
    #[test]
    fn encode_neon_sli_kat_llvm_mc_v0_8b_v1_8b() {
        let want0 = 0x2f085420u32;
        let mc0 = llvm_mc_word("sli v0.8b, v1.8b, #0").expect("llvm-mc KAT #0");
        assert_eq!(mc0, want0, "llvm-mc KAT mapping broken for 8b #0");
        let ops0 = [neon_arr(0, "8b"), neon_arr(1, "8b"), Operand::Imm(0)];
        let sut0 = sut_word(&ops0).expect("SUT KAT #0");
        assert_eq!(sut0, want0);

        let want7 = 0x2f0f5420u32;
        let mc7 = llvm_mc_word("sli v0.8b, v1.8b, #7").expect("llvm-mc KAT #7");
        assert_eq!(mc7, want7, "llvm-mc KAT mapping broken for 8b #7");
        let ops7 = [neon_arr(0, "8b"), neon_arr(1, "8b"), Operand::Imm(7)];
        let sut7 = sut_word(&ops7).expect("SUT KAT #7");
        assert_eq!(sut7, want7);
    }

    /// Known-answer: 16B / 4H / 2D bounds (Q and esize).
    #[test]
    fn encode_neon_sli_kat_llvm_mc_width_bounds() {
        let want_16b = 0x6f0b5420u32;
        let mc_16b = llvm_mc_word("sli v0.16b, v1.16b, #3").expect("llvm-mc KAT 16b");
        assert_eq!(mc_16b, want_16b, "llvm-mc KAT mapping broken for 16b");
        let ops_16b = [neon_arr(0, "16b"), neon_arr(1, "16b"), Operand::Imm(3)];
        assert_eq!(sut_word(&ops_16b).expect("SUT KAT 16b"), want_16b);

        let want_4h = 0x2f1f5420u32;
        let mc_4h = llvm_mc_word("sli v0.4h, v1.4h, #15").expect("llvm-mc KAT 4h");
        assert_eq!(mc_4h, want_4h, "llvm-mc KAT mapping broken for 4h");
        let ops_4h = [neon_arr(0, "4h"), neon_arr(1, "4h"), Operand::Imm(15)];
        assert_eq!(sut_word(&ops_4h).expect("SUT KAT 4h"), want_4h);

        let want_2d0 = 0x6f405420u32;
        let mc_2d0 = llvm_mc_word("sli v0.2d, v1.2d, #0").expect("llvm-mc KAT 2d #0");
        assert_eq!(mc_2d0, want_2d0, "llvm-mc KAT mapping broken for 2d #0");
        let ops_2d0 = [neon_arr(0, "2d"), neon_arr(1, "2d"), Operand::Imm(0)];
        assert_eq!(sut_word(&ops_2d0).expect("SUT KAT 2d #0"), want_2d0);

        let want_2d63 = 0x6f7f5420u32;
        let mc_2d63 = llvm_mc_word("sli v0.2d, v1.2d, #63").expect("llvm-mc KAT 2d #63");
        assert_eq!(mc_2d63, want_2d63, "llvm-mc KAT mapping broken for 2d #63");
        let ops_2d63 = [neon_arr(0, "2d"), neon_arr(1, "2d"), Operand::Imm(63)];
        assert_eq!(sut_word(&ops_2d63).expect("SUT KAT 2d #63"), want_2d63);
    }

    proptest! {
        #![proptest_config(cfg())]

        #[test]
        fn encode_neon_sli_diff_llvm_mc(
            rd in reg_num(),
            rn in reg_num(),
            t_shift in valid_t_shift(),
        ) {
            let (t, shift) = t_shift;
            let asm = format!("sli {}.{}, {}.{}, #{}", vreg(rd), t, vreg(rn), t, shift);
            let ops = [neon_arr(rd, t), neon_arr(rn, t), Operand::Imm(shift)];
            let mc = llvm_mc_word(&asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
            let sut = sut_word(&ops)
                .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
            prop_assert_eq!(sut, mc, "mismatch for {}", asm);
        }

        #[test]
        fn encode_neon_sli_roundtrip_arm_fields(
            rd in reg_num(),
            rn in reg_num(),
            t_shift in valid_t_shift(),
        ) {
            let (t, shift) = t_shift;
            let ops = [neon_arr(rd, t), neon_arr(rn, t), Operand::Imm(shift)];
            let w = sut_word(&ops)
                .unwrap_or_else(|e| panic!("SUT rejected valid field unpack: {}", e));
            let q = q_of(t);
            let immh_immb = esize(t) + (shift as u32);
            prop_assert_eq!((w >> 31) & 1, 0u32, "bit 31 must be 0");
            prop_assert_eq!((w >> 30) & 1, q, "Q bit");
            prop_assert_eq!((w >> 29) & 1, 1u32, "U bit must be 1 for SLI");
            prop_assert_eq!((w >> 23) & 0b111111, 0b011110u32, "bits[28:23]=011110");
            prop_assert_eq!((w >> 16) & 0x7f, immh_immb, "immh:immb = esize + shift");
            prop_assert_eq!((w >> 10) & 0b111111, 0b010101u32, "bits[15:10]=010101");
            prop_assert_eq!((w >> 5) & 0b11111, rn, "Rn");
            prop_assert_eq!(w & 0b11111, rd, "Rd");
        }

        #[test]
        fn encode_neon_sli_metamorphic_q_bit(
            rd in reg_num(),
            rn in reg_num(),
            pair in q_pair(),
            t_shift in valid_t_shift(),
        ) {
            let (tlo, thi) = pair;
            let e = esize(tlo) as i64;
            // Reuse a shift that is valid for this pair's esize (ignore t_shift.0).
            let shift = t_shift.1.rem_euclid(e);
            let ops_lo = [neon_arr(rd, tlo), neon_arr(rn, tlo), Operand::Imm(shift)];
            let ops_hi = [neon_arr(rd, thi), neon_arr(rn, thi), Operand::Imm(shift)];
            let a = sut_word(&ops_lo)
                .unwrap_or_else(|e| panic!("SUT Q=0 rejected: {}", e));
            let b = sut_word(&ops_hi)
                .unwrap_or_else(|e| panic!("SUT Q=1 rejected: {}", e));
            prop_assert_eq!(a ^ b, 1u32 << 30, "Q pair {}.{} vs {}.{} must toggle only bit 30", tlo, shift, thi, shift);
        }

        #[test]
        fn encode_neon_sli_metamorphic_shift_inc(
            rd in reg_num(),
            rn in reg_num(),
            t_shift in valid_t_shift_inc(),
        ) {
            let (t, shift) = t_shift;
            let ops0 = [neon_arr(rd, t), neon_arr(rn, t), Operand::Imm(shift)];
            let ops1 = [neon_arr(rd, t), neon_arr(rn, t), Operand::Imm(shift + 1)];
            let a = sut_word(&ops0)
                .unwrap_or_else(|e| panic!("SUT shift={} rejected: {}", shift, e));
            let b = sut_word(&ops1)
                .unwrap_or_else(|e| panic!("SUT shift={} rejected: {}", shift + 1, e));
            prop_assert_eq!(
                b.wrapping_sub(a),
                1u32 << 16,
                "shift+1 must add 1 to immh:immb (bits [22:16]) and leave all other bits unchanged"
            );
        }

        #[test]
        fn encode_neon_sli_neg_extra_operands(
            rd in reg_num(),
            rn in reg_num(),
            extra in reg_num(),
            t_shift in valid_t_shift(),
            extra_kind in 0u32..=3u32,
        ) {
            let (t, shift) = t_shift;
            let extra_op = match extra_kind {
                0 => neon_arr(extra, t),
                1 => Operand::Imm(1),
                2 => Operand::Reg(vreg(extra)),
                _ => Operand::Mem {
                    base: "x0".into(),
                    offset: 0,
                },
            };
            let extra_asm = match extra_kind {
                0 => format!("{}.{}", vreg(extra), t),
                1 => "#1".to_string(),
                2 => vreg(extra),
                _ => "[x0]".to_string(),
            };
            let asm = format!(
                "sli {}.{}, {}.{}, #{}, {}",
                vreg(rd), t, vreg(rn), t, shift, extra_asm
            );
            prop_assert!(
                llvm_mc_word(&asm).is_err(),
                "llvm-mc unexpectedly accepted extra operand {}",
                asm
            );
            let ops = [
                neon_arr(rd, t),
                neon_arr(rn, t),
                Operand::Imm(shift),
                extra_op,
            ];
            prop_assert!(
                encode_neon_sli(&ops).is_err(),
                "4th operand must Err (llvm-mc rejects {})",
                asm
            );
        }

        #[test]
        fn encode_neon_sli_neg_shift_out_of_range(
            rd in reg_num(),
            rn in reg_num(),
            t_shift in oob_t_shift(),
        ) {
            let (t, shift) = t_shift;
            let e = esize(t) as i64;
            prop_assume!(shift < 0 || shift >= e);
            let asm = format!("sli {}.{}, {}.{}, #{}", vreg(rd), t, vreg(rn), t, shift);
            prop_assert!(
                llvm_mc_word(&asm).is_err(),
                "llvm-mc unexpectedly accepted oob shift {}",
                asm
            );
            let ops = [neon_arr(rd, t), neon_arr(rn, t), Operand::Imm(shift)];
            prop_assert!(
                encode_neon_sli(&ops).is_err(),
                "shift={} for T={} (esize={}) must Err (llvm-mc rejects {})",
                shift,
                t,
                e,
                asm
            );
        }

        #[test]
        fn encode_neon_sli_neg_arity_and_shape(
            n in 0usize..=2,
            which in 0u32..=5,
            rd in reg_num(),
            tbad in prop::sample::select(vec!["1d", "8s", "1s", "3s", "8s", "", "b", "h"]),
            bad in prop_oneof![
                Just("v32".to_string()),
                Just("foo".to_string()),
                Just("".to_string()),
                Just("v".to_string()),
                Just("v-1".to_string()),
                Just("v99".to_string()),
            ],
        ) {
            let dest = neon_arr(rd, "8b");
            let src = neon_arr(rd, "8b");
            let imm = Operand::Imm(0);
            let short: Vec<Operand> = [dest.clone(), src.clone(), imm.clone()].iter().take(n).cloned().collect();
            prop_assert!(
                encode_neon_sli(&short).is_err(),
                "len={} must Err (sli requires 3 operands)",
                n
            );
            if !tbad.is_empty() {
                let asm = format!("sli {}.{}, {}.{}, #0", vreg(rd), tbad, vreg(rd), tbad);
                prop_assert!(
                    llvm_mc_word(&asm).is_err(),
                    "llvm-mc unexpectedly accepted unsupported T {}",
                    asm
                );
            }
            let ops_bad_t = [neon_arr(rd, tbad), neon_arr(rd, tbad), Operand::Imm(0)];
            prop_assert!(
                encode_neon_sli(&ops_bad_t).is_err(),
                "T={} is not a valid SLI arrangement and must Err",
                tbad
            );
            let bad_src = match which {
                0 => Operand::Imm(0),
                1 => Operand::Mem {
                    base: "x0".into(),
                    offset: 0,
                },
                2 => Operand::Symbol("foo".into()),
                3 => Operand::Shift {
                    kind: "lsl".into(),
                    amount: 0,
                },
                4 => Operand::Cond("eq".into()),
                _ => Operand::Label(".L0".into()),
            };
            let ops_bad_src = [dest.clone(), bad_src, Operand::Imm(0)];
            prop_assert!(
                encode_neon_sli(&ops_bad_src).is_err(),
                "non-register source which={} must Err",
                which
            );
            let ops_bad_dest = [
                Operand::RegArrangement {
                    reg: bad.clone(),
                    arrangement: "8b".into(),
                },
                neon_arr(rd, "8b"),
                Operand::Imm(0),
            ];
            prop_assert!(
                encode_neon_sli(&ops_bad_dest).is_err(),
                "invalid dest name {} must Err",
                bad
            );
            let ops_bad_rn = [
                neon_arr(rd, "8b"),
                Operand::RegArrangement {
                    reg: bad.clone(),
                    arrangement: "8b".into(),
                },
                Operand::Imm(0),
            ];
            prop_assert!(
                encode_neon_sli(&ops_bad_rn).is_err(),
                "invalid src name {} must Err",
                bad
            );
            let bad_dest_shape = match which {
                0 => Operand::Imm(0),
                1 => Operand::Mem {
                    base: "x0".into(),
                    offset: 0,
                },
                2 => Operand::Symbol("foo".into()),
                3 => Operand::Shift {
                    kind: "lsl".into(),
                    amount: 0,
                },
                4 => Operand::Cond("eq".into()),
                _ => Operand::Label(".L0".into()),
            };
            let ops_bad_dest_shape = [bad_dest_shape, neon_arr(rd, "8b"), Operand::Imm(0)];
            prop_assert!(
                encode_neon_sli(&ops_bad_dest_shape).is_err(),
                "non-register dest which={} must Err",
                which
            );
            let ops_reg_dest = [Operand::Reg(vreg(rd)), neon_arr(rd, "8b"), Operand::Imm(0)];
            prop_assert!(
                encode_neon_sli(&ops_reg_dest).is_err(),
                "dest Operand::Reg (no arrangement) must Err"
            );
            let ops_non_imm = [neon_arr(rd, "8b"), neon_arr(rd, "8b"), Operand::Reg("x0".into())];
            prop_assert!(
                encode_neon_sli(&ops_non_imm).is_err(),
                "non-Imm shift operand must Err"
            );
        }

        #[test]
        fn encode_neon_sli_neg_arrangement_mismatch(
            rd in reg_num(),
            rn in reg_num(),
            td in valid_t(),
            tn in valid_t(),
            t_shift in valid_t_shift(),
        ) {
            let tn = if td == tn {
                if td == "8b" { "16b" } else { "8b" }
            } else {
                tn
            };
            let shift = t_shift.1.rem_euclid(esize(td) as i64);
            let asm = format!(
                "sli {}.{}, {}.{}, #{}",
                vreg(rd), td, vreg(rn), tn, shift
            );
            prop_assert!(
                llvm_mc_word(&asm).is_err(),
                "llvm-mc unexpectedly accepted mismatched T {}",
                asm
            );
            let ops = [neon_arr(rd, td), neon_arr(rn, tn), Operand::Imm(shift)];
            prop_assert!(
                encode_neon_sli(&ops).is_err(),
                "dest T={} src T={} must Err (llvm-mc rejects {})",
                td,
                tn,
                asm
            );
            // Coverage sweep: get_neon_reg Operand::Reg source (empty arrangement).
            let ops_reg_src = [neon_arr(rd, td), Operand::Reg(vreg(rn)), Operand::Imm(shift)];
            prop_assert!(
                encode_neon_sli(&ops_reg_src).is_err(),
                "src Operand::Reg (no arrangement) must Err"
            );
        }

        #[test]
        fn encode_neon_sli_neg_non_v_prefix(
            rd in reg_num(),
            rn in reg_num(),
            t_shift in valid_t_shift(),
            prefix in prop::sample::select(vec!["x", "w", "d", "s", "q", "h", "b"]),
            which in 0u32..=1u32,
        ) {
            let (t, shift) = t_shift;
            let bad_name = format!("{}{}", prefix, rd);
            let bad_op = Operand::RegArrangement {
                reg: bad_name.clone(),
                arrangement: t.to_string(),
            };
            let (ops, asm) = if which == 0 {
                (
                    [bad_op, neon_arr(rn, t), Operand::Imm(shift)],
                    format!(
                        "sli {}.{}, {}.{}, #{}",
                        bad_name, t, vreg(rn), t, shift
                    ),
                )
            } else {
                (
                    [neon_arr(rd, t), bad_op, Operand::Imm(shift)],
                    format!(
                        "sli {}.{}, {}.{}, #{}",
                        vreg(rd), t, bad_name, t, shift
                    ),
                )
            };
            prop_assert!(
                llvm_mc_word(&asm).is_err(),
                "llvm-mc unexpectedly accepted non-v prefix {}",
                asm
            );
            prop_assert!(
                encode_neon_sli(&ops).is_err(),
                "non-v prefix {} must Err (llvm-mc rejects {})",
                bad_name,
                asm
            );
        }
    }

    /// Deterministic regression: extra 4th operand (shrunk from neg_extra_operands).
    #[test]
    fn test_encode_neon_sli_regression_extra_operand() {
        let ops = [
            neon_arr(0, "8b"),
            neon_arr(0, "8b"),
            Operand::Imm(0),
            neon_arr(0, "8b"),
        ];
        assert!(
            encode_neon_sli(&ops).is_err(),
            "sli v0.8b, v0.8b, #0, v0.8b must Err (exactly 3 operands)"
        );
    }

    /// Deterministic regression: negative shift (shrunk from neg_shift_out_of_range).
    #[test]
    fn test_encode_neon_sli_regression_negative_shift() {
        let ops = [neon_arr(0, "8b"), neon_arr(0, "8b"), Operand::Imm(-1)];
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| encode_neon_sli(&ops)));
        assert!(
            matches!(result, Ok(Err(_))),
            "sli v0.8b, v0.8b, #-1 must Err, got {:?}",
            result
        );
    }

    /// Deterministic regression: shift = esize (8b #8) — mask wrap, not panic.
    #[test]
    fn test_encode_neon_sli_regression_shift_eq_esize() {
        let ops = [neon_arr(0, "8b"), neon_arr(0, "8b"), Operand::Imm(8)];
        assert!(
            encode_neon_sli(&ops).is_err(),
            "sli v0.8b, v0.8b, #8 must Err (shift in [0, 7])"
        );
    }

    /// Deterministic regression: dest/src arrangement mismatch (shrunk from neg_arrangement_mismatch).
    #[test]
    fn test_encode_neon_sli_regression_arrangement_mismatch() {
        let ops = [neon_arr(0, "8b"), neon_arr(0, "16b"), Operand::Imm(0)];
        assert!(
            encode_neon_sli(&ops).is_err(),
            "sli v0.8b, v0.16b, #0 must Err (T must match)"
        );
    }

    /// Deterministic regression: source Operand::Reg (coverage sweep).
    #[test]
    fn test_encode_neon_sli_regression_src_reg_no_arrangement() {
        let ops = [neon_arr(0, "8b"), Operand::Reg("v0".into()), Operand::Imm(0)];
        assert!(
            encode_neon_sli(&ops).is_err(),
            "sli v0.8b, v0, #0 must Err (source needs arrangement T)"
        );
    }

    /// Deterministic regression: non-V prefix (shrunk from neg_non_v_prefix).
    #[test]
    fn test_encode_neon_sli_regression_non_v_prefix() {
        let ops = [
            Operand::RegArrangement {
                reg: "x0".into(),
                arrangement: "8b".into(),
            },
            neon_arr(0, "8b"),
            Operand::Imm(0),
        ];
        assert!(
            encode_neon_sli(&ops).is_err(),
            "sli x0.8b, v0.8b, #0 must Err (V register required)"
        );
    }
}

#[cfg(test)]
mod encode_neon_float_three_same_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler
    // Evidence: src/backend/arm/assembler/README.md "accepts the same textual assembly that GCC's gas would consume";
    //   encoder/mod.rs:1-7 "Encodes AArch64 instructions into 32-bit machine code words";
    //   encoder/mod.rs:377-496 fadd/fsub/fmul/fdiv/fmax/fmin/fmaxnm/fminnm/fmla/fmls/frecps/frsqrts/fcmeq/fcmge/fcmgt/facge/facgt => encode_neon_float_three_same;
    //   neon.rs:1388 Format 0 Q U 01110 size 1 Rm opcode 1 Rn Rd;
    //   ARM ARM Advanced SIMD three-same FP (size[1]=size_hi, size[0]=sz)
    // Stronger considered:
    //   - State machine: rejected — encode_neon_float_three_same is a pure function with no lifecycle
    //   - Algebraic round-trip: rejected as primary — no in-tree FP three-same decoder (field unpack kept as weaker algebraic)
    //   - Differential vs encode_neon_three_same / encode_neon_float_cmp_zero / encode_fp_arith: rejected — integer three-same / compare-to-zero two-misc / scalar FP (same-job gate)
    // Weaker available: algebraic.metamorphic (U bit, size_hi bit, Q bit), algebraic.invariant (word layout), negative_error (arity / T / extra / mismatch / non-V)
    // Differential: candidate=encode_neon_float_three_same, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=[RegArrangement(Vd, T), RegArrangement(Vn, T), RegArrangement(Vm, T)] + (U, size_hi, opcode) <-> `{fadd|fsub|fmul|...} Vd.T, Vn.T, Vm.T`

    use super::encode_neon_float_three_same;
    use super::EncodeResult;
    use crate::backend::arm::assembler::parser::Operand;
    use proptest::prelude::*;
    use std::io::Write;
    use std::process::{Command, Stdio};

    const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
    const CASES: u32 = 1000;

    fn cfg() -> ProptestConfig {
        ProptestConfig::with_cases(CASES)
    }

    fn vreg(n: u32) -> String {
        format!("v{}", n)
    }

    fn neon_arr(reg: u32, arr: &str) -> Operand {
        Operand::RegArrangement {
            reg: vreg(reg),
            arrangement: arr.to_string(),
        }
    }

    fn q_sz_of(t: &str) -> (u32, u32) {
        match t {
            "2s" => (0, 0),
            "4s" => (1, 0),
            "2d" => (1, 1),
            _ => (0xff, 0xff),
        }
    }

    /// ARM-correct (U, size_hi, opcode, mnemonic) for vector FP three-same.
    fn insn_table() -> impl Strategy<Value = (u32, u32, u32, &'static str)> {
        prop::sample::select(vec![
            (0u32, 0u32, 0b11010u32, "fadd"),
            (0u32, 1u32, 0b11010u32, "fsub"),
            (1u32, 0u32, 0b11011u32, "fmul"),
            (1u32, 0u32, 0b11111u32, "fdiv"),
            (0u32, 0u32, 0b11110u32, "fmax"),
            (0u32, 1u32, 0b11110u32, "fmin"),
            (0u32, 0u32, 0b11000u32, "fmaxnm"),
            (0u32, 1u32, 0b11000u32, "fminnm"),
            (0u32, 0u32, 0b11001u32, "fmla"),
            (0u32, 1u32, 0b11001u32, "fmls"),
            (0u32, 0u32, 0b11111u32, "frecps"),
            (0u32, 1u32, 0b11111u32, "frsqrts"),
            (0u32, 0u32, 0b11100u32, "fcmeq"),
            (1u32, 0u32, 0b11100u32, "fcmge"),
            (1u32, 1u32, 0b11100u32, "fcmgt"),
            (1u32, 0u32, 0b11101u32, "facge"),
            (1u32, 1u32, 0b11101u32, "facgt"),
            (1u32, 1u32, 0b11010u32, "fabd"),
        ])
    }

    fn sut_word(ops: &[Operand], u: u32, size_hi: u32, opcode: u32) -> Result<u32, String> {
        match encode_neon_float_three_same(ops, u, size_hi, opcode)? {
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

    fn reg_num() -> impl Strategy<Value = u32> {
        prop_oneof![Just(0u32), Just(1u32), Just(30u32), Just(31u32), 0u32..=31]
    }

    fn valid_t() -> impl Strategy<Value = &'static str> {
        prop::sample::select(vec!["2s", "4s", "2d"])
    }

    /// Known-answer gate for the llvm-mc differential connection (FADD 4S).
    #[test]
    fn encode_neon_float_three_same_kat_llvm_mc_fadd_v0_4s_v1_4s_v2_4s() {
        let want = 0x4e22d420u32;
        let mc = llvm_mc_word("fadd v0.4s, v1.4s, v2.4s").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [neon_arr(0, "4s"), neon_arr(1, "4s"), neon_arr(2, "4s")];
        let sut = sut_word(&ops, 0, 0, 0b11010).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    /// Known-answer: FADD 2S / 2D (Q/sz bounds) and FSUB/FMUL 4S (size_hi / U).
    #[test]
    fn encode_neon_float_three_same_kat_llvm_mc_width_and_u_size_hi() {
        let want_2s = 0x0e22d420u32;
        let mc_2s = llvm_mc_word("fadd v0.2s, v1.2s, v2.2s").expect("llvm-mc KAT 2s");
        assert_eq!(mc_2s, want_2s, "llvm-mc KAT mapping broken for 2s");
        let ops2s = [neon_arr(0, "2s"), neon_arr(1, "2s"), neon_arr(2, "2s")];
        let sut_2s = sut_word(&ops2s, 0, 0, 0b11010).expect("SUT KAT 2s");
        assert_eq!(sut_2s, want_2s);

        let want_2d = 0x4e62d420u32;
        let mc_2d = llvm_mc_word("fadd v0.2d, v1.2d, v2.2d").expect("llvm-mc KAT 2d");
        assert_eq!(mc_2d, want_2d, "llvm-mc KAT mapping broken for 2d");
        let ops2d = [neon_arr(0, "2d"), neon_arr(1, "2d"), neon_arr(2, "2d")];
        let sut_2d = sut_word(&ops2d, 0, 0, 0b11010).expect("SUT KAT 2d");
        assert_eq!(sut_2d, want_2d);

        let want_sub = 0x4ea2d420u32;
        let mc_sub = llvm_mc_word("fsub v0.4s, v1.4s, v2.4s").expect("llvm-mc KAT fsub");
        assert_eq!(mc_sub, want_sub, "llvm-mc KAT mapping broken for fsub");
        let ops4 = [neon_arr(0, "4s"), neon_arr(1, "4s"), neon_arr(2, "4s")];
        let sut_sub = sut_word(&ops4, 0, 1, 0b11010).expect("SUT KAT fsub");
        assert_eq!(sut_sub, want_sub);

        let want_mul = 0x6e22dc20u32;
        let mc_mul = llvm_mc_word("fmul v0.4s, v1.4s, v2.4s").expect("llvm-mc KAT fmul");
        assert_eq!(mc_mul, want_mul, "llvm-mc KAT mapping broken for fmul");
        let sut_mul = sut_word(&ops4, 1, 0, 0b11011).expect("SUT KAT fmul");
        assert_eq!(sut_mul, want_mul);
    }

    proptest! {
        #![proptest_config(cfg())]

        #[test]
        fn encode_neon_float_three_same_diff_llvm_mc(
            rd in reg_num(),
            rn in reg_num(),
            rm in reg_num(),
            t in valid_t(),
            insn in insn_table(),
        ) {
            let (u, size_hi, opcode, mnem) = insn;
            let asm = format!(
                "{0} {1}.{2}, {3}.{2}, {4}.{2}",
                mnem, vreg(rd), t, vreg(rn), vreg(rm)
            );
            let ops = [neon_arr(rd, t), neon_arr(rn, t), neon_arr(rm, t)];
            let mc = llvm_mc_word(&asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
            let sut = sut_word(&ops, u, size_hi, opcode)
                .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
            prop_assert_eq!(sut, mc, "mismatch for {}", asm);
        }

        #[test]
        fn encode_neon_float_three_same_roundtrip_arm_fields(
            rd in reg_num(),
            rn in reg_num(),
            rm in reg_num(),
            t in valid_t(),
            u in 0u32..=1u32,
            size_hi in 0u32..=1u32,
            opcode in 0u32..=31u32,
        ) {
            let ops = [neon_arr(rd, t), neon_arr(rn, t), neon_arr(rm, t)];
            let w = sut_word(&ops, u, size_hi, opcode)
                .unwrap_or_else(|e| panic!("SUT rejected valid field unpack: {}", e));
            let (q, sz) = q_sz_of(t);
            let size = (size_hi << 1) | sz;
            prop_assert_eq!((w >> 31) & 1, 0u32, "bit 31 must be 0");
            prop_assert_eq!((w >> 30) & 1, q, "Q bit");
            prop_assert_eq!((w >> 29) & 1, u, "U bit");
            prop_assert_eq!((w >> 24) & 0b11111, 0b01110u32, "bits[28:24]=01110");
            prop_assert_eq!((w >> 22) & 0b11, size, "size = (size_hi<<1)|sz");
            prop_assert_eq!((w >> 21) & 1, 1u32, "bit 21 must be 1");
            prop_assert_eq!((w >> 16) & 0b11111, rm, "Rm");
            prop_assert_eq!((w >> 11) & 0b11111, opcode, "opcode[15:11]");
            prop_assert_eq!((w >> 10) & 1, 1u32, "bit 10 must be 1");
            prop_assert_eq!((w >> 5) & 0b11111, rn, "Rn");
            prop_assert_eq!(w & 0b11111, rd, "Rd");
        }

        #[test]
        fn encode_neon_float_three_same_metamorphic_u_bit(
            rd in reg_num(),
            rn in reg_num(),
            rm in reg_num(),
            t in valid_t(),
            size_hi in 0u32..=1u32,
            opcode in 0u32..=31u32,
        ) {
            let ops = [neon_arr(rd, t), neon_arr(rn, t), neon_arr(rm, t)];
            let a = sut_word(&ops, 0, size_hi, opcode)
                .unwrap_or_else(|e| panic!("SUT U=0 rejected: {}", e));
            let b = sut_word(&ops, 1, size_hi, opcode)
                .unwrap_or_else(|e| panic!("SUT U=1 rejected: {}", e));
            prop_assert_eq!(a ^ b, 1u32 << 29, "u must toggle only U (bit 29)");
        }

        #[test]
        fn encode_neon_float_three_same_metamorphic_size_hi(
            rd in reg_num(),
            rn in reg_num(),
            rm in reg_num(),
            t in valid_t(),
            u in 0u32..=1u32,
            opcode in 0u32..=31u32,
        ) {
            let ops = [neon_arr(rd, t), neon_arr(rn, t), neon_arr(rm, t)];
            let a = sut_word(&ops, u, 0, opcode)
                .unwrap_or_else(|e| panic!("SUT size_hi=0 rejected: {}", e));
            let b = sut_word(&ops, u, 1, opcode)
                .unwrap_or_else(|e| panic!("SUT size_hi=1 rejected: {}", e));
            prop_assert_eq!(a ^ b, 1u32 << 23, "size_hi must toggle only bit 23");
        }

        #[test]
        fn encode_neon_float_three_same_metamorphic_q(
            rd in reg_num(),
            rn in reg_num(),
            rm in reg_num(),
            u in 0u32..=1u32,
            size_hi in 0u32..=1u32,
            opcode in 0u32..=31u32,
        ) {
            let ops_2s = [neon_arr(rd, "2s"), neon_arr(rn, "2s"), neon_arr(rm, "2s")];
            let ops_4s = [neon_arr(rd, "4s"), neon_arr(rn, "4s"), neon_arr(rm, "4s")];
            let a = sut_word(&ops_2s, u, size_hi, opcode)
                .unwrap_or_else(|e| panic!("SUT 2s rejected: {}", e));
            let b = sut_word(&ops_4s, u, size_hi, opcode)
                .unwrap_or_else(|e| panic!("SUT 4s rejected: {}", e));
            prop_assert_eq!(a ^ b, 1u32 << 30, "2s vs 4s must toggle only Q (bit 30)");
        }

        #[test]
        fn encode_neon_float_three_same_neg_unsupported_arrangement(
            rd in reg_num(),
            rn in reg_num(),
            rm in reg_num(),
            u in 0u32..=1u32,
            size_hi in 0u32..=1u32,
            opcode in 0u32..=31u32,
            t in prop::sample::select(vec!["8b", "16b", "4h", "8h", "1d", "1s", "3s", "8s", "", "b", "h"]),
        ) {
            let asm = format!(
                "fadd {0}.{1}, {2}.{1}, {3}.{1}",
                vreg(rd), t, vreg(rn), vreg(rm)
            );
            if !t.is_empty() {
                prop_assert!(
                    llvm_mc_word(&asm).is_err(),
                    "llvm-mc unexpectedly accepted unsupported T {}",
                    asm
                );
            }
            let ops = [neon_arr(rd, t), neon_arr(rn, t), neon_arr(rm, t)];
            prop_assert!(
                encode_neon_float_three_same(&ops, u, size_hi, opcode).is_err(),
                "T={} is not 2s/4s/2d and must Err (llvm-mc rejects {})",
                t,
                asm
            );
        }

        #[test]
        fn encode_neon_float_three_same_neg_extra_operands(
            rd in reg_num(),
            rn in reg_num(),
            rm in reg_num(),
            extra in reg_num(),
            t in valid_t(),
            insn in insn_table(),
            extra_kind in 0u32..=3u32,
        ) {
            let (u, size_hi, opcode, mnem) = insn;
            let extra_op = match extra_kind {
                0 => neon_arr(extra, t),
                1 => Operand::Imm(1),
                2 => Operand::Reg(vreg(extra)),
                _ => Operand::Mem {
                    base: "x0".into(),
                    offset: 0,
                },
            };
            let extra_asm = match extra_kind {
                0 => format!("{}.{}", vreg(extra), t),
                1 => "#1".to_string(),
                2 => vreg(extra),
                _ => "[x0]".to_string(),
            };
            let asm = format!(
                "{} {}.{}, {}.{}, {}.{}, {}",
                mnem, vreg(rd), t, vreg(rn), t, vreg(rm), t, extra_asm
            );
            prop_assert!(
                llvm_mc_word(&asm).is_err(),
                "llvm-mc unexpectedly accepted extra operand {}",
                asm
            );
            let ops = [
                neon_arr(rd, t),
                neon_arr(rn, t),
                neon_arr(rm, t),
                extra_op,
            ];
            prop_assert!(
                encode_neon_float_three_same(&ops, u, size_hi, opcode).is_err(),
                "4th operand must Err (llvm-mc rejects {})",
                asm
            );
        }

        #[test]
        fn encode_neon_float_three_same_neg_arity_and_shape(
            n in 0usize..=2,
            which in 0u32..=5,
            u in 0u32..=1u32,
            rd in reg_num(),
            t in valid_t(),
            bad in prop_oneof![
                Just("v32".to_string()),
                Just("foo".to_string()),
                Just("".to_string()),
                Just("v".to_string()),
                Just("v-1".to_string()),
                Just("v99".to_string()),
            ],
        ) {
            let dest = neon_arr(rd, t);
            let srcn = neon_arr(rd, t);
            let srcm = neon_arr(rd, t);
            let short: Vec<Operand> = [dest.clone(), srcn.clone(), srcm.clone()]
                .iter()
                .take(n)
                .cloned()
                .collect();
            prop_assert!(
                encode_neon_float_three_same(&short, u, 0, 0b11010).is_err(),
                "len={} must Err (requires 3 NEON registers)",
                n
            );

            let bad_shape = match which {
                0 => Operand::Imm(0),
                1 => Operand::Mem {
                    base: "x0".into(),
                    offset: 0,
                },
                2 => Operand::Symbol("foo".into()),
                3 => Operand::Shift {
                    kind: "lsl".into(),
                    amount: 0,
                },
                4 => Operand::Cond("eq".into()),
                _ => Operand::Label(".L0".into()),
            };
            let mut ops_bad = [dest.clone(), srcn.clone(), srcm.clone()];
            ops_bad[which as usize % 3] = bad_shape;
            prop_assert!(
                encode_neon_float_three_same(&ops_bad, u, 0, 0b11010).is_err(),
                "non-register operand which={} must Err",
                which
            );

            let bad_named = Operand::RegArrangement {
                reg: bad.clone(),
                arrangement: t.to_string(),
            };
            let mut ops_bad_name = [dest.clone(), srcn.clone(), srcm.clone()];
            ops_bad_name[which as usize % 3] = bad_named;
            prop_assert!(
                encode_neon_float_three_same(&ops_bad_name, u, 0, 0b11010).is_err(),
                "invalid name {} must Err",
                bad
            );

            // dest Operand::Reg (empty arrangement) must Err.
            let ops_reg_dest = [Operand::Reg(vreg(rd)), srcn.clone(), srcm.clone()];
            prop_assert!(
                encode_neon_float_three_same(&ops_reg_dest, u, 0, 0b11010).is_err(),
                "dest Operand::Reg (no arrangement) must Err"
            );
        }

        #[test]
        fn encode_neon_float_three_same_neg_src_reg_no_arrangement(
            rd in reg_num(),
            rn in reg_num(),
            rm in reg_num(),
            t in valid_t(),
            insn in insn_table(),
            which in 1u32..=2u32,
        ) {
            let (u, size_hi, opcode, mnem) = insn;
            let dest = neon_arr(rd, t);
            let srcn = neon_arr(rn, t);
            let srcm = neon_arr(rm, t);
            let mut ops = [dest, srcn, srcm];
            ops[which as usize] = Operand::Reg(vreg(if which == 1 { rn } else { rm }));
            let asm = if which == 1 {
                format!("{0} {1}.{2}, {3}, {4}.{2}", mnem, vreg(rd), t, vreg(rn), vreg(rm))
            } else {
                format!("{0} {1}.{2}, {3}.{2}, {4}", mnem, vreg(rd), t, vreg(rn), vreg(rm))
            };
            prop_assert!(
                llvm_mc_word(&asm).is_err(),
                "llvm-mc unexpectedly accepted {}",
                asm
            );
            prop_assert!(
                encode_neon_float_three_same(&ops, u, size_hi, opcode).is_err(),
                "src Operand::Reg (no arrangement) must Err (llvm-mc rejects {})",
                asm
            );
        }

        #[test]
        fn encode_neon_float_three_same_neg_non_v_prefix(
            rd in reg_num(),
            rn in reg_num(),
            rm in reg_num(),
            t in valid_t(),
            insn in insn_table(),
            prefix in prop::sample::select(vec!["x", "w", "d", "s", "q", "h", "b"]),
            slot in 0u32..=2u32,
        ) {
            let (u, size_hi, opcode, mnem) = insn;
            let dest = neon_arr(rd, t);
            let srcn = neon_arr(rn, t);
            let srcm = neon_arr(rm, t);
            let num = if slot == 0 { rd } else if slot == 1 { rn } else { rm };
            let bad_name = format!("{}{}", prefix, num);
            let bad_op = Operand::RegArrangement {
                reg: bad_name.clone(),
                arrangement: t.to_string(),
            };
            let mut ops = [dest, srcn, srcm];
            ops[slot as usize] = bad_op;
            let n0 = if slot == 0 { bad_name.clone() } else { vreg(rd) };
            let n1 = if slot == 1 { bad_name.clone() } else { vreg(rn) };
            let n2 = if slot == 2 { bad_name.clone() } else { vreg(rm) };
            let asm = format!("{0} {1}.{4}, {2}.{4}, {3}.{4}", mnem, n0, n1, n2, t);
            prop_assert!(
                llvm_mc_word(&asm).is_err(),
                "llvm-mc unexpectedly accepted non-v prefix {}",
                asm
            );
            prop_assert!(
                encode_neon_float_three_same(&ops, u, size_hi, opcode).is_err(),
                "non-v prefix {} must Err (llvm-mc rejects {})",
                bad_name,
                asm
            );
        }

        #[test]
        fn encode_neon_float_three_same_neg_arrangement_mismatch(
            rd in reg_num(),
            rn in reg_num(),
            rm in reg_num(),
            td in valid_t(),
            tn in valid_t(),
            tm in valid_t(),
            insn in insn_table(),
        ) {
            prop_assume!(td != tn || td != tm);
            let (u, size_hi, opcode, mnem) = insn;
            let asm = format!(
                "{0} {1}.{2}, {3}.{4}, {5}.{6}",
                mnem, vreg(rd), td, vreg(rn), tn, vreg(rm), tm
            );
            prop_assert!(
                llvm_mc_word(&asm).is_err(),
                "llvm-mc unexpectedly accepted mismatched T {}",
                asm
            );
            let ops = [neon_arr(rd, td), neon_arr(rn, tn), neon_arr(rm, tm)];
            prop_assert!(
                encode_neon_float_three_same(&ops, u, size_hi, opcode).is_err(),
                "T mismatch dest={} vn={} vm={} must Err (llvm-mc rejects {})",
                td,
                tn,
                tm,
                asm
            );
        }
    }

    /// Deterministic regression: extra 4th operand (shrunk from neg_extra_operands).
    #[test]
    fn test_encode_neon_float_three_same_regression_extra_operand() {
        let ops = [
            neon_arr(0, "2s"),
            neon_arr(0, "2s"),
            neon_arr(0, "2s"),
            neon_arr(0, "2s"),
        ];
        assert!(
            encode_neon_float_three_same(&ops, 0, 0, 0b11010).is_err(),
            "fadd v0.2s, v0.2s, v0.2s, v0.2s must Err (no 4th operand)"
        );
    }

    /// Deterministic regression: source Operand::Reg (shrunk from neg_src_reg_no_arrangement).
    #[test]
    fn test_encode_neon_float_three_same_regression_src_reg_no_arrangement() {
        let ops = [
            neon_arr(0, "2s"),
            Operand::Reg("v0".into()),
            neon_arr(0, "2s"),
        ];
        assert!(
            encode_neon_float_three_same(&ops, 0, 0, 0b11010).is_err(),
            "fadd v0.2s, v0, v0.2s must Err (source needs arrangement T)"
        );
    }

    /// Deterministic regression: non-V prefix (shrunk from neg_non_v_prefix).
    #[test]
    fn test_encode_neon_float_three_same_regression_non_v_prefix() {
        let ops = [
            Operand::RegArrangement {
                reg: "x0".into(),
                arrangement: "2s".into(),
            },
            neon_arr(0, "2s"),
            neon_arr(0, "2s"),
        ];
        assert!(
            encode_neon_float_three_same(&ops, 0, 0, 0b11010).is_err(),
            "fadd x0.2s, v0.2s, v0.2s must Err (V register required)"
        );
    }

    /// Deterministic regression: dest T != src T (shrunk from neg_arrangement_mismatch).
    #[test]
    fn test_encode_neon_float_three_same_regression_arrangement_mismatch() {
        let ops = [neon_arr(0, "2d"), neon_arr(0, "2s"), neon_arr(0, "2s")];
        assert!(
            encode_neon_float_three_same(&ops, 0, 0, 0b11010).is_err(),
            "fadd v0.2d, v0.2s, v0.2s must Err (T must match)"
        );
    }
}

#[cfg(test)]
mod encode_neon_qshrn_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler
    // Evidence: src/backend/arm/assembler/README.md "accepts the same textual assembly that GCC's gas would consume";
    //   encoder/mod.rs:1-7 "Encodes AArch64 instructions into 32-bit machine code words";
    //   encoder/mod.rs:637-648 sqshrn/uqshrn/sqrshrn/uqrshrn (+2) => encode_neon_qshrn;
    //   neon.rs:1495-1510 0 Q U 011110 immh:immb opcode Rn Rd; opcode 100101 / 100111;
    //   ARM ARM Advanced SIMD shift by immediate SQSHRN/UQSHRN/SQRSHRN/UQRSHRN;
    //   sibling encode_neon_shrn neon.rs:1443-1444 half_bits = source/2 (dest element size)
    // Stronger considered:
    //   - State machine: rejected — encode_neon_qshrn is a pure function with no lifecycle
    //   - Algebraic round-trip: rejected — no in-tree QSHRN decoder
    //   - Differential vs encode_neon_shrn / encode_neon_sqshrun / encode_neon_scalar_qshrn:
    //     rejected — different opcode / signed-to-unsigned / scalar vs vector (same-job gate)
    // Weaker available: algebraic.metamorphic (Q/U/rounding bits), algebraic.invariant (word layout),
    //   negative_error (arity / Ta / Tb / shift range / extra / non-reg)
    // Differential: candidate=encode_neon_qshrn, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=[RegArrangement(Vd,Tb), RegArrangement(Vn,Ta), Imm(shift)] <-> `{mnem} Vd.Tb, Vn.Ta, #shift`

    use super::encode_neon_qshrn;
    use super::EncodeResult;
    use crate::backend::arm::assembler::parser::Operand;
    use proptest::prelude::*;
    use std::io::Write;
    use std::process::{Command, Stdio};

    const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
    const CASES: u32 = 1000;

    fn cfg() -> ProptestConfig {
        ProptestConfig::with_cases(CASES)
    }

    fn vreg(n: u32) -> String {
        format!("v{}", n)
    }

    fn neon_arr(reg: u32, arr: &str) -> Operand {
        Operand::RegArrangement {
            reg: vreg(reg),
            arrangement: arr.to_string(),
        }
    }

    fn mandated_tb(ta: &str, is_high: bool) -> &'static str {
        match (ta, is_high) {
            ("8h", false) => "8b",
            ("8h", true) => "16b",
            ("4s", false) => "4h",
            ("4s", true) => "8h",
            ("2d", false) => "2s",
            ("2d", true) => "4s",
            _ => "8b",
        }
    }

    fn src_esize(ta: &str) -> u32 {
        match ta {
            "8h" => 16,
            "4s" => 32,
            "2d" => 64,
            _ => 0,
        }
    }

    fn dest_esize(ta: &str) -> u32 {
        src_esize(ta) / 2
    }

    fn mnemonic(u_bit: u32, is_rounding: bool, is_high: bool) -> &'static str {
        match (u_bit, is_rounding, is_high) {
            (0, false, false) => "sqshrn",
            (0, false, true) => "sqshrn2",
            (1, false, false) => "uqshrn",
            (1, false, true) => "uqshrn2",
            (0, true, false) => "sqrshrn",
            (0, true, true) => "sqrshrn2",
            (1, true, false) => "uqrshrn",
            (1, true, true) => "uqrshrn2",
            _ => "sqshrn",
        }
    }

    fn opcode_bits(is_rounding: bool) -> u32 {
        if is_rounding {
            0b100111
        } else {
            0b100101
        }
    }

    fn sut_word(
        ops: &[Operand],
        u_bit: u32,
        is_rounding: bool,
        is_high: bool,
    ) -> Result<u32, String> {
        match encode_neon_qshrn(ops, u_bit, is_rounding, is_high)? {
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

    fn reg_num() -> impl Strategy<Value = u32> {
        prop_oneof![Just(0u32), Just(1u32), Just(30u32), Just(31u32), 0u32..=31]
    }

    fn ta_arr() -> impl Strategy<Value = &'static str> {
        prop::sample::select(vec!["8h", "4s", "2d"])
    }

    /// Co-generate Ta with a shift in [1, dest_esize], pinning 1 and dest_esize.
    fn valid_ta_shift() -> impl Strategy<Value = (&'static str, i64)> {
        ta_arr().prop_flat_map(|ta| {
            let d = dest_esize(ta) as i64;
            prop_oneof![Just(1i64), Just(d), 1i64..=d].prop_map(move |s| (ta, s))
        })
    }

    /// Out-of-range shift, pinning -1, 0, dest_esize+1, src_esize, src_esize+1.
    fn oob_ta_shift() -> impl Strategy<Value = (&'static str, i64)> {
        ta_arr().prop_flat_map(|ta| {
            let d = dest_esize(ta) as i64;
            let src = src_esize(ta) as i64;
            prop_oneof![
                Just(-1i64),
                Just(0i64),
                Just(d + 1),
                Just(src),
                Just(src + 1),
                Just(256i64),
                (-16i64..=-1),
                (d + 1)..=src,
                (src + 1)..=(src + 16),
            ]
            .prop_map(move |s| (ta, s))
        })
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_neon_qshrn_kat_llvm_mc_sqshrn_v0_8b_v1_8h() {
        let want = 0x0f0f9420u32;
        let mc = llvm_mc_word("sqshrn v0.8b, v1.8h, #1").expect("llvm-mc KAT #1");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken for sqshrn #1");
        let ops = [neon_arr(0, "8b"), neon_arr(1, "8h"), Operand::Imm(1)];
        let sut = sut_word(&ops, 0, false, false).expect("SUT KAT #1");
        assert_eq!(sut, want);

        let want8 = 0x0f089420u32;
        let mc8 = llvm_mc_word("sqshrn v0.8b, v1.8h, #8").expect("llvm-mc KAT #8");
        assert_eq!(mc8, want8, "llvm-mc KAT mapping broken for sqshrn #8");
        let ops8 = [neon_arr(0, "8b"), neon_arr(1, "8h"), Operand::Imm(8)];
        assert_eq!(sut_word(&ops8, 0, false, false).expect("SUT KAT #8"), want8);
    }

    /// Known-answer: Q / U / rounding / 2d bounds.
    #[test]
    fn encode_neon_qshrn_kat_llvm_mc_variants() {
        let want_2 = 0x4f0f9420u32;
        let mc_2 = llvm_mc_word("sqshrn2 v0.16b, v1.8h, #1").expect("llvm-mc KAT sqshrn2");
        assert_eq!(mc_2, want_2, "llvm-mc KAT mapping broken for sqshrn2");
        let ops_2 = [neon_arr(0, "16b"), neon_arr(1, "8h"), Operand::Imm(1)];
        assert_eq!(sut_word(&ops_2, 0, false, true).expect("SUT KAT sqshrn2"), want_2);

        let want_u = 0x2f0f9420u32;
        let mc_u = llvm_mc_word("uqshrn v0.8b, v1.8h, #1").expect("llvm-mc KAT uqshrn");
        assert_eq!(mc_u, want_u, "llvm-mc KAT mapping broken for uqshrn");
        let ops_u = [neon_arr(0, "8b"), neon_arr(1, "8h"), Operand::Imm(1)];
        assert_eq!(sut_word(&ops_u, 1, false, false).expect("SUT KAT uqshrn"), want_u);

        let want_r = 0x0f0f9c20u32;
        let mc_r = llvm_mc_word("sqrshrn v0.8b, v1.8h, #1").expect("llvm-mc KAT sqrshrn");
        assert_eq!(mc_r, want_r, "llvm-mc KAT mapping broken for sqrshrn");
        let ops_r = [neon_arr(0, "8b"), neon_arr(1, "8h"), Operand::Imm(1)];
        assert_eq!(sut_word(&ops_r, 0, true, false).expect("SUT KAT sqrshrn"), want_r);

        let want_ur2 = 0x6f209c20u32;
        let mc_ur2 = llvm_mc_word("uqrshrn2 v0.4s, v1.2d, #32").expect("llvm-mc KAT uqrshrn2");
        assert_eq!(mc_ur2, want_ur2, "llvm-mc KAT mapping broken for uqrshrn2");
        let ops_ur2 = [neon_arr(0, "4s"), neon_arr(1, "2d"), Operand::Imm(32)];
        assert_eq!(
            sut_word(&ops_ur2, 1, true, true).expect("SUT KAT uqrshrn2"),
            want_ur2
        );

        let want_4h = 0x0f109420u32;
        let mc_4h = llvm_mc_word("sqshrn v0.4h, v1.4s, #16").expect("llvm-mc KAT 4h #16");
        assert_eq!(mc_4h, want_4h, "llvm-mc KAT mapping broken for 4h #16");
        let ops_4h = [neon_arr(0, "4h"), neon_arr(1, "4s"), Operand::Imm(16)];
        assert_eq!(sut_word(&ops_4h, 0, false, false).expect("SUT KAT 4h"), want_4h);
    }

    proptest! {
        #![proptest_config(cfg())]

        #[test]
        fn encode_neon_qshrn_diff_llvm_mc(
            rd in reg_num(),
            rn in reg_num(),
            ta_shift in valid_ta_shift(),
            is_high in any::<bool>(),
            u_bit in 0u32..=1u32,
            is_rounding in any::<bool>(),
        ) {
            let (ta, shift) = ta_shift;
            let tb = mandated_tb(ta, is_high);
            let mnem = mnemonic(u_bit, is_rounding, is_high);
            let asm = format!(
                "{} {}.{}, {}.{}, #{}",
                mnem, vreg(rd), tb, vreg(rn), ta, shift
            );
            let ops = [neon_arr(rd, tb), neon_arr(rn, ta), Operand::Imm(shift)];
            let mc = llvm_mc_word(&asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
            let sut = sut_word(&ops, u_bit, is_rounding, is_high)
                .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
            prop_assert_eq!(sut, mc, "mismatch for {}", asm);
        }

        #[test]
        fn encode_neon_qshrn_metamorphic_q(
            rd in reg_num(),
            rn in reg_num(),
            ta_shift in valid_ta_shift(),
            u_bit in 0u32..=1u32,
            is_rounding in any::<bool>(),
        ) {
            let (ta, shift) = ta_shift;
            let tb = mandated_tb(ta, false);
            let ops = [neon_arr(rd, tb), neon_arr(rn, ta), Operand::Imm(shift)];
            let lo = sut_word(&ops, u_bit, is_rounding, false)
                .unwrap_or_else(|e| panic!("SUT Q=0 rejected: {}", e));
            let hi = sut_word(&ops, u_bit, is_rounding, true)
                .unwrap_or_else(|e| panic!("SUT Q=1 rejected: {}", e));
            prop_assert_eq!(lo ^ hi, 1u32 << 30, "is_high must toggle only Q (bit 30)");
        }

        #[test]
        fn encode_neon_qshrn_metamorphic_u_round(
            rd in reg_num(),
            rn in reg_num(),
            ta_shift in valid_ta_shift(),
            is_high in any::<bool>(),
        ) {
            let (ta, shift) = ta_shift;
            let tb = mandated_tb(ta, is_high);
            let ops = [neon_arr(rd, tb), neon_arr(rn, ta), Operand::Imm(shift)];
            let u0 = sut_word(&ops, 0, false, is_high)
                .unwrap_or_else(|e| panic!("SUT U=0 rejected: {}", e));
            let u1 = sut_word(&ops, 1, false, is_high)
                .unwrap_or_else(|e| panic!("SUT U=1 rejected: {}", e));
            prop_assert_eq!(u0 ^ u1, 1u32 << 29, "u_bit must toggle only U (bit 29)");
            let r0 = sut_word(&ops, 0, false, is_high)
                .unwrap_or_else(|e| panic!("SUT round=0 rejected: {}", e));
            let r1 = sut_word(&ops, 0, true, is_high)
                .unwrap_or_else(|e| panic!("SUT round=1 rejected: {}", e));
            prop_assert_eq!(r0 ^ r1, 1u32 << 11, "is_rounding must toggle only opcode bit 11");
        }

        #[test]
        fn encode_neon_qshrn_invariant_arm_fields(
            rd in reg_num(),
            rn in reg_num(),
            ta_shift in valid_ta_shift(),
            is_high in any::<bool>(),
            u_bit in 0u32..=1u32,
            is_rounding in any::<bool>(),
        ) {
            let (ta, shift) = ta_shift;
            let tb = mandated_tb(ta, is_high);
            let ops = [neon_arr(rd, tb), neon_arr(rn, ta), Operand::Imm(shift)];
            let w = sut_word(&ops, u_bit, is_rounding, is_high)
                .unwrap_or_else(|e| panic!("SUT rejected valid layout: {}", e));
            let q = if is_high { 1u32 } else { 0 };
            let immhb = src_esize(ta) - shift as u32;
            prop_assert_eq!((w >> 31) & 1, 0u32, "bit 31 must be 0");
            prop_assert_eq!((w >> 30) & 1, q, "Q bit");
            prop_assert_eq!((w >> 29) & 1, u_bit, "U bit");
            prop_assert_eq!((w >> 23) & 0b111111, 0b011110u32, "bits[28:23]=011110");
            prop_assert_eq!((w >> 19) & 0b1111, immhb >> 3, "immh");
            prop_assert_eq!((w >> 16) & 0b111, immhb & 7, "immb");
            prop_assert_eq!((w >> 10) & 0b111111, opcode_bits(is_rounding), "opcode");
            prop_assert_eq!((w >> 5) & 0b11111, rn, "Rn");
            prop_assert_eq!(w & 0b11111, rd, "Rd");
        }

        #[test]
        fn encode_neon_qshrn_neg_shift_oob(
            rd in reg_num(),
            rn in reg_num(),
            ta_shift in oob_ta_shift(),
            is_high in any::<bool>(),
            u_bit in 0u32..=1u32,
            is_rounding in any::<bool>(),
        ) {
            let (ta, shift) = ta_shift;
            let d = dest_esize(ta) as i64;
            prop_assume!(shift < 1 || shift > d);
            let tb = mandated_tb(ta, is_high);
            let mnem = mnemonic(u_bit, is_rounding, is_high);
            let asm = format!(
                "{} {}.{}, {}.{}, #{}",
                mnem, vreg(rd), tb, vreg(rn), ta, shift
            );
            prop_assert!(
                llvm_mc_word(&asm).is_err(),
                "llvm-mc unexpectedly accepted OOB shift {}",
                asm
            );
            let ops = [neon_arr(rd, tb), neon_arr(rn, ta), Operand::Imm(shift)];
            prop_assert!(
                encode_neon_qshrn(&ops, u_bit, is_rounding, is_high).is_err(),
                "shift {} for Ta={} dest_esize={} must Err (llvm-mc rejects {})",
                shift,
                ta,
                d,
                asm
            );
        }

        #[test]
        fn encode_neon_qshrn_neg_dest_tb(
            rd in reg_num(),
            rn in reg_num(),
            ta_shift in valid_ta_shift(),
            tb in prop::sample::select(vec!["8b", "16b", "4h", "8h", "2s", "4s", "1d", "2d"]),
            is_high in any::<bool>(),
            u_bit in 0u32..=1u32,
            is_rounding in any::<bool>(),
        ) {
            let (ta, shift) = ta_shift;
            prop_assume!(tb != mandated_tb(ta, is_high));
            let mnem = mnemonic(u_bit, is_rounding, is_high);
            let asm = format!(
                "{} {}.{}, {}.{}, #{}",
                mnem, vreg(rd), tb, vreg(rn), ta, shift
            );
            prop_assert!(
                llvm_mc_word(&asm).is_err(),
                "llvm-mc unexpectedly accepted mismatched Tb {}",
                asm
            );
            let ops = [neon_arr(rd, tb), neon_arr(rn, ta), Operand::Imm(shift)];
            prop_assert!(
                encode_neon_qshrn(&ops, u_bit, is_rounding, is_high).is_err(),
                "mismatched dest Tb={} for Ta={} is_high={} must Err (llvm-mc rejects {})",
                tb,
                ta,
                is_high,
                asm
            );
        }

        #[test]
        fn encode_neon_qshrn_neg_gpr_dest(
            prefix in prop::sample::select(vec!["x", "w", "d", "s", "q", "h", "b"]),
            n in prop_oneof![Just(0u32), Just(31u32), 0u32..=31],
            rn in reg_num(),
            ta_shift in valid_ta_shift(),
            is_high in any::<bool>(),
            u_bit in 0u32..=1u32,
            is_rounding in any::<bool>(),
        ) {
            let (ta, shift) = ta_shift;
            let dest = format!("{}{}", prefix, n);
            let mnem = mnemonic(u_bit, is_rounding, is_high);
            let asm = format!(
                "{} {}, {}.{}, #{}",
                mnem, dest, vreg(rn), ta, shift
            );
            prop_assert!(
                llvm_mc_word(&asm).is_err(),
                "llvm-mc unexpectedly accepted GPR/FP dest {}",
                asm
            );
            let ops = [
                Operand::Reg(dest.clone()),
                neon_arr(rn, ta),
                Operand::Imm(shift),
            ];
            prop_assert!(
                encode_neon_qshrn(&ops, u_bit, is_rounding, is_high).is_err(),
                "GPR/FP dest {} is not a NEON Vd.Tb and must Err (llvm-mc rejects {})",
                dest,
                asm
            );
        }

        #[test]
        fn encode_neon_qshrn_neg_extra_operand(
            rd in reg_num(),
            rn in reg_num(),
            extra in reg_num(),
            ta_shift in valid_ta_shift(),
            is_high in any::<bool>(),
            u_bit in 0u32..=1u32,
            is_rounding in any::<bool>(),
        ) {
            let (ta, shift) = ta_shift;
            let tb = mandated_tb(ta, is_high);
            let mnem = mnemonic(u_bit, is_rounding, is_high);
            let asm = format!(
                "{} {}.{}, {}.{}, #{}, {}.{} ",
                mnem, vreg(rd), tb, vreg(rn), ta, shift, vreg(extra), tb
            )
            .trim_end()
            .to_string();
            prop_assert!(
                llvm_mc_word(&asm).is_err(),
                "llvm-mc unexpectedly accepted 4-operand {}",
                asm
            );
            let ops = [
                neon_arr(rd, tb),
                neon_arr(rn, ta),
                Operand::Imm(shift),
                neon_arr(extra, tb),
            ];
            prop_assert!(
                encode_neon_qshrn(&ops, u_bit, is_rounding, is_high).is_err(),
                "4 operands must Err (llvm-mc rejects {})",
                asm
            );
        }

        #[test]
        fn encode_neon_qshrn_neg_arity_src_nonreg(
            n in 0usize..=2,
            rd in reg_num(),
            rn in reg_num(),
            ta in prop::sample::select(vec!["8b", "16b", "4h", "2s", "1d", "16h", "8s", "4d", "", "b", "h"]),
            slot in 0usize..=2,
            which in 0u32..=5,
            u_bit in 0u32..=1u32,
            is_rounding in any::<bool>(),
            is_high in any::<bool>(),
            bad in prop_oneof![
                Just("v32".to_string()),
                Just("v99".to_string()),
                Just("foo".to_string()),
                Just("".to_string()),
                Just("v".to_string()),
                Just("v-1".to_string()),
            ],
        ) {
            // Too few operands.
            let all = [neon_arr(rd, "8b"), neon_arr(rn, "8h")];
            let ops: Vec<Operand> = all.iter().take(n).cloned().collect();
            prop_assert!(
                encode_neon_qshrn(&ops, u_bit, is_rounding, is_high).is_err(),
                "len={} must Err (requires 3 operands)",
                n
            );

            // Unsupported source Ta.
            let ops_ta = [
                neon_arr(rd, "8b"),
                neon_arr(rn, ta),
                Operand::Imm(1),
            ];
            prop_assert!(
                encode_neon_qshrn(&ops_ta, u_bit, is_rounding, is_high).is_err(),
                "source Ta={} is not 8h/4s/2d and must Err",
                ta
            );

            // Non-register / non-imm at a slot.
            let bad_op = match which {
                0 => Operand::Imm(0),
                1 => Operand::Mem {
                    base: "x0".into(),
                    offset: 0,
                },
                2 => Operand::Symbol("foo".into()),
                3 => Operand::Shift {
                    kind: "lsl".into(),
                    amount: 0,
                },
                4 => Operand::Cond("eq".into()),
                _ => Operand::Label(".L0".into()),
            };
            let mut ops_kind = vec![
                neon_arr(rd, "8b"),
                neon_arr(rn, "8h"),
                Operand::Imm(1),
            ];
            ops_kind[slot] = bad_op;
            prop_assert!(
                encode_neon_qshrn(&ops_kind, u_bit, is_rounding, is_high).is_err(),
                "non-matching kind at slot {} which={} must Err",
                slot,
                which
            );

            // Invalid register name.
            let mut ops_bad = vec![
                neon_arr(rd, "8b"),
                neon_arr(rn, "8h"),
                Operand::Imm(1),
            ];
            if slot < 2 {
                ops_bad[slot] = Operand::RegArrangement {
                    reg: bad.clone(),
                    arrangement: if slot == 1 {
                        "8h".into()
                    } else {
                        "8b".into()
                    },
                };
                prop_assert!(
                    encode_neon_qshrn(&ops_bad, u_bit, is_rounding, is_high).is_err(),
                    "invalid NEON register {} at slot {} must Err",
                    bad,
                    slot
                );
            }
        }
    }

    /// Deterministic regression: shift must be in 1..=dest_esize (shrunk from neg_shift_oob).
    #[test]
    fn test_encode_neon_qshrn_regression_shift_oob_dest_esize() {
        let ops = [neon_arr(0, "8b"), neon_arr(0, "8h"), Operand::Imm(9)];
        assert!(
            encode_neon_qshrn(&ops, 0, false, false).is_err(),
            "sqshrn v0.8b, v0.8h, #9 must Err (shift 9 > dest esize 8; llvm-mc range [1, 8])"
        );
    }

    /// Deterministic regression: dest Tb must match Ta and the 2-suffix (shrunk from neg_dest_tb).
    #[test]
    fn test_encode_neon_qshrn_regression_mismatched_dest_tb() {
        let ops = [neon_arr(0, "4h"), neon_arr(0, "8h"), Operand::Imm(1)];
        assert!(
            encode_neon_qshrn(&ops, 0, false, false).is_err(),
            "sqshrn v0.4h, v0.8h, #1 must Err (dest Tb=4h is not 8b)"
        );
    }

    /// Deterministic regression: exactly 3 operands (shrunk from neg_extra_operand).
    #[test]
    fn test_encode_neon_qshrn_regression_extra_operand() {
        let ops = [
            neon_arr(0, "8b"),
            neon_arr(0, "8h"),
            Operand::Imm(1),
            neon_arr(0, "8b"),
        ];
        assert!(
            encode_neon_qshrn(&ops, 0, false, false).is_err(),
            "sqshrn v0.8b, v0.8h, #1, v0.8b must Err (exactly 3 operands)"
        );
    }

    /// Deterministic regression: dest must be Vd.Tb, not a GPR (shrunk from neg_gpr_dest).
    #[test]
    fn test_encode_neon_qshrn_regression_gpr_dest() {
        let ops = [
            Operand::Reg("x0".into()),
            neon_arr(0, "8h"),
            Operand::Imm(1),
        ];
        assert!(
            encode_neon_qshrn(&ops, 0, false, false).is_err(),
            "sqshrn x0, v0.8h, #1 must Err (GPR dest is not Vd.Tb)"
        );
    }

    /// Deterministic regression: i64 Imm must not be truncated via `as u32` (shrunk from neg_shift_i64_trunc).
    #[test]
    fn test_encode_neon_qshrn_regression_shift_i64_trunc() {
        let ops = [neon_arr(0, "8b"), neon_arr(0, "8h"), Operand::Imm(4294967297)];
        assert!(
            encode_neon_qshrn(&ops, 0, false, false).is_err(),
            "sqshrn v0.8b, v0.8h, #4294967297 must Err (not in [1, 8]; must not truncate to #1)"
        );
    }

    proptest! {
        #![proptest_config(cfg())]

        /// Coverage sweep: i64 immediates whose low 32 bits look like a valid shift
        /// (the SUT does `get_imm as u32`) must still Err when the i64 is not in [1, dest_esize].
        #[test]
        fn encode_neon_qshrn_neg_shift_i64_trunc(
            rd in reg_num(),
            rn in reg_num(),
            ta_shift in valid_ta_shift(),
            is_high in any::<bool>(),
            u_bit in 0u32..=1u32,
            is_rounding in any::<bool>(),
            k in prop_oneof![Just(1i64), Just(-1i64), Just(2i64), -4i64..=4i64],
        ) {
            let (ta, shift) = ta_shift;
            prop_assume!(k != 0);
            let wide = shift.wrapping_add(k.wrapping_mul(1i64 << 32));
            prop_assume!(wide != shift);
            let tb = mandated_tb(ta, is_high);
            let ops = [neon_arr(rd, tb), neon_arr(rn, ta), Operand::Imm(wide)];
            prop_assert!(
                encode_neon_qshrn(&ops, u_bit, is_rounding, is_high).is_err(),
                "Imm({}) truncates to shift {} but is not in [1, dest_esize]; must Err",
                wide,
                shift
            );
        }

        /// Coverage sweep: source must be Vn.Ta, not a bare GPR/FP register.
        #[test]
        fn encode_neon_qshrn_neg_reg_source(
            rd in reg_num(),
            prefix in prop::sample::select(vec!["x", "w", "d", "s", "q", "h", "b", "v"]),
            n in prop_oneof![Just(0u32), Just(31u32), 0u32..=31],
            ta_shift in valid_ta_shift(),
            is_high in any::<bool>(),
            u_bit in 0u32..=1u32,
            is_rounding in any::<bool>(),
        ) {
            let (ta, shift) = ta_shift;
            let tb = mandated_tb(ta, is_high);
            let src = format!("{}{}", prefix, n);
            let ops = [
                neon_arr(rd, tb),
                Operand::Reg(src.clone()),
                Operand::Imm(shift),
            ];
            prop_assert!(
                encode_neon_qshrn(&ops, u_bit, is_rounding, is_high).is_err(),
                "source {} is not Vn.Ta and must Err",
                src
            );
        }
    }
}

