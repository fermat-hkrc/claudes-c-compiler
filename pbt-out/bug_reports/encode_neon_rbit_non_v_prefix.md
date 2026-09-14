# Bug: encode_neon_rbit encodes GPR/FP prefixes as V registers
**Law:** Vector RBIT takes SIMD `V` registers (`v0`–`v31`). Names with `x`/`w`/`d`/`s`/`q`/`h`/`b` prefixes are not NEON vector operands and must be rejected with Err.
**Impact:** Parser `is_register` accepts `x0.8b` as `RegArrangement`, the rbit dispatcher forwards it to encode_neon_rbit, and `parse_reg_num` maps `x0` to register 0. `rbit x0.8b, x0.8b` encodes as `rbit v0.8b, v0.8b`. llvm-mc/gas reject the text.
**Function:** encode_neon_rbit
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** rd=0, rn=0, t="8b", prefix="x" — `rbit x0.8b, x0.8b`
**Expected:** Err
**Actual:** Ok(Word(0x2e605800)) — `parse_reg_num` accepts prefixes `x|w|d|s|q|v|h|b`
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::encode_neon_rbit_pbt::test_encode_neon_rbit_regression_x_prefix
**Serial reconfirmation:** reproduced with `cargo test --lib encode_neon_rbit_pbt -- --test-threads=1`
