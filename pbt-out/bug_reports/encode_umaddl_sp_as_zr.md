# Bug: encode_umaddl encodes SP/WSP as XZR/WZR
**Law:** ARM ARM Data-processing (3 source) register 31 is XZR/WZR, never SP/WSP. llvm-mc rejects `umaddl sp, w1, w2, x3`, `umaddl x0, wsp, w2, x3`, and `umaddl x0, w1, w2, sp` (`invalid operand for instruction`).
**Impact:** `umaddl wsp, w0, w0, x0` is encoded as `umaddl xzr, w0, w0, x0` because `parse_reg_num` maps `sp`/`wsp` to 31. Stack-pointer operands are silently rewritten to the zero register.
**Function:** encode_umaddl
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `[Reg("wsp"), Reg("w0"), Reg("w0"), Reg("x0")]` (which=0, is_64=false, a=0, b=0). SP/WSP in any of the four slots is accepted.
**Expected:** `Err`
**Actual:** `Ok(Word(0x9ba0001f))` with that slot encoded as register 31 (ZR)
**Severity:** medium
**Regression test:** `src/backend/arm/assembler/encoder/data_processing.rs` `test_encode_umaddl_regression_sp`
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1` / `--test-threads=1`
