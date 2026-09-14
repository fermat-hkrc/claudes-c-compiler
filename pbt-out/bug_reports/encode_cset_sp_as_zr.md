# Bug: encode_cset encodes SP as XZR
**Law:** CSET register 31 is XZR/WZR, never SP/WSP; encode_cset with SP or WSP as Rd must be Err.
**Impact:** `cset sp, eq` is encoded as `cset xzr, eq` (0x9a9f17ff). Callers that accidentally pass SP get a silent ZR set instead of an assembler error. llvm-mc rejects SP/WSP for CSET.
**Function:** encode_cset
**Detected by:** Negative/Error Contract
**Minimal input:** [Reg("sp"), Cond("eq")]  (`cset sp, eq`)
**Expected:** Err
**Actual:** Ok(Word(0x9a9f17ff)) — parse_reg_num maps "sp"/"wsp" to 31, the same encoding as XZR/WZR.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_cset_pbt::test_encode_cset_regression_sp
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib encode_cset_pbt -- --test-threads=1 reproduced encode_cset_neg_wrong_reg with kind=0 n=0.
