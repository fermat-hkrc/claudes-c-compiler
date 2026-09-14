# Bug: encode_orn accepts mixed X/W register widths
**Law:** ARM ARM Logical (shifted register) requires Rd, Rn, Rm to be the same width (all X or all W). llvm-mc rejects mixed-width ORN.
**Impact:** `orn w0, w0, x0` encodes using sf from Rd only. A 32-bit destination with a 64-bit source is assembled instead of rejected, producing a different instruction than the source text.
**Function:** encode_orn
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `orn w0, w0, x0` (rd=0, rn=0, rm=0, rd64=false, rn64=false, rm64=true)
**Expected:** Err (llvm-mc: "expected compatible register or logical immediate")
**Actual:** Ok(Word) — `get_reg` returns is_64 only for Rd; Rn/Rm widths are discarded (`let (rn, _) = get_reg`).
**Severity:** medium
**Regression test:** `test_encode_orn_regression_mixed_width` in src/backend/arm/assembler/encoder/data_processing.rs
**Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_orn_neg -- --test-threads=1`
