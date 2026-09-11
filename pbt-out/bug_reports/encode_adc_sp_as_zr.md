# Bug: encode_adc treats SP/WSP as XZR/WZR
**Law:** ARM ADC encoding uses register 31 as WZR/XZR, not WSP/SP. llvm-mc rejects `adc wsp, w0, w0` and `adc sp, x0, x1`. encode_adc must Err when any of Rd/Rn/Rm is sp/wsp.
**Impact:** `adc wsp, w0, w0` is assembled as `adc wzr, w0, w0` (encoding 0x1a00001f). The object file contains a different instruction than the source text — writes WZR instead of addressing the stack pointer.
**Function:** encode_adc
**Detected by:** Algebraic — Negative/Error Contract (4e)
**Minimal input:** operands = [Reg("wsp"), Reg("w0"), Reg("w0")], set_flags = false
**Expected:** Err (invalid operand; SP is not a valid ADC register)
**Actual:** Ok(Word(0x1a00001f)) — same encoding as `adc wzr, w0, w0`
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::encode_adc_pbt::test_encode_adc_regression_sp
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 (cargo test --lib encode_adc_neg -- --test-threads=1)
