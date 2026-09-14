# Bug: encode_sbfiz accepts mixed W/X register widths
**Law:** SBFIZ requires matching datasize: Wd,Wn or Xd,Xn. Mixed W/X must be rejected with Err.
**Impact:** `sbfiz x0, w0, #0, #1` encodes using sf from Rd only, silently ignoring Rn's width. gas / llvm-mc reject mixed widths (`invalid operand for instruction`).
**Function:** encode_sbfiz
**Detected by:** Negative/Error Contract
**Minimal input:** rd=0, rn=0, rd64=true, rn64=false — `sbfiz x0, w0, #0, #1` (serial reconfirm with `--test-threads=1` / PBT_TEST_JOBS=1)
**Expected:** Err
**Actual:** Ok(Word) — get_reg for Rn discards is_64 (`let (rn, _) = get_reg(operands, 1)?`).
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs::test_encode_sbfiz_regression_mixed_width
