use super::*;

// ── Instruction encoders ──────────────────────────────────────────────

pub(crate) fn encode_lui(operands: &[Operand]) -> Result<EncodeResult, String> {
    let rd = get_reg(operands, 0)?;
    match &operands.get(1) {
        Some(Operand::Imm(imm)) => {
            Ok(EncodeResult::Word(encode_u(OP_LUI, rd, (*imm as u32) << 12)))
        }
        Some(Operand::Symbol(s)) => {
            // %hi(symbol)
            Ok(EncodeResult::WordWithReloc {
                word: encode_u(OP_LUI, rd, 0),
                reloc: Relocation {
                    reloc_type: if s.starts_with("%tprel_hi(") {
                        RelocType::TprelHi20
                    } else {
                        RelocType::Hi20
                    },
                    symbol: extract_modifier_symbol(s),
                    addend: 0,
                },
            })
        }
        _ => Err("lui: invalid operands".to_string()),
    }
}

pub(crate) fn encode_auipc(operands: &[Operand]) -> Result<EncodeResult, String> {
    let rd = get_reg(operands, 0)?;
    match &operands.get(1) {
        Some(Operand::Imm(imm)) => {
            Ok(EncodeResult::Word(encode_u(OP_AUIPC, rd, (*imm as u32) << 12)))
        }
        Some(Operand::Symbol(s)) => {
            let (reloc_type, symbol) = parse_reloc_modifier(s);
            Ok(EncodeResult::WordWithReloc {
                word: encode_u(OP_AUIPC, rd, 0),
                reloc: Relocation {
                    reloc_type,
                    symbol,
                    addend: 0,
                },
            })
        }
        _ => Err("auipc: invalid operands".to_string()),
    }
}

pub(crate) fn encode_jal(operands: &[Operand]) -> Result<EncodeResult, String> {
    // jal rd, offset  OR  jal offset (rd = ra)
    if operands.len() == 1 {
        // jal offset (implicit rd = ra)
        match &operands[0] {
            Operand::Imm(imm) => {
                Ok(EncodeResult::Word(encode_j(OP_JAL, 1, *imm as i32)))
            }
            Operand::Symbol(s) | Operand::Label(s) | Operand::Reg(s) => {
                Ok(EncodeResult::WordWithReloc {
                    word: encode_j(OP_JAL, 1, 0),
                    reloc: Relocation {
                        reloc_type: RelocType::Jal,
                        symbol: s.clone(),
                        addend: 0,
                    },
                })
            }
            _ => Err("jal: invalid operand".to_string()),
        }
    } else {
        let rd = get_reg(operands, 0)?;
        match &operands[1] {
            Operand::Imm(imm) => {
                Ok(EncodeResult::Word(encode_j(OP_JAL, rd, *imm as i32)))
            }
            Operand::Symbol(s) | Operand::Label(s) | Operand::Reg(s) => {
                Ok(EncodeResult::WordWithReloc {
                    word: encode_j(OP_JAL, rd, 0),
                    reloc: Relocation {
                        reloc_type: RelocType::Jal,
                        symbol: s.clone(),
                        addend: 0,
                    },
                })
            }
            _ => Err("jal: invalid operand".to_string()),
        }
    }
}

pub(crate) fn encode_jalr(operands: &[Operand]) -> Result<EncodeResult, String> {
    // jalr rd, rs1, offset  OR  jalr rd, offset(rs1)  OR  jalr rs1
    match operands.len() {
        1 => {
            // jalr rs1 (rd = ra, offset = 0)
            let rs1 = get_reg(operands, 0)?;
            Ok(EncodeResult::Word(encode_i(OP_JALR, 1, 0, rs1, 0)))
        }
        2 => {
            // jalr rd, rs1  (offset = 0)
            let rd = get_reg(operands, 0)?;
            match &operands[1] {
                Operand::Reg(name) => {
                    let rs1 = reg_num(name).ok_or("invalid register")?;
                    Ok(EncodeResult::Word(encode_i(OP_JALR, rd, 0, rs1, 0)))
                }
                Operand::Mem { base, offset } => {
                    let rs1 = reg_num(base).ok_or("invalid base register")?;
                    Ok(EncodeResult::Word(encode_i(OP_JALR, rd, 0, rs1, *offset as i32)))
                }
                _ => Err("jalr: invalid operands".to_string()),
            }
        }
        3 => {
            let rd = get_reg(operands, 0)?;
            let rs1 = get_reg(operands, 1)?;
            let imm = get_imm(operands, 2)?;
            Ok(EncodeResult::Word(encode_i(OP_JALR, rd, 0, rs1, imm as i32)))
        }
        _ => Err("jalr: wrong number of operands".to_string()),
    }
}

pub(crate) fn encode_branch_instr(operands: &[Operand], funct3: u32) -> Result<EncodeResult, String> {
    let rs1 = get_reg(operands, 0)?;
    let rs2 = get_reg(operands, 1)?;

    match &operands.get(2) {
        Some(Operand::Imm(imm)) => {
            Ok(EncodeResult::Word(encode_b(OP_BRANCH, funct3, rs1, rs2, *imm as i32)))
        }
        Some(Operand::Symbol(s)) | Some(Operand::Label(s)) | Some(Operand::Reg(s)) => {
            Ok(EncodeResult::WordWithReloc {
                word: encode_b(OP_BRANCH, funct3, rs1, rs2, 0),
                reloc: Relocation {
                    reloc_type: RelocType::Branch,
                    symbol: s.clone(),
                    addend: 0,
                },
            })
        }
        _ => Err("branch: expected offset or label as 3rd operand".to_string()),
    }
}

pub(crate) fn encode_load(operands: &[Operand], funct3: u32) -> Result<EncodeResult, String> {
    let rd = get_reg(operands, 0)?;
    match &operands.get(1) {
        Some(Operand::Mem { base, offset }) => {
            let rs1 = reg_num(base).ok_or("invalid base register")?;
            Ok(EncodeResult::Word(encode_i(OP_LOAD, rd, funct3, rs1, *offset as i32)))
        }
        Some(Operand::MemSymbol { base, symbol, .. }) => {
            let rs1 = reg_num(base).ok_or("invalid base register")?;
            let (reloc_type, sym) = parse_reloc_modifier(symbol);
            // Use Lo12I for load-type relocations
            let reloc_type = match reloc_type {
                RelocType::PcrelHi20 => RelocType::PcrelLo12I,
                RelocType::Hi20 => RelocType::Lo12I,
                RelocType::TprelHi20 => RelocType::TprelLo12I,
                other => other,
            };
            Ok(EncodeResult::WordWithReloc {
                word: encode_i(OP_LOAD, rd, funct3, rs1, 0),
                reloc: Relocation {
                    reloc_type,
                    symbol: sym,
                    addend: 0,
                },
            })
        }
        // Bare symbol: "ld rd, symbol" pseudo-instruction
        // Expand to: auipc rd, %pcrel_hi(symbol) ; ld rd, 0(rd)
        // with R_RISCV_PCREL_HI20 on auipc and R_RISCV_PCREL_LO12_I on ld
        Some(Operand::Symbol(s)) | Some(Operand::Label(s)) => {
            Ok(EncodeResult::WordsWithRelocs(vec![
                (encode_u(OP_AUIPC, rd, 0), Some(Relocation {
                    reloc_type: RelocType::PcrelHi20,
                    symbol: s.clone(),
                    addend: 0,
                })),
                (encode_i(OP_LOAD, rd, funct3, rd, 0), Some(Relocation {
                    reloc_type: RelocType::PcrelLo12I,
                    symbol: s.clone(),
                    addend: 0,
                })),
            ]))
        }
        _ => Err("load: expected memory operand".to_string()),
    }
}

pub(crate) fn encode_store(operands: &[Operand], funct3: u32) -> Result<EncodeResult, String> {
    let rs2 = get_reg(operands, 0)?;
    match &operands.get(1) {
        Some(Operand::Mem { base, offset }) => {
            let rs1 = reg_num(base).ok_or("invalid base register")?;
            Ok(EncodeResult::Word(encode_s(OP_STORE, funct3, rs1, rs2, *offset as i32)))
        }
        Some(Operand::MemSymbol { base, symbol, .. }) => {
            let rs1 = reg_num(base).ok_or("invalid base register")?;
            let (reloc_type, sym) = parse_reloc_modifier(symbol);
            let reloc_type = match reloc_type {
                RelocType::PcrelHi20 => RelocType::PcrelLo12S,
                RelocType::Hi20 => RelocType::Lo12S,
                RelocType::TprelHi20 => RelocType::TprelLo12S,
                other => other,
            };
            Ok(EncodeResult::WordWithReloc {
                word: encode_s(OP_STORE, funct3, rs1, rs2, 0),
                reloc: Relocation {
                    reloc_type,
                    symbol: sym,
                    addend: 0,
                },
            })
        }
        _ => Err("store: expected memory operand".to_string()),
    }
}

pub(crate) fn encode_alu_imm(operands: &[Operand], funct3: u32) -> Result<EncodeResult, String> {
    let rd = get_reg(operands, 0)?;
    let rs1 = get_reg(operands, 1)?;
    match &operands.get(2) {
        Some(Operand::Imm(imm)) => {
            Ok(EncodeResult::Word(encode_i(OP_OP_IMM, rd, funct3, rs1, *imm as i32)))
        }
        Some(Operand::Symbol(s)) => {
            let (reloc_type, sym) = parse_reloc_modifier(s);
            let reloc_type = match reloc_type {
                RelocType::PcrelHi20 => RelocType::PcrelLo12I,
                RelocType::Hi20 => RelocType::Lo12I,
                RelocType::TprelHi20 => RelocType::TprelLo12I,
                other => other,
            };
            Ok(EncodeResult::WordWithReloc {
                word: encode_i(OP_OP_IMM, rd, funct3, rs1, 0),
                reloc: Relocation {
                    reloc_type,
                    symbol: sym,
                    addend: 0,
                },
            })
        }
        _ => Err("alu_imm: expected immediate".to_string()),
    }
}

pub(crate) fn encode_shift_imm(operands: &[Operand], funct3: u32, funct6: u32) -> Result<EncodeResult, String> {
    let rd = get_reg(operands, 0)?;
    let rs1 = get_reg(operands, 1)?;
    let shamt = get_imm(operands, 2)? as u32;
    // For RV64, shift amount is 6 bits
    let imm = (funct6 << 6) | (shamt & 0x3F);
    Ok(EncodeResult::Word(encode_i(OP_OP_IMM, rd, funct3, rs1, imm as i32)))
}

pub(crate) fn encode_alu_reg(operands: &[Operand], funct3: u32, funct7: u32) -> Result<EncodeResult, String> {
    let rd = get_reg(operands, 0)?;
    let rs1 = get_reg(operands, 1)?;
    let rs2 = get_reg(operands, 2)?;
    Ok(EncodeResult::Word(encode_r(OP_OP, rd, funct3, rs1, rs2, funct7)))
}

pub(crate) fn encode_alu_imm_w(operands: &[Operand], funct3: u32) -> Result<EncodeResult, String> {
    let rd = get_reg(operands, 0)?;
    let rs1 = get_reg(operands, 1)?;
    let imm = get_imm(operands, 2)? as i32;
    Ok(EncodeResult::Word(encode_i(OP_OP_IMM_32, rd, funct3, rs1, imm)))
}

pub(crate) fn encode_shift_imm_w(operands: &[Operand], funct3: u32, funct7: u32) -> Result<EncodeResult, String> {
    let rd = get_reg(operands, 0)?;
    let rs1 = get_reg(operands, 1)?;
    let shamt = get_imm(operands, 2)? as u32;
    // For RV32/W operations, shift amount is 5 bits
    let imm = (funct7 << 5) | (shamt & 0x1F);
    Ok(EncodeResult::Word(encode_i(OP_OP_IMM_32, rd, funct3, rs1, imm as i32)))
}

pub(crate) fn encode_alu_reg_w(operands: &[Operand], funct3: u32, funct7: u32) -> Result<EncodeResult, String> {
    let rd = get_reg(operands, 0)?;
    let rs1 = get_reg(operands, 1)?;
    let rs2 = get_reg(operands, 2)?;
    Ok(EncodeResult::Word(encode_r(OP_OP_32, rd, funct3, rs1, rs2, funct7)))
}

// ── Zbb (bit manipulation) helpers ──

/// Encode a Zbb unary instruction (clz, ctz, cpop, sext.b, sext.h, rev8).
/// These are I-type with funct3=001 and the 12-bit immediate encoding the operation.
pub(crate) fn encode_zbb_unary(operands: &[Operand], imm12: u32) -> Result<EncodeResult, String> {
    let rd = get_reg(operands, 0)?;
    let rs1 = get_reg(operands, 1)?;
    Ok(EncodeResult::Word(encode_i(OP_OP_IMM, rd, 0b001, rs1, imm12 as i32)))
}

/// Encode a Zbb unary instruction with funct3=101 (rev8, orc.b).
pub(crate) fn encode_zbb_unary_f5(operands: &[Operand], imm12: u32) -> Result<EncodeResult, String> {
    let rd = get_reg(operands, 0)?;
    let rs1 = get_reg(operands, 1)?;
    Ok(EncodeResult::Word(encode_i(OP_OP_IMM, rd, 0b101, rs1, imm12 as i32)))
}

/// Encode a Zbb unary word instruction (clzw, ctzw, cpopw).
/// These are I-type on OP-IMM-32 with funct3=001.
pub(crate) fn encode_zbb_unary_w(operands: &[Operand], imm12: u32) -> Result<EncodeResult, String> {
    let rd = get_reg(operands, 0)?;
    let rs1 = get_reg(operands, 1)?;
    Ok(EncodeResult::Word(encode_i(OP_OP_IMM_32, rd, 0b001, rs1, imm12 as i32)))
}

/// Encode zext.h rd, rs1 (R-type on OP-32: funct7=0000100, rs2=0, funct3=100).
pub(crate) fn encode_zbb_zexth(operands: &[Operand]) -> Result<EncodeResult, String> {
    let rd = get_reg(operands, 0)?;
    let rs1 = get_reg(operands, 1)?;
    Ok(EncodeResult::Word(encode_r(OP_OP_32, rd, 0b100, rs1, 0, 0b0000100)))
}

#[cfg(test)]
mod pbt_zbb_diff {
    /// P-ZBB-DIFF: 17 个 Zbb 形式与 llvm-21 (-mattr=+zbb) 字节精确比对
    /// 参考 #1041 只报了扩展门控缺失；编码位级从未被验证（当时 llvm-mc 10.0 不支持）
    #[test]
    fn p_zbb_byte_exact() {
        let asm = ".text\nt:\n\
            andn x1, x2, x3\n orn x1, x2, x3\n xnor x1, x2, x3\n\
            clz x1, x2\n ctz x1, x2\n cpop x1, x2\n\
            rol x1, x2, x3\n ror x1, x2, x3\n rolw x1, x2, x3\n rorw x1, x2, x3\n\
            rev8 x1, x2\n\
            min x1, x2, x3\n max x1, x2, x3\n minu x1, x2, x3\n maxu x1, x2, x3\n\
            sext.b x1, x2\n sext.h x1, x2\n zext.h x1, x2\n";
        // llvm-21 参考词（x1,x2,x3 = ra,sp,gp）
        let expect: Vec<(&str, u32)> = vec![
            ("andn",   0x403170b3), ("orn",  0x403160b3), ("xnor", 0x403140b3),
            ("clz",    0x60011093), ("ctz",  0x60111093),
            ("pcnt/cpop", 0x60211093),
            ("rol",    0x603110b3), ("ror",  0x603150b3),
            ("rolw",   0x603110bb), ("rorw", 0x603150bb),
            ("rev8",   0x6b815093),
            ("min",    0x0a3140b3), ("max",  0x0a3160b3),
            ("minu",   0x0a3150b3), ("maxu", 0x0a3170b3),
            ("sext.b", 0x60411093), ("sext.h", 0x60511093),
            ("zext.h", 0x080140bb),
        ];
        let out = "/tmp/pbt_zbb.o";
        let _ = std::fs::remove_file(out);
        match crate::backend::riscv::assembler::assemble_with_args(asm, out, &[]) {
            Ok(()) => {
                let d = std::process::Command::new("/usr/lib/llvm-21/bin/llvm-objdump")
                    .args(["-d", "--triple=riscv64", "--mattr=+zbb", out]).output().unwrap();
                let text = String::from_utf8_lossy(&d.stdout);
                // llvm-objdump format: "  0: 403170b3  andn ra, sp, gp" (single word)
                let words: Vec<u32> = text.lines()
                    .filter_map(|l| {
                        let cols: Vec<&str> = l.split_whitespace().collect();
                        if cols.len() >= 2 {
                            let addr = cols[0].trim_end_matches(':');
                            if u32::from_str_radix(addr, 16).is_ok() && cols[1].len() == 8 {
                                return u32::from_str_radix(cols[1], 16).ok();
                            }
                        }
                        None
                    })
                    .collect();
                println!("repo emitted {} words", words.len());
                let mut mismatch = 0;
                for (i, (name, want)) in expect.iter().enumerate() {
                    if let Some(&got) = words.get(i) {
                        if got != *want {
                            println!("  MISMATCH {}: got {:#010x}, want {:#010x}", name, got, want);
                            mismatch += 1;
                        }
                    } else {
                        println!("  MISSING {}: repo emitted only {} words", name, words.len());
                        mismatch += 1;
                    }
                }
                // 打印全部反汇编供人工检查
                for l in text.lines().filter(|l| l.contains(':') && l.contains("0x")) {
                    println!("  DUMP: {}", l.trim_end());
                }
                assert_eq!(mismatch, 0, "{} Zbb encodings mismatched llvm-21", mismatch);
            }
            Err(e) => panic!("repo assembler rejected Zbb input: {}", &e[..e.len().min(200)]),
        }
    }
}

#[cfg(test)]
mod pbt_extra_base_zbb {
    use super::*;
    use crate::backend::riscv::assembler::parser::Operand;
    fn r(s: &str) -> Operand { Operand::Reg(s.to_string()) }
    fn i(v: i64) -> Operand { Operand::Imm(v) }

    #[test]
    fn p_extra_base_zbb_sweep() {
        let cases: Vec<(&str, Vec<Operand>, Vec<Operand>)> = vec![
            ("encode_zbb_unary", vec![r("a0"), r("a1")], vec![r("a0"), r("a1"), r("a2")]),
            ("encode_zbb_unary_f5", vec![r("a0"), r("a1")], vec![r("a0"), r("a1"), r("a2")]),
            ("encode_zbb_unary_w", vec![r("a0"), r("a1")], vec![r("a0"), r("a1"), r("a2")]),
            ("encode_zbb_zexth", vec![r("a0"), r("a1")], vec![r("a0"), r("a1"), r("a2")]),
            ("encode_zbb_binary", vec![r("a0"), r("a1"), r("a2")], vec![r("a0"), r("a1"), r("a2"), r("a3")]),
            ("encode_shift_imm", vec![r("a0"), r("a1"), i(3)], vec![r("a0"), r("a1"), i(3), r("a2")]),
            ("encode_shift_imm_w", vec![r("a0"), r("a1"), i(3)], vec![r("a0"), r("a1"), i(3), r("a2")]),
            ("encode_amo_suffixed", vec![r("a0"), r("a1"), Operand::Mem { base: "a2".into(), offset: 0 }],
                vec![r("a0"), r("a1"), Operand::Mem { base: "a2".into(), offset: 0 }, r("a3")]),
            ("encode_lr_suffixed", vec![r("a0"), Operand::Mem { base: "a1".into(), offset: 0 }],
                vec![r("a0"), Operand::Mem { base: "a1".into(), offset: 0 }, r("a2")]),
            ("encode_sc_suffixed", vec![r("a0"), r("a1"), Operand::Mem { base: "a2".into(), offset: 0 }],
                vec![r("a0"), r("a1"), Operand::Mem { base: "a2".into(), offset: 0 }, r("a3")]),
        ];
        let mut ignored = Vec::new();
        for (name, base, extra) in &cases {
            let rb = dispatch2(name, base);
            let re = dispatch2(name, extra);
            match (rb, re) {
                (Ok(_), Ok(_)) => { println!("  {}: extra IGNORED", name); ignored.push(*name); }
                (Ok(_), Err(_)) => println!("  {}: extra REJECTED (correct)", name),
                (Err(e), _) => println!("  {}: BASE FAILS: {}", name, &e[..e.len().min(50)]),
            }
        }
        println!("IGNORED: {:?}", ignored);
        // 10/10 ignore extras — ALL filed: zbb_unary/f5/w/zexth/shift_imm/_w → #1122-#1127; amo/lr/sc_suffixed → #1128-#1130
        let known = ["encode_zbb_unary","encode_zbb_unary_f5","encode_zbb_unary_w","encode_zbb_zexth",
                     "encode_shift_imm","encode_shift_imm_w","encode_amo_suffixed","encode_lr_suffixed","encode_sc_suffixed",
                     "encode_zbb_binary" /* = encode_alu_reg, #946 */];
        let unexpected: Vec<_> = ignored.iter().filter(|n| !known.contains(n)).collect();
        assert!(unexpected.is_empty(), "NEW extra-operand ignorer: {:?}", unexpected);
    }

    fn dispatch2(name: &str, ops: &[Operand]) -> Result<EncodeResult, String> {
        match name {
            "encode_zbb_unary" => encode_zbb_unary(ops, 0x600),
            "encode_zbb_unary_f5" => encode_zbb_unary_f5(ops, 0b0110000),
            "encode_zbb_unary_w" => encode_zbb_unary_w(ops, 0x600),
            "encode_zbb_zexth" => encode_zbb_zexth(ops),
            "encode_zbb_binary" => encode_alu_reg(ops, 0b111, 0b0100000),
            "encode_shift_imm" => encode_shift_imm(ops, 0b001, 0b000000),
            "encode_shift_imm_w" => encode_shift_imm_w(ops, 0b001, 0b0000000),
            "encode_amo_suffixed" => crate::backend::riscv::assembler::encoder::atomics::encode_amo_suffixed("amoadd.w", ops),
            "encode_lr_suffixed" => crate::backend::riscv::assembler::encoder::atomics::encode_lr_suffixed("lr.w", ops),
            _ => crate::backend::riscv::assembler::encoder::atomics::encode_sc_suffixed("sc.w", ops),
        }
    }
}
