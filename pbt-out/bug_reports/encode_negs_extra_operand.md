# Bug: encode_negs silently ignores extra non-shift operands
**Law:** NEGS takes exactly two GPR operands plus an optional shift; a trailing non-shift operand must be rejected.
**Impact:** Assembler accepts invalid GNU-style text that gas/llvm-mc reject (`negs w0, w0, x0` and `negs w0, w0, lsl #1, x0`), encoding as if the extra operand were absent. Downstream objects silently drop an operand.
**Function:** encode_negs
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `[Reg("w0"), Reg("w0"), Reg("x0")]` (also `[Reg("w0"), Reg("w0"), Shift{lsl,1}, Reg("x0")]`)
**Expected:** Err
**Actual:** Ok(Word) encoding `negs w0, w0` / `negs w0, w0, lsl #1`
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::test_encode_negs_regression_extra_operand (and test_encode_negs_regression_trailing_after_shift)
**Serial reconfirm:** PBT_TEST_JOBS=1, `--test-threads=1` — reproduced
