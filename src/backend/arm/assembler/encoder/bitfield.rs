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
