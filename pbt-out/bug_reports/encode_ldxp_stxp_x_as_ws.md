# Bug: encode_ldxp_stxp accepts an X register as STXP status
**Law:** STXP/STLXP status is Ws (32-bit W, 31=WZR), never Xt/SP. llvm-mc rejects `stxp x0, x1, x2, [x3]`.
**Impact:** Ws is only parsed for its number; width is ignored, so `stxp x0, x1, x2, [x3]` encodes as `stxp w0, x1, x2, [x3]`. Silent mis-assembly.
**Function:** encode_ldxp_stxp
**Detected by:** Negative/Error Contract
**Minimal input:** `stxp x0, x1, x2, [x3]` (kind=6 in encode_ldxp_stxp_neg_invalid_rt_base)
**Expected:** Err
**Actual:** Ok(Word) encoding of `stxp w0, x1, x2, [x3]`
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_ldxp_stxp_pbt::test_encode_ldxp_stxp_regression_x_as_ws
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / RUST_TEST_THREADS=1
