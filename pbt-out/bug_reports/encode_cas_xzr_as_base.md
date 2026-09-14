# Bug: encode_cas accepts XZR/x31/WZR as the memory base
**Law:** ARM ARM Rn is Xn|SP. Register 31 in the Rn field is SP, not XZR. llvm-mc/gas reject `cas x0, x1, [xzr]` and `[x31]`.
**Impact:** `cas x0, x1, [xzr]` and `[x31]` encode as `cas x0, x1, [sp]` because parse_reg_num maps xzr/x31/wzr to 31. Silent wrong address register (ZR becomes SP).
**Function:** encode_cas
**Detected by:** Negative/Error Contract
**Minimal input:** base="xzr" (also "x31", "wzr") — `cas x0, x1, [xzr]`
**Expected:** Err
**Actual:** Ok(Word) encoding of `cas x0, x1, [sp]`
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_cas_pbt::test_encode_cas_regression_xzr_as_base
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / --test-threads=1 (encode_cas_neg_sp_zr_base covers these kinds; shrunk witness was SP-as-Rs)
