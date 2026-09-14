# Bug: encode_movk encodes SP/WSP as XZR/WZR
**Law:** MOVK Rd is a GPR; register 31 is XZR/WZR, never SP/WSP. `movk sp, #imm` / `movk wsp, #imm` must be rejected.
**Impact:** Stack-pointer names assemble as the zero register, silently targeting the wrong architectural register.
**Function:** encode_movk
**Detected by:** Negative/Error Contract — SP as Rd
**Minimal input:** `movk wsp, #0` (operands `[Reg("wsp"), Imm(0)]`)
**Expected:** Err (llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word) with Rd=31 (WZR), because parse_reg_num maps "sp"/"wsp" to 31
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::encode_movk_pbt::test_encode_movk_regression_sp
**Serial reconfirmation:** reproduced with PBT_TEST_JOBS=1 (same shrunk witness `is_64=false, imm=0, hw=0`)
