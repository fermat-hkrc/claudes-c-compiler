# Bug: encode_ret accepts FP/SIMD register names as GPRs
**Law:** RET Rn is a 64-bit GPR. FP/SIMD names (d/s/q/v/h/b) must be rejected (llvm-mc rejects `ret d0`; ARM ARM Rn is Xn).
**Impact:** `ret d0` encodes as `ret x0` (0xd65f0000). parse_reg_num accepts d/s/q/v/h/b prefixes and returns the numeric suffix, so a SIMD register is silently encoded as the corresponding X register.
**Function:** encode_ret
**Detected by:** Algebraic — Negative/Error Contract (4e)
**Minimal input:** operands = [Reg("d0")]
**Expected:** Err (llvm-mc: invalid operand for instruction)
**Actual:** Ok(Word(0xd65f0000)) — same encoding as `ret x0`
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_ret_pbt::test_encode_ret_regression_fp_reg
**Serial reconfirmation:** reproduced with PBT_TEST_JOBS=1 / --test-threads=1
**Root cause:** parse_reg_num accepts prefix in {d,s,q,v,h,b}; encode_ret does not call is_fp_reg / is_64bit_reg.
