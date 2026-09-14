# Bug: encode_bfi panics or encodes out-of-range #lsb/#width
**Law:** BFI requires 0 <= lsb < R and 1 <= width <= R-lsb (R=32 for W, 64 for X). Out-of-range immediates must be rejected with Err.
**Impact:** `bfi w0, w0, #0, #0` panics in debug (`attempt to subtract with overflow` at `width - 1`). `lsb > R` panics at `reg_width - lsb`. Other out-of-range pairs that do not overflow (e.g. width=33 for W) encode a BFM word with an illegal immr/imms instead of Err. gas / llvm-mc reject these (`expected integer in range [1, 32]`). A panic on assembler input is a crash of the builtin assembler.
**Function:** encode_bfi
**Detected by:** Negative/Error Contract
**Minimal input:** lsb=0, width=0, is_64=false — `bfi w0, w0, #0, #0` (debug overflow at bitfield.rs:113; serial reconfirm with PBT_TEST_JOBS=1)
**Expected:** Err
**Actual:** panic (debug overflow) rather than Err. Related: `lsb > reg_width` panics at bitfield.rs:112; in-range-of-u32 but out-of-ARM-range values encode Ok(Word).
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs::test_encode_bfi_regression_width_zero
