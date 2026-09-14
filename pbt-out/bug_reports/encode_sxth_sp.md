# Bug: encode_sxth encodes SP/WSP as ZR
**Law:** SXTH/SBFM register 31 is WZR/XZR, never SP/WSP. llvm-mc rejects `sxth wsp, w0` / `sxth sp, w1` / `sxth x0, sp`.
**Impact:** `sxth wsp, ...` or `sxth ..., sp` is assembled as the ZR encoding, silently changing the operand identity (stack pointer vs zero register).
**Function:** encode_sxth
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `[Reg("wsp"), Reg("w0")]` (sxth wsp, w0)
**Expected:** `Err(...)`
**Actual:** `Ok(Word)` — `parse_reg_num` maps `sp`/`wsp` to 31, same as ZR
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs `test_encode_sxth_regression_sp`
**Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_sxth_neg -- --test-threads=1`
