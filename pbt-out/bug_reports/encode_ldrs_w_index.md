# Bug: encode_ldrs accepts a W index without uxtw/sxtw
**Law:** A 32-bit index register requires an explicit uxtw or sxtw extend. `ldrsb Xt, [Xn, Wm]` is invalid.
**Impact:** `ldrsb x0, [x1, w2]` encodes as uxtw (option=010) instead of being rejected.
**Function:** encode_ldrs
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** size=0, is_64=false, rt=0, rn=0, rm=0 → `[Reg("w0"), MemRegOffset { base: "x0", index: "w0", extend: None, shift: None }]`
**Expected:** Err
**Actual:** Ok(Word(0x38e04800)) — None extend with W index defaults to option 010
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::test_encode_ldrs_regression_w_index_no_extend
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_ldrs_neg_w_index -- --test-threads=1`
