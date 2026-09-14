# Bug: encode_smull accepts W dest and X sources
**Law:** ARM ARM SMULL form is `SMULL <Xd>, <Wn>, <Wm>` only. A W destination or an X source must return Err. llvm-mc rejects `smull w0, w0, w0`, `smull x0, x0, x0`, and mixed X/W sources (`invalid operand for instruction`).
**Impact:** `smull w0, w1, w2` is encoded as if it were `smull x0, w1, w2` (sf always 1, `is_64` discarded). Callers get a 64-bit long-multiply encoding for a 32-bit dest mnemonic, which is the wrong instruction.
**Function:** encode_smull
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `[Reg("w0"), Reg("w0"), Reg("w0")]` (rd64=false, rn64=false, rm64=false; rd=rn=rm=0). Any width triple other than (X, W, W) is also accepted.
**Expected:** `Err`
**Actual:** `Ok(Word(...))` with bit 31 forced to 1 and register numbers taken regardless of X/W prefix
**Severity:** medium
**Regression test:** `src/backend/arm/assembler/encoder/data_processing.rs` `test_encode_smull_regression_wrong_width`
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1` / `--test-threads=1`
