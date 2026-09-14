# Bug: encode_ldxr_stxr encodes SP as ZR in Rt
**Law:** Exclusive load/store Rt is Wt/Xt with register 31 meaning WZR/XZR, never SP/WSP. `ldxr sp, [Xn]` must be rejected.
**Impact:** `ldxr sp, [x0]` is encoded as `ldxr xzr, [x0]`. GNU as / llvm-mc reject SP as Rt. Silent wrong-register assembly.
**Function:** encode_ldxr_stxr
**Detected by:** Negative/Error Contract
**Minimal input:** kind=0, n=0 — `ldxr sp, [x0]`
**Expected:** Err
**Actual:** Ok(Word) encoding Rt=31 (XZR) with 64-bit size
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_ldxr_stxr_pbt::test_encode_ldxr_stxr_regression_sp_as_rt
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / RUST_TEST_THREADS=1
