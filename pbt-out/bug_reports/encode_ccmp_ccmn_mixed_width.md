# Bug: encode_ccmp_ccmn accepts mixed x/w register widths
**Law:** CCMP/CCMN register form requires Rn and Rm to be the same width (both Xt or both Wt); mixed pairs must be Err.
**Impact:** `ccmn w0, x0, #0, eq` encodes as a 32-bit CCMN with Rm=0 (sf taken only from operand 0; Rm width is never checked). llvm-mc rejects mixed x/w. The emitted word is a valid-looking CCMN that does not match the source operands.
**Function:** encode_ccmp_ccmn
**Detected by:** Negative/Error Contract
**Minimal input:** [Reg("w0"), Reg("x0"), Imm(0), Cond("eq")], is_ccmp=false  (`ccmn w0, x0, #0, eq`)
**Expected:** Err
**Actual:** Ok(Word) with sf=0 (from w0) and Rm=0 (from x0's number).
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_ccmp_ccmn_pbt::test_encode_ccmp_ccmn_regression_mixed_width
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib encode_ccmp_ccmn_neg_mixed_width -- --test-threads=1 reproduced the failure.
