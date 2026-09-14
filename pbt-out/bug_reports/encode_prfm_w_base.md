# Bug: encode_prfm accepts a W register as the memory base
**Law:** ARM ARM Rn is Xn|SP; a 32-bit W base must be rejected. llvm-mc rejects `prfm pldl1keep, [w0]` and `[wsp]`.
**Impact:** `prfm #0, [w0]` encodes as `prfm #0, [x0]` because parse_reg_num drops the width prefix. Silent wrong-base encoding. WSP similarly encodes as SP.
**Function:** encode_prfm
**Detected by:** Negative/Error Contract
**Minimal input:** kind=0, n=0 — `prfm #0, [w0]` (also `[wsp]`)
**Expected:** Err
**Actual:** Ok(Word) encoding of `prfm #0, [x0]` / `[sp]`
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_prfm_pbt::test_encode_prfm_regression_w_base (and test_encode_prfm_regression_wsp_base)
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / --test-threads=1 (encode_prfm_neg_invalid_base)
