# Bug: encode_uxtw ignores extra operands
**Law:** UXTW has exactly two register operands; a third operand must be rejected (llvm-mc / GNU as reject `uxtw x0, w1, x0` and `uxtw x0, w1, #0`).
**Impact:** The assembler silently encodes `uxtw Xd, Wn, extra` as the two-operand form, producing a (wrong) instruction word for invalid assembly. Callers that pass a leftover shift/extend/register get code instead of an error.
**Function:** encode_uxtw
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `[Reg("x0"), Reg("w0"), Reg("x0")]` (uxtw x0, w0, x0)
**Expected:** `Err(...)`
**Actual:** `Ok(Word)` — extra operand ignored; `get_reg` only reads slots 0 and 1
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs `test_encode_uxtw_regression_extra_operand`
**Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_uxtw -- --test-threads=1`
