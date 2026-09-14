# Bug: encode_cmn accepts mixed x/w register widths
**Law:** CMN shifted-register form requires Rn and Rm to be the same width (both Xt or both Wt); mixed pairs without an extend must be Err.
**Impact:** `cmn x0, w0` encodes as `cmn x0, x0` (sf taken from the prepended XZR; Rm width is never checked). llvm-mc rejects mixed x/w without an extend specifier. The emitted word is a valid-looking CMN that does not match the source operands.
**Function:** encode_cmn
**Detected by:** Negative/Error Contract
**Minimal input:** [Reg("x0"), Reg("w0")]  (`cmn x0, w0`)
**Expected:** Err
**Actual:** Ok(Word) with sf=1 (from XZR) and Rm=0 (from w0's number).
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_cmn_pbt::test_encode_cmn_regression_mixed_width
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib encode_cmn_neg -- --test-threads=1 reproduced encode_cmn_neg_wrong_reg; mixed-width is kind=2 of the same property.
