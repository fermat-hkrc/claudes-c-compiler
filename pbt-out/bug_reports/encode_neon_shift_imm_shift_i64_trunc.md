# Bug: encode_neon_shift_imm truncates i64 shift via `as u32`
**Law:** The shift immediate is an i64 (`Operand::Imm`). Values outside [1, esize] must be rejected; the encoder must not fold the immediate modulo 2^32.
**Impact:** `Imm(1 + 2^32)` encodes as `#1`, so an out-of-range shift is silently accepted as a different in-range shift.
**Function:** encode_neon_shift_imm
**Detected by:** Negative/Error Contract (4e) — encode_neon_shift_imm_neg_shift_i64_trunc
**Minimal input:** `encode_neon_shift_imm([v0.8b, v0.8b, Imm(4294967297)], true)`
**Expected:** `Err` (4294967297 ∉ [1, 8])
**Actual:** `Ok(EncodeResult::Word(_))` encoding the same word as `#1` — `(16 - shift as u32) & 0xF` (`neon.rs:389-390`)
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::encode_neon_shift_imm_pbt::test_encode_neon_shift_imm_regression_shift_i64_trunc
**Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1` (proptest replay; not a test-isolation defect)
