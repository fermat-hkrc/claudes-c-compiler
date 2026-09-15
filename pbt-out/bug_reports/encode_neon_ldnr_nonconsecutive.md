# Bug: Non-consecutive / mixed-arrangement register lists are accepted
**Law:** gas/llvm-mc require a consecutive wrapping list of identical arrangements (`{v0.8b, v1.8b}`, `{v31.8b, v0.8b}`). `{v0.8b, v2.8b}` and `{v0.8b, v1.16b}` are invalid.
**Impact:** Only the first register number, first arrangement, and list length are encoded. A non-consecutive or mixed list silently becomes the consecutive same-T encoding of the first register.
**Function:** encode_neon_ldnr
**Detected by:** Negative/error contract
**Minimal input:** encode_neon_ldnr([RegList([v0.8b, v2.8b]), Mem{x1,0}], 2) returns Ok. Also [v0.8b, v1.16b].
**Expected:** Err
**Actual:** Ok — only `regs[0]` and `regs.len()` are inspected (neon.rs:1531-1542).
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs test_encode_neon_ldnr_regression_nonconsecutive / test_encode_neon_ldnr_regression_mixed_arr
