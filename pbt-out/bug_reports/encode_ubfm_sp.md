# Bug: encode_ubfm accepts SP/WSP as Rd or Rn
**Law:** UBFM register 31 is ZR (xzr/wzr), not SP/WSP. SP or WSP as Rd or Rn must be rejected.
**Impact:** `ubfm wsp, w0, #0, #0` is encoded as UBFM with Rd=31 (same encoding as wzr) instead of Err. Callers that pass the stack pointer get a ZR bitfield move. llvm-mc reports `invalid operand for instruction`.
**Function:** encode_ubfm
**Detected by:** Negative/Error Contract
**Minimal input:** which=0, sp=wsp, is_64=false, other=0 — `[Reg("wsp"), Reg("w0"), Imm(0), Imm(0)]` (serial reconfirm with PBT_TEST_JOBS=1)
**Expected:** Err
**Actual:** Ok(Word) — parse_reg_num maps "sp"|"wsp" to 31, same as xzr/wzr; encode_ubfm does not distinguish SP from ZR
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs::test_encode_ubfm_regression_sp
