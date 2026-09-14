# Bug: encode_csinv encodes SP as XZR
**Law:** CSINV register 31 is XZR/WZR, never SP/WSP; encode_csinv with SP or WSP in Rd, Rn, or Rm must be Err.
**Impact:** `csinv sp, x0, x1, eq` is encoded as `csinv xzr, x0, x1, eq`. Callers that accidentally pass SP get a silent ZR select-invert instead of an assembler error. llvm-mc rejects SP for CSINV.
**Function:** encode_csinv
**Detected by:** Negative/Error Contract
**Minimal input:** [Reg("sp"), Reg("x0"), Reg("x0"), Cond("eq")]  (`csinv sp, x0, x0, eq`)
**Expected:** Err
**Actual:** Ok(Word) — parse_reg_num maps "sp"/"wsp" to 31, the same encoding as XZR/WZR.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_csinv_pbt::test_encode_csinv_regression_sp
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib encode_csinv_neg -- --test-threads=1 reproduced encode_csinv_neg_wrong_reg with kind=0 n=0.
