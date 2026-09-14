# Bug: encode_neon_dup ignores extra operands
**Law:** GNU-style DUP is a 2-operand instruction; a third operand must be rejected (gas/llvm-mc reject it).
**Impact:** Invalid assembly `dup Vd.T, Rn, extra` is silently encoded as the 2-operand form, producing a well-formed instruction the source did not request.
**Function:** encode_neon_dup
**Detected by:** Negative/Error Contract (5)
**Minimal input:** operands = [RegArrangement(v0, 8b), Reg(w0), Reg(w0)]  — `dup v0.8b, w0, w0`
**Expected:** Err (llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word) of `dup v0.8b, w0` (extra operand ignored). Serial reconfirmation: `PBT_TEST_JOBS=1 cargo test --lib encode_neon_dup_neg_extra -- --test-threads=1` still fails.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::test_encode_neon_dup_regression_extra_operand
