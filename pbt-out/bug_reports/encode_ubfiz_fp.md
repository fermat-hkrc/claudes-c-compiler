# Bug: encode_ubfiz accepts FP/SIMD registers as GPR operands
**Law:** UBFIZ operands are general-purpose registers only. FP/SIMD prefixes (d/s/q/v/h/b) must be rejected with Err.
**Impact:** `ubfiz d0, x1, #0, #1` encodes as a GPR bitfield instruction because parse_reg_num accepts `d0` as register 0. gas / llvm-mc reject FP/SIMD (`invalid operand for instruction`).
**Function:** encode_ubfiz
**Detected by:** Negative/Error Contract
**Minimal input:** which=0, prefix="d", n=0 — `ubfiz d0, x1, #0, #1` (serial reconfirm with `--test-threads=1` / PBT_TEST_JOBS=1)
**Expected:** Err
**Actual:** Ok(Word) — parse_reg_num accepts d/s/q/v/h/b prefixes; encode_ubfiz does not check is_fp_reg.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs::test_encode_ubfiz_regression_fp
