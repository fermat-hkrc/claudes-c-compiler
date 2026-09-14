# Bug: encode_csel accepts FP/SIMD register names as GPRs
**Law:** CSEL takes Wt/Xt only; FP/SIMD names (d/s/q/v/h/b) must be Err. (FCSEL is a different instruction.)
**Impact:** `csel d0, d1, d2, eq` encodes as a 32-bit CSEL of W0/W1/W2 (parse_reg_num accepts the d/s/q/v/h/b prefixes and is_64bit_reg is false for them). llvm-mc rejects FP/SIMD operands for CSEL.
**Function:** encode_csel
**Detected by:** Negative/Error Contract
**Minimal input:** [Reg("d0"), Reg("d1"), Reg("d2"), Cond("eq")]  (`csel d0, d1, d2, eq`)
**Expected:** Err
**Actual:** Ok(Word) — parse_reg_num maps dN to N and sf is 0, so the word is a W-form CSEL.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_csel_pbt::test_encode_csel_regression_fp_reg
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib test_encode_csel_regression_fp_reg -- --test-threads=1 reproduced the failure.
