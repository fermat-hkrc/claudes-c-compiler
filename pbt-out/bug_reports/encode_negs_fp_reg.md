# Bug: encode_negs accepts FP/SIMD register names as GPRs
**Law:** NEGS operands are integer GPRs (W/X including ZR). Floating-point and SIMD names (d/s/q/v/h/b) must be rejected.
**Impact:** `negs d0, x1` is accepted because parse_reg_num maps the `d` prefix to register number 0. llvm-mc/gas reject FP/SIMD names.
**Function:** encode_negs
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `[Reg("d0"), Reg("x1")]` (which=0, prefix="d", n=0)
**Expected:** Err
**Actual:** Ok(Word) encoding Rd as GPR 0
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::test_encode_negs_regression_fp_reg
**Serial reconfirm:** PBT_TEST_JOBS=1, `--test-threads=1` — reproduced
