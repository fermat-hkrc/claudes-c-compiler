# Bug: encode_mvn accepts FP/SIMD register names as GPRs
**Law:** Scalar MVN operands must be GPRs (Wn/Xn/WZR/XZR). FP/SIMD names (d/s/q/v/h/b) are not valid.
**Impact:** `mvn d0, x1` is accepted; parse_reg_num maps the numeric suffix so d0 encodes as w0/x0. llvm-mc/gas reject it.
**Function:** encode_mvn
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `[Reg("d0"), Reg("x1")]` (which=0, prefix="d", n=0)
**Expected:** Err
**Actual:** Ok(Word) treating d0 as register number 0
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::test_encode_mvn_regression_fp_reg
**Serial reconfirm:** PBT_TEST_JOBS=1, `--test-threads=1` — reproduced
