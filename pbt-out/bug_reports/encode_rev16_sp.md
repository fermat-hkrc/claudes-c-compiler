# Bug: encode_rev16 encodes SP/WSP as ZR
**Law:** ARM ARM Data-processing (1 source) REV16 uses register 31 as ZR, not SP. llvm-mc rejects `rev16 wsp, w0` and `rev16 sp, x0`.
**Impact:** `rev16 wsp, w0` is encoded as `rev16 wzr, w0` (and `rev16 sp, x0` as `rev16 xzr, x0`), producing the wrong instruction instead of an assembler error.
**Function:** encode_rev16
**Detected by:** Negative/Error Contract
**Minimal input:** `[Reg("wsp"), Reg("w0")]` (serial reconfirm: PBT_TEST_JOBS=1)
**Expected:** `Err(_)`
**Actual:** `Ok(Word(...))` — `parse_reg_num` maps `sp`/`wsp` to 31
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs `test_encode_rev16_regression_sp`
