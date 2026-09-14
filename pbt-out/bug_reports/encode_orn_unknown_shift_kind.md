# Bug: encode_orn treats unknown shift kinds as LSL
**Law:** Valid ORN shifts are LSL, LSR, ASR, ROR. llvm-mc rejects unknown kinds such as `lslx`, `rrx`, `rol`.
**Impact:** `orn w0, w0, w0, lslx #0` encodes as `orn w0, w0, w0` (LSL #0). A misspelled shift is silently canonicalized to LSL.
**Function:** encode_orn
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `orn w0, w0, w0, lslx #0` (rd=rn=rm=0, is_64=false, unknown="lslx", amt_ok=0)
**Expected:** Err (llvm-mc: unexpected token / expected lsl, lsr, or asr)
**Actual:** Ok(Word) — the shift-kind match arm `_ => 0b00` defaults unknown kinds to LSL.
**Severity:** low
**Regression test:** `test_encode_orn_regression_unknown_shift_kind` in src/backend/arm/assembler/encoder/data_processing.rs
**Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_orn_neg -- --test-threads=1`
