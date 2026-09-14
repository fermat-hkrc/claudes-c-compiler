# Bug: encode_ldxp_stxp ignores extra operands
**Law:** LDXP/LDAXP take exactly three operands (`Rt, Rt2, [Xn|SP]`); STXP/STLXP take exactly four (`Ws, Rt, Rt2, [Xn|SP]`). A surplus operand must be rejected.
**Impact:** Assembler accepts `stxp w0, x1, x2, [x3], x2` (and any trailing operand) and emits a 32-bit word as if the extra operand were absent. GNU as / llvm-mc reject the same text. Silent mis-assembly of malformed input.
**Function:** encode_ldxp_stxp
**Detected by:** Negative/Error Contract
**Minimal input:** rt=0, rt2=0, rn=0, ws=0, is_load=false, acqrel=false, is_64=false, extra=Reg("x2") — `stxp w0, w0, w0, [x0], x2`
**Expected:** Err
**Actual:** Ok(Word) encoding of `stxp w0, w0, w0, [x0]`
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_ldxp_stxp_pbt::test_encode_ldxp_stxp_regression_extra_operand
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / RUST_TEST_THREADS=1
