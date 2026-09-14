# Bug: encode_extr panics or encodes out-of-range #lsb
**Law:** EXTR requires 0 <= lsb <= 31 (W) / 63 (X). Out-of-range immediates must be rejected with Err (llvm-mc: immediate must be an integer in range [0, 31] / [0, 63]).
**Impact:** `extr w0, w0, w0, #-1` panics in debug (`attempt to shift left with overflow` at `lsb << 10` after `-1i64 as u32`). Other out-of-range values that do not overflow (e.g. lsb=32 for W) encode an EXTR word with an illegal imms field instead of Err. A panic on assembler input is a crash of the builtin assembler.
**Function:** encode_extr
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** lsb=-1, is_64=false — `extr w0, w0, w0, #-1` (serial reconfirm with --test-threads=1)
**Expected:** Err
**Actual:** panic (debug overflow) rather than Err. Related: lsb=32 for W and lsb=64 for X encode Ok(Word) with imms overlapping neighbouring fields.
**Severity:** high
**Fix:** Reject lsb outside 0..=31 (W) / 0..=63 (X) before shifting into the imms field.
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs::test_encode_extr_regression_lsb_neg
**Serial reconfirmation:** reproduced with cargo test --lib encode_extr -- --test-threads=1
