# Bug: encode_prfm accepts XZR/x31 as the memory base
**Law:** ARM ARM Rn is Xn|SP. Register 31 in the Rn field is SP, not XZR. llvm-mc rejects `prfm pldl1keep, [xzr]` and `[x31]`.
**Impact:** `prfm #0, [xzr]` and `[x31]` encode as `prfm #0, [sp]` because parse_reg_num maps xzr/x31 to 31.
**Function:** encode_prfm
**Detected by:** Negative/Error Contract
**Minimal input:** base="xzr" (also "x31")
**Expected:** Err
**Actual:** Ok(Word) encoding of `prfm #0, [sp]`
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_prfm_pbt::test_encode_prfm_regression_xzr_base (and test_encode_prfm_regression_x31_base)
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / --test-threads=1 (encode_prfm_neg_invalid_base covers these kinds; shrunk witness was W-base)
