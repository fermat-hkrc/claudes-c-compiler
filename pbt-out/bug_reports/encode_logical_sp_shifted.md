# Bug: encode_logical accepts SP/WSP in shifted-register form
**Law:** Logical shifted-register Rd/Rn/Rm encoding 31 is XZR/WZR, never SP/WSP. llvm-mc rejects `and wsp, w0, w0` and `and sp, x0, x1`.
**Impact:** `and sp, x0, x1` is encoded as `and xzr, x0, x1`, silently targeting the zero register instead of SP.
**Function:** encode_logical
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** operands `[Reg("wsp"), Reg("w0"), Reg("w0")]`, opc=0 (`and wsp, w0, w0`)
**Expected:** Err
**Actual:** Ok(Word) — parse_reg_num maps sp/wsp to 31, same as xzr/wzr
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::test_encode_logical_regression_sp_shifted
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1
