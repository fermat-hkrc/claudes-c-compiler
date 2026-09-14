# Bug: encode_cas accepts a W register or WSP as the memory base
**Law:** ARM ARM Rn is Xn|SP (64-bit integer or SP). llvm-mc/gas reject `cas w0, w1, [w2]` and `[wsp]`.
**Impact:** `cas w0, w1, [w2]` encodes with Rn=2 as if the base were X2. WSP encodes as SP. Silent wrong base width.
**Function:** encode_cas
**Detected by:** Negative/Error Contract
**Minimal input:** `cas w0, w1, [w2]`
**Expected:** Err
**Actual:** Ok(Word) encoding of `cas w0, w1, [x2]`
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_cas_pbt::test_encode_cas_regression_w_base
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / --test-threads=1 (encode_cas_neg_sp_zr_base covers this kind; shrunk witness was SP-as-Rs)
