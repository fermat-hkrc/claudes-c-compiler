# Bug: encode_ldrsw encodes SP destination as XZR
**Law:** LDRSW destination is Xt with 31=XZR; SP is not a valid Rt and must be rejected.
**Impact:** `ldrsw sp, [x1]` encodes Rt=31 (XZR) instead of erroring. llvm-mc: "invalid operand".
**Function:** encode_ldrsw
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `[Reg("sp"), Mem { base: "x1", offset: 0 }]`
**Expected:** Err
**Actual:** Ok(Word) with Rt=31 — parse_reg_num maps "sp" to 31
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::test_encode_ldrsw_regression_sp_dest
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_ldrsw -- --test-threads=1`
