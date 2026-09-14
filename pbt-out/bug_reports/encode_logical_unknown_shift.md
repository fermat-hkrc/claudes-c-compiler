# Bug: encode_logical treats unknown shift kinds as LSL
**Law:** The only valid shift kinds are lsl, lsr, asr, ror. llvm-mc rejects `and w0, w0, w0, lslx #0`.
**Impact:** A typo such as `lslx` or `uxtw` is silently encoded as LSL #amount.
**Function:** encode_logical
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** operands `[Reg("w0"), Reg("w0"), Reg("w0"), Shift{"lslx", 0}]`, opc=0
**Expected:** Err
**Actual:** Ok(Word) — match arm `_ => 0b00` defaults unknown kinds to LSL
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::test_encode_logical_regression_unknown_shift
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1
