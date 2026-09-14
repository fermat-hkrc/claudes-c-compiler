# Bug: encode_neon_float_two_misc ignores source arrangement
**Law:** ARM Advanced SIMD two-register miscellaneous FP requires dest T = src T (`Vd.<T>, Vn.<T>`). Mismatched arrangements must be rejected.
**Impact:** `fneg v0.2s, v0.4s` encodes as 2S (Q from dest) instead of error, producing the wrong element width.
**Function:** encode_neon_float_two_misc
**Detected by:** Negative/Error Contract (5)
**Minimal input:** `[RegArrangement(v0, 2s), RegArrangement(v0, 4s)]` with (U,size_hi,opcode)=(1,1,0b01111)
**Expected:** Err (llvm-mc rejects `fneg v0.2s, v0.4s`)
**Actual:** Ok(Word) — source arrangement discarded (`let (rn, _) = get_neon_reg(operands, 1)`)
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs `test_encode_neon_float_two_misc_regression_arrangement_mismatch`
**Serial reconfirm:** PBT_TEST_JOBS=1 — reproduces serially
