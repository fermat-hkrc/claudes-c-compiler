# Bug: encode_ldaxr_stlxr encodes SIMD/FP names as GPR Rt
**Law:** LDAXR/STLXR data registers are general-purpose Wt/Xt. SIMD/FP names (d/s/q/v/h/b) must be rejected.
**Impact:** `ldaxr d0, [x1]` is encoded as `ldaxr x0, [x1]` because parse_reg_num accepts d/s/q/v/h/b prefixes. GNU as / llvm-mc reject SIMD/FP as Rt. Silent wrong-register assembly.
**Function:** encode_ldaxr_stlxr
**Detected by:** Negative/Error Contract
**Minimal input:** `ldaxr d0, [x1]` (regression of encode_ldaxr_stlxr_neg_invalid_regs kind=3)
**Expected:** Err
**Actual:** Ok(Word) encoding Rt=0 as if d0 were x0
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_ldaxr_stlxr_pbt::test_encode_ldaxr_stlxr_regression_fp_as_rt
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / RUST_TEST_THREADS=1
