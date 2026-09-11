# Bug: encode_bic accepts FP/SIMD register names as GPRs
**Law:** Scalar BIC operands are general-purpose registers (X/W/XZR/WZR). FP/SIMD names (d/s/q/v/h/b) must be rejected.
**Impact:** `bic d0, x1, x2` is encoded as 32-bit `bic w0, x1, x2` (sf=0 because `is_64bit_reg("d0")` is false, `parse_reg_num("d0")` = 0). A SIMD register in a GPR slot is silently remapped.
**Function:** encode_bic
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `bic d0, x1, x2` (which=0, prefix=d, n=0)
**Expected:** Err (llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word) — `parse_reg_num` accepts prefixes `d|s|q|v|h|b` and returns 0..=31.
**Severity:** medium
**Regression test:** `test_encode_bic_regression_fp_reg` in src/backend/arm/assembler/encoder/data_processing.rs
**Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_bic_neg_fp_reg -- --test-threads=1`
