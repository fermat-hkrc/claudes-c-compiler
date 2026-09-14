# Bug: encode_ubfx treats SP/WSP as ZR
**Law:** Register 31 in UBFX is WZR/XZR, not SP/WSP. SP/WSP is not a valid UBFX operand.
**Impact:** `ubfx wsp, w0, #0, #1` (and `sp` in Rd or Rn) is encoded as ZR number 31. gas / llvm-mc reject SP/WSP as "invalid operand".
**Function:** encode_ubfx
**Detected by:** Negative/Error Contract
**Minimal input:** which=0, sp64=false, is_64=false, other=0 — `ubfx wsp, w0, #0, #1` (serial reconfirm with PBT_TEST_JOBS=1)
**Expected:** Err
**Actual:** Ok(Word) — parse_reg_num maps sp/wsp to 31 and encode_ubfx does not distinguish SP from ZR
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs::test_encode_ubfx_regression_sp
