# Bug: encode_ldrsw encodes pre/post-index LDRSW when Rt equals Rn
**Law:** Writeback LDRSW with the destination equal to the base (and the base not SP) is unpredictable; assemblers must reject it.
**Impact:** `ldrsw x0, [x0, #4]!` is encoded. llvm-mc: "unpredictable LDR instruction, writeback base is also a source".
**Function:** encode_ldrsw
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `[Reg("x0"), MemPreIndex { base: "x0", offset: 4 }]`
**Expected:** Err
**Actual:** Ok(Word(0xb8804c00))
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::test_encode_ldrsw_regression_writeback_rt_eq_rn
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_ldrsw -- --test-threads=1`
