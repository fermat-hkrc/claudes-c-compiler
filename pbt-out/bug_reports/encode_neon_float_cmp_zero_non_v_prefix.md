# Bug: encode_neon_float_cmp_zero accepts non-V register prefixes
**Law:** Vector FCMEQ/FCMGE/FCMGT/FCMLE/FCMLT-to-zero take V registers (`<Vd>.<T>, <Vn>.<T>, #0.0`). A GPR/scalar/element prefix (x/w/d/s/q/h/b) must be rejected with Err.
**Impact:** GNU-incompatible assembly such as `fcmeq x0.2s, v0.2s, #0.0` is silently encoded as `fcmeq v0.2s, v0.2s, #0.0`, so the built-in assembler accepts text that gas/llvm-mc reject.
**Function:** encode_neon_float_cmp_zero
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** rd=0, rn=0, t="2s", insn=(0, 0b01101, "fcmeq"), prefix="x", which=0 — `fcmeq x0.2s, v0.2s, #0.0` (operands `[RegArrangement{"x0","2s"}, RegArrangement{v0,"2s"}]`, u=0, size_hi=1, opcode=0b01101)
**Expected:** Err
**Actual:** Ok(Word(...)) — `parse_reg_num` maps x/w/d/s/q/v/h/b prefixes with num<=31 to the same 5-bit register number; no V-prefix check
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::encode_neon_float_cmp_zero_pbt::test_encode_neon_float_cmp_zero_regression_non_v_prefix
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_neon_float_cmp_zero_neg_non_v -- --test-threads=1`
