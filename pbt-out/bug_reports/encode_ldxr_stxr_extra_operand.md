# Bug: encode_ldxr_stxr ignores extra operands
**Law:** LDXR/LDXRB/LDXRH take exactly two operands (`Rt, [Xn|SP]`); STXR/STXRB/STXRH take exactly three (`Ws, Rt, [Xn|SP]`). A surplus operand must be rejected.
**Impact:** Assembler accepts `stxr w0, x1, [x2], x2` (and any trailing operand) and emits a 32-bit word as if the extra operand were absent. GNU as / llvm-mc reject the same text. Silent mis-assembly of malformed input.
**Function:** encode_ldxr_stxr
**Detected by:** Negative/Error Contract
**Minimal input:** rt=0, rn=0, ws=0, is_load=false, variant=0, is_64=false, extra=Reg("x2") — `stxr w0, w0, [x0], x2`
**Expected:** Err
**Actual:** Ok(Word(0x88007c00)) encoding of `stxr w0, w0, [x0]`
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_ldxr_stxr_pbt::test_encode_ldxr_stxr_regression_extra_operand
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / RUST_TEST_THREADS=1
