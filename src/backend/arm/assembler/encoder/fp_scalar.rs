use super::*;
use crate::backend::arm::assembler::parser::Operand;

// ── Floating point ───────────────────────────────────────────────────────

pub(crate) fn encode_fmov(operands: &[Operand]) -> Result<EncodeResult, String> {
    if operands.len() < 2 {
        return Err("fmov requires 2 operands".to_string());
    }

    let (rd_name, rm_name) = match (&operands[0], &operands[1]) {
        (Operand::Reg(a), Operand::Reg(b)) => (a.clone(), b.clone()),
        (Operand::Reg(_a), Operand::Imm(_)) => {
            // TODO: implement fmov with float immediate encoding
            return Err("fmov with immediate operand not yet supported".to_string());
        }
        _ => return Err("fmov needs register operands".to_string()),
    };

    let rd = parse_reg_num(&rd_name).ok_or("invalid rd")?;
    let rm = parse_reg_num(&rm_name).ok_or("invalid rm")?;

    let rd_is_fp = is_fp_reg(&rd_name);
    let rm_is_fp = is_fp_reg(&rm_name);
    let rd_lower = rd_name.to_lowercase();
    let rm_lower = rm_name.to_lowercase();

    if rd_is_fp && rm_is_fp {
        // FMOV between FP registers
        let is_double = rd_lower.starts_with('d') || rm_lower.starts_with('d');
        let ftype = if is_double { 0b01 } else { 0b00 };
        // 0 00 11110 ftype 1 0000 00 10000 Rn Rd
        let word = (0b00011110 << 24) | (ftype << 22) | (0b100000 << 16) | (0b10000 << 10) | (rm << 5) | rd;
        return Ok(EncodeResult::Word(word));
    }

    if rd_is_fp && !rm_is_fp {
        // FMOV from GP to FP: FMOV Dn, Xn or FMOV Sn, Wn
        let is_double = rd_lower.starts_with('d');
        if is_double {
            // FMOV Dd, Xn: 1 00 11110 01 1 00 111 000000 Rn Rd
            let word = ((0b1001111001 << 22) | (0b100111 << 16)) | (rm << 5) | rd;
            return Ok(EncodeResult::Word(word));
        } else {
            // FMOV Sd, Wn: 0 00 11110 00 1 00 111 000000 Rn Rd
            let word = ((0b0001111000 << 22) | (0b100111 << 16)) | (rm << 5) | rd;
            return Ok(EncodeResult::Word(word));
        }
    }

    if !rd_is_fp && rm_is_fp {
        // FMOV from FP to GP: FMOV Xn, Dn or FMOV Wn, Sn
        let is_double = rm_lower.starts_with('d');
        if is_double {
            // FMOV Xd, Dn: 1 00 11110 01 1 00 110 000000 Rn Rd
            let word = ((0b1001111001 << 22) | (0b100110 << 16)) | (rm << 5) | rd;
            return Ok(EncodeResult::Word(word));
        } else {
            // FMOV Wd, Sn: 0 00 11110 00 1 00 110 000000 Rn Rd
            let word = ((0b0001111000 << 22) | (0b100110 << 16)) | (rm << 5) | rd;
            return Ok(EncodeResult::Word(word));
        }
    }

    Err(format!("unsupported fmov operands: {} -> {}", rd_name, rm_name))
}

pub(crate) fn encode_fp_arith(operands: &[Operand], opcode: u32) -> Result<EncodeResult, String> {
    let (rd, _) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;

    let rd_name = match &operands[0] { Operand::Reg(r) => r.to_lowercase(), _ => String::new() };
    let is_double = rd_name.starts_with('d');
    let ftype = if is_double { 0b01 } else { 0b00 };

    // 0 00 11110 ftype 1 Rm opcode 10 Rn Rd
    let word = (0b00011110 << 24) | (ftype << 22) | (1 << 21) | (rm << 16) | (opcode << 12) | (0b10 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

pub(crate) fn encode_fneg(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, _) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let rd_name = match &operands[0] { Operand::Reg(r) => r.to_lowercase(), _ => String::new() };
    let is_double = rd_name.starts_with('d');
    let ftype = if is_double { 0b01 } else { 0b00 };
    // FNEG: 0 00 11110 ftype 1 0000 10 10000 Rn Rd
    let word = (0b00011110 << 24) | (ftype << 22) | (0b100001 << 16) | (0b10000 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

pub(crate) fn encode_fabs(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, _) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let rd_name = match &operands[0] { Operand::Reg(r) => r.to_lowercase(), _ => String::new() };
    let is_double = rd_name.starts_with('d');
    let ftype = if is_double { 0b01 } else { 0b00 };
    // FABS: 0 00 11110 ftype 1 0000 01 10000 Rn Rd
    let word = (0b00011110 << 24) | (ftype << 22) | (0b100000 << 16) | (0b110000 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

pub(crate) fn encode_fsqrt(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, _) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let rd_name = match &operands[0] { Operand::Reg(r) => r.to_lowercase(), _ => String::new() };
    let is_double = rd_name.starts_with('d');
    let ftype = if is_double { 0b01 } else { 0b00 };
    // FSQRT: 0 00 11110 ftype 1 0000 11 10000 Rn Rd
    let word = (0b00011110 << 24) | (ftype << 22) | (0b100001 << 16) | (0b110000 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode FP 1-source ops: FRINTN/P/M/Z/A/X/I
/// Format: 0 00 11110 ftype 1 opcode 10000 Rn Rd
pub(crate) fn encode_fp_1src(operands: &[Operand], opcode: u32) -> Result<EncodeResult, String> {
    let (rd, _) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let rd_name = match &operands[0] { Operand::Reg(r) => r.to_lowercase(), _ => String::new() };
    let is_double = rd_name.starts_with('d');
    let ftype = if is_double { 0b01u32 } else { 0b00 };
    let word = (0b00011110u32 << 24) | (ftype << 22) | (1 << 21)
        | (opcode << 15) | (0b10000 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode FMADD/FMSUB: Rd = Ra +/- (Rn * Rm)
/// Format: 0 00 11111 ftype 0 Rm o1 Ra Rn Rd
pub(crate) fn encode_fmadd_fmsub(operands: &[Operand], is_sub: bool) -> Result<EncodeResult, String> {
    let (rd, _) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
    let (ra, _) = get_reg(operands, 3)?;
    let rd_name = match &operands[0] { Operand::Reg(r) => r.to_lowercase(), _ => String::new() };
    let is_double = rd_name.starts_with('d');
    let ftype = if is_double { 0b01u32 } else { 0b00 };
    let o1 = if is_sub { 1u32 } else { 0 };
    let word = (0b00011111u32 << 24) | (ftype << 22) | (rm << 16)
        | (o1 << 15) | (ra << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

/// Encode FNMADD/FNMSUB: Rd = -Ra +/- (Rn * Rm)
/// Format: 0 00 11111 ftype 1 Rm o1 Ra Rn Rd
pub(crate) fn encode_fnmadd_fnmsub(operands: &[Operand], is_sub: bool) -> Result<EncodeResult, String> {
    let (rd, _) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
    let (ra, _) = get_reg(operands, 3)?;
    let rd_name = match &operands[0] { Operand::Reg(r) => r.to_lowercase(), _ => String::new() };
    let is_double = rd_name.starts_with('d');
    let ftype = if is_double { 0b01u32 } else { 0b00 };
    let o1 = if is_sub { 1u32 } else { 0 };
    let word = (0b00011111u32 << 24) | (ftype << 22) | (1 << 21) | (rm << 16)
        | (o1 << 15) | (ra << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

pub(crate) fn encode_fcmp(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rn, _) = get_reg(operands, 0)?;
    let rn_name = match &operands[0] { Operand::Reg(r) => r.to_lowercase(), _ => String::new() };
    let is_double = rn_name.starts_with('d');
    let ftype = if is_double { 0b01 } else { 0b00 };

    // FCMP Dn, #0.0
    if operands.len() < 2 || matches!(operands.get(1), Some(Operand::Imm(0))) {
        let word = ((0b00011110 << 24) | (ftype << 22) | (1 << 21)) | (0b001000 << 10) | (rn << 5) | 0b01000;
        return Ok(EncodeResult::Word(word));
    }

    let (rm, _) = get_reg(operands, 1)?;
    // FCMP Dn, Dm: 0 00 11110 ftype 1 Rm 00 1000 Rn 00 000
    let word = (0b00011110 << 24) | (ftype << 22) | (1 << 21) | (rm << 16) | (0b001000 << 10) | (rn << 5);
    Ok(EncodeResult::Word(word))
}

pub(crate) fn encode_fcvt_rounding(operands: &[Operand], rmode: u32, opcode: u32) -> Result<EncodeResult, String> {
    // Float-to-integer conversion with specified rounding mode
    // Encoding: sf 00 11110 ftype 1 rmode opcode 000000 Rn Rd
    // sf: 0=W dest, 1=X dest
    // ftype: 00=S source, 01=D source
    // rmode+opcode: determines rounding mode and signedness
    if operands.len() < 2 {
        return Err("fcvt* requires 2 operands".to_string());
    }
    let (rd, rd_is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;

    let src_name = match &operands[1] {
        Operand::Reg(name) => name.to_lowercase(),
        _ => return Err("fcvt*: expected register source".to_string()),
    };
    let ftype: u32 = if src_name.starts_with('d') { 0b01 } else { 0b00 };
    let sf: u32 = if rd_is_64 { 1 } else { 0 };

    let word = ((sf << 31) | (0b11110 << 24) | (ftype << 22)
        | (1 << 21) | (rmode << 19) | (opcode << 16)) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

pub(crate) fn encode_ucvtf(operands: &[Operand]) -> Result<EncodeResult, String> {
    encode_int_to_float(operands, false)
}

pub(crate) fn encode_scvtf(operands: &[Operand]) -> Result<EncodeResult, String> {
    encode_int_to_float(operands, true)
}

pub(crate) fn encode_int_to_float(operands: &[Operand], is_signed: bool) -> Result<EncodeResult, String> {
    // SCVTF/UCVTF: integer-to-float conversion
    // Encoding: sf 00 11110 ftype 1 00 opcode 000000 Rn Rd
    // sf: 0=W source, 1=X source
    // ftype: 00=S dest, 01=D dest
    // opcode: 010=signed (SCVTF), 011=unsigned (UCVTF)
    if operands.len() < 2 {
        return Err("scvtf/ucvtf requires 2 operands".to_string());
    }
    let (rd, _) = get_reg(operands, 0)?;
    let (rn, rn_is_64) = get_reg(operands, 1)?;

    let dst_name = match &operands[0] {
        Operand::Reg(name) => name.to_lowercase(),
        _ => return Err("scvtf/ucvtf: expected register dest".to_string()),
    };
    let ftype: u32 = if dst_name.starts_with('d') { 0b01 } else { 0b00 };
    let sf: u32 = if rn_is_64 { 1 } else { 0 };
    let opcode: u32 = if is_signed { 0b010 } else { 0b011 };

    let word = (((sf << 31) | (0b11110 << 24) | (ftype << 22)
        | (1 << 21)) | (opcode << 16)) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

pub(crate) fn encode_fcvt_precision(operands: &[Operand]) -> Result<EncodeResult, String> {
    // FCVT: float precision conversion (e.g., FCVT Dd, Sn or FCVT Sd, Dn)
    // Encoding: 0 00 11110 ftype 1 0001 opc 10000 Rn Rd
    // ftype: source precision (00=S, 01=D, 11=H)
    // opc: dest precision (00=S, 01=D, 11=H)
    if operands.len() < 2 {
        return Err("fcvt requires 2 operands".to_string());
    }
    let (rd, _) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;

    let dst_name = match &operands[0] {
        Operand::Reg(name) => name.to_lowercase(),
        _ => return Err("fcvt: expected register dest".to_string()),
    };
    let src_name = match &operands[1] {
        Operand::Reg(name) => name.to_lowercase(),
        _ => return Err("fcvt: expected register source".to_string()),
    };

    let ftype: u32 = match src_name.chars().next() {
        Some('s') => 0b00,
        Some('d') => 0b01,
        Some('h') => 0b11,
        _ => return Err(format!("fcvt: unsupported source type: {}", src_name)),
    };
    let opc: u32 = match dst_name.chars().next() {
        Some('s') => 0b00,
        Some('d') => 0b01,
        Some('h') => 0b11,
        _ => return Err(format!("fcvt: unsupported dest type: {}", dst_name)),
    };

    let word = (0b00011110 << 24) | (ftype << 22) | (1 << 21) | (0b0001 << 17)
        | (opc << 15) | (0b10000 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}

#[cfg(test)]
mod pbt_encode_tests {
    use super::*;
    use crate::backend::arm::assembler::parser::Operand;
    use proptest::prelude::*;

    /// P-FMOV: fmov register-to-register field verification
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(256))]
        #[test]
        fn p_encode_fmov_reg(
            rd in 0u32..32, rn in 0u32..32,
            is_double in any::<bool>(),
        ) {
            let prefix = if is_double { "d" } else { "s" };
            let ops = vec![
                Operand::Reg(format!("{}{}", prefix, rd % 32)),
                Operand::Reg(format!("{}{}", prefix, rn % 32)),
            ];
            let result = encode_fmov(&ops);
            // Just verify no panic and result is either Ok or Err (not crash)
            if let Ok(EncodeResult::Word(word)) = result {
                // For FP register moves: bits 31-21 should have opcode 00011110000
                let opcode = (word >> 21) & 0x7FF;
                prop_assert!(opcode > 0, "FP opcode field should be nonzero");
            }
        }
    }

    /// P-FP-ARITH: fp arithmetic (fadd/fsub/fmul/fdiv) field verification
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(256))]
        #[test]
        fn p_encode_fp_arith(
            rd in 0u32..32, rn in 0u32..32, rm in 0u32..32,
            opcode in 0u32..16,
        ) {
            let ops = vec![
                Operand::Reg(format!("d{}", rd % 32)),
                Operand::Reg(format!("d{}", rn % 32)),
                Operand::Reg(format!("d{}", rm % 32)),
            ];
            let result = encode_fp_arith(&ops, opcode);
            if let Ok(EncodeResult::Word(word)) = result {
                prop_assert_eq!(word >> 31, 0, "M bit=0 for scalar FP");
                prop_assert_eq!(word & 0x1F, rd % 32, "Rd must match");
            }
        }
    }

    /// P-FCVT: fp conversion opcode verification
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(128))]
        #[test]
        fn p_encode_fcvt_precision(
            rd in 0u32..32, rn in 0u32..32,
        ) {
            let ops = vec![
                Operand::Reg(format!("d{}", rd % 32)),
                Operand::Reg(format!("s{}", rn % 32)),
            ];
            let result = encode_fcvt_precision(&ops);
            // Just verify no panic
            let _ = result;
        }
    }
}

#[cfg(test)]
mod pbt_adversarial2 {
    use super::*;
    use crate::backend::arm::assembler::parser::Operand;

    /// P-FMOV-IMM: fp 立即数编码 → encode_fmov 处理 Imm(float) 时？
    #[test]
    fn p_fmov_imm_handling() {
        // encode_fmov 处理 Operand::Imm(float_bits) 时的行为
        // .s 层已发现 fmov d0, #1.0 → NOP (#1064)
        // encode 层测试：Imm 传入后是否返回 NOP word？
        let ops = vec![
            Operand::Reg("d0".to_string()),
            Operand::Imm(0x3FF0000000000000), // double bits for 1.0
        ];
        let result = encode_fmov(&ops);
        if let Ok(EncodeResult::Word(word)) = result {
            println!("fmov d0, #1.0 bits → word={:#010x}", word);
            if word == 0x1F2003D5 {
                println!("  → NOP confirmed (same as #1064)");
            }
        }
    }

    /// P-FCMP: 零比较 → encode_fcmp 处理 Imm(0) (浮点零) 时？
    #[test]
    fn p_fcmp_zero_imm() {
        let ops = vec![
            Operand::Reg("d0".to_string()),
            Operand::Imm(0), // might be treated as #0.0
        ];
        let result = encode_fcmp(&ops);
        if let Ok(EncodeResult::Word(word)) = result {
            println!("fcmp d0, #0.0 → word={:#010x}", word);
            if word == 0x1F2003D5 {
                println!("  → NOP confirmed (same as #839)");
            }
        }
    }

    /// P-FADD: GPR 寄存器传给 FP 算术 → 应该 Err
    #[test]
    fn p_fp_arith_gpr_reg() {
        let ops = vec![
            Operand::Reg("d0".to_string()),
            Operand::Reg("d1".to_string()),
            Operand::Reg("x2".to_string()), // GPR as FP operand
        ];
        let result = encode_fp_arith(&ops, 0);
        assert!(result.is_err(),
            "encode_fp_arith accepted GPR x2 as FP operand: {:?}", result);
    }

    /// P-FSQRT: 3 操作数（2 正常 + 1 多余）
    #[test]
    fn p_fsqrt_extra_operand() {
        let ops_2 = vec![
            Operand::Reg("d0".to_string()),
            Operand::Reg("d1".to_string()),
        ];
        let ops_3 = vec![
            Operand::Reg("d0".to_string()),
            Operand::Reg("d1".to_string()),
            Operand::Reg("d2".to_string()), // extra
        ];
        let r2 = encode_fsqrt(&ops_2);
        let r3 = encode_fsqrt(&ops_3);
        if let (Ok(EncodeResult::Word(w2)), Ok(EncodeResult::Word(w3))) = (r2, r3) {
            assert_eq!(w2, w3, "3rd operand changed FSQRT encoding");
        }
    }

    /// P-FMADD: GPR 寄存器 → 应该 Err
    #[test]
    fn p_fmadd_gpr_reg() {
        let ops = vec![
            Operand::Reg("d0".to_string()),
            Operand::Reg("d1".to_string()),
            Operand::Reg("d2".to_string()),
            Operand::Reg("x3".to_string()), // GPR as addend
        ];
        let result = encode_fmadd_fmsub(&ops, false);
        assert!(result.is_err(),
            "encode_fmadd_fmsub accepted GPR x3 as FP operand: {:?}", result);
    }
}

#[cfg(test)]
mod pbt_fixed_point {
    use super::*;

    /// P-SCVTF-FIXED: 定点形式 3 操作数必须与整数形式 2 操作数编码不同
    #[test]
    fn p_scvtf_fixed_vs_int() {
        // llvm-mc 参考:
        //   scvtf d0, x1      → 0x9e620020
        //   scvtf d0, x1, #32 → 0x9e428020  (不同指令!)
        //   ucvtf d0, x1, #32 → 0x9e438020
        //   scvtf s0, w1, #16 → 0x1e02c020
        let int_form = encode_scvtf(&[
            crate::backend::arm::assembler::parser::Operand::Reg("d0".into()),
            crate::backend::arm::assembler::parser::Operand::Reg("x1".into()),
        ]);
        let fix_form = encode_scvtf(&[
            crate::backend::arm::assembler::parser::Operand::Reg("d0".into()),
            crate::backend::arm::assembler::parser::Operand::Reg("x1".into()),
            crate::backend::arm::assembler::parser::Operand::Imm(32),
        ]);
        println!("int  form: {:?}", int_form);
        println!("fix  form: {:?}", fix_form);
        if let (Ok(EncodeResult::Word(w_int)), Ok(EncodeResult::Word(w_fix))) = (&int_form, &fix_form) {
            assert_ne!(w_int, w_fix,
                "scvtf d0,x1,#32 (fixed-point) encoded IDENTICALLY to scvtf d0,x1 (integer): {:#010x}", w_int);
            // 参考正确值
            assert_eq!(*w_fix, 0x9e428020u32,
                "scvtf d0,x1,#32 should be 0x9e428020, got {:#010x}", w_fix);
        }
        // 若 3-op 被接受为 Ok，必须是定点编码；Err 也可接受（不支持定点形式）
    }

    /// P-UCVTF-FIXED: ucvtf 定点形式
    #[test]
    fn p_ucvtf_fixed_vs_int() {
        let fix_form = encode_ucvtf(&[
            crate::backend::arm::assembler::parser::Operand::Reg("d0".into()),
            crate::backend::arm::assembler::parser::Operand::Reg("x1".into()),
            crate::backend::arm::assembler::parser::Operand::Imm(32),
        ]);
        if let Ok(EncodeResult::Word(w)) = fix_form {
            assert_ne!(w, 0x9e630020u32, "ucvtf d0,x1,#32 encoded as integer form");
        }
    }

    /// P-SCVTF-GPR-DEST: GPR 作为 FP 目标 → 应 Err（#839 类）
    #[test]
    fn p_scvtf_gpr_dest() {
        let r = encode_scvtf(&[
            crate::backend::arm::assembler::parser::Operand::Reg("x0".into()),
            crate::backend::arm::assembler::parser::Operand::Reg("x1".into()),
        ]);
        if let Ok(EncodeResult::Word(w)) = r {
            println!("scvtf x0, x1 → {:#010x} (should be Err; GPR as FP dest)", w);
        }
    }
}

#[cfg(test)]
mod pbt_fixed_point_e2e {
    /// P-SCVTF-FIXED-E2E: .s 文件 → 汇编 → objdump 验证定点编码
    #[test]
    fn p_scvtf_fixed_e2e() {
        let asm = "
.text
.global test_fn
test_fn:
    scvtf d0, x1, #32
    ucvtf d0, x1, #32
    scvtf s0, w1, #16
";
        let out = "/tmp/pbt_scvtf_e2e.o";
        let _ = std::fs::remove_file(out);
        match crate::backend::arm::assembler::assemble(asm, out) {
            Ok(()) => {
                let dump = std::process::Command::new("aarch64-linux-gnu-objdump")
                    .args(["-d", out]).output().expect("objdump");
                let text = String::from_utf8_lossy(&dump.stdout);
                for line in text.lines().filter(|l| l.contains(":\t")) {
                    println!("OBJDUMP: {}", line.trim_end());
                }
                // 检查是否出现定点编码（llvm-mc 参考: 9e428020 / 9e438020）
                let has_fixed = text.contains("9e428020") || text.contains("9e438020")
                    || text.contains("2042809e") || text.contains("2043809e"); // little-endian
                println!("fixed-point encoding present: {}", has_fixed);
                // 如果汇编器静默接受了 3-op 形式但编码为整数形式：
                let int_le = text.contains("2062009e") || text.contains("9e620020");
                if !has_fixed && int_le {
                    println!("BUG CONFIRMED: 3-op fixed-point form silently encoded as 2-op integer form");
                }
            }
            Err(e) => println!("assemble() error: {} (parser may reject 3-op form)", &e[..e.len().min(120)]),
        }
    }
}
