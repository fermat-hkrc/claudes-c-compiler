# Bug: encode_fcvt_precision ignores a third operand
**Law:** Scalar FCVT takes exactly two register operands. llvm-mc rejects `fcvt d0, s1, s2` with "invalid operand for instruction". Extra operands must Err.
**Impact:** `fcvt s0, d0, s0` (and any extra Imm/Shift/arrangement) is assembled as `fcvt s0, d0`. A mistyped extra operand is silently dropped.
**Function:** encode_fcvt_precision
**Detected by:** Negative/error contract — llvm-mc "invalid operand"; README.md:11 gas-compatible assembler
**Minimal input:** `[Reg("s0"), Reg("d0"), Reg("s0")]` (shrunk `rd=0, rn=0, dest_ty=0, src_off=1, extra=Reg("s0")`; serial reconfirm with PBT_TEST_JOBS=1)
**Expected:** Err
**Actual:** Ok(Word(0x1e624000)) — FCVT of the first two operands
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/fp_scalar.rs::test_encode_fcvt_precision_regression_extra_operand
