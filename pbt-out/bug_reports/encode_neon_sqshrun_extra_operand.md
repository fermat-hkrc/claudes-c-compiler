# Bug: SQSHRUN ignores operands beyond index 2
**Law:** GNU-style `sqshrun`/`sqrshrun`(+2) takes exactly 3 operands (Vd.Tb, Vn.Ta, #shift). A 4th operand must be rejected.
**Impact:** `sqshrun v0.8b, v0.8h, #1, v0.8b` is encoded as the 3-operand form; trailing operands are dropped, so a typo or extra operand does not fail the assemble.
**Function:** encode_neon_sqshrun
**Detected by:** Negative/Error Contract (5) — llvm-mc rejects the 4-operand form; README.md:1-14 gas-compatible GNU-style text
**Minimal input:** `[RegArrangement(v0, 8b), RegArrangement(v0, 8h), Imm(1), RegArrangement(v0, 8b)]`, is_rounding=false, is_high=false
**Expected:** Err (llvm-mc: invalid operand)
**Actual:** Ok(Word). Body only checks `operands.len() < 3` and never requires exact arity 3.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::test_encode_neon_sqshrun_regression_extra_operand
**Serial reconfirmation:** reproduced with PBT_TEST_JOBS=1 (cargo test --lib encode_neon_sqshrun -- --test-threads=1)
