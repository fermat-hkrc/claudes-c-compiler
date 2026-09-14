# Bug: encode_neon_aes ignores source arrangement
**Law:** ARM ARM / GNU AES requires matching `.16B` on `Vd` and `Vn`. Mismatched arrangements must be rejected with Err.
**Impact:** `aese v0.16b, v0.8b` is encoded using only register numbers. llvm-mc/gas reject the instruction; the assembler emits a 32-bit word for illegal SIMD syntax.
**Function:** encode_neon_aes
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** rd=0, rn=0, tn="8b", opc=4 — `aese v0.16b, v0.8b`
**Expected:** Err
**Actual:** Ok(Word(0x4e284800)) — both arrangements are discarded (`get_neon_reg` result `_`)
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::encode_neon_aes_pbt::test_encode_neon_aes_regression_mismatch_arr
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_neon_aes -- --test-threads=1`
