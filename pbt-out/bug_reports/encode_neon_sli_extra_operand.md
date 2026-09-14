# Bug: encode_neon_sli ignores extra operands
**Law:** Vector SLI takes `Vd.T, Vn.T, #shift` only. A fourth operand must be rejected with Err.
**Impact:** GNU-incompatible assembly such as `sli v0.8b, v0.8b, #0, v0.8b` is silently encoded as SLI, so the built-in assembler accepts text that gas/llvm-mc reject.
**Function:** encode_neon_sli
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** rd=0, rn=0, extra=0, t="8b", shift=0, extra_kind=0 — `sli v0.8b, v0.8b, #0, v0.8b` (operands `[RegArrangement{v0,"8b"}, RegArrangement{v0,"8b"}, Imm(0), RegArrangement{v0,"8b"}]`)
**Expected:** Err
**Actual:** Ok(Word(...)) — arity check is `operands.len() < 3`, so extra operands beyond index 2 are ignored
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::encode_neon_sli_pbt::test_encode_neon_sli_regression_extra_operand
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_neon_sli_neg -- --test-threads=1`
