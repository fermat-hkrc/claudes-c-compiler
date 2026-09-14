# Bug: encode_ldtr_sized accepts SIMD/FP registers as Rt
**Law:** ARM ARM LDTRB/LDTRH/STTRB/STTRH Rt is a GPR Wt. SIMD/FP names (b/h/s/d/q/v) must be rejected. llvm-mc rejects `ldtrb d0, [x1]`.
**Impact:** `ldtrb d0, [x0]` encodes as `ldtrb w0, [x0]` because parse_reg_num accepts d/s/q/v/h/b prefixes. Silent wrong-register encoding of unprivileged byte/half transfers.
**Function:** encode_ldtr_sized
**Detected by:** Negative/Error Contract
**Minimal input:** kind=2, n=0, simm in [-256,255], fp='b' or 'd' — `ldtrb d0, [x0]`
**Expected:** Err
**Actual:** Ok(Word) encoding of `ldtrb w0, [x0]`
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_ldtr_sized_pbt::test_encode_ldtr_sized_regression_fp_dest
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / --test-threads=1 (same property encode_ldtr_sized_neg_invalid_regs)
