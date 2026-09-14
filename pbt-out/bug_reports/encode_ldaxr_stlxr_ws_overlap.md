# Bug: encode_ldaxr_stlxr encodes STLXR when Ws aliases a source
**Law:** ARM CONSTRAINED UNPREDICTABLE / llvm-mc: "unpredictable STXR instruction, status is also a source" when Ws is the same architectural register as Rt or Xn. (WZR vs SP, both encoding 31, is allowed.)
**Impact:** `stlxr w0, w0, [x1]` is encoded instead of rejected. GNU as / llvm-mc refuse the same text. Silent emission of an UNPREDICTABLE exclusive store.
**Function:** encode_ldaxr_stlxr
**Detected by:** Negative/Error Contract
**Minimal input:** rt=0, rn=0, variant=0, is_64=false, overlap_rt=false — `stlxr w0, w0, [x0]` (Ws aliases both Rt and Xn)
**Expected:** Err
**Actual:** Ok(Word(0x8800FC00))
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_ldaxr_stlxr_pbt::test_encode_ldaxr_stlxr_regression_ws_overlap
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / RUST_TEST_THREADS=1
