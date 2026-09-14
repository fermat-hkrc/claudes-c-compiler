# Bug: encode_rev encodes SP/WSP as ZR
**Law:** ARM ARM Data-processing (1 source) REV uses register 31 as ZR, not SP. llvm-mc rejects `rev wsp, w0` and `rev sp, x0`.
**Impact:** `rev wsp, w0` is encoded as `rev wzr, w0` (and `rev sp, x0` as `rev xzr, x0`), producing the wrong instruction instead of an assembler error.
**Function:** encode_rev
**Detected by:** Negative/Error Contract
**Minimal input:** `[Reg("wsp"), Reg("w0")]` (serial reconfirm: PBT_TEST_JOBS=1)
**Expected:** `Err(_)`
**Actual:** `Ok(Word(...))` — `parse_reg_num` maps `sp`/`wsp` to 31
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs `test_encode_rev_regression_sp`
