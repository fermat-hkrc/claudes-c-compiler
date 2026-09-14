# Bug: encode_mvn accepts NEON arrangements other than 8b/16b
**Law:** ARM ARM Advanced SIMD NOT/MVN allows T in {8B,16B} only. Other arrangements are invalid.
**Impact:** `mvn v0.4h, v0.4h` is accepted; encode_neon_not sets Q from dest=="16b" only, so 4h encodes as the 8b (Q=0) form. llvm-mc/gas reject it.
**Function:** encode_mvn
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `[RegArrangement{v0, 4h}, RegArrangement{v0, 4h}]`
**Expected:** Err
**Actual:** Ok(Word) with Q=0 (8b encoding)
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::test_encode_mvn_regression_neon_t
**Serial reconfirm:** PBT_TEST_JOBS=1, `--test-threads=1` — reproduced
