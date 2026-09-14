# Bug: encode_csetm accepts FP/SIMD register names as GPRs
**Law:** CSETM takes Wt/Xt only; FP/SIMD names (d/s/q/v/h/b) must be Err.
**Impact:** `csetm d0, eq` encodes as a 32-bit CSETM of W0 (0x5a9f13e0): parse_reg_num accepts the d/s/q/v/h/b prefixes and is_64bit_reg is false for them. llvm-mc rejects FP/SIMD operands for CSETM.
**Function:** encode_csetm
**Detected by:** Negative/Error Contract
**Minimal input:** [Reg("d0"), Cond("eq")]  (`csetm d0, eq`)
**Expected:** Err
**Actual:** Ok(Word(0x5a9f13e0)) — parse_reg_num maps dN to N and sf is 0, so the word is a W-form CSETM.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_csetm_pbt::test_encode_csetm_regression_fp_reg
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib encode_csetm_neg -- --test-threads=1 reproduced encode_csetm_neg_wrong_reg (SP shrink); FP witness confirmed by test_encode_csetm_regression_fp_reg.
