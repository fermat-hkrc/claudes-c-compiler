# Bug: Illegal post-index immediate is accepted
**Law:** gas/llvm-mc require the post-index immediate to equal num_structs*esize (e.g. ld2r .8b → #2). `#0`, `#-1`, `#4` on ld2r .8b are invalid.
**Impact:** Wrong post-index immediates encode as the immediate-post-index form (Rm=11111) regardless of the offset, so the assembler accepts illegal syntax and the writeback size is not checked.
**Function:** encode_neon_ldnr
**Detected by:** Negative/error contract
**Minimal input:** encode_neon_ldnr([RegList({v0.8b, v1.8b}), MemPostIndex{x1, -1}], 2) returns Ok. Same for offset 0.
**Expected:** Err
**Actual:** Ok — MemPostIndex always sets Rm=11111 and ignores `offset` (neon.rs:1563-1565).
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs test_encode_neon_ldnr_regression_bad_post_imm
