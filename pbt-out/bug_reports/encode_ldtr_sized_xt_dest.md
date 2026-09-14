# Bug: encode_ldtr_sized accepts Xt/lr as dest
**Law:** ARM ARM LDTRB/LDTRH/STTRB/STTRH dest/src is Wt only (register 31 = WZR). Xt, XZR, and lr must be rejected.
**Impact:** `sttrb x0, [x0, #-256]` encodes as `sttrb w0, [x0, #-256]` because get_reg discards is_64. llvm-mc rejects `ldtrb x0, [x1]` and `ldtrb lr, [x1]` as invalid operands. Silent wrong-width encoding.
**Function:** encode_ldtr_sized
**Detected by:** Negative/Error Contract
**Minimal input:** rt=0, rn=0, simm=-256, is_load=false, size=0, use_lr=false — `sttrb x0, [x0, #-256]`
**Expected:** Err
**Actual:** Ok(Word) encoding of `sttrb w0, [x0, #-256]`
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_ldtr_sized_pbt::test_encode_ldtr_sized_regression_xt_dest
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / --test-threads=1
