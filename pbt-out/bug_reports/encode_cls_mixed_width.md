# Bug: encode_cls accepts mixed W/X register widths
**Law:** ARM CLS requires matching W/W or X/X; mixed width must return Err (llvm-mc: invalid operand).
**Impact:** `cls x0, w0` is encoded as 64-bit `cls x0, x0` (sf taken from Rd only), producing the wrong instruction size.
**Function:** encode_cls
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** operands = [Reg("x0"), Reg("w0")]  (cls x0, w0)
**Expected:** Err
**Actual:** Ok(Word(0xdac01400)) — same encoding as `cls x0, x0`. encode_cls uses is_64 from Rd and ignores Rn's width.
**Severity:** medium
**Fix:** Compare Rd and Rn widths from get_reg and Err when they differ.
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs::test_encode_cls_regression_mixed_width
**Serial reconfirmation:** reproduced with cargo test --lib encode_cls -- --test-threads=1
