# Bug: encode_fp_arith ignores extra operands
**Law:** Scalar FADD/FSUB/FMUL/FDIV/FMAX/FMIN/FMAXNM/FMINNM takes exactly three matching FP registers (Sd, Sn, Sm or Dd, Dn, Dm); a fourth operand must be rejected.
**Impact:** The assembler silently encodes `fadd s0, s0, s0, s0` (and extra Imm/Shift/RegArrangement) as the three-operand scalar form, so invalid GNU-style assembly produces a machine-code word. llvm-mc and gas reject the extra operand.
**Function:** encode_fp_arith
**Detected by:** Negative/Error Contract
**Minimal input:** `[Reg("s0"), Reg("s0"), Reg("s0"), Reg("s0")]` with opcode=0b0010 (shrunk; serial reconfirm with PBT_TEST_JOBS=1)
**Expected:** Err
**Actual:** Ok(Word) — get_reg only reads indices 0, 1, and 2, so operands beyond index 2 are ignored
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/fp_scalar.rs::test_encode_fp_arith_regression_extra_operand
