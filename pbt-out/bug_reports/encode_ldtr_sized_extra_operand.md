# Bug: encode_ldtr_sized ignores extra operands
**Law:** LDTRB/LDTRH/STTRB/STTRH take exactly two operands (`Wt, [Xn|SP{, #imm}]`); a third operand must be rejected.
**Impact:** Assembler accepts `sttrb w0, [x0, #-256], x2` (and any trailing operand) and emits a 32-bit word as if the extra operand were absent. GNU as / llvm-mc reject the same text. Silent mis-assembly of malformed input.
**Function:** encode_ldtr_sized
**Detected by:** Negative/Error Contract
**Minimal input:** rt=0, rn=0, simm=-256, is_load=false, size=0, extra=Reg("x2") — `sttrb w0, [x0, #-256], x2`
**Expected:** Err
**Actual:** Ok(Word) encoding of `sttrb w0, [x0, #-256]`
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_ldtr_sized_pbt::test_encode_ldtr_sized_regression_extra_operand
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / --test-threads=1
