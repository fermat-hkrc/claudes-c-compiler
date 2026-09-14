# Bug: encode_mul silently ignores a fourth operand
**Law:** MUL takes exactly three register operands; a fourth operand must be rejected (gas/llvm-mc "invalid operand for instruction").
**Impact:** Assembler accepts `mul w0, w0, w0, x0` (and any extra Shift/Imm) and emits the 3-operand encoding, so a typo extra operand is assembled instead of diagnosed. Downstream object code silently drops the extra operand.
**Function:** encode_mul
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `encode_mul([Reg("w0"), Reg("w0"), Reg("w0"), Reg("x0")])` i.e. `mul w0, w0, w0, x0`
**Expected:** Err
**Actual:** Ok(Word) — same as `mul w0, w0, w0`; extra operand ignored. Reproduced serially (`PBT_TEST_JOBS=1`).
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::encode_mul_pbt::test_encode_mul_regression_extra_operand
