# Bug: encode_sxtw accepts FP/SIMD register names as GPRs
**Law:** SXTW operands are GPRs; llvm-mc rejects FP/SIMD names (`sxtw d0, w1`).
**Impact:** `parse_reg_num` accepts prefixes d/s/q/v/h/b, so `sxtw d0, w1` encodes as `sxtw x0, w1`. Invalid assembly becomes a GPR sign-extend.
**Function:** encode_sxtw
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `[Reg("d0"), Reg("w1")]` (sxtw d0, w1)
**Expected:** `Err(...)`
**Actual:** `Ok(Word)` — FP name encoded as GPR number 0
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs `test_encode_sxtw_regression_fp`
**Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_sxtw_neg -- --test-threads=1`
