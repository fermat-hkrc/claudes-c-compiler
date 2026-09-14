# Bug: encode_eon accepts mixed x/w register widths
**Law:** ARM ARM EON requires all three registers to be Wt or all Xt. llvm-mc rejects mixed x/w.
**Impact:** `eon w0, w0, x0` encodes as a 32-bit EON (sf taken only from Rd). A width mismatch in asm is assembled instead of rejected.
**Function:** encode_eon
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `eon w0, w0, x0` (rd=rn=rm=0, rd64=false, rn64=false, rm64=true)
**Expected:** Err (llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word) — sf is taken only from operand 0 via get_reg; Rn/Rm widths are never checked.
**Severity:** medium
**Regression test:** `test_encode_eon_regression_mixed_width` in src/backend/arm/assembler/encoder/data_processing.rs
**Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_eon_neg_mixed_width -- --test-threads=1`
