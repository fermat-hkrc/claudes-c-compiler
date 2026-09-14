# Bug: encode_mul ignores source NEON arrangements
**Law:** ARM ARM Advanced SIMD MUL requires all three operands to share arrangement T. Mismatched T must be rejected (gas/llvm-mc "invalid operand").
**Impact:** `mul v0.8b, v0.8b, v0.16b` is encoded using only the destination arrangement (8b, Q=0, size=00). The source T is discarded, so a width mismatch that is a hard assembler error in llvm-mc silently produces a valid-looking 8b MUL. Note: `encode_instruction` currently routes NEON MUL to `encode_neon_three_same` rather than `encode_mul`.
**Function:** encode_mul
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `encode_mul([RegArrangement{v0,8b}, RegArrangement{v0,8b}, RegArrangement{v0,16b}])` i.e. `mul v0.8b, v0.8b, v0.16b`
**Expected:** Err
**Actual:** Ok(Word) using dest T only. Reproduced serially (`PBT_TEST_JOBS=1`).
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::encode_mul_pbt::test_encode_mul_regression_neon_mismatch_t
