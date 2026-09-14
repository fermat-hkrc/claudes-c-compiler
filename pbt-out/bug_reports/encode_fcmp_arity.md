# Bug: encode_fcmp treats a missing second operand as FCMP #0.0
**Law:** ARM FCMP always has two operands (`Sn, Sm` / `Dn, Dm` or `Sn, #0.0` / `Dn, #0.0`). llvm-mc rejects `fcmp s0` with "too few operands for instruction". A GNU-style assembler (README.md:11) must Err, not silently encode compare-to-zero.
**Impact:** `fcmp s0` (and `fcmp d0`) is assembled as `fcmp s0, #0.0` (0x1e202008). Callers that drop an operand get a well-formed but wrong compare against 0.0 instead of an assembler error.
**Function:** encode_fcmp
**Detected by:** Negative/error contract — llvm-mc "too few operands"; ARM ARM two-operand FCMP syntax
**Minimal input:** `[Reg("s0")]` (shrunk `len=1, n=0, is_d=false`; serial reconfirm with PBT_TEST_JOBS=1)
**Expected:** Err
**Actual:** Ok(Word(0x1e202008)) — the `operands.len() < 2` branch encodes FCMP #0.0
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/fp_scalar.rs::test_encode_fcmp_regression_arity_one
