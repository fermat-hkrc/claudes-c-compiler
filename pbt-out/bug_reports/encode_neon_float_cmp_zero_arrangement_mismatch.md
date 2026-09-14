# Bug: encode_neon_float_cmp_zero ignores source arrangement
**Law:** ARM ARM FCMEQ/FCMGE/FCMGT/FCMLE/FCMLT-to-zero require the same arrangement T on dest and source (`<Vd>.<T>, <Vn>.<T>, #0.0`). Mismatched T must be rejected with Err.
**Impact:** GNU-incompatible assembly such as `fcmeq v0.4s, v0.2s, #0.0` is silently encoded as the dest-T form (`fcmeq v0.4s, v0.4s, #0.0`), so the built-in assembler accepts text that gas/llvm-mc reject and emits a different instruction than written.
**Function:** encode_neon_float_cmp_zero
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** rd=0, rn=0, td="4s", tn="2s", insn=(0, 0b01101, "fcmeq") — `fcmeq v0.4s, v0.2s, #0.0` (operands `[RegArrangement{v0,"4s"}, RegArrangement{v0,"2s"}]`, u=0, size_hi=1, opcode=0b01101)
**Expected:** Err
**Actual:** Ok(Word(...)) — Q/sz taken only from dest arrangement (`let (rn, _) = get_neon_reg(operands, 1)`); src T is discarded
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::encode_neon_float_cmp_zero_pbt::test_encode_neon_float_cmp_zero_regression_arrangement_mismatch
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_neon_float_cmp_zero_neg -- --test-threads=1`
