# Bug: encode_bic masks out-of-range shift amounts instead of rejecting them
**Law:** BIC (shifted register) shift amount must be in [0, 31] for W-form and [0, 63] for X-form. Amounts outside that range (bound+1) must be rejected.
**Impact:** `bic w0, w0, w0, lsl #32` is encoded with imm6=32 (bit 5 set). That encoding is UNALLOCATED in the 32-bit logical-shifted-register space; the assembler should error rather than emit an illegal instruction.
**Function:** encode_bic
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `bic w0, w0, w0, lsl #32` (is_64=false, kind=lsl, amt=32)
**Expected:** Err (llvm-mc: "expected 'lsl', 'lsr' or 'asr' with optional integer in range [0, 31]")
**Actual:** Ok(Word) — shift amount is stored as `(shift_amount & 0x3F) << 10` with no range check against sf.
**Severity:** medium
**Regression test:** `test_encode_bic_regression_shift32` in src/backend/arm/assembler/encoder/data_processing.rs
**Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_bic_neg_shift_range_neon_arr -- --test-threads=1`
