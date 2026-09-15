# Bug: encode_ldrs accepts a W register as the memory base
**Law:** LDRSB/LDRSH base is Xn or SP, never Wn.
**Impact:** `ldrsb x0, [w1]` encodes as if the base were x1.
**Function:** encode_ldrs
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** size=0, is_64=false, rt=0, rn=0 → `[Reg("w0"), Mem { base: "w0", offset: 0 }]`
**Expected:** Err
**Actual:** Ok(Word(0x39c00000)) — parse_reg_num accepts the w prefix
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::test_encode_ldrs_regression_w_base
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_ldrs_neg_w_base -- --test-threads=1`
