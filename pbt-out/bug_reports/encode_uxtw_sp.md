# Bug: encode_uxtw encodes SP/WSP as XZR/WZR
**Law:** UXTW register 31 is XZR/WZR, never SP/WSP; llvm-mc rejects `uxtw sp, w1` and `uxtw x0, wsp`.
**Impact:** `parse_reg_num` maps sp/wsp to 31, so `uxtw wsp, w0` encodes as register 31 (ZR). Stack-pointer names are accepted as zero registers.
**Function:** encode_uxtw
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `[Reg("wsp"), Reg("w0")]` (uxtw wsp, w0)
**Expected:** `Err(...)`
**Actual:** `Ok(Word)` — SP/WSP encoded as register 31 (ZR)
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs `test_encode_uxtw_regression_sp`
**Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_uxtw -- --test-threads=1`
