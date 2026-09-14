# Bug: encode_sbfm accepts mixed W/X registers
**Law:** SBFM Rd and Rn must have the same width (both W or both X). Mixed W/X must be rejected.
**Impact:** `sbfm x0, w0, #0, #0` is encoded using sf from Rd only, ignoring Rn width. gas / llvm-mc reject mixed W/X as `invalid operand for instruction`.
**Function:** encode_sbfm
**Detected by:** Negative/Error Contract
**Minimal input:** rd=0, rn=0, rd64=true, rn64=false — `sbfm x0, w0, #0, #0` (serial reconfirm with PBT_TEST_JOBS=1)
**Expected:** Err
**Actual:** Ok(Word) — get_reg for Rn discards is_64; sf is taken from Rd only
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs::test_encode_sbfm_regression_mixed_width
