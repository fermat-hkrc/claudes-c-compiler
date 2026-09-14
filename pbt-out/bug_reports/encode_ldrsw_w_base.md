# Bug: encode_ldrsw accepts a W register as the memory base
**Law:** LDRSW base is Xn|SP; Wn is invalid.
**Impact:** `ldrsw x0, [w1]` encodes as `[x1]`. parse_reg_num("w1")=1 with no width check.
**Function:** encode_ldrsw
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `[Reg("x0"), Mem { base: "w1", offset: 0 }]`
**Expected:** Err
**Actual:** Ok(Word) with Rn=1
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::test_encode_ldrsw_regression_w_base
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_ldrsw -- --test-threads=1`
