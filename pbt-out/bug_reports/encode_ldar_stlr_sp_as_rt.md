# Bug: encode_ldar_stlr accepts SP/WSP as Rt
**Law:** ARM ARM LDAR/STLR Rt is Wt/Xt (register 31 = WZR/XZR), never SP/WSP. Invalid Rt must be rejected.
**Impact:** `stlr sp, [x0]` encodes as `stlr xzr, [x0]` (Rt=31). llvm-mc rejects `ldar/stlr sp, [x0]` as an invalid operand. Silent wrong-register encoding; also accepts WSP and FP/SIMD names via parse_reg_num.
**Function:** encode_ldar_stlr
**Detected by:** Negative/Error Contract
**Minimal input:** is_load=false, kind=0, n=0 — `stlr sp, [x0]`
**Expected:** Err
**Actual:** Ok(Word) with Rt=31 (XZR encoding)
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_ldar_stlr_pbt::test_encode_ldar_stlr_regression_sp_as_rt
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / RUST_TEST_THREADS=1
