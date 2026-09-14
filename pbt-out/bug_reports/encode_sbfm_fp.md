# Bug: encode_sbfm accepts FP/SIMD registers as GPR operands
**Law:** SBFM operands Rd and Rn are general-purpose registers (W/X). FP/SIMD registers (d/s/q/v/h/b) must be rejected.
**Impact:** `sbfm d0, x1, #0, #0` is encoded as 32-bit SBFM w0 (parse_reg_num accepts the 'd' prefix and is_64bit_reg is false for names that do not start with x). gas / llvm-mc reject FP/SIMD as "invalid operand".
**Function:** encode_sbfm
**Detected by:** Negative/Error Contract
**Minimal input:** which=0, prefix="d", n=0 — `sbfm d0, x1, #0, #0` (serial reconfirm with PBT_TEST_JOBS=1)
**Expected:** Err
**Actual:** Ok(Word) — parse_reg_num maps d0..b31 to 0..31 and encode_sbfm does not check is_fp_reg
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs::test_encode_sbfm_regression_fp
