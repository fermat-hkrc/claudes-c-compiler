# Bug: encode_cas accepts mixed W/X Rs and Rt
**Law:** CAS requires Rs and Rt to be the same width (both W or both X). llvm-mc/gas reject `cas x0, w0, [x1]`.
**Impact:** Size is taken only from Rs (`is_64` of operand 0). `cas x0, w0, [x1]` encodes as 64-bit CAS with Rt=0, silently ignoring Wt vs Xt.
**Function:** encode_cas
**Detected by:** Negative/Error Contract
**Minimal input:** n=0, kind=0 — `cas x0, w0, [x1]`
**Expected:** Err
**Actual:** Ok(Word) encoding of `cas x0, x0, [x1]` (size=11 from Rs)
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_cas_pbt::test_encode_cas_regression_mixed_width
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / --test-threads=1
