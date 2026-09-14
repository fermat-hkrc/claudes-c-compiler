# Bug: encode_rbit accepts FP/SIMD registers as scalar RBIT operands
**Law:** Scalar RBIT operands are GPRs only; `rbit d0, x1` must return Err (llvm-mc: invalid operand). NEON vector RBIT is a different form (RegArrangement, T in {8B,16B}).
**Impact:** parse_reg_num accepts d/s/q/v/h/b prefixes and get_reg does not call is_fp_reg, so `rbit d0, x1` is encoded as `rbit w0, w1` (sf=0 because d is not an X register). FP names are silently remapped to GPR encodings.
**Function:** encode_rbit
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** operands = [Reg("d0"), Reg("x1")]  (rbit d0, x1)
**Expected:** Err
**Actual:** Ok(Word(0x5ac00020)) — same encoding as `rbit w0, w1`.
**Severity:** medium
**Fix:** Reject FP/SIMD register names in both Rd and Rn on the scalar path (use is_fp_reg, or restrict parse to w/x/xzr/wzr/lr).
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs::test_encode_rbit_regression_fp
**Serial reconfirmation:** reproduced with PBT_TEST_JOBS=1 cargo test --lib encode_rbit_pbt::encode_rbit_neg_ -- --test-threads=1
