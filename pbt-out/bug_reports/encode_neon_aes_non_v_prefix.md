# Bug: encode_neon_aes accepts non-V register prefixes
**Law:** ARM ARM Cryptographic AES operands are SIMD registers `Vd`/`Vn` (V0–V31). GPR/FP prefixes (x/w/d/s/q/h/b) must be rejected with Err.
**Impact:** `aese x0.16b, x0.16b` is encoded as AESE V0, V0. llvm-mc/gas reject the instruction; the assembler treats a GPR name as a NEON register.
**Function:** encode_neon_aes
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** rd=0, rn=0, opc=4, prefix="x" — `aese x0.16b, x0.16b`
**Expected:** Err
**Actual:** Ok(Word(0x4e284800)) — `parse_reg_num` accepts prefix `x|w|d|s|q|v|h|b` and returns the numeric suffix
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::encode_neon_aes_pbt::test_encode_neon_aes_regression_x_prefix
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_neon_aes -- --test-threads=1`
