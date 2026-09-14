# Bug: encode_ldxp_stxp encodes SIMD/FP registers as GPRs
**Law:** LDXP/STXP data registers are Wt/Xt only. llvm-mc rejects `ldxp d0, x1, [x2]` (and v/q/s/h/b).
**Impact:** `parse_reg_num` accepts d/s/q/v/h/b prefixes, so `ldxp d0, x1, [x2]` encodes as `ldxp x0, x1, [x2]` (or W, depending on is_64bit_reg). Silent mis-assembly.
**Function:** encode_ldxp_stxp
**Detected by:** Negative/Error Contract
**Minimal input:** `ldxp d0, x1, [x2]` (kind=4 in encode_ldxp_stxp_neg_invalid_rt_base)
**Expected:** Err
**Actual:** Ok(Word) encoding of the matching-number GPR form
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_ldxp_stxp_pbt::test_encode_ldxp_stxp_regression_fp_as_rt
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / RUST_TEST_THREADS=1
