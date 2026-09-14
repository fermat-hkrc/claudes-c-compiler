# Bug: encode_rev32 ignores extra operands
**Law:** REV32 takes exactly two operands; a third operand must be rejected.
**Impact:** Invalid assembly such as `rev32 x0, x1, x2` is silently encoded as `rev32 x0, x1`, producing a well-formed instruction from malformed input. Diverges from gas/llvm-mc.
**Function:** encode_rev32
**Detected by:** Negative/Error Contract
**Minimal input:** `[Reg("x0"), Reg("x0"), Reg("x0")]`
**Expected:** Err (llvm-mc: invalid operand)
**Actual:** Ok(Word(0xdac00800)) — extra operand ignored
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs::test_encode_rev32_regression_extra_operand
**Serial reconfirm:** PBT_TEST_JOBS=1 RUST_TEST_THREADS=1 reproduced
