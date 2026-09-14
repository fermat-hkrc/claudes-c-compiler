# Bug: encode_uxtw accepts FP/SIMD register names as GPRs
**Law:** UXTW operands are GPRs; llvm-mc rejects FP/SIMD names (`uxtw d0, w1`).
**Impact:** `parse_reg_num` accepts prefixes d/s/q/v/h/b, so `uxtw d0, w1` encodes as GPR 0. Invalid assembly becomes a GPR zero-extend/MOV.
**Function:** encode_uxtw
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `[Reg("d0"), Reg("w1")]` (uxtw d0, w1)
**Expected:** `Err(...)`
**Actual:** `Ok(Word)` — FP name encoded as GPR number 0
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs `test_encode_uxtw_regression_fp`
**Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_uxtw -- --test-threads=1`
