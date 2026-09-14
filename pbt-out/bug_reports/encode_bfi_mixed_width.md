# Bug: encode_bfi accepts mixed W/X registers
**Law:** BFI requires both registers to be the same width: Wd, Wn or Xd, Xn.
**Impact:** `bfi x0, w0, #0, #1` (and the W dest / X src swap) is encoded using the destination's sf bit and the source's register number, producing a same-width BFM encoding that gas / llvm-mc reject as "invalid operand".
**Function:** encode_bfi
**Detected by:** Negative/Error Contract
**Minimal input:** rd=0, rn=0, rd64=true, rn64=false — `bfi x0, w0, #0, #1` (serial reconfirm with PBT_TEST_JOBS=1)
**Expected:** Err
**Actual:** Ok(Word) — encode_bfi takes is_64 only from Rd (`get_reg(operands, 0)`) and discards Rn's width (`let (rn, _) = get_reg(operands, 1)`)
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs::test_encode_bfi_regression_mixed_width
