# Bug: encode_bfi accepts FP/SIMD registers as GPR operands
**Law:** BFI is a general-purpose bitfield insert; Rd and Rn must be W/X (or ZR), not S/D/Q/V/H/B.
**Impact:** `bfi d0, x1, #0, #1` is encoded as if d0 were w0/x0 (parse_reg_num accepts prefix d/s/q/v/h/b). gas / llvm-mc reject FP/SIMD registers as "invalid operand".
**Function:** encode_bfi
**Detected by:** Negative/Error Contract
**Minimal input:** which=0, prefix="d", n=0 — `bfi d0, x1, #0, #1` (serial reconfirm with PBT_TEST_JOBS=1)
**Expected:** Err
**Actual:** Ok(Word) — parse_reg_num maps d0 to register 0; encode_bfi does not require a GPR prefix
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs::test_encode_bfi_regression_fp
