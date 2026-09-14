# Bug: encode_movz ignores extra operands after optional lsl
**Law:** MOVZ accepts exactly Rd + imm16 + optional lsl; an extra operand must be rejected.
**Impact:** Trailing registers, immediates, or extra shifts are dropped, so invalid GNU as is assembled as a shorter (legal-looking) MOVZ.
**Function:** encode_movz
**Detected by:** Negative/Error Contract — extra operand
**Minimal input:** `movz x0, #0, x0` (operands `[Reg("x0"), Imm(0), Reg("x0")]`)
**Expected:** Err (llvm-mc: "expected 'lsl' with optional integer 0, 16, 32 or 48")
**Actual:** Ok(Word) encoding `movz x0, #0` (third operand is not Shift, so hw=0)
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::encode_movz_pbt::test_encode_movz_regression_extra_operand
**Serial reconfirmation:** reproduced with PBT_TEST_JOBS=1 (same shrunk witness `rd=0, is_64=true, hw=0, imm=0, extra=Reg("x0")`)
