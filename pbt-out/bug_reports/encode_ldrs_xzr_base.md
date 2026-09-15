# Bug: encode_ldrs accepts XZR as the memory base
**Law:** LDRSB/LDRSH base is Xn or SP; register 31 in Rn means SP, not XZR. Assemblers reject `[xzr]`.
**Impact:** `ldrsb x0, [xzr]` encodes as `ldrsb x0, [sp]`.
**Function:** encode_ldrs
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** size=0, is_64=false, rt=0 → `[Reg("w0"), Mem { base: "xzr", offset: 0 }]`
**Expected:** Err
**Actual:** Ok(Word(0x39c003e0)) — parse_reg_num maps xzr to 31 (SP encoding)
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::test_encode_ldrs_regression_xzr_base
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_ldrs_neg_xzr_base -- --test-threads=1`
