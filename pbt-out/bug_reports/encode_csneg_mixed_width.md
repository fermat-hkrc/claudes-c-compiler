# Bug: encode_csneg accepts mixed x/w register widths
**Law:** CSNEG requires same-width GPRs (all Xt or all Wt). Mixed x/w operands must be Err.
**Impact:** `csneg x0, w1, x2, eq` is encoded as a 64-bit CSNEG using sf from Rd only (0xda820420, same as `csneg x0, x1, x2, eq`). The W-form Rn is silently treated as X1. llvm-mc rejects mixed x/w.
**Function:** encode_csneg
**Detected by:** Negative/Error Contract
**Minimal input:** [Reg("x0"), Reg("w1"), Reg("x2"), Cond("eq")]  (`csneg x0, w1, x2, eq`)
**Expected:** Err
**Actual:** Ok(Word(0xda820420)) — sf is taken only from operand 0; Rn/Rm widths are never checked.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_csneg_pbt::test_encode_csneg_regression_mixed_width
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib test_encode_csneg_regression_mixed_width -- --test-threads=1 reproduced the failure.
