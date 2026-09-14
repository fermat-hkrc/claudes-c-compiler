# Bug: encode_extr accepts mixed W/X registers
**Law:** EXTR requires Rd, Rn, and Rm to be the same size (all W or all X). Mixed W/X must return Err (llvm-mc: invalid operand).
**Impact:** `extr w0, x0, w0, #0` is encoded using sf from Rd only, silently ignoring Rn/Rm width. The assembler emits a 32-bit EXTR while one source was 64-bit, producing wrong machine code.
**Function:** encode_extr
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** operands = [Reg("w0"), Reg("x0"), Reg("w0"), Imm(0)]  (rd64=false, rn64=true, rm64=false)
**Expected:** Err
**Actual:** Ok(Word). encode_extr takes is_64 only from Rd (get_reg index 0) and discards Rn/Rm width.
**Severity:** medium
**Fix:** Require matching W/X on Rd, Rn, and Rm; return Err on mixed width.
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs::test_encode_extr_regression_mixed_width
**Serial reconfirmation:** reproduced with cargo test --lib encode_extr -- --test-threads=1
