# Bug: encode_mul treats FP/SIMD register names as GPRs
**Law:** Integer MUL is GPR-only. FP/SIMD names (d/s/q/v/h/b prefixes) must be rejected (gas/llvm-mc "invalid operand").
**Impact:** `mul d0, x1, x2` is assembled as `mul x0, x1, x2` because parse_reg_num maps `d0` to register number 0. A floating-point operand is silently recoded as a GPR, producing wrong object code.
**Function:** encode_mul
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `encode_mul([Reg("d0"), Reg("x1"), Reg("x2")])` i.e. `mul d0, x1, x2`
**Expected:** Err
**Actual:** Ok(Word) encoding Rd=0 as if `x0`/`w0`. Reproduced serially (`PBT_TEST_JOBS=1`).
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::encode_mul_pbt::test_encode_mul_regression_fp_reg
