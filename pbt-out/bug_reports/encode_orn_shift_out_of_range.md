# Bug: encode_orn masks out-of-range shift amounts instead of rejecting them
**Law:** ARM ARM Logical (shifted register) imm6 is 0..31 when sf=0 and 0..63 when sf=1. llvm-mc rejects `orn w0, w0, w0, lsl #32` and `orn x0, x0, x0, lsl #64`.
**Impact:** `orn w0, w0, w0, lsl #32` encodes with imm6 = 32 (bit 5 set), which is UNALLOCATED for 32-bit ORN. An out-of-range shift in compiler output is assembled as a reserved encoding rather than rejected.
**Function:** encode_orn
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `orn w0, w0, w0, lsl #32` (rd=rn=rm=0, is_64=false, kind=lsl, amt_w=32)
**Expected:** Err (llvm-mc: "expected 'lsl', 'lsr' or 'asr' with optional integer in range [0, 31]")
**Actual:** Ok(Word) — shift amount is stored as `shift_amount & 0x3F` with no range check against sf.
**Severity:** medium
**Regression test:** `test_encode_orn_regression_shift32` in src/backend/arm/assembler/encoder/data_processing.rs
**Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_orn_neg -- --test-threads=1`
