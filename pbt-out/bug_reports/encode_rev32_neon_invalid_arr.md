# Bug: encode_rev32 NEON path accepts invalid arrangements 2s/4s/2d/1d
**Law:** Vector REV32 arrangement T must be in {8B,16B,4H,8H}. 2S/4S/2D/1D must be rejected.
**Impact:** `rev32 v0.2s, v1.2s` is encoded instead of rejected. llvm-mc/gas reject those arrangements (ARM ARM Advanced SIMD two-register miscellaneous REV32). neon_arr_to_q_size maps 2s→(Q=0,size=10) so a plausible but illegal encoding is emitted.
**Function:** encode_rev32
**Detected by:** Negative/Error Contract
**Minimal input:** `[RegArrangement{v0, "2s"}, RegArrangement{v1, "2s"}]` (rd=0, rn=0, arr="2s")
**Expected:** Err (llvm-mc: invalid operand)
**Actual:** Ok(Word) — arrangement accepted via neon_arr_to_q_size
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs::test_encode_rev32_regression_neon_invalid_arr
**Serial reconfirm:** PBT_TEST_JOBS=1 RUST_TEST_THREADS=1 reproduced
