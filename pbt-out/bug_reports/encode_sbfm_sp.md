# Bug: encode_sbfm treats SP/WSP as ZR
**Law:** SBFM register 31 is WZR/XZR, not SP/WSP. SP/WSP as Rd or Rn must be rejected.
**Impact:** `sbfm wsp, w0, #0, #0` is encoded as SBFM wzr (parse_reg_num maps sp/wsp to 31). gas / llvm-mc reject SP/WSP as `invalid operand for instruction`.
**Function:** encode_sbfm
**Detected by:** Negative/Error Contract
**Minimal input:** which=0, sp64=false, is_64=false, other=0 — `sbfm wsp, w0, #0, #0` (serial reconfirm with PBT_TEST_JOBS=1)
**Expected:** Err
**Actual:** Ok(Word) — parse_reg_num maps "sp"/"wsp" to 31 and encode_sbfm does not distinguish SP from ZR
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs::test_encode_sbfm_regression_sp
