# Bug: encode_blr accepts FP/SIMD register names as GPR Rn
**Law:** ARM BLR operands are 64-bit X registers only. llvm-mc rejects `blr d0`. encode_blr must Err when Rn is a d/s/q/v/h/b register.
**Impact:** `blr d0` is encoded as `blr x0` (0xd63f0000) because parse_reg_num maps d0→0 and encode_blr ignores the is_64 flag. The assembler silently produces an integer BLR instead of rejecting illegal FP operands.
**Function:** encode_blr
**Detected by:** Algebraic — Negative/Error Contract (4e); coverage of parse_reg_num FP prefixes
**Minimal input:** operands = [Reg("d0")]
**Expected:** Err (invalid operand; BLR is not an FP instruction)
**Actual:** Ok(Word(0xd63f0000)) — treated as x0 (rn=0)
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_blr_pbt::test_encode_blr_regression_fp_reg
**Serial reconfirmation:** reproduced with PBT_TEST_JOBS=1 after adding the regression witness (same get_reg/parse_reg_num path as encode_blr_neg_wrong_reg_class which=2).
