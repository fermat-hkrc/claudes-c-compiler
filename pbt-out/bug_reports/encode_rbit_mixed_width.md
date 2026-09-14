# Bug: encode_rbit accepts mixed W/X register widths
**Law:** ARM RBIT requires matching W/W or X/X; mixed width (`rbit x0, w0`) must return Err (llvm-mc: invalid operand).
**Impact:** sf is taken only from Rd; Rn's width is discarded (`let (rn, _) = get_reg(...)`). Mixed-width assembly is encoded as a 64- or 32-bit RBIT instead of rejected, so the object file silently disagrees with the source text.
**Function:** encode_rbit
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** operands = [Reg("x0"), Reg("w0")]  (rbit x0, w0)
**Expected:** Err
**Actual:** Ok(Word(0xdac00000)) — encoded as `rbit x0, x0` (sf from Rd only).
**Severity:** medium
**Fix:** Require Rd and Rn to have the same GPR width (both W or both X) before encoding.
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs::test_encode_rbit_regression_mixed_width
**Serial reconfirmation:** reproduced with PBT_TEST_JOBS=1 cargo test --lib encode_rbit_pbt::encode_rbit_neg_ -- --test-threads=1
