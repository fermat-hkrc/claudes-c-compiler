# Bug: encode_logical accepts FP/SIMD register names as GPRs
**Law:** Logical GPR form requires X/W/XZR/WZR/SP/WSP/LR. llvm-mc rejects `and d0, x0, x0`.
**Impact:** `and d0, x0, x0` is encoded as `and x0, x0, x0` because parse_reg_num accepts d/s/q/v/h/b prefixes.
**Function:** encode_logical
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** operands `[Reg("d0"), Reg("x0"), Reg("x0")]`, opc=0 (`and d0, x0, x0`)
**Expected:** Err
**Actual:** Ok(Word) — parse_reg_num("d0") = 0
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::test_encode_logical_regression_fp_as_gpr
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1
