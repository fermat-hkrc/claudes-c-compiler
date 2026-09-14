# Bug: encode_ldtr_sized accepts a W register as the memory base
**Law:** ARM ARM Rn is Xn|SP; a 32-bit W base must be rejected. llvm-mc rejects `ldtrb w0, [w1]`.
**Impact:** `ldtrb w0, [w0]` encodes as `ldtrb w0, [x0]` because parse_reg_num drops the width prefix. Silent wrong-base encoding.
**Function:** encode_ldtr_sized
**Detected by:** Negative/Error Contract
**Minimal input:** kind=3, n=0, simm in [-256,255] — `ldtrb w0, [w0]`
**Expected:** Err
**Actual:** Ok(Word) encoding of `ldtrb w0, [x0]`
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_ldtr_sized_pbt::test_encode_ldtr_sized_regression_w_base
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / --test-threads=1 (same property encode_ldtr_sized_neg_invalid_regs)
