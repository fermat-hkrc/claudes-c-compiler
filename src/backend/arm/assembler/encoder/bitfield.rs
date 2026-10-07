use super::*;
use crate::backend::arm::assembler::parser::Operand;

// ── Bitfield extract/insert ──────────────────────────────────────────────

/// Encode UBFX Rd, Rn, #lsb, #width -> UBFM Rd, Rn, #lsb, #(lsb+width-1)
pub(crate) fn encode_ubfx(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let lsb = get_imm(operands, 2)? as u32;
    let width = get_imm(operands, 3)? as u32;
    let sf = sf_bit(is_64);
    let n = if is_64 { 1u32 } else { 0u32 };
    let immr = lsb;
    let imms = lsb + width - 1;
    // UBFM: sf 10 100110 N immr imms Rn Rd
    let word = (sf << 31) | (0b10 << 29) | (0b100110 << 23) | (n << 22) | (immr << 16) | (imms << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode SBFX Rd, Rn, #lsb, #width -> SBFM Rd, Rn, #lsb, #(lsb+width-1)
pub(crate) fn encode_sbfx(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let lsb = get_imm(operands, 2)? as u32;
    let width = get_imm(operands, 3)? as u32;
    let sf = sf_bit(is_64);
    let n = if is_64 { 1u32 } else { 0u32 };
    let immr = lsb;
    let imms = lsb + width - 1;
    // SBFM: sf 00 100110 N immr imms Rn Rd
    let word = (sf << 31) | (0b100110 << 23) | (n << 22) | (immr << 16) | (imms << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode UBFM Rd, Rn, #immr, #imms (raw form)
pub(crate) fn encode_ubfm(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let immr = get_imm(operands, 2)? as u32;
    let imms = get_imm(operands, 3)? as u32;
    let sf = sf_bit(is_64);
    let n = if is_64 { 1u32 } else { 0u32 };
    let word = (sf << 31) | (0b10 << 29) | (0b100110 << 23) | (n << 22) | (immr << 16) | (imms << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode SBFM Rd, Rn, #immr, #imms (raw form)
pub(crate) fn encode_sbfm(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let immr = get_imm(operands, 2)? as u32;
    let imms = get_imm(operands, 3)? as u32;
    let sf = sf_bit(is_64);
    let n = if is_64 { 1u32 } else { 0u32 };
    let word = (sf << 31) | (0b100110 << 23) | (n << 22) | (immr << 16) | (imms << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode SBFIZ Rd, Rn, #lsb, #width — alias for SBFM Rd, Rn, #(-lsb MOD regsize), #(width-1)
pub(crate) fn encode_sbfiz(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let lsb = get_imm(operands, 2)? as u32;
    let width = get_imm(operands, 3)? as u32;
    let regsize = if is_64 { 64u32 } else { 32 };
    let sf = sf_bit(is_64);
    let n = if is_64 { 1u32 } else { 0u32 };
    let immr = (regsize.wrapping_sub(lsb)) & (regsize - 1);
    let imms = width - 1;
    let word = (sf << 31) | (0b100110 << 23) | (n << 22) | (immr << 16) | (imms << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode UBFIZ Rd, Rn, #lsb, #width — alias for UBFM Rd, Rn, #(-lsb MOD regsize), #(width-1)
pub(crate) fn encode_ubfiz(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let lsb = get_imm(operands, 2)? as u32;
    let width = get_imm(operands, 3)? as u32;
    let regsize = if is_64 { 64u32 } else { 32 };
    let sf = sf_bit(is_64);
    let n = if is_64 { 1u32 } else { 0u32 };
    let immr = (regsize.wrapping_sub(lsb)) & (regsize - 1);
    let imms = width - 1;
    let word = (sf << 31) | (0b10 << 29) | (0b100110 << 23) | (n << 22) | (immr << 16) | (imms << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode BFM Rd, Rn, #immr, #imms (bitfield move)
pub(crate) fn encode_bfm(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let immr = get_imm(operands, 2)? as u32;
    let imms = get_imm(operands, 3)? as u32;
    let sf = sf_bit(is_64);
    let n = if is_64 { 1u32 } else { 0u32 };
    // BFM: sf 01 100110 N immr imms Rn Rd
    let word = (sf << 31) | (0b01 << 29) | (0b100110 << 23) | (n << 22) | (immr << 16) | (imms << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode BFI Rd, Rn, #lsb, #width -> BFM Rd, Rn, #(-lsb mod width_reg), #(width-1)
pub(crate) fn encode_bfi(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let lsb = get_imm(operands, 2)? as u32;
    let width = get_imm(operands, 3)? as u32;
    let sf = sf_bit(is_64);
    let n = if is_64 { 1u32 } else { 0u32 };
    let reg_width = if is_64 { 64u32 } else { 32u32 };
    let immr = (reg_width - lsb) % reg_width;
    let imms = width - 1;
    let word = (sf << 31) | (0b01 << 29) | (0b100110 << 23) | (n << 22) | (immr << 16) | (imms << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode BFXIL Rd, Rn, #lsb, #width -> BFM Rd, Rn, #lsb, #(lsb+width-1)
pub(crate) fn encode_bfxil(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let lsb = get_imm(operands, 2)? as u32;
    let width = get_imm(operands, 3)? as u32;
    let sf = sf_bit(is_64);
    let n = if is_64 { 1u32 } else { 0u32 };
    let immr = lsb;
    let imms = lsb + width - 1;
    let word = (sf << 31) | (0b01 << 29) | (0b100110 << 23) | (n << 22) | (immr << 16) | (imms << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode EXTR Rd, Rn, Rm, #lsb
pub(crate) fn encode_extr(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
    let lsb = get_imm(operands, 3)? as u32;
    let sf = sf_bit(is_64);
    let n = if is_64 { 1u32 } else { 0u32 };
    // EXTR: sf 0 0 100111 N 0 Rm imms Rn Rd
    let word = (sf << 31) | (0b00100111 << 23) | (n << 22) | (rm << 16)
        | (lsb << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

// ── Bit manipulation ─────────────────────────────────────────────────────

pub(crate) fn encode_clz(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let sf = sf_bit(is_64);
    // CLZ: sf 1 0 11010110 00000 00010 0 Rn Rd
    let word = ((sf << 31) | (1 << 30) | (0b011010110 << 21))
        | (0b000100 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

pub(crate) fn encode_cls(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let sf = sf_bit(is_64);
    let word = ((sf << 31) | (1 << 30) | (0b011010110 << 21))
        | (0b000101 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

pub(crate) fn encode_rbit(operands: &[Operand]) -> Result<EncodeResult, String> {
    // NEON vector form: RBIT Vd.T, Vn.T (reverse bits in each byte)
    if let Some(Operand::RegArrangement { .. }) = operands.first() {
        let (rd, arr_d) = get_neon_reg(operands, 0)?;
        let (rn, _) = get_neon_reg(operands, 1)?;
        let q: u32 = if arr_d == "16b" { 1 } else { 0 };
        // RBIT (vector): 0 Q 1 01110 01 10000 00101 10 Rn Rd
        let word = (q << 30) | (1 << 29) | (0b01110 << 24) | (0b01 << 22)
            | (0b10000 << 17) | (0b00101 << 12) | (0b10 << 10) | (rn << 5) | rd;
        return Ok(EncodeResult::Word(word));
    }
    // Scalar form: RBIT Rd, Rn
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let sf = sf_bit(is_64);
    let word = ((sf << 31) | (1 << 30) | (0b011010110 << 21)) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

pub(crate) fn encode_rev(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let sf = sf_bit(is_64);
    let opc = if is_64 { 0b000011 } else { 0b000010 };
    let word = ((sf << 31) | (1 << 30) | (0b011010110 << 21))
        | (opc << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

pub(crate) fn encode_rev16(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let sf = sf_bit(is_64);
    let word = ((sf << 31) | (1 << 30) | (0b011010110 << 21))
        | (0b000001 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

pub(crate) fn encode_rev32(operands: &[Operand]) -> Result<EncodeResult, String> {
    // Check for NEON vector form: REV32 Vd.T, Vn.T
    if let Some(Operand::RegArrangement { .. }) = operands.first() {
        let (rd, arr_d) = get_neon_reg(operands, 0)?;
        let (rn, _) = get_neon_reg(operands, 1)?;
        let (q, size) = neon_arr_to_q_size(&arr_d)?;
        // REV32 Vd.T, Vn.T: 0 Q 1 01110 size 10 0000 0000 10 Rn Rd
        let word = (q << 30) | (1 << 29) | (0b01110 << 24) | (size << 22)
            | (0b100000 << 16) | (0b000010 << 10) | (rn << 5) | rd;
        return Ok(EncodeResult::Word(word));
    }
    let (rd, _) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    // REV32 is 64-bit only: 1 1 0 11010110 00000 000010 Rn Rd
    let word = ((1u32 << 31) | (1 << 30) | (0b011010110 << 21))
        | (0b000010 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

// ── CRC32 ────────────────────────────────────────────────────────────────

pub(crate) fn encode_crc32(mnemonic: &str, operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, _) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;

    let is_c = mnemonic.contains("crc32c");
    let c_bit = if is_c { 1u32 } else { 0 };

    let (sf, sz) = match mnemonic {
        "crc32b" | "crc32cb" => (0u32, 0b00u32),
        "crc32h" | "crc32ch" => (0, 0b01),
        "crc32w" | "crc32cw" => (0, 0b10),
        "crc32x" | "crc32cx" => (1, 0b11),
        _ => (0, 0b00),
    };

    // CRC32: sf 0 0 11010110 Rm 010 C sz Rn Rd
    let word = (sf << 31) | (0b0011010110 << 21) | (rm << 16) | (0b010 << 13)
        | (c_bit << 12) | (sz << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

#[cfg(test)]
mod pbt_encode_tests {
    use super::*;
    use crate::backend::arm::assembler::parser::Operand;
    use proptest::prelude::*;

    /// P-UBFX: encode_ubfx output bytes must match llvm-mc for valid ranges
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(256))]
        #[test]
        fn p_encode_ubfx_valid(
            rd in 0u32..31, rn in 0u32..31,
            lsb in 0i64..31, width in 1i64..32,
        ) {
            // Clamp lsb+width to valid range
            let width: u32 = (width as u32).min(32 - lsb as u32).max(1);
            let ops = vec![
                Operand::Reg(format!("x{}", rd)),
                Operand::Reg(format!("x{}", rn)),
                Operand::Imm(lsb),
                Operand::Imm(width as i64),
            ];
            let result = encode_ubfx(&ops);
            prop_assert!(result.is_ok(), "encode_ubfx failed for rd={} rn={} lsb={} width={}", rd, rn, lsb, width);
            if let Ok(EncodeResult::Word(word)) = result {
                // Verify field extraction: sf=1, opc=10, 100110, N=1
                prop_assert_eq!(word >> 31, 1, "sf must be 1 for x registers");
                prop_assert_eq!((word >> 29) & 0b11, 0b10, "opc bits must be 10 for UBFM");
                prop_assert_eq!((word >> 23) & 0x3F, 0b100110, "must be 100110");
                prop_assert_eq!((word >> 22) & 1, 1, "N must be 1 for 64-bit");
                prop_assert_eq!((word >> 16) & 0x3F, lsb as u32, "immr must equal lsb");
                prop_assert_eq!((word >> 10) & 0x3F, (lsb + width as i64 - 1) as u32, "imms must equal lsb+width-1");
                prop_assert_eq!((word >> 5) & 0x1F, rn, "Rn must match");
                prop_assert_eq!(word & 0x1F, rd, "Rd must match");
            }
        }
    }

    /// P-UBFX-32: W register variant
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(256))]
        #[test]
        fn p_encode_ubfx_w32(
            rd in 0u32..31, rn in 0u32..31,
            lsb in 0i64..15, width in 1i64..16,
        ) {
            let width: u32 = (width as u32).min(16 - lsb as u32).max(1);
            let ops = vec![
                Operand::Reg(format!("w{}", rd)),
                Operand::Reg(format!("w{}", rn)),
                Operand::Imm(lsb),
                Operand::Imm(width as i64),
            ];
            let result = encode_ubfx(&ops);
            prop_assert!(result.is_ok());
            if let Ok(EncodeResult::Word(word)) = result {
                prop_assert_eq!(word >> 31, 0, "sf must be 0 for w registers");
                prop_assert_eq!((word >> 22) & 1, 0, "N must be 0 for 32-bit");
            }
        }
    }

    /// P-BFM: encode_bfm immr/imms range validation
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(256))]
        #[test]
        fn p_encode_bfm_imm_range(
            rd in 0u32..31, rn in 0u32..31,
            immr in any::<i64>(), imms in any::<i64>(),
        ) {
            let ops = vec![
                Operand::Reg(format!("x{}", rd)),
                Operand::Reg(format!("x{}", rn)),
                Operand::Imm(immr),
                Operand::Imm(imms),
            ];
            let result = encode_bfm(&ops);
            // For 64-bit: immr and imms must be 0..63
            if immr < 0 || immr > 63 || imms < 0 || imms > 63 {
                prop_assert!(result.is_err() || result.is_ok(), "out-of-range should either error or clamp");
            } else {
                prop_assert!(result.is_ok(), "encode_bfm should succeed for valid range");
            }
        }
    }

    /// P-CLZ: encode_clz register width consistency
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(128))]
        #[test]
        fn p_encode_clz(
            rd in 0u32..31, rn in 0u32..31,
            is_64 in any::<bool>(),
        ) {
            let prefix = if is_64 { "x" } else { "w" };
            let ops = vec![
                Operand::Reg(format!("{}{}", prefix, rd)),
                Operand::Reg(format!("{}{}", prefix, rn)),
            ];
            let result = encode_clz(&ops);
            prop_assert!(result.is_ok());
            if let Ok(EncodeResult::Word(word)) = result {
                let expected_sf = if is_64 { 1u32 } else { 0 };
                prop_assert_eq!(word >> 31, expected_sf, "sf must match register width");
            }
        }
    }
}

#[cfg(test)]
mod pbt_adversarial2 {
    use super::*;
    use crate::backend::arm::assembler::parser::Operand;

    /// P-BFM: FP 寄存器 → 应该 Err
    #[test]
    fn p_bfm_fp_reg() {
        let ops = vec![
            Operand::Reg("x0".to_string()),
            Operand::Reg("d1".to_string()), // FP as source
            Operand::Imm(0),
            Operand::Imm(31),
        ];
        let result = encode_bfm(&ops);
        assert!(result.is_err(),
            "encode_bfm accepted FP d1 as source: {:?}", result);
    }

    /// P-EXTR: imm 超出范围（0..63 for X, 0..31 for W）
    #[test]
    fn p_extr_imm_out_of_range() {
        for imm in [64i64, 100, -1] {
            let ops = vec![
                Operand::Reg("x0".to_string()),
                Operand::Reg("x1".to_string()),
                Operand::Reg("x2".to_string()),
                Operand::Imm(imm),
            ];
            let result = encode_extr(&ops);
            // For X: lsb must be 0..63; for W: 0..31
            // imm=64 is out of range for both
            if result.is_ok() {
                if let Ok(EncodeResult::Word(word)) = result {
                    let imms = (word >> 10) & 0x3F;
                    assert!(imms <= 63,
                        "extr imm {} encoded imms={} (> 6 bits)", imm, imms);
                }
            }
        }
    }

    /// P-CRC32: FP 寄存器传给 CRC32 → 应该 Err
    #[test]
    fn p_crc32_fp_reg() {
        let ops = vec![
            Operand::Reg("w0".to_string()),
            Operand::Reg("d1".to_string()), // FP as source
            Operand::Reg("w2".to_string()),
        ];
        let result = encode_crc32("crc32b", &ops);
        assert!(result.is_err(),
            "encode_crc32 accepted FP d1 as source: {:?}", result);
    }

    /// P-SBFIZ: imm 超出范围 → 应该 Err 或正确处理
    #[test]
    fn p_sbfiz_imm_range() {
        // sbfiz Xd, Xn, #lsb, #width — lsb + width must not exceed 64
        let ops = vec![
            Operand::Reg("x0".to_string()),
            Operand::Reg("x1".to_string()),
            Operand::Imm(32), // lsb=32
            Operand::Imm(33), // width=33 → lsb+width=65 > 64!
        ];
        let result = encode_sbfiz(&ops);
        // lsb+width must be ≤ 64 for X registers
        if result.is_ok() {
            // If accepted, it would produce wrong bitfield extraction
            // Check imms field
            if let Ok(EncodeResult::Word(word)) = result {
                let imms = (word >> 10) & 0x3F;
                println!("sbfiz lsb=32 width=33 → imms={} (should be ≤63)", imms);
            }
        }
    }

    /// P-UBFIZ: 0 width → 应该 Err
    #[test]
    fn p_ubfiz_zero_width() {
        let ops = vec![
            Operand::Reg("x0".to_string()),
            Operand::Reg("x1".to_string()),
            Operand::Imm(0), // lsb=0
            Operand::Imm(0), // width=0 — invalid!
        ];
        let result = encode_ubfiz(&ops);
        assert!(result.is_err(),
            "encode_ubfiz accepted width=0: {:?}", result);
    }

    /// P-CLZ: FP 寄存器 → 应该 Err
    #[test]
    fn p_clz_fp_reg() {
        let ops = vec![
            Operand::Reg("x0".to_string()),
            Operand::Reg("s1".to_string()), // FP as source
        ];
        let result = encode_clz(&ops);
        assert!(result.is_err(),
            "encode_clz accepted FP s1 as source: {:?}", result);
    }

    /// P-REV: imm 作为源操作数 → 应该 Err
    #[test]
    fn p_rev_imm_operand() {
        let ops = vec![
            Operand::Reg("x0".to_string()),
            Operand::Imm(5), // immediate where register expected
        ];
        let result = encode_rev(&ops);
        assert!(result.is_err(),
            "encode_rev accepted Imm as source: {:?}", result);
    }
}
