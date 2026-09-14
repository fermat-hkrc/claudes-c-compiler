# Bug: encode_bfxil ignores extra operands
**Law:** BFXIL takes exactly four operands (Rd, Rn, #lsb, #width); a fifth operand must be rejected.
**Impact:** The assembler silently encodes `bfxil w0, w0, #0, #1, x0` (and any extra Imm/Shift/RegArrangement) as a valid BFXIL of the first four operands, so invalid GNU-style assembly produces a machine-code word instead of an error. gas / llvm-mc reject the extra operand (`unrecognized instruction mnemonic`).
**Function:** encode_bfxil
**Detected by:** Negative/Error Contract
**Minimal input:** `[Reg("w0"), Reg("w0"), Imm(0), Imm(1), Reg("x0")]` (shrunk from extra_operand; serial reconfirm with PBT_TEST_JOBS=1)
**Expected:** Err
**Actual:** Ok(Word) — get_reg/get_imm only read indices 0..3, so operands beyond index 3 are ignored
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs::test_encode_bfxil_regression_extra_operand
