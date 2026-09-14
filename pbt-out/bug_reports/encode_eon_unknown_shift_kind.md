# Bug: encode_eon treats unknown shift kinds as LSL
**Law:** The optional 4th operand of EON must be one of lsl/lsr/asr/ror. Any other shift kind must be rejected.
**Impact:** `eon w0, w0, w0, lslx #0` encodes as unshifted EON (LSL #0). A typo in the shift mnemonic is silently accepted as LSL.
**Function:** encode_eon
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `eon w0, w0, w0, lslx #0` (unknown="lslx", amt=0)
**Expected:** Err (llvm-mc: "unexpected token in argument list")
**Actual:** Ok(Word) — match arm `_ => 0b00` defaults unknown kinds to LSL.
**Severity:** medium
**Regression test:** `test_encode_eon_regression_unknown_shift_kind` in src/backend/arm/assembler/encoder/data_processing.rs
**Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_eon_neg_unknown_shift -- --test-threads=1`
