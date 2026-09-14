# Bug: encode_div silently ignores a fourth operand
**Law:** UDIV/SDIV take exactly three register operands; a fourth operand must be rejected (gas/llvm-mc "invalid operand for instruction").
**Impact:** Assembler accepts `sdiv w0, w0, w0, x0` (and any extra Shift/Imm) and emits the 3-operand encoding, so a typo extra operand is assembled instead of diagnosed. Downstream object code silently drops the extra operand.
**Function:** encode_div
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `encode_div([Reg("w0"), Reg("w0"), Reg("w0"), Reg("x0")], unsigned=false)` i.e. `sdiv w0, w0, w0, x0`
**Expected:** Err
**Actual:** Ok(Word(0x1ac00c00)) — same as `sdiv w0, w0, w0`; extra operand ignored. Reproduced serially (`PBT_TEST_JOBS=1`).
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::encode_div_pbt::test_encode_div_regression_extra_operand
