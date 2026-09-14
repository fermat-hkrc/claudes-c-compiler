# Bug: encode_extr accepts SP/WSP as a GPR
**Law:** EXTR uses ZR not SP at register 31. SP/WSP in Rd, Rn, or Rm must return Err (llvm-mc: invalid operand).
**Impact:** `extr wsp, w0, w0, #0` is encoded as if WZR were written, producing a well-formed EXTR that does not match the source text. Callers that accidentally pass SP get silent wrong code.
**Function:** encode_extr
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** operands = [Reg("wsp"), Reg("w0"), Reg("w0"), Imm(0)]  (extr wsp, w0, w0, #0; which=0)
**Expected:** Err
**Actual:** Ok(Word). parse_reg_num maps sp/wsp to 31, so SP is encoded as ZR.
**Severity:** medium
**Fix:** Reject SP/WSP in any of Rd/Rn/Rm (register 31 is ZR for EXTR).
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs::test_encode_extr_regression_sp
**Serial reconfirmation:** reproduced with cargo test --lib encode_extr -- --test-threads=1
