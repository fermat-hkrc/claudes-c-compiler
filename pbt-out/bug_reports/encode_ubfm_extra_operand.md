# Bug: encode_ubfm ignores extra operands
**Law:** UBFM takes exactly four operands (Rd, Rn, #immr, #imms); a fifth operand must be rejected.
**Impact:** The assembler silently encodes `ubfm w0, w0, #0, #0, x0` (and any extra Imm/Shift/RegArrangement) as a valid UBFM of the first four operands, so invalid GNU-style assembly produces a machine-code word instead of an error. gas / llvm-mc reject the extra operand (`invalid operand for instruction`).
**Function:** encode_ubfm
**Detected by:** Negative/Error Contract
**Minimal input:** `[Reg("w0"), Reg("w0"), Imm(0), Imm(0), Reg("x0")]` (shrunk from extra_operand; serial reconfirm with PBT_TEST_JOBS=1)
**Expected:** Err
**Actual:** Ok(Word) — get_reg/get_imm only read indices 0..3, so operands beyond index 3 are ignored
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs::test_encode_ubfm_regression_extra_operand
