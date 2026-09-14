# Bug: encode_ldaxr_stlxr accepts an X register as STLXR status
**Law:** STLXR status is Ws (Wt / WZR). An X-width status register must be rejected.
**Impact:** `stlxr x0, x1, [x2]` is encoded as `stlxr w0, x1, [x2]` because get_reg discards is_64 for the status operand. GNU as / llvm-mc reject X as STLXR status. Silent wrong-register assembly.
**Function:** encode_ldaxr_stlxr
**Detected by:** Negative/Error Contract
**Minimal input:** `stlxr x0, x1, [x2]` (regression of encode_ldaxr_stlxr_neg_invalid_regs kind=4)
**Expected:** Err
**Actual:** Ok(Word) encoding Ws=0 as if the status were w0
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_ldaxr_stlxr_pbt::test_encode_ldaxr_stlxr_regression_x_as_ws
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / RUST_TEST_THREADS=1
