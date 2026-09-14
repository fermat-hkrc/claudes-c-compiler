# Bug: encode_rev32 accepts FP/SIMD registers as GPR operands
**Law:** Scalar REV32 operands are GPRs (Xd, Xn). FP/SIMD prefixes d/s/q/v/h/b must be rejected.
**Impact:** `rev32 d0, x1` encodes as `rev32 x0, x1` because parse_reg_num accepts d/s/q/v/h/b and encode_rev32 never checks the prefix. Silent wrong-register-class assembly; llvm-mc/gas reject it.
**Function:** encode_rev32
**Detected by:** Negative/Error Contract
**Minimal input:** `[Reg("d0"), Reg("x1")]` (which=0, prefix="d", n=0)
**Expected:** Err (llvm-mc: invalid operand)
**Actual:** Ok(Word(0xdac00820)) — same encoding as `rev32 x0, x1`
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs::test_encode_rev32_regression_fp
**Serial reconfirm:** PBT_TEST_JOBS=1 RUST_TEST_THREADS=1 reproduced
