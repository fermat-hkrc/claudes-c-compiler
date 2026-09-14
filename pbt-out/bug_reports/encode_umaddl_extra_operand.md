# Bug: encode_umaddl silently ignores a 5th operand
**Law:** Scalar UMADDL is a 4-operand instruction (`Xd, Wn, Wm, Xa`). A 5th operand must return Err, matching GNU as / llvm-mc (`invalid operand for instruction`).
**Impact:** Assembler accepts illegal syntax such as `umaddl x0, w0, w0, x0, x0` and emits the 4-operand encoding, so a typo or extra operand is silently dropped instead of failing the assemble.
**Function:** encode_umaddl
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `[Reg("x0"), Reg("w0"), Reg("w0"), Reg("x0"), Reg("x0")]` (shrunk; any extra Operand kind also accepted)
**Expected:** `Err`
**Actual:** `Ok(Word(0x9ba00000))` — extra operand ignored; `get_reg` only reads indices 0..3
**Severity:** medium
**Regression test:** `src/backend/arm/assembler/encoder/data_processing.rs` `test_encode_umaddl_regression_extra_operand`
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1` / `--test-threads=1`
