# Bug: encode_negs maps unknown and ROR shift kinds to LSL
**Law:** ARM ARM Add/subtract (shifted register) shift is one of LSL, LSR, ASR (not ROR). Any other kind must be rejected.
**Impact:** `negs w0, w0, ror #0` and `negs w0, w0, foo #0` are accepted and encoded as LSL (st=0). llvm-mc/gas reject ROR and unknown shift mnemonics for NEGS.
**Function:** encode_negs
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `[Reg("w0"), Reg("w0"), Shift{kind:"ror", amount:0}]`
**Expected:** Err
**Actual:** Ok(Word) with shift type 0 (LSL)
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::test_encode_negs_regression_bad_shift_kind
**Serial reconfirm:** PBT_TEST_JOBS=1, `--test-threads=1` — reproduced
