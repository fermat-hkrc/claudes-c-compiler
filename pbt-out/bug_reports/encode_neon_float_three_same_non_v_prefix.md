# Bug: encode_neon_float_three_same encodes non-V register prefixes as V registers
**Law:** Vector FP three-same operands must be V registers (`v0`–`v31`). llvm-mc rejects `fadd x0.2s, v0.2s, v0.2s` and other x/w/d/s/q/h/b prefixes.
**Impact:** `parse_reg_num` accepts x/w/d/s/q/v/h/b, so `x0.2s` is encoded as `v0.2s`. Invalid GNU-style text is assembled into a plausible NEON word instead of being rejected.
**Function:** encode_neon_float_three_same
**Detected by:** Negative/Error Contract
**Minimal input:** `fadd x0.2s, v0.2s, v0.2s` — dest `RegArrangement { reg: "x0", arrangement: "2s" }`, U=0, size_hi=0, opcode=0b11010
**Expected:** Err (V register required)
**Actual:** Ok(Word) identical to `fadd v0.2s, v0.2s, v0.2s`. Serial reconfirm: `PBT_TEST_JOBS=1` reproduced.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::encode_neon_float_three_same_pbt::test_encode_neon_float_three_same_regression_non_v_prefix
