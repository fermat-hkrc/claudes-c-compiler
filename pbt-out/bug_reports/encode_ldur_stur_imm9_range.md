# Bug: encode_ldur_stur silently wraps offsets outside signed 9-bit range
**Law:** ARM ARM unscaled/unprivileged load/store immediate is a signed 9-bit byte offset in [-256, 255]. llvm-mc rejects `#256` and `#-257` (`index must be an integer in range [-256, 255]`). encode_ldur_stur must Err outside that range.
**Impact:** `stur w0, [x0, #-257]` encodes as `stur w0, [x0, #255]` because `(-257 as u32) & 0x1FF == 0xFF`. Out-of-range offsets wrap to a different, in-range address — silent wrong address.
**Function:** encode_ldur_stur
**Detected by:** Negative/Error Contract
**Minimal input:** rt=0, rn=0, offset=-257, is_load=false, is_64=false, unpriv=false — `stur w0, [x0, #-257]`
**Expected:** Err (simm9 out of range)
**Actual:** Ok(Word) — same encoding as `stur w0, [x0, #255]`
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_ldur_stur_pbt::test_encode_ldur_stur_regression_imm9_range
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / RUST_TEST_THREADS=1
