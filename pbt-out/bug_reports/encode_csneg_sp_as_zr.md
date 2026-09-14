# Bug: encode_csneg encodes SP as XZR
**Law:** CSNEG register 31 is XZR/WZR, never SP/WSP; encode_csneg with SP or WSP in Rd, Rn, or Rm must be Err.
**Impact:** `csneg sp, x0, x1, eq` is encoded as `csneg xzr, x0, x1, eq` (0xda81041f). Callers that accidentally pass SP get a silent ZR select-negate instead of an assembler error. llvm-mc rejects SP for CSNEG.
**Function:** encode_csneg
**Detected by:** Negative/Error Contract
**Minimal input:** [Reg("sp"), Reg("x0"), Reg("x0"), Cond("eq")]  (`csneg sp, x0, x0, eq`)
**Expected:** Err
**Actual:** Ok(Word) — parse_reg_num maps "sp"/"wsp" to 31, the same encoding as XZR/WZR.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_csneg_pbt::test_encode_csneg_regression_sp
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib encode_csneg_neg -- --test-threads=1 reproduced encode_csneg_neg_wrong_reg with kind=0 n=0.
