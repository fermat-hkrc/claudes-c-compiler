# Bug: encode_negs encodes SP/WSP as XZR/WZR
**Law:** ARM ARM Add/subtract (shifted register) register 31 is XZR/WZR, never SP/WSP. NEGS does not have an extended-register form. SP/WSP in either slot must be rejected.
**Impact:** `negs wsp, w0` is accepted and encoded as `negs wzr, w0`. llvm-mc/gas reject SP as a NEGS operand.
**Function:** encode_negs
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `[Reg("wsp"), Reg("w0")]` (which=0, is_64=false, other=0)
**Expected:** Err
**Actual:** Ok(Word) with Rd=31 (WZR)
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::test_encode_negs_regression_sp
**Serial reconfirm:** PBT_TEST_JOBS=1, `--test-threads=1` — reproduced
