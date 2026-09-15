# Bug: encode_ldrs truncates out-of-range offsets instead of rejecting them
**Law:** Unsigned pimm is imm12*scale (byte 0..4095, half even 0..8190). Otherwise the offset must be a simm9 in [-256, 255]. llvm-mc rejects values outside both encodings.
**Impact:** `ldrsb w0, [x0, #-257]` encodes as LDURSB with imm9 = (-257)&0x1FF = 255 (Word 0x38cff000). `ldrsb x0, [x1, #4096]` and `ldrsh x0, [x1, #8192]` similarly wrap. Wrong address is assembled.
**Function:** encode_ldrs
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `[Reg("w0"), Mem { base: "x0", offset: -257 }]`, size=0
**Expected:** Err
**Actual:** Ok(Word(0x38cff000)) — `(*offset as i32) & 0x1FF` with no range check
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::test_encode_ldrs_regression_imm9_range (and test_encode_ldrs_regression_pimm_overflow)
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_ldrs_neg_offset_range_extra -- --test-threads=1`
