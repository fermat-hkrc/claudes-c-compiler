# Bug: encode_csinc accepts mixed x/w register widths
**Law:** CSINC requires all three GPRs to be the same width (Wt/Wn/Wm or Xt/Xn/Xm); mixed x/w must be Err.
**Impact:** `csinc x0, w1, x2, eq` encodes using sf from Rd only, producing a 64-bit CSINC with Rn/Rm numbers taken from W registers. llvm-mc rejects mixed widths. A mistyped operand silently yields the wrong instruction.
**Function:** encode_csinc
**Detected by:** Negative/Error Contract
**Minimal input:** [Reg("x0"), Reg("w1"), Reg("x2"), Cond("eq")]  (`csinc x0, w1, x2, eq`)
**Expected:** Err
**Actual:** Ok(Word) — sf is taken only from operand 0; Rn and Rm widths are never checked.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_csinc_pbt::test_encode_csinc_regression_mixed_width
**Serial reconfirmation:** kind=4 of encode_csinc_neg_wrong_reg (mixed x/w) is in the same failing property; the deterministic regression test isolates this class.
