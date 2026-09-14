# Bug: encode_ldrsw silently truncates out-of-range offsets
**Law:** Unsigned LDRSW pimm is [0, 16380] multiple of 4; otherwise unscaled/pre/post simm9 is [-256, 255]. Offsets outside both ranges must be Err, not masked into imm9.
**Impact:** `ldrsw x0, [x1, #-257]` encodes as LDURSW with imm9=255 (offset +255). `ldrsw x0, [x1, #16384]` encodes as LDURSW with imm9=0 (offset 0). Silent wrong address.
**Function:** encode_ldrsw
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `[Reg("x0"), Mem { base: "x1", offset: -257 }]` → Ok(Word(0xb88ff000)); also offset 16384 → Ok(Word(0xb8800020))
**Expected:** Err
**Actual:** Ok(Word) with imm9 = offset as i32 & 0x1FF
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::test_encode_ldrsw_regression_imm9_range and test_encode_ldrsw_regression_pimm_overflow
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_ldrsw -- --test-threads=1`
