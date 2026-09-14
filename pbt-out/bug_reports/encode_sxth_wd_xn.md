# Bug: encode_sxth accepts W dest with X source
**Law:** ARM ARM SXTH assembler syntax is `SXTH <Wd>, <Wn>` or `SXTH <Xd>, <Wn>`. A 32-bit dest with a 64-bit source (`sxth w0, x0`) is invalid; llvm-mc / GNU as reject it.
**Impact:** Mixed-width `sxth Wd, Xn` is encoded as 32-bit SBFM using only the dest width, so invalid assembly becomes a silent W-form SXTH.
**Function:** encode_sxth
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `[Reg("w0"), Reg("x0")]` (sxth w0, x0)
**Expected:** `Err(...)`
**Actual:** `Ok(Word)` — source `is_64` from `get_reg` is discarded; dest width alone sets sf/N
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs `test_encode_sxth_regression_wd_xn`
**Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_sxth_neg -- --test-threads=1`
