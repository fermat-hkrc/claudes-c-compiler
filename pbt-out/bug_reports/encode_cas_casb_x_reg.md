# Bug: encode_cas accepts X registers for CASB/CASH
**Law:** CASB/CASH (and acquire/release byte/half variants) require W registers. llvm-mc/gas reject `casb x0, x1, [x2]`.
**Impact:** Size is forced from the 'b'/'h' suffix; register width is ignored. `casb x0, x1, [x2]` encodes as CASB w0, w1, [x2]. Silent wrong register width.
**Function:** encode_cas
**Detected by:** Negative/Error Contract
**Minimal input:** `casb x0, x1, [x2]` (also cash/casab/casalh with X)
**Expected:** Err
**Actual:** Ok(Word) encoding of `casb w0, w1, [x2]`
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_cas_pbt::test_encode_cas_regression_casb_x_reg
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / --test-threads=1 (encode_cas_neg_mixed_fp_xbyte covers casb/cash X; shrunk witness was mixed W/X)
