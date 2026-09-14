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
mod encode_fcvt_rounding_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler
    // Evidence: src/backend/arm/assembler/README.md "accepts the same textual assembly that GCC's gas would consume";
    //   encoder/mod.rs "Encodes AArch64 instructions into 32-bit machine code words";
    //   encoder/mod.rs:440-453 fcvtzs/fcvtzu/fcvtas/au/ns/nu/ms/mu/ps/pu dispatch;
    //   ARM ARM Conversion between floating-point and integer:
    //   sf 00 11110 ftype 1 rmode opcode 000000 Rn Rd;
    //   fp_scalar.rs:180-183 purpose comment (sf W/X, ftype S/D);
    //   README.md:223 lists the 10 scalar mnemonics.
    // Stronger considered:
    //   - State machine: rejected — encode_fcvt_rounding is a pure function with no lifecycle
    //   - Algebraic round-trip: rejected — no in-tree FCVT* integer decoder
    //   - encode_int_to_float / encode_scvtf / encode_ucvtf as differential sibling:
    //     rejected — same-job gate fails (integer-to-float, different ARM class)
    //   - encode_fcvt_precision: rejected — float-to-float precision conversion
    //   - encode_neon_float_two_misc: rejected — vector/SIMD-scalar form
    // Weaker available: algebraic.metamorphic (sf/ftype/rmode/opcode/Rd/Rn),
    //   algebraic.invariant (ARM fields), negative_error (arity / extra / SP / wrong type)
    // Differential: candidate=encode_fcvt_rounding, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler (caller-reachable from encode_instruction),
    //   mapping=[Reg(Wd|Xd), Reg(Sn|Dn)]+(rmode,opcode) <-> `fcvt* Wd|Xd, Sn|Dn`

    use super::encode_fcvt_rounding;
    use super::super::EncodeResult;
    use crate::backend::arm::assembler::parser::Operand;
    use proptest::prelude::*;
    use std::io::Write;
    use std::process::{Command, Stdio};

    const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
    const CASES: u32 = 1000;

    /// Documented integer-conversion (rmode, opcode, mnemonic) triples from encoder/mod.rs:440-453.
    const FCVT_INT: &[(u32, u32, &str)] = &[
        (0b11, 0b000, "fcvtzs"),
        (0b11, 0b001, "fcvtzu"),
        (0b00, 0b100, "fcvtas"),
        (0b00, 0b101, "fcvtau"),
        (0b00, 0b000, "fcvtns"),
        (0b00, 0b001, "fcvtnu"),
        (0b10, 0b000, "fcvtms"),
        (0b10, 0b001, "fcvtmu"),
        (0b01, 0b000, "fcvtps"),
        (0b01, 0b001, "fcvtpu"),
    ];

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

    fn fp(is_d: bool, n: u32) -> String {
        format!("{}{}", if is_d { "d" } else { "s" }, n)
    }

    fn sut_word(ops: &[Operand], rmode: u32, opcode: u32) -> Result<u32, String> {
        match encode_fcvt_rounding(ops, rmode, opcode)? {
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

    fn llvm_mc_word_with(asm: &str, extra_args: &[&str]) -> Result<u32, String> {
        let mut args = vec!["-triple=aarch64", "-show-encoding"];
        args.extend_from_slice(extra_args);
        let mut child = Command::new(LLVM_MC)
            .args(&args)
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

    fn llvm_mc_word(asm: &str) -> Result<u32, String> {
        llvm_mc_word_with(asm, &[])
    }

    fn llvm_mc_fp16_word(asm: &str) -> Result<u32, String> {
        llvm_mc_word_with(asm, &["-mattr=+fullfp16"])
    }

    fn extra_operand() -> impl Strategy<Value = Operand> {
        prop_oneof![
            (0u32..=31).prop_map(|n| Operand::Reg(format!("x{n}"))),
            Just(Operand::Imm(0)),
            Just(Operand::Imm(1)),
            Just(Operand::Imm(32)),
            Just(Operand::Shift {
                kind: "lsl".into(),
                amount: 0,
            }),
            Just(Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "4s".into(),
            }),
        ]
    }

    fn dest_spelling(is_64: bool, n: u32, kind: u32) -> String {
        match kind {
            0 if n == 31 && is_64 => "x31".into(),
            0 if n == 31 && !is_64 => "w31".into(),
            1 if n == 31 && is_64 => "XZR".into(),
            1 if n == 31 && !is_64 => "WZR".into(),
            2 if n == 30 && is_64 => "LR".into(),
            2 if n == 30 && is_64 => "lr".into(),
            3 => gpr(is_64, n).to_uppercase(),
            _ => gpr(is_64, n),
        }
    }

    fn src_spelling(is_d: bool, n: u32, kind: u32) -> String {
        match kind {
            0 => fp(is_d, n).to_uppercase(),
            _ => fp(is_d, n),
        }
    }

    fn wrong_type_pair() -> impl Strategy<Value = (String, String)> {
        let n = 0u32..=31;
        prop_oneof![
            // FP dest (including SIMD-scalar S/D) + S/D source
            (n.clone(), n.clone(), 0u32..=5, any::<bool>()).prop_map(|(d, s, p, src_d)| {
                let pref = ["s", "d", "h", "q", "v", "b"][p as usize];
                (format!("{pref}{d}"), fp(src_d, s))
            }),
            // W/X dest + GP source
            (n.clone(), n.clone(), any::<bool>(), any::<bool>()).prop_map(|(d, s, d64, s64)| {
                (gpr(d64, d), gpr(s64, s))
            }),
            // W/X dest + Q/V/B source
            (n.clone(), n.clone(), any::<bool>(), 0u32..=2).prop_map(|(d, s, d64, p)| {
                let pref = ["q", "v", "b"][p as usize];
                (gpr(d64, d), format!("{pref}{s}"))
            }),
            // Q/V/B dest + S/D source
            (n.clone(), n.clone(), 0u32..=2, any::<bool>()).prop_map(|(d, s, p, src_d)| {
                let pref = ["q", "v", "b"][p as usize];
                (format!("{pref}{d}"), fp(src_d, s))
            }),
        ]
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_fcvt_rounding_kat_llvm_mc_fcvtzs_w0_s1() {
        let want = 0x1e380020u32;
        let mc = llvm_mc_word("fcvtzs w0, s1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [Operand::Reg("w0".into()), Operand::Reg("s1".into())];
        let sut = sut_word(&ops, 0b11, 0b000).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_fcvt_rounding_kat_llvm_mc_fcvtzs_x0_d1() {
        let want = 0x9e780020u32;
        let mc = llvm_mc_word("fcvtzs x0, d1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [Operand::Reg("x0".into()), Operand::Reg("d1".into())];
        let sut = sut_word(&ops, 0b11, 0b000).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_fcvt_rounding_kat_llvm_mc_fcvtzu_w0_s1() {
        let want = 0x1e390020u32;
        let mc = llvm_mc_word("fcvtzu w0, s1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [Operand::Reg("w0".into()), Operand::Reg("s1".into())];
        let sut = sut_word(&ops, 0b11, 0b001).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_fcvt_rounding_kat_llvm_mc_fcvtas_w0_s1() {
        let want = 0x1e240020u32;
        let mc = llvm_mc_word("fcvtas w0, s1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [Operand::Reg("w0".into()), Operand::Reg("s1".into())];
        let sut = sut_word(&ops, 0b00, 0b100).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_fcvt_rounding_kat_llvm_mc_fcvtzs_xzr_d0() {
        let want = 0x9e78001fu32;
        let mc = llvm_mc_word("fcvtzs xzr, d0").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [Operand::Reg("xzr".into()), Operand::Reg("d0".into())];
        let sut = sut_word(&ops, 0b11, 0b000).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_fcvt_rounding_kat_llvm_mc_fcvtzs_lr_s0() {
        let want = 0x9e38001eu32;
        let mc = llvm_mc_word("fcvtzs lr, s0").expect("llvm-mc LR KAT");
        assert_eq!(mc, want, "llvm-mc LR KAT mapping broken");
        let ops = [Operand::Reg("lr".into()), Operand::Reg("s0".into())];
        let sut = sut_word(&ops, 0b11, 0b000).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_fcvt_rounding_kat_llvm_mc_half_fcvtzs_w0_h1() {
        let want = 0x1ef80020u32;
        let mc = llvm_mc_fp16_word("fcvtzs w0, h1").expect("llvm-mc fp16 KAT");
        assert_eq!(mc, want, "llvm-mc fp16 KAT mapping broken");
    }

    #[test]
    fn test_encode_fcvt_rounding_regression_extra_operand() {
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Reg("s1".into()),
            Operand::Reg("x0".into()),
        ];
        assert!(
            encode_fcvt_rounding(&ops, 0b11, 0b000).is_err(),
            "integer FCVT* must reject a 3rd operand"
        );
    }

    #[test]
    fn test_encode_fcvt_rounding_regression_sp_dest() {
        let ops = [Operand::Reg("wsp".into()), Operand::Reg("s0".into())];
        assert!(
            encode_fcvt_rounding(&ops, 0b11, 0b000).is_err(),
            "WSP/SP is not a valid integer FCVT* dest"
        );
    }

    #[test]
    fn test_encode_fcvt_rounding_regression_fp_dest() {
        let ops = [Operand::Reg("s0".into()), Operand::Reg("s0".into())];
        assert!(
            encode_fcvt_rounding(&ops, 0b11, 0b000).is_err(),
            "FP dest is SIMD-scalar FCVTZS, not integer conversion"
        );
    }

    #[test]
    fn test_encode_fcvt_rounding_regression_half_ftype() {
        let ops = [Operand::Reg("w0".into()), Operand::Reg("h1".into())];
        let sut = sut_word(&ops, 0b11, 0b000).expect("H source is a valid fp16 integer FCVTZS");
        assert_eq!(
            sut, 0x1ef80020u32,
            "H source must use ftype=11 (0x1ef80020), not ftype=00 S"
        );
    }

    proptest! {
        #![proptest_config(cfg())]

        #[test]
        fn encode_fcvt_rounding_diff_valid(
            rd in 0u32..=31,
            rn in 0u32..=31,
            dest64 in any::<bool>(),
            src_d in any::<bool>(),
            idx in 0usize..FCVT_INT.len(),
            dest_kind in 0u32..=4,
            src_kind in 0u32..=1,
        ) {
            let (rmode, opcode, mnem) = FCVT_INT[idx];
            let dest = dest_spelling(dest64, rd, dest_kind);
            let src = src_spelling(src_d, rn, src_kind);
            let asm = format!("{mnem} {dest}, {src}");
            let ops = [Operand::Reg(dest.clone()), Operand::Reg(src.clone())];
            let mc = llvm_mc_word(&asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid {asm}: {e}"));
            let sut = sut_word(&ops, rmode, opcode)
                .unwrap_or_else(|e| panic!("SUT rejected valid {asm}: {e}"));
            prop_assert_eq!(sut, mc, "FCVT* mismatch for {}", asm);
        }

        #[test]
        fn encode_fcvt_rounding_arm_fields(
            rd in 0u32..=31,
            rn in 0u32..=31,
            dest64 in any::<bool>(),
            src_d in any::<bool>(),
            idx in 0usize..FCVT_INT.len(),
        ) {
            let (rmode, opcode, _) = FCVT_INT[idx];
            let ops = [
                Operand::Reg(gpr(dest64, rd)),
                Operand::Reg(fp(src_d, rn)),
            ];
            let w = sut_word(&ops, rmode, opcode).expect("SUT");
            let sf = if dest64 { 1u32 } else { 0 };
            let ftype = if src_d { 0b01u32 } else { 0b00 };
            let want = (sf << 31)
                | (0b11110 << 24)
                | (ftype << 22)
                | (1 << 21)
                | (rmode << 19)
                | (opcode << 16)
                | (rn << 5)
                | rd;
            prop_assert_eq!(w, want, "ARM ARM FCVT* integer field layout");
            prop_assert_eq!((w >> 31) & 1, sf, "sf");
            prop_assert_eq!((w >> 29) & 0b11, 0, "bits[30:29] must be 00");
            prop_assert_eq!((w >> 24) & 0b11111, 0b11110, "bits[28:24]");
            prop_assert_eq!((w >> 22) & 0b11, ftype, "ftype");
            prop_assert_eq!((w >> 21) & 1, 1, "bit21 must be 1 (integer, not fixed-point)");
            prop_assert_eq!((w >> 19) & 0b11, rmode, "rmode");
            prop_assert_eq!((w >> 16) & 0b111, opcode, "opcode");
            prop_assert_eq!((w >> 10) & 0b111111, 0, "scale/bits[15:10] must be 0");
            prop_assert_eq!((w >> 5) & 0x1f, rn, "Rn");
            prop_assert_eq!(w & 0x1f, rd, "Rd");
        }

        #[test]
        fn encode_fcvt_rounding_metamorphic_fields(
            rd in 0u32..=30,
            rn in 0u32..=30,
            dest64 in any::<bool>(),
            src_d in any::<bool>(),
        ) {
            let ops = |
                d: u32,
                n: u32,
                d64: bool,
                sd: bool,
            | {
                [
                    Operand::Reg(gpr(d64, d)),
                    Operand::Reg(fp(sd, n)),
                ]
            };
            let w = sut_word(&ops(rd, rn, dest64, src_d), 0b11, 0b000).expect("base FCVTZS");
            let w_rd = sut_word(&ops(rd + 1, rn, dest64, src_d), 0b11, 0b000).expect("Rd+1");
            let w_rn = sut_word(&ops(rd, rn + 1, dest64, src_d), 0b11, 0b000).expect("Rn+1");
            let w_sf = sut_word(&ops(rd, rn, !dest64, src_d), 0b11, 0b000).expect("sf flip");
            let w_ft = sut_word(&ops(rd, rn, dest64, !src_d), 0b11, 0b000).expect("ftype flip");
            let w_u = sut_word(&ops(rd, rn, dest64, src_d), 0b11, 0b001).expect("FCVTZU");
            prop_assert_eq!(w_rd, w + 1, "Rd+1 must increment bits[4:0] only");
            prop_assert_eq!(w_rn, w + (1 << 5), "Rn+1 must increment bits[9:5] only");
            prop_assert_eq!(w_sf ^ w, 1u32 << 31, "W vs X dest must flip only sf bit 31");
            prop_assert_eq!(w_ft ^ w, 1u32 << 22, "S vs D source must flip only ftype bit 22");
            prop_assert_eq!(w_u ^ w, 1u32 << 16, "FCVTZS XOR FCVTZU must be opcode LSB bit 16");
        }

        #[test]
        fn encode_fcvt_rounding_neg_arity(
            len in 0usize..=1,
            n in 0u32..=31,
        ) {
            let mut ops = Vec::new();
            if len >= 1 {
                ops.push(Operand::Reg(gpr(false, n)));
            }
            prop_assert!(
                encode_fcvt_rounding(&ops, 0b11, 0b000).is_err(),
                "FCVT* integer form with {} operands must Err (llvm-mc: too few operands)",
                len
            );
        }

        #[test]
        fn encode_fcvt_rounding_neg_extra_operand(
            rd in 0u32..=31,
            rn in 0u32..=31,
            dest64 in any::<bool>(),
            src_d in any::<bool>(),
            extra in extra_operand(),
        ) {
            let ops = vec![
                Operand::Reg(gpr(dest64, rd)),
                Operand::Reg(fp(src_d, rn)),
                extra,
            ];
            prop_assert!(
                encode_fcvt_rounding(&ops, 0b11, 0b000).is_err(),
                "FCVT* integer form has no 3rd operand; extra must Err (fixed-point is a different encoding)"
            );
        }

        #[test]
        fn encode_fcvt_rounding_neg_sp_dest(
            is_64 in any::<bool>(),
            rn in 0u32..=31,
            src_d in any::<bool>(),
        ) {
            let sp = if is_64 { "sp" } else { "wsp" };
            let ops = [
                Operand::Reg(sp.into()),
                Operand::Reg(fp(src_d, rn)),
            ];
            prop_assert!(
                encode_fcvt_rounding(&ops, 0b11, 0b000).is_err(),
                "SP/WSP is not a valid FCVT* integer dest (sp={})",
                sp
            );
        }

        #[test]
        fn encode_fcvt_rounding_neg_wrong_types(
            (dest, src) in wrong_type_pair(),
        ) {
            let ops = [Operand::Reg(dest.clone()), Operand::Reg(src.clone())];
            prop_assert!(
                encode_fcvt_rounding(&ops, 0b11, 0b000).is_err(),
                "FCVT* integer form requires Wd|Xd, Sn|Dn; dest={} src={} must Err",
                dest,
                src
            );
        }

        #[test]
        fn encode_fcvt_rounding_neg_nonreg(
            which in 0u32..=1,
            kind in 0u32..=5,
        ) {
            let bad = match kind {
                0 => Operand::Imm(0),
                1 => Operand::Symbol("foo".into()),
                2 => Operand::Label("1f".into()),
                3 => Operand::Mem {
                    base: "x0".into(),
                    offset: 0,
                },
                4 => Operand::Cond("eq".into()),
                _ => Operand::Shift {
                    kind: "lsl".into(),
                    amount: 0,
                },
            };
            let mut ops = vec![
                Operand::Reg("w0".into()),
                Operand::Reg("s0".into()),
            ];
            ops[which as usize] = bad;
            prop_assert!(
                encode_fcvt_rounding(&ops, 0b11, 0b000).is_err(),
                "non-register at slot {} must Err",
                which
            );
        }

        #[test]
        fn encode_fcvt_rounding_neg_invalid_name(
            which in 0u32..=1,
            name in prop::sample::select(vec![
                "foo", "x32", "w32", "s32", "d32", "r0", "x", "s", "",
            ]),
        ) {
            let mut ops = vec![
                Operand::Reg("w0".into()),
                Operand::Reg("s0".into()),
            ];
            ops[which as usize] = Operand::Reg(name.to_string());
            prop_assert!(
                encode_fcvt_rounding(&ops, 0b11, 0b000).is_err(),
                "invalid name {:?} at slot {} must Err",
                name,
                which
            );
        }

        #[test]
        fn encode_fcvt_rounding_diff_half(
            rd in 0u32..=31,
            rn in 0u32..=31,
            dest64 in any::<bool>(),
            idx in 0usize..FCVT_INT.len(),
        ) {
            let (rmode, opcode, mnem) = FCVT_INT[idx];
            let dest = gpr(dest64, rd);
            let src = format!("h{rn}");
            let asm = format!("{mnem} {dest}, {src}");
            let ops = [Operand::Reg(dest.clone()), Operand::Reg(src)];
            let mc = llvm_mc_fp16_word(&asm)
                .unwrap_or_else(|e| panic!("llvm-mc fp16 rejected valid {asm}: {e}"));
            let sut = sut_word(&ops, rmode, opcode)
                .unwrap_or_else(|e| panic!("SUT rejected valid fp16 {asm}: {e}"));
            prop_assert_eq!(sut, mc, "FCVT* half-precision mismatch for {}", asm);
        }
    }
}

#[cfg(test)]
mod encode_fp_1src_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler
    // Evidence: src/backend/arm/assembler/README.md "accepts the same textual assembly that GCC's gas would consume";
    //   encoder/mod.rs "Encodes AArch64 instructions into 32-bit machine code words";
    //   encoder/mod.rs:414-434 frintn/p/m/z/a/x/i dispatch to encode_fp_1src;
    //   ARM ARM Floating-point data-processing (1 source):
    //   M=0 S=0 11110 ftype 1 opcode 10000 Rn Rd;
    //   fp_scalar.rs:115-116 purpose comment (FRINTN/P/M/Z/A/X/I);
    //   README.md:223 lists the 7 scalar mnemonics.
    // Stronger considered:
    //   - State machine: rejected — encode_fp_1src is a pure function with no lifecycle
    //   - Algebraic round-trip: rejected — no in-tree FRINT decoder
    //   - encode_fneg / encode_fabs / encode_fsqrt as differential sibling:
    //     rejected — same-job gate fails (FNEG/FABS/FSQRT, hardcoded opcodes)
    //   - encode_neon_float_two_misc: rejected — vector/SIMD-scalar form
    //   - encode_fp_arith: rejected — 2-source FP arithmetic
    // Weaker available: algebraic.metamorphic (ftype/opcode/Rd/Rn),
    //   algebraic.invariant (ARM fields), negative_error (arity / extra / wrong type)
    // Differential: candidate=encode_fp_1src, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler (caller-reachable from encode_instruction),
    //   mapping=[Reg(Sd|Dd), Reg(Sn|Dn)]+opcode <-> `frint* Sd|Dd, Sn|Dn`

    use super::encode_fp_1src;
    use super::super::EncodeResult;
    use crate::backend::arm::assembler::parser::Operand;
    use proptest::prelude::*;
    use std::io::Write;
    use std::process::{Command, Stdio};

    const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
    const CASES: u32 = 1000;

    /// Documented FRINT* (opcode, mnemonic) pairs from encoder/mod.rs:414-434.
    const FRINT: &[(u32, &str)] = &[
        (0b001000, "frintn"),
        (0b001001, "frintp"),
        (0b001010, "frintm"),
        (0b001011, "frintz"),
        (0b001100, "frinta"),
        (0b001110, "frintx"),
        (0b001111, "frinti"),
    ];

    fn cfg() -> ProptestConfig {
        ProptestConfig::with_cases(CASES)
    }

    fn fp(is_d: bool, n: u32) -> String {
        format!("{}{}", if is_d { "d" } else { "s" }, n)
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

    fn sut_word(ops: &[Operand], opcode: u32) -> Result<u32, String> {
        match encode_fp_1src(ops, opcode)? {
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

    fn llvm_mc_word_with(asm: &str, extra_args: &[&str]) -> Result<u32, String> {
        let mut args = vec!["-triple=aarch64", "-show-encoding"];
        args.extend_from_slice(extra_args);
        let mut child = Command::new(LLVM_MC)
            .args(&args)
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

    fn llvm_mc_word(asm: &str) -> Result<u32, String> {
        llvm_mc_word_with(asm, &[])
    }

    fn llvm_mc_fp16_word(asm: &str) -> Result<u32, String> {
        llvm_mc_word_with(asm, &["-mattr=+fullfp16"])
    }

    fn extra_operand() -> impl Strategy<Value = Operand> {
        prop_oneof![
            (0u32..=31).prop_map(|n| Operand::Reg(format!("s{n}"))),
            Just(Operand::Imm(0)),
            Just(Operand::Imm(1)),
            Just(Operand::Imm(32)),
            Just(Operand::Shift {
                kind: "lsl".into(),
                amount: 0,
            }),
            Just(Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "4s".into(),
            }),
        ]
    }

    fn fp_spelling(is_d: bool, n: u32, kind: u32) -> String {
        match kind {
            0 => fp(is_d, n).to_uppercase(),
            _ => fp(is_d, n),
        }
    }

    fn wrong_type_pair() -> impl Strategy<Value = (String, String)> {
        let n = 0u32..=31;
        prop_oneof![
            // mixed S/D
            (n.clone(), n.clone(), any::<bool>()).prop_map(|(d, s, dest_d)| {
                (fp(dest_d, d), fp(!dest_d, s))
            }),
            // GPR dest + FP src
            (n.clone(), n.clone(), any::<bool>(), any::<bool>()).prop_map(|(d, s, d64, src_d)| {
                (gpr(d64, d), fp(src_d, s))
            }),
            // FP dest + GPR src
            (n.clone(), n.clone(), any::<bool>(), any::<bool>()).prop_map(|(d, s, dest_d, s64)| {
                (fp(dest_d, d), gpr(s64, s))
            }),
            // Q/V/B dest + S/D src
            (n.clone(), n.clone(), 0u32..=2, any::<bool>()).prop_map(|(d, s, p, src_d)| {
                let pref = ["q", "v", "b"][p as usize];
                (format!("{pref}{d}"), fp(src_d, s))
            }),
            // S/D dest + Q/V/B src
            (n.clone(), n.clone(), any::<bool>(), 0u32..=2).prop_map(|(d, s, dest_d, p)| {
                let pref = ["q", "v", "b"][p as usize];
                (fp(dest_d, d), format!("{pref}{s}"))
            }),
            // SP/WSP dest + FP src
            (n.clone(), any::<bool>(), any::<bool>()).prop_map(|(s, is_64, src_d)| {
                let sp = if is_64 { "sp" } else { "wsp" };
                (sp.to_string(), fp(src_d, s))
            }),
        ]
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_fp_1src_kat_llvm_mc_frintn_s0_s1() {
        let want = 0x1e244020u32;
        let mc = llvm_mc_word("frintn s0, s1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [Operand::Reg("s0".into()), Operand::Reg("s1".into())];
        let sut = sut_word(&ops, 0b001000).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_fp_1src_kat_llvm_mc_frintn_d0_d1() {
        let want = 0x1e644020u32;
        let mc = llvm_mc_word("frintn d0, d1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [Operand::Reg("d0".into()), Operand::Reg("d1".into())];
        let sut = sut_word(&ops, 0b001000).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_fp_1src_kat_llvm_mc_frintp_s0_s1() {
        let want = 0x1e24c020u32;
        let mc = llvm_mc_word("frintp s0, s1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [Operand::Reg("s0".into()), Operand::Reg("s1".into())];
        let sut = sut_word(&ops, 0b001001).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_fp_1src_kat_llvm_mc_frintz_s0_s1() {
        let want = 0x1e25c020u32;
        let mc = llvm_mc_word("frintz s0, s1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [Operand::Reg("s0".into()), Operand::Reg("s1".into())];
        let sut = sut_word(&ops, 0b001011).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_fp_1src_kat_llvm_mc_frinti_s0_s1() {
        let want = 0x1e27c020u32;
        let mc = llvm_mc_word("frinti s0, s1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [Operand::Reg("s0".into()), Operand::Reg("s1".into())];
        let sut = sut_word(&ops, 0b001111).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_fp_1src_kat_llvm_mc_frintn_s31_s31() {
        let want = 0x1e2443ffu32;
        let mc = llvm_mc_word("frintn s31, s31").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [Operand::Reg("s31".into()), Operand::Reg("s31".into())];
        let sut = sut_word(&ops, 0b001000).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_fp_1src_kat_llvm_mc_half_frintn_h0_h1() {
        let want = 0x1ee44020u32;
        let mc = llvm_mc_fp16_word("frintn h0, h1").expect("llvm-mc fp16 KAT");
        assert_eq!(mc, want, "llvm-mc fp16 KAT mapping broken");
    }

    #[test]
    fn test_encode_fp_1src_regression_extra_operand() {
        let ops = [
            Operand::Reg("s0".into()),
            Operand::Reg("s1".into()),
            Operand::Reg("s0".into()),
        ];
        assert!(
            encode_fp_1src(&ops, 0b001000).is_err(),
            "FRINT* must reject a 3rd operand"
        );
    }

    #[test]
    fn test_encode_fp_1src_regression_mixed_sd() {
        let ops = [Operand::Reg("s0".into()), Operand::Reg("d0".into())];
        assert!(
            encode_fp_1src(&ops, 0b001000).is_err(),
            "FRINT* must reject mixed S/D operands"
        );
    }

    #[test]
    fn test_encode_fp_1src_regression_half_ftype() {
        let ops = [Operand::Reg("h0".into()), Operand::Reg("h0".into())];
        let sut = sut_word(&ops, 0b001000).expect("H,H is a valid fp16 FRINTN");
        assert_eq!(
            sut, 0x1ee44000u32,
            "H registers must use ftype=11 (0x1ee44000), not ftype=00 S"
        );
    }

    proptest! {
        #![proptest_config(cfg())]

        #[test]
        fn encode_fp_1src_diff_valid(
            rd in 0u32..=31,
            rn in 0u32..=31,
            is_d in any::<bool>(),
            idx in 0usize..FRINT.len(),
            dest_kind in 0u32..=1,
            src_kind in 0u32..=1,
        ) {
            let (opcode, mnem) = FRINT[idx];
            let dest = fp_spelling(is_d, rd, dest_kind);
            let src = fp_spelling(is_d, rn, src_kind);
            let asm = format!("{mnem} {dest}, {src}");
            let ops = [Operand::Reg(dest.clone()), Operand::Reg(src.clone())];
            let mc = llvm_mc_word(&asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid {asm}: {e}"));
            let sut = sut_word(&ops, opcode)
                .unwrap_or_else(|e| panic!("SUT rejected valid {asm}: {e}"));
            prop_assert_eq!(sut, mc, "FRINT* mismatch for {}", asm);
        }

        #[test]
        fn encode_fp_1src_arm_fields(
            rd in 0u32..=31,
            rn in 0u32..=31,
            is_d in any::<bool>(),
            idx in 0usize..FRINT.len(),
        ) {
            let (opcode, _) = FRINT[idx];
            let ops = [
                Operand::Reg(fp(is_d, rd)),
                Operand::Reg(fp(is_d, rn)),
            ];
            let w = sut_word(&ops, opcode).expect("SUT");
            let ftype = if is_d { 0b01u32 } else { 0b00 };
            let want = (0b00011110u32 << 24)
                | (ftype << 22)
                | (1 << 21)
                | (opcode << 15)
                | (0b10000 << 10)
                | (rn << 5)
                | rd;
            prop_assert_eq!(w, want, "ARM ARM FP 1-source field layout");
            prop_assert_eq!((w >> 24) & 0xff, 0b00011110, "bits[31:24] M=0 S=0 11110");
            prop_assert_eq!((w >> 22) & 0b11, ftype, "ftype");
            prop_assert_eq!((w >> 21) & 1, 1, "bit21 must be 1");
            prop_assert_eq!((w >> 15) & 0x3f, opcode, "opcode bits[20:15]");
            prop_assert_eq!((w >> 10) & 0x1f, 0b10000, "bits[14:10] must be 10000");
            prop_assert_eq!((w >> 5) & 0x1f, rn, "Rn");
            prop_assert_eq!(w & 0x1f, rd, "Rd");
        }

        #[test]
        fn encode_fp_1src_metamorphic_fields(
            rd in 0u32..=30,
            rn in 0u32..=30,
            is_d in any::<bool>(),
        ) {
            let ops = |d: u32, n: u32, sd: bool| {
                [
                    Operand::Reg(fp(sd, d)),
                    Operand::Reg(fp(sd, n)),
                ]
            };
            let w = sut_word(&ops(rd, rn, is_d), 0b001000).expect("base FRINTN");
            let w_rd = sut_word(&ops(rd + 1, rn, is_d), 0b001000).expect("Rd+1");
            let w_rn = sut_word(&ops(rd, rn + 1, is_d), 0b001000).expect("Rn+1");
            let w_ft = sut_word(&ops(rd, rn, !is_d), 0b001000).expect("ftype flip");
            let w_p = sut_word(&ops(rd, rn, is_d), 0b001001).expect("FRINTP");
            prop_assert_eq!(w_rd, w + 1, "Rd+1 must increment bits[4:0] only");
            prop_assert_eq!(w_rn, w + (1 << 5), "Rn+1 must increment bits[9:5] only");
            prop_assert_eq!(w_ft ^ w, 1u32 << 22, "S vs D must flip only ftype bit 22");
            prop_assert_eq!(w_p ^ w, 1u32 << 15, "FRINTN XOR FRINTP must be opcode LSB bit 15");
        }

        #[test]
        fn encode_fp_1src_neg_arity(
            len in 0usize..=1,
            n in 0u32..=31,
        ) {
            let mut ops = Vec::new();
            if len >= 1 {
                ops.push(Operand::Reg(fp(false, n)));
            }
            prop_assert!(
                encode_fp_1src(&ops, 0b001000).is_err(),
                "FRINT* with {} operands must Err (llvm-mc: too few operands)",
                len
            );
        }

        #[test]
        fn encode_fp_1src_neg_extra_operand(
            rd in 0u32..=31,
            rn in 0u32..=31,
            is_d in any::<bool>(),
            extra in extra_operand(),
        ) {
            let ops = vec![
                Operand::Reg(fp(is_d, rd)),
                Operand::Reg(fp(is_d, rn)),
                extra,
            ];
            prop_assert!(
                encode_fp_1src(&ops, 0b001000).is_err(),
                "FRINT* has no 3rd operand; extra must Err"
            );
        }

        #[test]
        fn encode_fp_1src_neg_wrong_types(
            (dest, src) in wrong_type_pair(),
        ) {
            let ops = [Operand::Reg(dest.clone()), Operand::Reg(src.clone())];
            prop_assert!(
                encode_fp_1src(&ops, 0b001000).is_err(),
                "FRINT* requires matching Sd,Sn or Dd,Dn; dest={} src={} must Err",
                dest,
                src
            );
        }

        #[test]
        fn encode_fp_1src_diff_half(
            rd in 0u32..=31,
            rn in 0u32..=31,
            idx in 0usize..FRINT.len(),
        ) {
            let (opcode, mnem) = FRINT[idx];
            let dest = format!("h{rd}");
            let src = format!("h{rn}");
            let asm = format!("{mnem} {dest}, {src}");
            let ops = [Operand::Reg(dest.clone()), Operand::Reg(src)];
            let mc = llvm_mc_fp16_word(&asm)
                .unwrap_or_else(|e| panic!("llvm-mc fp16 rejected valid {asm}: {e}"));
            let sut = sut_word(&ops, opcode)
                .unwrap_or_else(|e| panic!("SUT rejected valid fp16 {asm}: {e}"));
            prop_assert_eq!(sut, mc, "FRINT* half-precision mismatch for {}", asm);
        }

        #[test]
        fn encode_fp_1src_neg_nonreg(
            which in 0u32..=1,
            kind in 0u32..=5,
        ) {
            let bad = match kind {
                0 => Operand::Imm(0),
                1 => Operand::Symbol("foo".into()),
                2 => Operand::Label("1f".into()),
                3 => Operand::Mem {
                    base: "x0".into(),
                    offset: 0,
                },
                4 => Operand::Cond("eq".into()),
                _ => Operand::Shift {
                    kind: "lsl".into(),
                    amount: 0,
                },
            };
            let mut ops = vec![
                Operand::Reg("s0".into()),
                Operand::Reg("s1".into()),
            ];
            ops[which as usize] = bad;
            prop_assert!(
                encode_fp_1src(&ops, 0b001000).is_err(),
                "non-register at slot {} must Err",
                which
            );
        }

        #[test]
        fn encode_fp_1src_neg_invalid_name(
            which in 0u32..=1,
            name in prop::sample::select(vec![
                "foo", "s32", "d32", "h32", "x32", "r0", "s", "d", "",
            ]),
        ) {
            let mut ops = vec![
                Operand::Reg("s0".into()),
                Operand::Reg("s1".into()),
            ];
            ops[which as usize] = Operand::Reg(name.to_string());
            prop_assert!(
                encode_fp_1src(&ops, 0b001000).is_err(),
                "invalid name {:?} at slot {} must Err",
                name,
                which
            );
        }
    }
}

#[cfg(test)]
mod encode_int_to_float_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler
    // Evidence: src/backend/arm/assembler/README.md "accepts the same textual assembly that GCC's gas would consume";
    //   encoder/mod.rs "Encodes AArch64 instructions into 32-bit machine code words";
    //   encoder/mod.rs:454-459 scvtf/ucvtf dispatch (vector RegArrangement goes to encode_neon_float_two_misc);
    //   ARM ARM Conversion between floating-point and integer:
    //   sf 00 11110 ftype 1 00 opcode 000000 Rn Rd;
    //   fp_scalar.rs:211-215 purpose comment (sf W/X source, ftype S/D dest, opcode 010 signed / 011 unsigned);
    //   README.md:223 lists scalar ucvtf/scvtf; codegen/cast_ops.rs:53-66 emits scvtf/ucvtf.
    // Stronger considered:
    //   - State machine: rejected — encode_int_to_float is a pure function with no lifecycle
    //   - Algebraic round-trip: rejected — no in-tree SCVTF/UCVTF integer decoder
    //   - encode_fcvt_rounding as differential sibling:
    //     rejected — same-job gate fails (float-to-integer, opposite conversion)
    //   - encode_scvtf / encode_ucvtf: rejected — thin wrappers that call this function
    //   - encode_neon_float_two_misc: rejected — vector/SIMD-scalar form (scvtf s0,s1 = 0x5e21d820)
    //   - encode_fcvt_precision: rejected — float-to-float precision conversion
    // Weaker available: algebraic.metamorphic (sf/ftype/opcode/Rd/Rn),
    //   algebraic.invariant (ARM fields), negative_error (arity / extra / SP / wrong type)
    // Differential: candidate=encode_int_to_float, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler (caller-reachable from encode_instruction),
    //   mapping=[Reg(Sd|Dd), Reg(Wn|Xn)]+is_signed <-> `scvtf|ucvtf Sd|Dd, Wn|Xn`

    use super::encode_int_to_float;
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

    fn fp(is_d: bool, n: u32) -> String {
        format!("{}{}", if is_d { "d" } else { "s" }, n)
    }

    fn mnem(is_signed: bool) -> &'static str {
        if is_signed {
            "scvtf"
        } else {
            "ucvtf"
        }
    }

    fn sut_word(ops: &[Operand], is_signed: bool) -> Result<u32, String> {
        match encode_int_to_float(ops, is_signed)? {
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

    fn llvm_mc_word_with(asm: &str, extra_args: &[&str]) -> Result<u32, String> {
        let mut args = vec!["-triple=aarch64", "-show-encoding"];
        args.extend_from_slice(extra_args);
        let mut child = Command::new(LLVM_MC)
            .args(&args)
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

    fn llvm_mc_word(asm: &str) -> Result<u32, String> {
        llvm_mc_word_with(asm, &[])
    }

    fn llvm_mc_fp16_word(asm: &str) -> Result<u32, String> {
        llvm_mc_word_with(asm, &["-mattr=+fullfp16"])
    }

    fn extra_operand() -> impl Strategy<Value = Operand> {
        prop_oneof![
            (0u32..=31).prop_map(|n| Operand::Reg(format!("x{n}"))),
            Just(Operand::Imm(0)),
            Just(Operand::Imm(1)),
            Just(Operand::Imm(8)),
            Just(Operand::Imm(32)),
            Just(Operand::Shift {
                kind: "lsl".into(),
                amount: 0,
            }),
            Just(Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "4s".into(),
            }),
        ]
    }

    fn dest_spelling(is_d: bool, n: u32, kind: u32) -> String {
        match kind {
            0 => fp(is_d, n).to_uppercase(),
            _ => fp(is_d, n),
        }
    }

    fn src_spelling(is_64: bool, n: u32, kind: u32) -> String {
        match kind {
            0 if n == 31 && is_64 => "x31".into(),
            0 if n == 31 && !is_64 => "w31".into(),
            1 if n == 31 && is_64 => "XZR".into(),
            1 if n == 31 && !is_64 => "WZR".into(),
            2 if n == 30 && is_64 => "LR".into(),
            3 if n == 30 && is_64 => "lr".into(),
            4 => gpr(is_64, n).to_uppercase(),
            _ => gpr(is_64, n),
        }
    }

    fn wrong_type_pair() -> impl Strategy<Value = (String, String)> {
        let n = 0u32..=31;
        prop_oneof![
            // GP dest + GP source
            (n.clone(), n.clone(), any::<bool>(), any::<bool>()).prop_map(|(d, s, d64, s64)| {
                (gpr(d64, d), gpr(s64, s))
            }),
            // GP dest + FP (S/D) source
            (n.clone(), n.clone(), any::<bool>(), any::<bool>()).prop_map(|(d, s, d64, src_d)| {
                (gpr(d64, d), fp(src_d, s))
            }),
            // FP dest + FP source (SIMD-scalar SCVTF, not integer form)
            (n.clone(), n.clone(), any::<bool>(), any::<bool>()).prop_map(|(d, s, dest_d, src_d)| {
                (fp(dest_d, d), fp(src_d, s))
            }),
            // Q/V/B dest + GP source
            (n.clone(), n.clone(), 0u32..=2, any::<bool>()).prop_map(|(d, s, p, s64)| {
                let pref = ["q", "v", "b"][p as usize];
                (format!("{pref}{d}"), gpr(s64, s))
            }),
            // S/D dest + Q/V/B source
            (n.clone(), n.clone(), any::<bool>(), 0u32..=2).prop_map(|(d, s, dest_d, p)| {
                let pref = ["q", "v", "b"][p as usize];
                (fp(dest_d, d), format!("{pref}{s}"))
            }),
            // S/D dest + H source
            (n.clone(), n.clone(), any::<bool>()).prop_map(|(d, s, dest_d)| {
                (fp(dest_d, d), format!("h{s}"))
            }),
        ]
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_int_to_float_kat_llvm_mc_scvtf_s0_w1() {
        let want = 0x1e220020u32;
        let mc = llvm_mc_word("scvtf s0, w1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [Operand::Reg("s0".into()), Operand::Reg("w1".into())];
        let sut = sut_word(&ops, true).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_int_to_float_kat_llvm_mc_scvtf_d0_x1() {
        let want = 0x9e620020u32;
        let mc = llvm_mc_word("scvtf d0, x1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [Operand::Reg("d0".into()), Operand::Reg("x1".into())];
        let sut = sut_word(&ops, true).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_int_to_float_kat_llvm_mc_ucvtf_s0_w1() {
        let want = 0x1e230020u32;
        let mc = llvm_mc_word("ucvtf s0, w1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [Operand::Reg("s0".into()), Operand::Reg("w1".into())];
        let sut = sut_word(&ops, false).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_int_to_float_kat_llvm_mc_ucvtf_d0_x1() {
        let want = 0x9e630020u32;
        let mc = llvm_mc_word("ucvtf d0, x1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [Operand::Reg("d0".into()), Operand::Reg("x1".into())];
        let sut = sut_word(&ops, false).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_int_to_float_kat_llvm_mc_scvtf_s0_x1() {
        let want = 0x9e220020u32;
        let mc = llvm_mc_word("scvtf s0, x1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [Operand::Reg("s0".into()), Operand::Reg("x1".into())];
        let sut = sut_word(&ops, true).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_int_to_float_kat_llvm_mc_scvtf_d31_xzr() {
        let want = 0x9e6203ffu32;
        let mc = llvm_mc_word("scvtf d31, xzr").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [Operand::Reg("d31".into()), Operand::Reg("xzr".into())];
        let sut = sut_word(&ops, true).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_int_to_float_kat_llvm_mc_scvtf_d0_lr() {
        let want = 0x9e6203c0u32;
        let mc = llvm_mc_word("scvtf d0, lr").expect("llvm-mc LR KAT");
        assert_eq!(mc, want, "llvm-mc LR KAT mapping broken");
        let ops = [Operand::Reg("d0".into()), Operand::Reg("lr".into())];
        let sut = sut_word(&ops, true).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_int_to_float_kat_llvm_mc_half_scvtf_h0_w1() {
        let want = 0x1ee20020u32;
        let mc = llvm_mc_fp16_word("scvtf h0, w1").expect("llvm-mc fp16 KAT");
        assert_eq!(mc, want, "llvm-mc fp16 KAT mapping broken");
    }

    #[test]
    fn test_encode_int_to_float_regression_extra_operand() {
        let ops = [
            Operand::Reg("s0".into()),
            Operand::Reg("w1".into()),
            Operand::Imm(8),
        ];
        assert!(
            encode_int_to_float(&ops, true).is_err(),
            "integer SCVTF must reject a 3rd #fbits operand (fixed-point is a different encoding)"
        );
    }

    #[test]
    fn test_encode_int_to_float_regression_sp_src() {
        let ops = [Operand::Reg("s0".into()), Operand::Reg("sp".into())];
        assert!(
            encode_int_to_float(&ops, true).is_err(),
            "SP/WSP is not a valid integer SCVTF source"
        );
    }

    #[test]
    fn test_encode_int_to_float_regression_fp_src() {
        let ops = [Operand::Reg("s0".into()), Operand::Reg("s1".into())];
        assert!(
            encode_int_to_float(&ops, true).is_err(),
            "FP source is SIMD-scalar SCVTF, not integer conversion"
        );
    }

    #[test]
    fn test_encode_int_to_float_regression_half_ftype() {
        let ops = [Operand::Reg("h0".into()), Operand::Reg("w1".into())];
        let sut = sut_word(&ops, true).expect("H dest is a valid fp16 integer SCVTF");
        assert_eq!(
            sut, 0x1ee20020u32,
            "H dest must use ftype=11 (0x1ee20020), not ftype=00 S"
        );
    }

    proptest! {
        #![proptest_config(cfg())]

        #[test]
        fn encode_int_to_float_diff_valid(
            rd in 0u32..=31,
            rn in 0u32..=31,
            dest_d in any::<bool>(),
            src64 in any::<bool>(),
            is_signed in any::<bool>(),
            dest_kind in 0u32..=1,
            src_kind in 0u32..=5,
        ) {
            let dest = dest_spelling(dest_d, rd, dest_kind);
            let src = src_spelling(src64, rn, src_kind);
            let asm = format!("{} {}, {}", mnem(is_signed), dest, src);
            let ops = [Operand::Reg(dest.clone()), Operand::Reg(src.clone())];
            let mc = llvm_mc_word(&asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid {asm}: {e}"));
            let sut = sut_word(&ops, is_signed)
                .unwrap_or_else(|e| panic!("SUT rejected valid {asm}: {e}"));
            prop_assert_eq!(sut, mc, "SCVTF/UCVTF mismatch for {}", asm);
        }

        #[test]
        fn encode_int_to_float_arm_fields(
            rd in 0u32..=31,
            rn in 0u32..=31,
            dest_d in any::<bool>(),
            src64 in any::<bool>(),
            is_signed in any::<bool>(),
        ) {
            let ops = [
                Operand::Reg(fp(dest_d, rd)),
                Operand::Reg(gpr(src64, rn)),
            ];
            let w = sut_word(&ops, is_signed).expect("SUT");
            let sf = if src64 { 1u32 } else { 0 };
            let ftype = if dest_d { 0b01u32 } else { 0b00 };
            let opcode = if is_signed { 0b010u32 } else { 0b011 };
            let want = (sf << 31)
                | (0b11110 << 24)
                | (ftype << 22)
                | (1 << 21)
                | (opcode << 16)
                | (rn << 5)
                | rd;
            prop_assert_eq!(w, want, "ARM ARM SCVTF/UCVTF integer field layout");
            prop_assert_eq!((w >> 31) & 1, sf, "sf");
            prop_assert_eq!((w >> 29) & 0b11, 0, "bits[30:29] must be 00");
            prop_assert_eq!((w >> 24) & 0b11111, 0b11110, "bits[28:24]");
            prop_assert_eq!((w >> 22) & 0b11, ftype, "ftype");
            prop_assert_eq!((w >> 21) & 1, 1, "bit21 must be 1 (integer, not fixed-point)");
            prop_assert_eq!((w >> 19) & 0b11, 0, "rmode must be 00");
            prop_assert_eq!((w >> 16) & 0b111, opcode, "opcode");
            prop_assert_eq!((w >> 10) & 0b111111, 0, "scale/bits[15:10] must be 0");
            prop_assert_eq!((w >> 5) & 0x1f, rn, "Rn");
            prop_assert_eq!(w & 0x1f, rd, "Rd");
        }

        #[test]
        fn encode_int_to_float_metamorphic_fields(
            rd in 0u32..=30,
            rn in 0u32..=30,
            dest_d in any::<bool>(),
            src64 in any::<bool>(),
        ) {
            let ops = |
                d: u32,
                n: u32,
                dd: bool,
                s64: bool,
            | {
                [
                    Operand::Reg(fp(dd, d)),
                    Operand::Reg(gpr(s64, n)),
                ]
            };
            let w = sut_word(&ops(rd, rn, dest_d, src64), true).expect("base SCVTF");
            let w_rd = sut_word(&ops(rd + 1, rn, dest_d, src64), true).expect("Rd+1");
            let w_rn = sut_word(&ops(rd, rn + 1, dest_d, src64), true).expect("Rn+1");
            let w_sf = sut_word(&ops(rd, rn, dest_d, !src64), true).expect("sf flip");
            let w_ft = sut_word(&ops(rd, rn, !dest_d, src64), true).expect("ftype flip");
            let w_u = sut_word(&ops(rd, rn, dest_d, src64), false).expect("UCVTF");
            prop_assert_eq!(w_rd, w + 1, "Rd+1 must increment bits[4:0] only");
            prop_assert_eq!(w_rn, w + (1 << 5), "Rn+1 must increment bits[9:5] only");
            prop_assert_eq!(w_sf ^ w, 1u32 << 31, "W vs X source must flip only sf bit 31");
            prop_assert_eq!(w_ft ^ w, 1u32 << 22, "S vs D dest must flip only ftype bit 22");
            prop_assert_eq!(w_u ^ w, 1u32 << 16, "SCVTF XOR UCVTF must be opcode LSB bit 16");
        }

        #[test]
        fn encode_int_to_float_neg_arity(
            len in 0usize..=1,
            n in 0u32..=31,
        ) {
            let mut ops = Vec::new();
            if len >= 1 {
                ops.push(Operand::Reg(fp(false, n)));
            }
            prop_assert!(
                encode_int_to_float(&ops, true).is_err(),
                "SCVTF/UCVTF integer form with {} operands must Err (llvm-mc: too few operands)",
                len
            );
        }

        #[test]
        fn encode_int_to_float_neg_extra_operand(
            rd in 0u32..=31,
            rn in 0u32..=31,
            dest_d in any::<bool>(),
            src64 in any::<bool>(),
            extra in extra_operand(),
        ) {
            let ops = vec![
                Operand::Reg(fp(dest_d, rd)),
                Operand::Reg(gpr(src64, rn)),
                extra,
            ];
            prop_assert!(
                encode_int_to_float(&ops, true).is_err(),
                "integer SCVTF/UCVTF has no 3rd operand; extra must Err (fixed-point is a different encoding)"
            );
        }

        #[test]
        fn encode_int_to_float_neg_sp_src(
            is_64 in any::<bool>(),
            rd in 0u32..=31,
            dest_d in any::<bool>(),
        ) {
            let sp = if is_64 { "sp" } else { "wsp" };
            let ops = [
                Operand::Reg(fp(dest_d, rd)),
                Operand::Reg(sp.into()),
            ];
            prop_assert!(
                encode_int_to_float(&ops, true).is_err(),
                "SP/WSP is not a valid SCVTF/UCVTF integer source (sp={})",
                sp
            );
        }

        #[test]
        fn encode_int_to_float_neg_wrong_types(
            (dest, src) in wrong_type_pair(),
        ) {
            let ops = [Operand::Reg(dest.clone()), Operand::Reg(src.clone())];
            prop_assert!(
                encode_int_to_float(&ops, true).is_err(),
                "integer SCVTF/UCVTF requires Sd|Dd, Wn|Xn; dest={} src={} must Err",
                dest,
                src
            );
        }

        #[test]
        fn encode_int_to_float_diff_half(
            rd in 0u32..=31,
            rn in 0u32..=31,
            src64 in any::<bool>(),
            is_signed in any::<bool>(),
        ) {
            let dest = format!("h{rd}");
            let src = gpr(src64, rn);
            let asm = format!("{} {}, {}", mnem(is_signed), dest, src);
            let ops = [Operand::Reg(dest.clone()), Operand::Reg(src)];
            let mc = llvm_mc_fp16_word(&asm)
                .unwrap_or_else(|e| panic!("llvm-mc fp16 rejected valid {asm}: {e}"));
            let sut = sut_word(&ops, is_signed)
                .unwrap_or_else(|e| panic!("SUT rejected valid fp16 {asm}: {e}"));
            prop_assert_eq!(sut, mc, "SCVTF/UCVTF half-precision mismatch for {}", asm);
        }

        #[test]
        fn encode_int_to_float_neg_nonreg(
            which in 0u32..=1,
            kind in 0u32..=5,
        ) {
            let bad = match kind {
                0 => Operand::Imm(0),
                1 => Operand::Symbol("foo".into()),
                2 => Operand::Label("1f".into()),
                3 => Operand::Mem {
                    base: "x0".into(),
                    offset: 0,
                },
                4 => Operand::Cond("eq".into()),
                _ => Operand::Shift {
                    kind: "lsl".into(),
                    amount: 0,
                },
            };
            let mut ops = vec![
                Operand::Reg("s0".into()),
                Operand::Reg("w0".into()),
            ];
            ops[which as usize] = bad;
            prop_assert!(
                encode_int_to_float(&ops, true).is_err(),
                "non-register at slot {} must Err",
                which
            );
        }

        #[test]
        fn encode_int_to_float_neg_invalid_name(
            which in 0u32..=1,
            name in prop::sample::select(vec![
                "foo", "x32", "w32", "s32", "d32", "r0", "x", "s", "",
            ]),
        ) {
            let mut ops = vec![
                Operand::Reg("s0".into()),
                Operand::Reg("w0".into()),
            ];
            ops[which as usize] = Operand::Reg(name.to_string());
            prop_assert!(
                encode_int_to_float(&ops, true).is_err(),
                "invalid name {:?} at slot {} must Err",
                name,
                which
            );
        }
    }
}

#[cfg(test)]
mod encode_fcmp_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler
    // Evidence: src/backend/arm/assembler/README.md "accepts the same textual assembly that GCC's gas would consume";
    //   encoder/mod.rs "Encodes AArch64 instructions into 32-bit machine code words";
    //   encoder/mod.rs:439 "fcmp" => encode_fcmp;
    //   ARM ARM Floating-point compare:
    //   0 00 11110 ftype 1 Rm 001000 Rn opc;
    //   opc 00000 = FCMP register, 01000 = FCMP #0.0;
    //   ftype 00=S 01=D 11=H;
    //   fp_scalar.rs:164-171 purpose comment (FCMP Dn, #0.0 / Dn, Dm);
    //   README.md:223 lists scalar fcmp;
    //   codegen/comparison.rs:15-19 emits fcmp s0, s1 / fcmp d0, d1.
    // Stronger considered:
    //   - State machine: rejected — encode_fcmp is a pure function with no lifecycle
    //   - Algebraic round-trip: rejected — no in-tree FCMP decoder
    //   - encode_fp_arith as differential sibling: rejected — same-job gate fails (3-operand FP arith)
    //   - encode_neon_float_cmp_zero: rejected — vector/SIMD compare-to-zero
    //   - fccmp: rejected — different mnemonic (NZCV/cond)
    // Weaker available: algebraic.metamorphic (ftype/Rn/Rm/opc),
    //   algebraic.invariant (ARM fields), negative_error (arity / extra / wrong type)
    // Differential: candidate=encode_fcmp, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler (caller-reachable from encode_instruction),
    //   mapping=[Reg(Sn|Dn), Reg(Sm|Dm)] <-> `fcmp Sn|Dn, Sm|Dm`;
    //            [Reg(Sn|Dn), Imm(0)] <-> `fcmp Sn|Dn, #0.0`

    use super::encode_fcmp;
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

    fn fp(is_d: bool, n: u32) -> String {
        format!("{}{}", if is_d { "d" } else { "s" }, n)
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
        match encode_fcmp(ops)? {
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

    fn llvm_mc_word_with(asm: &str, extra_args: &[&str]) -> Result<u32, String> {
        let mut args = vec!["-triple=aarch64", "-show-encoding"];
        args.extend_from_slice(extra_args);
        let mut child = Command::new(LLVM_MC)
            .args(&args)
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

    fn llvm_mc_word(asm: &str) -> Result<u32, String> {
        llvm_mc_word_with(asm, &[])
    }

    fn llvm_mc_fp16_word(asm: &str) -> Result<u32, String> {
        llvm_mc_word_with(asm, &["-mattr=+fullfp16"])
    }

    fn extra_operand() -> impl Strategy<Value = Operand> {
        prop_oneof![
            (0u32..=31).prop_map(|n| Operand::Reg(format!("s{n}"))),
            Just(Operand::Imm(0)),
            Just(Operand::Imm(1)),
            Just(Operand::Shift {
                kind: "lsl".into(),
                amount: 0,
            }),
            Just(Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "4s".into(),
            }),
        ]
    }

    fn fp_spelling(is_d: bool, n: u32, kind: u32) -> String {
        match kind {
            0 => fp(is_d, n).to_uppercase(),
            _ => fp(is_d, n),
        }
    }

    fn wrong_type_pair() -> impl Strategy<Value = (String, String)> {
        let n = 0u32..=31;
        prop_oneof![
            // mixed S/D
            (n.clone(), n.clone(), any::<bool>()).prop_map(|(a, b, a_d)| {
                (fp(a_d, a), fp(!a_d, b))
            }),
            // GPR, GPR
            (n.clone(), n.clone(), any::<bool>(), any::<bool>()).prop_map(|(a, b, a64, b64)| {
                (gpr(a64, a), gpr(b64, b))
            }),
            // FP + GPR
            (n.clone(), n.clone(), any::<bool>(), any::<bool>()).prop_map(|(a, b, a_d, b64)| {
                (fp(a_d, a), gpr(b64, b))
            }),
            // GPR + FP
            (n.clone(), n.clone(), any::<bool>(), any::<bool>()).prop_map(|(a, b, a64, b_d)| {
                (gpr(a64, a), fp(b_d, b))
            }),
            // Q/V/B + S/D
            (n.clone(), n.clone(), 0u32..=2, any::<bool>()).prop_map(|(a, b, p, b_d)| {
                let pref = ["q", "v", "b"][p as usize];
                (format!("{pref}{a}"), fp(b_d, b))
            }),
            // S/D + Q/V/B
            (n.clone(), n.clone(), any::<bool>(), 0u32..=2).prop_map(|(a, b, a_d, p)| {
                let pref = ["q", "v", "b"][p as usize];
                (fp(a_d, a), format!("{pref}{b}"))
            }),
            // SP/WSP as first
            (n.clone(), any::<bool>(), any::<bool>()).prop_map(|(b, is_64, b_d)| {
                let sp = if is_64 { "sp" } else { "wsp" };
                (sp.to_string(), fp(b_d, b))
            }),
            // SP/WSP as second
            (n.clone(), any::<bool>(), any::<bool>()).prop_map(|(a, is_64, a_d)| {
                let sp = if is_64 { "sp" } else { "wsp" };
                (fp(a_d, a), sp.to_string())
            }),
        ]
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_fcmp_kat_llvm_mc_s0_s1() {
        let want = 0x1e212000u32;
        let mc = llvm_mc_word("fcmp s0, s1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [Operand::Reg("s0".into()), Operand::Reg("s1".into())];
        let sut = sut_word(&ops).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_fcmp_kat_llvm_mc_d0_d1() {
        let want = 0x1e612000u32;
        let mc = llvm_mc_word("fcmp d0, d1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [Operand::Reg("d0".into()), Operand::Reg("d1".into())];
        let sut = sut_word(&ops).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_fcmp_kat_llvm_mc_s0_zero() {
        let want = 0x1e202008u32;
        let mc = llvm_mc_word("fcmp s0, #0.0").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [Operand::Reg("s0".into()), Operand::Imm(0)];
        let sut = sut_word(&ops).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_fcmp_kat_llvm_mc_d0_zero() {
        let want = 0x1e602008u32;
        let mc = llvm_mc_word("fcmp d0, #0.0").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [Operand::Reg("d0".into()), Operand::Imm(0)];
        let sut = sut_word(&ops).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_fcmp_kat_llvm_mc_s31_s31() {
        let want = 0x1e3f23e0u32;
        let mc = llvm_mc_word("fcmp s31, s31").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [Operand::Reg("s31".into()), Operand::Reg("s31".into())];
        let sut = sut_word(&ops).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_fcmp_kat_llvm_mc_d31_d0() {
        let want = 0x1e6023e0u32;
        let mc = llvm_mc_word("fcmp d31, d0").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [Operand::Reg("d31".into()), Operand::Reg("d0".into())];
        let sut = sut_word(&ops).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_fcmp_kat_llvm_mc_half_h0_h1() {
        let want = 0x1ee12000u32;
        let mc = llvm_mc_fp16_word("fcmp h0, h1").expect("llvm-mc fp16 KAT");
        assert_eq!(mc, want, "llvm-mc fp16 KAT mapping broken");
    }

    #[test]
    fn encode_fcmp_kat_llvm_mc_half_h0_zero() {
        let want = 0x1ee02008u32;
        let mc = llvm_mc_fp16_word("fcmp h0, #0.0").expect("llvm-mc fp16 KAT");
        assert_eq!(mc, want, "llvm-mc fp16 KAT mapping broken");
    }

    #[test]
    fn test_encode_fcmp_regression_arity_one() {
        let ops = [Operand::Reg("s0".into())];
        assert!(
            encode_fcmp(&ops).is_err(),
            "FCMP with 1 operand must Err (llvm-mc: too few operands), not encode as #0.0"
        );
    }

    #[test]
    fn test_encode_fcmp_regression_extra_operand() {
        let ops = [
            Operand::Reg("s0".into()),
            Operand::Reg("s0".into()),
            Operand::Reg("s0".into()),
        ];
        assert!(
            encode_fcmp(&ops).is_err(),
            "FCMP must reject a 3rd operand"
        );
    }

    #[test]
    fn test_encode_fcmp_regression_mixed_sd() {
        let ops = [Operand::Reg("s0".into()), Operand::Reg("d0".into())];
        assert!(
            encode_fcmp(&ops).is_err(),
            "FCMP must reject mixed S/D operands"
        );
    }

    #[test]
    fn test_encode_fcmp_regression_half_ftype() {
        let ops = [Operand::Reg("h0".into()), Operand::Reg("h0".into())];
        let sut = sut_word(&ops).expect("H,H is a valid fp16 FCMP");
        assert_eq!(
            sut, 0x1ee02000u32,
            "H registers must use ftype=11 (0x1ee02000), not ftype=00 S"
        );
    }

    proptest! {
        #![proptest_config(cfg())]

        #[test]
        fn encode_fcmp_diff_valid(
            rn in 0u32..=31,
            rm in 0u32..=31,
            is_d in any::<bool>(),
            rn_kind in 0u32..=1,
            rm_kind in 0u32..=1,
        ) {
            let a = fp_spelling(is_d, rn, rn_kind);
            let b = fp_spelling(is_d, rm, rm_kind);
            let asm = format!("fcmp {a}, {b}");
            let ops = [Operand::Reg(a.clone()), Operand::Reg(b.clone())];
            let mc = llvm_mc_word(&asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid {asm}: {e}"));
            let sut = sut_word(&ops)
                .unwrap_or_else(|e| panic!("SUT rejected valid {asm}: {e}"));
            prop_assert_eq!(sut, mc, "FCMP mismatch for {}", asm);
        }

        #[test]
        fn encode_fcmp_diff_zero(
            rn in 0u32..=31,
            is_d in any::<bool>(),
            rn_kind in 0u32..=1,
        ) {
            let a = fp_spelling(is_d, rn, rn_kind);
            let asm = format!("fcmp {a}, #0.0");
            let ops = [Operand::Reg(a.clone()), Operand::Imm(0)];
            let mc = llvm_mc_word(&asm)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid {asm}: {e}"));
            let sut = sut_word(&ops)
                .unwrap_or_else(|e| panic!("SUT rejected valid {asm}: {e}"));
            prop_assert_eq!(sut, mc, "FCMP #0.0 mismatch for {}", asm);
        }

        #[test]
        fn encode_fcmp_arm_fields(
            rn in 0u32..=31,
            rm in 0u32..=31,
            is_d in any::<bool>(),
        ) {
            let ops = [
                Operand::Reg(fp(is_d, rn)),
                Operand::Reg(fp(is_d, rm)),
            ];
            let w = sut_word(&ops).expect("SUT register form");
            let ftype = if is_d { 0b01u32 } else { 0b00 };
            let want = (0b00011110u32 << 24)
                | (ftype << 22)
                | (1 << 21)
                | (rm << 16)
                | (0b001000 << 10)
                | (rn << 5);
            prop_assert_eq!(w, want, "ARM ARM FCMP register field layout");
            prop_assert_eq!((w >> 24) & 0xff, 0b00011110, "bits[31:24] M=0 S=0 11110");
            prop_assert_eq!((w >> 22) & 0b11, ftype, "ftype");
            prop_assert_eq!((w >> 21) & 1, 1, "bit21 must be 1");
            prop_assert_eq!((w >> 16) & 0x1f, rm, "Rm");
            prop_assert_eq!((w >> 10) & 0x3f, 0b001000, "bits[15:10] must be 001000");
            prop_assert_eq!((w >> 5) & 0x1f, rn, "Rn");
            prop_assert_eq!(w & 0x1f, 0, "opc must be 00000 (FCMP register)");

            let zops = [Operand::Reg(fp(is_d, rn)), Operand::Imm(0)];
            let z = sut_word(&zops).expect("SUT zero form");
            let zwant = (0b00011110u32 << 24)
                | (ftype << 22)
                | (1 << 21)
                | (0b001000 << 10)
                | (rn << 5)
                | 0b01000;
            prop_assert_eq!(z, zwant, "ARM ARM FCMP #0.0 field layout");
            prop_assert_eq!((z >> 16) & 0x1f, 0, "Rm must be 00000 for #0.0");
            prop_assert_eq!(z & 0x1f, 0b01000, "opc must be 01000 (FCMP #0.0)");
        }

        #[test]
        fn encode_fcmp_metamorphic_fields(
            rn in 0u32..=30,
            rm in 0u32..=30,
            is_d in any::<bool>(),
        ) {
            let ops = |n: u32, m: u32, sd: bool| {
                [
                    Operand::Reg(fp(sd, n)),
                    Operand::Reg(fp(sd, m)),
                ]
            };
            let w = sut_word(&ops(rn, rm, is_d)).expect("base FCMP");
            let w_rn = sut_word(&ops(rn + 1, rm, is_d)).expect("Rn+1");
            let w_rm = sut_word(&ops(rn, rm + 1, is_d)).expect("Rm+1");
            let w_ft = sut_word(&ops(rn, rm, !is_d)).expect("ftype flip");
            let w_rm0 = sut_word(&ops(rn, 0, is_d)).expect("Rm=0");
            let w_z = sut_word(&[Operand::Reg(fp(is_d, rn)), Operand::Imm(0)]).expect("#0.0");
            prop_assert_eq!(w_rn, w + (1 << 5), "Rn+1 must increment bits[9:5] only");
            prop_assert_eq!(w_rm, w + (1 << 16), "Rm+1 must increment bits[20:16] only");
            prop_assert_eq!(w_ft ^ w, 1u32 << 22, "S vs D must flip only ftype bit 22");
            prop_assert_eq!(w_z ^ w_rm0, 1u32 << 3, "register XOR #0.0 (Rm=0) must be opc bit 3");
        }

        #[test]
        fn encode_fcmp_neg_arity(
            len in 0usize..=1,
            n in 0u32..=31,
            is_d in any::<bool>(),
        ) {
            let mut ops = Vec::new();
            if len >= 1 {
                ops.push(Operand::Reg(fp(is_d, n)));
            }
            prop_assert!(
                encode_fcmp(&ops).is_err(),
                "FCMP with {} operands must Err (llvm-mc: too few operands)",
                len
            );
        }

        #[test]
        fn encode_fcmp_neg_extra_operand(
            rn in 0u32..=31,
            rm in 0u32..=31,
            is_d in any::<bool>(),
            extra in extra_operand(),
        ) {
            let ops = vec![
                Operand::Reg(fp(is_d, rn)),
                Operand::Reg(fp(is_d, rm)),
                extra,
            ];
            prop_assert!(
                encode_fcmp(&ops).is_err(),
                "FCMP has no 3rd operand; extra must Err (fccmp is a different mnemonic)"
            );
        }

        #[test]
        fn encode_fcmp_neg_wrong_types(
            (a, b) in wrong_type_pair(),
        ) {
            let ops = [Operand::Reg(a.clone()), Operand::Reg(b.clone())];
            prop_assert!(
                encode_fcmp(&ops).is_err(),
                "FCMP requires matching Sn,Sm or Dn,Dm; a={} b={} must Err",
                a,
                b
            );
        }

        #[test]
        fn encode_fcmp_diff_half(
            rn in 0u32..=31,
            rm in 0u32..=31,
            zero in any::<bool>(),
        ) {
            if zero {
                let asm = format!("fcmp h{rn}, #0.0");
                let ops = [Operand::Reg(format!("h{rn}")), Operand::Imm(0)];
                let mc = llvm_mc_fp16_word(&asm)
                    .unwrap_or_else(|e| panic!("llvm-mc fp16 rejected valid {asm}: {e}"));
                let sut = sut_word(&ops)
                    .unwrap_or_else(|e| panic!("SUT rejected valid fp16 {asm}: {e}"));
                prop_assert_eq!(sut, mc, "FCMP half-precision #0.0 mismatch for {}", asm);
            } else {
                let asm = format!("fcmp h{rn}, h{rm}");
                let ops = [Operand::Reg(format!("h{rn}")), Operand::Reg(format!("h{rm}"))];
                let mc = llvm_mc_fp16_word(&asm)
                    .unwrap_or_else(|e| panic!("llvm-mc fp16 rejected valid {asm}: {e}"));
                let sut = sut_word(&ops)
                    .unwrap_or_else(|e| panic!("SUT rejected valid fp16 {asm}: {e}"));
                prop_assert_eq!(sut, mc, "FCMP half-precision mismatch for {}", asm);
            }
        }

        #[test]
        fn encode_fcmp_neg_nonzero_imm(
            rn in 0u32..=31,
            is_d in any::<bool>(),
            imm in prop_oneof![
                Just(-1i64),
                Just(1i64),
                Just(i64::MIN),
                Just(i64::MAX),
                2i64..=32,
            ],
        ) {
            let ops = [Operand::Reg(fp(is_d, rn)), Operand::Imm(imm)];
            prop_assert!(
                encode_fcmp(&ops).is_err(),
                "FCMP immediate form allows only #0.0; Imm({}) must Err",
                imm
            );
        }

        #[test]
        fn encode_fcmp_neg_nonreg(
            which in 0u32..=1,
            kind in 0u32..=5,
        ) {
            let bad = match kind {
                0 => Operand::Imm(1),
                1 => Operand::Symbol("foo".into()),
                2 => Operand::Label("1f".into()),
                3 => Operand::Mem {
                    base: "x0".into(),
                    offset: 0,
                },
                4 => Operand::Cond("eq".into()),
                _ => Operand::Shift {
                    kind: "lsl".into(),
                    amount: 0,
                },
            };
            let mut ops = vec![
                Operand::Reg("s0".into()),
                Operand::Reg("s1".into()),
            ];
            ops[which as usize] = bad;
            prop_assert!(
                encode_fcmp(&ops).is_err(),
                "non-register at slot {} must Err",
                which
            );
        }

        #[test]
        fn encode_fcmp_neg_invalid_name(
            which in 0u32..=1,
            name in prop::sample::select(vec![
                "foo", "s32", "d32", "h32", "x32", "r0", "s", "d", "",
            ]),
        ) {
            let mut ops = vec![
                Operand::Reg("s0".into()),
                Operand::Reg("s1".into()),
            ];
            ops[which as usize] = Operand::Reg(name.to_string());
            prop_assert!(
                encode_fcmp(&ops).is_err(),
                "invalid name {:?} at slot {} must Err",
                name,
                which
            );
        }
    }
}

#[cfg(test)]
mod encode_fcvt_precision_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler
    // Evidence: src/backend/arm/assembler/README.md "accepts the same textual assembly that GCC's gas would consume";
    //   encoder/mod.rs "Encodes AArch64 instructions into 32-bit machine code words";
    //   encoder/mod.rs:460 "fcvt" => encode_fcvt_precision;
    //   ARM ARM Floating-point data-processing (1 source) FCVT:
    //   0 00 11110 ftype 1 0001 opc 10000 Rn Rd;
    //   ftype 00=S 01=D 11=H source; opc 00=S 01=D 11=H dest;
    //   ftype==opc is unallocated;
    //   fp_scalar.rs:236-239 purpose comment (FCVT Dd,Sn / Sd,Dn);
    //   README.md:223 lists scalar fcvt;
    //   codegen/cast_ops.rs:74-78 emits fcvt d0, s0 / fcvt s0, d0.
    // Stronger considered:
    //   - State machine: rejected — encode_fcvt_precision is a pure function with no lifecycle
    //   - Algebraic round-trip: rejected — no in-tree FCVT precision decoder
    //   - encode_fcvt_rounding: rejected — same-job gate fails (float-to-integer)
    //   - encode_int_to_float: rejected — integer-to-float
    //   - encode_neon_fcvtl / encode_neon_fcvtn: rejected — vector widen/narrow
    // Weaker available: algebraic.metamorphic (ftype/opc/Rn/Rd),
    //   algebraic.invariant (ARM fields), negative_error (arity / extra / same-precision / wrong type)
    // Differential: candidate=encode_fcvt_precision, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler (caller-reachable from encode_instruction),
    //   mapping=[Reg(Sd|Dd|Hd), Reg(Sn|Dn|Hn)] dest_ty != src_ty <-> `fcvt Sd|Dd|Hd, Sn|Dn|Hn`

    use super::encode_fcvt_precision;
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

    /// 0=S, 1=D, 2=H
    fn prec(ty: u32) -> &'static str {
        ["s", "d", "h"][ty as usize]
    }

    fn ftype_of(src_ty: u32) -> u32 {
        [0b00, 0b01, 0b11][src_ty as usize]
    }

    fn opc_of(dest_ty: u32) -> u32 {
        [0b00, 0b01, 0b11][dest_ty as usize]
    }

    fn fp_name(ty: u32, n: u32) -> String {
        format!("{}{}", prec(ty), n)
    }

    fn fp_spelling(ty: u32, n: u32, kind: u32) -> String {
        let s = fp_name(ty, n);
        if kind == 0 {
            s.to_uppercase()
        } else {
            s
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

    fn sut_word(ops: &[Operand]) -> Result<u32, String> {
        match encode_fcvt_precision(ops)? {
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

    fn llvm_mc_word_with(asm: &str, extra_args: &[&str]) -> Result<u32, String> {
        let mut args = vec!["-triple=aarch64", "-show-encoding"];
        args.extend_from_slice(extra_args);
        let mut child = Command::new(LLVM_MC)
            .args(&args)
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

    fn llvm_mc_word(asm: &str) -> Result<u32, String> {
        llvm_mc_word_with(asm, &[])
    }

    fn llvm_mc_fp16_word(asm: &str) -> Result<u32, String> {
        llvm_mc_word_with(asm, &["-mattr=+fullfp16"])
    }

    fn llvm_ref(asm: &str, uses_h: bool) -> Result<u32, String> {
        if uses_h {
            llvm_mc_fp16_word(asm)
        } else {
            llvm_mc_word(asm)
        }
    }

    fn extra_operand() -> impl Strategy<Value = Operand> {
        prop_oneof![
            (0u32..=31).prop_map(|n| Operand::Reg(format!("s{n}"))),
            Just(Operand::Imm(0)),
            Just(Operand::Imm(1)),
            Just(Operand::Shift {
                kind: "lsl".into(),
                amount: 0,
            }),
            Just(Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "4s".into(),
            }),
        ]
    }

    fn wrong_type_pair() -> impl Strategy<Value = (String, String)> {
        let n = 0u32..=31;
        prop_oneof![
            // GPR dest + S/D/H src
            (n.clone(), n.clone(), any::<bool>(), 0u32..=2).prop_map(|(d, s, d64, sty)| {
                (gpr(d64, d), fp_name(sty, s))
            }),
            // S/D/H dest + GPR src
            (n.clone(), n.clone(), 0u32..=2, any::<bool>()).prop_map(|(d, s, dty, s64)| {
                (fp_name(dty, d), gpr(s64, s))
            }),
            // Q/V/B dest + S/D/H src
            (n.clone(), n.clone(), 0u32..=2, 0u32..=2).prop_map(|(d, s, p, sty)| {
                let pref = ["q", "v", "b"][p as usize];
                (format!("{pref}{d}"), fp_name(sty, s))
            }),
            // S/D/H dest + Q/V/B src
            (n.clone(), n.clone(), 0u32..=2, 0u32..=2).prop_map(|(d, s, dty, p)| {
                let pref = ["q", "v", "b"][p as usize];
                (fp_name(dty, d), format!("{pref}{s}"))
            }),
            // SP/WSP dest
            (n.clone(), any::<bool>(), 0u32..=2).prop_map(|(s, is_64, sty)| {
                let sp = if is_64 { "sp" } else { "wsp" };
                (sp.to_string(), fp_name(sty, s))
            }),
            // SP/WSP src
            (n.clone(), any::<bool>(), 0u32..=2).prop_map(|(d, is_64, dty)| {
                let sp = if is_64 { "sp" } else { "wsp" };
                (fp_name(dty, d), sp.to_string())
            }),
        ]
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_fcvt_precision_kat_llvm_mc_d0_s1() {
        let want = 0x1e22c020u32;
        let mc = llvm_mc_word("fcvt d0, s1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [Operand::Reg("d0".into()), Operand::Reg("s1".into())];
        let sut = sut_word(&ops).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_fcvt_precision_kat_llvm_mc_s0_d1() {
        let want = 0x1e624020u32;
        let mc = llvm_mc_word("fcvt s0, d1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [Operand::Reg("s0".into()), Operand::Reg("d1".into())];
        let sut = sut_word(&ops).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_fcvt_precision_kat_llvm_mc_h0_s1() {
        let want = 0x1e23c020u32;
        let mc = llvm_mc_fp16_word("fcvt h0, s1").expect("llvm-mc fp16 KAT");
        assert_eq!(mc, want, "llvm-mc fp16 KAT mapping broken");
        let ops = [Operand::Reg("h0".into()), Operand::Reg("s1".into())];
        let sut = sut_word(&ops).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_fcvt_precision_kat_llvm_mc_s0_h1() {
        let want = 0x1ee24020u32;
        let mc = llvm_mc_fp16_word("fcvt s0, h1").expect("llvm-mc fp16 KAT");
        assert_eq!(mc, want, "llvm-mc fp16 KAT mapping broken");
        let ops = [Operand::Reg("s0".into()), Operand::Reg("h1".into())];
        let sut = sut_word(&ops).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_fcvt_precision_kat_llvm_mc_d0_h1() {
        let want = 0x1ee2c020u32;
        let mc = llvm_mc_fp16_word("fcvt d0, h1").expect("llvm-mc fp16 KAT");
        assert_eq!(mc, want, "llvm-mc fp16 KAT mapping broken");
        let ops = [Operand::Reg("d0".into()), Operand::Reg("h1".into())];
        let sut = sut_word(&ops).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_fcvt_precision_kat_llvm_mc_h0_d1() {
        let want = 0x1e63c020u32;
        let mc = llvm_mc_fp16_word("fcvt h0, d1").expect("llvm-mc fp16 KAT");
        assert_eq!(mc, want, "llvm-mc fp16 KAT mapping broken");
        let ops = [Operand::Reg("h0".into()), Operand::Reg("d1".into())];
        let sut = sut_word(&ops).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_fcvt_precision_kat_llvm_mc_d31_s31() {
        let want = 0x1e22c3ffu32;
        let mc = llvm_mc_word("fcvt d31, s31").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [Operand::Reg("d31".into()), Operand::Reg("s31".into())];
        let sut = sut_word(&ops).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_fcvt_precision_kat_llvm_mc_s31_d0() {
        let want = 0x1e62401fu32;
        let mc = llvm_mc_word("fcvt s31, d0").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let ops = [Operand::Reg("s31".into()), Operand::Reg("d0".into())];
        let sut = sut_word(&ops).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn test_encode_fcvt_precision_regression_extra_operand() {
        let ops = [
            Operand::Reg("d0".into()),
            Operand::Reg("s1".into()),
            Operand::Reg("s2".into()),
        ];
        assert!(
            encode_fcvt_precision(&ops).is_err(),
            "FCVT must reject a 3rd operand"
        );
    }

    #[test]
    fn test_encode_fcvt_precision_regression_same_precision_s() {
        let ops = [Operand::Reg("s0".into()), Operand::Reg("s1".into())];
        assert!(
            encode_fcvt_precision(&ops).is_err(),
            "FCVT must reject same-precision S,S (ARM ARM ftype==opc unallocated)"
        );
    }

    #[test]
    fn test_encode_fcvt_precision_regression_same_precision_d() {
        let ops = [Operand::Reg("d0".into()), Operand::Reg("d1".into())];
        assert!(
            encode_fcvt_precision(&ops).is_err(),
            "FCVT must reject same-precision D,D (ARM ARM ftype==opc unallocated)"
        );
    }

    #[test]
    fn test_encode_fcvt_precision_regression_same_precision_h() {
        let ops = [Operand::Reg("h0".into()), Operand::Reg("h1".into())];
        assert!(
            encode_fcvt_precision(&ops).is_err(),
            "FCVT must reject same-precision H,H (ARM ARM ftype==opc unallocated)"
        );
    }

    #[test]
    fn test_encode_fcvt_precision_regression_sp_as_s() {
        let ops = [Operand::Reg("sp".into()), Operand::Reg("d0".into())];
        assert!(
            encode_fcvt_precision(&ops).is_err(),
            "FCVT must reject SP dest (not an S register)"
        );
    }

    #[test]
    fn test_encode_fcvt_precision_regression_sp_src() {
        let ops = [Operand::Reg("d0".into()), Operand::Reg("sp".into())];
        assert!(
            encode_fcvt_precision(&ops).is_err(),
            "FCVT must reject SP source (not an S register)"
        );
    }

    proptest! {
        #![proptest_config(cfg())]

        #[test]
        fn encode_fcvt_precision_diff_valid(
            rd in 0u32..=31,
            rn in 0u32..=31,
            dest_ty in 0u32..=2,
            src_off in 1u32..=2,
            dest_spell in 0u32..=1,
            src_spell in 0u32..=1,
        ) {
            let src_ty = (dest_ty + src_off) % 3;
            let dest = fp_spelling(dest_ty, rd, dest_spell);
            let src = fp_spelling(src_ty, rn, src_spell);
            let asm = format!("fcvt {dest}, {src}");
            let ops = [Operand::Reg(dest.clone()), Operand::Reg(src.clone())];
            let uses_h = dest_ty == 2 || src_ty == 2;
            let mc = llvm_ref(&asm, uses_h)
                .unwrap_or_else(|e| panic!("llvm-mc rejected valid {asm}: {e}"));
            let sut = sut_word(&ops)
                .unwrap_or_else(|e| panic!("SUT rejected valid {asm}: {e}"));
            prop_assert_eq!(sut, mc, "FCVT mismatch for {}", asm);
        }

        #[test]
        fn encode_fcvt_precision_arm_fields(
            rd in 0u32..=31,
            rn in 0u32..=31,
            dest_ty in 0u32..=2,
            src_off in 1u32..=2,
        ) {
            let src_ty = (dest_ty + src_off) % 3;
            let ops = [
                Operand::Reg(fp_name(dest_ty, rd)),
                Operand::Reg(fp_name(src_ty, rn)),
            ];
            let w = sut_word(&ops).expect("SUT FCVT");
            let ftype = ftype_of(src_ty);
            let opc = opc_of(dest_ty);
            let want = (0b00011110u32 << 24)
                | (ftype << 22)
                | (1 << 21)
                | (0b0001 << 17)
                | (opc << 15)
                | (0b10000 << 10)
                | (rn << 5)
                | rd;
            prop_assert_eq!(w, want, "ARM ARM FCVT field layout");
            prop_assert_eq!((w >> 24) & 0xff, 0b00011110, "bits[31:24] M=0 S=0 11110");
            prop_assert_eq!((w >> 22) & 0b11, ftype, "ftype");
            prop_assert_eq!((w >> 21) & 1, 1, "bit21 must be 1");
            prop_assert_eq!((w >> 17) & 0xf, 0b0001, "bits[20:17] must be 0001");
            prop_assert_eq!((w >> 15) & 0b11, opc, "opc dest precision");
            prop_assert_eq!((w >> 10) & 0x1f, 0b10000, "bits[14:10] must be 10000");
            prop_assert_eq!((w >> 5) & 0x1f, rn, "Rn");
            prop_assert_eq!(w & 0x1f, rd, "Rd");
        }

        #[test]
        fn encode_fcvt_precision_metamorphic_fields(
            rd in 0u32..=30,
            rn in 0u32..=30,
            dest_ty in 0u32..=2,
            src_off in 1u32..=2,
        ) {
            let src_ty = (dest_ty + src_off) % 3;
            let ops = |dty: u32, sty: u32, d: u32, n: u32| {
                [
                    Operand::Reg(fp_name(dty, d)),
                    Operand::Reg(fp_name(sty, n)),
                ]
            };
            let w = sut_word(&ops(dest_ty, src_ty, rd, rn)).expect("base FCVT");
            let w_rd = sut_word(&ops(dest_ty, src_ty, rd + 1, rn)).expect("Rd+1");
            let w_rn = sut_word(&ops(dest_ty, src_ty, rd, rn + 1)).expect("Rn+1");
            prop_assert_eq!(w_rd, w + 1, "Rd+1 must increment bits[4:0] only");
            prop_assert_eq!(w_rn, w + (1 << 5), "Rn+1 must increment bits[9:5] only");

            // dest S vs D with H src (both dest_ty != 2, src H) flips only opc bit 15
            let w_sh = sut_word(&ops(0, 2, rd, rn)).expect("S,H");
            let w_dh = sut_word(&ops(1, 2, rd, rn)).expect("D,H");
            prop_assert_eq!(w_dh ^ w_sh, 1u32 << 15, "S dest vs D dest (H src) must flip only opc bit 15");

            // src S vs D with H dest flips only ftype bit 22
            let w_hs = sut_word(&ops(2, 0, rd, rn)).expect("H,S");
            let w_hd = sut_word(&ops(2, 1, rd, rn)).expect("H,D");
            prop_assert_eq!(w_hd ^ w_hs, 1u32 << 22, "S src vs D src (H dest) must flip only ftype bit 22");
        }

        #[test]
        fn encode_fcvt_precision_neg_arity(
            len in 0usize..=1,
            n in 0u32..=31,
            ty in 0u32..=2,
        ) {
            let mut ops = Vec::new();
            if len >= 1 {
                ops.push(Operand::Reg(fp_name(ty, n)));
            }
            prop_assert!(
                encode_fcvt_precision(&ops).is_err(),
                "FCVT with {} operands must Err (llvm-mc: too few operands)",
                len
            );
        }

        #[test]
        fn encode_fcvt_precision_neg_extra_operand(
            rd in 0u32..=31,
            rn in 0u32..=31,
            dest_ty in 0u32..=2,
            src_off in 1u32..=2,
            extra in extra_operand(),
        ) {
            let src_ty = (dest_ty + src_off) % 3;
            let ops = vec![
                Operand::Reg(fp_name(dest_ty, rd)),
                Operand::Reg(fp_name(src_ty, rn)),
                extra,
            ];
            prop_assert!(
                encode_fcvt_precision(&ops).is_err(),
                "FCVT has no 3rd operand; extra must Err"
            );
        }

        #[test]
        fn encode_fcvt_precision_neg_same_precision(
            rd in 0u32..=31,
            rn in 0u32..=31,
            ty in 0u32..=2,
        ) {
            let ops = [
                Operand::Reg(fp_name(ty, rd)),
                Operand::Reg(fp_name(ty, rn)),
            ];
            prop_assert!(
                encode_fcvt_precision(&ops).is_err(),
                "FCVT same-precision {}{}, {}{} must Err (ARM ARM ftype==opc unallocated)",
                prec(ty), rd, prec(ty), rn
            );
        }

        #[test]
        fn encode_fcvt_precision_neg_gpr_qvb(
            n in 0u32..=31,
            m in 0u32..=31,
            which in 0u32..=1,
            kind in 0u32..=4,
            is_64 in any::<bool>(),
            fp_ty in 0u32..=2,
        ) {
            // GPR (x/w) and Q/V/B — first char is not s/d/h, so these hit the
            // documented "unsupported dest/source type" arms. SP is excluded
            // (separate failing witness: first char 's').
            let bad = match kind {
                0 => gpr(is_64, n),
                1 => format!("q{n}"),
                2 => format!("v{n}"),
                3 => format!("b{n}"),
                _ => "wsp".to_string(),
            };
            let good = fp_name(fp_ty, m);
            let ops = if which == 0 {
                [Operand::Reg(bad.clone()), Operand::Reg(good.clone())]
            } else {
                [Operand::Reg(good.clone()), Operand::Reg(bad.clone())]
            };
            prop_assert!(
                encode_fcvt_precision(&ops).is_err(),
                "FCVT must reject GPR/QVB/WSP {} at slot {} (other={})",
                bad,
                which,
                good
            );
        }

        #[test]
        fn encode_fcvt_precision_neg_wrong_types(
            (a, b) in wrong_type_pair(),
        ) {
            let ops = [Operand::Reg(a.clone()), Operand::Reg(b.clone())];
            prop_assert!(
                encode_fcvt_precision(&ops).is_err(),
                "FCVT requires Sd|Dd|Hd, Sn|Dn|Hn with dest!=src precision; a={} b={} must Err",
                a,
                b
            );
        }

        #[test]
        fn encode_fcvt_precision_neg_nonreg_invalid_name(
            which in 0u32..=1,
            kind in 0u32..=12,
        ) {
            let bad = match kind {
                0 => Operand::Imm(1),
                1 => Operand::Symbol("foo".into()),
                2 => Operand::Label("1f".into()),
                3 => Operand::Mem {
                    base: "x0".into(),
                    offset: 0,
                },
                4 => Operand::Cond("eq".into()),
                5 => Operand::Shift {
                    kind: "lsl".into(),
                    amount: 0,
                },
                6 => Operand::Reg("foo".into()),
                7 => Operand::Reg("s32".into()),
                8 => Operand::Reg("d32".into()),
                9 => Operand::Reg("h32".into()),
                10 => Operand::Reg("r0".into()),
                11 => Operand::Reg("s".into()),
                _ => Operand::Reg("".into()),
            };
            let mut ops = vec![
                Operand::Reg("d0".into()),
                Operand::Reg("s1".into()),
            ];
            ops[which as usize] = bad;
            prop_assert!(
                encode_fcvt_precision(&ops).is_err(),
                "non-register or invalid name at slot {} must Err",
                which
            );
        }
    }
}
