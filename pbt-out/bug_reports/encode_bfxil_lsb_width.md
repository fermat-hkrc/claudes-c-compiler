# Bug: encode_bfxil panics or encodes out-of-range #lsb/#width
**Law:** BFXIL requires 0 <= lsb < R and 1 <= width <= R-lsb (R=32 for W, 64 for X). Out-of-range immediates must be rejected with Err.
**Impact:** `bfxil w0, w0, #0, #0` panics in debug (`attempt to subtract with overflow` at `lsb + width - 1`). Negative width/`as u32` wrap plus large lsb panics with add overflow. Other out-of-range pairs that do not overflow (e.g. width=33 for W, or width=0 with lsb>0) encode a BFM word with an illegal or wrong immr/imms instead of Err. gas / llvm-mc reject these (`expected integer in range [1, 32]` / `requested extract overflows register`). A panic on assembler input is a crash of the builtin assembler.
**Function:** encode_bfxil
**Detected by:** Negative/Error Contract
**Minimal input:** lsb=0, width=0, is_64=false — `bfxil w0, w0, #0, #0` (debug overflow at bitfield.rs:127; serial reconfirm with PBT_TEST_JOBS=1)
**Expected:** Err
**Actual:** panic (debug overflow) rather than Err. Related: large `lsb + width` panics at bitfield.rs:127; in-range-of-u32 but out-of-ARM-range values encode Ok(Word).
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs::test_encode_bfxil_regression_width_zero
