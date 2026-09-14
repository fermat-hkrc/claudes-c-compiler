# Bug: encode_ldxp_stxp encodes STXP when Ws aliases a source
**Law:** ARM CONSTRAINED UNPREDICTABLE / llvm-mc: "unpredictable STXP instruction, status is also a source" when Ws is the same architectural register as Rt, Rt2, or Xn. (WZR vs SP, both encoding 31, is allowed.)
**Impact:** `stxp w0, w0, w1, [x2]` is encoded instead of rejected. GNU as / llvm-mc refuse the same text. Silent emission of an UNPREDICTABLE exclusive store.
**Function:** encode_ldxp_stxp
**Detected by:** Negative/Error Contract
**Minimal input:** rt=0, rt2=0, rn=0, acqrel=false, is_64=false, kind=0 — `stxp w0, w0, w0, [x0]`
**Expected:** Err
**Actual:** Ok(Word(0x88200000))
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_ldxp_stxp_pbt::test_encode_ldxp_stxp_regression_ws_overlap
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / RUST_TEST_THREADS=1
