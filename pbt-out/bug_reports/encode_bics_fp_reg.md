# Bug: encode_bics accepts FP/SIMD register names as GPRs
**Law:** BICS operands must be GPRs (x/w/xzr/wzr/lr). FP/SIMD names (d/s/q/v/h/b) must be rejected.
**Impact:** `bics d0, x1, x2` encodes as a 32-bit BICS of w0. A mistyped SIMD register in asm is assembled as a GPR instruction instead of an error.
**Function:** encode_bics
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `bics d0, x1, x2`
**Expected:** Err (llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word) — `parse_reg_num` accepts prefixes d/s/q/v/h/b; `is_64bit_reg` is false for `d0`, so sf=0.
**Severity:** medium
**Regression test:** `test_encode_bics_regression_fp_reg` in src/backend/arm/assembler/encoder/data_processing.rs
**Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib test_encode_bics_regression_fp_reg -- --test-threads=1`
