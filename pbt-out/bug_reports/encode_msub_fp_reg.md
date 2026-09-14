# Bug: encode_msub accepts FP/SIMD register names as GPRs
**Law:** Integer MSUB is GPR-only. llvm-mc rejects `msub d0, x1, x2, x3` ("invalid operand"). encode_msub must Err when any operand is a d/s/q/v/h/b register.
**Impact:** `msub d0, x1, x2, x3` is assembled as `msub w0, x1, x2, x3` (parse_reg_num accepts d/s/q/v/h/b prefixes; is_64bit_reg is false so sf=0). The object file contains a GPR multiply-subtract, not an FP instruction and not a diagnostic.
**Function:** encode_msub
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** operands = [Reg("d0"), Reg("x1"), Reg("x2"), Reg("x3")]
**Expected:** Err (FP/SIMD names are not MSUB operands)
**Actual:** Ok(Word) — encodes as 32-bit MSUB with Rd=0. Reproduced serially (`PBT_TEST_JOBS=1`).
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::encode_msub_pbt::test_encode_msub_regression_fp_reg
