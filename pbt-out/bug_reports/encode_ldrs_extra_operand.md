# Bug: encode_ldrs ignores operands beyond the second
**Law:** LDRSB/LDRSH take exactly two operands; a third must be rejected.
**Impact:** `ldrsb w0, [x0], x2` (as three parsed operands) encodes as `ldrsb w0, [x0]`. The extra operand is dropped with no error.
**Function:** encode_ldrs
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `[Reg("w0"), Mem { base: "x0", offset: 0 }, Reg("x2")]`, size=0
**Expected:** Err
**Actual:** Ok(Word(0x39c00000)) — only `operands.len() < 2` is checked
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::test_encode_ldrs_regression_extra_operand
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_ldrs_neg_extra -- --test-threads=1`
