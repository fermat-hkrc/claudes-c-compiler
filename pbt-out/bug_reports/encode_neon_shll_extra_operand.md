# Bug: encode_neon_shll ignores extra operands beyond index 2
**Law:** SSHLL/USHLL takes exactly three operands (Vd.Ta, Vn.Tb, #shift). A fourth operand must be rejected.
**Impact:** `sshll v0.8h, v0.8b, #0, v0.8h` is invalid AArch64 (llvm-mc: invalid operand) but the helper encodes the first three operands and drops the rest. Extra-operand assembly is silently accepted.
**Function:** encode_neon_shll
**Detected by:** Algebraic — Negative/Error Contract (4e); llvm-mc differential on the invalid domain
**Minimal input:** rd=0, rn=0, extra=0, tb=8b, ta=8h, shift=0, u_bit=0, is_high=false (`sshll v0.8h, v0.8b, #0, v0.8h`)
**Expected:** Err (llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word) — body only checks `operands.len() < 3`
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::test_encode_neon_shll_regression_extra_operand
**Serial reconfirm:** PBT_TEST_JOBS=1 `cargo test --lib encode_neon_shll_neg -- --test-threads=1` reproduced the failure.

Doc evidence for the law:
- README.md:1-14 gas-compatible GNU-style assembly
- ARM ARM SSHLL syntax: Vd.Ta, Vn.Tb, #shift
- llvm-mc rejects the 4-operand form
