# Bug: encode_neon_float_two_misc ignores extra operands
**Law:** Vector FP two-register misc (fneg/fabs/fsqrt/...) takes exactly two SIMD registers; a third operand must be rejected.
**Impact:** Assembler silently encodes `fneg v0.2s, v0.2s, v0.2s` instead of reporting an error, so extra-operand typos assemble to a valid instruction.
**Function:** encode_neon_float_two_misc
**Detected by:** Negative/Error Contract (5)
**Minimal input:** `[RegArrangement(v0, 2s), RegArrangement(v0, 2s), RegArrangement(v0, 2s)]` with (U,size_hi,opcode)=(1,1,0b01111)
**Expected:** Err (llvm-mc rejects `fneg v0.2s, v0.2s, v0.2s`)
**Actual:** Ok(Word) — extra operand ignored; no `operands.len()` check
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs `test_encode_neon_float_two_misc_regression_extra_operand`
**Serial reconfirm:** PBT_TEST_JOBS=1 `cargo test --lib test_encode_neon_float_two_misc_regression -- --test-threads=1` — 4 failed / 0 passed (reproduces serially)
