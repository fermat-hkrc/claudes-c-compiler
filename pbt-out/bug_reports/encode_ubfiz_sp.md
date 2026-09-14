# Bug: encode_ubfiz accepts SP/WSP as register 31
**Law:** ARM Bitfield Move uses ZR for register 31, not SP. `ubfiz wsp/sp, ...` and `ubfiz ..., wsp/sp, ...` must be rejected with Err.
**Impact:** `ubfiz wsp, w0, #0, #1` encodes as UBFIZ WZR, W0 (register 31). gas / llvm-mc reject SP/WSP (`invalid operand for instruction`). The assembler silently treats a stack-pointer operand as the zero register.
**Function:** encode_ubfiz
**Detected by:** Negative/Error Contract
**Minimal input:** which=0, sp64=false, is_64=false, other=0 — `ubfiz wsp, w0, #0, #1` (serial reconfirm with `--test-threads=1` / PBT_TEST_JOBS=1)
**Expected:** Err
**Actual:** Ok(Word) — parse_reg_num maps sp/wsp to 31; encode_ubfiz does not distinguish SP from ZR.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs::test_encode_ubfiz_regression_sp
