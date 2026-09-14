# Bug: encode_neon_aes encodes SP as V31
**Law:** ARM ARM Cryptographic AES operands are SIMD registers `Vd`/`Vn`. SP is not a valid AES operand and must be rejected with Err.
**Impact:** `aese sp.16b, v0.16b` is encoded as AESE V31, V0. llvm-mc/gas reject the instruction.
**Function:** encode_neon_aes
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** dest="sp", rn=0, opc=4 — `aese sp.16b, v0.16b`. Sweep also: dest="wsp" — same mapping to 31.
**Expected:** Err
**Actual:** Ok(Word) with Rd=31 — `parse_reg_num` maps `"sp"`/`"wsp"` to 31
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::encode_neon_aes_pbt::test_encode_neon_aes_regression_sp
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_neon_aes -- --test-threads=1`
