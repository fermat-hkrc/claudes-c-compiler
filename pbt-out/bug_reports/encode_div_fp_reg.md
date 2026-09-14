# Bug: encode_div accepts FP/SIMD register names as GPRs
**Law:** ARM ARM UDIV/SDIV take only Wt/Xt. llvm-mc rejects `udiv d0, ...`. Floating-point/SIMD names (d/s/q/v/h/b) are not valid UDIV/SDIV operands.
**Impact:** `sdiv d0, x1, x2` is assembled as 32-bit `sdiv w0, w1, w2` because parse_reg_num accepts the d/s/q/v/h/b prefixes and is_64bit_reg is false for them. FP typos silently become W-register divides.
**Function:** encode_div
**Detected by:** Negative/Error Contract (4e) — contract-surface sweep of parse_reg_num FP prefixes
**Minimal input:** `encode_div([Reg("d0"), Reg("x1"), Reg("x2")], unsigned=false)` i.e. `sdiv d0, x1, x2` (which=0, prefix="d", n=0)
**Expected:** Err
**Actual:** Ok(Word) — encodes as 32-bit SDIV with Rd=0 (same number as d0). Reproduced serially (`PBT_TEST_JOBS=1`).
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::encode_div_pbt::test_encode_div_regression_fp_reg
