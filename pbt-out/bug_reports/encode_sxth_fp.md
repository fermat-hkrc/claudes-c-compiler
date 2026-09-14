# Bug: encode_sxth encodes FP/SIMD register names as GPRs
**Law:** SXTH is a GPR bitfield alias. llvm-mc rejects FP/SIMD names (`d`/`s`/`q`/`v`/`h`/`b`) in either operand slot.
**Impact:** `sxth d0, w1` is encoded as `sxth w0, w1` (prefix `d` parsed as a register number), producing GPR machine code for SIMD-looking assembly.
**Function:** encode_sxth
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `[Reg("d0"), Reg("w1")]` (sxth d0, w1)
**Expected:** `Err(...)`
**Actual:** `Ok(Word)` — `parse_reg_num` accepts prefixes `d|s|q|v|h|b` and `is_fp_reg` is never consulted
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs `test_encode_sxth_regression_fp`
**Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_sxth_neg -- --test-threads=1`
