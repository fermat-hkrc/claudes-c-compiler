# Bug: encode_ubfiz accepts mixed W/X register widths
**Law:** UBFIZ requires Rd and Rn to have the same datasize (both W or both X). Mixed W/X must be rejected with Err.
**Impact:** `ubfiz x0, w0, #0, #1` encodes using sf from Rd (X) and Rn's number, producing a 64-bit UBFIZ with Rn=w0's encoding. gas / llvm-mc reject mixed widths (`invalid operand for instruction`).
**Function:** encode_ubfiz
**Detected by:** Negative/Error Contract
**Minimal input:** rd=0, rn=0, rd64=true, rn64=false — `ubfiz x0, w0, #0, #1` (serial reconfirm with `--test-threads=1` / PBT_TEST_JOBS=1)
**Expected:** Err
**Actual:** Ok(Word) — encode_ubfiz takes sf from Rd via get_reg and ignores Rn's width.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs::test_encode_ubfiz_regression_mixed_width
