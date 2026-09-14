# Bug: encode_ldxr_stxr accepts an X register as STXR status
**Law:** STXR/STXRB/STXRH status is Ws (32-bit), with 31 meaning WZR. An X register or SP as status must be rejected.
**Impact:** `stxr x0, x1, [x2]` is encoded using the X register number in Rs. GNU as / llvm-mc reject X as STXR status. Silent mis-assembly.
**Function:** encode_ldxr_stxr
**Detected by:** Negative/Error Contract
**Minimal input:** kind=4 — `stxr x0, x1, [x2]`
**Expected:** Err
**Actual:** Ok(Word) with Rs equal to the X register number
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_ldxr_stxr_pbt::test_encode_ldxr_stxr_regression_x_as_ws
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / RUST_TEST_THREADS=1 (kind=4 of encode_ldxr_stxr_neg_invalid_regs; dedicated regression)
