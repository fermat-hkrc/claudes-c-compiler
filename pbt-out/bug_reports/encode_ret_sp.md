# Bug: encode_ret encodes SP as XZR
**Law:** RET register 31 is XZR, never SP. llvm-mc rejects `ret sp`. ARM ARM Unconditional branch (register) Rn is Xn/XZR.
**Impact:** `ret sp` encodes as `ret xzr` (0xd65f03e0). A stack-pointer operand is accepted and silently rewritten to the zero register.
**Function:** encode_ret
**Detected by:** Algebraic — Negative/Error Contract (4e)
**Minimal input:** operands = [Reg("sp")]  (which=0, n=0)
**Expected:** Err (llvm-mc: invalid operand for instruction)
**Actual:** Ok(Word(0xd65f03e0)) — parse_reg_num maps "sp" to 31, same as xzr
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_ret_pbt::test_encode_ret_regression_sp
**Serial reconfirmation:** reproduced with PBT_TEST_JOBS=1 / --test-threads=1
**Root cause:** parse_reg_num("sp"|"wsp") returns 31; encode_ret does not reject SP.
