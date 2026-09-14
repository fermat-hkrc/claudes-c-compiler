# Bug: encode_neon_aes accepts a bare Vn without arrangement
**Law:** GNU-style AES requires `Vn.16B`. A source register without an arrangement specifier must be rejected with Err.
**Impact:** `aese v0.16b, v0` (Operand::Reg) is encoded as AESE V0, V0. llvm-mc/gas reject the instruction.
**Function:** encode_neon_aes
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** rd=0, rn=0, opc=4 — operands `[RegArrangement{v0,"16b"}, Reg("v0")]`
**Expected:** Err
**Actual:** Ok(Word(0x4e284800)) — `get_neon_reg` accepts `Operand::Reg` and returns an empty arrangement, which encode_neon_aes ignores
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::encode_neon_aes_pbt::test_encode_neon_aes_regression_bare_src
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_neon_aes -- --test-threads=1`
