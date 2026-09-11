# Bug: encode_adr silently truncates immediates outside the 21-bit signed range
**Law:** ARM ADR immediate is a 21-bit signed PC-relative byte offset in [-1048576, 1048575] (±1MB). llvm-mc rejects `#1048576` and `#-1048577`. encode_adr documents the missing check (`TODO: validate 21-bit signed immediate range` at load_store.rs:697) and must Err outside that range.
**Impact:** `adr x0, #-1048577` encodes as `adr x0, #-1` (0x70ffffe0) because the low 21 bits of -1048577 are all ones. Out-of-range offsets wrap to a different, in-range address — silent wrong target.
**Function:** encode_adr
**Detected by:** Algebraic — Negative/Error Contract (4e)
**Minimal input:** operands = [Reg("x0"), Imm(-1048577)]
**Expected:** Err (21-bit signed immediate out of range)
**Actual:** Ok(Word(0x70ffffe0)) — same encoding as `adr x0, #-1`
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_adr_pbt::test_encode_adr_regression_imm_range
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 (cargo test --lib encode_adr_neg_imm_range -- --test-threads=1)
