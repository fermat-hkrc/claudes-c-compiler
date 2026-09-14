# Bug: encode_bfm accepts mixed W/X registers
**Law:** BFM requires Rd and Rn to have the same width (both W or both X). Mixed W/X must be rejected.
**Impact:** `bfm x0, w0, #0, #0` is encoded using sf from Rd only, producing a 64-bit BFM with Rn taken from a W register. gas / llvm-mc reject mixed W/X as "invalid operand".
**Function:** encode_bfm
**Detected by:** Negative/Error Contract
**Minimal input:** rd=0, rn=0, rd64=true, rn64=false — `bfm x0, w0, #0, #0` (serial reconfirm with PBT_TEST_JOBS=1)
**Expected:** Err
**Actual:** Ok(Word) — encode_bfm takes is_64 from Rd via get_reg and discards Rn's width
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs::test_encode_bfm_regression_mixed_width
