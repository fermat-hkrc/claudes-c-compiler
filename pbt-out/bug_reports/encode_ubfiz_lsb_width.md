# Bug: encode_ubfiz panics or encodes out-of-range #lsb/#width
**Law:** UBFIZ requires 0 <= lsb < R and 1 <= width <= R-lsb (R=32 for W, 64 for X). Out-of-range immediates must be rejected with Err.
**Impact:** `ubfiz w0, w0, #0, #0` panics in debug (`attempt to subtract with overflow` at `width - 1`). Other out-of-range pairs that do not overflow (e.g. width=33 for W, lsb=32) encode a UBFM word with an illegal immr/imms instead of Err. gas / llvm-mc reject these (`expected integer in range [1, 32]` / `[0, 31]`). A panic on assembler input is a crash of the builtin assembler.
**Function:** encode_ubfiz
**Detected by:** Negative/Error Contract
**Minimal input:** lsb=0, width=0, is_64=false — `ubfiz w0, w0, #0, #0` (debug overflow at bitfield.rs:85; serial reconfirm with `--test-threads=1` / PBT_TEST_JOBS=1)
**Expected:** Err
**Actual:** panic (debug overflow) rather than Err. Related: wrapping_sub on lsb keeps encoding for lsb >= R; in-range-of-u32 but out-of-ARM-range values encode Ok(Word).
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs::test_encode_ubfiz_regression_width_zero
