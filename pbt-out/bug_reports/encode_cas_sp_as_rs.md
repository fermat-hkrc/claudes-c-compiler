# Bug: encode_cas accepts SP/WSP as Rs or Rt
**Law:** ARM ARM Rs/Rt are integer ZR, not SP. llvm-mc/gas reject `cas sp, w1, [x2]` and `cas wsp, w1, [x2]`.
**Impact:** `cas sp, …` / `cas …, sp, …` encode as `cas xzr, …` because parse_reg_num maps sp/wsp to 31. Silent wrong register class.
**Function:** encode_cas
**Detected by:** Negative/Error Contract
**Minimal input:** v=0 (cas), n=0, kind=0, is_64=false — Rs="sp", Rt="w1", [x2]
**Expected:** Err
**Actual:** Ok(Word) encoding of `cas wzr, w1, [x2]` (SP aliased to ZR)
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_cas_pbt::test_encode_cas_regression_sp_as_rs
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / --test-threads=1
