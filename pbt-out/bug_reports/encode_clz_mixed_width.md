# Bug: encode_clz accepts mixed W/X register widths
**Law:** ARM CLZ requires matching W/W or X/X; mixed width (`clz x0, w0`) must return Err (llvm-mc: invalid operand).
**Impact:** sf is taken only from Rd; Rn's width is discarded (`let (rn, _) = get_reg(...)`). Mixed-width assembly is encoded as a 64- or 32-bit CLZ instead of rejected, so the object file silently disagrees with the source text.
**Function:** encode_clz
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** operands = [Reg("x0"), Reg("w0")]  (clz x0, w0)
**Expected:** Err
**Actual:** Ok(Word(0xdac01000)) — encoded as `clz x0, x0` (sf from Rd only).
**Severity:** medium
**Fix:** Require Rd and Rn to have the same GPR width (both W or both X) before encoding.
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs::test_encode_clz_regression_mixed_width
**Serial reconfirmation:** reproduced with PBT_TEST_JOBS=1 cargo test --lib encode_clz_neg -- --test-threads=1
