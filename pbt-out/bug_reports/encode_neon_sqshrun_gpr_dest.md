# Bug: SQSHRUN encodes a GPR/scalar-FP dest as a NEON Rd
**Law:** GNU-style `sqshrun` dest is Vd.Tb. A GPR or scalar FP register (`x0`, `w0`, `d0`, …) must be rejected.
**Impact:** `sqshrun x0, v0.8h, #1` encodes Rd=0 (the same encoding as `v0`), producing a vector instruction the source text did not request.
**Function:** encode_neon_sqshrun
**Detected by:** Negative/Error Contract (5) — llvm-mc rejects GPR dest; README.md:229 NEON narrow is Vd.Tb
**Minimal input:** `sqshrun x0, v0.8h, #1` — operands `[Reg("x0"), RegArrangement(v0, 8h), Imm(1)]`, is_rounding=false, is_high=false
**Expected:** Err (llvm-mc: invalid operand)
**Actual:** Ok(Word). `get_neon_reg` accepts `Operand::Reg` and `parse_reg_num("x0")` returns 0, so dest is encoded as v0.
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::test_encode_neon_sqshrun_regression_gpr_dest
**Serial reconfirmation:** reproduced with PBT_TEST_JOBS=1 (cargo test --lib encode_neon_sqshrun -- --test-threads=1)
