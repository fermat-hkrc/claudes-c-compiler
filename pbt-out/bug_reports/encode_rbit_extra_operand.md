# Bug: encode_rbit ignores extra operands
**Law:** RBIT is a two-operand instruction; a third operand must return Err (llvm-mc: invalid operand).
**Impact:** Assembler silently drops trailing garbage and emits a valid RBIT encoding, so malformed assembly is assembled instead of rejected.
**Function:** encode_rbit
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** operands = [Reg("w0"), Reg("w0"), Reg("x0")]  (rbit w0, w0, x0)
**Expected:** Err
**Actual:** Ok(Word(0x5ac00000)) — same encoding as `rbit w0, w0`. encode_rbit never checks operands.len(); get_reg only reads indices 0 and 1.
**Severity:** medium
**Fix:** Reject `operands.len() != 2` before reading Rd/Rn (same arity contract as llvm-mc).
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs::test_encode_rbit_regression_extra_operand
**Serial reconfirmation:** reproduced with PBT_TEST_JOBS=1 cargo test --lib encode_rbit_pbt::encode_rbit_neg_ -- --test-threads=1
