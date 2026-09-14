# Bug: encode_msub treats SP/WSP as XZR/WZR
**Law:** ARM MSUB encoding uses register 31 as WZR/XZR, not WSP/SP. llvm-mc rejects `msub wsp, w0, w0, w0` and `msub sp, x0, x1, x2`. encode_msub must Err when any of Rd/Rn/Rm/Ra is sp/wsp.
**Impact:** `msub wsp, w0, w0, w0` is assembled as `msub wzr, w0, w0, w0`. The object file contains a different instruction than the source text — writes WZR instead of addressing the stack pointer.
**Function:** encode_msub
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** operands = [Reg("wsp"), Reg("w0"), Reg("w0"), Reg("w0")]
**Expected:** Err (invalid operand; SP is not a valid MSUB register)
**Actual:** Ok(Word) — parse_reg_num maps sp/wsp to 31, same encoding as `msub wzr, w0, w0, w0`. Reproduced serially (`PBT_TEST_JOBS=1`).
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::encode_msub_pbt::test_encode_msub_regression_sp
