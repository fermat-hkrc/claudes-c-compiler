# Bug: ADD/SUB accepts mixed x/w register widths
**Law:** ARM ARM ADD/SUB immediate and shifted-register forms require all registers the same width (`<Wd>, <Wn>, …` or `<Xd>, <Xn>, …`). llvm-mc rejects `add x0, w0, w0` and `add x0, w1, #1`. Mixed width must Err. (Extended-register `add Xd, Xn, Wm, sxtw` is a different form and is not this bug.)
**Impact:** `add x0, w0, w0` is encoded as a 64-bit ADD (sf taken only from Rd). The 32-bit operands are silently widened; the assembled instruction is not the source text.
**Function:** encode_add_sub
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** operands = [Reg("x0"), Reg("w0"), Reg("w0")], is_sub=false, set_flags=false
**Expected:** Err
**Actual:** Ok(Word) — `sf` comes only from operand 0; Rn/Rm width is ignored
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::test_encode_add_sub_regression_mixed_width
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_add_sub_neg_mixed_width -- --test-threads=1`
