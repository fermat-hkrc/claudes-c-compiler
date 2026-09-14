# Bug: encode_umaddl encodes FP/SIMD register names as GPRs
**Law:** Scalar UMADDL operands are GPRs (`Xd`, `Wn`, `Wm`, `Xa`). FP/SIMD names (`d`/`s`/`q`/`v`/`h`/`b` prefixes) must return Err. llvm-mc rejects `umaddl d0, w1, w2, x3` (`invalid operand for instruction`).
**Impact:** `umaddl d0, w1, w2, x3` is encoded as `umaddl x0, w1, w2, x3` because `parse_reg_num` accepts `d`/`s`/`q`/`v`/`h`/`b` prefixes and returns the numeric suffix. A floating-point dest/source/acc is silently treated as the same-numbered GPR.
**Function:** encode_umaddl
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `[Reg("d0"), Reg("w1"), Reg("w2"), Reg("x3")]` (which=0, prefix="d", n=0). Any of the four slots with a d/s/q/v/h/b name is accepted.
**Expected:** `Err`
**Actual:** `Ok(Word(0x9ba20c20))` with that slot encoded as the matching GPR number
**Severity:** medium
**Regression test:** `src/backend/arm/assembler/encoder/data_processing.rs` `test_encode_umaddl_regression_fp`
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1` / `--test-threads=1`
