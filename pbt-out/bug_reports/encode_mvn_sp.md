# Bug: encode_mvn encodes SP/WSP as XZR/WZR
**Law:** Register 31 in Logical (shifted register) MVN is XZR/WZR, never SP/WSP. SP/WSP must be rejected.
**Impact:** `mvn wsp, w0` is accepted and encoded as `mvn wzr, w0`. llvm-mc/gas reject SP/WSP in either slot.
**Function:** encode_mvn
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `[Reg("wsp"), Reg("w0")]` (which=0, is_64=false, other=0)
**Expected:** Err
**Actual:** Ok(Word) with Rd=31 (WZR)
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::test_encode_mvn_regression_sp
**Serial reconfirm:** PBT_TEST_JOBS=1, `--test-threads=1` — reproduced
