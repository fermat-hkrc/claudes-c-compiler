# Bug: encode_prfm ignores extra operands
**Law:** PRFM is a two-operand instruction (`PRFM <prfop>, [<Xn|SP>{, #<pimm>}]`). llvm-mc rejects a third operand. Extra operands must be Err, not ignored.
**Impact:** `prfm pldl1keep, [x0], x2` (or any third Operand) encodes as a valid PRFM of the first two operands. Assembler silently drops trailing garbage.
**Function:** encode_prfm
**Detected by:** Negative/Error Contract
**Minimal input:** prfop=#0, rn=x0, pimm=0, extra=Reg("x2")
**Expected:** Err
**Actual:** Ok(Word(0xF9800000)) — `operands.len() < 2` does not reject extras
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_prfm_pbt::test_encode_prfm_regression_extra_operand
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / --test-threads=1 (encode_prfm_neg_extra_operand)
