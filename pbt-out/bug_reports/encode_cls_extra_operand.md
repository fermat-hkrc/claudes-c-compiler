# Bug: encode_cls ignores extra operands
**Law:** CLS is a two-operand instruction; a third operand must return Err (llvm-mc: invalid operand).
**Impact:** Assembler silently drops trailing garbage and emits a valid CLS encoding, so malformed assembly is assembled instead of rejected.
**Function:** encode_cls
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** operands = [Reg("w0"), Reg("w0"), Reg("x0")]  (cls w0, w0, x0)
**Expected:** Err
**Actual:** Ok(Word(0x5ac01400)) — same encoding as `cls w0, w0`. encode_cls never checks operands.len(); get_reg only reads indices 0 and 1.
**Severity:** medium
**Fix:** Reject `operands.len() != 2` before reading Rd/Rn (same arity contract as llvm-mc).
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs::test_encode_cls_regression_extra_operand
**Serial reconfirmation:** reproduced with cargo test --lib encode_cls -- --test-threads=1
