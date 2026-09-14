# Bug: encode_sbfiz silently ignores a 5th operand
**Law:** SBFIZ has exactly four operands (Rd, Rn, #lsb, #width). An extra operand must be rejected with Err.
**Impact:** `sbfiz w0, w0, #0, #1, x0` encodes the 4-operand form instead of failing. gas / llvm-mc reject the extra operand (`unrecognized instruction mnemonic`). The builtin assembler can accept invalid assembly that the rest of the toolchain would reject.
**Function:** encode_sbfiz
**Detected by:** Negative/Error Contract
**Minimal input:** is_64=false, rd=0, rn=0, lsb=0, width=1, extra=Reg("x0") — `sbfiz w0, w0, #0, #1, x0` (serial reconfirm with `--test-threads=1` / PBT_TEST_JOBS=1)
**Expected:** Err
**Actual:** Ok(Word) — encode_sbfiz never checks operands.len(); extra operands are ignored.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs::test_encode_sbfiz_regression_extra_operand
