# Bug: encode_madd accepts FP/SIMD register names as GPRs
**Law:** Integer MADD is GPR-only. llvm-mc rejects `madd d0, x1, x2, x3` ("invalid operand"). encode_madd must Err when any operand is a d/s/q/v/h/b register.
**Impact:** `madd d0, x1, x2, x3` is assembled as `madd w0, x1, x2, x3` (parse_reg_num accepts d/s/q/v/h/b prefixes; is_64bit_reg is false so sf=0). The object file contains a GPR multiply-add, not an FP instruction and not a diagnostic.
**Function:** encode_madd
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** operands = [Reg("d0"), Reg("x1"), Reg("x2"), Reg("x3")]
**Expected:** Err (FP/SIMD names are not MADD operands)
**Actual:** Ok(Word) — encodes as 32-bit MADD with Rd=0. Reproduced serially (`PBT_TEST_JOBS=1`).
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::encode_madd_pbt::test_encode_madd_regression_fp_reg
