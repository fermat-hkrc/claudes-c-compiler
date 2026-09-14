# Bug: encode_neon_qshrn ignores destination arrangement Tb
**Law:** Vector SQSHRN requires Vd.Tb to match Ta and the `2` suffix: 8H→8B (Q=0) / 16B (Q=1); 4S→4H / 8H; 2D→2S / 4S. A mismatched dest Tb must be rejected.
**Impact:** `sqshrn v0.4h, v0.8h, #1` is invalid AArch64 (llvm-mc: invalid operand) but the helper encodes it as if dest were v0.8b, using only Rd's register number. Wrong-arrangement assembly is silently accepted.
**Function:** encode_neon_qshrn
**Detected by:** Algebraic — Negative/Error Contract (4e); llvm-mc differential on the invalid domain
**Minimal input:** rd=0, rn=0, ta=8h, tb=4h, shift=1, u_bit=0, is_rounding=false, is_high=false (`sqshrn v0.4h, v0.8h, #1`)
**Expected:** Err (llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word) — `let (rd, _) = get_neon_reg(operands, 0)` discards dest arrangement
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::test_encode_neon_qshrn_regression_mismatched_dest_tb
**Serial reconfirm:** PBT_TEST_JOBS=1 `cargo test --lib encode_neon_qshrn_neg -- --test-threads=1` reproduced the failure.

Doc evidence for the law:
- README.md:5-14 gas-compatible GNU-style assembly
- ARM ARM SQSHRN Tb/Ta pairs
- llvm-mc rejects `sqshrn v0.4h, v0.8h, #1`
- Dispatch encoder/mod.rs:637-648: this helper owns vector sqshrn encoding
