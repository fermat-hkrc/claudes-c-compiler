# Bug: encode_neon_dup masks out-of-range lane indices
**Law:** ARM ARM DUP (element) lane index ranges are b[0-15], h[0-7], s[0-3], d[0-1]. Indices outside that range must be rejected. llvm-mc reports "vector lane must be an integer in range [0, N]".
**Impact:** `dup v0.8b, v0.b[16]` is encoded as `dup v0.8b, v0.b[0]` because the SUT does `index & 0xF` (and `& 0x7` / `& 0x3` / `& 0x1` for h/s/d). A typo in the lane index silently selects a different lane.
**Function:** encode_neon_dup
**Detected by:** Negative/Error Contract (5)
**Minimal input:** operands = [RegArrangement(v0, 8b), RegLane(v0, b, 16)]  — `dup v0.8b, v0.b[16]`
**Expected:** Err (llvm-mc: vector lane must be an integer in range [0, 15])
**Actual:** Ok(Word) of `dup v0.8b, v0.b[0]` (index wrapped). Serial reconfirmation: `PBT_TEST_JOBS=1 cargo test --lib encode_neon_dup_neg_index -- --test-threads=1` still fails.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::test_encode_neon_dup_regression_index_oor
