# Bug: encode_ldxp_stxp accepts a mixed X/W exclusive pair
**Law:** LDXP/STXP Rt and Rt2 must be the same width (both Wt or both Xt). llvm-mc rejects `ldxp x0, w1, [x2]`.
**Impact:** sz is taken only from operand 0 (load) or operand 1 (store); the other data register's width is ignored, so `ldxp x0, w1, [x2]` encodes as `ldxp x0, x1, [x2]`. Silent mis-assembly.
**Function:** encode_ldxp_stxp
**Detected by:** Negative/Error Contract
**Minimal input:** `ldxp x0, w1, [x2]` (kind=5 in encode_ldxp_stxp_neg_invalid_rt_base)
**Expected:** Err
**Actual:** Ok(Word) encoding of `ldxp x0, x1, [x2]`
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_ldxp_stxp_pbt::test_encode_ldxp_stxp_regression_mixed_width
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / RUST_TEST_THREADS=1
