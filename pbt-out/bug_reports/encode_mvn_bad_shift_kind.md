# Bug: encode_mvn maps unknown shift kinds to LSL
**Law:** ARM ARM Logical (shifted register) shift is one of LSL, LSR, ASR, ROR. Any other kind must be rejected.
**Impact:** `mvn w0, w0, foo #0` is accepted and encoded as LSL (st=0). llvm-mc/gas reject unknown shift mnemonics.
**Function:** encode_mvn
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `[Reg("w0"), Reg("w0"), Shift{kind:"foo", amount:0}]`
**Expected:** Err
**Actual:** Ok(Word) with shift type 0 (LSL)
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::test_encode_mvn_regression_bad_shift_kind
**Serial reconfirm:** PBT_TEST_JOBS=1, `--test-threads=1` — reproduced
