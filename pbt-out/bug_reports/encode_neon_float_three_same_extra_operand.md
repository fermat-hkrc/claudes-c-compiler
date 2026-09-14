# Bug: encode_neon_float_three_same ignores extra operands
**Law:** Vector FP three-same (FADD/FSUB/FMUL/...) takes exactly three operands; a fourth must be rejected, matching llvm-mc/gas.
**Impact:** Invalid assembly with a trailing operand is silently encoded as the three-register form, so the assembler accepts text GCC's gas would reject and emits a 4-byte word as if the extra operand were absent.
**Function:** encode_neon_float_three_same
**Detected by:** Negative/Error Contract
**Minimal input:** `fadd v0.2s, v0.2s, v0.2s, v0.2s` — operands `[RegArrangement(v0,2s) × 4]`, U=0, size_hi=0, opcode=0b11010
**Expected:** Err (llvm-mc: extra operand)
**Actual:** Ok(Word) — same encoding as `fadd v0.2s, v0.2s, v0.2s` (0x0e20d400). Serial reconfirm: `PBT_TEST_JOBS=1` reproduced.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::encode_neon_float_three_same_pbt::test_encode_neon_float_three_same_regression_extra_operand
