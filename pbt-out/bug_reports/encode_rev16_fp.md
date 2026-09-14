# Bug: encode_rev16 accepts FP/SIMD register names as GPRs
**Law:** Scalar REV16 operands are W/X GPRs. llvm-mc rejects `rev16 d0, x1`. Vector REV16 uses a separate encoding (Advanced SIMD two-register miscellaneous) dispatched to encode_neon_two_misc.
**Impact:** `rev16 d0, x1` is encoded as 32-bit `rev16 w0, w1` (parse_reg_num strips the `d` prefix), silently assembling the wrong instruction class.
**Function:** encode_rev16
**Detected by:** Negative/Error Contract
**Minimal input:** `[Reg("d0"), Reg("x1")]` (serial reconfirm: PBT_TEST_JOBS=1)
**Expected:** `Err(_)`
**Actual:** `Ok(Word(...))` — `parse_reg_num` accepts d/s/q/v/h/b prefixes; `is_64bit_reg("d0")` is false so sf=0
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs `test_encode_rev16_regression_fp`
