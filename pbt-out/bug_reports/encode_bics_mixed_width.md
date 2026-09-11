# Bug: encode_bics accepts mixed-width GPR operands
**Law:** All three GPR operands of scalar BICS must be the same width (all X or all W). Mixed x/w must be rejected.
**Impact:** The assembler emits a 32- or 64-bit BICS based only on Rd, silently ignoring Rn/Rm width. Assembling `bics w0, w0, x0` produces a 32-bit BICS of w0 with itself instead of an error, so a width mismatch in compiler output or hand-written asm is assembled incorrectly.
**Function:** encode_bics
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `bics w0, w0, x0` (rd=0, rn=0, rm=0, rd64=false, rn64=false, rm64=true)
**Expected:** Err (llvm-mc: "expected compatible register or logical immediate")
**Actual:** Ok(Word) — sf taken only from operand 0 (`get_reg` on Rd); Rn/Rm width flags are discarded (`let (rn, _) = get_reg(operands, 1)`).
**Severity:** medium
**Regression test:** `test_encode_bics_regression_mixed_width` in src/backend/arm/assembler/encoder/data_processing.rs
**Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_bics_neg_mixed_width -- --test-threads=1`
