# Bug: encode_cbz accepts FP/SIMD register names as Rt
**Law:** ARM CBZ/CBNZ Rt is a GPR (Wt/Xt/WZR/XZR). llvm-mc rejects `cbz d0, L` and the other FP/SIMD prefixes (s/q/v/h/b). encode_cbz must Err on those names.
**Impact:** `cbz d0, L` is assembled as `cbz w0, L` (sf=0, Rt=0). An FP name is silently recoded as a 32-bit GPR, producing the wrong instruction.
**Function:** encode_cbz
**Detected by:** Algebraic — Negative/Error Contract (4e)
**Minimal input:** operands = [Reg("d0"), Symbol("L")], is_nz=false
**Expected:** Err (invalid operand; FP/SIMD is not a valid CBZ register)
**Actual:** Ok(WordWithReloc) with sf=0 and Rt=0 — parse_reg_num accepts d/s/q/v/h/b prefixes and is_64bit_reg is false for them.
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_cbz_pbt::test_encode_cbz_regression_fp_reg
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib encode_cbz -- --test-threads=1 reproduced the failure (shrunk witness was sp; d0 confirmed via the dedicated regression test).
