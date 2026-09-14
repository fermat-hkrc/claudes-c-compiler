# Bug: encode_movz encodes SP/WSP as XZR/WZR
**Law:** MOVZ register 31 is XZR/WZR, never SP/WSP; `movz sp` / `movz wsp` must be rejected.
**Impact:** `movz wsp, #0` is accepted and encoded as `movz wzr, #0`, so a stack-pointer destination silently becomes the zero register.
**Function:** encode_movz
**Detected by:** Negative/Error Contract — SP as Rd
**Minimal input:** `movz wsp, #0` (operands `[Reg("wsp"), Imm(0)]`)
**Expected:** Err (llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word) encoding 32-bit MOVZ with Rd=31 (parse_reg_num maps wsp -> 31)
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::encode_movz_pbt::test_encode_movz_regression_sp
**Serial reconfirmation:** reproduced with PBT_TEST_JOBS=1 (same shrunk witness `is_64=false, imm=0, hw=0`)
