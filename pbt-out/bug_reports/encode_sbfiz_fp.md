# Bug: encode_sbfiz accepts FP/SIMD registers as GPR
**Law:** SBFIZ operands are general-purpose registers (W/X/ZR). FP/SIMD prefixes d/s/q/v/h/b must be rejected with Err.
**Impact:** `sbfiz d0, x1, #0, #1` encodes as if d0 were w0. gas / llvm-mc reject FP/SIMD (`invalid operand for instruction`).
**Function:** encode_sbfiz
**Detected by:** Negative/Error Contract
**Minimal input:** which=0, prefix="d", n=0 — `sbfiz d0, x1, #0, #1` (serial reconfirm with `--test-threads=1` / PBT_TEST_JOBS=1)
**Expected:** Err
**Actual:** Ok(Word) — parse_reg_num accepts d/s/q/v/h/b prefixes as register numbers; is_fp_reg is never consulted.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs::test_encode_sbfiz_regression_fp
