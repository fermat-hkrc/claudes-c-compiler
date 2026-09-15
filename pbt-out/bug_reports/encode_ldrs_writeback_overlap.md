# Bug: encode_ldrs encodes pre/post-index when Rt==Rn (Rn!=SP)
**Law:** ARM: pre/post writeback with Rt==Rn (and Rn not SP) is unpredictable; llvm-mc/gas reject it.
**Impact:** `ldrsb x0, [x0, #4]!` is encoded instead of rejected, producing unpredictable-at-runtime code.
**Function:** encode_ldrs
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** size=0, is_64=false, rt=0 → `[Reg("w0"), MemPreIndex { base: "x0", offset: 4 }]`
**Expected:** Err
**Actual:** Ok(Word(0x38c04c00)) — no Rt/Rn overlap check
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::test_encode_ldrs_regression_writeback_rt_eq_rn
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_ldrs_neg_writeback_overlap -- --test-threads=1`
