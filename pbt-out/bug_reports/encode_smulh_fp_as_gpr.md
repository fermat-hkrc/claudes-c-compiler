# Bug: encode_smulh accepts FP/SIMD registers as GPRs
**Law:** SMULH operands must be 64-bit GPRs (Xd/Xn/Xm); FP/SIMD registers (d/s/q/v/h/b) must be rejected.
**Impact:** `smulh d0, x1, x2` is encoded using the numeric suffix as a GPR number, producing a valid-looking SMULH word that gas / llvm-mc reject as "invalid operand".
**Function:** encode_smulh
**Detected by:** Negative/Error Contract
**Minimal input:** which=0, prefix="d", n=0 — `smulh d0, x1, x2` (serial reconfirm with PBT_TEST_JOBS=1)
**Expected:** Err
**Actual:** Ok(Word) — parse_reg_num accepts d/s/q/v/h/b prefixes and encode_smulh never calls is_fp_reg
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::test_encode_smulh_regression_fp
