# Bug: encode_ldaxr_stlxr accepts a W register as exclusive base
**Law:** Exclusive addressing is `[Xn|SP]`. A 32-bit W register as the base must be rejected.
**Impact:** `ldaxr x0, [w1]` is encoded as `ldaxr x0, [x1]`. GNU as / llvm-mc reject a W base. Silent wrong-register assembly.
**Function:** encode_ldaxr_stlxr
**Detected by:** Negative/Error Contract
**Minimal input:** `ldaxr x0, [w1]` (regression of encode_ldaxr_stlxr_neg_invalid_regs kind=1)
**Expected:** Err
**Actual:** Ok(Word) encoding Rn=1 as if the base were x1
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_ldaxr_stlxr_pbt::test_encode_ldaxr_stlxr_regression_w_base
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / RUST_TEST_THREADS=1
