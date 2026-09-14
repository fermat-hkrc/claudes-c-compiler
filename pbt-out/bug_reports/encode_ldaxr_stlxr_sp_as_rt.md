# Bug: encode_ldaxr_stlxr encodes SP as ZR in Rt
**Law:** Exclusive load/store Rt is Wt/Xt with register 31 meaning WZR/XZR, never SP/WSP. `ldaxr sp, [Xn]` must be rejected.
**Impact:** `ldaxr sp, [x0]` is encoded as `ldaxr xzr, [x0]`. GNU as / llvm-mc reject SP as Rt. Silent wrong-register assembly.
**Function:** encode_ldaxr_stlxr
**Detected by:** Negative/Error Contract
**Minimal input:** kind=0, n=0 — `ldaxr sp, [x0]`
**Expected:** Err
**Actual:** Ok(Word(0xC85FFC1F)) encoding Rt=31 (XZR) with 64-bit size
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_ldaxr_stlxr_pbt::test_encode_ldaxr_stlxr_regression_sp_as_rt
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / RUST_TEST_THREADS=1
