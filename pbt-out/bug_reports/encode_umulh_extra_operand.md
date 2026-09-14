# Bug: encode_umulh ignores extra operands
**Law:** UMULH takes exactly three registers (Xd, Xn, Xm); a fourth operand must be rejected.
**Impact:** The assembler silently encodes `umulh x0, x0, x0, x0` (and any extra Imm/Shift/RegArrangement) as a valid UMULH of the first three registers, so invalid GNU-style assembly produces a machine-code word instead of an error. gas / llvm-mc reject the extra operand.
**Function:** encode_umulh
**Detected by:** Negative/Error Contract
**Minimal input:** `[Reg("x0"), Reg("x0"), Reg("x0"), Reg("x0")]` (shrunk from extra_operand; serial reconfirm with PBT_TEST_JOBS=1)
**Expected:** Err
**Actual:** Ok(Word) — get_reg only reads indices 0..2, so operands beyond index 2 are ignored
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::test_encode_umulh_regression_extra_operand
