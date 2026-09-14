# Bug: encode_ldrsw accepts a W register destination
**Law:** LDRSW destination is Xt (64-bit); Wt is invalid and must be rejected.
**Impact:** `ldrsw w0, [x1]` is encoded as if it were `ldrsw x0, [x1]` (word 0xb9800000). Callers get a 64-bit sign-extending load targeting the wrong register width with no error.
**Function:** encode_ldrsw
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `[Reg("w0"), Mem { base: "x1", offset: 0 }]`
**Expected:** Err
**Actual:** Ok(Word(0xb9800000)) — get_reg discards is_64; Rt is encoded from parse_reg_num("w0")=0
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::test_encode_ldrsw_regression_w_dest
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_ldrsw -- --test-threads=1`
