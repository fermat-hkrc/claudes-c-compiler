# Bug: encode_ubfm accepts FP/SIMD registers as GPR operands
**Law:** UBFM is a GPR instruction; d/s/q/v/h/b registers are not valid Rd or Rn.
**Impact:** `ubfm d0, x1, #0, #0` is encoded: parse_reg_num accepts the `d` prefix and returns 0, is_64bit_reg is false, so the word is a 32-bit UBFM of w0. llvm-mc reports `invalid operand for instruction`. FP/SIMD names are silently treated as W-form GPRs.
**Function:** encode_ubfm
**Detected by:** Negative/Error Contract
**Minimal input:** which=0, prefix="d", n=0 — `[Reg("d0"), Reg("x1"), Imm(0), Imm(0)]`
**Expected:** Err
**Actual:** Ok(Word) — parse_reg_num accepts d/s/q/v/h/b; encode_ubfm never calls is_fp_reg
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs::test_encode_ubfm_regression_fp
