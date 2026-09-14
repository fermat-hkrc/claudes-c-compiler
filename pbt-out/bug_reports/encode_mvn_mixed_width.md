# Bug: encode_mvn accepts mixed X/W register widths
**Law:** Logical (shifted register) MVN requires Rd and Rm to have the same width (both X or both W).
**Impact:** `mvn w0, x0` is accepted; sf is taken only from Rd, so a 32-bit encoding is emitted for a 64-bit source. llvm-mc/gas reject mixed width.
**Function:** encode_mvn
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `[Reg("w0"), Reg("x0")]` (rd64=false, rm64=true)
**Expected:** Err
**Actual:** Ok(Word) with sf from Rd only
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::test_encode_mvn_regression_mixed_width
**Serial reconfirm:** PBT_TEST_JOBS=1, `--test-threads=1` — reproduced
