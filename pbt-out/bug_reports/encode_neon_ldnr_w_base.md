# Bug: W-register base is accepted for ld2r/ld3r/ld4r
**Law:** ARM/gas/llvm-mc require base Xn|SP. `[w0]`, `[wzr]`, `[wsp]` are invalid.
**Impact:** 32-bit base registers assemble to the same encoding as the corresponding X/SP register, producing the wrong addressing mode.
**Function:** encode_neon_ldnr
**Detected by:** Negative/error contract
**Minimal input:** encode_neon_ldnr([RegList({v0.8b, v1.8b}), Mem{base:"w0", offset:0}], 2) returns Ok.
**Expected:** Err
**Actual:** Ok — parse_reg_num maps w0 to 0.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs test_encode_neon_ldnr_regression_w_base
