# Bug: XZR and x31 are accepted as ldNr base (encoded as SP)
**Law:** ARM/gas/llvm-mc accept only Xn|SP as the base. `[xzr]` and `[x31]` are invalid (SP is the Rn=31 spelling).
**Impact:** Loads intended to be rejected are encoded as `[sp]`, silently changing the address.
**Function:** encode_neon_ldnr
**Detected by:** Negative/error contract
**Minimal input:** encode_neon_ldnr([RegList({v0.8b, v1.8b}), Mem{base:"xzr", offset:0}], 2) returns Ok. Same for base "x31".
**Expected:** Err
**Actual:** Ok — parse_reg_num maps xzr and x31 to 31.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs test_encode_neon_ldnr_regression_xzr_base / test_encode_neon_ldnr_regression_x31_base
