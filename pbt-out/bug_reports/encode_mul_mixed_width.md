# Bug: encode_mul accepts mixed X/W register widths
**Law:** MUL uses a single sf bit; Rd, Rn, and Rm must all be 64-bit (X/XZR) or all 32-bit (W/WZR). Mixed width must be rejected (gas/llvm-mc "invalid operand").
**Impact:** `mul w0, w0, x0` is encoded with sf taken only from Rd (W), so the 32-bit MUL encoding is emitted for a mixed-width instruction that is not valid AArch64. The assembler fails to diagnose a width mismatch that would be a hard error in gas/llvm-mc.
**Function:** encode_mul
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `encode_mul([Reg("w0"), Reg("w0"), Reg("x0")])` i.e. `mul w0, w0, x0`
**Expected:** Err
**Actual:** Ok(Word) with sf=0 (from Rd). Reproduced serially (`PBT_TEST_JOBS=1`).
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::encode_mul_pbt::test_encode_mul_regression_mixed_width
