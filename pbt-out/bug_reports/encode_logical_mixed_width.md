# Bug: encode_logical accepts mixed X/W register widths
**Law:** All GPR operands of a logical instruction must be the same width (all X or all W). llvm-mc rejects `and w0, w0, x0`.
**Impact:** Mixed-width assembly is encoded using only Rd's width, producing a 32-bit or 64-bit instruction that does not match the source.
**Function:** encode_logical
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** operands `[Reg("w0"), Reg("w0"), Reg("x0")]`, opc=0 (`and w0, w0, x0`)
**Expected:** Err
**Actual:** Ok(Word) — sf is taken only from operand 0 via get_reg; Rn/Rm widths are ignored
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::test_encode_logical_regression_mixed_width
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1
