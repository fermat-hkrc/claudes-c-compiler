# Bug: encode_eon masks out-of-range shift amounts instead of rejecting them
**Law:** ARM ARM EON shift amount is [0, 31] for 32-bit and [0, 63] for 64-bit. llvm-mc rejects `lsl #32` (W) and `lsl #64` (X).
**Impact:** `eon w0, w0, w0, lsl #32` encodes with imm6=32, an unallocated 32-bit encoding. `eon x0, x0, x0, lsl #64` encodes as lsl #0 because amount is masked with 0x3F.
**Function:** encode_eon
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `eon w0, w0, w0, lsl #32` (rd=rn=rm=0, is_64=false, kind="lsl", amt_w=32)
**Expected:** Err (llvm-mc: "expected 'lsl', 'lsr' or 'asr' with optional integer in range [0, 31]")
**Actual:** Ok(Word) — shift amount is taken as `amount & 0x3F` with no width check.
**Severity:** medium
**Regression test:** `test_encode_eon_regression_shift32` in src/backend/arm/assembler/encoder/data_processing.rs
**Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_eon_neg_shift_range -- --test-threads=1`
