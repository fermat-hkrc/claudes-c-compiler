# Bug: encode_bics accepts out-of-range shift amounts
**Law:** BICS imm6 is [0, 31] for 32-bit and [0, 63] for 64-bit. Amounts at bound+1 (32 / 64) must be rejected.
**Impact:** `bics w0, w0, w0, lsl #32` is accepted; the amount is masked with `0x3F`, so #32 encodes as imm6=32 which is UNALLOCATED in 32-bit form (imm6 bit 5 must be 0). The assembler emits a reserved encoding instead of an error.
**Function:** encode_bics
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `bics w0, w0, w0, lsl #32` (rd=rn=rm=0, is_64=false, kind=lsl, amt=32)
**Expected:** Err (llvm-mc: "expected 'lsl', 'lsr' or 'asr' with optional integer in range [0, 31]")
**Actual:** Ok(Word) — `shift_amount & 0x3F` with no range check against sf.
**Severity:** medium
**Regression test:** `test_encode_bics_regression_shift32` in src/backend/arm/assembler/encoder/data_processing.rs
**Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_bics_neg_shift_range -- --test-threads=1`
