# Bug: encode_csinv accepts FP/SIMD register names as GPRs
**Law:** CSINV takes Wt/Xt only; FP/SIMD names (d/s/q/v/h/b) must be Err. (FCSEL is a different instruction.)
**Impact:** `csinv d0, d1, d2, eq` encodes as a 32-bit CSINV of W0/W1/W2 (parse_reg_num accepts the d/s/q/v/h/b prefixes and is_64bit_reg is false for them). llvm-mc rejects FP/SIMD operands for CSINV.
**Function:** encode_csinv
**Detected by:** Negative/Error Contract
**Minimal input:** [Reg("d0"), Reg("d1"), Reg("d2"), Cond("eq")]  (`csinv d0, d1, d2, eq`)
**Expected:** Err
**Actual:** Ok(Word) — parse_reg_num maps dN to N and sf is 0, so the word is a W-form CSINV.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_csinv_pbt::test_encode_csinv_regression_fp_reg
**Serial reconfirmation:** kind=5 of encode_csinv_neg_wrong_reg (FP d-regs) is in the same failing property; the deterministic regression test isolates this class.
