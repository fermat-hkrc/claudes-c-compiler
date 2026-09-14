# Bug: encode_madd treats SP/WSP as XZR/WZR
**Law:** ARM MADD encoding uses register 31 as WZR/XZR, not WSP/SP. llvm-mc rejects `madd wsp, w0, w0, w0` and `madd sp, x0, x1, x2`. encode_madd must Err when any of Rd/Rn/Rm/Ra is sp/wsp.
**Impact:** `madd wsp, w0, w0, w0` is assembled as `madd wzr, w0, w0, w0`. The object file contains a different instruction than the source text — writes WZR instead of addressing the stack pointer.
**Function:** encode_madd
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** operands = [Reg("wsp"), Reg("w0"), Reg("w0"), Reg("w0")]
**Expected:** Err (invalid operand; SP is not a valid MADD register)
**Actual:** Ok(Word) — parse_reg_num maps sp/wsp to 31, same encoding as `madd wzr, w0, w0, w0`. Reproduced serially (`PBT_TEST_JOBS=1`).
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::encode_madd_pbt::test_encode_madd_regression_sp
