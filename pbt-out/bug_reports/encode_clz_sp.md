# Bug: encode_clz accepts SP/WSP as a GPR
**Law:** ARM CLZ register 31 is ZR not SP; `clz wsp, w0` / `clz sp, x0` must return Err (llvm-mc: invalid operand).
**Impact:** SP/WSP is encoded as register 31 (ZR), producing a CLZ of/into the zero register instead of a diagnostic. Callers that pass SP get a silently wrong instruction.
**Function:** encode_clz
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** operands = [Reg("wsp"), Reg("w0")]  (clz wsp, w0)
**Expected:** Err
**Actual:** Ok(Word(0x5ac0101f)) — parse_reg_num maps "sp"/"wsp" to 31; encode_clz does not reject SP.
**Severity:** medium
**Fix:** Reject SP/WSP in both Rd and Rn; register 31 is ZR (wzr/xzr) only.
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs::test_encode_clz_regression_sp
**Serial reconfirmation:** reproduced with PBT_TEST_JOBS=1 cargo test --lib encode_clz_neg -- --test-threads=1
