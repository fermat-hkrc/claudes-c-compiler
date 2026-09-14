# Bug: encode_ldtr_sized accepts XZR as the memory base
**Law:** ARM ARM Rn is Xn|SP; register 31 as base is SP, never XZR. llvm-mc rejects `ldtrb w0, [xzr]`.
**Impact:** `ldtrb w0, [xzr]` encodes as `ldtrb w0, [sp]` because parse_reg_num maps xzr to 31. Silent wrong-base encoding (XZR vs SP).
**Function:** encode_ldtr_sized
**Detected by:** Negative/Error Contract
**Minimal input:** kind=4, n=0, simm in [-256,255] — `ldtrb w0, [xzr]`
**Expected:** Err
**Actual:** Ok(Word) encoding of `ldtrb w0, [sp]`
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_ldtr_sized_pbt::test_encode_ldtr_sized_regression_xzr_base
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / --test-threads=1 (same property encode_ldtr_sized_neg_invalid_regs)
