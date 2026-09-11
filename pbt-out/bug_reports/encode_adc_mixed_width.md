# Bug: encode_adc accepts mixed 32/64-bit register operands
**Law:** ARM ADC requires all three registers the same width (`ADC <Wd>, <Wn>, <Wm>` or `ADC <Xd>, <Xn>, <Xm>`). Mixed x/w operands must Err. llvm-mc rejects `adc w0, w0, x0`.
**Impact:** `adc w0, w0, x0` is encoded using only Rd's sf bit (32-bit ADC), silently assembling a different instruction than the mixed-width text.
**Function:** encode_adc
**Detected by:** Algebraic — Negative/Error Contract (4e)
**Minimal input:** operands = [Reg("w0"), Reg("w0"), Reg("x0")], set_flags = false
**Expected:** Err (invalid operand; mixed register width)
**Actual:** Ok(Word) — sf taken only from Rd; encodes as 32-bit `adc w0, w0, w0`
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::encode_adc_pbt::test_encode_adc_regression_mixed_width
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 (cargo test --lib encode_adc_neg -- --test-threads=1)
