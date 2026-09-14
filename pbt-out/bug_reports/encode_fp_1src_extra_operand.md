# Bug: encode_fp_1src ignores extra operands
**Law:** Scalar FRINTN/P/M/Z/A/X/I takes exactly two matching FP registers (Sd, Sn or Dd, Dn); a third operand must be rejected.
**Impact:** The assembler silently encodes `frintn s0, s1, s0` (and extra Imm/Shift/RegArrangement) as the two-operand scalar form, so invalid GNU-style assembly produces a machine-code word. llvm-mc and gas reject the extra operand.
**Function:** encode_fp_1src
**Detected by:** Negative/Error Contract
**Minimal input:** `[Reg("s0"), Reg("s0"), Reg("s0")]` with opcode=0b001000 (shrunk; serial reconfirm with PBT_TEST_JOBS=1)
**Expected:** Err
**Actual:** Ok(Word) — get_reg only reads indices 0 and 1, so operands beyond index 1 are ignored
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/fp_scalar.rs::test_encode_fp_1src_regression_extra_operand
