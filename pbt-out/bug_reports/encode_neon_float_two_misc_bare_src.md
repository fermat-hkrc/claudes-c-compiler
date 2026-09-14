# Bug: encode_neon_float_two_misc accepts a bare source register
**Law:** Vector FP two-misc requires a source with an arrangement specifier (`Vn.<T>`). A bare `Vn` must be rejected.
**Impact:** Operands equivalent to `fneg v0.2s, v0` encode instead of error; llvm-mc rejects that form as too few / invalid operands.
**Function:** encode_neon_float_two_misc
**Detected by:** Negative/Error Contract (5)
**Minimal input:** `[RegArrangement(v0, 2s), Reg(v0)]` with (U,size_hi,opcode)=(1,1,0b01111)
**Expected:** Err (llvm-mc rejects `fneg v0.2s, v0`)
**Actual:** Ok(Word) — `get_neon_reg` accepts `Operand::Reg` and the function ignores source arrangement
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs `test_encode_neon_float_two_misc_regression_bare_src`
**Serial reconfirm:** PBT_TEST_JOBS=1 — reproduces serially
