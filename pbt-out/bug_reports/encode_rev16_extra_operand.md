# Bug: encode_rev16 silently ignores a third operand
**Law:** REV16 takes exactly two register operands; an extra operand must be rejected (llvm-mc / GNU as reject `rev16 w0, w1, w2`).
**Impact:** The assembler accepts invalid instruction text and emits a 32-bit word as if the extra operand were absent, so malformed assembly is silently assembled.
**Function:** encode_rev16
**Detected by:** Negative/Error Contract
**Minimal input:** `[Reg("w0"), Reg("w0"), Reg("x0")]` (serial reconfirm: PBT_TEST_JOBS=1)
**Expected:** `Err(_)`
**Actual:** `Ok(Word(...))` — `get_reg` only reads indices 0 and 1; `operands.len()` is never checked
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs `test_encode_rev16_regression_extra_operand`
