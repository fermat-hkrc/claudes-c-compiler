# Bug: encode_neon_qshrn ignores extra operands beyond index 2
**Law:** SQSHRN takes exactly three operands (Vd.Tb, Vn.Ta, #shift). A fourth operand must be rejected.
**Impact:** `sqshrn v0.8b, v0.8h, #1, v0.8b` is invalid AArch64 (llvm-mc: invalid operand) but the helper encodes the first three operands and drops the rest. Extra-operand assembly is silently accepted.
**Function:** encode_neon_qshrn
**Detected by:** Algebraic — Negative/Error Contract (4e); llvm-mc differential on the invalid domain
**Minimal input:** rd=0, rn=0, extra=0, ta=8h, tb=8b, shift=1, u_bit=0, is_rounding=false, is_high=false (`sqshrn v0.8b, v0.8h, #1, v0.8b`)
**Expected:** Err (llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word) — body only checks `operands.len() < 3`
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::test_encode_neon_qshrn_regression_extra_operand
**Serial reconfirm:** PBT_TEST_JOBS=1 `cargo test --lib encode_neon_qshrn_neg -- --test-threads=1` reproduced the failure.

Doc evidence for the law:
- README.md:5-14 gas-compatible GNU-style assembly
- ARM ARM SQSHRN syntax: Vd.Tb, Vn.Ta, #shift
- llvm-mc rejects the 4-operand form
