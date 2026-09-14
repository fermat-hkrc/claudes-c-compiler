# Bug: encode_csetm encodes SP as XZR
**Law:** CSETM register 31 is XZR/WZR, never SP/WSP; encode_csetm with SP or WSP as Rd must be Err.
**Impact:** `csetm sp, eq` is encoded as `csetm xzr, eq` (0xda9f13ff). Callers that accidentally pass SP get a silent ZR set-mask instead of an assembler error. llvm-mc rejects SP/WSP for CSETM.
**Function:** encode_csetm
**Detected by:** Negative/Error Contract
**Minimal input:** [Reg("sp"), Cond("eq")]  (`csetm sp, eq`)
**Expected:** Err
**Actual:** Ok(Word(0xda9f13ff)) — parse_reg_num maps "sp"/"wsp" to 31, the same encoding as XZR/WZR.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_csetm_pbt::test_encode_csetm_regression_sp
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib encode_csetm_neg -- --test-threads=1 reproduced encode_csetm_neg_wrong_reg with kind=0 n=0.
