# Bug: encode_csneg encodes FP/SIMD register names as GPRs
**Law:** CSNEG takes Wt/Xt only; FP/SIMD names (d/s/q/v/h/b) must be Err.
**Impact:** `csneg d0, d1, d2, eq` is encoded as `csneg w0, w1, w2, eq` (0x5a820420). parse_reg_num accepts d/s/q/v/h/b prefixes and is_64bit_reg is false for them, so they become 32-bit GPRs. llvm-mc rejects FP/SIMD for CSNEG.
**Function:** encode_csneg
**Detected by:** Negative/Error Contract
**Minimal input:** [Reg("d0"), Reg("d1"), Reg("d2"), Cond("eq")]  (`csneg d0, d1, d2, eq`)
**Expected:** Err
**Actual:** Ok(Word(0x5a820420)) — parse_reg_num maps d0→0; is_64bit_reg("d0") is false, so sf=0.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_csneg_pbt::test_encode_csneg_regression_fp_reg
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib test_encode_csneg_regression_fp_reg -- --test-threads=1 reproduced the failure.
