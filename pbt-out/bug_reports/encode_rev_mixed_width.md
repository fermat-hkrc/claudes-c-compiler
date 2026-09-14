# Bug: encode_rev accepts mixed W/X register widths
**Law:** ARM ARM REV is either `<Wd>, <Wn>` or `<Xd>, <Xn>`. llvm-mc rejects `rev x0, w0`.
**Impact:** Mixed-width operands are encoded with `sf` and opcode taken only from Rd, so `rev x0, w0` is emitted as 64-bit REV (opc=000011) of x0 with Rn numbered from the W register — a different instruction than the text.
**Function:** encode_rev
**Detected by:** Negative/Error Contract
**Minimal input:** `[Reg("x0"), Reg("w0")]` (serial reconfirm: PBT_TEST_JOBS=1)
**Expected:** `Err(_)`
**Actual:** `Ok(Word(...))` — `sf`/`opc` come from Rd via `is_64bit_reg`; Rn width is ignored
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs `test_encode_rev_regression_mixed_width`
