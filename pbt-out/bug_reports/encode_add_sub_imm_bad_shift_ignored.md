# Bug: ADD/SUB immediate form silently ignores a non-LSL#12 fourth operand
**Law:** AArch64 ADD/SUB (immediate) permits only `LSL #0` or `LSL #12` after `#imm`. llvm-mc: `add w0, w0, #0, lsr #0` → "only 'lsl #+N' valid after immediate". A gas-compatible assembler must Err, not drop the shift.
**Impact:** `add Rd, Rn, #imm, lsr #N` (also asr/ror, or lsl with N ∉ {0,12}) is encoded as `add Rd, Rn, #imm`. The extra operand is ignored, so the assembled instruction does not match the source text.
**Function:** encode_add_sub
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** operands = [Reg("w0"), Reg("w0"), Imm(0), Shift { kind: "lsr", amount: 0 }], is_sub=false, set_flags=false
**Expected:** Err
**Actual:** Ok(Word) — `explicit_shift` is true only for `lsl && amount == 12`; any other Shift is treated as absent
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::test_encode_add_sub_regression_imm_lsr_ignored
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_add_sub_neg_imm_bad_shift -- --test-threads=1`
