# Bug: encode_madd silently ignores a fifth operand
**Law:** MADD takes exactly four register operands; a fifth operand must be rejected (gas/llvm-mc "invalid operand for instruction").
**Impact:** Assembler accepts `madd w0, w0, w0, w0, x0` (and any extra Shift/Imm) and emits the 4-operand encoding, so a typo extra operand is assembled instead of diagnosed. Downstream object code silently drops the extra operand.
**Function:** encode_madd
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `encode_madd([Reg("w0"), Reg("w0"), Reg("w0"), Reg("w0"), Reg("x0")])` i.e. `madd w0, w0, w0, w0, x0`
**Expected:** Err
**Actual:** Ok(Word) — same as `madd w0, w0, w0, w0`; extra operand ignored. Reproduced serially (`PBT_TEST_JOBS=1`).
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::encode_madd_pbt::test_encode_madd_regression_extra_operand
