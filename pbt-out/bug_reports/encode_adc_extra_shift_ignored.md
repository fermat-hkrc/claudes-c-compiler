# Bug: encode_adc silently ignores a trailing shift operand
**Law:** ARM ADC has no shifted-register form; `encode_adc([Rd, Rn, Rm, Shift{...}], _)` must return Err, matching llvm-mc / GNU as which reject `adc Rd, Rn, Rm, lsl #N`.
**Impact:** Source text `adc w0, w0, w0, lsl #0` is assembled as unshifted `adc w0, w0, w0` instead of being rejected. The assembled object silently disagrees with the written instruction.
**Function:** encode_adc
**Detected by:** Algebraic — Negative/Error Contract (4e)
**Minimal input:** operands = [Reg("w0"), Reg("w0"), Reg("w0"), Shift { kind: "lsl", amount: 0 }], set_flags = false
**Expected:** Err (invalid operand; ADC has no shift field; bits 15:10 are fixed 000000)
**Actual:** Ok(Word) — extra operand ignored; encodes as `adc w0, w0, w0` (0x1a000000)
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::encode_adc_pbt::test_encode_adc_regression_extra_shift
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 (cargo test --lib encode_adc_neg -- --test-threads=1)
