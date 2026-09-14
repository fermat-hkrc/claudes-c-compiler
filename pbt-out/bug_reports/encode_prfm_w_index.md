# Bug: encode_prfm accepts a W index without UXTW/SXTW
**Law:** ARM ARM PRFM (register) requires option UXTW or SXTW when the index is Wm. llvm-mc rejects `prfm pldl1keep, [x0, w0]` with "expected 'uxtw' or 'sxtw' with optional shift of #0 or #3".
**Impact:** Bare `[Xn, Wm]` is encoded as UXTW (option=010). That is a different instruction than the source text; GNU as/llvm-mc refuse it.
**Function:** encode_prfm
**Detected by:** Negative/Error Contract
**Minimal input:** prfm pldl1keep, [x0, w0] (idx=0, rn=0, rm=0, extend=None)
**Expected:** Err
**Actual:** Ok(Word) with option=UXTW, S=0
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_prfm_pbt::test_encode_prfm_regression_w_index
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / --test-threads=1 (encode_prfm_neg_w_index)
