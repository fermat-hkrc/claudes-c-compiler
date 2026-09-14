# Bug: encode_logical ignores a trailing extra GPR operand
**Law:** A fourth operand that is not a shift must be rejected (llvm-mc: "expected 'lsl', 'lsr' or 'asr'").
**Impact:** Assembler silently drops extra operands, encoding a different instruction than the source text and hiding typos.
**Function:** encode_logical
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** operands `[Reg("w0"), Reg("w0"), Reg("w0"), Reg("w0")]`, opc=0 (`and w0, w0, w0, w0`)
**Expected:** Err
**Actual:** Ok(Word) — operands beyond index 3 are never inspected unless index 3 is a Shift
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::test_encode_logical_regression_extra_operand
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1
