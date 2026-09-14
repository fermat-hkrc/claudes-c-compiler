# Bug: encode_neon_shift_left_imm accepts a source without arrangement
**Law:** SQSHL/UQSHL immediate requires `Vn.T` with an arrangement specifier matching dest T. A bare `Vn` (Operand::Reg) must be rejected.
**Impact:** `get_neon_reg` accepts `Operand::Reg` and returns an empty arrangement, which the encoder then discards. `sqshl v0.8b, v0, #0` encodes as if the source were `v0.8b`. llvm-mc rejects the missing arrangement. Dest-without-arrangement already Errs (empty T is unsupported); only the source path is open.
**Function:** encode_neon_shift_left_imm
**Detected by:** Negative/Error Contract (coverage sweep)
**Minimal input:** `[RegArrangement(v0, 8b), Reg(v0), Imm(0)]` with `u=0`, `opcode=0b01110` (asm `sqshl v0.8b, v0, #0`)
**Expected:** `Err(...)`
**Actual:** `Ok(Word(...))` encoded as `.8b` SQSHL
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::encode_neon_shift_left_imm_pbt::test_encode_neon_shift_left_imm_regression_src_reg_no_arrangement
**Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1` / `--test-threads=1`
