# Bug: encode_sxth ignores extra operands
**Law:** SXTH has exactly two register operands; a third operand must be rejected (llvm-mc / GNU as reject it).
**Impact:** The assembler silently encodes `sxth Rd, Rn, extra` as the two-operand form, producing a valid instruction word for invalid assembly. Callers that pass a leftover shift/extend/register get wrong code instead of an error.
**Function:** encode_sxth
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `[Reg("w0"), Reg("w0"), Reg("x0")]` (sxth w0, w0, x0)
**Expected:** `Err(...)`
**Actual:** `Ok(Word(0x13003c00))` — extra operand ignored; `get_reg` only reads slots 0 and 1
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs `test_encode_sxth_regression_extra_operand`
**Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_sxth_neg -- --test-threads=1`
