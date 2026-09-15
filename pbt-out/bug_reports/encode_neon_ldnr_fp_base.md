# Bug: SIMD/FP register accepted as ldNr base
**Law:** llvm-mc/gas reject `[d0]`, `[s0]`, `[v0]`, `[q0]` as the ld2r/ld3r/ld4r base. Base must be Xn|SP.
**Impact:** A floating-point/SIMD name is parsed as a GPR number and encoded as that X register.
**Function:** encode_neon_ldnr
**Detected by:** Negative/error contract
**Minimal input:** encode_neon_ldnr([RegList({v0.8b, v1.8b}), Mem{base:"d0", offset:0}], 2) returns Ok.
**Expected:** Err
**Actual:** Ok — parse_reg_num accepts d/s/v/q prefixes.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs test_encode_neon_ldnr_regression_fp_base
