# Bug: encode_fmov ignores extra operands
**Law:** Scalar FMOV (register/general) takes exactly two operands; a third operand must be rejected.
**Impact:** The assembler silently encodes `fmov s0, s1, s0` (and extra Imm/Shift/RegArrangement) as the two-operand form, so invalid GNU-style assembly produces a machine-code word. llvm-mc and gas reject the extra operand.
**Function:** encode_fmov
**Detected by:** Negative/Error Contract
**Minimal input:** `[Reg("s0"), Reg("s0"), Reg("s0")]` (shrunk; serial reconfirm with PBT_TEST_JOBS=1)
**Expected:** Err
**Actual:** Ok(Word) — encode_fmov only checks `operands.len() < 2`, so operands beyond index 1 are ignored
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/fp_scalar.rs::test_encode_fmov_regression_extra_operand
