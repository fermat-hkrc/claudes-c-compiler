# Bug: encode_neon_shift_left_imm ignores extra operands
**Law:** GNU-style SQSHL/UQSHL immediate takes exactly 3 operands (`Vd.T, Vn.T, #shift`). A 4th operand must be rejected.
**Impact:** The assembler silently encodes a 3-operand instruction when given extra operands, producing a valid-looking word for invalid assembly. That disagrees with llvm-mc/gas (`invalid operand for instruction`) and can hide caller bugs.
**Function:** encode_neon_shift_left_imm
**Detected by:** Negative/Error Contract
**Minimal input:** `[RegArrangement(v0, 8b), RegArrangement(v0, 8b), Imm(0), RegArrangement(v0, 8b)]` with `u=0`, `opcode=0b01110` (asm `sqshl v0.8b, v0.8b, #0, v0.8b`)
**Expected:** `Err(...)`
**Actual:** `Ok(Word(0x0f087400))` — the 4th operand is ignored because the body only checks `operands.len() < 3`
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::encode_neon_shift_left_imm_pbt::test_encode_neon_shift_left_imm_regression_extra_operand
**Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1` / `--test-threads=1`
