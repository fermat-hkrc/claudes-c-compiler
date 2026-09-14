# Bug: encode_smull encodes FP/SIMD register names as GPRs
**Law:** Scalar SMULL operands are GPRs (`Xd`, `Wn`, `Wm`). FP/SIMD names (`d`/`s`/`q`/`v`/`h`/`b` prefixes) must return Err. llvm-mc rejects `smull d0, w1, w2` (`invalid operand for instruction`).
**Impact:** `smull d0, w1, w2` is encoded as `smull x0, w1, w2` because `parse_reg_num` accepts `d`/`s`/`q`/`v`/`h`/`b` prefixes and returns the numeric suffix. A floating-point dest/source is silently treated as the same-numbered GPR.
**Function:** encode_smull
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `[Reg("d0"), Reg("w1"), Reg("w2")]` (which=0, prefix="d", n=0). Any of the three slots with a d/s/q/v/h/b name is accepted.
**Expected:** `Err`
**Actual:** `Ok(Word(...))` with that slot encoded as the matching GPR number
**Severity:** medium
**Regression test:** `src/backend/arm/assembler/encoder/data_processing.rs` `test_encode_smull_regression_fp`
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1` / `--test-threads=1`
