# Bug: encode_neon_tbl ignores Vm arrangement mismatch with Vd.Ta
**Law:** ARM ARM requires Vm.Ta to equal Vd.Ta. llvm-mc rejects `tbl v0.8b, {v0.16b}, v0.16b`. Mismatched destination/index arrangements must Err.
**Impact:** The index register's arrangement is discarded (`let (rm, _) = get_neon_reg(operands, 2)`), so a `.16b` index with a `.8b` dest encodes as Q=0 TBL.
**Function:** encode_neon_tbl
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** Vd.8b, table `{v0.16b}`, Vm.16b — `tbl v0.8b, {v0.16b}, v0.16b`
**Expected:** Err
**Actual:** Ok(Word(...)) — Vm arrangement is ignored; Q is taken only from dest
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::encode_neon_tbl_pbt::test_encode_neon_tbl_regression_mismatched_t
**Serial reconfirmation:** SUT discards Vm arrangement; covered by encode_neon_tbl_neg_arity_kinds (mismatched-T arm)
