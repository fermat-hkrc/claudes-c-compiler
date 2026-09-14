# Bug: encode_div accepts SP/WSP as a GPR operand
**Law:** ARM ARM UDIV/SDIV encode register 31 as XZR/WZR, never SP/WSP. llvm-mc rejects `udiv sp, ...` / `sdiv wsp, ...`.
**Impact:** `sdiv wsp, w0, w0` is assembled as `sdiv wzr, w0, w0` (parse_reg_num maps both wsp and wzr to 31). Stack-pointer typos silently become zero-register divides.
**Function:** encode_div
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `encode_div([Reg("wsp"), Reg("w0"), Reg("w0")], unsigned=false)` i.e. `sdiv wsp, w0, w0` (which=0, is_64=false)
**Expected:** Err
**Actual:** Ok(Word(0x1ac00c1f)) — same as `sdiv wzr, w0, w0`. Reproduced serially (`PBT_TEST_JOBS=1`).
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::encode_div_pbt::test_encode_div_regression_sp
