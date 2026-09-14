# Bug: encode_umull accepts W dest and X sources
**Law:** ARM ARM UMULL form is `UMULL <Xd>, <Wn>, <Wm>` only. A W destination or an X source must return Err. llvm-mc rejects `umull w0, w0, w0`, `umull x0, x0, x0`, and mixed X/W sources (`invalid operand for instruction`).
**Impact:** `umull w0, w1, w2` is encoded as if it were `umull x0, w1, w2` (sf always 1, `is_64` discarded). Callers get a 64-bit unsigned long-multiply encoding for a 32-bit dest mnemonic, which is the wrong instruction.
**Function:** encode_umull
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `[Reg("w0"), Reg("w0"), Reg("w0")]` (rd64=false, rn64=false, rm64=false; rd=rn=rm=0). Any width triple other than (X, W, W) is also accepted.
**Expected:** `Err`
**Actual:** `Ok(Word(...))` with bit 31 forced to 1 and register numbers taken regardless of X/W prefix
**Severity:** medium
**Regression test:** `src/backend/arm/assembler/encoder/data_processing.rs` `test_encode_umull_regression_wrong_width`
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1` / `RUST_TEST_THREADS=1`
