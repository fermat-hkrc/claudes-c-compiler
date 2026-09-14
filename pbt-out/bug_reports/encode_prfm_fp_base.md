# Bug: encode_prfm accepts an FP/SIMD register as the memory base
**Law:** ARM ARM Rn is Xn|SP. llvm-mc rejects `prfm pldl1keep, [d0]` (and s/q/v/h/b).
**Impact:** `prfm #0, [d0]` encodes as `prfm #0, [x0]` because parse_reg_num accepts d/s/q/v/h/b prefixes. Silent wrong-base encoding.
**Function:** encode_prfm
**Detected by:** Negative/Error Contract
**Minimal input:** base="d0"
**Expected:** Err
**Actual:** Ok(Word) encoding of `prfm #0, [x0]`
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_prfm_pbt::test_encode_prfm_regression_fp_base
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / --test-threads=1 (encode_prfm_neg_invalid_base covers FP kinds; shrunk witness was W-base)
