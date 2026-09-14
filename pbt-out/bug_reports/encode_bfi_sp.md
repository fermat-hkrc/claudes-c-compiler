# Bug: encode_bfi treats SP/WSP as ZR
**Law:** Register 31 in BFI is WZR/XZR, not SP/WSP. SP/WSP is not a valid BFI operand.
**Impact:** `bfi wsp, w0, #0, #1` (and `sp` in Rd or Rn) is encoded as ZR number 31. gas / llvm-mc reject SP/WSP as "invalid operand".
**Function:** encode_bfi
**Detected by:** Negative/Error Contract
**Minimal input:** which=0, sp64=false, is_64=false, other=0 — `bfi wsp, w0, #0, #1` (serial reconfirm with PBT_TEST_JOBS=1)
**Expected:** Err
**Actual:** Ok(Word) — parse_reg_num maps sp/wsp to 31 and encode_bfi does not distinguish SP from ZR
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs::test_encode_bfi_regression_sp
