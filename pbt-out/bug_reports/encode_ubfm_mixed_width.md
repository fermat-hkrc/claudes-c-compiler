# Bug: encode_ubfm accepts mixed W/X registers
**Law:** UBFM requires Rd and Rn to have the same datasize (both W or both X). Mixed-width pairs must be rejected.
**Impact:** `ubfm x0, w0, #0, #0` is encoded using sf from Rd only, producing a 64-bit UBFM with Rn's number. llvm-mc reports `invalid operand for instruction`. Mixed-width assembly is accepted and silently given the destination's width.
**Function:** encode_ubfm
**Detected by:** Negative/Error Contract
**Minimal input:** rd=0, rn=0, rd64=true, rn64=false — `[Reg("x0"), Reg("w0"), Imm(0), Imm(0)]` (reproduced in the sweep run; same deterministic Ok(Word) as sibling encoders)
**Expected:** Err
**Actual:** Ok(Word) — encode_ubfm takes sf from Rd via get_reg and ignores Rn's is_64
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs::test_encode_ubfm_regression_mixed_width
