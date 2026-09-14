# Bug: encode_msub silently ignores a fifth operand
**Law:** MSUB takes exactly four register operands; a fifth operand must be rejected (gas/llvm-mc "invalid operand for instruction").
**Impact:** Assembler accepts `msub w0, w0, w0, w0, x0` (and any extra Shift/Imm) and emits the 4-operand encoding, so a typo extra operand is assembled instead of diagnosed. Downstream object code silently drops the extra operand.
**Function:** encode_msub
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `encode_msub([Reg("w0"), Reg("w0"), Reg("w0"), Reg("w0"), Reg("x0")])` i.e. `msub w0, w0, w0, w0, x0`
**Expected:** Err
**Actual:** Ok(Word) — same as `msub w0, w0, w0, w0`; extra operand ignored. Reproduced serially (`PBT_TEST_JOBS=1`).
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::encode_msub_pbt::test_encode_msub_regression_extra_operand
