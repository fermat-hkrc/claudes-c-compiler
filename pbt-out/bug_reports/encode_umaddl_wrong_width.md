# Bug: encode_umaddl accepts W dest, X sources, and W accumulator
**Law:** ARM ARM UMADDL form is `UMADDL <Xd>, <Wn>, <Wm>, <Xa>` only. A W destination, an X multiply source, or a W accumulator must return Err. llvm-mc rejects `umaddl w0, w0, w0, w0`, `umaddl x0, x0, x0, x0`, and `umaddl x0, w1, w2, w3` (`invalid operand for instruction`).
**Impact:** `umaddl w0, w1, w2, w3` is encoded as if it were `umaddl x0, w1, w2, x3` (sf always 1, `is_64` discarded). Callers get a 64-bit unsigned long-multiply-add encoding for a 32-bit dest/acc mnemonic, which is the wrong instruction.
**Function:** encode_umaddl
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `[Reg("w0"), Reg("w0"), Reg("w0"), Reg("w0")]` (rd64=false, rn64=false, rm64=false, ra64=false; rd=rn=rm=ra=0). Any width quadruple other than (X, W, W, X) is also accepted.
**Expected:** `Err`
**Actual:** `Ok(Word(0x9ba00000))` with bit 31 forced to 1 and register numbers taken regardless of X/W prefix
**Severity:** medium
**Regression test:** `src/backend/arm/assembler/encoder/data_processing.rs` `test_encode_umaddl_regression_wrong_width`
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1` / `--test-threads=1`
