# Bug: encode_neon_shll ignores destination arrangement
**Law:** SSHLL dest Ta is determined by Q and source Tb (8B→8H, 16B→8H, 4H→4S, 8H→4S, 2S→2D, 4S→2D). A mismatched dest arrangement must be rejected.
**Impact:** `sshll v0.8b, v0.8b, #0` is invalid AArch64 (llvm-mc: invalid operand) but the helper discards dest arrangement (`let (rd, _) = get_neon_reg(operands, 0)`) and encodes as if Ta were 8H. Wrong-arrangement assembly is silently accepted.
**Function:** encode_neon_shll
**Detected by:** Algebraic — Negative/Error Contract (4e); llvm-mc differential on the invalid domain
**Minimal input:** rd=0, rn=0, tb=8b, ta=8b, shift=0, u_bit=0, is_high=false (`sshll v0.8b, v0.8b, #0`)
**Expected:** Err (llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word) — dest arrangement is unused
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::test_encode_neon_shll_regression_mismatched_dest_ta
**Serial reconfirm:** PBT_TEST_JOBS=1 `cargo test --lib encode_neon_shll_neg -- --test-threads=1` reproduced the failure.

Doc evidence for the law:
- ARM ARM SSHLL: Ta encoded in Q:immh; 8B source requires dest 8H
- llvm-mc rejects `sshll v0.8b, v1.8b, #0`
