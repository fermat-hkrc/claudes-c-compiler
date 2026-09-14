# Bug: encode_rev16 accepts mixed W/X register widths
**Law:** ARM ARM REV16 is either `<Wd>, <Wn>` or `<Xd>, <Xn>`. llvm-mc rejects `rev16 x0, w0`.
**Impact:** Mixed-width operands are encoded with `sf` taken only from Rd, so `rev16 x0, w0` is emitted as 64-bit REV16 of x0, x0-numbered source — a different instruction than the text.
**Function:** encode_rev16
**Detected by:** Negative/Error Contract
**Minimal input:** `[Reg("x0"), Reg("w0")]` (serial reconfirm: PBT_TEST_JOBS=1)
**Expected:** `Err(_)`
**Actual:** `Ok(Word(...))` — `sf` comes from Rd via `is_64bit_reg`; Rn width is ignored
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs `test_encode_rev16_regression_mixed_width`
