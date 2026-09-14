# Bug: encode_orn encodes FP/SIMD names as GPRs
**Law:** Shifted-register ORN operands are GPRs (X/W/XZR/WZR). llvm-mc rejects FP/SIMD names (`d0`, `s0`, `q0`, `v0`, `h0`, `b0`) in this form.
**Impact:** `orn d0, x1, x2` assembles as `orn x0, x1, x2`. A floating-point destination is silently treated as a GPR of the same number.
**Function:** encode_orn
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `orn d0, x1, x2`
**Expected:** Err (llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word) — parse_reg_num accepts d/s/q/v/h/b prefixes and returns the numeric suffix.
**Severity:** medium
**Regression test:** `test_encode_orn_regression_fp_reg` in src/backend/arm/assembler/encoder/data_processing.rs
**Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_orn_neg -- --test-threads=1`
