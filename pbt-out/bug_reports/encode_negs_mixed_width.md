# Bug: encode_negs accepts mixed X/W register widths
**Law:** ARM ARM NEGS requires Rd and Rm to have the same width (both X or both W). Mixed-width pairs must be rejected.
**Impact:** `negs w0, x0` is accepted; sf is taken only from Rd, so the 64-bit Rm is encoded in a 32-bit instruction. llvm-mc/gas reject mixed-width NEGS.
**Function:** encode_negs
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `[Reg("w0"), Reg("x0")]` (rd64=false, rm64=true, rd=0, rm=0)
**Expected:** Err
**Actual:** Ok(Word) with sf from Rd only
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::test_encode_negs_regression_mixed_width
**Serial reconfirm:** PBT_TEST_JOBS=1, `--test-threads=1` — reproduced
