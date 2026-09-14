# Bug: encode_ldtr_sized silently wraps offsets outside signed 9-bit range
**Law:** ARM ARM unprivileged load/store immediate is a signed 9-bit byte offset in [-256, 255]. llvm-mc rejects `#256` and `#-257` (`index must be an integer in range [-256, 255]`). encode_ldtr_sized must Err outside that range.
**Impact:** `sttrb w0, [x0, #-257]` encodes as `sttrb w0, [x0, #255]` because `(-257 as u32) & 0x1FF == 0xFF`. Out-of-range offsets wrap to a different, in-range address — silent wrong address.
**Function:** encode_ldtr_sized
**Detected by:** Negative/Error Contract
**Minimal input:** rt=0, rn=0, is_load=false, size=0, use_offset=true, bad_offset=-257 — `sttrb w0, [x0, #-257]`
**Expected:** Err (simm9 out of range)
**Actual:** Ok(Word) — same encoding as `sttrb w0, [x0, #255]`
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_ldtr_sized_pbt::test_encode_ldtr_sized_regression_imm9_range
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / --test-threads=1
