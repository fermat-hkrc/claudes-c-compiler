# Bug: encode_mvn ignores source NEON arrangement T
**Law:** NEON MVN requires matching T on Vd and Vn (`mvn Vd.T, Vn.T`).
**Impact:** `mvn v0.16b, v0.8b` is accepted; Q is taken from dest T only. llvm-mc/gas reject mismatched T.
**Function:** encode_mvn
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `[RegArrangement{v0, 16b}, RegArrangement{v0, 8b}]`
**Expected:** Err
**Actual:** Ok(Word) with Q=1 from dest T
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::test_encode_mvn_regression_neon_mismatch_t
**Serial reconfirm:** PBT_TEST_JOBS=1, `--test-threads=1` — reproduced
