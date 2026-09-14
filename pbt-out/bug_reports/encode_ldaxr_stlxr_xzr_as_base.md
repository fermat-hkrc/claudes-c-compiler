# Bug: encode_ldaxr_stlxr encodes XZR as SP in the exclusive base
**Law:** Exclusive addressing register 31 is SP, never XZR/WZR. `ldaxr Xt, [xzr]` must be rejected.
**Impact:** `ldaxr x0, [xzr]` is encoded as `ldaxr x0, [sp]`. GNU as / llvm-mc reject XZR as base. Silent wrong-register assembly.
**Function:** encode_ldaxr_stlxr
**Detected by:** Negative/Error Contract
**Minimal input:** `ldaxr x0, [xzr]` (regression of encode_ldaxr_stlxr_neg_invalid_regs kind=2)
**Expected:** Err
**Actual:** Ok(Word) encoding Rn=31 (SP)
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_ldaxr_stlxr_pbt::test_encode_ldaxr_stlxr_regression_xzr_as_base
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / RUST_TEST_THREADS=1
