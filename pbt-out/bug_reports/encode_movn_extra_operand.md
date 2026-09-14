# Bug: encode_movn ignores extra operands
**Law:** MOVN has the form `Rd, #imm16 [, lsl #N]`; a further operand must be rejected.
**Impact:** Trailing operands are ignored, so typos such as `movn x0, #0, x0` assemble as `movn x0, #0` instead of failing. Gas/llvm-mc reject the extra operand.
**Function:** encode_movn
**Detected by:** Negative/Error Contract — arity
**Minimal input:** `movn x0, #0, x0` (operands `[Reg("x0"), Imm(0), Reg("x0")]`)
**Expected:** Err (llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word) encoding `movn x0, #0` (non-Shift at index 2 sets hw=0)
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::encode_movn_pbt::test_encode_movn_regression_extra_operand
**Serial reconfirmation:** reproduced with PBT_TEST_JOBS=1 (same shrunk witness `rd=0, is_64=true, hw=0, imm=0, extra=Reg("x0")`)
