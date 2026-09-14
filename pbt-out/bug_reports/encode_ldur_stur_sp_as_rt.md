# Bug: encode_ldur_stur accepts SP/WSP as Rt
**Law:** ARM ARM LDUR/STUR Rt is Wt/Xt (register 31 = WZR/XZR), never SP/WSP. Invalid Rt must be rejected.
**Impact:** `stur sp, [x0, #-256]` encodes as `stur wzr, [x0, #-256]` (Rt=31, size=32-bit because `"sp".starts_with('x')` is false). llvm-mc rejects `ldur/stur sp, [x0]` as an invalid operand. Silent wrong-register encoding.
**Function:** encode_ldur_stur
**Detected by:** Negative/Error Contract
**Minimal input:** is_load=false, kind=0, n=0, offset=-256 — `stur sp, [x0, #-256]`
**Expected:** Err
**Actual:** Ok(Word(0xbc10001f)) encoding of `stur wzr, [x0, #-256]`
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_ldur_stur_pbt::test_encode_ldur_stur_regression_sp_as_rt
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / RUST_TEST_THREADS=1
