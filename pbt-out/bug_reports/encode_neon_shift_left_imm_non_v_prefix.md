# Bug: encode_neon_shift_left_imm accepts non-V register prefixes
**Law:** GNU-style SQSHL/UQSHL vector form requires V registers (`Vd.T, Vn.T, #shift`). GPR and scalar prefixes (x/w/d/s/q/h/b) must be rejected.
**Impact:** `parse_reg_num` accepts any of those prefixes and returns the number, so `sqshl x0.8b, v0.8b, #0` encodes as `v0.8b`. llvm-mc rejects it (`invalid operand for instruction`). Illegal assembly is assembled as a NEON instruction.
**Function:** encode_neon_shift_left_imm
**Detected by:** Negative/Error Contract (coverage sweep)
**Minimal input:** `[RegArrangement(x0, 8b), RegArrangement(v0, 8b), Imm(0)]` with `u=0`, `opcode=0b01110` (asm `sqshl x0.8b, v0.8b, #0`)
**Expected:** `Err(...)`
**Actual:** `Ok(Word(...))` encoded as SQSHL V0.8B
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::encode_neon_shift_left_imm_pbt::test_encode_neon_shift_left_imm_regression_non_v_prefix
**Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1` / `--test-threads=1`
