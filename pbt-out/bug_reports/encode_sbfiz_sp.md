# Bug: encode_sbfiz accepts SP/WSP as register 31
**Law:** SBFIZ operands are GPR/ZR; register 31 is ZR not SP. SP/WSP must be rejected with Err.
**Impact:** `sbfiz wsp, w0, #0, #1` encodes as if the destination were wzr. gas / llvm-mc reject SP/WSP (`invalid operand for instruction`). Callers that emit or parse SP in this slot get a ZR encoding instead of an assembler error.
**Function:** encode_sbfiz
**Detected by:** Negative/Error Contract
**Minimal input:** which=0, sp64=false, is_64=false, other=0 — `sbfiz wsp, w0, #0, #1` (serial reconfirm with `--test-threads=1` / PBT_TEST_JOBS=1)
**Expected:** Err
**Actual:** Ok(Word) — parse_reg_num maps sp/wsp to 31, so SBFIZ encodes ZR.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs::test_encode_sbfiz_regression_sp
