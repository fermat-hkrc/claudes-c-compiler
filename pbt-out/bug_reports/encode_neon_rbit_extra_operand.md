# Bug: encode_neon_rbit ignores extra operands
**Law:** GNU-style vector RBIT takes exactly two operands (`Vd.T`, `Vn.T`). A third operand must be rejected with Err.
**Impact:** GNU-incompatible assembly such as `rbit v0.8b, v0.8b, v0.8b` is silently encoded as RBIT, so the built-in assembler accepts text that gas/llvm-mc reject.
**Function:** encode_neon_rbit
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** rd=0, rn=0, extra=0, t="8b", extra_kind=0 — `rbit v0.8b, v0.8b, v0.8b` (operands `[RegArrangement{v0,"8b"}, RegArrangement{v0,"8b"}, RegArrangement{v0,"8b"}]`)
**Expected:** Err
**Actual:** Ok(Word(0x2e605800)) — arity check is `operands.len() < 2`, so extra operands beyond index 1 are ignored
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::encode_neon_rbit_pbt::test_encode_neon_rbit_regression_extra_operand
**Serial reconfirmation:** reproduced with `cargo test --lib encode_neon_rbit_pbt -- --test-threads=1`
