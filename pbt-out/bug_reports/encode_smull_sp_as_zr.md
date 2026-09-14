# Bug: encode_smull encodes SP/WSP as XZR/WZR
**Law:** ARM ARM Data-processing (3 source) register 31 is XZR/WZR, never SP/WSP. llvm-mc rejects `smull sp, w1, w2` and `smull x0, wsp, w2` (`invalid operand for instruction`).
**Impact:** `smull wsp, w0, w0` is encoded as `smull xzr, w0, w0` because `parse_reg_num` maps `sp`/`wsp` to 31. Stack-pointer operands are silently rewritten to the zero register.
**Function:** encode_smull
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `[Reg("wsp"), Reg("w0"), Reg("w0")]` (which=0, is_64=false, a=0, b=0). SP/WSP in any of the three slots is accepted.
**Expected:** `Err`
**Actual:** `Ok(Word(...))` with that slot encoded as register 31 (ZR)
**Severity:** medium
**Regression test:** `src/backend/arm/assembler/encoder/data_processing.rs` `test_encode_smull_regression_sp`
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1` / `--test-threads=1`
