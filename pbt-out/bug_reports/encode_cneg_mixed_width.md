# Bug: encode_cneg accepts mixed x/w register widths
**Law:** CNEG requires Rd and Rn to be the same width (both Xt or both Wt); mixed pairs must be Err.
**Impact:** `cneg x0, w0, eq` encodes as a 64-bit CNEG with Rn=0 (sf taken only from operand 0; Rn width is never checked). llvm-mc rejects mixed x/w. The emitted word is a valid-looking CNEG that does not match the source operands.
**Function:** encode_cneg
**Detected by:** Negative/Error Contract
**Minimal input:** [Reg("x0"), Reg("w0"), Cond("eq")]  (`cneg x0, w0, eq`)
**Expected:** Err
**Actual:** Ok(Word) with sf=1 (from x0) and Rn=0 (from w0's number).
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_cneg_pbt::test_encode_cneg_regression_mixed_width
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib encode_cneg_neg -- --test-threads=1 reproduced encode_cneg_neg_wrong_reg; mixed-width is kind=3 of the same property. Isolated regression test_encode_cneg_regression_mixed_width fails.
