# Bug: SQSHRUN truncates i64 shift immediates with `as u32`
**Law:** Operand::Imm is i64. A shift that is not in 1..=dest_esize (as an i64) must be rejected. Truncating via `as u32` must not turn an out-of-range immediate into a legal encoding.
**Impact:** Imm(4294967297) (= 2^32+1) encodes as shift #1. An out-of-range immediate is silently wrapped, so the assembled word does not match the written constant.
**Function:** encode_neon_sqshrun
**Detected by:** Negative/Error Contract (5) — coverage sweep of the `*v as u32` path; GNU-style `#imm` is an i64 immediate
**Minimal input:** `[RegArrangement(v0, 8b), RegArrangement(v0, 8h), Imm(4294967297)]`, is_rounding=false, is_high=false
**Expected:** Err (4294967297 is not in [1, 8])
**Actual:** Ok(Word) encoding shift #1, because `4294967297i64 as u32 == 1`.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::test_encode_neon_sqshrun_regression_shift_i64_trunc
**Serial reconfirmation:** reproduced with PBT_TEST_JOBS=1 (cargo test --lib encode_neon_sqshrun -- --test-threads=1)
