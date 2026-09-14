# Bug: encode_ldrsw accepts SIMD/FP destination registers
**Law:** LDRSW destination is a GP Xt; SIMD/FP names (Dn/Sn/Qn/Hn/Bn) are invalid.
**Impact:** `ldrsw d0, [x1]` encodes as Rt=0 (same as x0). parse_reg_num accepts d/s/q/v/h/b prefixes.
**Function:** encode_ldrsw
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `[Reg("d0"), Mem { base: "x1", offset: 0 }]`
**Expected:** Err
**Actual:** Ok(Word) with Rt=0
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::test_encode_ldrsw_regression_fp_dest
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_ldrsw -- --test-threads=1`
