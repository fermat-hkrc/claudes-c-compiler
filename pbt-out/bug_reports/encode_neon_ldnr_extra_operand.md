# Bug: Surplus operand on ld2r/ld3r/ld4r is ignored
**Law:** llvm-mc/gas reject a surplus operand after a complete ldNr (README.md:12 same textual assembly as gas). encode_neon_ldnr must return Err.
**Impact:** Malformed assembly is silently encoded as a 2-operand load, hiding typos and extra tokens.
**Function:** encode_neon_ldnr
**Detected by:** Negative/error contract
**Minimal input:** encode_neon_ldnr([RegList({v0.8b, v1.8b}), Mem{x0,0}, Cond("eq")], 2) returns Ok(Word) instead of Err.
**Expected:** Err
**Actual:** Ok — only `operands.len() < 2` is checked (neon.rs:1529).
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs test_encode_neon_ldnr_regression_extra_operand
