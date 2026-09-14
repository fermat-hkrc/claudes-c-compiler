# Bug: encode_umulh encodes SP/WSP as XZR/WZR
**Law:** In the Data-processing (3 source) UMULH format, register 31 is XZR, never SP. SP/WSP is not a valid UMULH operand.
**Impact:** `umulh wsp, x0, x0` (and SP in any slot) is encoded as `umulh wzr/xzr, ...` instead of an error. gas / llvm-mc reject SP/WSP.
**Function:** encode_umulh
**Detected by:** Negative/Error Contract
**Minimal input:** which=0, is_64=false, a=0, b=0 — `umulh wsp, x0, x0` (serial reconfirm with PBT_TEST_JOBS=1)
**Expected:** Err
**Actual:** Ok(Word) encoding register 31 — parse_reg_num maps "sp"|"wsp" to 31
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::test_encode_umulh_regression_sp
