# Bug: encode_cls accepts FP/SIMD registers as GPRs
**Law:** Scalar CLS is integer GPR only; d/s/q/v/h/b registers must return Err (llvm-mc: invalid operand).
**Impact:** `cls d0, x1` encodes as 32-bit `cls w0, w1`, silently retargeting an FP/SIMD register to a GPR encoding.
**Function:** encode_cls
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** operands = [Reg("d0"), Reg("x1")]  (cls d0, x1)
**Expected:** Err
**Actual:** Ok(Word(0x5ac01420)) — same encoding as `cls w0, w1`. parse_reg_num accepts d/s/q/v/h/b prefixes; encode_cls does not call is_fp_reg.
**Severity:** medium
**Fix:** Reject FP/SIMD prefixes (d/s/q/v/h/b) via is_fp_reg before encoding.
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs::test_encode_cls_regression_fp
**Serial reconfirmation:** reproduced with cargo test --lib encode_cls -- --test-threads=1
