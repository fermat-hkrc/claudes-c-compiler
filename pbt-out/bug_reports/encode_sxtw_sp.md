# Bug: encode_sxtw encodes SP/WSP as XZR/WZR
**Law:** SXTW register 31 is XZR/WZR, never SP/WSP; llvm-mc rejects `sxtw sp, w0` and `sxtw x0, wsp`.
**Impact:** `parse_reg_num` maps sp/wsp to 31, so `sxtw wsp, w0` encodes as `sxtw xzr, w0`. Stack-pointer names are accepted as zero registers.
**Function:** encode_sxtw
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `[Reg("wsp"), Reg("w0")]` (sxtw wsp, w0)
**Expected:** `Err(...)`
**Actual:** `Ok(Word)` — SP/WSP encoded as register 31 (ZR)
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs `test_encode_sxtw_regression_sp`
**Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_sxtw_neg -- --test-threads=1`
