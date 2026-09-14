# Bug: encode_cas ignores extra operands
**Law:** CAS takes exactly three operands (`Rs, Rt, [Xn|SP]`); a fourth operand must be rejected.
**Impact:** Assembler accepts `cas w0, w0, [x0], x2` (and any trailing operand) and emits a 32-bit word as if the extra operand were absent. GNU as / llvm-mc reject the same text. Silent mis-assembly of malformed input.
**Function:** encode_cas
**Detected by:** Negative/Error Contract
**Minimal input:** v=0 (cas), rs=0, rt=0, rn=0, is_64=false, extra=Reg("x2") — `cas w0, w0, [x0], x2`
**Expected:** Err
**Actual:** Ok(Word) encoding of `cas w0, w0, [x0]`
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_cas_pbt::test_encode_cas_regression_extra_operand
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / --test-threads=1
