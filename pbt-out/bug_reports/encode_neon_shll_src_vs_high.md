# Bug: encode_neon_shll does not require Tb to match the 2-suffix / Q
**Law:** SSHLL (Q=0) source Tb is {8B,4H,2S}; SSHLL2 (Q=1) source Tb is {16B,8H,4S}. A 2-suffix / Tb mismatch must be rejected.
**Impact:** `sshll2 v0.8h, v0.8b, #0` and `sshll v0.8h, v0.16b, #0` are invalid AArch64 (llvm-mc: invalid operand) but Q is taken only from the `is_high` parameter and 8b/16b share the same esize=8. The helper encodes a word whose Q does not match the source arrangement.
**Function:** encode_neon_shll
**Detected by:** Algebraic — Negative/Error Contract (4e); llvm-mc differential on the invalid domain
**Minimal input:** rd=0, rn=0, tb=8b, shift=0, u_bit=0, is_high=true (`sshll2 v0.8h, v0.8b, #0`)
**Expected:** Err (llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word) with Q=1 and 8-bit immh
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::test_encode_neon_shll_regression_src_vs_high
**Serial reconfirm:** PBT_TEST_JOBS=1 `cargo test --lib encode_neon_shll_neg_src_vs -- --test-threads=1` reproduced the failure.

Doc evidence for the law:
- ARM ARM SSHLL vs SSHLL2 Tb sets
- llvm-mc rejects `sshll v0.8h, v1.16b, #0` and `sshll2 v0.8h, v1.8b, #0`
