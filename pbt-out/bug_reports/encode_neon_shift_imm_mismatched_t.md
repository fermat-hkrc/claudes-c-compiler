# Bug: encode_neon_shift_imm discards source arrangement
**Law:** ARM Advanced SIMD USHR is `Vd.<T>, Vn.<T>, #shift` with matching T. Mismatched dest/source arrangements must be rejected, matching llvm-mc / GNU as.
**Impact:** `ushr v0.8b, v0.16b, #1` (and any Td≠Ts pair) encodes using only dest T (Q and esize), producing a well-formed but wrong instruction instead of an assembler error.
**Function:** encode_neon_shift_imm
**Detected by:** Negative/Error Contract (4e) — encode_neon_shift_imm_neg_mismatched_t
**Minimal input:** `encode_neon_shift_imm([v0.8b, v0.16b, #1], true)` corresponding to `ushr v0.8b, v0.16b, #1`
**Expected:** `Err` (llvm-mc: invalid operand for instruction)
**Actual:** `Ok(EncodeResult::Word(_))` — `let (rn, _) = get_neon_reg(operands, 1)` drops source T (`neon.rs:379`)
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::encode_neon_shift_imm_pbt::test_encode_neon_shift_imm_regression_mismatched_t
**Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1` (not a test-isolation defect)
