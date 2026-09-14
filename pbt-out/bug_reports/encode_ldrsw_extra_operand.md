# Bug: encode_ldrsw ignores operands beyond the second
**Law:** LDRSW takes exactly two operands; a third must be rejected.
**Impact:** `ldrsw x0, [x1], x2` (as three parsed operands) encodes as `ldrsw x0, [x1]`. The extra operand is dropped with no error.
**Function:** encode_ldrsw
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `[Reg("x0"), Mem { base: "x1", offset: 0 }, Reg("x2")]`
**Expected:** Err
**Actual:** Ok(Word(0xb9800000)) — only `operands.len() < 2` is checked
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::test_encode_ldrsw_regression_extra_operand
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_ldrsw -- --test-threads=1`
