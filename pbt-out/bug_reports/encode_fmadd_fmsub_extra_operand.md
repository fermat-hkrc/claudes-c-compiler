# Bug: encode_fmadd_fmsub ignores extra operands
**Law:** Scalar FMADD/FMSUB takes exactly four matching FP registers (Sd, Sn, Sm, Sa or Dd, Dn, Dm, Da); a fifth operand must be rejected.
**Impact:** The assembler silently encodes `fmadd s0, s0, s0, s0, s0` (and extra Imm/Shift/RegArrangement) as the four-operand scalar form, so invalid GNU-style assembly produces a machine-code word. llvm-mc and gas reject the extra operand.
**Function:** encode_fmadd_fmsub
**Detected by:** Negative/Error Contract
**Minimal input:** `[Reg("s0"), Reg("s0"), Reg("s0"), Reg("s0"), Reg("s0")]` with is_sub=false (shrunk; serial reconfirm with PBT_TEST_JOBS=1)
**Expected:** Err
**Actual:** Ok(Word) — get_reg only reads indices 0..3, so operands beyond index 3 are ignored
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/fp_scalar.rs::test_encode_fmadd_fmsub_regression_extra_operand
