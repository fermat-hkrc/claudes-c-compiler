# Bug: encode_bic immediate form treats XZR/WZR as SP/WSP
**Law:** BIC (immediate) is an alias of AND (immediate). Rd of 31 is SP/WSP, not XZR/WZR. `bic xzr, xn, #imm` / `bic wzr, wn, #imm` must be rejected. (`bic sp, xn, #imm` is valid.)
**Impact:** `bic wzr, w0, #1` is encoded as `and wsp, w0, #0xfffffffe`. A request to discard the result (write the zero register) instead writes the stack pointer.
**Function:** encode_bic
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `bic wzr, w0, #1` (is_64=false, rn=0)
**Expected:** Err (llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word) — `parse_reg_num` maps both `wzr` and `wsp` to 31; AND-immediate encoding interprets 31 as WSP.
**Severity:** high
**Regression test:** `test_encode_bic_regression_imm_xzr_rd` in src/backend/arm/assembler/encoder/data_processing.rs
**Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_bic_neg_imm_xzr_rd -- --test-threads=1`
