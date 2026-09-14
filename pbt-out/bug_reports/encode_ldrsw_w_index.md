# Bug: encode_ldrsw encodes a W index without UXTW/SXTW as LSL
**Law:** LDRSW (register) with a 32-bit index requires uxtw or sxtw (amount 0 or 2). A bare Wm is invalid.
**Impact:** `ldrsw x0, [x1, w2]` encodes option=011 (LSL) as if the index were Xm. llvm-mc: "expected 'uxtw' or 'sxtw'".
**Function:** encode_ldrsw
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `[Reg("x0"), MemRegOffset { base: "x1", index: "w2", extend: None, shift: None }]`
**Expected:** Err
**Actual:** Ok(Word) with option=011, Rm=2 — match arm `(None, None) => (0b011, 0)` ignores W vs X
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::test_encode_ldrsw_regression_w_index_no_extend
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_ldrsw -- --test-threads=1`
