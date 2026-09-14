# Bug: encode_clz accepts FP/SIMD registers as scalar CLZ operands
**Law:** Scalar CLZ operands are GPRs only; `clz d0, x1` must return Err (llvm-mc: invalid operand). NEON vector CLZ is a different dispatch path (`encode_neon_two_misc` on RegArrangement).
**Impact:** parse_reg_num accepts d/s/q/v/h/b prefixes and get_reg does not call is_fp_reg, so `clz d0, x1` is encoded as `clz w0, x1` (sf=0 because d is not an X register). FP names are silently remapped to GPR encodings.
**Function:** encode_clz
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** operands = [Reg("d0"), Reg("x1")]  (clz d0, x1)
**Expected:** Err
**Actual:** Ok(Word(0x5ac01020)) — same encoding as `clz w0, w1`.
**Severity:** medium
**Fix:** Reject FP/SIMD register names in both Rd and Rn (use is_fp_reg, or restrict parse to w/x/xzr/wzr/lr).
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs::test_encode_clz_regression_fp
**Serial reconfirmation:** reproduced with PBT_TEST_JOBS=1 cargo test --lib encode_clz_neg -- --test-threads=1
