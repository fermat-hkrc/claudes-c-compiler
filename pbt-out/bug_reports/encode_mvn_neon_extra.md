# Bug: encode_mvn NEON form silently ignores a third operand
**Law:** NEON MVN takes exactly two arrangement operands (`mvn Vd.T, Vn.T`). A third operand must be rejected.
**Impact:** `mvn v0.16b, v0.16b, x0` is accepted and encoded as two-operand MVN. llvm-mc/gas reject the extra operand.
**Function:** encode_mvn
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `[RegArrangement{v0, 16b}, RegArrangement{v0, 16b}, Reg("x0")]`
**Expected:** Err
**Actual:** Ok(Word) encoding `mvn v0.16b, v0.16b`
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::test_encode_mvn_regression_neon_extra
**Serial reconfirm:** PBT_TEST_JOBS=1, `--test-threads=1` — reproduced
