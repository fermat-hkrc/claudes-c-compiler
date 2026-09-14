# Bug: encode_neon_float_two_misc encodes SP/WSP/XZR/WZR/LR as a SIMD register
**Law:** Vector FP two-misc operands are V registers. SP/WSP/XZR/WZR/LR are GP aliases and must be rejected.
**Impact:** `fneg sp.2s, v0.2s` encodes as `fneg v31.2s, v0.2s` because `parse_reg_num` maps `sp`/`wsp`/`xzr`/`wzr` to 31 and `lr` to 30. The parser accepts `sp.2s` (`is_register("sp")`).
**Function:** encode_neon_float_two_misc
**Detected by:** Negative/Error Contract (5)
**Minimal input:** `[RegArrangement(sp, 2s), RegArrangement(v0, 2s)]` with (U,size_hi,opcode)=(1,1,0b01111)
**Expected:** Err (llvm-mc rejects `fneg sp.2s, v0.2s`)
**Actual:** Ok(Word) with Rd=31
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs `test_encode_neon_float_two_misc_regression_sp`
**Serial reconfirm:** reproduces with the same deterministic input as extra/mismatch/non-v/bare-src (PBT_TEST_JOBS=1)
