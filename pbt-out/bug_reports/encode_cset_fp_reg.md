# Bug: encode_cset accepts FP/SIMD register names as GPRs
**Law:** CSET takes Wt/Xt only; FP/SIMD names (d/s/q/v/h/b) must be Err.
**Impact:** `cset d0, eq` encodes as a 32-bit CSET of W0 (0x1a9f17e0): parse_reg_num accepts the d/s/q/v/h/b prefixes and is_64bit_reg is false for them. llvm-mc rejects FP/SIMD operands for CSET.
**Function:** encode_cset
**Detected by:** Negative/Error Contract
**Minimal input:** [Reg("d0"), Cond("eq")]  (`cset d0, eq`)
**Expected:** Err
**Actual:** Ok(Word(0x1a9f17e0)) — parse_reg_num maps dN to N and sf is 0, so the word is a W-form CSET.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_cset_pbt::test_encode_cset_regression_fp_reg
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib encode_cset_pbt -- --test-threads=1 reproduced encode_cset_neg_wrong_reg (SP shrink); FP witness confirmed by test_encode_cset_regression_fp_reg.
