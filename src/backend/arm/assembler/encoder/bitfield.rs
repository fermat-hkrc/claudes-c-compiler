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
mod encode_bfi_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler
    // Evidence: src/backend/arm/assembler/README.md:11 "accepts the same textual assembly that GCC's gas would consume";
    //   encoder/mod.rs "Encodes AArch64 instructions into 32-bit machine code words";
    //   encoder/mod.rs:892 "bfi" => encode_bfi; README.md:216 lists bfi;
    //   bitfield.rs:103 purpose comment BFI -> BFM #(-lsb mod width_reg), #(width-1);
    //   ARM ARM Bitfield Move BFI alias of BFM: sf 01 100110 N immr imms Rn Rd,
    //   N=sf, immr=(-lsb MOD datasize), imms=width-1; register 31 is ZR not SP.
    // Stronger considered:
    //   - State machine: rejected — encode_bfi is a pure function with no lifecycle
    //   - Algebraic round-trip: rejected — no in-tree BFI decoder
    //   - encode_bfm as differential sibling: rejected — same-job gate fails (raw immr/imms form);
    //     used only as algebraic alias after the ARM mapping
    //   - encode_ubfiz / encode_sbfiz: rejected — UBFM/SBFM opc, different instruction
    // Weaker available: algebraic.metamorphic (BFI alias of BFM; Rd/Rn field independence),
    //   algebraic.invariant (ARM fields), negative_error (arity / extra / SP / lsb-width)
    // Differential: candidate=encode_bfi, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=[Reg(Rd), Reg(Rn), Imm(lsb), Imm(width)] <-> `bfi Rd, Rn, #lsb, #width`

    use super::{encode_bfi, encode_bfm};
    use super::super::EncodeResult;
    use crate::backend::arm::assembler::parser::Operand;
    use proptest::prelude::*;
    use std::io::Write;
    use std::panic::{catch_unwind, AssertUnwindSafe};
    use std::process::{Command, Stdio};

    const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
    const CASES: u32 = 1000;

    fn cfg() -> ProptestConfig {
        ProptestConfig::with_cases(CASES)
    }

    fn gpr(is_64: bool, n: u32) -> String {
        if is_64 {
            if n == 31 {
                "xzr".into()
            } else {
                format!("x{n}")
            }
        } else if n == 31 {
            "wzr".into()
        } else {
            format!("w{n}")
        }
    }

    fn lsb_width(r: u32) -> impl Strategy<Value = (u32, u32)> {
        prop_oneof![Just(0u32), Just(r - 1), 0u32..r].prop_flat_map(move |lsb| {
            let max_w = r - lsb;
            prop_oneof![Just(1u32), Just(max_w), 1u32..=max_w]
                .prop_map(move |width| (lsb, width))
        })
    }

    fn bfi_valid() -> impl Strategy<Value = (bool, u32, u32, u32, u32)> {
        any::<bool>().prop_flat_map(|is_64| {
            let r = if is_64 { 64u32 } else { 32 };
            (
                Just(is_64),
                0u32..=31,
                0u32..=31,
                lsb_width(r),
            )
                .prop_map(|(is_64, rd, rn, (lsb, width))| (is_64, rd, rn, lsb, width))
        })
    }

    fn invalid_lsb_width(is_64: bool) -> impl Strategy<Value = (i64, i64)> {
        let r = if is_64 { 64i64 } else { 32 };
        prop_oneof![
            (0i64..r).prop_map(|lsb| (lsb, 0i64)),
            (0i64..r).prop_map(|lsb| (lsb, -1i64)),
            (1i64..=r).prop_map(|width| (-1i64, width)),
            Just((r, 1i64)),
            Just((r, r)),
            Just((r - 1, 2i64)),
            Just((0i64, r + 1)),
            Just((r + 1, 1i64)),
        ]
    }

    fn ops4(is_64: bool, rd: u32, rn: u32, lsb: i64, width: i64) -> [Operand; 4] {
        [
            Operand::Reg(gpr(is_64, rd)),
            Operand::Reg(gpr(is_64, rn)),
            Operand::Imm(lsb),
            Operand::Imm(width),
        ]
    }

    fn sut_word(ops: &[Operand]) -> Result<u32, String> {
        match encode_bfi(ops)? {
            EncodeResult::Word(w) => Ok(w),
            other => Err(format!("expected Word, got {:?}", other)),
        }
    }

    fn bfm_word(ops: &[Operand]) -> Result<u32, String> {
        match encode_bfm(ops)? {
            EncodeResult::Word(w) => Ok(w),
            other => Err(format!("expected Word, got {:?}", other)),
        }
    }

    fn must_err(ops: &[Operand]) -> bool {
        match catch_unwind(AssertUnwindSafe(|| encode_bfi(ops))) {
            Ok(Err(_)) => true,
            _ => false,
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
            Just(Operand::Imm(-1)),
            Just(Operand::Shift {
                kind: "lsl".into(),
                amount: 0,
            }),
            Just(Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "8h".into(),
            }),
        ]
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_bfi_kat_llvm_mc_w0_w1_lsb0_width1() {
        let want = 0x33000020u32;
        let mc = llvm_mc_word("bfi w0, w1, #0, #1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops4(false, 0, 1, 0, 1)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_bfi_kat_llvm_mc_w0_w1_lsb1_width1() {
        let want = 0x331f0020u32;
        let mc = llvm_mc_word("bfi w0, w1, #1, #1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops4(false, 0, 1, 1, 1)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_bfi_kat_llvm_mc_x0_x1_lsb1_width8() {
        let want = 0xb37f1c20u32;
        let mc = llvm_mc_word("bfi x0, x1, #1, #8").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops4(true, 0, 1, 1, 8)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_bfi_kat_llvm_mc_wzr_wzr_lsb31_width1() {
        let want = 0x330103ffu32;
        let mc = llvm_mc_word("bfi wzr, wzr, #31, #1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops4(false, 31, 31, 31, 1)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_bfi_kat_llvm_mc_x0_xzr_lsb63_width1() {
        let want = 0xb34103e0u32;
        let mc = llvm_mc_word("bfi x0, xzr, #63, #1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops4(true, 0, 31, 63, 1)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_bfi_kat_llvm_mc_lr_x1_lsb8_width16() {
        let want = 0xb3783c3eu32;
        let mc = llvm_mc_word("bfi lr, x1, #8, #16").expect("llvm-mc LR KAT");
        assert_eq!(mc, want, "llvm-mc LR KAT mapping broken");
        let ops = [
            Operand::Reg("lr".into()),
            Operand::Reg("x1".into()),
            Operand::Imm(8),
            Operand::Imm(16),
        ];
        let sut = sut_word(&ops).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    proptest! {
        #![proptest_config(cfg())]

        #[test]
        fn encode_bfi_diff_valid_gpr((is_64, rd, rn, lsb, width) in bfi_valid()) {
            let dest = gpr(is_64, rd);
            let src = gpr(is_64, rn);
            let asm = format!("bfi {}, {}, #{}, #{}", dest, src, lsb, width);
            let ops = ops4(is_64, rd, rn, lsb as i64, width as i64);
            let mc = llvm_mc_word(&asm).expect("llvm-mc");
            let sut = sut_word(&ops).expect("SUT");
            prop_assert_eq!(sut, mc, "BFI mismatch for {}", asm);
        }

        #[test]
        fn encode_bfi_alias_bfm((is_64, rd, rn, lsb, width) in bfi_valid()) {
            let r = if is_64 { 64i64 } else { 32 };
            let immr = (-(lsb as i64)).rem_euclid(r);
            let imms = (width as i64) - 1;
            let bfi = sut_word(&ops4(is_64, rd, rn, lsb as i64, width as i64)).expect("BFI");
            let bfm = bfm_word(&[
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
                Operand::Imm(immr),
                Operand::Imm(imms),
            ]).expect("BFM");
            prop_assert_eq!(bfi, bfm, "BFI must alias BFM #(-lsb MOD {}), #(width-1)", r);
        }

        #[test]
        fn encode_bfi_arm_fields((is_64, rd, rn, lsb, width) in bfi_valid()) {
            let r = if is_64 { 64i64 } else { 32 };
            let immr = (-(lsb as i64)).rem_euclid(r) as u32;
            let imms = width - 1;
            let sf = if is_64 { 1u32 } else { 0 };
            let w = sut_word(&ops4(is_64, rd, rn, lsb as i64, width as i64)).expect("SUT");
            let want = (sf << 31)
                | (0b01 << 29)
                | (0b100110 << 23)
                | (sf << 22)
                | (immr << 16)
                | (imms << 10)
                | (rn << 5)
                | rd;
            prop_assert_eq!(w, want, "ARM ARM BFM/BFI field layout");
            prop_assert_eq!(w >> 31, sf, "sf");
            prop_assert_eq!((w >> 29) & 0b11, 0b01, "opc=01 BFM");
            prop_assert_eq!((w >> 23) & 0x3f, 0b100110, "bits[28:23]");
            prop_assert_eq!((w >> 22) & 1, sf, "N=sf");
            prop_assert_eq!((w >> 16) & 0x3f, immr, "immr=(-lsb MOD R)");
            prop_assert_eq!((w >> 10) & 0x3f, imms, "imms=width-1");
            prop_assert_eq!((w >> 5) & 0x1f, rn, "Rn");
            prop_assert_eq!(w & 0x1f, rd, "Rd");
        }

        #[test]
        fn encode_bfi_metamorphic_rd_rn(
            is_64 in any::<bool>(),
            rd in 0u32..=30,
            rn in 0u32..=30,
            lsb_w in 0u32..=1,
        ) {
            let r = if is_64 { 64u32 } else { 32 };
            let lsb = if lsb_w == 0 { 0 } else { 1 };
            let width = 1u32;
            prop_assume!(lsb < r && width <= r - lsb);
            let base = sut_word(&ops4(is_64, rd, rn, lsb as i64, width as i64)).expect("base");
            let w_rd = sut_word(&ops4(is_64, rd + 1, rn, lsb as i64, width as i64)).expect("rd+1");
            let w_rn = sut_word(&ops4(is_64, rd, rn + 1, lsb as i64, width as i64)).expect("rn+1");
            prop_assert_eq!(w_rd & 0x1f, rd + 1, "Rd+1 updates Rd field");
            prop_assert_eq!(w_rd & !0x1fu32, base & !0x1fu32, "Rd+1 leaves other fields unchanged");
            prop_assert_eq!((w_rn >> 5) & 0x1f, rn + 1, "Rn+1 updates Rn field");
            prop_assert_eq!(w_rn & !(0x1fu32 << 5), base & !(0x1fu32 << 5), "Rn+1 leaves other fields unchanged");
        }

        #[test]
        fn encode_bfi_neg_arity(
            len in 0usize..=3,
            is_64 in any::<bool>(),
            rd in 0u32..=31,
            rn in 0u32..=31,
        ) {
            let mut ops = vec![
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
                Operand::Imm(0),
                Operand::Imm(1),
            ];
            ops.truncate(len);
            prop_assert!(
                encode_bfi(&ops).is_err(),
                "BFI with {} operands must Err (llvm-mc: too few operands)",
                len
            );
        }

        #[test]
        fn encode_bfi_neg_extra_operand(
            (is_64, rd, rn, lsb, width) in bfi_valid(),
            extra in extra_operand(),
        ) {
            let mut ops = ops4(is_64, rd, rn, lsb as i64, width as i64).to_vec();
            ops.push(extra);
            prop_assert!(
                encode_bfi(&ops).is_err(),
                "BFI has no 5th operand; extra operand must Err (llvm-mc rejects it)"
            );
        }

        #[test]
        fn encode_bfi_neg_sp(
            which in 0u32..=1,
            sp64 in any::<bool>(),
            is_64 in any::<bool>(),
            other in 0u32..=30,
        ) {
            let sp = if sp64 { "sp" } else { "wsp" };
            let mut ops = ops4(is_64, other, other, 0, 1);
            ops[which as usize] = Operand::Reg(sp.to_string());
            prop_assert!(
                encode_bfi(&ops).is_err(),
                "SP/WSP is not a valid BFI operand (which={} sp={})",
                which,
                sp
            );
        }

        #[test]
        fn encode_bfi_neg_lsb_width(
            (is_64, rd, rn, lsb, width) in any::<bool>().prop_flat_map(|is_64| {
                (Just(is_64), 0u32..=31, 0u32..=31, invalid_lsb_width(is_64))
                    .prop_map(|(is_64, rd, rn, (lsb, width))| (is_64, rd, rn, lsb, width))
            }),
        ) {
            let r = if is_64 { 64i64 } else { 32 };
            prop_assume!(!(lsb >= 0 && width >= 1 && lsb < r && width <= r - lsb));
            let ops = ops4(is_64, rd, rn, lsb, width);
            prop_assert!(
                must_err(&ops),
                "BFI lsb={} width={} R={} must Err (ARM: 0<=lsb<R, 1<=width<=R-lsb)",
                lsb,
                width,
                r
            );
        }

        #[test]
        fn encode_bfi_diff_alt_spellings(
            (is_64, rd, rn, lsb, width) in bfi_valid(),
            dest_spell in 0u32..=4,
            src_spell in 0u32..=4,
        ) {
            let dest = spell(is_64, rd, dest_spell);
            let src = spell(is_64, rn, src_spell);
            let asm = format!("bfi {}, {}, #{}, #{}", dest, src, lsb, width);
            let ops = [
                Operand::Reg(dest),
                Operand::Reg(src),
                Operand::Imm(lsb as i64),
                Operand::Imm(width as i64),
            ];
            let mc = llvm_mc_word(&asm).expect("llvm-mc");
            let sut = sut_word(&ops).expect("SUT");
            prop_assert_eq!(sut, mc, "BFI alt-spelling mismatch for {}", asm);
        }

        #[test]
        fn encode_bfi_neg_mixed_width(
            rd in 0u32..=31,
            rn in 0u32..=31,
            rd64 in any::<bool>(),
            rn64 in any::<bool>(),
        ) {
            prop_assume!(rd64 != rn64);
            let ops = [
                Operand::Reg(gpr(rd64, rd)),
                Operand::Reg(gpr(rn64, rn)),
                Operand::Imm(0),
                Operand::Imm(1),
            ];
            prop_assert!(
                encode_bfi(&ops).is_err(),
                "BFI mixed W/X (rd64={} rn64={}) must Err (llvm-mc rejects it)",
                rd64,
                rn64
            );
        }

        #[test]
        fn encode_bfi_neg_fp(
            which in 0u32..=1,
            prefix in prop::sample::select(vec!["d", "s", "q", "v", "h", "b"]),
            n in 0u32..=31,
        ) {
            let fp = format!("{}{}", prefix, n);
            let mut ops = ops4(true, 0, 1, 0, 1);
            ops[which as usize] = Operand::Reg(fp.clone());
            prop_assert!(
                encode_bfi(&ops).is_err(),
                "FP/SIMD register {} is not a valid BFI operand (which={})",
                fp,
                which
            );
        }

        #[test]
        fn encode_bfi_neg_nonreg(
            which in 0u32..=3,
            bad in non_reg_operand(),
        ) {
            if which >= 2 {
                prop_assume!(!matches!(bad, Operand::Imm(_)));
            }
            let mut ops = ops4(false, 0, 1, 0, 1).to_vec();
            ops[which as usize] = bad;
            prop_assert!(
                encode_bfi(&ops).is_err(),
                "wrong operand kind at slot {} must Err",
                which
            );
        }

        #[test]
        fn encode_bfi_neg_invalid_name(
            which in 0u32..=1,
            name in invalid_name(),
        ) {
            let mut ops = ops4(false, 0, 1, 0, 1);
            ops[which as usize] = Operand::Reg(name.clone());
            prop_assert!(
                encode_bfi(&ops).is_err(),
                "invalid register name {:?} at slot {} must Err",
                name,
                which
            );
        }
    }

    fn spell(is_64: bool, n: u32, kind: u32) -> String {
        match kind {
            0 if n == 31 => {
                if is_64 {
                    "x31".into()
                } else {
                    "w31".into()
                }
            }
            1 if n == 31 => {
                if is_64 {
                    "XZR".into()
                } else {
                    "WZR".into()
                }
            }
            2 if n == 30 && is_64 => "LR".into(),
            3 => gpr(is_64, n).to_uppercase(),
            _ => gpr(is_64, n),
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
            "x-1".into(),
            "x99".into(),
            "w".into(),
        ])
    }

    fn non_reg_operand() -> impl Strategy<Value = Operand> {
        prop_oneof![
            any::<i64>().prop_map(Operand::Imm),
            Just(Operand::Shift {
                kind: "lsl".into(),
                amount: 0,
            }),
            Just(Operand::Mem {
                base: "x0".into(),
                offset: 0,
            }),
            Just(Operand::Label("L0".into())),
            Just(Operand::Symbol("foo".into())),
            Just(Operand::Cond("eq".into())),
            Just(Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "8b".into(),
            }),
        ]
    }

    #[test]
    fn test_encode_bfi_regression_extra_operand() {
        let mut ops = ops4(false, 0, 0, 0, 1).to_vec();
        ops.push(Operand::Reg("x0".into()));
        assert!(
            encode_bfi(&ops).is_err(),
            "BFI w0, w0, #0, #1, x0 must Err; extra operand is invalid (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_bfi_regression_sp() {
        let ops = [
            Operand::Reg("wsp".into()),
            Operand::Reg("w0".into()),
            Operand::Imm(0),
            Operand::Imm(1),
        ];
        assert!(
            encode_bfi(&ops).is_err(),
            "BFI wsp, w0, #0, #1 must Err; register 31 is ZR not SP/WSP (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_bfi_regression_width_zero() {
        let ops = ops4(false, 0, 0, 0, 0);
        let result = catch_unwind(AssertUnwindSafe(|| encode_bfi(&ops)));
        assert!(
            matches!(result, Ok(Err(_))),
            "BFI w0, w0, #0, #0 must Err; width=0 is outside 1..=32-lsb (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_bfi_regression_mixed_width() {
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("w0".into()),
            Operand::Imm(0),
            Operand::Imm(1),
        ];
        assert!(
            encode_bfi(&ops).is_err(),
            "BFI x0, w0, #0, #1 must Err; mixed W/X is invalid (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_bfi_regression_fp() {
        let ops = [
            Operand::Reg("d0".into()),
            Operand::Reg("x1".into()),
            Operand::Imm(0),
            Operand::Imm(1),
        ];
        assert!(
            encode_bfi(&ops).is_err(),
            "BFI d0, x1, #0, #1 must Err; FP/SIMD registers are not BFI operands (llvm-mc rejects it)"
        );
    }
}

#[cfg(test)]
mod encode_bfxil_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler
    // Evidence: src/backend/arm/assembler/README.md:11 "accepts the same textual assembly that GCC's gas would consume";
    //   encoder/mod.rs "Encodes AArch64 instructions into 32-bit machine code words";
    //   encoder/mod.rs:893 "bfxil" => encode_bfxil; README.md:216 lists bfxil;
    //   bitfield.rs:118 purpose comment BFXIL -> BFM #lsb, #(lsb+width-1);
    //   ARM ARM Bitfield Move BFXIL alias of BFM: sf 01 100110 N immr imms Rn Rd,
    //   N=sf, immr=lsb, imms=lsb+width-1; register 31 is ZR not SP.
    // Stronger considered:
    //   - State machine: rejected — encode_bfxil is a pure function with no lifecycle
    //   - Algebraic round-trip: rejected — no in-tree BFXIL decoder
    //   - encode_bfm as differential sibling: rejected — same-job gate fails (raw immr/imms form);
    //     used only as algebraic alias after the ARM mapping
    //   - encode_ubfx / encode_sbfx: rejected — UBFM/SBFM opc, different instruction
    // Weaker available: algebraic.metamorphic (BFXIL alias of BFM; Rd/Rn field independence),
    //   algebraic.invariant (ARM fields), negative_error (arity / extra / SP / lsb-width)
    // Differential: candidate=encode_bfxil, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=[Reg(Rd), Reg(Rn), Imm(lsb), Imm(width)] <-> `bfxil Rd, Rn, #lsb, #width`

    use super::{encode_bfxil, encode_bfm};
    use super::super::EncodeResult;
    use crate::backend::arm::assembler::parser::Operand;
    use proptest::prelude::*;
    use std::io::Write;
    use std::panic::{catch_unwind, AssertUnwindSafe};
    use std::process::{Command, Stdio};

    const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
    const CASES: u32 = 1000;

    fn cfg() -> ProptestConfig {
        ProptestConfig::with_cases(CASES)
    }

    fn gpr(is_64: bool, n: u32) -> String {
        if is_64 {
            if n == 31 {
                "xzr".into()
            } else {
                format!("x{n}")
            }
        } else if n == 31 {
            "wzr".into()
        } else {
            format!("w{n}")
        }
    }

    fn lsb_width(r: u32) -> impl Strategy<Value = (u32, u32)> {
        prop_oneof![Just(0u32), Just(r - 1), 0u32..r].prop_flat_map(move |lsb| {
            let max_w = r - lsb;
            prop_oneof![Just(1u32), Just(max_w), 1u32..=max_w]
                .prop_map(move |width| (lsb, width))
        })
    }

    fn bfxil_valid() -> impl Strategy<Value = (bool, u32, u32, u32, u32)> {
        any::<bool>().prop_flat_map(|is_64| {
            let r = if is_64 { 64u32 } else { 32 };
            (
                Just(is_64),
                0u32..=31,
                0u32..=31,
                lsb_width(r),
            )
                .prop_map(|(is_64, rd, rn, (lsb, width))| (is_64, rd, rn, lsb, width))
        })
    }

    fn invalid_lsb_width(is_64: bool) -> impl Strategy<Value = (i64, i64)> {
        let r = if is_64 { 64i64 } else { 32 };
        prop_oneof![
            (0i64..r).prop_map(|lsb| (lsb, 0i64)),
            (0i64..r).prop_map(|lsb| (lsb, -1i64)),
            (1i64..=r).prop_map(|width| (-1i64, width)),
            Just((r, 1i64)),
            Just((r, r)),
            Just((r - 1, 2i64)),
            Just((0i64, r + 1)),
            Just((r + 1, 1i64)),
        ]
    }

    fn ops4(is_64: bool, rd: u32, rn: u32, lsb: i64, width: i64) -> [Operand; 4] {
        [
            Operand::Reg(gpr(is_64, rd)),
            Operand::Reg(gpr(is_64, rn)),
            Operand::Imm(lsb),
            Operand::Imm(width),
        ]
    }

    fn sut_word(ops: &[Operand]) -> Result<u32, String> {
        match encode_bfxil(ops)? {
            EncodeResult::Word(w) => Ok(w),
            other => Err(format!("expected Word, got {:?}", other)),
        }
    }

    fn bfm_word(ops: &[Operand]) -> Result<u32, String> {
        match encode_bfm(ops)? {
            EncodeResult::Word(w) => Ok(w),
            other => Err(format!("expected Word, got {:?}", other)),
        }
    }

    fn must_err(ops: &[Operand]) -> bool {
        match catch_unwind(AssertUnwindSafe(|| encode_bfxil(ops))) {
            Ok(Err(_)) => true,
            _ => false,
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
            Just(Operand::Imm(-1)),
            Just(Operand::Shift {
                kind: "lsl".into(),
                amount: 0,
            }),
            Just(Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "8h".into(),
            }),
        ]
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_bfxil_kat_llvm_mc_w0_w1_lsb0_width1() {
        let want = 0x33000020u32;
        let mc = llvm_mc_word("bfxil w0, w1, #0, #1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops4(false, 0, 1, 0, 1)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_bfxil_kat_llvm_mc_w0_w1_lsb1_width1() {
        let want = 0x33010420u32;
        let mc = llvm_mc_word("bfxil w0, w1, #1, #1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops4(false, 0, 1, 1, 1)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_bfxil_kat_llvm_mc_x0_x1_lsb1_width8() {
        let want = 0xb3412020u32;
        let mc = llvm_mc_word("bfxil x0, x1, #1, #8").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops4(true, 0, 1, 1, 8)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_bfxil_kat_llvm_mc_wzr_wzr_lsb31_width1() {
        let want = 0x331f7fffu32;
        let mc = llvm_mc_word("bfxil wzr, wzr, #31, #1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops4(false, 31, 31, 31, 1)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_bfxil_kat_llvm_mc_x0_xzr_lsb63_width1() {
        let want = 0xb37fffe0u32;
        let mc = llvm_mc_word("bfxil x0, xzr, #63, #1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops4(true, 0, 31, 63, 1)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_bfxil_kat_llvm_mc_lr_x1_lsb8_width16() {
        let want = 0xb3485c3eu32;
        let mc = llvm_mc_word("bfxil lr, x1, #8, #16").expect("llvm-mc LR KAT");
        assert_eq!(mc, want, "llvm-mc LR KAT mapping broken");
        let ops = [
            Operand::Reg("lr".into()),
            Operand::Reg("x1".into()),
            Operand::Imm(8),
            Operand::Imm(16),
        ];
        let sut = sut_word(&ops).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    proptest! {
        #![proptest_config(cfg())]

        #[test]
        fn encode_bfxil_diff_valid_gpr((is_64, rd, rn, lsb, width) in bfxil_valid()) {
            let dest = gpr(is_64, rd);
            let src = gpr(is_64, rn);
            let asm = format!("bfxil {}, {}, #{}, #{}", dest, src, lsb, width);
            let ops = ops4(is_64, rd, rn, lsb as i64, width as i64);
            let mc = llvm_mc_word(&asm).expect("llvm-mc");
            let sut = sut_word(&ops).expect("SUT");
            prop_assert_eq!(sut, mc, "BFXIL mismatch for {}", asm);
        }

        #[test]
        fn encode_bfxil_alias_bfm((is_64, rd, rn, lsb, width) in bfxil_valid()) {
            let imms = (lsb as i64) + (width as i64) - 1;
            let bfxil = sut_word(&ops4(is_64, rd, rn, lsb as i64, width as i64)).expect("BFXIL");
            let bfm = bfm_word(&[
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
                Operand::Imm(lsb as i64),
                Operand::Imm(imms),
            ]).expect("BFM");
            prop_assert_eq!(bfxil, bfm, "BFXIL must alias BFM #lsb, #(lsb+width-1)");
        }

        #[test]
        fn encode_bfxil_arm_fields((is_64, rd, rn, lsb, width) in bfxil_valid()) {
            let immr = lsb;
            let imms = lsb + width - 1;
            let sf = if is_64 { 1u32 } else { 0 };
            let w = sut_word(&ops4(is_64, rd, rn, lsb as i64, width as i64)).expect("SUT");
            let want = (sf << 31)
                | (0b01 << 29)
                | (0b100110 << 23)
                | (sf << 22)
                | (immr << 16)
                | (imms << 10)
                | (rn << 5)
                | rd;
            prop_assert_eq!(w, want, "ARM ARM BFM/BFXIL field layout");
            prop_assert_eq!(w >> 31, sf, "sf");
            prop_assert_eq!((w >> 29) & 0b11, 0b01, "opc=01 BFM");
            prop_assert_eq!((w >> 23) & 0x3f, 0b100110, "bits[28:23]");
            prop_assert_eq!((w >> 22) & 1, sf, "N=sf");
            prop_assert_eq!((w >> 16) & 0x3f, immr, "immr=lsb");
            prop_assert_eq!((w >> 10) & 0x3f, imms, "imms=lsb+width-1");
            prop_assert_eq!((w >> 5) & 0x1f, rn, "Rn");
            prop_assert_eq!(w & 0x1f, rd, "Rd");
        }

        #[test]
        fn encode_bfxil_metamorphic_rd_rn(
            is_64 in any::<bool>(),
            rd in 0u32..=30,
            rn in 0u32..=30,
            lsb_w in 0u32..=1,
        ) {
            let r = if is_64 { 64u32 } else { 32 };
            let lsb = if lsb_w == 0 { 0 } else { 1 };
            let width = 1u32;
            prop_assume!(lsb < r && width <= r - lsb);
            let base = sut_word(&ops4(is_64, rd, rn, lsb as i64, width as i64)).expect("base");
            let w_rd = sut_word(&ops4(is_64, rd + 1, rn, lsb as i64, width as i64)).expect("rd+1");
            let w_rn = sut_word(&ops4(is_64, rd, rn + 1, lsb as i64, width as i64)).expect("rn+1");
            prop_assert_eq!(w_rd & 0x1f, rd + 1, "Rd+1 updates Rd field");
            prop_assert_eq!(w_rd & !0x1fu32, base & !0x1fu32, "Rd+1 leaves other fields unchanged");
            prop_assert_eq!((w_rn >> 5) & 0x1f, rn + 1, "Rn+1 updates Rn field");
            prop_assert_eq!(w_rn & !(0x1fu32 << 5), base & !(0x1fu32 << 5), "Rn+1 leaves other fields unchanged");
        }

        #[test]
        fn encode_bfxil_neg_arity(
            len in 0usize..=3,
            is_64 in any::<bool>(),
            rd in 0u32..=31,
            rn in 0u32..=31,
        ) {
            let mut ops = vec![
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
                Operand::Imm(0),
                Operand::Imm(1),
            ];
            ops.truncate(len);
            prop_assert!(
                encode_bfxil(&ops).is_err(),
                "BFXIL with {} operands must Err (llvm-mc: too few operands)",
                len
            );
        }

        #[test]
        fn encode_bfxil_neg_extra_operand(
            (is_64, rd, rn, lsb, width) in bfxil_valid(),
            extra in extra_operand(),
        ) {
            let mut ops = ops4(is_64, rd, rn, lsb as i64, width as i64).to_vec();
            ops.push(extra);
            prop_assert!(
                encode_bfxil(&ops).is_err(),
                "BFXIL has no 5th operand; extra operand must Err (llvm-mc rejects it)"
            );
        }

        #[test]
        fn encode_bfxil_neg_sp(
            which in 0u32..=1,
            sp64 in any::<bool>(),
            is_64 in any::<bool>(),
            other in 0u32..=30,
        ) {
            let sp = if sp64 { "sp" } else { "wsp" };
            let mut ops = ops4(is_64, other, other, 0, 1);
            ops[which as usize] = Operand::Reg(sp.to_string());
            prop_assert!(
                encode_bfxil(&ops).is_err(),
                "SP/WSP is not a valid BFXIL operand (which={} sp={})",
                which,
                sp
            );
        }

        #[test]
        fn encode_bfxil_neg_lsb_width(
            (is_64, rd, rn, lsb, width) in any::<bool>().prop_flat_map(|is_64| {
                (Just(is_64), 0u32..=31, 0u32..=31, invalid_lsb_width(is_64))
                    .prop_map(|(is_64, rd, rn, (lsb, width))| (is_64, rd, rn, lsb, width))
            }),
        ) {
            let r = if is_64 { 64i64 } else { 32 };
            prop_assume!(!(lsb >= 0 && width >= 1 && lsb < r && width <= r - lsb));
            let ops = ops4(is_64, rd, rn, lsb, width);
            prop_assert!(
                must_err(&ops),
                "BFXIL lsb={} width={} R={} must Err (ARM: 0<=lsb<R, 1<=width<=R-lsb)",
                lsb,
                width,
                r
            );
        }

        #[test]
        fn encode_bfxil_diff_alt_spellings(
            (is_64, rd, rn, lsb, width) in bfxil_valid(),
            dest_spell in 0u32..=4,
            src_spell in 0u32..=4,
        ) {
            let dest = spell(is_64, rd, dest_spell);
            let src = spell(is_64, rn, src_spell);
            let asm = format!("bfxil {}, {}, #{}, #{}", dest, src, lsb, width);
            let ops = [
                Operand::Reg(dest),
                Operand::Reg(src),
                Operand::Imm(lsb as i64),
                Operand::Imm(width as i64),
            ];
            let mc = llvm_mc_word(&asm).expect("llvm-mc");
            let sut = sut_word(&ops).expect("SUT");
            prop_assert_eq!(sut, mc, "BFXIL alt-spelling mismatch for {}", asm);
        }

        #[test]
        fn encode_bfxil_neg_mixed_width(
            rd in 0u32..=31,
            rn in 0u32..=31,
            rd64 in any::<bool>(),
            rn64 in any::<bool>(),
        ) {
            prop_assume!(rd64 != rn64);
            let ops = [
                Operand::Reg(gpr(rd64, rd)),
                Operand::Reg(gpr(rn64, rn)),
                Operand::Imm(0),
                Operand::Imm(1),
            ];
            prop_assert!(
                encode_bfxil(&ops).is_err(),
                "BFXIL mixed W/X (rd64={} rn64={}) must Err (llvm-mc rejects it)",
                rd64,
                rn64
            );
        }

        #[test]
        fn encode_bfxil_neg_fp(
            which in 0u32..=1,
            prefix in prop::sample::select(vec!["d", "s", "q", "v", "h", "b"]),
            n in 0u32..=31,
        ) {
            let fp = format!("{}{}", prefix, n);
            let mut ops = ops4(true, 0, 1, 0, 1);
            ops[which as usize] = Operand::Reg(fp.clone());
            prop_assert!(
                encode_bfxil(&ops).is_err(),
                "FP/SIMD register {} is not a valid BFXIL operand (which={})",
                fp,
                which
            );
        }

        #[test]
        fn encode_bfxil_neg_nonreg(
            which in 0u32..=3,
            bad in non_reg_operand(),
        ) {
            if which >= 2 {
                prop_assume!(!matches!(bad, Operand::Imm(_)));
            }
            let mut ops = ops4(false, 0, 1, 0, 1).to_vec();
            ops[which as usize] = bad;
            prop_assert!(
                encode_bfxil(&ops).is_err(),
                "wrong operand kind at slot {} must Err",
                which
            );
        }

        #[test]
        fn encode_bfxil_neg_invalid_name(
            which in 0u32..=1,
            name in invalid_name(),
        ) {
            let mut ops = ops4(false, 0, 1, 0, 1);
            ops[which as usize] = Operand::Reg(name.clone());
            prop_assert!(
                encode_bfxil(&ops).is_err(),
                "invalid register name {:?} at slot {} must Err",
                name,
                which
            );
        }
    }

    fn spell(is_64: bool, n: u32, kind: u32) -> String {
        match kind {
            0 if n == 31 => {
                if is_64 {
                    "x31".into()
                } else {
                    "w31".into()
                }
            }
            1 if n == 31 => {
                if is_64 {
                    "XZR".into()
                } else {
                    "WZR".into()
                }
            }
            2 if n == 30 && is_64 => "LR".into(),
            3 => gpr(is_64, n).to_uppercase(),
            _ => gpr(is_64, n),
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
            "x-1".into(),
            "x99".into(),
            "w".into(),
        ])
    }

    fn non_reg_operand() -> impl Strategy<Value = Operand> {
        prop_oneof![
            any::<i64>().prop_map(Operand::Imm),
            Just(Operand::Shift {
                kind: "lsl".into(),
                amount: 0,
            }),
            Just(Operand::Mem {
                base: "x0".into(),
                offset: 0,
            }),
            Just(Operand::Label("L0".into())),
            Just(Operand::Symbol("foo".into())),
            Just(Operand::Cond("eq".into())),
            Just(Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "8b".into(),
            }),
        ]
    }

    #[test]
    fn test_encode_bfxil_regression_extra_operand() {
        let mut ops = ops4(false, 0, 0, 0, 1).to_vec();
        ops.push(Operand::Reg("x0".into()));
        assert!(
            encode_bfxil(&ops).is_err(),
            "BFXIL w0, w0, #0, #1, x0 must Err; extra operand is invalid (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_bfxil_regression_sp() {
        let ops = [
            Operand::Reg("wsp".into()),
            Operand::Reg("w0".into()),
            Operand::Imm(0),
            Operand::Imm(1),
        ];
        assert!(
            encode_bfxil(&ops).is_err(),
            "BFXIL wsp, w0, #0, #1 must Err; register 31 is ZR not SP/WSP (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_bfxil_regression_width_zero() {
        let ops = ops4(false, 0, 0, 0, 0);
        let result = catch_unwind(AssertUnwindSafe(|| encode_bfxil(&ops)));
        assert!(
            matches!(result, Ok(Err(_))),
            "BFXIL w0, w0, #0, #0 must Err; width=0 is outside 1..=32-lsb (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_bfxil_regression_mixed_width() {
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("w0".into()),
            Operand::Imm(0),
            Operand::Imm(1),
        ];
        assert!(
            encode_bfxil(&ops).is_err(),
            "BFXIL x0, w0, #0, #1 must Err; mixed W/X is invalid (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_bfxil_regression_fp() {
        let ops = [
            Operand::Reg("d0".into()),
            Operand::Reg("x1".into()),
            Operand::Imm(0),
            Operand::Imm(1),
        ];
        assert!(
            encode_bfxil(&ops).is_err(),
            "BFXIL d0, x1, #0, #1 must Err; FP/SIMD registers are not BFXIL operands (llvm-mc rejects it)"
        );
    }
}


#[cfg(test)]
mod encode_cls_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler
    // Evidence: src/backend/arm/assembler/README.md:11 "accepts the same textual assembly that GCC's gas would consume";
    //   encoder/mod.rs "Encodes AArch64 instructions into 32-bit machine code words";
    //   encoder/mod.rs:570-572 "cls" => encode_cls (scalar) / encode_neon_two_misc (RegArrangement);
    //   README.md:240 lists cls under Bit manipulation;
    //   ARM ARM Data-processing (1 source) CLS: sf 1 0 11010110 00000 000101 Rn Rd;
    //   register 31 is ZR not SP.
    // Stronger considered:
    //   - State machine: rejected — encode_cls is a pure function with no lifecycle
    //   - Algebraic round-trip: rejected — no in-tree CLS decoder
    //   - encode_clz as differential sibling: rejected — same-job gate fails (CLZ opcode 000100)
    // Weaker available: algebraic.metamorphic (Rd/Rn field independence; W vs X sf),
    //   algebraic.invariant (ARM fields), negative_error (arity / extra / SP / mixed / FP)
    // Differential: candidate=encode_cls, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=[Reg(Rd), Reg(Rn)] <-> `cls Rd, Rn`

    use super::encode_cls;
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
        if is_64 {
            if n == 31 {
                "xzr".into()
            } else {
                format!("x{n}")
            }
        } else if n == 31 {
            "wzr".into()
        } else {
            format!("w{n}")
        }
    }

    fn ops2(is_64: bool, rd: u32, rn: u32) -> [Operand; 2] {
        [Operand::Reg(gpr(is_64, rd)), Operand::Reg(gpr(is_64, rn))]
    }

    fn sut_word(ops: &[Operand]) -> Result<u32, String> {
        match encode_cls(ops)? {
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
            Just(Operand::Imm(-1)),
            Just(Operand::Shift {
                kind: "lsl".into(),
                amount: 0,
            }),
            Just(Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "8h".into(),
            }),
        ]
    }

    fn spell(is_64: bool, n: u32, kind: u32) -> String {
        match kind {
            0 if n == 31 => {
                if is_64 {
                    "x31".into()
                } else {
                    "w31".into()
                }
            }
            1 if n == 31 => {
                if is_64 {
                    "XZR".into()
                } else {
                    "WZR".into()
                }
            }
            2 if n == 30 && is_64 => "LR".into(),
            3 => gpr(is_64, n).to_uppercase(),
            _ => gpr(is_64, n),
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
            "x-1".into(),
            "x99".into(),
            "w".into(),
        ])
    }

    fn non_reg_operand() -> impl Strategy<Value = Operand> {
        prop_oneof![
            any::<i64>().prop_map(Operand::Imm),
            Just(Operand::Shift {
                kind: "lsl".into(),
                amount: 0,
            }),
            Just(Operand::Mem {
                base: "x0".into(),
                offset: 0,
            }),
            Just(Operand::Label("L0".into())),
            Just(Operand::Symbol("foo".into())),
            Just(Operand::Cond("eq".into())),
            Just(Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "8b".into(),
            }),
        ]
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_cls_kat_llvm_mc_w0_w1() {
        let want = 0x5ac01420u32;
        let mc = llvm_mc_word("cls w0, w1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops2(false, 0, 1)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_cls_kat_llvm_mc_x0_x1() {
        let want = 0xdac01420u32;
        let mc = llvm_mc_word("cls x0, x1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops2(true, 0, 1)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_cls_kat_llvm_mc_wzr_wzr() {
        let want = 0x5ac017ffu32;
        let mc = llvm_mc_word("cls wzr, wzr").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops2(false, 31, 31)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_cls_kat_llvm_mc_xzr_xzr() {
        let want = 0xdac017ffu32;
        let mc = llvm_mc_word("cls xzr, xzr").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops2(true, 31, 31)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_cls_kat_llvm_mc_lr_x1() {
        let want = 0xdac0143eu32;
        let mc = llvm_mc_word("cls lr, x1").expect("llvm-mc LR KAT");
        assert_eq!(mc, want, "llvm-mc LR KAT mapping broken");
        let ops = [Operand::Reg("lr".into()), Operand::Reg("x1".into())];
        let sut = sut_word(&ops).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_cls_kat_llvm_mc_x0_xzr() {
        let want = 0xdac017e0u32;
        let mc = llvm_mc_word("cls x0, xzr").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops2(true, 0, 31)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    proptest! {
        #![proptest_config(cfg())]

        #[test]
        fn encode_cls_diff_valid_gpr(is_64 in any::<bool>(), rd in 0u32..=31, rn in 0u32..=31) {
            let dest = gpr(is_64, rd);
            let src = gpr(is_64, rn);
            let asm = format!("cls {}, {}", dest, src);
            let ops = ops2(is_64, rd, rn);
            let mc = llvm_mc_word(&asm).expect("llvm-mc");
            let sut = sut_word(&ops).expect("SUT");
            prop_assert_eq!(sut, mc, "CLS mismatch for {}", asm);
        }

        #[test]
        fn encode_cls_arm_fields(is_64 in any::<bool>(), rd in 0u32..=31, rn in 0u32..=31) {
            let sf = if is_64 { 1u32 } else { 0 };
            let w = sut_word(&ops2(is_64, rd, rn)).expect("SUT");
            let want = (sf << 31)
                | (1u32 << 30)
                | (0b011010110 << 21)
                | (0b000101 << 10)
                | (rn << 5)
                | rd;
            prop_assert_eq!(w, want, "ARM ARM CLS field layout");
            prop_assert_eq!(w >> 31, sf, "sf");
            prop_assert_eq!((w >> 30) & 1, 1, "bit30=1");
            prop_assert_eq!((w >> 29) & 1, 0, "S=0");
            prop_assert_eq!((w >> 21) & 0xff, 0b11010110, "bits[28:21]");
            prop_assert_eq!((w >> 16) & 0x1f, 0, "opcode2=00000");
            prop_assert_eq!((w >> 10) & 0x3f, 0b000101, "opcode=000101 CLS");
            prop_assert_eq!((w >> 5) & 0x1f, rn, "Rn");
            prop_assert_eq!(w & 0x1f, rd, "Rd");
        }

        #[test]
        fn encode_cls_metamorphic_rd_rn(
            is_64 in any::<bool>(),
            rd in 0u32..=30,
            rn in 0u32..=30,
        ) {
            let base = sut_word(&ops2(is_64, rd, rn)).expect("base");
            let w_rd = sut_word(&ops2(is_64, rd + 1, rn)).expect("rd+1");
            let w_rn = sut_word(&ops2(is_64, rd, rn + 1)).expect("rn+1");
            prop_assert_eq!(w_rd & 0x1f, rd + 1, "Rd+1 updates Rd field");
            prop_assert_eq!(w_rd & !0x1fu32, base & !0x1fu32, "Rd+1 leaves other fields unchanged");
            prop_assert_eq!((w_rn >> 5) & 0x1f, rn + 1, "Rn+1 updates Rn field");
            prop_assert_eq!(w_rn & !(0x1fu32 << 5), base & !(0x1fu32 << 5), "Rn+1 leaves other fields unchanged");
            let w_sf = sut_word(&ops2(!is_64, rd, rn)).expect("sf flip");
            prop_assert_eq!(w_sf ^ base, 1u32 << 31, "W vs X flips only sf");
        }

        #[test]
        fn encode_cls_neg_arity(
            len in 0usize..=1,
            is_64 in any::<bool>(),
            rd in 0u32..=31,
            rn in 0u32..=31,
        ) {
            let mut ops = vec![
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
            ];
            ops.truncate(len);
            prop_assert!(
                encode_cls(&ops).is_err(),
                "CLS with {} operands must Err (llvm-mc: too few operands)",
                len
            );
        }

        #[test]
        fn encode_cls_neg_extra_operand(
            is_64 in any::<bool>(),
            rd in 0u32..=31,
            rn in 0u32..=31,
            extra in extra_operand(),
        ) {
            let mut ops = ops2(is_64, rd, rn).to_vec();
            ops.push(extra);
            prop_assert!(
                encode_cls(&ops).is_err(),
                "CLS has no 3rd operand; extra operand must Err (llvm-mc rejects it)"
            );
        }

        #[test]
        fn encode_cls_neg_sp(
            which in 0u32..=1,
            sp64 in any::<bool>(),
            is_64 in any::<bool>(),
            other in 0u32..=30,
        ) {
            let sp = if sp64 { "sp" } else { "wsp" };
            let mut ops = ops2(is_64, other, other);
            ops[which as usize] = Operand::Reg(sp.to_string());
            prop_assert!(
                encode_cls(&ops).is_err(),
                "SP/WSP is not a valid CLS operand (which={} sp={})",
                which,
                sp
            );
        }

        #[test]
        fn encode_cls_neg_mixed_width(
            rd in 0u32..=31,
            rn in 0u32..=31,
            rd64 in any::<bool>(),
            rn64 in any::<bool>(),
        ) {
            prop_assume!(rd64 != rn64);
            let ops = [
                Operand::Reg(gpr(rd64, rd)),
                Operand::Reg(gpr(rn64, rn)),
            ];
            prop_assert!(
                encode_cls(&ops).is_err(),
                "CLS mixed W/X (rd64={} rn64={}) must Err (llvm-mc rejects it)",
                rd64,
                rn64
            );
        }

        #[test]
        fn encode_cls_neg_fp(
            which in 0u32..=1,
            prefix in prop::sample::select(vec!["d", "s", "q", "v", "h", "b"]),
            n in 0u32..=31,
        ) {
            let fp = format!("{}{}", prefix, n);
            let mut ops = ops2(true, 0, 1);
            ops[which as usize] = Operand::Reg(fp.clone());
            prop_assert!(
                encode_cls(&ops).is_err(),
                "FP/SIMD register {} is not a valid CLS operand (which={})",
                fp,
                which
            );
        }

        #[test]
        fn encode_cls_diff_alt_spellings(
            is_64 in any::<bool>(),
            rd in 0u32..=31,
            rn in 0u32..=31,
            dest_spell in 0u32..=4,
            src_spell in 0u32..=4,
        ) {
            let dest = spell(is_64, rd, dest_spell);
            let src = spell(is_64, rn, src_spell);
            let asm = format!("cls {}, {}", dest, src);
            let ops = [Operand::Reg(dest), Operand::Reg(src)];
            let mc = llvm_mc_word(&asm).expect("llvm-mc");
            let sut = sut_word(&ops).expect("SUT");
            prop_assert_eq!(sut, mc, "CLS alt-spelling mismatch for {}", asm);
        }

        #[test]
        fn encode_cls_neg_nonreg(
            which in 0u32..=1,
            bad in non_reg_operand(),
        ) {
            let mut ops = ops2(false, 0, 1).to_vec();
            ops[which as usize] = bad;
            prop_assert!(
                encode_cls(&ops).is_err(),
                "wrong operand kind at slot {} must Err",
                which
            );
        }

        #[test]
        fn encode_cls_neg_invalid_name(
            which in 0u32..=1,
            name in invalid_name(),
        ) {
            let mut ops = ops2(false, 0, 1);
            ops[which as usize] = Operand::Reg(name.clone());
            prop_assert!(
                encode_cls(&ops).is_err(),
                "invalid register name {:?} at slot {} must Err",
                name,
                which
            );
        }
    }

    #[test]
    fn test_encode_cls_regression_extra_operand() {
        let mut ops = ops2(false, 0, 0).to_vec();
        ops.push(Operand::Reg("x0".into()));
        assert!(
            encode_cls(&ops).is_err(),
            "CLS w0, w0, x0 must Err; extra operand is invalid (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_cls_regression_sp() {
        let ops = [Operand::Reg("wsp".into()), Operand::Reg("w0".into())];
        assert!(
            encode_cls(&ops).is_err(),
            "CLS wsp, w0 must Err; register 31 is ZR not SP/WSP (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_cls_regression_mixed_width() {
        let ops = [Operand::Reg("x0".into()), Operand::Reg("w0".into())];
        assert!(
            encode_cls(&ops).is_err(),
            "CLS x0, w0 must Err; mixed W/X is invalid (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_cls_regression_fp() {
        let ops = [Operand::Reg("d0".into()), Operand::Reg("x1".into())];
        assert!(
            encode_cls(&ops).is_err(),
            "CLS d0, x1 must Err; FP/SIMD registers are not CLS operands (llvm-mc rejects it)"
        );
    }
}

#[cfg(test)]
mod encode_clz_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler
    // Evidence: src/backend/arm/assembler/README.md:11 "accepts the same textual assembly that GCC's gas would consume";
    //   encoder/mod.rs "Encodes AArch64 instructions into 32-bit machine code words";
    //   encoder/mod.rs:573-575 "clz" => encode_clz (scalar) / encode_neon_two_misc (RegArrangement);
    //   README.md:240 lists clz under Bit manipulation;
    //   ARM ARM Data-processing (1 source) CLZ: sf 1 0 11010110 00000 000100 Rn Rd;
    //   register 31 is ZR not SP.
    // Stronger considered:
    //   - State machine: rejected — encode_clz is a pure function with no lifecycle
    //   - Algebraic round-trip: rejected — no in-tree CLZ decoder
    //   - encode_cls as differential sibling: rejected — same-job gate fails (CLS opcode 000101)
    // Weaker available: algebraic.metamorphic (Rd/Rn field independence; W vs X sf),
    //   algebraic.invariant (ARM fields), negative_error (arity / extra / SP / mixed / FP)
    // Differential: candidate=encode_clz, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=[Reg(Rd), Reg(Rn)] <-> `clz Rd, Rn`

    use super::encode_clz;
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
        if is_64 {
            if n == 31 {
                "xzr".into()
            } else {
                format!("x{n}")
            }
        } else if n == 31 {
            "wzr".into()
        } else {
            format!("w{n}")
        }
    }

    fn ops2(is_64: bool, rd: u32, rn: u32) -> [Operand; 2] {
        [Operand::Reg(gpr(is_64, rd)), Operand::Reg(gpr(is_64, rn))]
    }

    fn sut_word(ops: &[Operand]) -> Result<u32, String> {
        match encode_clz(ops)? {
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
            Just(Operand::Imm(-1)),
            Just(Operand::Shift {
                kind: "lsl".into(),
                amount: 0,
            }),
            Just(Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "8h".into(),
            }),
        ]
    }

    fn spell(is_64: bool, n: u32, kind: u32) -> String {
        match kind {
            0 if n == 31 => {
                if is_64 {
                    "x31".into()
                } else {
                    "w31".into()
                }
            }
            1 if n == 31 => {
                if is_64 {
                    "XZR".into()
                } else {
                    "WZR".into()
                }
            }
            2 if n == 30 && is_64 => "LR".into(),
            3 => gpr(is_64, n).to_uppercase(),
            _ => gpr(is_64, n),
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
            "x-1".into(),
            "x99".into(),
            "w".into(),
        ])
    }

    fn non_reg_operand() -> impl Strategy<Value = Operand> {
        prop_oneof![
            any::<i64>().prop_map(Operand::Imm),
            Just(Operand::Shift {
                kind: "lsl".into(),
                amount: 0,
            }),
            Just(Operand::Mem {
                base: "x0".into(),
                offset: 0,
            }),
            Just(Operand::Label("L0".into())),
            Just(Operand::Symbol("foo".into())),
            Just(Operand::Cond("eq".into())),
            Just(Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "8b".into(),
            }),
        ]
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_clz_kat_llvm_mc_w0_w1() {
        let want = 0x5ac01020u32;
        let mc = llvm_mc_word("clz w0, w1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops2(false, 0, 1)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_clz_kat_llvm_mc_x0_x1() {
        let want = 0xdac01020u32;
        let mc = llvm_mc_word("clz x0, x1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops2(true, 0, 1)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_clz_kat_llvm_mc_wzr_wzr() {
        let want = 0x5ac013ffu32;
        let mc = llvm_mc_word("clz wzr, wzr").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops2(false, 31, 31)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_clz_kat_llvm_mc_xzr_xzr() {
        let want = 0xdac013ffu32;
        let mc = llvm_mc_word("clz xzr, xzr").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops2(true, 31, 31)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_clz_kat_llvm_mc_lr_x1() {
        let want = 0xdac0103eu32;
        let mc = llvm_mc_word("clz lr, x1").expect("llvm-mc LR KAT");
        assert_eq!(mc, want, "llvm-mc LR KAT mapping broken");
        let ops = [Operand::Reg("lr".into()), Operand::Reg("x1".into())];
        let sut = sut_word(&ops).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_clz_kat_llvm_mc_x0_xzr() {
        let want = 0xdac013e0u32;
        let mc = llvm_mc_word("clz x0, xzr").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops2(true, 0, 31)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    proptest! {
        #![proptest_config(cfg())]

        #[test]
        fn encode_clz_diff_valid_gpr(is_64 in any::<bool>(), rd in 0u32..=31, rn in 0u32..=31) {
            let dest = gpr(is_64, rd);
            let src = gpr(is_64, rn);
            let asm = format!("clz {}, {}", dest, src);
            let ops = ops2(is_64, rd, rn);
            let mc = llvm_mc_word(&asm).expect("llvm-mc");
            let sut = sut_word(&ops).expect("SUT");
            prop_assert_eq!(sut, mc, "CLZ mismatch for {}", asm);
        }

        #[test]
        fn encode_clz_arm_fields(is_64 in any::<bool>(), rd in 0u32..=31, rn in 0u32..=31) {
            let sf = if is_64 { 1u32 } else { 0 };
            let w = sut_word(&ops2(is_64, rd, rn)).expect("SUT");
            let want = (sf << 31)
                | (1u32 << 30)
                | (0b011010110 << 21)
                | (0b000100 << 10)
                | (rn << 5)
                | rd;
            prop_assert_eq!(w, want, "ARM ARM CLZ field layout");
            prop_assert_eq!(w >> 31, sf, "sf");
            prop_assert_eq!((w >> 30) & 1, 1, "bit30=1");
            prop_assert_eq!((w >> 29) & 1, 0, "S=0");
            prop_assert_eq!((w >> 21) & 0xff, 0b11010110, "bits[28:21]");
            prop_assert_eq!((w >> 16) & 0x1f, 0, "opcode2=00000");
            prop_assert_eq!((w >> 10) & 0x3f, 0b000100, "opcode=000100 CLZ");
            prop_assert_eq!((w >> 5) & 0x1f, rn, "Rn");
            prop_assert_eq!(w & 0x1f, rd, "Rd");
        }

        #[test]
        fn encode_clz_metamorphic_rd_rn(
            is_64 in any::<bool>(),
            rd in 0u32..=30,
            rn in 0u32..=30,
        ) {
            let base = sut_word(&ops2(is_64, rd, rn)).expect("base");
            let w_rd = sut_word(&ops2(is_64, rd + 1, rn)).expect("rd+1");
            let w_rn = sut_word(&ops2(is_64, rd, rn + 1)).expect("rn+1");
            prop_assert_eq!(w_rd & 0x1f, rd + 1, "Rd+1 updates Rd field");
            prop_assert_eq!(w_rd & !0x1fu32, base & !0x1fu32, "Rd+1 leaves other fields unchanged");
            prop_assert_eq!((w_rn >> 5) & 0x1f, rn + 1, "Rn+1 updates Rn field");
            prop_assert_eq!(w_rn & !(0x1fu32 << 5), base & !(0x1fu32 << 5), "Rn+1 leaves other fields unchanged");
            let w_sf = sut_word(&ops2(!is_64, rd, rn)).expect("sf flip");
            prop_assert_eq!(w_sf ^ base, 1u32 << 31, "W vs X flips only sf");
        }

        #[test]
        fn encode_clz_neg_arity(
            len in 0usize..=1,
            is_64 in any::<bool>(),
            rd in 0u32..=31,
            rn in 0u32..=31,
        ) {
            let mut ops = vec![
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
            ];
            ops.truncate(len);
            prop_assert!(
                encode_clz(&ops).is_err(),
                "CLZ with {} operands must Err (llvm-mc: too few operands)",
                len
            );
        }

        #[test]
        fn encode_clz_neg_extra_operand(
            is_64 in any::<bool>(),
            rd in 0u32..=31,
            rn in 0u32..=31,
            extra in extra_operand(),
        ) {
            let mut ops = ops2(is_64, rd, rn).to_vec();
            ops.push(extra);
            prop_assert!(
                encode_clz(&ops).is_err(),
                "CLZ has no 3rd operand; extra operand must Err (llvm-mc rejects it)"
            );
        }

        #[test]
        fn encode_clz_neg_sp(
            which in 0u32..=1,
            sp64 in any::<bool>(),
            is_64 in any::<bool>(),
            other in 0u32..=30,
        ) {
            let sp = if sp64 { "sp" } else { "wsp" };
            let mut ops = ops2(is_64, other, other);
            ops[which as usize] = Operand::Reg(sp.to_string());
            prop_assert!(
                encode_clz(&ops).is_err(),
                "SP/WSP is not a valid CLZ operand (which={} sp={})",
                which,
                sp
            );
        }

        #[test]
        fn encode_clz_neg_mixed_width(
            rd in 0u32..=31,
            rn in 0u32..=31,
            rd64 in any::<bool>(),
            rn64 in any::<bool>(),
        ) {
            prop_assume!(rd64 != rn64);
            let ops = [
                Operand::Reg(gpr(rd64, rd)),
                Operand::Reg(gpr(rn64, rn)),
            ];
            prop_assert!(
                encode_clz(&ops).is_err(),
                "CLZ mixed W/X (rd64={} rn64={}) must Err (llvm-mc rejects it)",
                rd64,
                rn64
            );
        }

        #[test]
        fn encode_clz_neg_fp(
            which in 0u32..=1,
            prefix in prop::sample::select(vec!["d", "s", "q", "v", "h", "b"]),
            n in 0u32..=31,
        ) {
            let fp = format!("{}{}", prefix, n);
            let mut ops = ops2(true, 0, 1);
            ops[which as usize] = Operand::Reg(fp.clone());
            prop_assert!(
                encode_clz(&ops).is_err(),
                "FP/SIMD register {} is not a valid CLZ operand (which={})",
                fp,
                which
            );
        }

        #[test]
        fn encode_clz_diff_alt_spellings(
            is_64 in any::<bool>(),
            rd in 0u32..=31,
            rn in 0u32..=31,
            dest_spell in 0u32..=4,
            src_spell in 0u32..=4,
        ) {
            let dest = spell(is_64, rd, dest_spell);
            let src = spell(is_64, rn, src_spell);
            let asm = format!("clz {}, {}", dest, src);
            let ops = [Operand::Reg(dest), Operand::Reg(src)];
            let mc = llvm_mc_word(&asm).expect("llvm-mc");
            let sut = sut_word(&ops).expect("SUT");
            prop_assert_eq!(sut, mc, "CLZ alt-spelling mismatch for {}", asm);
        }

        #[test]
        fn encode_clz_neg_nonreg(
            which in 0u32..=1,
            bad in non_reg_operand(),
        ) {
            let mut ops = ops2(false, 0, 1).to_vec();
            ops[which as usize] = bad;
            prop_assert!(
                encode_clz(&ops).is_err(),
                "wrong operand kind at slot {} must Err",
                which
            );
        }

        #[test]
        fn encode_clz_neg_invalid_name(
            which in 0u32..=1,
            name in invalid_name(),
        ) {
            let mut ops = ops2(false, 0, 1);
            ops[which as usize] = Operand::Reg(name.clone());
            prop_assert!(
                encode_clz(&ops).is_err(),
                "invalid register name {:?} at slot {} must Err",
                name,
                which
            );
        }
    }

    #[test]
    fn test_encode_clz_regression_extra_operand() {
        let mut ops = ops2(false, 0, 0).to_vec();
        ops.push(Operand::Reg("x0".into()));
        assert!(
            encode_clz(&ops).is_err(),
            "CLZ w0, w0, x0 must Err; extra operand is invalid (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_clz_regression_sp() {
        let ops = [Operand::Reg("wsp".into()), Operand::Reg("w0".into())];
        assert!(
            encode_clz(&ops).is_err(),
            "CLZ wsp, w0 must Err; register 31 is ZR not SP/WSP (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_clz_regression_mixed_width() {
        let ops = [Operand::Reg("x0".into()), Operand::Reg("w0".into())];
        assert!(
            encode_clz(&ops).is_err(),
            "CLZ x0, w0 must Err; mixed W/X is invalid (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_clz_regression_fp() {
        let ops = [Operand::Reg("d0".into()), Operand::Reg("x1".into())];
        assert!(
            encode_clz(&ops).is_err(),
            "CLZ d0, x1 must Err; FP/SIMD registers are not CLZ operands (llvm-mc rejects it)"
        );
    }
}

#[cfg(test)]
mod encode_extr_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler
    // Evidence: src/backend/arm/assembler/README.md:11 "accepts the same textual assembly that GCC's gas would consume";
    //   encoder/mod.rs "Encodes AArch64 instructions into 32-bit machine code words";
    //   encoder/mod.rs:894 "extr" => encode_extr; README.md:216 lists extr;
    //   bitfield.rs:132 purpose comment EXTR Rd, Rn, Rm, #lsb;
    //   ARM ARM Extract EXTR: sf 00 100111 N 0 Rm imms Rn Rd, N=sf, imms=lsb;
    //   0 <= lsb <= 31 (W) / 63 (X); register 31 is ZR not SP.
    //   ROR (immediate) is the ARM alias of EXTR when Rn=Rm.
    // Stronger considered:
    //   - State machine: rejected — encode_extr is a pure function with no lifecycle
    //   - Algebraic round-trip: rejected — no in-tree EXTR decoder
    //   - encode_shift ROR as differential sibling: rejected — same-job gate fails
    //     (ROR is a 3-operand shift mnemonic; EXTR is 4-operand extract);
    //     used only as algebraic alias after the ARM mapping when Rn=Rm
    //   - encode_ubfx / encode_sbfx / encode_bfm: rejected — different opc / class
    // Weaker available: algebraic.metamorphic (ROR alias when Rn=Rm; Rd/Rn/Rm field independence),
    //   algebraic.invariant (ARM fields), negative_error (arity / extra / SP / lsb)
    // Differential: candidate=encode_extr, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=[Reg(Rd), Reg(Rn), Reg(Rm), Imm(lsb)] <-> `extr Rd, Rn, Rm, #lsb`

    use super::encode_extr;
    use super::super::EncodeResult;
    use crate::backend::arm::assembler::parser::Operand;
    use proptest::prelude::*;
    use std::io::Write;
    use std::panic::{catch_unwind, AssertUnwindSafe};
    use std::process::{Command, Stdio};

    const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
    const CASES: u32 = 1000;

    fn cfg() -> ProptestConfig {
        ProptestConfig::with_cases(CASES)
    }

    fn gpr(is_64: bool, n: u32) -> String {
        if is_64 {
            if n == 31 {
                "xzr".into()
            } else {
                format!("x{n}")
            }
        } else if n == 31 {
            "wzr".into()
        } else {
            format!("w{n}")
        }
    }

    fn lsb_valid(r: u32) -> impl Strategy<Value = u32> {
        prop_oneof![Just(0u32), Just(r - 1), 0u32..r]
    }

    fn extr_valid() -> impl Strategy<Value = (bool, u32, u32, u32, u32)> {
        any::<bool>().prop_flat_map(|is_64| {
            let r = if is_64 { 64u32 } else { 32 };
            (
                Just(is_64),
                0u32..=31,
                0u32..=31,
                0u32..=31,
                lsb_valid(r),
            )
                .prop_map(|(is_64, rd, rn, rm, lsb)| (is_64, rd, rn, rm, lsb))
        })
    }

    fn invalid_lsb(is_64: bool) -> impl Strategy<Value = i64> {
        let r = if is_64 { 64i64 } else { 32 };
        prop_oneof![
            Just(-1i64),
            Just(-2i64),
            Just(r),
            Just(r + 1),
            Just(128i64),
            Just(i64::MIN),
            Just(i64::MAX),
        ]
    }

    fn ops4(is_64: bool, rd: u32, rn: u32, rm: u32, lsb: i64) -> [Operand; 4] {
        [
            Operand::Reg(gpr(is_64, rd)),
            Operand::Reg(gpr(is_64, rn)),
            Operand::Reg(gpr(is_64, rm)),
            Operand::Imm(lsb),
        ]
    }

    fn sut_word(ops: &[Operand]) -> Result<u32, String> {
        match encode_extr(ops)? {
            EncodeResult::Word(w) => Ok(w),
            other => Err(format!("expected Word, got {:?}", other)),
        }
    }

    fn must_err(ops: &[Operand]) -> bool {
        match catch_unwind(AssertUnwindSafe(|| encode_extr(ops))) {
            Ok(Err(_)) => true,
            _ => false,
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
            Just(Operand::Imm(-1)),
            Just(Operand::Shift {
                kind: "lsl".into(),
                amount: 0,
            }),
            Just(Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "8h".into(),
            }),
        ]
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_extr_kat_llvm_mc_w0_w1_w2_lsb0() {
        let want = 0x13820020u32;
        let mc = llvm_mc_word("extr w0, w1, w2, #0").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops4(false, 0, 1, 2, 0)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_extr_kat_llvm_mc_w0_w1_w2_lsb1() {
        let want = 0x13820420u32;
        let mc = llvm_mc_word("extr w0, w1, w2, #1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops4(false, 0, 1, 2, 1)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_extr_kat_llvm_mc_w0_w1_w2_lsb31() {
        let want = 0x13827c20u32;
        let mc = llvm_mc_word("extr w0, w1, w2, #31").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops4(false, 0, 1, 2, 31)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_extr_kat_llvm_mc_x0_x1_x2_lsb0() {
        let want = 0x93c20020u32;
        let mc = llvm_mc_word("extr x0, x1, x2, #0").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops4(true, 0, 1, 2, 0)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_extr_kat_llvm_mc_x0_x1_x2_lsb63() {
        let want = 0x93c2fc20u32;
        let mc = llvm_mc_word("extr x0, x1, x2, #63").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops4(true, 0, 1, 2, 63)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_extr_kat_llvm_mc_wzr_wzr_wzr_lsb0() {
        let want = 0x139f03ffu32;
        let mc = llvm_mc_word("extr wzr, wzr, wzr, #0").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops4(false, 31, 31, 31, 0)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_extr_kat_llvm_mc_lr_x1_x2_lsb8() {
        let want = 0x93c2203eu32;
        let mc = llvm_mc_word("extr lr, x1, x2, #8").expect("llvm-mc LR KAT");
        assert_eq!(mc, want, "llvm-mc LR KAT mapping broken");
        let ops = [
            Operand::Reg("lr".into()),
            Operand::Reg("x1".into()),
            Operand::Reg("x2".into()),
            Operand::Imm(8),
        ];
        let sut = sut_word(&ops).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_extr_kat_llvm_mc_ror_alias_w0_w1_lsb1() {
        let want = 0x13810420u32;
        let mc = llvm_mc_word("ror w0, w1, #1").expect("llvm-mc ROR KAT");
        assert_eq!(mc, want, "llvm-mc ROR KAT mapping broken");
        let sut = sut_word(&ops4(false, 0, 1, 1, 1)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    proptest! {
        #![proptest_config(cfg())]

        #[test]
        fn encode_extr_diff_valid_gpr((is_64, rd, rn, rm, lsb) in extr_valid()) {
            let dest = gpr(is_64, rd);
            let src_n = gpr(is_64, rn);
            let src_m = gpr(is_64, rm);
            let asm = format!("extr {}, {}, {}, #{}", dest, src_n, src_m, lsb);
            let ops = ops4(is_64, rd, rn, rm, lsb as i64);
            let mc = llvm_mc_word(&asm).expect("llvm-mc");
            let sut = sut_word(&ops).expect("SUT");
            prop_assert_eq!(sut, mc, "EXTR mismatch for {}", asm);
        }

        #[test]
        fn encode_extr_arm_fields((is_64, rd, rn, rm, lsb) in extr_valid()) {
            let sf = if is_64 { 1u32 } else { 0 };
            let w = sut_word(&ops4(is_64, rd, rn, rm, lsb as i64)).expect("SUT");
            let want = (sf << 31)
                | (0b00100111 << 23)
                | (sf << 22)
                | (rm << 16)
                | (lsb << 10)
                | (rn << 5)
                | rd;
            prop_assert_eq!(w, want, "ARM ARM EXTR field layout");
            prop_assert_eq!(w >> 31, sf, "sf");
            prop_assert_eq!((w >> 23) & 0xff, 0b00100111, "bits[30:23]=00100111");
            prop_assert_eq!((w >> 22) & 1, sf, "N=sf");
            prop_assert_eq!((w >> 21) & 1, 0, "bit21=0");
            prop_assert_eq!((w >> 16) & 0x1f, rm, "Rm");
            prop_assert_eq!((w >> 10) & 0x3f, lsb, "imms=lsb");
            prop_assert_eq!((w >> 5) & 0x1f, rn, "Rn");
            prop_assert_eq!(w & 0x1f, rd, "Rd");
        }

        #[test]
        fn encode_extr_metamorphic_rd_rn_rm(
            is_64 in any::<bool>(),
            rd in 0u32..=30,
            rn in 0u32..=30,
            rm in 0u32..=30,
            lsb in 0u32..=1,
        ) {
            let r = if is_64 { 64u32 } else { 32 };
            prop_assume!(lsb < r);
            let base = sut_word(&ops4(is_64, rd, rn, rm, lsb as i64)).expect("base");
            let w_rd = sut_word(&ops4(is_64, rd + 1, rn, rm, lsb as i64)).expect("rd+1");
            let w_rn = sut_word(&ops4(is_64, rd, rn + 1, rm, lsb as i64)).expect("rn+1");
            let w_rm = sut_word(&ops4(is_64, rd, rn, rm + 1, lsb as i64)).expect("rm+1");
            prop_assert_eq!(w_rd & 0x1f, rd + 1, "Rd+1 updates Rd field");
            prop_assert_eq!(w_rd & !0x1fu32, base & !0x1fu32, "Rd+1 leaves other fields unchanged");
            prop_assert_eq!((w_rn >> 5) & 0x1f, rn + 1, "Rn+1 updates Rn field");
            prop_assert_eq!(w_rn & !(0x1fu32 << 5), base & !(0x1fu32 << 5), "Rn+1 leaves other fields unchanged");
            prop_assert_eq!((w_rm >> 16) & 0x1f, rm + 1, "Rm+1 updates Rm field");
            prop_assert_eq!(w_rm & !(0x1fu32 << 16), base & !(0x1fu32 << 16), "Rm+1 leaves other fields unchanged");
        }

        #[test]
        fn encode_extr_alias_ror((is_64, rd, rn, _rm, lsb) in extr_valid()) {
            let dest = gpr(is_64, rd);
            let src = gpr(is_64, rn);
            let asm = format!("ror {}, {}, #{}", dest, src, lsb);
            let ops = ops4(is_64, rd, rn, rn, lsb as i64);
            let mc = llvm_mc_word(&asm).expect("llvm-mc ror");
            let sut = sut_word(&ops).expect("SUT");
            prop_assert_eq!(sut, mc, "EXTR Rd,Rn,Rn,#lsb must alias ROR Rd,Rn,#lsb for {}", asm);
        }

        #[test]
        fn encode_extr_neg_arity(
            len in 0usize..=3,
            is_64 in any::<bool>(),
            rd in 0u32..=31,
            rn in 0u32..=31,
            rm in 0u32..=31,
        ) {
            let mut ops = vec![
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
                Operand::Reg(gpr(is_64, rm)),
                Operand::Imm(0),
            ];
            ops.truncate(len);
            prop_assert!(
                encode_extr(&ops).is_err(),
                "EXTR with {} operands must Err (llvm-mc: too few operands)",
                len
            );
        }

        #[test]
        fn encode_extr_neg_extra_operand(
            (is_64, rd, rn, rm, lsb) in extr_valid(),
            extra in extra_operand(),
        ) {
            let mut ops = ops4(is_64, rd, rn, rm, lsb as i64).to_vec();
            ops.push(extra);
            prop_assert!(
                encode_extr(&ops).is_err(),
                "EXTR has no 5th operand; extra operand must Err (llvm-mc rejects it)"
            );
        }

        #[test]
        fn encode_extr_neg_sp(
            which in 0u32..=2,
            sp64 in any::<bool>(),
            is_64 in any::<bool>(),
            other in 0u32..=30,
        ) {
            let sp = if sp64 { "sp" } else { "wsp" };
            let mut ops = ops4(is_64, other, other, other, 0);
            ops[which as usize] = Operand::Reg(sp.to_string());
            prop_assert!(
                encode_extr(&ops).is_err(),
                "SP/WSP is not a valid EXTR operand (which={} sp={})",
                which,
                sp
            );
        }

        #[test]
        fn encode_extr_neg_lsb(
            (is_64, rd, rn, rm, lsb) in any::<bool>().prop_flat_map(|is_64| {
                (Just(is_64), 0u32..=31, 0u32..=31, 0u32..=31, invalid_lsb(is_64))
                    .prop_map(|(is_64, rd, rn, rm, lsb)| (is_64, rd, rn, rm, lsb))
            }),
        ) {
            let r = if is_64 { 64i64 } else { 32 };
            prop_assume!(lsb < 0 || lsb >= r);
            let ops = ops4(is_64, rd, rn, rm, lsb);
            prop_assert!(
                must_err(&ops),
                "EXTR lsb={} R={} must Err (ARM: 0<=lsb<R)",
                lsb,
                r
            );
        }

        #[test]
        fn encode_extr_diff_alt_spellings(
            (is_64, rd, rn, rm, lsb) in extr_valid(),
            dest_spell in 0u32..=4,
            src_n_spell in 0u32..=4,
            src_m_spell in 0u32..=4,
        ) {
            let dest = spell(is_64, rd, dest_spell);
            let src_n = spell(is_64, rn, src_n_spell);
            let src_m = spell(is_64, rm, src_m_spell);
            let asm = format!("extr {}, {}, {}, #{}", dest, src_n, src_m, lsb);
            let ops = [
                Operand::Reg(dest),
                Operand::Reg(src_n),
                Operand::Reg(src_m),
                Operand::Imm(lsb as i64),
            ];
            let mc = llvm_mc_word(&asm).expect("llvm-mc");
            let sut = sut_word(&ops).expect("SUT");
            prop_assert_eq!(sut, mc, "EXTR alt-spelling mismatch for {}", asm);
        }

        #[test]
        fn encode_extr_neg_mixed_width(
            rd in 0u32..=31,
            rn in 0u32..=31,
            rm in 0u32..=31,
            rd64 in any::<bool>(),
            rn64 in any::<bool>(),
            rm64 in any::<bool>(),
        ) {
            prop_assume!(rd64 != rn64 || rn64 != rm64 || rd64 != rm64);
            let ops = [
                Operand::Reg(gpr(rd64, rd)),
                Operand::Reg(gpr(rn64, rn)),
                Operand::Reg(gpr(rm64, rm)),
                Operand::Imm(0),
            ];
            prop_assert!(
                encode_extr(&ops).is_err(),
                "EXTR mixed W/X (rd64={} rn64={} rm64={}) must Err (llvm-mc rejects it)",
                rd64,
                rn64,
                rm64
            );
        }

        #[test]
        fn encode_extr_neg_fp(
            which in 0u32..=2,
            prefix in prop::sample::select(vec!["d", "s", "q", "v", "h", "b"]),
            n in 0u32..=31,
        ) {
            let fp = format!("{}{}", prefix, n);
            let mut ops = ops4(true, 0, 1, 2, 0);
            ops[which as usize] = Operand::Reg(fp.clone());
            prop_assert!(
                encode_extr(&ops).is_err(),
                "FP/SIMD register {} is not a valid EXTR operand (which={})",
                fp,
                which
            );
        }

        #[test]
        fn encode_extr_neg_nonreg(
            which in 0u32..=3,
            bad in non_reg_operand(),
        ) {
            if which == 3 {
                prop_assume!(!matches!(bad, Operand::Imm(_)));
            }
            let mut ops = ops4(false, 0, 1, 2, 0).to_vec();
            ops[which as usize] = bad;
            prop_assert!(
                encode_extr(&ops).is_err(),
                "wrong operand kind at slot {} must Err",
                which
            );
        }

        #[test]
        fn encode_extr_neg_invalid_name(
            which in 0u32..=2,
            name in invalid_name(),
        ) {
            let mut ops = ops4(false, 0, 1, 2, 0);
            ops[which as usize] = Operand::Reg(name.clone());
            prop_assert!(
                encode_extr(&ops).is_err(),
                "invalid register name {:?} at slot {} must Err",
                name,
                which
            );
        }
    }

    fn spell(is_64: bool, n: u32, kind: u32) -> String {
        match kind {
            0 if n == 31 => {
                if is_64 {
                    "x31".into()
                } else {
                    "w31".into()
                }
            }
            1 if n == 31 => {
                if is_64 {
                    "XZR".into()
                } else {
                    "WZR".into()
                }
            }
            2 if n == 30 && is_64 => "LR".into(),
            3 => gpr(is_64, n).to_uppercase(),
            _ => gpr(is_64, n),
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
            "x-1".into(),
            "x99".into(),
            "w".into(),
        ])
    }

    fn non_reg_operand() -> impl Strategy<Value = Operand> {
        prop_oneof![
            any::<i64>().prop_map(Operand::Imm),
            Just(Operand::Shift {
                kind: "lsl".into(),
                amount: 0,
            }),
            Just(Operand::Mem {
                base: "x0".into(),
                offset: 0,
            }),
            Just(Operand::Label("L0".into())),
            Just(Operand::Symbol("foo".into())),
            Just(Operand::Cond("eq".into())),
            Just(Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "8b".into(),
            }),
        ]
    }

    #[test]
    fn test_encode_extr_regression_extra_operand() {
        let mut ops = ops4(false, 0, 0, 0, 0).to_vec();
        ops.push(Operand::Reg("x0".into()));
        assert!(
            encode_extr(&ops).is_err(),
            "EXTR w0, w0, w0, #0, x0 must Err; extra operand is invalid (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_extr_regression_sp() {
        let ops = [
            Operand::Reg("wsp".into()),
            Operand::Reg("w0".into()),
            Operand::Reg("w0".into()),
            Operand::Imm(0),
        ];
        assert!(
            encode_extr(&ops).is_err(),
            "EXTR wsp, w0, w0, #0 must Err; register 31 is ZR not SP/WSP (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_extr_regression_lsb_neg() {
        let ops = ops4(false, 0, 0, 0, -1);
        let result = catch_unwind(AssertUnwindSafe(|| encode_extr(&ops)));
        assert!(
            matches!(result, Ok(Err(_))),
            "EXTR w0, w0, w0, #-1 must Err; lsb=-1 is outside 0..=31 (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_extr_regression_mixed_width() {
        let ops = [
            Operand::Reg("w0".into()),
            Operand::Reg("x0".into()),
            Operand::Reg("w0".into()),
            Operand::Imm(0),
        ];
        assert!(
            encode_extr(&ops).is_err(),
            "EXTR w0, x0, w0, #0 must Err; mixed W/X is invalid (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_extr_regression_fp() {
        let ops = [
            Operand::Reg("d0".into()),
            Operand::Reg("x1".into()),
            Operand::Reg("x2".into()),
            Operand::Imm(0),
        ];
        assert!(
            encode_extr(&ops).is_err(),
            "EXTR d0, x1, x2, #0 must Err; FP/SIMD registers are not EXTR operands (llvm-mc rejects it)"
        );
    }
}

#[cfg(test)]
mod encode_rbit_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler
    // Evidence: src/backend/arm/assembler/README.md:11 "accepts the same textual assembly that GCC's gas would consume";
    //   encoder/mod.rs "Encodes AArch64 instructions into 32-bit machine code words";
    //   encoder/mod.rs:902-909 "rbit" => encode_neon_rbit (RegArrangement) / encode_rbit (scalar);
    //   README.md:240 lists rbit under Bit manipulation;
    //   ARM ARM Data-processing (1 source) RBIT: sf 1 0 11010110 00000 000000 Rn Rd;
    //   register 31 is ZR not SP.
    // Stronger considered:
    //   - State machine: rejected — encode_rbit is a pure function with no lifecycle
    //   - Algebraic round-trip: rejected — no in-tree RBIT decoder
    //   - encode_neon_rbit as differential sibling: rejected — same-job gate fails
    //     (vector/RegArrangement, different ARM class; dispatch already splits the two)
    //   - encode_clz / encode_cls / encode_rev: rejected — different opcode 000100 / 000101 / 000010
    // Weaker available: algebraic.metamorphic (Rd/Rn field independence; W vs X sf),
    //   algebraic.invariant (ARM fields), negative_error (arity / extra / SP / mixed / FP)
    // Differential: candidate=encode_rbit, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=[Reg(Rd), Reg(Rn)] <-> `rbit Rd, Rn`

    use super::encode_rbit;
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
        if is_64 {
            if n == 31 {
                "xzr".into()
            } else {
                format!("x{n}")
            }
        } else if n == 31 {
            "wzr".into()
        } else {
            format!("w{n}")
        }
    }

    fn ops2(is_64: bool, rd: u32, rn: u32) -> [Operand; 2] {
        [Operand::Reg(gpr(is_64, rd)), Operand::Reg(gpr(is_64, rn))]
    }

    fn sut_word(ops: &[Operand]) -> Result<u32, String> {
        match encode_rbit(ops)? {
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
            Just(Operand::Imm(-1)),
            Just(Operand::Shift {
                kind: "lsl".into(),
                amount: 0,
            }),
            Just(Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "8h".into(),
            }),
        ]
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_rbit_kat_llvm_mc_w0_w1() {
        let want = 0x5ac00020u32;
        let mc = llvm_mc_word("rbit w0, w1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops2(false, 0, 1)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_rbit_kat_llvm_mc_x0_x1() {
        let want = 0xdac00020u32;
        let mc = llvm_mc_word("rbit x0, x1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops2(true, 0, 1)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_rbit_kat_llvm_mc_wzr_wzr() {
        let want = 0x5ac003ffu32;
        let mc = llvm_mc_word("rbit wzr, wzr").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops2(false, 31, 31)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_rbit_kat_llvm_mc_xzr_xzr() {
        let want = 0xdac003ffu32;
        let mc = llvm_mc_word("rbit xzr, xzr").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops2(true, 31, 31)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_rbit_kat_llvm_mc_lr_x1() {
        let want = 0xdac0003eu32;
        let mc = llvm_mc_word("rbit lr, x1").expect("llvm-mc LR KAT");
        assert_eq!(mc, want, "llvm-mc LR KAT mapping broken");
        let ops = [Operand::Reg("lr".into()), Operand::Reg("x1".into())];
        let sut = sut_word(&ops).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_rbit_kat_llvm_mc_x0_xzr() {
        let want = 0xdac003e0u32;
        let mc = llvm_mc_word("rbit x0, xzr").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops2(true, 0, 31)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    proptest! {
        #![proptest_config(cfg())]

        #[test]
        fn encode_rbit_diff_valid_gpr(is_64 in any::<bool>(), rd in 0u32..=31, rn in 0u32..=31) {
            let dest = gpr(is_64, rd);
            let src = gpr(is_64, rn);
            let asm = format!("rbit {}, {}", dest, src);
            let ops = ops2(is_64, rd, rn);
            let mc = llvm_mc_word(&asm).expect("llvm-mc");
            let sut = sut_word(&ops).expect("SUT");
            prop_assert_eq!(sut, mc, "RBIT mismatch for {}", asm);
        }

        #[test]
        fn encode_rbit_arm_fields(is_64 in any::<bool>(), rd in 0u32..=31, rn in 0u32..=31) {
            let sf = if is_64 { 1u32 } else { 0 };
            let w = sut_word(&ops2(is_64, rd, rn)).expect("SUT");
            let want = (sf << 31)
                | (1u32 << 30)
                | (0b011010110 << 21)
                | (rn << 5)
                | rd;
            prop_assert_eq!(w, want, "ARM ARM RBIT field layout");
            prop_assert_eq!(w >> 31, sf, "sf");
            prop_assert_eq!((w >> 30) & 1, 1, "bit30=1");
            prop_assert_eq!((w >> 29) & 1, 0, "S=0");
            prop_assert_eq!((w >> 21) & 0xff, 0b11010110, "bits[28:21]");
            prop_assert_eq!((w >> 16) & 0x1f, 0, "opcode2=00000");
            prop_assert_eq!((w >> 10) & 0x3f, 0, "opcode=000000 RBIT");
            prop_assert_eq!((w >> 5) & 0x1f, rn, "Rn");
            prop_assert_eq!(w & 0x1f, rd, "Rd");
        }

        #[test]
        fn encode_rbit_metamorphic_rd_rn_sf(
            is_64 in any::<bool>(),
            rd in 0u32..=30,
            rn in 0u32..=30,
        ) {
            let base = sut_word(&ops2(is_64, rd, rn)).expect("base");
            let w_rd = sut_word(&ops2(is_64, rd + 1, rn)).expect("rd+1");
            let w_rn = sut_word(&ops2(is_64, rd, rn + 1)).expect("rn+1");
            prop_assert_eq!(w_rd & 0x1f, rd + 1, "Rd+1 updates Rd field");
            prop_assert_eq!(w_rd & !0x1fu32, base & !0x1fu32, "Rd+1 leaves other fields unchanged");
            prop_assert_eq!((w_rn >> 5) & 0x1f, rn + 1, "Rn+1 updates Rn field");
            prop_assert_eq!(w_rn & !(0x1fu32 << 5), base & !(0x1fu32 << 5), "Rn+1 leaves other fields unchanged");
            let w_sf = sut_word(&ops2(!is_64, rd, rn)).expect("sf flip");
            prop_assert_eq!(w_sf ^ base, 1u32 << 31, "W vs X flips only sf");
        }

        #[test]
        fn encode_rbit_neg_arity(
            len in 0usize..=1,
            is_64 in any::<bool>(),
            rd in 0u32..=31,
            rn in 0u32..=31,
        ) {
            let mut ops = vec![
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
            ];
            ops.truncate(len);
            prop_assert!(
                encode_rbit(&ops).is_err(),
                "RBIT with {} operands must Err (llvm-mc: too few operands)",
                len
            );
        }

        #[test]
        fn encode_rbit_neg_extra_operand(
            is_64 in any::<bool>(),
            rd in 0u32..=31,
            rn in 0u32..=31,
            extra in extra_operand(),
        ) {
            let mut ops = ops2(is_64, rd, rn).to_vec();
            ops.push(extra);
            prop_assert!(
                encode_rbit(&ops).is_err(),
                "RBIT has no 3rd operand; extra operand must Err (llvm-mc rejects it)"
            );
        }

        #[test]
        fn encode_rbit_neg_sp(
            which in 0u32..=1,
            sp64 in any::<bool>(),
            is_64 in any::<bool>(),
            other in 0u32..=30,
        ) {
            let sp = if sp64 { "sp" } else { "wsp" };
            let mut ops = ops2(is_64, other, other);
            ops[which as usize] = Operand::Reg(sp.to_string());
            prop_assert!(
                encode_rbit(&ops).is_err(),
                "SP/WSP is not a valid RBIT operand (which={} sp={})",
                which,
                sp
            );
        }

        #[test]
        fn encode_rbit_neg_mixed_width(
            rd in 0u32..=31,
            rn in 0u32..=31,
            rd64 in any::<bool>(),
            rn64 in any::<bool>(),
        ) {
            prop_assume!(rd64 != rn64);
            let ops = [
                Operand::Reg(gpr(rd64, rd)),
                Operand::Reg(gpr(rn64, rn)),
            ];
            prop_assert!(
                encode_rbit(&ops).is_err(),
                "RBIT mixed W/X (rd64={} rn64={}) must Err (llvm-mc rejects it)",
                rd64,
                rn64
            );
        }

        #[test]
        fn encode_rbit_neg_fp(
            which in 0u32..=1,
            prefix in prop::sample::select(vec!["d", "s", "q", "v", "h", "b"]),
            n in 0u32..=31,
        ) {
            let fp = format!("{}{}", prefix, n);
            let mut ops = ops2(true, 0, 1);
            ops[which as usize] = Operand::Reg(fp.clone());
            prop_assert!(
                encode_rbit(&ops).is_err(),
                "FP/SIMD register {} is not a valid RBIT operand (which={})",
                fp,
                which
            );
        }

        #[test]
        fn encode_rbit_diff_alt_spellings(
            is_64 in any::<bool>(),
            rd in 0u32..=31,
            rn in 0u32..=31,
            dest_spell in 0u32..=4,
            src_spell in 0u32..=4,
        ) {
            let dest = spell(is_64, rd, dest_spell);
            let src = spell(is_64, rn, src_spell);
            let asm = format!("rbit {}, {}", dest, src);
            let ops = [Operand::Reg(dest), Operand::Reg(src)];
            let mc = llvm_mc_word(&asm).expect("llvm-mc");
            let sut = sut_word(&ops).expect("SUT");
            prop_assert_eq!(sut, mc, "RBIT alt-spelling mismatch for {}", asm);
        }

        #[test]
        fn encode_rbit_neg_nonreg(
            which in 0u32..=1,
            bad in non_reg_operand(),
        ) {
            let mut ops = ops2(false, 0, 1).to_vec();
            ops[which as usize] = bad;
            prop_assert!(
                encode_rbit(&ops).is_err(),
                "wrong operand kind at slot {} must Err",
                which
            );
        }

        #[test]
        fn encode_rbit_neg_invalid_name(
            which in 0u32..=1,
            name in invalid_name(),
        ) {
            let mut ops = ops2(false, 0, 1);
            ops[which as usize] = Operand::Reg(name.clone());
            prop_assert!(
                encode_rbit(&ops).is_err(),
                "invalid register name {:?} at slot {} must Err",
                name,
                which
            );
        }

        #[test]
        fn encode_rbit_diff_valid_neon(
            q16 in any::<bool>(),
            rd in 0u32..=31,
            rn in 0u32..=31,
        ) {
            let t = if q16 { "16b" } else { "8b" };
            let asm = format!("rbit v{}.{}, v{}.{}", rd, t, rn, t);
            let ops = [
                Operand::RegArrangement {
                    reg: format!("v{rd}"),
                    arrangement: t.into(),
                },
                Operand::RegArrangement {
                    reg: format!("v{rn}"),
                    arrangement: t.into(),
                },
            ];
            let mc = llvm_mc_word(&asm).expect("llvm-mc");
            let sut = sut_word(&ops).expect("SUT");
            prop_assert_eq!(sut, mc, "RBIT NEON mismatch for {}", asm);
        }
    }

    fn spell(is_64: bool, n: u32, kind: u32) -> String {
        match kind {
            0 if n == 31 => {
                if is_64 {
                    "x31".into()
                } else {
                    "w31".into()
                }
            }
            1 if n == 31 => {
                if is_64 {
                    "XZR".into()
                } else {
                    "WZR".into()
                }
            }
            2 if n == 30 && is_64 => "LR".into(),
            3 => gpr(is_64, n).to_uppercase(),
            _ => gpr(is_64, n),
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
            "x-1".into(),
            "x99".into(),
            "w".into(),
        ])
    }

    fn non_reg_operand() -> impl Strategy<Value = Operand> {
        prop_oneof![
            any::<i64>().prop_map(Operand::Imm),
            Just(Operand::Shift {
                kind: "lsl".into(),
                amount: 0,
            }),
            Just(Operand::Mem {
                base: "x0".into(),
                offset: 0,
            }),
            Just(Operand::Label("L0".into())),
            Just(Operand::Symbol("foo".into())),
            Just(Operand::Cond("eq".into())),
        ]
    }

    #[test]
    fn test_encode_rbit_regression_extra_operand() {
        let mut ops = ops2(false, 0, 0).to_vec();
        ops.push(Operand::Reg("x0".into()));
        assert!(
            encode_rbit(&ops).is_err(),
            "RBIT w0, w0, x0 must Err; extra operand is invalid (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_rbit_regression_sp() {
        let ops = [Operand::Reg("wsp".into()), Operand::Reg("w0".into())];
        assert!(
            encode_rbit(&ops).is_err(),
            "RBIT wsp, w0 must Err; register 31 is ZR not SP/WSP (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_rbit_regression_mixed_width() {
        let ops = [Operand::Reg("x0".into()), Operand::Reg("w0".into())];
        assert!(
            encode_rbit(&ops).is_err(),
            "RBIT x0, w0 must Err; mixed W/X is invalid (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_rbit_regression_fp() {
        let ops = [Operand::Reg("d0".into()), Operand::Reg("x1".into())];
        assert!(
            encode_rbit(&ops).is_err(),
            "RBIT d0, x1 must Err; FP/SIMD registers are not RBIT operands (llvm-mc rejects it)"
        );
    }
}

#[cfg(test)]
mod encode_rev16_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler
    // Evidence: src/backend/arm/assembler/README.md:11 "accepts the same textual assembly that GCC's gas would consume";
    //   encoder/mod.rs "Encodes AArch64 instructions into 32-bit machine code words";
    //   encoder/mod.rs:576-578 "rev16" => encode_neon_two_misc (RegArrangement) / encode_rev16 (scalar);
    //   README.md:240 lists rev16 under Bit manipulation;
    //   ARM ARM Data-processing (1 source) REV16: sf 1 0 11010110 00000 000001 Rn Rd;
    //   register 31 is ZR not SP.
    // Stronger considered:
    //   - State machine: rejected — encode_rev16 is a pure function with no lifecycle
    //   - Algebraic round-trip: rejected — no in-tree REV16 decoder
    //   - encode_neon_two_misc as differential sibling: rejected — same-job gate fails
    //     (vector/RegArrangement, different ARM class; dispatch already splits the two)
    //   - encode_rev / encode_rev32 / encode_rbit / encode_clz / encode_cls: rejected —
    //     different opcode 000010/000011 / 000010 / 000000 / 000100 / 000101
    // Weaker available: algebraic.metamorphic (Rd/Rn field independence; W vs X sf),
    //   algebraic.invariant (ARM fields), negative_error (arity / extra / SP / mixed / FP)
    // Differential: candidate=encode_rev16, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=[Reg(Rd), Reg(Rn)] <-> `rev16 Rd, Rn`

    use super::encode_rev16;
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
        if is_64 {
            if n == 31 {
                "xzr".into()
            } else {
                format!("x{n}")
            }
        } else if n == 31 {
            "wzr".into()
        } else {
            format!("w{n}")
        }
    }

    fn ops2(is_64: bool, rd: u32, rn: u32) -> [Operand; 2] {
        [Operand::Reg(gpr(is_64, rd)), Operand::Reg(gpr(is_64, rn))]
    }

    fn sut_word(ops: &[Operand]) -> Result<u32, String> {
        match encode_rev16(ops)? {
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
            Just(Operand::Imm(-1)),
            Just(Operand::Shift {
                kind: "lsl".into(),
                amount: 0,
            }),
            Just(Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "8h".into(),
            }),
        ]
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_rev16_kat_llvm_mc_w0_w1() {
        let want = 0x5ac00420u32;
        let mc = llvm_mc_word("rev16 w0, w1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops2(false, 0, 1)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_rev16_kat_llvm_mc_x0_x1() {
        let want = 0xdac00420u32;
        let mc = llvm_mc_word("rev16 x0, x1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops2(true, 0, 1)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_rev16_kat_llvm_mc_wzr_wzr() {
        let want = 0x5ac007ffu32;
        let mc = llvm_mc_word("rev16 wzr, wzr").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops2(false, 31, 31)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_rev16_kat_llvm_mc_xzr_xzr() {
        let want = 0xdac007ffu32;
        let mc = llvm_mc_word("rev16 xzr, xzr").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops2(true, 31, 31)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_rev16_kat_llvm_mc_lr_x1() {
        let want = 0xdac0043eu32;
        let mc = llvm_mc_word("rev16 lr, x1").expect("llvm-mc LR KAT");
        assert_eq!(mc, want, "llvm-mc LR KAT mapping broken");
        let ops = [Operand::Reg("lr".into()), Operand::Reg("x1".into())];
        let sut = sut_word(&ops).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_rev16_kat_llvm_mc_x0_xzr() {
        let want = 0xdac007e0u32;
        let mc = llvm_mc_word("rev16 x0, xzr").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops2(true, 0, 31)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    proptest! {
        #![proptest_config(cfg())]

        #[test]
        fn encode_rev16_diff_valid_gpr(is_64 in any::<bool>(), rd in 0u32..=31, rn in 0u32..=31) {
            let dest = gpr(is_64, rd);
            let src = gpr(is_64, rn);
            let asm = format!("rev16 {}, {}", dest, src);
            let ops = ops2(is_64, rd, rn);
            let mc = llvm_mc_word(&asm).expect("llvm-mc");
            let sut = sut_word(&ops).expect("SUT");
            prop_assert_eq!(sut, mc, "REV16 mismatch for {}", asm);
        }

        #[test]
        fn encode_rev16_arm_fields(is_64 in any::<bool>(), rd in 0u32..=31, rn in 0u32..=31) {
            let sf = if is_64 { 1u32 } else { 0 };
            let w = sut_word(&ops2(is_64, rd, rn)).expect("SUT");
            let want = (sf << 31)
                | (1u32 << 30)
                | (0b011010110 << 21)
                | (0b000001 << 10)
                | (rn << 5)
                | rd;
            prop_assert_eq!(w, want, "ARM ARM REV16 field layout");
            prop_assert_eq!(w >> 31, sf, "sf");
            prop_assert_eq!((w >> 30) & 1, 1, "bit30=1");
            prop_assert_eq!((w >> 29) & 1, 0, "S=0");
            prop_assert_eq!((w >> 21) & 0xff, 0b11010110, "bits[28:21]");
            prop_assert_eq!((w >> 16) & 0x1f, 0, "opcode2=00000");
            prop_assert_eq!((w >> 10) & 0x3f, 0b000001, "opcode=000001 REV16");
            prop_assert_eq!((w >> 5) & 0x1f, rn, "Rn");
            prop_assert_eq!(w & 0x1f, rd, "Rd");
        }

        #[test]
        fn encode_rev16_metamorphic_rd_rn_sf(
            is_64 in any::<bool>(),
            rd in 0u32..=30,
            rn in 0u32..=30,
        ) {
            let base = sut_word(&ops2(is_64, rd, rn)).expect("base");
            let w_rd = sut_word(&ops2(is_64, rd + 1, rn)).expect("rd+1");
            let w_rn = sut_word(&ops2(is_64, rd, rn + 1)).expect("rn+1");
            prop_assert_eq!(w_rd & 0x1f, rd + 1, "Rd+1 updates Rd field");
            prop_assert_eq!(w_rd & !0x1fu32, base & !0x1fu32, "Rd+1 leaves other fields unchanged");
            prop_assert_eq!((w_rn >> 5) & 0x1f, rn + 1, "Rn+1 updates Rn field");
            prop_assert_eq!(w_rn & !(0x1fu32 << 5), base & !(0x1fu32 << 5), "Rn+1 leaves other fields unchanged");
            let w_sf = sut_word(&ops2(!is_64, rd, rn)).expect("sf flip");
            prop_assert_eq!(w_sf ^ base, 1u32 << 31, "W vs X flips only sf");
        }

        #[test]
        fn encode_rev16_neg_arity(
            len in 0usize..=1,
            is_64 in any::<bool>(),
            rd in 0u32..=31,
            rn in 0u32..=31,
        ) {
            let mut ops = vec![
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
            ];
            ops.truncate(len);
            prop_assert!(
                encode_rev16(&ops).is_err(),
                "REV16 with {} operands must Err (llvm-mc: too few operands)",
                len
            );
        }

        #[test]
        fn encode_rev16_neg_extra_operand(
            is_64 in any::<bool>(),
            rd in 0u32..=31,
            rn in 0u32..=31,
            extra in extra_operand(),
        ) {
            let mut ops = ops2(is_64, rd, rn).to_vec();
            ops.push(extra);
            prop_assert!(
                encode_rev16(&ops).is_err(),
                "REV16 has no 3rd operand; extra operand must Err (llvm-mc rejects it)"
            );
        }

        #[test]
        fn encode_rev16_neg_sp(
            which in 0u32..=1,
            sp64 in any::<bool>(),
            is_64 in any::<bool>(),
            other in 0u32..=30,
        ) {
            let sp = if sp64 { "sp" } else { "wsp" };
            let mut ops = ops2(is_64, other, other);
            ops[which as usize] = Operand::Reg(sp.to_string());
            prop_assert!(
                encode_rev16(&ops).is_err(),
                "SP/WSP is not a valid REV16 operand (which={} sp={})",
                which,
                sp
            );
        }

        #[test]
        fn encode_rev16_neg_mixed_width(
            rd in 0u32..=31,
            rn in 0u32..=31,
            rd64 in any::<bool>(),
            rn64 in any::<bool>(),
        ) {
            prop_assume!(rd64 != rn64);
            let ops = [
                Operand::Reg(gpr(rd64, rd)),
                Operand::Reg(gpr(rn64, rn)),
            ];
            prop_assert!(
                encode_rev16(&ops).is_err(),
                "REV16 mixed W/X (rd64={} rn64={}) must Err (llvm-mc rejects it)",
                rd64,
                rn64
            );
        }

        #[test]
        fn encode_rev16_neg_fp(
            which in 0u32..=1,
            prefix in prop::sample::select(vec!["d", "s", "q", "v", "h", "b"]),
            n in 0u32..=31,
        ) {
            let fp = format!("{}{}", prefix, n);
            let mut ops = ops2(true, 0, 1);
            ops[which as usize] = Operand::Reg(fp.clone());
            prop_assert!(
                encode_rev16(&ops).is_err(),
                "FP/SIMD register {} is not a valid REV16 operand (which={})",
                fp,
                which
            );
        }

        #[test]
        fn encode_rev16_diff_alt_spellings(
            is_64 in any::<bool>(),
            rd in 0u32..=31,
            rn in 0u32..=31,
            dest_spell in 0u32..=4,
            src_spell in 0u32..=4,
        ) {
            let dest = spell(is_64, rd, dest_spell);
            let src = spell(is_64, rn, src_spell);
            let asm = format!("rev16 {}, {}", dest, src);
            let ops = [Operand::Reg(dest), Operand::Reg(src)];
            let mc = llvm_mc_word(&asm).expect("llvm-mc");
            let sut = sut_word(&ops).expect("SUT");
            prop_assert_eq!(sut, mc, "REV16 alt-spelling mismatch for {}", asm);
        }

        #[test]
        fn encode_rev16_neg_nonreg(
            which in 0u32..=1,
            bad in non_reg_operand(),
        ) {
            let mut ops = ops2(false, 0, 1).to_vec();
            ops[which as usize] = bad;
            prop_assert!(
                encode_rev16(&ops).is_err(),
                "wrong operand kind at slot {} must Err",
                which
            );
        }

        #[test]
        fn encode_rev16_neg_invalid_name(
            which in 0u32..=1,
            name in invalid_name(),
        ) {
            let mut ops = ops2(false, 0, 1);
            ops[which as usize] = Operand::Reg(name.clone());
            prop_assert!(
                encode_rev16(&ops).is_err(),
                "invalid register name {:?} at slot {} must Err",
                name,
                which
            );
        }
    }

    fn spell(is_64: bool, n: u32, kind: u32) -> String {
        match kind {
            0 if n == 31 => {
                if is_64 {
                    "x31".into()
                } else {
                    "w31".into()
                }
            }
            1 if n == 31 => {
                if is_64 {
                    "XZR".into()
                } else {
                    "WZR".into()
                }
            }
            2 if n == 30 && is_64 => "LR".into(),
            3 => gpr(is_64, n).to_uppercase(),
            _ => gpr(is_64, n),
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
            "x-1".into(),
            "x99".into(),
            "w".into(),
        ])
    }

    fn non_reg_operand() -> impl Strategy<Value = Operand> {
        prop_oneof![
            any::<i64>().prop_map(Operand::Imm),
            Just(Operand::Shift {
                kind: "lsl".into(),
                amount: 0,
            }),
            Just(Operand::Mem {
                base: "x0".into(),
                offset: 0,
            }),
            Just(Operand::Label("L0".into())),
            Just(Operand::Symbol("foo".into())),
            Just(Operand::Cond("eq".into())),
        ]
    }

    #[test]
    fn test_encode_rev16_regression_extra_operand() {
        let mut ops = ops2(false, 0, 0).to_vec();
        ops.push(Operand::Reg("x0".into()));
        assert!(
            encode_rev16(&ops).is_err(),
            "REV16 w0, w0, x0 must Err; extra operand is invalid (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_rev16_regression_sp() {
        let ops = [Operand::Reg("wsp".into()), Operand::Reg("w0".into())];
        assert!(
            encode_rev16(&ops).is_err(),
            "REV16 wsp, w0 must Err; register 31 is ZR not SP/WSP (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_rev16_regression_mixed_width() {
        let ops = [Operand::Reg("x0".into()), Operand::Reg("w0".into())];
        assert!(
            encode_rev16(&ops).is_err(),
            "REV16 x0, w0 must Err; mixed W/X is invalid (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_rev16_regression_fp() {
        let ops = [Operand::Reg("d0".into()), Operand::Reg("x1".into())];
        assert!(
            encode_rev16(&ops).is_err(),
            "REV16 d0, x1 must Err; FP/SIMD registers are not REV16 operands (llvm-mc rejects it)"
        );
    }
}

#[cfg(test)]
mod encode_rev32_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler
    // Evidence: src/backend/arm/assembler/README.md:11 "accepts the same textual assembly that GCC's gas would consume";
    //   encoder/mod.rs "Encodes AArch64 instructions into 32-bit machine code words";
    //   encoder/mod.rs:579-581 "rev32" => encode_neon_two_misc (RegArrangement) / encode_rev32 (scalar);
    //   README.md:240 lists rev32 under Bit manipulation; README.md:225 lists rev32 under NEON two-misc;
    //   ARM ARM Data-processing (1 source) REV32: 1 1 0 11010110 00000 000010 Rn Rd (Xd,Xn only);
    //   ARM ARM Advanced SIMD two-register miscellaneous REV32 T in {8B,16B,4H,8H};
    //   register 31 is ZR not SP; purpose comment bitfield.rs:218 "REV32 is 64-bit only".
    // Stronger considered:
    //   - State machine: rejected — encode_rev32 is a pure function with no lifecycle
    //   - Algebraic round-trip: rejected — no in-tree REV32 decoder
    //   - encode_neon_two_misc as differential sibling: rejected — same-job gate fails
    //     (vector/RegArrangement, different ARM class; dispatch already splits the two)
    //   - encode_rev / encode_rev16 / encode_rbit / encode_clz / encode_cls: rejected —
    //     different opcode 000010-W/000011-X / 000001 / 000000 / 000100 / 000101
    // Weaker available: algebraic.metamorphic (Rd/Rn field independence),
    //   algebraic.invariant (ARM fields), negative_error (arity / extra / W-form / SP)
    // Differential: candidate=encode_rev32, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=[Reg(Xd), Reg(Xn)] <-> `rev32 Xd, Xn`;
    //   NEON path [RegArrangement(Vd.T), RegArrangement(Vn.T)] <-> `rev32 Vd.T, Vn.T`

    use super::encode_rev32;
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

    fn gpr64(n: u32) -> String {
        if n == 31 {
            "xzr".into()
        } else {
            format!("x{n}")
        }
    }

    fn gpr32(n: u32) -> String {
        if n == 31 {
            "wzr".into()
        } else {
            format!("w{n}")
        }
    }

    fn ops2(rd: u32, rn: u32) -> [Operand; 2] {
        [Operand::Reg(gpr64(rd)), Operand::Reg(gpr64(rn))]
    }

    fn neon_ops(rd: u32, rn: u32, arr: &str) -> [Operand; 2] {
        [
            Operand::RegArrangement {
                reg: format!("v{rd}"),
                arrangement: arr.to_string(),
            },
            Operand::RegArrangement {
                reg: format!("v{rn}"),
                arrangement: arr.to_string(),
            },
        ]
    }

    fn sut_word(ops: &[Operand]) -> Result<u32, String> {
        match encode_rev32(ops)? {
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
            Just(Operand::Imm(-1)),
            Just(Operand::Shift {
                kind: "lsl".into(),
                amount: 0,
            }),
            Just(Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "8h".into(),
            }),
        ]
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_rev32_kat_llvm_mc_x0_x1() {
        let want = 0xdac00820u32;
        let mc = llvm_mc_word("rev32 x0, x1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops2(0, 1)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_rev32_kat_llvm_mc_xzr_xzr() {
        let want = 0xdac00bffu32;
        let mc = llvm_mc_word("rev32 xzr, xzr").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops2(31, 31)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_rev32_kat_llvm_mc_lr_x1() {
        let want = 0xdac0083eu32;
        let mc = llvm_mc_word("rev32 lr, x1").expect("llvm-mc LR KAT");
        assert_eq!(mc, want, "llvm-mc LR KAT mapping broken");
        let ops = [Operand::Reg("lr".into()), Operand::Reg("x1".into())];
        let sut = sut_word(&ops).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_rev32_kat_llvm_mc_x0_xzr() {
        let want = 0xdac00be0u32;
        let mc = llvm_mc_word("rev32 x0, xzr").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops2(0, 31)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_rev32_kat_llvm_mc_v0_8b() {
        let want = 0x2e200820u32;
        let mc = llvm_mc_word("rev32 v0.8b, v1.8b").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc NEON KAT mapping broken");
        let sut = sut_word(&neon_ops(0, 1, "8b")).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_rev32_kat_llvm_mc_v0_16b() {
        let want = 0x6e200820u32;
        let mc = llvm_mc_word("rev32 v0.16b, v1.16b").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc NEON KAT mapping broken");
        let sut = sut_word(&neon_ops(0, 1, "16b")).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_rev32_kat_llvm_mc_v0_4h() {
        let want = 0x2e600820u32;
        let mc = llvm_mc_word("rev32 v0.4h, v1.4h").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc NEON KAT mapping broken");
        let sut = sut_word(&neon_ops(0, 1, "4h")).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_rev32_kat_llvm_mc_v0_8h() {
        let want = 0x6e600820u32;
        let mc = llvm_mc_word("rev32 v0.8h, v1.8h").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc NEON KAT mapping broken");
        let sut = sut_word(&neon_ops(0, 1, "8h")).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    proptest! {
        #![proptest_config(cfg())]

        #[test]
        fn encode_rev32_diff_valid_gpr(rd in 0u32..=31, rn in 0u32..=31) {
            let dest = gpr64(rd);
            let src = gpr64(rn);
            let asm = format!("rev32 {}, {}", dest, src);
            let ops = ops2(rd, rn);
            let mc = llvm_mc_word(&asm).expect("llvm-mc");
            let sut = sut_word(&ops).expect("SUT");
            prop_assert_eq!(sut, mc, "REV32 mismatch for {}", asm);
        }

        #[test]
        fn encode_rev32_arm_fields(rd in 0u32..=31, rn in 0u32..=31) {
            let w = sut_word(&ops2(rd, rn)).expect("SUT");
            let want = (1u32 << 31)
                | (1u32 << 30)
                | (0b011010110 << 21)
                | (0b000010 << 10)
                | (rn << 5)
                | rd;
            prop_assert_eq!(w, want, "ARM ARM REV32 field layout");
            prop_assert_eq!(w >> 31, 1, "sf=1 (64-bit only)");
            prop_assert_eq!((w >> 30) & 1, 1, "bit30=1");
            prop_assert_eq!((w >> 29) & 1, 0, "S=0");
            prop_assert_eq!((w >> 21) & 0xff, 0b11010110, "bits[28:21]");
            prop_assert_eq!((w >> 16) & 0x1f, 0, "opcode2=00000");
            prop_assert_eq!((w >> 10) & 0x3f, 0b000010, "opcode=000010 REV32");
            prop_assert_eq!((w >> 5) & 0x1f, rn, "Rn");
            prop_assert_eq!(w & 0x1f, rd, "Rd");
        }

        #[test]
        fn encode_rev32_metamorphic_rd_rn(rd in 0u32..=30, rn in 0u32..=30) {
            let base = sut_word(&ops2(rd, rn)).expect("base");
            let w_rd = sut_word(&ops2(rd + 1, rn)).expect("rd+1");
            let w_rn = sut_word(&ops2(rd, rn + 1)).expect("rn+1");
            prop_assert_eq!(w_rd & 0x1f, rd + 1, "Rd+1 updates Rd field");
            prop_assert_eq!(w_rd & !0x1fu32, base & !0x1fu32, "Rd+1 leaves other fields unchanged");
            prop_assert_eq!((w_rn >> 5) & 0x1f, rn + 1, "Rn+1 updates Rn field");
            prop_assert_eq!(w_rn & !(0x1fu32 << 5), base & !(0x1fu32 << 5), "Rn+1 leaves other fields unchanged");
        }

        #[test]
        fn encode_rev32_diff_neon(
            rd in 0u32..=31,
            rn in 0u32..=31,
            arr in prop::sample::select(vec!["8b", "16b", "4h", "8h"]),
        ) {
            let asm = format!("rev32 v{}.{}, v{}.{}", rd, arr, rn, arr);
            let ops = neon_ops(rd, rn, arr);
            let mc = llvm_mc_word(&asm).expect("llvm-mc");
            let sut = sut_word(&ops).expect("SUT");
            prop_assert_eq!(sut, mc, "REV32 NEON mismatch for {}", asm);
        }

        #[test]
        fn encode_rev32_neg_arity(
            len in 0usize..=1,
            rd in 0u32..=31,
            rn in 0u32..=31,
        ) {
            let mut ops = vec![
                Operand::Reg(gpr64(rd)),
                Operand::Reg(gpr64(rn)),
            ];
            ops.truncate(len);
            prop_assert!(
                encode_rev32(&ops).is_err(),
                "REV32 with {} operands must Err (llvm-mc: too few operands)",
                len
            );
        }

        #[test]
        fn encode_rev32_neg_extra_operand(
            rd in 0u32..=31,
            rn in 0u32..=31,
            extra in extra_operand(),
        ) {
            let mut ops = ops2(rd, rn).to_vec();
            ops.push(extra);
            prop_assert!(
                encode_rev32(&ops).is_err(),
                "REV32 has no 3rd operand; extra operand must Err (llvm-mc rejects it)"
            );
        }

        #[test]
        fn encode_rev32_neg_w32(rd in 0u32..=31, rn in 0u32..=31) {
            let ops = [
                Operand::Reg(gpr32(rd)),
                Operand::Reg(gpr32(rn)),
            ];
            prop_assert!(
                encode_rev32(&ops).is_err(),
                "REV32 is 64-bit only; W registers must Err (llvm-mc rejects rev32 w{}, w{})",
                rd,
                rn
            );
        }

        #[test]
        fn encode_rev32_neg_sp(
            which in 0u32..=1,
            sp64 in any::<bool>(),
            other in 0u32..=30,
        ) {
            let sp = if sp64 { "sp" } else { "wsp" };
            let mut ops = ops2(other, other);
            ops[which as usize] = Operand::Reg(sp.to_string());
            prop_assert!(
                encode_rev32(&ops).is_err(),
                "SP/WSP is not a valid REV32 operand (which={} sp={})",
                which,
                sp
            );
        }

        #[test]
        fn encode_rev32_diff_alt_spellings(
            rd in 0u32..=31,
            rn in 0u32..=31,
            dest_spell in 0u32..=4,
            src_spell in 0u32..=4,
        ) {
            let dest = spell(rd, dest_spell);
            let src = spell(rn, src_spell);
            let asm = format!("rev32 {}, {}", dest, src);
            let ops = [Operand::Reg(dest), Operand::Reg(src)];
            let mc = llvm_mc_word(&asm).expect("llvm-mc");
            let sut = sut_word(&ops).expect("SUT");
            prop_assert_eq!(sut, mc, "REV32 alt-spelling mismatch for {}", asm);
        }

        #[test]
        fn encode_rev32_neg_mixed_width(
            rd in 0u32..=31,
            rn in 0u32..=31,
            rd64 in any::<bool>(),
            rn64 in any::<bool>(),
        ) {
            prop_assume!(rd64 != rn64);
            let dest = if rd64 { gpr64(rd) } else { gpr32(rd) };
            let src = if rn64 { gpr64(rn) } else { gpr32(rn) };
            let ops = [Operand::Reg(dest), Operand::Reg(src)];
            prop_assert!(
                encode_rev32(&ops).is_err(),
                "REV32 mixed W/X (rd64={} rn64={}) must Err (llvm-mc rejects it)",
                rd64,
                rn64
            );
        }

        #[test]
        fn encode_rev32_neg_fp(
            which in 0u32..=1,
            prefix in prop::sample::select(vec!["d", "s", "q", "v", "h", "b"]),
            n in 0u32..=31,
        ) {
            let fp = format!("{}{}", prefix, n);
            let mut ops = ops2(0, 1);
            ops[which as usize] = Operand::Reg(fp.clone());
            prop_assert!(
                encode_rev32(&ops).is_err(),
                "FP/SIMD register {} is not a valid REV32 operand (which={})",
                fp,
                which
            );
        }

        #[test]
        fn encode_rev32_neg_nonreg(
            which in 0u32..=1,
            bad in non_reg_operand(),
        ) {
            let mut ops = ops2(0, 1).to_vec();
            ops[which as usize] = bad;
            prop_assert!(
                encode_rev32(&ops).is_err(),
                "wrong operand kind at slot {} must Err",
                which
            );
        }

        #[test]
        fn encode_rev32_neg_invalid_name(
            which in 0u32..=1,
            name in invalid_name(),
        ) {
            let mut ops = ops2(0, 1);
            ops[which as usize] = Operand::Reg(name.clone());
            prop_assert!(
                encode_rev32(&ops).is_err(),
                "invalid register name {:?} at slot {} must Err",
                name,
                which
            );
        }

        #[test]
        fn encode_rev32_neg_neon_invalid_arr(
            rd in 0u32..=31,
            rn in 0u32..=31,
            arr in prop::sample::select(vec!["2s", "4s", "2d", "1d"]),
        ) {
            let ops = neon_ops(rd, rn, arr);
            prop_assert!(
                encode_rev32(&ops).is_err(),
                "REV32 NEON T={} is outside {{8B,16B,4H,8H}}; must Err (llvm-mc rejects it)",
                arr
            );
        }
    }

    fn spell(n: u32, kind: u32) -> String {
        match kind {
            0 if n == 31 => "x31".into(),
            1 if n == 31 => "XZR".into(),
            2 if n == 30 => "LR".into(),
            3 => gpr64(n).to_uppercase(),
            _ => gpr64(n),
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
            "x-1".into(),
            "x99".into(),
            "w".into(),
        ])
    }

    fn non_reg_operand() -> impl Strategy<Value = Operand> {
        prop_oneof![
            any::<i64>().prop_map(Operand::Imm),
            Just(Operand::Shift {
                kind: "lsl".into(),
                amount: 0,
            }),
            Just(Operand::Mem {
                base: "x0".into(),
                offset: 0,
            }),
            Just(Operand::Label("L0".into())),
            Just(Operand::Symbol("foo".into())),
            Just(Operand::Cond("eq".into())),
        ]
    }

    #[test]
    fn test_encode_rev32_regression_extra_operand() {
        let mut ops = ops2(0, 0).to_vec();
        ops.push(Operand::Reg("x0".into()));
        assert!(
            encode_rev32(&ops).is_err(),
            "REV32 x0, x0, x0 must Err; extra operand is invalid (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_rev32_regression_sp() {
        let ops = [Operand::Reg("wsp".into()), Operand::Reg("x0".into())];
        assert!(
            encode_rev32(&ops).is_err(),
            "REV32 wsp, x0 must Err; register 31 is ZR not SP/WSP (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_rev32_regression_w32() {
        let ops = [Operand::Reg("w0".into()), Operand::Reg("w0".into())];
        assert!(
            encode_rev32(&ops).is_err(),
            "REV32 w0, w0 must Err; REV32 is 64-bit only (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_rev32_regression_mixed_width() {
        let ops = [Operand::Reg("x0".into()), Operand::Reg("w0".into())];
        assert!(
            encode_rev32(&ops).is_err(),
            "REV32 x0, w0 must Err; mixed W/X is invalid (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_rev32_regression_fp() {
        let ops = [Operand::Reg("d0".into()), Operand::Reg("x1".into())];
        assert!(
            encode_rev32(&ops).is_err(),
            "REV32 d0, x1 must Err; FP/SIMD registers are not REV32 operands (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_rev32_regression_neon_invalid_arr() {
        let ops = neon_ops(0, 1, "2s");
        assert!(
            encode_rev32(&ops).is_err(),
            "REV32 v0.2s, v1.2s must Err; T is outside {{8B,16B,4H,8H}} (llvm-mc rejects it)"
        );
    }
}

#[cfg(test)]
mod encode_rev_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler
    // Evidence: src/backend/arm/assembler/README.md:11 "accepts the same textual assembly that GCC's gas would consume";
    //   encoder/mod.rs "Encodes AArch64 instructions into 32-bit machine code words";
    //   encoder/mod.rs:910 "rev" => encode_rev (no NEON split; vector reverse is rev16/rev32/rev64);
    //   README.md:240 lists rev under Bit manipulation;
    //   ARM ARM Data-processing (1 source) REV: sf 1 0 11010110 00000 opc Rn Rd,
    //   opc=000010 (Wd,Wn) / 000011 (Xd,Xn); register 31 is ZR not SP.
    // Stronger considered:
    //   - State machine: rejected — encode_rev is a pure function with no lifecycle
    //   - Algebraic round-trip: rejected — no in-tree REV decoder
    //   - encode_neon_two_misc as differential sibling: rejected — same-job gate fails
    //     (vector rev16/rev32/rev64, different ARM class; public dispatch does not route `rev` there)
    //   - encode_rev16 / encode_rev32 / encode_rbit / encode_clz / encode_cls: rejected —
    //     different opcode 000001 / 000010-X / 000000 / 000100 / 000101
    // Weaker available: algebraic.metamorphic (Rd/Rn field independence; W vs X flips sf AND opcode LSB),
    //   algebraic.invariant (ARM fields), negative_error (arity / extra / SP / mixed / FP)
    // Differential: candidate=encode_rev, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=[Reg(Rd), Reg(Rn)] <-> `rev Rd, Rn`

    use super::encode_rev;
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
        if is_64 {
            if n == 31 {
                "xzr".into()
            } else {
                format!("x{n}")
            }
        } else if n == 31 {
            "wzr".into()
        } else {
            format!("w{n}")
        }
    }

    fn ops2(is_64: bool, rd: u32, rn: u32) -> [Operand; 2] {
        [Operand::Reg(gpr(is_64, rd)), Operand::Reg(gpr(is_64, rn))]
    }

    fn sut_word(ops: &[Operand]) -> Result<u32, String> {
        match encode_rev(ops)? {
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
            Just(Operand::Imm(-1)),
            Just(Operand::Shift {
                kind: "lsl".into(),
                amount: 0,
            }),
            Just(Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "8h".into(),
            }),
        ]
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_rev_kat_llvm_mc_w0_w1() {
        let want = 0x5ac00820u32;
        let mc = llvm_mc_word("rev w0, w1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops2(false, 0, 1)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_rev_kat_llvm_mc_x0_x1() {
        let want = 0xdac00c20u32;
        let mc = llvm_mc_word("rev x0, x1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops2(true, 0, 1)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_rev_kat_llvm_mc_wzr_wzr() {
        let want = 0x5ac00bffu32;
        let mc = llvm_mc_word("rev wzr, wzr").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops2(false, 31, 31)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_rev_kat_llvm_mc_xzr_xzr() {
        let want = 0xdac00fffu32;
        let mc = llvm_mc_word("rev xzr, xzr").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops2(true, 31, 31)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_rev_kat_llvm_mc_lr_x1() {
        let want = 0xdac00c3eu32;
        let mc = llvm_mc_word("rev lr, x1").expect("llvm-mc LR KAT");
        assert_eq!(mc, want, "llvm-mc LR KAT mapping broken");
        let ops = [Operand::Reg("lr".into()), Operand::Reg("x1".into())];
        let sut = sut_word(&ops).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_rev_kat_llvm_mc_x0_xzr() {
        let want = 0xdac00fe0u32;
        let mc = llvm_mc_word("rev x0, xzr").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops2(true, 0, 31)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    proptest! {
        #![proptest_config(cfg())]

        #[test]
        fn encode_rev_diff_valid_gpr(is_64 in any::<bool>(), rd in 0u32..=31, rn in 0u32..=31) {
            let dest = gpr(is_64, rd);
            let src = gpr(is_64, rn);
            let asm = format!("rev {}, {}", dest, src);
            let ops = ops2(is_64, rd, rn);
            let mc = llvm_mc_word(&asm).expect("llvm-mc");
            let sut = sut_word(&ops).expect("SUT");
            prop_assert_eq!(sut, mc, "REV mismatch for {}", asm);
        }

        #[test]
        fn encode_rev_arm_fields(is_64 in any::<bool>(), rd in 0u32..=31, rn in 0u32..=31) {
            let sf = if is_64 { 1u32 } else { 0 };
            let opc = if is_64 { 0b000011u32 } else { 0b000010 };
            let w = sut_word(&ops2(is_64, rd, rn)).expect("SUT");
            let want = (sf << 31)
                | (1u32 << 30)
                | (0b011010110 << 21)
                | (opc << 10)
                | (rn << 5)
                | rd;
            prop_assert_eq!(w, want, "ARM ARM REV field layout");
            prop_assert_eq!(w >> 31, sf, "sf");
            prop_assert_eq!((w >> 30) & 1, 1, "bit30=1");
            prop_assert_eq!((w >> 29) & 1, 0, "S=0");
            prop_assert_eq!((w >> 21) & 0xff, 0b11010110, "bits[28:21]");
            prop_assert_eq!((w >> 16) & 0x1f, 0, "opcode2=00000");
            prop_assert_eq!((w >> 10) & 0x3f, opc, "opcode=000010 W / 000011 X");
            prop_assert_eq!((w >> 5) & 0x1f, rn, "Rn");
            prop_assert_eq!(w & 0x1f, rd, "Rd");
        }

        #[test]
        fn encode_rev_metamorphic_rd_rn_sf(
            is_64 in any::<bool>(),
            rd in 0u32..=30,
            rn in 0u32..=30,
        ) {
            let base = sut_word(&ops2(is_64, rd, rn)).expect("base");
            let w_rd = sut_word(&ops2(is_64, rd + 1, rn)).expect("rd+1");
            let w_rn = sut_word(&ops2(is_64, rd, rn + 1)).expect("rn+1");
            prop_assert_eq!(w_rd & 0x1f, rd + 1, "Rd+1 updates Rd field");
            prop_assert_eq!(w_rd & !0x1fu32, base & !0x1fu32, "Rd+1 leaves other fields unchanged");
            prop_assert_eq!((w_rn >> 5) & 0x1f, rn + 1, "Rn+1 updates Rn field");
            prop_assert_eq!(w_rn & !(0x1fu32 << 5), base & !(0x1fu32 << 5), "Rn+1 leaves other fields unchanged");
            let w_sf = sut_word(&ops2(!is_64, rd, rn)).expect("sf flip");
            // Unlike REV16/CLZ/RBIT, REV opcode bits[15:10] also change with sf (000010 W / 000011 X).
            prop_assert_eq!(w_sf ^ base, (1u32 << 31) | (1u32 << 10), "W vs X flips sf and opcode LSB");
        }

        #[test]
        fn encode_rev_neg_arity(
            len in 0usize..=1,
            is_64 in any::<bool>(),
            rd in 0u32..=31,
            rn in 0u32..=31,
        ) {
            let mut ops = vec![
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
            ];
            ops.truncate(len);
            prop_assert!(
                encode_rev(&ops).is_err(),
                "REV with {} operands must Err (llvm-mc: too few operands)",
                len
            );
        }

        #[test]
        fn encode_rev_neg_extra_operand(
            is_64 in any::<bool>(),
            rd in 0u32..=31,
            rn in 0u32..=31,
            extra in extra_operand(),
        ) {
            let mut ops = ops2(is_64, rd, rn).to_vec();
            ops.push(extra);
            prop_assert!(
                encode_rev(&ops).is_err(),
                "REV has no 3rd operand; extra operand must Err (llvm-mc rejects it)"
            );
        }

        #[test]
        fn encode_rev_neg_sp(
            which in 0u32..=1,
            sp64 in any::<bool>(),
            is_64 in any::<bool>(),
            other in 0u32..=30,
        ) {
            let sp = if sp64 { "sp" } else { "wsp" };
            let mut ops = ops2(is_64, other, other);
            ops[which as usize] = Operand::Reg(sp.to_string());
            prop_assert!(
                encode_rev(&ops).is_err(),
                "SP/WSP is not a valid REV operand (which={} sp={})",
                which,
                sp
            );
        }

        #[test]
        fn encode_rev_neg_mixed_width(
            rd in 0u32..=31,
            rn in 0u32..=31,
            rd64 in any::<bool>(),
            rn64 in any::<bool>(),
        ) {
            prop_assume!(rd64 != rn64);
            let ops = [
                Operand::Reg(gpr(rd64, rd)),
                Operand::Reg(gpr(rn64, rn)),
            ];
            prop_assert!(
                encode_rev(&ops).is_err(),
                "REV mixed W/X (rd64={} rn64={}) must Err (llvm-mc rejects it)",
                rd64,
                rn64
            );
        }

        #[test]
        fn encode_rev_neg_fp(
            which in 0u32..=1,
            prefix in prop::sample::select(vec!["d", "s", "q", "v", "h", "b"]),
            n in 0u32..=31,
        ) {
            let fp = format!("{}{}", prefix, n);
            let mut ops = ops2(true, 0, 1);
            ops[which as usize] = Operand::Reg(fp.clone());
            prop_assert!(
                encode_rev(&ops).is_err(),
                "FP/SIMD register {} is not a valid REV operand (which={})",
                fp,
                which
            );
        }

        #[test]
        fn encode_rev_diff_alt_spellings(
            is_64 in any::<bool>(),
            rd in 0u32..=31,
            rn in 0u32..=31,
            dest_spell in 0u32..=4,
            src_spell in 0u32..=4,
        ) {
            let dest = spell(is_64, rd, dest_spell);
            let src = spell(is_64, rn, src_spell);
            let asm = format!("rev {}, {}", dest, src);
            let ops = [Operand::Reg(dest), Operand::Reg(src)];
            let mc = llvm_mc_word(&asm).expect("llvm-mc");
            let sut = sut_word(&ops).expect("SUT");
            prop_assert_eq!(sut, mc, "REV alt-spelling mismatch for {}", asm);
        }

        #[test]
        fn encode_rev_neg_nonreg(
            which in 0u32..=1,
            bad in non_reg_operand(),
        ) {
            let mut ops = ops2(false, 0, 1).to_vec();
            ops[which as usize] = bad;
            prop_assert!(
                encode_rev(&ops).is_err(),
                "wrong operand kind at slot {} must Err",
                which
            );
        }

        #[test]
        fn encode_rev_neg_invalid_name(
            which in 0u32..=1,
            name in invalid_name(),
        ) {
            let mut ops = ops2(false, 0, 1);
            ops[which as usize] = Operand::Reg(name.clone());
            prop_assert!(
                encode_rev(&ops).is_err(),
                "invalid register name {:?} at slot {} must Err",
                name,
                which
            );
        }
    }

    fn spell(is_64: bool, n: u32, kind: u32) -> String {
        match kind {
            0 if n == 31 => {
                if is_64 {
                    "x31".into()
                } else {
                    "w31".into()
                }
            }
            1 if n == 31 => {
                if is_64 {
                    "XZR".into()
                } else {
                    "WZR".into()
                }
            }
            2 if n == 30 && is_64 => "LR".into(),
            3 => gpr(is_64, n).to_uppercase(),
            _ => gpr(is_64, n),
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
            "x-1".into(),
            "x99".into(),
            "w".into(),
        ])
    }

    fn non_reg_operand() -> impl Strategy<Value = Operand> {
        prop_oneof![
            any::<i64>().prop_map(Operand::Imm),
            Just(Operand::Shift {
                kind: "lsl".into(),
                amount: 0,
            }),
            Just(Operand::Mem {
                base: "x0".into(),
                offset: 0,
            }),
            Just(Operand::Label("L0".into())),
            Just(Operand::Symbol("foo".into())),
            Just(Operand::Cond("eq".into())),
        ]
    }

    #[test]
    fn test_encode_rev_regression_extra_operand() {
        let mut ops = ops2(false, 0, 0).to_vec();
        ops.push(Operand::Reg("x0".into()));
        assert!(
            encode_rev(&ops).is_err(),
            "REV w0, w0, x0 must Err; extra operand is invalid (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_rev_regression_sp() {
        let ops = [Operand::Reg("wsp".into()), Operand::Reg("w0".into())];
        assert!(
            encode_rev(&ops).is_err(),
            "REV wsp, w0 must Err; register 31 is ZR not SP/WSP (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_rev_regression_mixed_width() {
        let ops = [Operand::Reg("x0".into()), Operand::Reg("w0".into())];
        assert!(
            encode_rev(&ops).is_err(),
            "REV x0, w0 must Err; mixed W/X is invalid (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_rev_regression_fp() {
        let ops = [Operand::Reg("d0".into()), Operand::Reg("x1".into())];
        assert!(
            encode_rev(&ops).is_err(),
            "REV d0, x1 must Err; FP/SIMD registers are not REV operands (llvm-mc rejects it)"
        );
    }
}

#[cfg(test)]
mod encode_sbfiz_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler
    // Evidence: src/backend/arm/assembler/README.md:11 "accepts the same textual assembly that GCC's gas would consume";
    //   encoder/mod.rs "Encodes AArch64 instructions into 32-bit machine code words";
    //   encoder/mod.rs:890 "sbfiz" => encode_sbfiz; README.md:216 lists sbfiz;
    //   bitfield.rs:60 purpose comment SBFIZ -> SBFM #(-lsb MOD regsize), #(width-1);
    //   ARM ARM Bitfield Move SBFIZ alias of SBFM: sf 00 100110 N immr imms Rn Rd,
    //   N=sf, immr=(-lsb MOD datasize), imms=width-1; register 31 is ZR not SP.
    // Stronger considered:
    //   - State machine: rejected — encode_sbfiz is a pure function with no lifecycle
    //   - Algebraic round-trip: rejected — no in-tree SBFIZ decoder
    //   - encode_sbfm as differential sibling: rejected — same-job gate fails (raw immr/imms form);
    //     used only as algebraic alias after the ARM mapping
    //   - encode_ubfiz / encode_sbfx / encode_bfi: rejected — UBFM/SBFX/BFM opc or different alias mapping
    // Weaker available: algebraic.metamorphic (SBFIZ alias of SBFM; Rd/Rn field independence),
    //   algebraic.invariant (ARM fields), negative_error (arity / extra / SP / lsb-width)
    // Differential: candidate=encode_sbfiz, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=[Reg(Rd), Reg(Rn), Imm(lsb), Imm(width)] <-> `sbfiz Rd, Rn, #lsb, #width`

    use super::{encode_sbfiz, encode_sbfm};
    use super::super::EncodeResult;
    use crate::backend::arm::assembler::parser::Operand;
    use proptest::prelude::*;
    use std::io::Write;
    use std::panic::{catch_unwind, AssertUnwindSafe};
    use std::process::{Command, Stdio};

    const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
    const CASES: u32 = 1000;

    fn cfg() -> ProptestConfig {
        ProptestConfig::with_cases(CASES)
    }

    fn gpr(is_64: bool, n: u32) -> String {
        if is_64 {
            if n == 31 {
                "xzr".into()
            } else {
                format!("x{n}")
            }
        } else if n == 31 {
            "wzr".into()
        } else {
            format!("w{n}")
        }
    }

    fn lsb_width(r: u32) -> impl Strategy<Value = (u32, u32)> {
        prop_oneof![Just(0u32), Just(r - 1), 0u32..r].prop_flat_map(move |lsb| {
            let max_w = r - lsb;
            prop_oneof![Just(1u32), Just(max_w), 1u32..=max_w]
                .prop_map(move |width| (lsb, width))
        })
    }

    fn sbfiz_valid() -> impl Strategy<Value = (bool, u32, u32, u32, u32)> {
        any::<bool>().prop_flat_map(|is_64| {
            let r = if is_64 { 64u32 } else { 32 };
            (
                Just(is_64),
                0u32..=31,
                0u32..=31,
                lsb_width(r),
            )
                .prop_map(|(is_64, rd, rn, (lsb, width))| (is_64, rd, rn, lsb, width))
        })
    }

    fn invalid_lsb_width(is_64: bool) -> impl Strategy<Value = (i64, i64)> {
        let r = if is_64 { 64i64 } else { 32 };
        prop_oneof![
            (0i64..r).prop_map(|lsb| (lsb, 0i64)),
            (0i64..r).prop_map(|lsb| (lsb, -1i64)),
            (1i64..=r).prop_map(|width| (-1i64, width)),
            Just((r, 1i64)),
            Just((r, r)),
            Just((r - 1, 2i64)),
            Just((0i64, r + 1)),
            Just((r + 1, 1i64)),
        ]
    }

    fn ops4(is_64: bool, rd: u32, rn: u32, lsb: i64, width: i64) -> [Operand; 4] {
        [
            Operand::Reg(gpr(is_64, rd)),
            Operand::Reg(gpr(is_64, rn)),
            Operand::Imm(lsb),
            Operand::Imm(width),
        ]
    }

    fn sut_word(ops: &[Operand]) -> Result<u32, String> {
        match encode_sbfiz(ops)? {
            EncodeResult::Word(w) => Ok(w),
            other => Err(format!("expected Word, got {:?}", other)),
        }
    }

    fn sbfm_word(ops: &[Operand]) -> Result<u32, String> {
        match encode_sbfm(ops)? {
            EncodeResult::Word(w) => Ok(w),
            other => Err(format!("expected Word, got {:?}", other)),
        }
    }

    fn must_err(ops: &[Operand]) -> bool {
        match catch_unwind(AssertUnwindSafe(|| encode_sbfiz(ops))) {
            Ok(Err(_)) => true,
            _ => false,
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
            Just(Operand::Imm(-1)),
            Just(Operand::Shift {
                kind: "lsl".into(),
                amount: 0,
            }),
            Just(Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "8h".into(),
            }),
        ]
    }

    fn spell(is_64: bool, n: u32, kind: u32) -> String {
        match kind {
            0 if n == 31 => {
                if is_64 {
                    "x31".into()
                } else {
                    "w31".into()
                }
            }
            1 if n == 31 => {
                if is_64 {
                    "XZR".into()
                } else {
                    "WZR".into()
                }
            }
            2 if n == 30 && is_64 => "LR".into(),
            3 => gpr(is_64, n).to_uppercase(),
            _ => gpr(is_64, n),
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
            "x-1".into(),
            "x99".into(),
            "w".into(),
        ])
    }

    fn non_reg_operand() -> impl Strategy<Value = Operand> {
        prop_oneof![
            any::<i64>().prop_map(Operand::Imm),
            Just(Operand::Shift {
                kind: "lsl".into(),
                amount: 0,
            }),
            Just(Operand::Mem {
                base: "x0".into(),
                offset: 0,
            }),
            Just(Operand::Label("L0".into())),
            Just(Operand::Symbol("foo".into())),
            Just(Operand::Cond("eq".into())),
            Just(Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "8b".into(),
            }),
        ]
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_sbfiz_kat_llvm_mc_w0_w1_lsb0_width1() {
        let want = 0x13000020u32;
        let mc = llvm_mc_word("sbfiz w0, w1, #0, #1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops4(false, 0, 1, 0, 1)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_sbfiz_kat_llvm_mc_w0_w1_lsb1_width1() {
        let want = 0x131f0020u32;
        let mc = llvm_mc_word("sbfiz w0, w1, #1, #1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops4(false, 0, 1, 1, 1)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_sbfiz_kat_llvm_mc_x0_x1_lsb1_width8() {
        let want = 0x937f1c20u32;
        let mc = llvm_mc_word("sbfiz x0, x1, #1, #8").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops4(true, 0, 1, 1, 8)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_sbfiz_kat_llvm_mc_wzr_wzr_lsb31_width1() {
        let want = 0x130103ffu32;
        let mc = llvm_mc_word("sbfiz wzr, wzr, #31, #1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops4(false, 31, 31, 31, 1)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_sbfiz_kat_llvm_mc_x0_xzr_lsb63_width1() {
        let want = 0x934103e0u32;
        let mc = llvm_mc_word("sbfiz x0, xzr, #63, #1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops4(true, 0, 31, 63, 1)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_sbfiz_kat_llvm_mc_lr_x1_lsb8_width16() {
        let want = 0x93783c3eu32;
        let mc = llvm_mc_word("sbfiz lr, x1, #8, #16").expect("llvm-mc LR KAT");
        assert_eq!(mc, want, "llvm-mc LR KAT mapping broken");
        let ops = [
            Operand::Reg("lr".into()),
            Operand::Reg("x1".into()),
            Operand::Imm(8),
            Operand::Imm(16),
        ];
        let sut = sut_word(&ops).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    proptest! {
        #![proptest_config(cfg())]

        #[test]
        fn encode_sbfiz_diff_valid_gpr((is_64, rd, rn, lsb, width) in sbfiz_valid()) {
            let dest = gpr(is_64, rd);
            let src = gpr(is_64, rn);
            let asm = format!("sbfiz {}, {}, #{}, #{}", dest, src, lsb, width);
            let ops = ops4(is_64, rd, rn, lsb as i64, width as i64);
            let mc = llvm_mc_word(&asm).expect("llvm-mc");
            let sut = sut_word(&ops).expect("SUT");
            prop_assert_eq!(sut, mc, "SBFIZ mismatch for {}", asm);
        }

        #[test]
        fn encode_sbfiz_alias_sbfm((is_64, rd, rn, lsb, width) in sbfiz_valid()) {
            let r = if is_64 { 64i64 } else { 32 };
            let immr = (-(lsb as i64)).rem_euclid(r);
            let imms = (width as i64) - 1;
            let sbfiz = sut_word(&ops4(is_64, rd, rn, lsb as i64, width as i64)).expect("SBFIZ");
            let sbfm = sbfm_word(&[
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
                Operand::Imm(immr),
                Operand::Imm(imms),
            ]).expect("SBFM");
            prop_assert_eq!(sbfiz, sbfm, "SBFIZ must alias SBFM #(-lsb MOD {}), #(width-1)", r);
        }

        #[test]
        fn encode_sbfiz_arm_fields((is_64, rd, rn, lsb, width) in sbfiz_valid()) {
            let r = if is_64 { 64i64 } else { 32 };
            let immr = (-(lsb as i64)).rem_euclid(r) as u32;
            let imms = width - 1;
            let sf = if is_64 { 1u32 } else { 0 };
            let w = sut_word(&ops4(is_64, rd, rn, lsb as i64, width as i64)).expect("SUT");
            let want = (sf << 31)
                | (0b100110 << 23)
                | (sf << 22)
                | (immr << 16)
                | (imms << 10)
                | (rn << 5)
                | rd;
            prop_assert_eq!(w, want, "ARM ARM SBFM/SBFIZ field layout");
            prop_assert_eq!(w >> 31, sf, "sf");
            prop_assert_eq!((w >> 29) & 0b11, 0b00, "opc=00 SBFM");
            prop_assert_eq!((w >> 23) & 0x3f, 0b100110, "bits[28:23]");
            prop_assert_eq!((w >> 22) & 1, sf, "N=sf");
            prop_assert_eq!((w >> 16) & 0x3f, immr, "immr=(-lsb MOD R)");
            prop_assert_eq!((w >> 10) & 0x3f, imms, "imms=width-1");
            prop_assert_eq!((w >> 5) & 0x1f, rn, "Rn");
            prop_assert_eq!(w & 0x1f, rd, "Rd");
        }

        #[test]
        fn encode_sbfiz_metamorphic_rd_rn(
            is_64 in any::<bool>(),
            rd in 0u32..=30,
            rn in 0u32..=30,
            lsb_w in 0u32..=1,
        ) {
            let r = if is_64 { 64u32 } else { 32 };
            let lsb = if lsb_w == 0 { 0 } else { 1 };
            let width = 1u32;
            prop_assume!(lsb < r && width <= r - lsb);
            let base = sut_word(&ops4(is_64, rd, rn, lsb as i64, width as i64)).expect("base");
            let w_rd = sut_word(&ops4(is_64, rd + 1, rn, lsb as i64, width as i64)).expect("rd+1");
            let w_rn = sut_word(&ops4(is_64, rd, rn + 1, lsb as i64, width as i64)).expect("rn+1");
            prop_assert_eq!(w_rd & 0x1f, rd + 1, "Rd+1 updates Rd field");
            prop_assert_eq!(w_rd & !0x1fu32, base & !0x1fu32, "Rd+1 leaves other fields unchanged");
            prop_assert_eq!((w_rn >> 5) & 0x1f, rn + 1, "Rn+1 updates Rn field");
            prop_assert_eq!(w_rn & !(0x1fu32 << 5), base & !(0x1fu32 << 5), "Rn+1 leaves other fields unchanged");
        }

        #[test]
        fn encode_sbfiz_neg_arity(
            len in 0usize..=3,
            is_64 in any::<bool>(),
            rd in 0u32..=31,
            rn in 0u32..=31,
        ) {
            let mut ops = vec![
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
                Operand::Imm(0),
                Operand::Imm(1),
            ];
            ops.truncate(len);
            prop_assert!(
                encode_sbfiz(&ops).is_err(),
                "SBFIZ with {} operands must Err (llvm-mc: too few operands)",
                len
            );
        }

        #[test]
        fn encode_sbfiz_neg_extra_operand(
            (is_64, rd, rn, lsb, width) in sbfiz_valid(),
            extra in extra_operand(),
        ) {
            let mut ops = ops4(is_64, rd, rn, lsb as i64, width as i64).to_vec();
            ops.push(extra);
            prop_assert!(
                encode_sbfiz(&ops).is_err(),
                "SBFIZ has no 5th operand; extra operand must Err (llvm-mc rejects it)"
            );
        }

        #[test]
        fn encode_sbfiz_neg_sp(
            which in 0u32..=1,
            sp64 in any::<bool>(),
            is_64 in any::<bool>(),
            other in 0u32..=30,
        ) {
            let sp = if sp64 { "sp" } else { "wsp" };
            let mut ops = ops4(is_64, other, other, 0, 1);
            ops[which as usize] = Operand::Reg(sp.to_string());
            prop_assert!(
                encode_sbfiz(&ops).is_err(),
                "SP/WSP is not a valid SBFIZ operand (which={} sp={})",
                which,
                sp
            );
        }

        #[test]
        fn encode_sbfiz_neg_lsb_width(
            (is_64, rd, rn, lsb, width) in any::<bool>().prop_flat_map(|is_64| {
                (Just(is_64), 0u32..=31, 0u32..=31, invalid_lsb_width(is_64))
                    .prop_map(|(is_64, rd, rn, (lsb, width))| (is_64, rd, rn, lsb, width))
            }),
        ) {
            let r = if is_64 { 64i64 } else { 32 };
            prop_assume!(!(lsb >= 0 && width >= 1 && lsb < r && width <= r - lsb));
            let ops = ops4(is_64, rd, rn, lsb, width);
            prop_assert!(
                must_err(&ops),
                "SBFIZ lsb={} width={} R={} must Err (ARM: 0<=lsb<R, 1<=width<=R-lsb)",
                lsb,
                width,
                r
            );
        }

        #[test]
        fn encode_sbfiz_diff_alt_spellings(
            (is_64, rd, rn, lsb, width) in sbfiz_valid(),
            dest_spell in 0u32..=4,
            src_spell in 0u32..=4,
        ) {
            let dest = spell(is_64, rd, dest_spell);
            let src = spell(is_64, rn, src_spell);
            let asm = format!("sbfiz {}, {}, #{}, #{}", dest, src, lsb, width);
            let ops = [
                Operand::Reg(dest),
                Operand::Reg(src),
                Operand::Imm(lsb as i64),
                Operand::Imm(width as i64),
            ];
            let mc = llvm_mc_word(&asm).expect("llvm-mc");
            let sut = sut_word(&ops).expect("SUT");
            prop_assert_eq!(sut, mc, "SBFIZ alt-spelling mismatch for {}", asm);
        }

        #[test]
        fn encode_sbfiz_neg_mixed_width(
            rd in 0u32..=31,
            rn in 0u32..=31,
            rd64 in any::<bool>(),
            rn64 in any::<bool>(),
        ) {
            prop_assume!(rd64 != rn64);
            let ops = [
                Operand::Reg(gpr(rd64, rd)),
                Operand::Reg(gpr(rn64, rn)),
                Operand::Imm(0),
                Operand::Imm(1),
            ];
            prop_assert!(
                encode_sbfiz(&ops).is_err(),
                "SBFIZ mixed W/X (rd64={} rn64={}) must Err (llvm-mc rejects it)",
                rd64,
                rn64
            );
        }

        #[test]
        fn encode_sbfiz_neg_fp(
            which in 0u32..=1,
            prefix in prop::sample::select(vec!["d", "s", "q", "v", "h", "b"]),
            n in 0u32..=31,
        ) {
            let fp = format!("{}{}", prefix, n);
            let mut ops = ops4(true, 0, 1, 0, 1);
            ops[which as usize] = Operand::Reg(fp.clone());
            prop_assert!(
                encode_sbfiz(&ops).is_err(),
                "FP/SIMD register {} is not a valid SBFIZ operand (which={})",
                fp,
                which
            );
        }

        #[test]
        fn encode_sbfiz_neg_nonreg(
            which in 0u32..=3,
            bad in non_reg_operand(),
        ) {
            if which >= 2 {
                prop_assume!(!matches!(bad, Operand::Imm(_)));
            }
            let mut ops = ops4(false, 0, 1, 0, 1).to_vec();
            ops[which as usize] = bad;
            prop_assert!(
                encode_sbfiz(&ops).is_err(),
                "wrong operand kind at slot {} must Err",
                which
            );
        }

        #[test]
        fn encode_sbfiz_neg_invalid_name(
            which in 0u32..=1,
            name in invalid_name(),
        ) {
            let mut ops = ops4(false, 0, 1, 0, 1);
            ops[which as usize] = Operand::Reg(name.clone());
            prop_assert!(
                encode_sbfiz(&ops).is_err(),
                "invalid register name {:?} at slot {} must Err",
                name,
                which
            );
        }
    }

    #[test]
    fn test_encode_sbfiz_regression_extra_operand() {
        let mut ops = ops4(false, 0, 0, 0, 1).to_vec();
        ops.push(Operand::Reg("x0".into()));
        assert!(
            encode_sbfiz(&ops).is_err(),
            "SBFIZ w0, w0, #0, #1, x0 must Err; extra operand is invalid (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_sbfiz_regression_sp() {
        let ops = [
            Operand::Reg("wsp".into()),
            Operand::Reg("w0".into()),
            Operand::Imm(0),
            Operand::Imm(1),
        ];
        assert!(
            encode_sbfiz(&ops).is_err(),
            "SBFIZ wsp, w0, #0, #1 must Err; register 31 is ZR not SP/WSP (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_sbfiz_regression_width_zero() {
        let ops = ops4(false, 0, 0, 0, 0);
        let result = catch_unwind(AssertUnwindSafe(|| encode_sbfiz(&ops)));
        assert!(
            matches!(result, Ok(Err(_))),
            "SBFIZ w0, w0, #0, #0 must Err; width=0 is outside 1..=32-lsb (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_sbfiz_regression_mixed_width() {
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("w0".into()),
            Operand::Imm(0),
            Operand::Imm(1),
        ];
        assert!(
            encode_sbfiz(&ops).is_err(),
            "SBFIZ x0, w0, #0, #1 must Err; mixed W/X is invalid (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_sbfiz_regression_fp() {
        let ops = [
            Operand::Reg("d0".into()),
            Operand::Reg("x1".into()),
            Operand::Imm(0),
            Operand::Imm(1),
        ];
        assert!(
            encode_sbfiz(&ops).is_err(),
            "SBFIZ d0, x1, #0, #1 must Err; FP/SIMD registers are not SBFIZ operands (llvm-mc rejects it)"
        );
    }
}

#[cfg(test)]
mod encode_ubfiz_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler
    // Evidence: src/backend/arm/assembler/README.md:11 "accepts the same textual assembly that GCC's gas would consume";
    //   encoder/mod.rs "Encodes AArch64 instructions into 32-bit machine code words";
    //   encoder/mod.rs:889 "ubfiz" => encode_ubfiz; README.md:216 lists ubfiz;
    //   bitfield.rs:75 purpose comment UBFIZ -> UBFM #(-lsb MOD regsize), #(width-1);
    //   ARM ARM Bitfield Move UBFIZ alias of UBFM: sf 10 100110 N immr imms Rn Rd,
    //   N=sf, immr=(-lsb MOD datasize), imms=width-1; register 31 is ZR not SP.
    // Stronger considered:
    //   - State machine: rejected — encode_ubfiz is a pure function with no lifecycle
    //   - Algebraic round-trip: rejected — no in-tree UBFIZ decoder
    //   - encode_ubfm as differential sibling: rejected — same-job gate fails (raw immr/imms form);
    //     used only as algebraic alias after the ARM mapping
    //   - encode_sbfiz / encode_ubfx / encode_bfi: rejected — SBFM/UBFX/BFM opc or different alias mapping
    // Weaker available: algebraic.metamorphic (UBFIZ alias of UBFM; Rd/Rn field independence),
    //   algebraic.invariant (ARM fields), negative_error (arity / extra / SP / lsb-width)
    // Differential: candidate=encode_ubfiz, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=[Reg(Rd), Reg(Rn), Imm(lsb), Imm(width)] <-> `ubfiz Rd, Rn, #lsb, #width`

    use super::{encode_ubfiz, encode_ubfm};
    use super::super::EncodeResult;
    use crate::backend::arm::assembler::parser::Operand;
    use proptest::prelude::*;
    use std::io::Write;
    use std::panic::{catch_unwind, AssertUnwindSafe};
    use std::process::{Command, Stdio};

    const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
    const CASES: u32 = 1000;

    fn cfg() -> ProptestConfig {
        ProptestConfig::with_cases(CASES)
    }

    fn gpr(is_64: bool, n: u32) -> String {
        if is_64 {
            if n == 31 {
                "xzr".into()
            } else {
                format!("x{n}")
            }
        } else if n == 31 {
            "wzr".into()
        } else {
            format!("w{n}")
        }
    }

    fn lsb_width(r: u32) -> impl Strategy<Value = (u32, u32)> {
        prop_oneof![Just(0u32), Just(r - 1), 0u32..r].prop_flat_map(move |lsb| {
            let max_w = r - lsb;
            prop_oneof![Just(1u32), Just(max_w), 1u32..=max_w]
                .prop_map(move |width| (lsb, width))
        })
    }

    fn ubfiz_valid() -> impl Strategy<Value = (bool, u32, u32, u32, u32)> {
        any::<bool>().prop_flat_map(|is_64| {
            let r = if is_64 { 64u32 } else { 32 };
            (
                Just(is_64),
                0u32..=31,
                0u32..=31,
                lsb_width(r),
            )
                .prop_map(|(is_64, rd, rn, (lsb, width))| (is_64, rd, rn, lsb, width))
        })
    }

    fn invalid_lsb_width(is_64: bool) -> impl Strategy<Value = (i64, i64)> {
        let r = if is_64 { 64i64 } else { 32 };
        prop_oneof![
            (0i64..r).prop_map(|lsb| (lsb, 0i64)),
            (0i64..r).prop_map(|lsb| (lsb, -1i64)),
            (1i64..=r).prop_map(|width| (-1i64, width)),
            Just((r, 1i64)),
            Just((r, r)),
            Just((r - 1, 2i64)),
            Just((0i64, r + 1)),
            Just((r + 1, 1i64)),
        ]
    }

    fn ops4(is_64: bool, rd: u32, rn: u32, lsb: i64, width: i64) -> [Operand; 4] {
        [
            Operand::Reg(gpr(is_64, rd)),
            Operand::Reg(gpr(is_64, rn)),
            Operand::Imm(lsb),
            Operand::Imm(width),
        ]
    }

    fn sut_word(ops: &[Operand]) -> Result<u32, String> {
        match encode_ubfiz(ops)? {
            EncodeResult::Word(w) => Ok(w),
            other => Err(format!("expected Word, got {:?}", other)),
        }
    }

    fn ubfm_word(ops: &[Operand]) -> Result<u32, String> {
        match encode_ubfm(ops)? {
            EncodeResult::Word(w) => Ok(w),
            other => Err(format!("expected Word, got {:?}", other)),
        }
    }

    fn must_err(ops: &[Operand]) -> bool {
        match catch_unwind(AssertUnwindSafe(|| encode_ubfiz(ops))) {
            Ok(Err(_)) => true,
            _ => false,
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
            Just(Operand::Imm(-1)),
            Just(Operand::Shift {
                kind: "lsl".into(),
                amount: 0,
            }),
            Just(Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "8h".into(),
            }),
        ]
    }

    fn spell(is_64: bool, n: u32, kind: u32) -> String {
        match kind {
            0 if n == 31 => {
                if is_64 {
                    "x31".into()
                } else {
                    "w31".into()
                }
            }
            1 if n == 31 => {
                if is_64 {
                    "XZR".into()
                } else {
                    "WZR".into()
                }
            }
            2 if n == 30 && is_64 => "LR".into(),
            3 => gpr(is_64, n).to_uppercase(),
            _ => gpr(is_64, n),
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
            "x-1".into(),
            "x99".into(),
            "w".into(),
        ])
    }

    fn non_reg_operand() -> impl Strategy<Value = Operand> {
        prop_oneof![
            any::<i64>().prop_map(Operand::Imm),
            Just(Operand::Shift {
                kind: "lsl".into(),
                amount: 0,
            }),
            Just(Operand::Mem {
                base: "x0".into(),
                offset: 0,
            }),
            Just(Operand::Label("L0".into())),
            Just(Operand::Symbol("foo".into())),
            Just(Operand::Cond("eq".into())),
            Just(Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "8b".into(),
            }),
        ]
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_ubfiz_kat_llvm_mc_w0_w1_lsb0_width1() {
        let want = 0x53000020u32;
        let mc = llvm_mc_word("ubfiz w0, w1, #0, #1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops4(false, 0, 1, 0, 1)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_ubfiz_kat_llvm_mc_w0_w1_lsb1_width1() {
        let want = 0x531f0020u32;
        let mc = llvm_mc_word("ubfiz w0, w1, #1, #1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops4(false, 0, 1, 1, 1)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_ubfiz_kat_llvm_mc_x0_x1_lsb1_width8() {
        let want = 0xd37f1c20u32;
        let mc = llvm_mc_word("ubfiz x0, x1, #1, #8").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops4(true, 0, 1, 1, 8)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_ubfiz_kat_llvm_mc_wzr_wzr_lsb31_width1() {
        let want = 0x530103ffu32;
        let mc = llvm_mc_word("ubfiz wzr, wzr, #31, #1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops4(false, 31, 31, 31, 1)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_ubfiz_kat_llvm_mc_x0_xzr_lsb63_width1() {
        let want = 0xd34103e0u32;
        let mc = llvm_mc_word("ubfiz x0, xzr, #63, #1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops4(true, 0, 31, 63, 1)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_ubfiz_kat_llvm_mc_lr_x1_lsb8_width16() {
        let want = 0xd3783c3eu32;
        let mc = llvm_mc_word("ubfiz lr, x1, #8, #16").expect("llvm-mc LR KAT");
        assert_eq!(mc, want, "llvm-mc LR KAT mapping broken");
        let ops = [
            Operand::Reg("lr".into()),
            Operand::Reg("x1".into()),
            Operand::Imm(8),
            Operand::Imm(16),
        ];
        let sut = sut_word(&ops).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    proptest! {
        #![proptest_config(cfg())]

        #[test]
        fn encode_ubfiz_diff_valid_gpr((is_64, rd, rn, lsb, width) in ubfiz_valid()) {
            let dest = gpr(is_64, rd);
            let src = gpr(is_64, rn);
            let asm = format!("ubfiz {}, {}, #{}, #{}", dest, src, lsb, width);
            let ops = ops4(is_64, rd, rn, lsb as i64, width as i64);
            let mc = llvm_mc_word(&asm).expect("llvm-mc");
            let sut = sut_word(&ops).expect("SUT");
            prop_assert_eq!(sut, mc, "UBFIZ mismatch for {}", asm);
        }

        #[test]
        fn encode_ubfiz_alias_ubfm((is_64, rd, rn, lsb, width) in ubfiz_valid()) {
            let r = if is_64 { 64i64 } else { 32 };
            let immr = (-(lsb as i64)).rem_euclid(r);
            let imms = (width as i64) - 1;
            let ubfiz = sut_word(&ops4(is_64, rd, rn, lsb as i64, width as i64)).expect("UBFIZ");
            let ubfm = ubfm_word(&[
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
                Operand::Imm(immr),
                Operand::Imm(imms),
            ]).expect("UBFM");
            prop_assert_eq!(ubfiz, ubfm, "UBFIZ must alias UBFM #(-lsb MOD {}), #(width-1)", r);
        }

        #[test]
        fn encode_ubfiz_arm_fields((is_64, rd, rn, lsb, width) in ubfiz_valid()) {
            let r = if is_64 { 64i64 } else { 32 };
            let immr = (-(lsb as i64)).rem_euclid(r) as u32;
            let imms = width - 1;
            let sf = if is_64 { 1u32 } else { 0 };
            let w = sut_word(&ops4(is_64, rd, rn, lsb as i64, width as i64)).expect("SUT");
            let want = (sf << 31)
                | (0b10 << 29)
                | (0b100110 << 23)
                | (sf << 22)
                | (immr << 16)
                | (imms << 10)
                | (rn << 5)
                | rd;
            prop_assert_eq!(w, want, "ARM ARM UBFM/UBFIZ field layout");
            prop_assert_eq!(w >> 31, sf, "sf");
            prop_assert_eq!((w >> 29) & 0b11, 0b10, "opc=10 UBFM");
            prop_assert_eq!((w >> 23) & 0x3f, 0b100110, "bits[28:23]");
            prop_assert_eq!((w >> 22) & 1, sf, "N=sf");
            prop_assert_eq!((w >> 16) & 0x3f, immr, "immr=(-lsb MOD R)");
            prop_assert_eq!((w >> 10) & 0x3f, imms, "imms=width-1");
            prop_assert_eq!((w >> 5) & 0x1f, rn, "Rn");
            prop_assert_eq!(w & 0x1f, rd, "Rd");
        }

        #[test]
        fn encode_ubfiz_metamorphic_rd_rn(
            is_64 in any::<bool>(),
            rd in 0u32..=30,
            rn in 0u32..=30,
            lsb_w in 0u32..=1,
        ) {
            let r = if is_64 { 64u32 } else { 32 };
            let lsb = if lsb_w == 0 { 0 } else { 1 };
            let width = 1u32;
            prop_assume!(lsb < r && width <= r - lsb);
            let base = sut_word(&ops4(is_64, rd, rn, lsb as i64, width as i64)).expect("base");
            let w_rd = sut_word(&ops4(is_64, rd + 1, rn, lsb as i64, width as i64)).expect("rd+1");
            let w_rn = sut_word(&ops4(is_64, rd, rn + 1, lsb as i64, width as i64)).expect("rn+1");
            prop_assert_eq!(w_rd & 0x1f, rd + 1, "Rd+1 updates Rd field");
            prop_assert_eq!(w_rd & !0x1fu32, base & !0x1fu32, "Rd+1 leaves other fields unchanged");
            prop_assert_eq!((w_rn >> 5) & 0x1f, rn + 1, "Rn+1 updates Rn field");
            prop_assert_eq!(w_rn & !(0x1fu32 << 5), base & !(0x1fu32 << 5), "Rn+1 leaves other fields unchanged");
        }

        #[test]
        fn encode_ubfiz_neg_arity(
            len in 0usize..=3,
            is_64 in any::<bool>(),
            rd in 0u32..=31,
            rn in 0u32..=31,
        ) {
            let mut ops = vec![
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
                Operand::Imm(0),
                Operand::Imm(1),
            ];
            ops.truncate(len);
            prop_assert!(
                encode_ubfiz(&ops).is_err(),
                "UBFIZ with {} operands must Err (llvm-mc: too few operands)",
                len
            );
        }

        #[test]
        fn encode_ubfiz_neg_extra_operand(
            (is_64, rd, rn, lsb, width) in ubfiz_valid(),
            extra in extra_operand(),
        ) {
            let mut ops = ops4(is_64, rd, rn, lsb as i64, width as i64).to_vec();
            ops.push(extra);
            prop_assert!(
                encode_ubfiz(&ops).is_err(),
                "UBFIZ has no 5th operand; extra operand must Err (llvm-mc rejects it)"
            );
        }

        #[test]
        fn encode_ubfiz_neg_sp(
            which in 0u32..=1,
            sp64 in any::<bool>(),
            is_64 in any::<bool>(),
            other in 0u32..=30,
        ) {
            let sp = if sp64 { "sp" } else { "wsp" };
            let mut ops = ops4(is_64, other, other, 0, 1);
            ops[which as usize] = Operand::Reg(sp.to_string());
            prop_assert!(
                encode_ubfiz(&ops).is_err(),
                "SP/WSP is not a valid UBFIZ operand (which={} sp={})",
                which,
                sp
            );
        }

        #[test]
        fn encode_ubfiz_neg_lsb_width(
            (is_64, rd, rn, lsb, width) in any::<bool>().prop_flat_map(|is_64| {
                (Just(is_64), 0u32..=31, 0u32..=31, invalid_lsb_width(is_64))
                    .prop_map(|(is_64, rd, rn, (lsb, width))| (is_64, rd, rn, lsb, width))
            }),
        ) {
            let r = if is_64 { 64i64 } else { 32 };
            prop_assume!(!(lsb >= 0 && width >= 1 && lsb < r && width <= r - lsb));
            let ops = ops4(is_64, rd, rn, lsb, width);
            prop_assert!(
                must_err(&ops),
                "UBFIZ lsb={} width={} R={} must Err (ARM: 0<=lsb<R, 1<=width<=R-lsb)",
                lsb,
                width,
                r
            );
        }

        #[test]
        fn encode_ubfiz_diff_alt_spellings(
            (is_64, rd, rn, lsb, width) in ubfiz_valid(),
            dest_spell in 0u32..=4,
            src_spell in 0u32..=4,
        ) {
            let dest = spell(is_64, rd, dest_spell);
            let src = spell(is_64, rn, src_spell);
            let asm = format!("ubfiz {}, {}, #{}, #{}", dest, src, lsb, width);
            let ops = [
                Operand::Reg(dest),
                Operand::Reg(src),
                Operand::Imm(lsb as i64),
                Operand::Imm(width as i64),
            ];
            let mc = llvm_mc_word(&asm).expect("llvm-mc");
            let sut = sut_word(&ops).expect("SUT");
            prop_assert_eq!(sut, mc, "UBFIZ alt-spelling mismatch for {}", asm);
        }

        #[test]
        fn encode_ubfiz_neg_mixed_width(
            rd in 0u32..=31,
            rn in 0u32..=31,
            rd64 in any::<bool>(),
            rn64 in any::<bool>(),
        ) {
            prop_assume!(rd64 != rn64);
            let ops = [
                Operand::Reg(gpr(rd64, rd)),
                Operand::Reg(gpr(rn64, rn)),
                Operand::Imm(0),
                Operand::Imm(1),
            ];
            prop_assert!(
                encode_ubfiz(&ops).is_err(),
                "UBFIZ mixed W/X (rd64={} rn64={}) must Err (llvm-mc rejects it)",
                rd64,
                rn64
            );
        }

        #[test]
        fn encode_ubfiz_neg_fp(
            which in 0u32..=1,
            prefix in prop::sample::select(vec!["d", "s", "q", "v", "h", "b"]),
            n in 0u32..=31,
        ) {
            let fp = format!("{}{}", prefix, n);
            let mut ops = ops4(true, 0, 1, 0, 1);
            ops[which as usize] = Operand::Reg(fp.clone());
            prop_assert!(
                encode_ubfiz(&ops).is_err(),
                "FP/SIMD register {} is not a valid UBFIZ operand (which={})",
                fp,
                which
            );
        }

        #[test]
        fn encode_ubfiz_neg_nonreg(
            which in 0u32..=3,
            bad in non_reg_operand(),
        ) {
            if which >= 2 {
                prop_assume!(!matches!(bad, Operand::Imm(_)));
            }
            let mut ops = ops4(false, 0, 1, 0, 1).to_vec();
            ops[which as usize] = bad;
            prop_assert!(
                encode_ubfiz(&ops).is_err(),
                "wrong operand kind at slot {} must Err",
                which
            );
        }

        #[test]
        fn encode_ubfiz_neg_invalid_name(
            which in 0u32..=1,
            name in invalid_name(),
        ) {
            let mut ops = ops4(false, 0, 1, 0, 1);
            ops[which as usize] = Operand::Reg(name.clone());
            prop_assert!(
                encode_ubfiz(&ops).is_err(),
                "invalid register name {:?} at slot {} must Err",
                name,
                which
            );
        }
    }

    #[test]
    fn test_encode_ubfiz_regression_extra_operand() {
        let mut ops = ops4(false, 0, 0, 0, 1).to_vec();
        ops.push(Operand::Reg("x0".into()));
        assert!(
            encode_ubfiz(&ops).is_err(),
            "UBFIZ w0, w0, #0, #1, x0 must Err; extra operand is invalid (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_ubfiz_regression_sp() {
        let ops = [
            Operand::Reg("wsp".into()),
            Operand::Reg("w0".into()),
            Operand::Imm(0),
            Operand::Imm(1),
        ];
        assert!(
            encode_ubfiz(&ops).is_err(),
            "UBFIZ wsp, w0, #0, #1 must Err; register 31 is ZR not SP/WSP (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_ubfiz_regression_width_zero() {
        let ops = ops4(false, 0, 0, 0, 0);
        let result = catch_unwind(AssertUnwindSafe(|| encode_ubfiz(&ops)));
        assert!(
            matches!(result, Ok(Err(_))),
            "UBFIZ w0, w0, #0, #0 must Err; width=0 is outside 1..=32-lsb (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_ubfiz_regression_mixed_width() {
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("w0".into()),
            Operand::Imm(0),
            Operand::Imm(1),
        ];
        assert!(
            encode_ubfiz(&ops).is_err(),
            "UBFIZ x0, w0, #0, #1 must Err; mixed W/X is invalid (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_ubfiz_regression_fp() {
        let ops = [
            Operand::Reg("d0".into()),
            Operand::Reg("x1".into()),
            Operand::Imm(0),
            Operand::Imm(1),
        ];
        assert!(
            encode_ubfiz(&ops).is_err(),
            "UBFIZ d0, x1, #0, #1 must Err; FP/SIMD registers are not UBFIZ operands (llvm-mc rejects it)"
        );
    }
}

#[cfg(test)]
mod encode_bfm_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler
    // Evidence: src/backend/arm/assembler/README.md:11 "accepts the same textual assembly that GCC's gas would consume";
    //   encoder/mod.rs "Encodes AArch64 instructions into 32-bit machine code words";
    //   encoder/mod.rs:891 "bfm" => encode_bfm; README.md:216 lists bfm;
    //   bitfield.rs:90 purpose comment Encode BFM Rd, Rn, #immr, #imms;
    //   ARM ARM Bitfield Move BFM: sf 01 100110 N immr imms Rn Rd,
    //   N=sf; 32-bit immr/imms in [0,31], 64-bit in [0,63]; register 31 is ZR not SP.
    // Stronger considered:
    //   - State machine: rejected — encode_bfm is a pure function with no lifecycle
    //   - Algebraic round-trip: rejected — no in-tree BFM decoder
    //   - encode_bfi / encode_bfxil as differential sibling: rejected — same-job gate fails (alias lsb/width);
    //     not used as same-job reference
    //   - encode_ubfm / encode_sbfm: rejected — UBFM/SBFM opc, different instruction
    // Weaker available: algebraic.metamorphic (Rd/Rn field independence),
    //   algebraic.invariant (ARM fields), negative_error (arity / extra / SP / immr-imms / mixed)
    // Differential: candidate=encode_bfm, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=[Reg(Rd), Reg(Rn), Imm(immr), Imm(imms)] <-> `bfm Rd, Rn, #immr, #imms`

    use super::encode_bfm;
    use super::super::EncodeResult;
    use crate::backend::arm::assembler::parser::Operand;
    use proptest::prelude::*;
    use std::io::Write;
    use std::panic::{catch_unwind, AssertUnwindSafe};
    use std::process::{Command, Stdio};

    const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
    const CASES: u32 = 1000;

    fn cfg() -> ProptestConfig {
        ProptestConfig::with_cases(CASES)
    }

    fn gpr(is_64: bool, n: u32) -> String {
        if is_64 {
            if n == 31 {
                "xzr".into()
            } else {
                format!("x{n}")
            }
        } else if n == 31 {
            "wzr".into()
        } else {
            format!("w{n}")
        }
    }

    fn immr_imms(r: u32) -> impl Strategy<Value = (u32, u32)> {
        (
            prop_oneof![Just(0u32), Just(r - 1), 0u32..r],
            prop_oneof![Just(0u32), Just(r - 1), 0u32..r],
        )
    }

    fn bfm_valid() -> impl Strategy<Value = (bool, u32, u32, u32, u32)> {
        any::<bool>().prop_flat_map(|is_64| {
            let r = if is_64 { 64u32 } else { 32 };
            (
                Just(is_64),
                0u32..=31,
                0u32..=31,
                immr_imms(r),
            )
                .prop_map(|(is_64, rd, rn, (immr, imms))| (is_64, rd, rn, immr, imms))
        })
    }

    fn invalid_immr_imms(is_64: bool) -> impl Strategy<Value = (i64, i64)> {
        let r = if is_64 { 64i64 } else { 32 };
        prop_oneof![
            Just((-1i64, 0i64)),
            Just((0i64, -1i64)),
            Just((-1i64, -1i64)),
            Just((r, 0i64)),
            Just((0i64, r)),
            Just((r, r)),
            Just((r + 1, 0i64)),
            Just((0i64, r + 1)),
            Just((r + 1, r + 1)),
        ]
    }

    fn ops4(is_64: bool, rd: u32, rn: u32, immr: i64, imms: i64) -> [Operand; 4] {
        [
            Operand::Reg(gpr(is_64, rd)),
            Operand::Reg(gpr(is_64, rn)),
            Operand::Imm(immr),
            Operand::Imm(imms),
        ]
    }

    fn sut_word(ops: &[Operand]) -> Result<u32, String> {
        match encode_bfm(ops)? {
            EncodeResult::Word(w) => Ok(w),
            other => Err(format!("expected Word, got {:?}", other)),
        }
    }

    fn must_err(ops: &[Operand]) -> bool {
        match catch_unwind(AssertUnwindSafe(|| encode_bfm(ops))) {
            Ok(Err(_)) => true,
            _ => false,
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
            Just(Operand::Imm(-1)),
            Just(Operand::Shift {
                kind: "lsl".into(),
                amount: 0,
            }),
            Just(Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "8h".into(),
            }),
        ]
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_bfm_kat_llvm_mc_w0_w1_immr0_imms0() {
        let want = 0x33000020u32;
        let mc = llvm_mc_word("bfm w0, w1, #0, #0").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops4(false, 0, 1, 0, 0)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_bfm_kat_llvm_mc_w0_w1_immr1_imms0() {
        let want = 0x33010020u32;
        let mc = llvm_mc_word("bfm w0, w1, #1, #0").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops4(false, 0, 1, 1, 0)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_bfm_kat_llvm_mc_x0_x1_immr1_imms8() {
        let want = 0xb3412020u32;
        let mc = llvm_mc_word("bfm x0, x1, #1, #8").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops4(true, 0, 1, 1, 8)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_bfm_kat_llvm_mc_wzr_wzr_immr31_imms0() {
        let want = 0x331f03ffu32;
        let mc = llvm_mc_word("bfm wzr, wzr, #31, #0").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops4(false, 31, 31, 31, 0)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_bfm_kat_llvm_mc_x0_xzr_immr63_imms63() {
        let want = 0xb37fffe0u32;
        let mc = llvm_mc_word("bfm x0, xzr, #63, #63").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops4(true, 0, 31, 63, 63)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_bfm_kat_llvm_mc_lr_x1_immr8_imms16() {
        let want = 0xb348403eu32;
        let mc = llvm_mc_word("bfm lr, x1, #8, #16").expect("llvm-mc LR KAT");
        assert_eq!(mc, want, "llvm-mc LR KAT mapping broken");
        let ops = [
            Operand::Reg("lr".into()),
            Operand::Reg("x1".into()),
            Operand::Imm(8),
            Operand::Imm(16),
        ];
        let sut = sut_word(&ops).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    proptest! {
        #![proptest_config(cfg())]

        #[test]
        fn encode_bfm_diff_valid_gpr((is_64, rd, rn, immr, imms) in bfm_valid()) {
            let dest = gpr(is_64, rd);
            let src = gpr(is_64, rn);
            let asm = format!("bfm {}, {}, #{}, #{}", dest, src, immr, imms);
            let ops = ops4(is_64, rd, rn, immr as i64, imms as i64);
            let mc = llvm_mc_word(&asm).expect("llvm-mc");
            let sut = sut_word(&ops).expect("SUT");
            prop_assert_eq!(sut, mc, "BFM mismatch for {}", asm);
        }

        #[test]
        fn encode_bfm_arm_fields((is_64, rd, rn, immr, imms) in bfm_valid()) {
            let sf = if is_64 { 1u32 } else { 0 };
            let w = sut_word(&ops4(is_64, rd, rn, immr as i64, imms as i64)).expect("SUT");
            let want = (sf << 31)
                | (0b01 << 29)
                | (0b100110 << 23)
                | (sf << 22)
                | (immr << 16)
                | (imms << 10)
                | (rn << 5)
                | rd;
            prop_assert_eq!(w, want, "ARM ARM BFM field layout");
            prop_assert_eq!(w >> 31, sf, "sf");
            prop_assert_eq!((w >> 29) & 0b11, 0b01, "opc=01 BFM");
            prop_assert_eq!((w >> 23) & 0x3f, 0b100110, "bits[28:23]");
            prop_assert_eq!((w >> 22) & 1, sf, "N=sf");
            prop_assert_eq!((w >> 16) & 0x3f, immr, "immr");
            prop_assert_eq!((w >> 10) & 0x3f, imms, "imms");
            prop_assert_eq!((w >> 5) & 0x1f, rn, "Rn");
            prop_assert_eq!(w & 0x1f, rd, "Rd");
        }

        #[test]
        fn encode_bfm_metamorphic_rd_rn(
            is_64 in any::<bool>(),
            rd in 0u32..=30,
            rn in 0u32..=30,
            immr in 0u32..=1,
            imms in 0u32..=1,
        ) {
            let r = if is_64 { 64u32 } else { 32 };
            prop_assume!(immr < r && imms < r);
            let base = sut_word(&ops4(is_64, rd, rn, immr as i64, imms as i64)).expect("base");
            let w_rd = sut_word(&ops4(is_64, rd + 1, rn, immr as i64, imms as i64)).expect("rd+1");
            let w_rn = sut_word(&ops4(is_64, rd, rn + 1, immr as i64, imms as i64)).expect("rn+1");
            prop_assert_eq!(w_rd & 0x1f, rd + 1, "Rd+1 updates Rd field");
            prop_assert_eq!(w_rd & !0x1fu32, base & !0x1fu32, "Rd+1 leaves other fields unchanged");
            prop_assert_eq!((w_rn >> 5) & 0x1f, rn + 1, "Rn+1 updates Rn field");
            prop_assert_eq!(w_rn & !(0x1fu32 << 5), base & !(0x1fu32 << 5), "Rn+1 leaves other fields unchanged");
        }

        #[test]
        fn encode_bfm_neg_arity(
            len in 0usize..=3,
            is_64 in any::<bool>(),
            rd in 0u32..=31,
            rn in 0u32..=31,
        ) {
            let mut ops = vec![
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
                Operand::Imm(0),
                Operand::Imm(0),
            ];
            ops.truncate(len);
            prop_assert!(
                encode_bfm(&ops).is_err(),
                "BFM with {} operands must Err (llvm-mc: too few operands)",
                len
            );
        }

        #[test]
        fn encode_bfm_neg_extra_operand(
            (is_64, rd, rn, immr, imms) in bfm_valid(),
            extra in extra_operand(),
        ) {
            let mut ops = ops4(is_64, rd, rn, immr as i64, imms as i64).to_vec();
            ops.push(extra);
            prop_assert!(
                encode_bfm(&ops).is_err(),
                "BFM has no 5th operand; extra operand must Err (llvm-mc rejects it)"
            );
        }

        #[test]
        fn encode_bfm_neg_sp(
            which in 0u32..=1,
            sp64 in any::<bool>(),
            is_64 in any::<bool>(),
            other in 0u32..=30,
        ) {
            let sp = if sp64 { "sp" } else { "wsp" };
            let mut ops = ops4(is_64, other, other, 0, 0);
            ops[which as usize] = Operand::Reg(sp.to_string());
            prop_assert!(
                encode_bfm(&ops).is_err(),
                "SP/WSP is not a valid BFM operand (which={} sp={})",
                which,
                sp
            );
        }

        #[test]
        fn encode_bfm_neg_immr_imms(
            (is_64, rd, rn, immr, imms) in any::<bool>().prop_flat_map(|is_64| {
                (Just(is_64), 0u32..=31, 0u32..=31, invalid_immr_imms(is_64))
                    .prop_map(|(is_64, rd, rn, (immr, imms))| (is_64, rd, rn, immr, imms))
            }),
        ) {
            let r = if is_64 { 64i64 } else { 32 };
            prop_assume!(!(immr >= 0 && imms >= 0 && immr < r && imms < r));
            let ops = ops4(is_64, rd, rn, immr, imms);
            prop_assert!(
                must_err(&ops),
                "BFM immr={} imms={} R={} must Err (ARM: 0<=immr,imms<R)",
                immr,
                imms,
                r
            );
        }

        #[test]
        fn encode_bfm_neg_mixed_width(
            rd in 0u32..=31,
            rn in 0u32..=31,
            rd64 in any::<bool>(),
            rn64 in any::<bool>(),
        ) {
            prop_assume!(rd64 != rn64);
            let ops = [
                Operand::Reg(gpr(rd64, rd)),
                Operand::Reg(gpr(rn64, rn)),
                Operand::Imm(0),
                Operand::Imm(0),
            ];
            prop_assert!(
                encode_bfm(&ops).is_err(),
                "BFM mixed W/X (rd64={} rn64={}) must Err (llvm-mc rejects it)",
                rd64,
                rn64
            );
        }

        #[test]
        fn encode_bfm_diff_alt_spellings(
            (is_64, rd, rn, immr, imms) in bfm_valid(),
            dest_spell in 0u32..=4,
            src_spell in 0u32..=4,
        ) {
            let dest = spell(is_64, rd, dest_spell);
            let src = spell(is_64, rn, src_spell);
            let asm = format!("bfm {}, {}, #{}, #{}", dest, src, immr, imms);
            let ops = [
                Operand::Reg(dest),
                Operand::Reg(src),
                Operand::Imm(immr as i64),
                Operand::Imm(imms as i64),
            ];
            let mc = llvm_mc_word(&asm).expect("llvm-mc");
            let sut = sut_word(&ops).expect("SUT");
            prop_assert_eq!(sut, mc, "BFM alt-spelling mismatch for {}", asm);
        }

        #[test]
        fn encode_bfm_neg_fp(
            which in 0u32..=1,
            prefix in prop::sample::select(vec!["d", "s", "q", "v", "h", "b"]),
            n in 0u32..=31,
        ) {
            let fp = format!("{}{}", prefix, n);
            let mut ops = ops4(true, 0, 1, 0, 0);
            ops[which as usize] = Operand::Reg(fp.clone());
            prop_assert!(
                encode_bfm(&ops).is_err(),
                "FP/SIMD register {} is not a valid BFM operand (which={})",
                fp,
                which
            );
        }

        #[test]
        fn encode_bfm_neg_nonreg(
            which in 0u32..=3,
            bad in non_reg_operand(),
        ) {
            if which >= 2 {
                prop_assume!(!matches!(bad, Operand::Imm(_)));
            }
            let mut ops = ops4(false, 0, 1, 0, 0).to_vec();
            ops[which as usize] = bad;
            prop_assert!(
                encode_bfm(&ops).is_err(),
                "wrong operand kind at slot {} must Err",
                which
            );
        }

        #[test]
        fn encode_bfm_neg_invalid_name(
            which in 0u32..=1,
            name in invalid_name(),
        ) {
            let mut ops = ops4(false, 0, 1, 0, 0);
            ops[which as usize] = Operand::Reg(name.clone());
            prop_assert!(
                encode_bfm(&ops).is_err(),
                "invalid register name {:?} at slot {} must Err",
                name,
                which
            );
        }
    }

    fn spell(is_64: bool, n: u32, kind: u32) -> String {
        match kind {
            0 if n == 31 => {
                if is_64 {
                    "x31".into()
                } else {
                    "w31".into()
                }
            }
            1 if n == 31 => {
                if is_64 {
                    "XZR".into()
                } else {
                    "WZR".into()
                }
            }
            2 if n == 30 && is_64 => "LR".into(),
            3 => gpr(is_64, n).to_uppercase(),
            _ => gpr(is_64, n),
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
            "x-1".into(),
            "x99".into(),
            "w".into(),
        ])
    }

    fn non_reg_operand() -> impl Strategy<Value = Operand> {
        prop_oneof![
            any::<i64>().prop_map(Operand::Imm),
            Just(Operand::Shift {
                kind: "lsl".into(),
                amount: 0,
            }),
            Just(Operand::Mem {
                base: "x0".into(),
                offset: 0,
            }),
            Just(Operand::Label("L0".into())),
            Just(Operand::Symbol("foo".into())),
            Just(Operand::Cond("eq".into())),
            Just(Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "8b".into(),
            }),
        ]
    }

    #[test]
    fn test_encode_bfm_regression_extra_operand() {
        let mut ops = ops4(false, 0, 0, 0, 0).to_vec();
        ops.push(Operand::Reg("x0".into()));
        assert!(
            encode_bfm(&ops).is_err(),
            "BFM w0, w0, #0, #0, x0 must Err; extra operand is invalid (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_bfm_regression_sp() {
        let ops = [
            Operand::Reg("wsp".into()),
            Operand::Reg("w0".into()),
            Operand::Imm(0),
            Operand::Imm(0),
        ];
        assert!(
            encode_bfm(&ops).is_err(),
            "BFM wsp, w0, #0, #0 must Err; register 31 is ZR not SP/WSP (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_bfm_regression_immr_neg() {
        let ops = ops4(false, 0, 0, -1, 0);
        let result = catch_unwind(AssertUnwindSafe(|| encode_bfm(&ops)));
        assert!(
            matches!(result, Ok(Err(_))),
            "BFM w0, w0, #-1, #0 must Err; immr=-1 is outside [0,31] (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_bfm_regression_mixed_width() {
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("w0".into()),
            Operand::Imm(0),
            Operand::Imm(0),
        ];
        assert!(
            encode_bfm(&ops).is_err(),
            "BFM x0, w0, #0, #0 must Err; mixed W/X is invalid (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_bfm_regression_fp() {
        let ops = [
            Operand::Reg("d0".into()),
            Operand::Reg("x1".into()),
            Operand::Imm(0),
            Operand::Imm(0),
        ];
        assert!(
            encode_bfm(&ops).is_err(),
            "BFM d0, x1, #0, #0 must Err; FP/SIMD registers are not BFM operands (llvm-mc rejects it)"
        );
    }
}

#[cfg(test)]
mod encode_sbfm_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler
    // Evidence: src/backend/arm/assembler/README.md:11 "accepts the same textual assembly that GCC's gas would consume";
    //   encoder/mod.rs "Encodes AArch64 instructions into 32-bit machine code words";
    //   encoder/mod.rs:888 "sbfm" => encode_sbfm; README.md:216 lists sbfm;
    //   bitfield.rs:48 purpose comment Encode SBFM Rd, Rn, #immr, #imms (raw form);
    //   ARM ARM Bitfield Move SBFM: sf 00 100110 N immr imms Rn Rd,
    //   N=sf; 32-bit immr/imms in [0,31], 64-bit in [0,63]; register 31 is ZR not SP.
    // Stronger considered:
    //   - State machine: rejected — encode_sbfm is a pure function with no lifecycle
    //   - Algebraic round-trip: rejected — no in-tree SBFM decoder
    //   - encode_sbfiz / encode_sbfx as differential sibling: rejected — same-job gate fails (alias lsb/width);
    //     used only as algebraic alias after the ARM mapping
    //   - encode_ubfm / encode_bfm: rejected — UBFM/BFM opc, different instruction
    // Weaker available: algebraic.metamorphic (Rd/Rn field independence),
    //   algebraic.invariant (ARM fields), negative_error (arity / extra / SP / immr-imms)
    // Differential: candidate=encode_sbfm, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=[Reg(Rd), Reg(Rn), Imm(immr), Imm(imms)] <-> `sbfm Rd, Rn, #immr, #imms`

    use super::encode_sbfm;
    use super::super::EncodeResult;
    use crate::backend::arm::assembler::parser::Operand;
    use proptest::prelude::*;
    use std::io::Write;
    use std::panic::{catch_unwind, AssertUnwindSafe};
    use std::process::{Command, Stdio};

    const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
    const CASES: u32 = 1000;

    fn cfg() -> ProptestConfig {
        ProptestConfig::with_cases(CASES)
    }

    fn gpr(is_64: bool, n: u32) -> String {
        if is_64 {
            if n == 31 {
                "xzr".into()
            } else {
                format!("x{n}")
            }
        } else if n == 31 {
            "wzr".into()
        } else {
            format!("w{n}")
        }
    }

    fn immr_imms(r: u32) -> impl Strategy<Value = (u32, u32)> {
        (
            prop_oneof![Just(0u32), Just(r - 1), 0u32..r],
            prop_oneof![Just(0u32), Just(r - 1), 0u32..r],
        )
    }

    fn sbfm_valid() -> impl Strategy<Value = (bool, u32, u32, u32, u32)> {
        any::<bool>().prop_flat_map(|is_64| {
            let r = if is_64 { 64u32 } else { 32 };
            (
                Just(is_64),
                0u32..=31,
                0u32..=31,
                immr_imms(r),
            )
                .prop_map(|(is_64, rd, rn, (immr, imms))| (is_64, rd, rn, immr, imms))
        })
    }

    fn invalid_immr_imms(is_64: bool) -> impl Strategy<Value = (i64, i64)> {
        let r = if is_64 { 64i64 } else { 32 };
        prop_oneof![
            Just((-1i64, 0i64)),
            Just((0i64, -1i64)),
            Just((-1i64, -1i64)),
            Just((r, 0i64)),
            Just((0i64, r)),
            Just((r, r)),
            Just((r + 1, 0i64)),
            Just((0i64, r + 1)),
            Just((r + 1, r + 1)),
        ]
    }

    fn ops4(is_64: bool, rd: u32, rn: u32, immr: i64, imms: i64) -> [Operand; 4] {
        [
            Operand::Reg(gpr(is_64, rd)),
            Operand::Reg(gpr(is_64, rn)),
            Operand::Imm(immr),
            Operand::Imm(imms),
        ]
    }

    fn sut_word(ops: &[Operand]) -> Result<u32, String> {
        match encode_sbfm(ops)? {
            EncodeResult::Word(w) => Ok(w),
            other => Err(format!("expected Word, got {:?}", other)),
        }
    }

    fn must_err(ops: &[Operand]) -> bool {
        match catch_unwind(AssertUnwindSafe(|| encode_sbfm(ops))) {
            Ok(Err(_)) => true,
            _ => false,
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
            Just(Operand::Imm(-1)),
            Just(Operand::Shift {
                kind: "lsl".into(),
                amount: 0,
            }),
            Just(Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "8h".into(),
            }),
        ]
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_sbfm_kat_llvm_mc_w0_w1_immr0_imms0() {
        let want = 0x13000020u32;
        let mc = llvm_mc_word("sbfm w0, w1, #0, #0").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops4(false, 0, 1, 0, 0)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_sbfm_kat_llvm_mc_w0_w1_immr1_imms0() {
        let want = 0x13010020u32;
        let mc = llvm_mc_word("sbfm w0, w1, #1, #0").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops4(false, 0, 1, 1, 0)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_sbfm_kat_llvm_mc_x0_x1_immr1_imms8() {
        let want = 0x93412020u32;
        let mc = llvm_mc_word("sbfm x0, x1, #1, #8").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops4(true, 0, 1, 1, 8)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_sbfm_kat_llvm_mc_wzr_wzr_immr31_imms0() {
        let want = 0x131f03ffu32;
        let mc = llvm_mc_word("sbfm wzr, wzr, #31, #0").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops4(false, 31, 31, 31, 0)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_sbfm_kat_llvm_mc_x0_xzr_immr63_imms63() {
        let want = 0x937fffe0u32;
        let mc = llvm_mc_word("sbfm x0, xzr, #63, #63").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops4(true, 0, 31, 63, 63)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_sbfm_kat_llvm_mc_lr_x1_immr8_imms16() {
        let want = 0x9348403eu32;
        let mc = llvm_mc_word("sbfm lr, x1, #8, #16").expect("llvm-mc LR KAT");
        assert_eq!(mc, want, "llvm-mc LR KAT mapping broken");
        let ops = [
            Operand::Reg("lr".into()),
            Operand::Reg("x1".into()),
            Operand::Imm(8),
            Operand::Imm(16),
        ];
        let sut = sut_word(&ops).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    proptest! {
        #![proptest_config(cfg())]

        #[test]
        fn encode_sbfm_diff_valid_gpr((is_64, rd, rn, immr, imms) in sbfm_valid()) {
            let dest = gpr(is_64, rd);
            let src = gpr(is_64, rn);
            let asm = format!("sbfm {}, {}, #{}, #{}", dest, src, immr, imms);
            let ops = ops4(is_64, rd, rn, immr as i64, imms as i64);
            let mc = llvm_mc_word(&asm).expect("llvm-mc");
            let sut = sut_word(&ops).expect("SUT");
            prop_assert_eq!(sut, mc, "SBFM mismatch for {}", asm);
        }

        #[test]
        fn encode_sbfm_arm_fields((is_64, rd, rn, immr, imms) in sbfm_valid()) {
            let sf = if is_64 { 1u32 } else { 0 };
            let w = sut_word(&ops4(is_64, rd, rn, immr as i64, imms as i64)).expect("SUT");
            let want = (sf << 31)
                | (0b100110 << 23)
                | (sf << 22)
                | (immr << 16)
                | (imms << 10)
                | (rn << 5)
                | rd;
            prop_assert_eq!(w, want, "ARM ARM SBFM field layout");
            prop_assert_eq!(w >> 31, sf, "sf");
            prop_assert_eq!((w >> 29) & 0b11, 0b00, "opc=00 SBFM");
            prop_assert_eq!((w >> 23) & 0x3f, 0b100110, "bits[28:23]");
            prop_assert_eq!((w >> 22) & 1, sf, "N=sf");
            prop_assert_eq!((w >> 16) & 0x3f, immr, "immr");
            prop_assert_eq!((w >> 10) & 0x3f, imms, "imms");
            prop_assert_eq!((w >> 5) & 0x1f, rn, "Rn");
            prop_assert_eq!(w & 0x1f, rd, "Rd");
        }

        #[test]
        fn encode_sbfm_metamorphic_rd_rn(
            is_64 in any::<bool>(),
            rd in 0u32..=30,
            rn in 0u32..=30,
            imm in 0u32..=1,
        ) {
            let r = if is_64 { 64u32 } else { 32 };
            prop_assume!(imm < r);
            let base = sut_word(&ops4(is_64, rd, rn, imm as i64, imm as i64)).expect("base");
            let w_rd = sut_word(&ops4(is_64, rd + 1, rn, imm as i64, imm as i64)).expect("rd+1");
            let w_rn = sut_word(&ops4(is_64, rd, rn + 1, imm as i64, imm as i64)).expect("rn+1");
            prop_assert_eq!(w_rd & 0x1f, rd + 1, "Rd+1 updates Rd field");
            prop_assert_eq!(w_rd & !0x1fu32, base & !0x1fu32, "Rd+1 leaves other fields unchanged");
            prop_assert_eq!((w_rn >> 5) & 0x1f, rn + 1, "Rn+1 updates Rn field");
            prop_assert_eq!(w_rn & !(0x1fu32 << 5), base & !(0x1fu32 << 5), "Rn+1 leaves other fields unchanged");
        }

        #[test]
        fn encode_sbfm_neg_arity(
            len in 0usize..=3,
            is_64 in any::<bool>(),
            rd in 0u32..=31,
            rn in 0u32..=31,
        ) {
            let mut ops = vec![
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
                Operand::Imm(0),
                Operand::Imm(0),
            ];
            ops.truncate(len);
            prop_assert!(
                encode_sbfm(&ops).is_err(),
                "SBFM with {} operands must Err (llvm-mc: too few operands)",
                len
            );
        }

        #[test]
        fn encode_sbfm_neg_extra_operand(
            (is_64, rd, rn, immr, imms) in sbfm_valid(),
            extra in extra_operand(),
        ) {
            let mut ops = ops4(is_64, rd, rn, immr as i64, imms as i64).to_vec();
            ops.push(extra);
            prop_assert!(
                encode_sbfm(&ops).is_err(),
                "SBFM has no 5th operand; extra operand must Err (llvm-mc rejects it)"
            );
        }

        #[test]
        fn encode_sbfm_neg_sp(
            which in 0u32..=1,
            sp64 in any::<bool>(),
            is_64 in any::<bool>(),
            other in 0u32..=30,
        ) {
            let sp = if sp64 { "sp" } else { "wsp" };
            let mut ops = ops4(is_64, other, other, 0, 0);
            ops[which as usize] = Operand::Reg(sp.to_string());
            prop_assert!(
                encode_sbfm(&ops).is_err(),
                "SP/WSP is not a valid SBFM operand (which={} sp={})",
                which,
                sp
            );
        }

        #[test]
        fn encode_sbfm_neg_immr_imms(
            (is_64, rd, rn, immr, imms) in any::<bool>().prop_flat_map(|is_64| {
                (Just(is_64), 0u32..=31, 0u32..=31, invalid_immr_imms(is_64))
                    .prop_map(|(is_64, rd, rn, (immr, imms))| (is_64, rd, rn, immr, imms))
            }),
        ) {
            let r = if is_64 { 64i64 } else { 32 };
            prop_assume!(!(immr >= 0 && imms >= 0 && immr < r && imms < r));
            let ops = ops4(is_64, rd, rn, immr, imms);
            prop_assert!(
                must_err(&ops),
                "SBFM immr={} imms={} R={} must Err (ARM: 0<=immr,imms<R)",
                immr,
                imms,
                r
            );
        }

        #[test]
        fn encode_sbfm_neg_mixed_width(
            rd in 0u32..=31,
            rn in 0u32..=31,
            rd64 in any::<bool>(),
            rn64 in any::<bool>(),
        ) {
            prop_assume!(rd64 != rn64);
            let ops = [
                Operand::Reg(gpr(rd64, rd)),
                Operand::Reg(gpr(rn64, rn)),
                Operand::Imm(0),
                Operand::Imm(0),
            ];
            prop_assert!(
                encode_sbfm(&ops).is_err(),
                "SBFM mixed W/X (rd64={} rn64={}) must Err (llvm-mc rejects it)",
                rd64,
                rn64
            );
        }

        #[test]
        fn encode_sbfm_diff_alt_spellings(
            (is_64, rd, rn, immr, imms) in sbfm_valid(),
            dest_spell in 0u32..=4,
            src_spell in 0u32..=4,
        ) {
            let dest = spell(is_64, rd, dest_spell);
            let src = spell(is_64, rn, src_spell);
            let asm = format!("sbfm {}, {}, #{}, #{}", dest, src, immr, imms);
            let ops = [
                Operand::Reg(dest),
                Operand::Reg(src),
                Operand::Imm(immr as i64),
                Operand::Imm(imms as i64),
            ];
            let mc = llvm_mc_word(&asm).expect("llvm-mc");
            let sut = sut_word(&ops).expect("SUT");
            prop_assert_eq!(sut, mc, "SBFM alt-spelling mismatch for {}", asm);
        }

        #[test]
        fn encode_sbfm_neg_fp(
            which in 0u32..=1,
            prefix in prop::sample::select(vec!["d", "s", "q", "v", "h", "b"]),
            n in 0u32..=31,
        ) {
            let fp = format!("{}{}", prefix, n);
            let mut ops = ops4(true, 0, 1, 0, 0);
            ops[which as usize] = Operand::Reg(fp.clone());
            prop_assert!(
                encode_sbfm(&ops).is_err(),
                "FP/SIMD register {} is not a valid SBFM operand (which={})",
                fp,
                which
            );
        }

        #[test]
        fn encode_sbfm_neg_nonreg(
            which in 0u32..=3,
            bad in non_reg_operand(),
        ) {
            if which >= 2 {
                prop_assume!(!matches!(bad, Operand::Imm(_)));
            }
            let mut ops = ops4(false, 0, 1, 0, 0).to_vec();
            ops[which as usize] = bad;
            prop_assert!(
                encode_sbfm(&ops).is_err(),
                "wrong operand kind at slot {} must Err",
                which
            );
        }

        #[test]
        fn encode_sbfm_neg_invalid_name(
            which in 0u32..=1,
            name in invalid_name(),
        ) {
            let mut ops = ops4(false, 0, 1, 0, 0);
            ops[which as usize] = Operand::Reg(name.clone());
            prop_assert!(
                encode_sbfm(&ops).is_err(),
                "invalid register name {:?} at slot {} must Err",
                name,
                which
            );
        }
    }

    fn spell(is_64: bool, n: u32, kind: u32) -> String {
        match kind {
            0 if n == 31 => {
                if is_64 {
                    "x31".into()
                } else {
                    "w31".into()
                }
            }
            1 if n == 31 => {
                if is_64 {
                    "XZR".into()
                } else {
                    "WZR".into()
                }
            }
            2 if n == 30 && is_64 => "LR".into(),
            3 => gpr(is_64, n).to_uppercase(),
            _ => gpr(is_64, n),
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
            "x-1".into(),
            "x99".into(),
            "w".into(),
        ])
    }

    fn non_reg_operand() -> impl Strategy<Value = Operand> {
        prop_oneof![
            any::<i64>().prop_map(Operand::Imm),
            Just(Operand::Shift {
                kind: "lsl".into(),
                amount: 0,
            }),
            Just(Operand::Mem {
                base: "x0".into(),
                offset: 0,
            }),
            Just(Operand::Label("L0".into())),
            Just(Operand::Symbol("foo".into())),
            Just(Operand::Cond("eq".into())),
            Just(Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "8b".into(),
            }),
        ]
    }

    #[test]
    fn test_encode_sbfm_regression_extra_operand() {
        let mut ops = ops4(false, 0, 0, 0, 0).to_vec();
        ops.push(Operand::Reg("x0".into()));
        assert!(
            encode_sbfm(&ops).is_err(),
            "SBFM w0, w0, #0, #0, x0 must Err; extra operand is invalid (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_sbfm_regression_sp() {
        let ops = [
            Operand::Reg("wsp".into()),
            Operand::Reg("w0".into()),
            Operand::Imm(0),
            Operand::Imm(0),
        ];
        assert!(
            encode_sbfm(&ops).is_err(),
            "SBFM wsp, w0, #0, #0 must Err; register 31 is ZR not SP/WSP (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_sbfm_regression_immr_neg() {
        let ops = ops4(false, 0, 0, -1, 0);
        let result = catch_unwind(AssertUnwindSafe(|| encode_sbfm(&ops)));
        assert!(
            matches!(result, Ok(Err(_))),
            "SBFM w0, w0, #-1, #0 must Err; immr=-1 is outside [0,31] (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_sbfm_regression_mixed_width() {
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("w0".into()),
            Operand::Imm(0),
            Operand::Imm(0),
        ];
        assert!(
            encode_sbfm(&ops).is_err(),
            "SBFM x0, w0, #0, #0 must Err; mixed W/X is invalid (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_sbfm_regression_fp() {
        let ops = [
            Operand::Reg("d0".into()),
            Operand::Reg("x1".into()),
            Operand::Imm(0),
            Operand::Imm(0),
        ];
        assert!(
            encode_sbfm(&ops).is_err(),
            "SBFM d0, x1, #0, #0 must Err; FP/SIMD registers are not SBFM operands (llvm-mc rejects it)"
        );
    }
}

#[cfg(test)]
mod encode_sbfx_pbt {
    // Oracle: differential — llvm-mc AArch64 assembler
    // Evidence: src/backend/arm/assembler/README.md:11 "accepts the same textual assembly that GCC's gas would consume";
    //   encoder/mod.rs "Encodes AArch64 instructions into 32-bit machine code words";
    //   encoder/mod.rs:886 "sbfx" => encode_sbfx; README.md:216 lists sbfx;
    //   bitfield.rs:21 purpose comment SBFX -> SBFM #lsb, #(lsb+width-1);
    //   ARM ARM Signed Bitfield Extract SBFX alias of SBFM: sf 00 100110 N immr imms Rn Rd,
    //   N=sf, immr=lsb, imms=lsb+width-1; register 31 is ZR not SP.
    // Stronger considered:
    //   - State machine: rejected — encode_sbfx is a pure function with no lifecycle
    //   - Algebraic round-trip: rejected — no in-tree SBFX decoder
    //   - encode_sbfm as differential sibling: rejected — same-job gate fails (raw immr/imms form);
    //     used only as algebraic alias after the ARM mapping
    //   - encode_ubfx / encode_bfxil: rejected — UBFM/BFM opc, different instruction
    // Weaker available: algebraic.metamorphic (SBFX alias of SBFM; Rd/Rn field independence),
    //   algebraic.invariant (ARM fields), negative_error (arity / extra / SP / lsb-width)
    // Differential: candidate=encode_sbfx, reference=llvm-mc -triple=aarch64 -show-encoding,
    //   SUT-boundary=internal-helper of GNU-style assembler,
    //   mapping=[Reg(Rd), Reg(Rn), Imm(lsb), Imm(width)] <-> `sbfx Rd, Rn, #lsb, #width`

    use super::{encode_sbfx, encode_sbfm};
    use super::super::EncodeResult;
    use crate::backend::arm::assembler::parser::Operand;
    use proptest::prelude::*;
    use std::io::Write;
    use std::panic::{catch_unwind, AssertUnwindSafe};
    use std::process::{Command, Stdio};

    const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
    const CASES: u32 = 1000;

    fn cfg() -> ProptestConfig {
        ProptestConfig::with_cases(CASES)
    }

    fn gpr(is_64: bool, n: u32) -> String {
        if is_64 {
            if n == 31 {
                "xzr".into()
            } else {
                format!("x{n}")
            }
        } else if n == 31 {
            "wzr".into()
        } else {
            format!("w{n}")
        }
    }

    fn lsb_width(r: u32) -> impl Strategy<Value = (u32, u32)> {
        prop_oneof![Just(0u32), Just(r - 1), 0u32..r].prop_flat_map(move |lsb| {
            let max_w = r - lsb;
            prop_oneof![Just(1u32), Just(max_w), 1u32..=max_w]
                .prop_map(move |width| (lsb, width))
        })
    }

    fn sbfx_valid() -> impl Strategy<Value = (bool, u32, u32, u32, u32)> {
        any::<bool>().prop_flat_map(|is_64| {
            let r = if is_64 { 64u32 } else { 32 };
            (
                Just(is_64),
                0u32..=31,
                0u32..=31,
                lsb_width(r),
            )
                .prop_map(|(is_64, rd, rn, (lsb, width))| (is_64, rd, rn, lsb, width))
        })
    }

    fn invalid_lsb_width(is_64: bool) -> impl Strategy<Value = (i64, i64)> {
        let r = if is_64 { 64i64 } else { 32 };
        prop_oneof![
            (0i64..r).prop_map(|lsb| (lsb, 0i64)),
            (0i64..r).prop_map(|lsb| (lsb, -1i64)),
            (1i64..=r).prop_map(|width| (-1i64, width)),
            Just((r, 1i64)),
            Just((r, r)),
            Just((r - 1, 2i64)),
            Just((0i64, r + 1)),
            Just((r + 1, 1i64)),
        ]
    }

    fn ops4(is_64: bool, rd: u32, rn: u32, lsb: i64, width: i64) -> [Operand; 4] {
        [
            Operand::Reg(gpr(is_64, rd)),
            Operand::Reg(gpr(is_64, rn)),
            Operand::Imm(lsb),
            Operand::Imm(width),
        ]
    }

    fn sut_word(ops: &[Operand]) -> Result<u32, String> {
        match encode_sbfx(ops)? {
            EncodeResult::Word(w) => Ok(w),
            other => Err(format!("expected Word, got {:?}", other)),
        }
    }

    fn sbfm_word(ops: &[Operand]) -> Result<u32, String> {
        match encode_sbfm(ops)? {
            EncodeResult::Word(w) => Ok(w),
            other => Err(format!("expected Word, got {:?}", other)),
        }
    }

    fn must_err(ops: &[Operand]) -> bool {
        match catch_unwind(AssertUnwindSafe(|| encode_sbfx(ops))) {
            Ok(Err(_)) => true,
            _ => false,
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
            Just(Operand::Imm(-1)),
            Just(Operand::Shift {
                kind: "lsl".into(),
                amount: 0,
            }),
            Just(Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "8h".into(),
            }),
        ]
    }

    /// Known-answer gate for the llvm-mc differential connection.
    #[test]
    fn encode_sbfx_kat_llvm_mc_w0_w1_lsb0_width1() {
        let want = 0x13000020u32;
        let mc = llvm_mc_word("sbfx w0, w1, #0, #1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops4(false, 0, 1, 0, 1)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_sbfx_kat_llvm_mc_w0_w1_lsb1_width1() {
        let want = 0x13010420u32;
        let mc = llvm_mc_word("sbfx w0, w1, #1, #1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops4(false, 0, 1, 1, 1)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_sbfx_kat_llvm_mc_x0_x1_lsb1_width8() {
        let want = 0x93412020u32;
        let mc = llvm_mc_word("sbfx x0, x1, #1, #8").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops4(true, 0, 1, 1, 8)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_sbfx_kat_llvm_mc_wzr_wzr_lsb31_width1() {
        let want = 0x131f7fffu32;
        let mc = llvm_mc_word("sbfx wzr, wzr, #31, #1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops4(false, 31, 31, 31, 1)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_sbfx_kat_llvm_mc_x0_xzr_lsb63_width1() {
        let want = 0x937fffe0u32;
        let mc = llvm_mc_word("sbfx x0, xzr, #63, #1").expect("llvm-mc KAT");
        assert_eq!(mc, want, "llvm-mc KAT mapping broken");
        let sut = sut_word(&ops4(true, 0, 31, 63, 1)).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    #[test]
    fn encode_sbfx_kat_llvm_mc_lr_x1_lsb8_width16() {
        let want = 0x93485c3eu32;
        let mc = llvm_mc_word("sbfx lr, x1, #8, #16").expect("llvm-mc LR KAT");
        assert_eq!(mc, want, "llvm-mc LR KAT mapping broken");
        let ops = [
            Operand::Reg("lr".into()),
            Operand::Reg("x1".into()),
            Operand::Imm(8),
            Operand::Imm(16),
        ];
        let sut = sut_word(&ops).expect("SUT KAT");
        assert_eq!(sut, want);
    }

    proptest! {
        #![proptest_config(cfg())]

        #[test]
        fn encode_sbfx_diff_valid_gpr((is_64, rd, rn, lsb, width) in sbfx_valid()) {
            let dest = gpr(is_64, rd);
            let src = gpr(is_64, rn);
            let asm = format!("sbfx {}, {}, #{}, #{}", dest, src, lsb, width);
            let ops = ops4(is_64, rd, rn, lsb as i64, width as i64);
            let mc = llvm_mc_word(&asm).expect("llvm-mc");
            let sut = sut_word(&ops).expect("SUT");
            prop_assert_eq!(sut, mc, "SBFX mismatch for {}", asm);
        }

        #[test]
        fn encode_sbfx_alias_sbfm((is_64, rd, rn, lsb, width) in sbfx_valid()) {
            let imms = (lsb as i64) + (width as i64) - 1;
            let sbfx = sut_word(&ops4(is_64, rd, rn, lsb as i64, width as i64)).expect("SBFX");
            let sbfm = sbfm_word(&[
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
                Operand::Imm(lsb as i64),
                Operand::Imm(imms),
            ]).expect("SBFM");
            prop_assert_eq!(sbfx, sbfm, "SBFX must alias SBFM #lsb, #(lsb+width-1)");
        }

        #[test]
        fn encode_sbfx_arm_fields((is_64, rd, rn, lsb, width) in sbfx_valid()) {
            let immr = lsb;
            let imms = lsb + width - 1;
            let sf = if is_64 { 1u32 } else { 0 };
            let w = sut_word(&ops4(is_64, rd, rn, lsb as i64, width as i64)).expect("SUT");
            let want = (sf << 31)
                | (0b100110 << 23)
                | (sf << 22)
                | (immr << 16)
                | (imms << 10)
                | (rn << 5)
                | rd;
            prop_assert_eq!(w, want, "ARM ARM SBFM/SBFX field layout");
            prop_assert_eq!(w >> 31, sf, "sf");
            prop_assert_eq!((w >> 29) & 0b11, 0b00, "opc=00 SBFM");
            prop_assert_eq!((w >> 23) & 0x3f, 0b100110, "bits[28:23]");
            prop_assert_eq!((w >> 22) & 1, sf, "N=sf");
            prop_assert_eq!((w >> 16) & 0x3f, immr, "immr=lsb");
            prop_assert_eq!((w >> 10) & 0x3f, imms, "imms=lsb+width-1");
            prop_assert_eq!((w >> 5) & 0x1f, rn, "Rn");
            prop_assert_eq!(w & 0x1f, rd, "Rd");
        }

        #[test]
        fn encode_sbfx_metamorphic_rd_rn(
            is_64 in any::<bool>(),
            rd in 0u32..=30,
            rn in 0u32..=30,
            lsb_w in 0u32..=1,
        ) {
            let r = if is_64 { 64u32 } else { 32 };
            let lsb = if lsb_w == 0 { 0 } else { 1 };
            let width = 1u32;
            prop_assume!(lsb < r && width <= r - lsb);
            let base = sut_word(&ops4(is_64, rd, rn, lsb as i64, width as i64)).expect("base");
            let w_rd = sut_word(&ops4(is_64, rd + 1, rn, lsb as i64, width as i64)).expect("rd+1");
            let w_rn = sut_word(&ops4(is_64, rd, rn + 1, lsb as i64, width as i64)).expect("rn+1");
            prop_assert_eq!(w_rd & 0x1f, rd + 1, "Rd+1 updates Rd field");
            prop_assert_eq!(w_rd & !0x1fu32, base & !0x1fu32, "Rd+1 leaves other fields unchanged");
            prop_assert_eq!((w_rn >> 5) & 0x1f, rn + 1, "Rn+1 updates Rn field");
            prop_assert_eq!(w_rn & !(0x1fu32 << 5), base & !(0x1fu32 << 5), "Rn+1 leaves other fields unchanged");
        }

        #[test]
        fn encode_sbfx_neg_arity(
            len in 0usize..=3,
            is_64 in any::<bool>(),
            rd in 0u32..=31,
            rn in 0u32..=31,
        ) {
            let mut ops = vec![
                Operand::Reg(gpr(is_64, rd)),
                Operand::Reg(gpr(is_64, rn)),
                Operand::Imm(0),
                Operand::Imm(1),
            ];
            ops.truncate(len);
            prop_assert!(
                encode_sbfx(&ops).is_err(),
                "SBFX with {} operands must Err (llvm-mc: too few operands)",
                len
            );
        }

        #[test]
        fn encode_sbfx_neg_extra_operand(
            (is_64, rd, rn, lsb, width) in sbfx_valid(),
            extra in extra_operand(),
        ) {
            let mut ops = ops4(is_64, rd, rn, lsb as i64, width as i64).to_vec();
            ops.push(extra);
            prop_assert!(
                encode_sbfx(&ops).is_err(),
                "SBFX has no 5th operand; extra operand must Err (llvm-mc rejects it)"
            );
        }

        #[test]
        fn encode_sbfx_neg_sp(
            which in 0u32..=1,
            sp64 in any::<bool>(),
            is_64 in any::<bool>(),
            other in 0u32..=30,
        ) {
            let sp = if sp64 { "sp" } else { "wsp" };
            let mut ops = ops4(is_64, other, other, 0, 1);
            ops[which as usize] = Operand::Reg(sp.to_string());
            prop_assert!(
                encode_sbfx(&ops).is_err(),
                "SP/WSP is not a valid SBFX operand (which={} sp={})",
                which,
                sp
            );
        }

        #[test]
        fn encode_sbfx_neg_lsb_width(
            (is_64, rd, rn, lsb, width) in any::<bool>().prop_flat_map(|is_64| {
                (Just(is_64), 0u32..=31, 0u32..=31, invalid_lsb_width(is_64))
                    .prop_map(|(is_64, rd, rn, (lsb, width))| (is_64, rd, rn, lsb, width))
            }),
        ) {
            let r = if is_64 { 64i64 } else { 32 };
            prop_assume!(!(lsb >= 0 && width >= 1 && lsb < r && width <= r - lsb));
            let ops = ops4(is_64, rd, rn, lsb, width);
            prop_assert!(
                must_err(&ops),
                "SBFX lsb={} width={} R={} must Err (ARM: 0<=lsb<R, 1<=width<=R-lsb)",
                lsb,
                width,
                r
            );
        }

        #[test]
        fn encode_sbfx_diff_alt_spellings(
            (is_64, rd, rn, lsb, width) in sbfx_valid(),
            dest_spell in 0u32..=4,
            src_spell in 0u32..=4,
        ) {
            let dest = spell(is_64, rd, dest_spell);
            let src = spell(is_64, rn, src_spell);
            let asm = format!("sbfx {}, {}, #{}, #{}", dest, src, lsb, width);
            let ops = [
                Operand::Reg(dest),
                Operand::Reg(src),
                Operand::Imm(lsb as i64),
                Operand::Imm(width as i64),
            ];
            let mc = llvm_mc_word(&asm).expect("llvm-mc");
            let sut = sut_word(&ops).expect("SUT");
            prop_assert_eq!(sut, mc, "SBFX alt-spelling mismatch for {}", asm);
        }

        #[test]
        fn encode_sbfx_neg_mixed_width(
            rd in 0u32..=31,
            rn in 0u32..=31,
            rd64 in any::<bool>(),
            rn64 in any::<bool>(),
        ) {
            prop_assume!(rd64 != rn64);
            let ops = [
                Operand::Reg(gpr(rd64, rd)),
                Operand::Reg(gpr(rn64, rn)),
                Operand::Imm(0),
                Operand::Imm(1),
            ];
            prop_assert!(
                encode_sbfx(&ops).is_err(),
                "SBFX mixed W/X (rd64={} rn64={}) must Err (llvm-mc rejects it)",
                rd64,
                rn64
            );
        }

        #[test]
        fn encode_sbfx_neg_fp(
            which in 0u32..=1,
            prefix in prop::sample::select(vec!["d", "s", "q", "v", "h", "b"]),
            n in 0u32..=31,
        ) {
            let fp = format!("{}{}", prefix, n);
            let mut ops = ops4(true, 0, 1, 0, 1);
            ops[which as usize] = Operand::Reg(fp.clone());
            prop_assert!(
                encode_sbfx(&ops).is_err(),
                "FP/SIMD register {} is not a valid SBFX operand (which={})",
                fp,
                which
            );
        }

        #[test]
        fn encode_sbfx_neg_nonreg(
            which in 0u32..=3,
            bad in non_reg_operand(),
        ) {
            if which >= 2 {
                prop_assume!(!matches!(bad, Operand::Imm(_)));
            }
            let mut ops = ops4(false, 0, 1, 0, 1).to_vec();
            ops[which as usize] = bad;
            prop_assert!(
                encode_sbfx(&ops).is_err(),
                "wrong operand kind at slot {} must Err",
                which
            );
        }

        #[test]
        fn encode_sbfx_neg_invalid_name(
            which in 0u32..=1,
            name in invalid_name(),
        ) {
            let mut ops = ops4(false, 0, 1, 0, 1);
            ops[which as usize] = Operand::Reg(name.clone());
            prop_assert!(
                encode_sbfx(&ops).is_err(),
                "invalid register name {:?} at slot {} must Err",
                name,
                which
            );
        }
    }

    fn spell(is_64: bool, n: u32, kind: u32) -> String {
        match kind {
            0 if n == 31 => {
                if is_64 {
                    "x31".into()
                } else {
                    "w31".into()
                }
            }
            1 if n == 31 => {
                if is_64 {
                    "XZR".into()
                } else {
                    "WZR".into()
                }
            }
            2 if n == 30 && is_64 => "LR".into(),
            3 => gpr(is_64, n).to_uppercase(),
            _ => gpr(is_64, n),
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
            "x-1".into(),
            "x99".into(),
            "w".into(),
        ])
    }

    fn non_reg_operand() -> impl Strategy<Value = Operand> {
        prop_oneof![
            any::<i64>().prop_map(Operand::Imm),
            Just(Operand::Shift {
                kind: "lsl".into(),
                amount: 0,
            }),
            Just(Operand::Mem {
                base: "x0".into(),
                offset: 0,
            }),
            Just(Operand::Label("L0".into())),
            Just(Operand::Symbol("foo".into())),
            Just(Operand::Cond("eq".into())),
            Just(Operand::RegArrangement {
                reg: "v0".into(),
                arrangement: "8b".into(),
            }),
        ]
    }

    #[test]
    fn test_encode_sbfx_regression_extra_operand() {
        let mut ops = ops4(false, 0, 0, 0, 1).to_vec();
        ops.push(Operand::Reg("x0".into()));
        assert!(
            encode_sbfx(&ops).is_err(),
            "SBFX w0, w0, #0, #1, x0 must Err; extra operand is invalid (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_sbfx_regression_sp() {
        let ops = [
            Operand::Reg("wsp".into()),
            Operand::Reg("w0".into()),
            Operand::Imm(0),
            Operand::Imm(1),
        ];
        assert!(
            encode_sbfx(&ops).is_err(),
            "SBFX wsp, w0, #0, #1 must Err; register 31 is ZR not SP/WSP (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_sbfx_regression_width_zero() {
        let ops = ops4(false, 0, 0, 0, 0);
        let result = catch_unwind(AssertUnwindSafe(|| encode_sbfx(&ops)));
        assert!(
            matches!(result, Ok(Err(_))),
            "SBFX w0, w0, #0, #0 must Err; width=0 is outside 1..=32-lsb (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_sbfx_regression_mixed_width() {
        let ops = [
            Operand::Reg("x0".into()),
            Operand::Reg("w0".into()),
            Operand::Imm(0),
            Operand::Imm(1),
        ];
        assert!(
            encode_sbfx(&ops).is_err(),
            "SBFX x0, w0, #0, #1 must Err; mixed W/X is invalid (llvm-mc rejects it)"
        );
    }

    #[test]
    fn test_encode_sbfx_regression_fp() {
        let ops = [
            Operand::Reg("d0".into()),
            Operand::Reg("x1".into()),
            Operand::Imm(0),
            Operand::Imm(1),
        ];
        assert!(
            encode_sbfx(&ops).is_err(),
            "SBFX d0, x1, #0, #1 must Err; FP/SIMD registers are not SBFX operands (llvm-mc rejects it)"
        );
    }
}
