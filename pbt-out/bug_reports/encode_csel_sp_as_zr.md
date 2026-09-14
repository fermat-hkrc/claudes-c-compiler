# Bug: encode_csel encodes SP as XZR
**Law:** CSEL register 31 is XZR/WZR, never SP/WSP; encode_csel with SP or WSP in Rd, Rn, or Rm must be Err.
**Impact:** `csel sp, x0, x1, eq` is encoded as `csel xzr, x0, x1, eq`. Callers that accidentally pass SP get a silent ZR select instead of an assembler error. llvm-mc rejects SP for CSEL.
**Function:** encode_csel
**Detected by:** Negative/Error Contract
**Minimal input:** [Reg("sp"), Reg("x0"), Reg("x1"), Cond("eq")]  (`csel sp, x0, x1, eq`)
**Expected:** Err
**Actual:** Ok(Word) — parse_reg_num maps "sp"/"wsp" to 31, the same encoding as XZR/WZR.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_csel_pbt::test_encode_csel_regression_sp
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib encode_csel_neg -- --test-threads=1 reproduced encode_csel_neg_wrong_reg with kind=0 n=0.
