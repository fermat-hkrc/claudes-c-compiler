# Bug: encode_ldxr_stxr ignores a nonzero exclusive offset
**Law:** Exclusive load/store memory operand is `[Xn|SP]` or `[Xn|SP, #0]`. A nonzero offset must be rejected (`index must be absent or #0`).
**Impact:** `ldxr x0, [x1, #-1]` (and any nonzero offset) is encoded as `ldxr x0, [x1]`. GNU as / llvm-mc reject the same text. Silent drop of the offset.
**Function:** encode_ldxr_stxr
**Detected by:** Negative/Error Contract
**Minimal input:** is_load=false, shape=8, rt=0, offset=-1 — `stxr w1, x0, [x2, #-1]`
**Expected:** Err
**Actual:** Ok(Word) encoding of the same instruction with offset 0
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_ldxr_stxr_pbt::test_encode_ldxr_stxr_regression_nonzero_offset
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / RUST_TEST_THREADS=1
