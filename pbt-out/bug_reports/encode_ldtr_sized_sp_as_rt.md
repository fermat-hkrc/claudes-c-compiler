# Bug: encode_ldtr_sized accepts SP/WSP as Rt
**Law:** ARM ARM LDTRB/H Rt is Wt (register 31 = WZR), never SP/WSP. Invalid Rt must be rejected.
**Impact:** `sttrb sp, [x0, #-256]` encodes as `sttrb wzr, [x0, #-256]` because parse_reg_num maps sp/wsp to 31. llvm-mc rejects `ldtrb sp, [x1]` and `ldtrb wsp, [x1]`. Silent wrong-register encoding.
**Function:** encode_ldtr_sized
**Detected by:** Negative/Error Contract
**Minimal input:** kind=0, n=0, simm=-256, is_load=false, size=0 — `sttrb sp, [x0, #-256]`
**Expected:** Err
**Actual:** Ok(Word) encoding of `sttrb wzr, [x0, #-256]`
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_ldtr_sized_pbt::test_encode_ldtr_sized_regression_sp_as_rt
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / --test-threads=1
