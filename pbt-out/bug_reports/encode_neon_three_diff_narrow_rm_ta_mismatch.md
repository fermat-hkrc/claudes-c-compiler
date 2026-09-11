# Bug: encode_neon_three_diff_narrow ignores Rm arrangement Ta
**Law:** ARM ARM ADDHN form is `Vd.Tb, Vn.Ta, Vm.Ta` — Vm must use the same arrangement as Vn. A mismatched Rm Ta must be rejected, matching llvm-mc / GNU as.
**Impact:** `addhn v0.4h, v0.4s, v0.8h` is encoded as ADDHN size=01 (from Rn's 4s) as if Vm were `.4s`. Invalid mixed-arrangement assembly is silently accepted.
**Function:** encode_neon_three_diff_narrow
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `addhn v0.4h, v0.4s, v0.8h` (rd=rn=rm=0, Ta_n=4s, Ta_m=8h, is_high=false, U=0, opcode=0b0100)
**Expected:** Err (llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word) — Rm arrangement is discarded (`let (rm, _) = get_neon_reg(operands, 2)`); size is taken only from operand 1.
**Severity:** medium
**Regression test:** `test_encode_neon_three_diff_narrow_regression_rm_ta_mismatch` in src/backend/arm/assembler/encoder/neon.rs
**Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_neon_three_diff_narrow_rm_ta_must_match -- --test-threads=1`
