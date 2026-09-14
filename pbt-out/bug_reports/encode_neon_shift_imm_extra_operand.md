# Bug: encode_neon_shift_imm ignores operands after index 2
**Law:** Vector USHR takes exactly three operands (`Vd.T, Vn.T, #shift`). A fourth operand must be rejected, matching llvm-mc / GNU as.
**Impact:** Trailing garbage after a well-formed USHR is assembled into a 32-bit word instead of an error, so invalid assembly is silently accepted.
**Function:** encode_neon_shift_imm
**Detected by:** Negative/Error Contract (4e) — encode_neon_shift_imm_neg_extra_operand
**Minimal input:** `encode_neon_shift_imm([v0.8b, v0.8b, #1, v0.8b], true)` corresponding to `ushr v0.8b, v0.8b, #1, v0.8b`
**Expected:** `Err` (llvm-mc: extra operand is invalid)
**Actual:** `Ok(EncodeResult::Word(_))` — `operands.len() < 3` is the only arity check (`neon.rs:374`)
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::encode_neon_shift_imm_pbt::test_encode_neon_shift_imm_regression_extra_operand
**Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1` (not a test-isolation defect)
