# Bug: encode_fcmp ignores a third operand
**Law:** Scalar FCMP has no third operand (FCCMP is a different mnemonic with NZCV/cond). llvm-mc rejects `fcmp s0, s0, s0` with "invalid operand for instruction". Extra operands must Err.
**Impact:** `fcmp s0, s0, s0` (and any extra Imm/Shift/arrangement) is assembled as `fcmp s0, s0`. A mistyped FCCMP or stray operand is silently dropped.
**Function:** encode_fcmp
**Detected by:** Negative/error contract — llvm-mc "invalid operand"; encoder/mod.rs:439 fcmp vs fccmp
**Minimal input:** `[Reg("s0"), Reg("s0"), Reg("s0")]` (shrunk `rn=0, rm=0, is_d=false, extra=Reg("s0")`; serial reconfirm with PBT_TEST_JOBS=1)
**Expected:** Err
**Actual:** Ok(Word(0x1e202000)) — register-form FCMP of the first two operands
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/fp_scalar.rs::test_encode_fcmp_regression_extra_operand
