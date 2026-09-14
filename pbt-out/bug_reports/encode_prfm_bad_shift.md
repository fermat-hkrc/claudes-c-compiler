# Bug: encode_prfm accepts illegal PRFM (register) shift amounts
**Law:** ARM ARM PRFM (register) S bit encodes amount 0 or 3 only. llvm-mc rejects `prfm pldl1keep, [x0, x1, lsl #1]` with "expected 'lsl' or 'sxtx' with optional shift of #0 or #3".
**Impact:** Any shift > 0 is encoded as S=1 (amount 3). `lsl #1` silently becomes `lsl #3`.
**Function:** encode_prfm
**Detected by:** Negative/Error Contract
**Minimal input:** prfm pldl1keep, [x0, x1, lsl #1] (amount=1)
**Expected:** Err
**Actual:** Ok(Word) with S=1 (same encoding as lsl #3)
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_prfm_pbt::test_encode_prfm_regression_bad_shift
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / --test-threads=1 (encode_prfm_neg_bad_shift)
