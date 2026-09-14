# Bug: encode_neon_rbit ignores source arrangement
**Law:** ARM ARM / GNU vector RBIT requires matching `T` on `Vd` and `Vn` (`T` is 8B or 16B). Mismatched arrangements must be rejected with Err.
**Impact:** `rbit v0.8b, v0.16b` (and dest `.16b` / src `.8b`, or dest `.8b` / src `.4h`) is encoded using only dest `T` for Q. llvm-mc/gas reject the instruction; the assembler emits a 32-bit word for illegal SIMD syntax.
**Function:** encode_neon_rbit
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** rd=0, rn=0, td="8b", tn="16b" — `rbit v0.8b, v0.16b`
**Expected:** Err
**Actual:** Ok(Word) with Q taken from dest only (`let (rn, _) = get_neon_reg(operands, 1)?` discards source `T`)
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::encode_neon_rbit_pbt::test_encode_neon_rbit_regression_mismatch_arr
**Serial reconfirmation:** reproduced with `cargo test --lib encode_neon_rbit_pbt -- --test-threads=1`
