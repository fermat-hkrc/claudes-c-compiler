# Bug: encode_neon_float_two_misc accepts non-V register prefixes
**Law:** Vector FP two-misc operands are SIMD V registers (v0–v31). GPR/scalar prefixes (x/w/d/s/q/h/b) must be rejected.
**Impact:** `fneg x0.2s, v0.2s` encodes as `fneg v0.2s, v0.2s` because `parse_reg_num` strips any of x/w/d/s/q/v/h/b. Wrong-bank typos assemble silently.
**Function:** encode_neon_float_two_misc
**Detected by:** Negative/Error Contract (5)
**Minimal input:** `[RegArrangement(x0, 2s), RegArrangement(v0, 2s)]` with (U,size_hi,opcode)=(1,1,0b01111)
**Expected:** Err (llvm-mc rejects `fneg x0.2s, v0.2s`)
**Actual:** Ok(Word) encoding Rd=0 (same as v0)
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs `test_encode_neon_float_two_misc_regression_non_v_prefix`
**Serial reconfirm:** PBT_TEST_JOBS=1 — reproduces serially
