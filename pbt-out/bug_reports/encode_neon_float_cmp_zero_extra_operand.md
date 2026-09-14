# Bug: encode_neon_float_cmp_zero ignores extra operands
**Law:** Vector FCMEQ/FCMGE/FCMGT/FCMLE/FCMLT-to-zero take `Vd.T, Vn.T, #0.0` only. A fourth operand must be rejected with Err.
**Impact:** GNU-incompatible assembly such as `fcmeq v0.2s, v0.2s, #0.0, v0.2s` is silently encoded as the compare-to-zero form, so the built-in assembler accepts text that gas/llvm-mc reject.
**Function:** encode_neon_float_cmp_zero
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** rd=0, rn=0, extra=0, t="2s", insn=(0, 0b01101, "fcmeq"), extra_kind=0 — `fcmeq v0.2s, v0.2s, #0.0, v0.2s` (operands `[RegArrangement{v0,"2s"}, RegArrangement{v0,"2s"}, Imm(0), RegArrangement{v0,"2s"}]`, u=0, size_hi=1, opcode=0b01101)
**Expected:** Err
**Actual:** Ok(Word(...)) — only operands[0] and operands[1] are read; there is no arity check (unlike integer sibling encode_neon_cmp_zero which at least rejects len<2)
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::encode_neon_float_cmp_zero_pbt::test_encode_neon_float_cmp_zero_regression_extra_operand
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_neon_float_cmp_zero_neg -- --test-threads=1`
