# Bug: encode_ldaxr_stlxr accepts Xt on LDAXRB/LDAXRH
**Law:** Byte and halfword exclusive forms take Wt, not Xt. `ldaxrb x0, [x1]` must be rejected.
**Impact:** `ldaxrb x0, [x1]` is encoded as `ldaxrb w0, [x1]` because forced_size overrides the data-register width. GNU as / llvm-mc require Wt. Silent wrong-register assembly.
**Function:** encode_ldaxr_stlxr
**Detected by:** Negative/Error Contract
**Minimal input:** `ldaxrb x0, [x1]` (regression of encode_ldaxr_stlxr_neg_invalid_regs kind=5)
**Expected:** Err
**Actual:** Ok(Word) encoding size=00 with Rt=0
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_ldaxr_stlxr_pbt::test_encode_ldaxr_stlxr_regression_x_data_byte
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / RUST_TEST_THREADS=1
