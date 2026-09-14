# Bug: encode_rev32 accepts 32-bit W registers
**Law:** Scalar REV32 is 64-bit only (ARM ARM REV32 <Xd>, <Xn>). W registers must be rejected.
**Impact:** `rev32 w0, w0` is encoded as `rev32 x0, x0` (sf hardcoded to 1). Silent wrong-width assembly; llvm-mc/gas reject the W form. The purpose comment at bitfield.rs:218 states "REV32 is 64-bit only" but the body never checks is_64.
**Function:** encode_rev32
**Detected by:** Negative/Error Contract
**Minimal input:** `[Reg("w0"), Reg("w0")]` (rd=0, rn=0)
**Expected:** Err (llvm-mc: invalid operand for instruction)
**Actual:** Ok(Word(0xdac00800)) — same encoding as `rev32 x0, x0`
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs::test_encode_rev32_regression_w32
**Serial reconfirm:** PBT_TEST_JOBS=1 RUST_TEST_THREADS=1 reproduced
