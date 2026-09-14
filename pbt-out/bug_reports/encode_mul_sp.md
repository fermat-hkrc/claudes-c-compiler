# Bug: encode_mul encodes SP/WSP as XZR/WZR
**Law:** MUL encoding uses register 31 as WZR/XZR, never WSP/SP. `mul sp, ...` / `mul wsp, ...` must be rejected (gas/llvm-mc "invalid operand").
**Impact:** `mul wsp, w0, w0` is assembled as `mul wzr, w0, w0`. A stack-pointer operand is silently rewritten to the zero register, producing wrong object code instead of an assembler error.
**Function:** encode_mul
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `encode_mul([Reg("wsp"), Reg("w0"), Reg("w0")])` i.e. `mul wsp, w0, w0`
**Expected:** Err
**Actual:** Ok(Word) encoding register 31 as WZR. Reproduced serially (`PBT_TEST_JOBS=1`).
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::encode_mul_pbt::test_encode_mul_regression_sp
