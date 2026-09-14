# Bug: encode_ldar_stlr ignores extra operands
**Law:** LDAR/STLR take exactly two operands (`Rt, [Xn|SP]`); a third operand must be rejected.
**Impact:** Assembler accepts `stlr w0, [x0], x2` (and any trailing operand) and emits a 32-bit word as if the extra operand were absent. GNU as / llvm-mc reject the same text. Silent mis-assembly of malformed input.
**Function:** encode_ldar_stlr
**Detected by:** Negative/Error Contract
**Minimal input:** rt=0, rn=0, is_load=false, variant=0 (STLR 32-bit), extra=Reg("x2") — `stlr w0, [x0], x2`
**Expected:** Err
**Actual:** Ok(Word) encoding of `stlr w0, [x0]`
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_ldar_stlr_pbt::test_encode_ldar_stlr_regression_extra_operand
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / RUST_TEST_THREADS=1
