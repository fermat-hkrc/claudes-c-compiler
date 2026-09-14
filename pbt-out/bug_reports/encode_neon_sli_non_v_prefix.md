# Bug: encode_neon_sli accepts non-V register prefixes as V registers
**Law:** Vector SLI operands are V registers (`v0`–`v31`) with an arrangement. GPR/scalar prefixes (x/w/d/s/q/h/b) must be rejected with Err.
**Impact:** GNU-incompatible assembly such as `sli x0.8b, v0.8b, #0` is encoded as `sli v0.8b, v0.8b, #0` because `parse_reg_num` accepts x/w/d/s/q/v/h/b prefixes. gas/llvm-mc reject the non-V form.
**Function:** encode_neon_sli
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** rd=0, rn=0, t="8b", shift=0, prefix="x", which=0 — `sli x0.8b, v0.8b, #0` (operands `[RegArrangement{x0,"8b"}, RegArrangement{v0,"8b"}, Imm(0)]`)
**Expected:** Err
**Actual:** Ok(Word(...)) — `parse_reg_num` maps `x0` to register 0, same as `v0`
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::encode_neon_sli_pbt::test_encode_neon_sli_regression_non_v_prefix
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_neon_sli_neg -- --test-threads=1`
