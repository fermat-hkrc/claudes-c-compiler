# Bug: encode_logical accepts out-of-range shift amounts
**Law:** 32-bit logical shifted-register imm6 is in [0, 31] (imm6[5] must be 0). 64-bit is [0, 63]. llvm-mc rejects `and w0, w0, w0, lsl #32`.
**Impact:** Shift amounts are masked with 0x3F, so `#32` on a W register encodes reserved imm6[5]=1 and `#64` on an X register wraps to `#0`.
**Function:** encode_logical
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** operands `[Reg("w0"), Reg("w0"), Reg("w0"), Shift{lsl, 32}]`, opc=0 (`and w0, w0, w0, lsl #32`)
**Expected:** Err
**Actual:** Ok(Word) — `(shift_amount & 0x3F) << 10` with no range check
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::test_encode_logical_regression_shift_oob
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1
