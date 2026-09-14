# Bug: encode_neon_shift_left_imm ignores source arrangement
**Law:** SQSHL/UQSHL immediate requires matching arrangements `Vd.T, Vn.T`. Mismatched T (e.g. `.8b` dest with `.16b` source) must be rejected.
**Impact:** The encoder reads dest T only (`let (rn, _) = get_neon_reg(operands, 1)?`) and emits a word as if both operands shared dest T. llvm-mc rejects `sqshl v0.8b, v0.16b, #0` (`invalid operand for instruction`). Callers can assemble illegal SIMD ops that a real assembler would refuse.
**Function:** encode_neon_shift_left_imm
**Detected by:** Negative/Error Contract
**Minimal input:** `[RegArrangement(v0, 8b), RegArrangement(v0, 16b), Imm(0)]` with `u=0`, `opcode=0b01110` (asm `sqshl v0.8b, v0.16b, #0`)
**Expected:** `Err(...)`
**Actual:** `Ok(Word(...))` encoded as `.8b` SQSHL (source arrangement discarded)
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::encode_neon_shift_left_imm_pbt::test_encode_neon_shift_left_imm_regression_arrangement_mismatch
**Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1` / `--test-threads=1`
