# Bug: encode_ubfx accepts FP/SIMD registers as GPR operands
**Law:** UBFX is a general-purpose unsigned bitfield extract; Rd and Rn must be W/X (or ZR), not S/D/Q/V/H/B.
**Impact:** `ubfx d0, x1, #0, #1` is encoded as if d0 were w0/x0 (parse_reg_num accepts prefix d/s/q/v/h/b). gas / llvm-mc reject FP/SIMD registers as "invalid operand".
**Function:** encode_ubfx
**Detected by:** Negative/Error Contract
**Minimal input:** which=0, prefix="d", n=0 — `ubfx d0, x1, #0, #1` (serial reconfirm with PBT_TEST_JOBS=1)
**Expected:** Err
**Actual:** Ok(Word) — parse_reg_num maps d0 to register 0; encode_ubfx does not require a GPR prefix
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs::test_encode_ubfx_regression_fp
