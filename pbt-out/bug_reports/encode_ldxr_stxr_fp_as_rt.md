# Bug: encode_ldxr_stxr encodes SIMD/FP names as GPR Rt
**Law:** Exclusive load/store Rt is a general-purpose Wt/Xt. SIMD/FP registers (B/H/S/D/Q/V) must be rejected.
**Impact:** `ldxr d0, [x1]` is encoded as a GPR exclusive load of register 0. GNU as / llvm-mc reject SIMD/FP as Rt. Silent wrong-register assembly.
**Function:** encode_ldxr_stxr
**Detected by:** Negative/Error Contract
**Minimal input:** kind=3 — `ldxr d0, [x1]`
**Expected:** Err
**Actual:** Ok(Word) with Rt=0 (GPR encoding of the numeric suffix)
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_ldxr_stxr_pbt::test_encode_ldxr_stxr_regression_fp_as_rt
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / RUST_TEST_THREADS=1 (kind=3 of encode_ldxr_stxr_neg_invalid_regs; dedicated regression)
