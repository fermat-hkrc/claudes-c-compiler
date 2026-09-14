# Bug: encode_ldxr_stxr accepts an X data register on byte/half exclusive
**Law:** LDXRB/LDXRH/STXRB/STXRH data register is Wt, not Xt. llvm-mc rejects `ldxrb x0, [x1]`.
**Impact:** `ldxrb x0, [x1]` is encoded with size=00 and Rt taken from the X register number. GNU as / llvm-mc require Wt. Silent mis-assembly of a reserved/invalid form.
**Function:** encode_ldxr_stxr
**Detected by:** Negative/Error Contract
**Minimal input:** kind=5 — `ldxrb x0, [x1]` (forced_size=Some(0b00) with Xt)
**Expected:** Err
**Actual:** Ok(Word) with size=00 and Rt from the X register
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_ldxr_stxr_pbt::test_encode_ldxr_stxr_regression_x_data_byte
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / RUST_TEST_THREADS=1 (kind=5 of encode_ldxr_stxr_neg_invalid_regs; dedicated regression)
