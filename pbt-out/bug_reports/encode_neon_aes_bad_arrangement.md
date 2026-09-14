# Bug: encode_neon_aes ignores arrangement (accepts T other than .16B)
**Law:** ARM ARM Cryptographic AES admits only `Vd.16B, Vn.16B`. Any other arrangement (`.8b`, `.4s`, `.8h`, `.2d`, …) must be rejected with Err.
**Impact:** `aese v0.8b, v0.8b` is encoded as the 16B AES instruction. llvm-mc/gas reject the instruction; the assembler emits a 32-bit word for illegal SIMD syntax.
**Function:** encode_neon_aes
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** rd=0, rn=0, t="8b", opc=4 — `aese v0.8b, v0.8b`
**Expected:** Err
**Actual:** Ok(Word(0x4e284800)) — `let (rd, _) = get_neon_reg(...)` / `let (rn, _) = get_neon_reg(...)` discard arrangement entirely
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::encode_neon_aes_pbt::test_encode_neon_aes_regression_8b_arrangement
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_neon_aes -- --test-threads=1`
