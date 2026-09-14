# Bug: encode_ldrsw encodes XZR/X31 as the memory base SP
**Law:** LDRSW base is Xn|SP; XZR is not a valid base (Rn=31 means SP).
**Impact:** `ldrsw x0, [xzr]` and `ldrsw x0, [x31]` encode as `[sp]`. llvm-mc: "invalid operand".
**Function:** encode_ldrsw
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `[Reg("x0"), Mem { base: "xzr", offset: 0 }]`
**Expected:** Err
**Actual:** Ok(Word) with Rn=31 (SP)
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::test_encode_ldrsw_regression_xzr_base
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_ldrsw -- --test-threads=1`
