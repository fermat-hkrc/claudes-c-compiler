# Bug: encode_neon_aes ignores extra operands
**Law:** GNU-style AES (AESE/AESD/AESMC/AESIMC) takes exactly two operands (`Vd.16B`, `Vn.16B`). A third operand must be rejected with Err.
**Impact:** GNU-incompatible assembly such as `aese v0.16b, v0.16b, v0.16b` is silently encoded as AESE, so the built-in assembler accepts text that gas/llvm-mc reject.
**Function:** encode_neon_aes
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** rd=0, rn=0, extra=0, opc=4, extra_kind=0 — `aese v0.16b, v0.16b, v0.16b` (operands `[RegArrangement{v0,"16b"}, RegArrangement{v0,"16b"}, RegArrangement{v0,"16b"}]`)
**Expected:** Err
**Actual:** Ok(Word(0x4e284800)) — arity check is `operands.len() < 2`, so extra operands beyond index 1 are ignored
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::encode_neon_aes_pbt::test_encode_neon_aes_regression_extra_operand
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_neon_aes -- --test-threads=1`
