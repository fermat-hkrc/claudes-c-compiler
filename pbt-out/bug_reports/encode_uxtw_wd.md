# Bug: encode_uxtw accepts 32-bit dest (Wd)
**Law:** ARM ARM UXTW has only the 64-bit dest form `UXTW <Xd>, <Wn>`; llvm-mc rejects `uxtw w0, w0`.
**Impact:** `uxtw w0, w0` is encoded (as 32-bit MOV/ORR) instead of rejected. Invalid GNU-style assembly becomes a real instruction word. Dest width from `get_reg` is discarded (`let (rd, _) = get_reg(...)`).
**Function:** encode_uxtw
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `[Reg("w0"), Reg("w0")]` (uxtw w0, w0)
**Expected:** `Err(...)`
**Actual:** `Ok(Word(0x2A0003E0))` — dest width discarded; encodes as MOV W0, W0
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs `test_encode_uxtw_regression_wd`
**Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_uxtw -- --test-threads=1`
