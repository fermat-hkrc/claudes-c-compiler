# Bug: encode_cls accepts SP/WSP as register 31
**Law:** ARM CLS register 31 is ZR not SP; SP/WSP must return Err (llvm-mc: invalid operand).
**Impact:** `cls wsp, w0` encodes as `cls wzr, w0`, silently substituting the zero register for the stack pointer.
**Function:** encode_cls
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** operands = [Reg("wsp"), Reg("w0")]  (cls wsp, w0)
**Expected:** Err
**Actual:** Ok(Word(0x5ac0141f)) — same encoding as `cls wzr, w0`. parse_reg_num maps "sp"/"wsp" to 31; encode_cls does not reject SP.
**Severity:** medium
**Fix:** After get_reg, reject names that parse as SP/WSP (register 31 is ZR for CLS).
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs::test_encode_cls_regression_sp
**Serial reconfirmation:** reproduced with cargo test --lib encode_cls -- --test-threads=1
