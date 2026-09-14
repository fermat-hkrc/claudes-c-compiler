# Bug: encode_ldxr_stxr accepts a W register as the address base
**Law:** Exclusive load/store Rn is Xn|SP. A 32-bit W register (or WSP) as the memory base must be rejected.
**Impact:** `ldxr x0, [w1]` is encoded as `ldxr x0, [x1]`. GNU as / llvm-mc reject a W base. Silent wrong-register assembly.
**Function:** encode_ldxr_stxr
**Detected by:** Negative/Error Contract
**Minimal input:** kind=1 — `ldxr x0, [w1]`
**Expected:** Err
**Actual:** Ok(Word) with Rn equal to the W register number
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_ldxr_stxr_pbt::test_encode_ldxr_stxr_regression_w_base
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / RUST_TEST_THREADS=1 (kind=1 of encode_ldxr_stxr_neg_invalid_regs; dedicated regression)
