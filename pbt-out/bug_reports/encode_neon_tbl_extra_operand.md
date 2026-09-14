# Bug: encode_neon_tbl ignores extra operands
**Law:** GNU-style TBL takes exactly three operands (`Vd.Ta`, register list, `Vm.Ta`). A fourth operand must be rejected with Err.
**Impact:** GNU-incompatible assembly such as `tbl v0.8b, {v0.16b}, v0.8b, v0.8b` is silently encoded as TBL, so the built-in assembler accepts text that gas/llvm-mc reject.
**Function:** encode_neon_tbl
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** rd=0, rn=0, rm=0, extra=0, ta="8b", n=1 — `tbl v0.8b, {v0.16b}, v0.8b, v0.8b` (operands `[RegArrangement{v0,"8b"}, RegList({v0.16b}), RegArrangement{v0,"8b"}, RegArrangement{v0,"8b"}]`)
**Expected:** Err
**Actual:** Ok(Word(...)) — arity check is `operands.len() < 3`, so extra operands beyond index 2 are ignored
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::encode_neon_tbl_pbt::test_encode_neon_tbl_regression_extra_operand
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_neon_tbl -- --test-threads=1`
