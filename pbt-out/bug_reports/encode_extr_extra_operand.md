# Bug: encode_extr ignores extra operands
**Law:** EXTR is a four-operand instruction; a fifth operand must return Err (llvm-mc: invalid operand).
**Impact:** Assembler silently drops trailing garbage and emits a valid EXTR encoding, so malformed assembly is assembled instead of rejected.
**Function:** encode_extr
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** operands = [Reg("w0"), Reg("w0"), Reg("w0"), Imm(0), Reg("x0")]  (extr w0, w0, w0, #0, x0)
**Expected:** Err
**Actual:** Ok(Word) — same encoding as `extr w0, w0, w0, #0`. encode_extr never checks operands.len(); get_reg/get_imm only read indices 0..3.
**Severity:** medium
**Fix:** Reject `operands.len() != 4` before reading Rd/Rn/Rm/lsb (same arity contract as llvm-mc).
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs::test_encode_extr_regression_extra_operand
**Serial reconfirmation:** reproduced with cargo test --lib encode_extr -- --test-threads=1
