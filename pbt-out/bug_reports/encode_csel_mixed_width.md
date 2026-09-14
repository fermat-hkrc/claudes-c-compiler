# Bug: encode_csel accepts mixed x/w register widths
**Law:** CSEL requires all three GPRs to be the same width (Wt/Wn/Wm or Xt/Xn/Xm); mixed x/w must be Err.
**Impact:** `csel x0, w1, x2, eq` encodes using sf from Rd only, producing a 64-bit CSEL with Rn/Rm numbers taken from W registers. llvm-mc rejects mixed widths. A mistyped operand silently yields the wrong instruction.
**Function:** encode_csel
**Detected by:** Negative/Error Contract
**Minimal input:** [Reg("x0"), Reg("w1"), Reg("x2"), Cond("eq")]  (`csel x0, w1, x2, eq`)
**Expected:** Err
**Actual:** Ok(Word) — sf is taken only from operand 0; Rn and Rm widths are never checked.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_csel_pbt::test_encode_csel_regression_mixed_width
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib test_encode_csel_regression_mixed_width -- --test-threads=1 reproduced the failure.
