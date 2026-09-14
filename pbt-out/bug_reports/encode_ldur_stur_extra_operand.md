# Bug: encode_ldur_stur ignores extra operands
**Law:** LDUR/STUR/LDTR/STTR take exactly two operands (`Rt, [Xn|SP{, #imm}]`); a third operand must be rejected.
**Impact:** Assembler accepts `stur w0, [x0, #-256], x2` (and any trailing operand) and emits a 32-bit word as if the extra operand were absent. GNU as / llvm-mc reject the same text. Silent mis-assembly of malformed input.
**Function:** encode_ldur_stur
**Detected by:** Negative/Error Contract
**Minimal input:** rt=0, rn=0, offset=-256, is_load=false, is_64=false, unpriv=false, extra=Reg("x2") — `stur w0, [x0, #-256], x2`
**Expected:** Err
**Actual:** Ok(Word) encoding of `stur w0, [x0, #-256]`
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_ldur_stur_pbt::test_encode_ldur_stur_regression_extra_operand
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / RUST_TEST_THREADS=1
