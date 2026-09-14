# Bug: encode_sxtw accepts 32-bit dest (Wd)
**Law:** ARM ARM SXTW has only the 64-bit dest form `SXTW <Xd>, <Wn>`; llvm-mc rejects `sxtw w0, w0`.
**Impact:** `sxtw w0, w0` is encoded as 64-bit SXTW (sf=1 hardcoded), so a 32-bit dest name silently produces the X-register encoding. Invalid GNU-style assembly becomes a real instruction word.
**Function:** encode_sxtw
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `[Reg("w0"), Reg("w0")]` (sxtw w0, w0)
**Expected:** `Err(...)`
**Actual:** `Ok(Word(0x93407c00))` — dest width discarded; word is 64-bit SXTW x0, w0
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs `test_encode_sxtw_regression_wd`
**Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_sxtw_neg -- --test-threads=1`
