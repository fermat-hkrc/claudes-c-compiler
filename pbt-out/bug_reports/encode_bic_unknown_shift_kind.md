# Bug: encode_bic maps unknown shift kinds to LSL
**Law:** BIC (shifted register) shift kind must be one of lsl, lsr, asr, ror. Any other kind must be rejected.
**Impact:** An unknown shift kind (`lslx`, `rrx`, `rol`, empty) is encoded as LSL. A typo in assembly or a parser glitch producing a bad Shift.kind silently changes the shift type rather than failing the assemble.
**Function:** encode_bic
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** Shift { kind: "lslx", amount: 0 } on `bic w0, w0, w0` (rd=0, rn=0, rm=0, is_64=false)
**Expected:** Err (ARM ARM lists only lsl/lsr/asr/ror; llvm-mc rejects unknown shift names)
**Actual:** Ok(Word) — `match kind { "lsl"=>00, "lsr"=>01, "asr"=>10, "ror"=>11, _ => 0b00 }` defaults to LSL.
**Severity:** medium
**Regression test:** `test_encode_bic_regression_unknown_shift_kind` in src/backend/arm/assembler/encoder/data_processing.rs
**Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_bic_neg_unknown_shift_kind -- --test-threads=1`
