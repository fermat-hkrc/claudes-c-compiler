# Bug: encode_sbc accepts mixed X/W register widths
**Law:** ARM SBC requires Rd, Rn, and Rm to be the same width (all X or all W). Mixed-width operands must return Err, matching llvm-mc which rejects `sbc w0, w0, x0`.
**Impact:** `sbc w0, w0, x0` is assembled using sf from Rd only, producing a 32-bit SBC with Rm taken from x0's number. The object file encodes a different instruction than the source text.
**Function:** encode_sbc
**Detected by:** Algebraic — Negative/Error Contract (4e)
**Minimal input:** operands = [Reg("w0"), Reg("w0"), Reg("x0")], set_flags = false
**Expected:** Err (operand width mismatch)
**Actual:** Ok(Word(0x5a000000)) — encodes as `sbc w0, w0, w0`; sf is taken only from Rd
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::encode_sbc_pbt::test_encode_sbc_regression_mixed_width
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 (cargo test --lib encode_sbc_neg -- --test-threads=1)
