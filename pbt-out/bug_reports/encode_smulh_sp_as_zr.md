# Bug: encode_smulh treats SP/WSP as XZR
**Law:** Register 31 in SMULH is XZR, not SP/WSP. SP/WSP is not a valid SMULH operand.
**Impact:** `smulh wsp, x0, x0` (and `sp` in any slot) is encoded as XZR/WZR number 31. gas / llvm-mc reject SP/WSP as "invalid operand".
**Function:** encode_smulh
**Detected by:** Negative/Error Contract
**Minimal input:** which=0, is_64=false, a=0, b=0 — `smulh wsp, x0, x0` (serial reconfirm with PBT_TEST_JOBS=1)
**Expected:** Err
**Actual:** Ok(Word) — parse_reg_num maps sp/wsp to 31 and encode_smulh does not distinguish SP from XZR
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::test_encode_smulh_regression_sp
