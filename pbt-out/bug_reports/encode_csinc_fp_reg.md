# Bug: encode_csinc accepts FP/SIMD register names as GPRs
**Law:** CSINC takes Wt/Xt only; FP/SIMD names (d/s/q/v/h/b) must be Err. (FCSEL is a different instruction.)
**Impact:** `csinc d0, d1, d2, eq` encodes as a 32-bit CSINC of W0/W1/W2 (parse_reg_num accepts the d/s/q/v/h/b prefixes and is_64bit_reg is false for them). llvm-mc rejects FP/SIMD operands for CSINC.
**Function:** encode_csinc
**Detected by:** Negative/Error Contract
**Minimal input:** [Reg("d0"), Reg("d1"), Reg("d2"), Cond("eq")]  (`csinc d0, d1, d2, eq`)
**Expected:** Err
**Actual:** Ok(Word) — parse_reg_num maps dN to N and sf is 0, so the word is a W-form CSINC.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_csinc_pbt::test_encode_csinc_regression_fp_reg
**Serial reconfirmation:** kind=5 of encode_csinc_neg_wrong_reg (FP d-regs) is in the same failing property; the deterministic regression test isolates this class.
