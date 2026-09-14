# Bug: encode_neon_float_three_same ignores source arrangement and accepts T mismatch
**Law:** Vector FP three-same requires dest T = Vn T = Vm T. llvm-mc rejects `fadd v0.2d, v0.2s, v0.2s`.
**Impact:** Only dest arrangement is used (`let (rn, _) = get_neon_reg`; `let (rm, _) = get_neon_reg`; match on `arr_d`). Mismatched source T is encoded as the dest form, so `fadd v0.2d, v0.2s, v0.2s` becomes the 2d encoding instead of an error.
**Function:** encode_neon_float_three_same
**Detected by:** Negative/Error Contract
**Minimal input:** `fadd v0.2d, v0.2s, v0.2s` — `[RegArrangement(v0,2d), RegArrangement(v0,2s), RegArrangement(v0,2s)]`, U=0, size_hi=0, opcode=0b11010
**Expected:** Err (T must match)
**Actual:** Ok(Word) of the 2d form (`Q=1,sz=1`). Serial reconfirm: `PBT_TEST_JOBS=1` reproduced.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::encode_neon_float_three_same_pbt::test_encode_neon_float_three_same_regression_arrangement_mismatch
