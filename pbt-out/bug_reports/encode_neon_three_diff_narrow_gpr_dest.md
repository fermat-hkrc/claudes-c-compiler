# Bug: encode_neon_three_diff_narrow accepts GPR/FP names as NEON Vd
**Law:** ADDHN destination is a NEON vector register with arrangement (Vd.Tb). GPR (x/w) and scalar FP (d/s/q/h/b) names must be rejected, matching llvm-mc / GNU as.
**Impact:** `addhn x0, v0.8h, v0.8h` is encoded as `addhn v0.8b, v0.8h, v0.8h` (`parse_reg_num("x0")` = 0; dest arrangement ignored so Q/size come from is_high and Rn). A GPR in a NEON slot is silently remapped.
**Function:** encode_neon_three_diff_narrow
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `addhn x0, v0.8h, v0.8h` (prefix=x, n=0, Ta=8h, is_high=false, U=0, opcode=0b0100)
**Expected:** Err (llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word) — `get_neon_reg` accepts `Operand::Reg`; `parse_reg_num` accepts prefixes `x|w|d|s|q|v|h|b`.
**Severity:** medium
**Regression test:** `test_encode_neon_three_diff_narrow_regression_gpr_dest` in src/backend/arm/assembler/encoder/neon.rs
**Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_neon_three_diff_narrow_gpr_dest_err -- --test-threads=1`
