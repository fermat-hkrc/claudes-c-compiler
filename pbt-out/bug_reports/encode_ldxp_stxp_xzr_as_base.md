# Bug: encode_ldxp_stxp encodes XZR/WZR as SP when used as the base
**Law:** ARM ARM Rn is Xn|SP; XZR/WZR (register 31 as ZR) is not a valid exclusive-pair base. llvm-mc rejects `[xzr]`.
**Impact:** `parse_reg_num("xzr")` / `parse_reg_num("wzr")` return 31, which is the SP encoding in Rn, so `ldxp x0, x1, [xzr]` encodes as `ldxp x0, x1, [sp]`. Silent mis-assembly of an invalid addressing mode.
**Function:** encode_ldxp_stxp
**Detected by:** Negative/Error Contract
**Minimal input:** `ldxp x0, x1, [xzr]` (kind=3 in encode_ldxp_stxp_neg_invalid_rt_base)
**Expected:** Err
**Actual:** Ok(Word) encoding of `ldxp x0, x1, [sp]`
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_ldxp_stxp_pbt::test_encode_ldxp_stxp_regression_xzr_as_base
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / RUST_TEST_THREADS=1
