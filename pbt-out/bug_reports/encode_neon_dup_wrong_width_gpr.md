# Bug: encode_neon_dup accepts wrong-width and non-GPR sources
**Law:** ARM ARM DUP (general) source is Wn (including WZR) for T in {8B,16B,4H,8H,2S,4S} and Xn (including XZR) for T=2D. SP/WSP and FP/SIMD names are not valid sources. llvm-mc/gas reject them.
**Impact:** `dup v0.8b, x0` is encoded as if the source were w0; `dup v0.4s, sp` is encoded as register 31 (WZR). Callers get a silent wrong instruction instead of an assembler error.
**Function:** encode_neon_dup
**Detected by:** Negative/Error Contract (5)
**Minimal input:** operands = [RegArrangement(v0, 8b), Reg(x0)]  — `dup v0.8b, x0`
**Expected:** Err (llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word) using parse_reg_num("x0")=0, identical to `dup v0.8b, w0`. Related witnesses that also encode: `dup v0.4s, sp` (SP→31), `dup v0.2d, w0` (W on 64-bit T), FP names d0/s0/q0/v0. Serial reconfirmation: `PBT_TEST_JOBS=1 cargo test --lib encode_neon_dup_neg_gpr -- --test-threads=1` still fails.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::test_encode_neon_dup_regression_wrong_width_x_on_8b (also test_encode_neon_dup_regression_sp)
