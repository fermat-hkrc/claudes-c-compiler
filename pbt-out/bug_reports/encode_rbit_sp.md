# Bug: encode_rbit accepts SP/WSP as a GPR
**Law:** ARM RBIT register 31 is ZR not SP; `rbit wsp, w0` / `rbit sp, x0` must return Err (llvm-mc: invalid operand).
**Impact:** SP/WSP is encoded as register 31 (ZR), producing an RBIT of/into the zero register instead of a diagnostic. Callers that pass SP get a silently wrong instruction.
**Function:** encode_rbit
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** operands = [Reg("wsp"), Reg("w0")]  (rbit wsp, w0)
**Expected:** Err
**Actual:** Ok(Word(0x5ac0001f)) — parse_reg_num maps "sp"/"wsp" to 31; encode_rbit does not reject SP. Encoded as `rbit wzr, w0`.
**Severity:** medium
**Fix:** Reject SP/WSP in both Rd and Rn; register 31 is ZR (wzr/xzr) only.
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs::test_encode_rbit_regression_sp
**Serial reconfirmation:** reproduced with PBT_TEST_JOBS=1 cargo test --lib encode_rbit_pbt::encode_rbit_neg_ -- --test-threads=1
