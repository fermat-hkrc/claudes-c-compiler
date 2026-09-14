# Bug: encode_neon_across_long ignores extra operands
**Law:** SADDLV/UADDLV take exactly two operands (`<V><d>, <Vn>.<T>`). A third operand must be rejected with Err.
**Impact:** GNU-incompatible assembly such as `saddlv h0, v0.8b, h0` is silently encoded as the two-operand form, so the built-in assembler accepts text that gas/llvm-mc reject.
**Function:** encode_neon_across_long
**Detected by:** Negative/Error Contract (5)
**Minimal input:** rd=0, rn=0, extra=0, u=0, t="8b", extra_kind=0 — `saddlv h0, v0.8b, h0` (operands `[Reg("h0"), RegArrangement{v0, "8b"}, Reg("h0")]`, u=0, opcode=0b00011)
**Expected:** Err
**Actual:** Ok(Word(...)) — only operands[0] and operands[1] are read; `if operands.len() < 2` does not reject len>2
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::encode_neon_across_long_pbt::test_encode_neon_across_long_regression_extra_operand
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_neon_across_long_neg -- --test-threads=1`
