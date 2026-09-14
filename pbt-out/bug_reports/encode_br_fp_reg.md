# Bug: encode_br accepts FP/SIMD register names as GPR Rn
**Law:** ARM BR operands are 64-bit X registers only. llvm-mc rejects `br d0`. encode_br must Err when Rn is a d/s/q/v/h/b register.
**Impact:** `br d0` is encoded as `br x0` (0xd61f0000) because parse_reg_num maps d0→0 and encode_br ignores the is_64 flag. The assembler silently produces an integer BR instead of rejecting illegal FP operands.
**Function:** encode_br
**Detected by:** Algebraic — Negative/Error Contract (4e); coverage of parse_reg_num FP prefixes
**Minimal input:** operands = [Reg("d0")]
**Expected:** Err (invalid operand; BR is not an FP instruction)
**Actual:** Ok(Word(0xd61f0000)) — treated as x0 (rn=0)
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_br_pbt::test_encode_br_regression_fp_reg
**Serial reconfirmation:** reproduced with PBT_TEST_JOBS=1 after adding the regression witness (same get_reg/parse_reg_num path as encode_br_neg_wrong_reg_class which=2).
