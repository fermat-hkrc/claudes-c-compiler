# Bug: encode_logical ignores mismatched NEON source arrangements
**Law:** Vd.T, Vn.T, Vm.T must share the same arrangement T. llvm-mc rejects `and v0.16b, v0.8b, v0.16b`.
**Impact:** Source arrangements are discarded; dest T is used, so a mixed 16b/8b instruction is encoded as if all were dest T.
**Function:** encode_logical (via encode_neon_logical)
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `[RegArrangement{v0,"16b"}, RegArrangement{v0,"8b"}, RegArrangement{v0,"16b"}]`, opc=0
**Expected:** Err
**Actual:** Ok(Word) — `_arr_n` / `_arr_m` are ignored
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::test_encode_logical_regression_neon_mismatch
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1
