# Bug: encode_ret ignores extra operands
**Law:** RET takes at most one Xn; a second operand must be rejected (GNU as / llvm-mc / ARM ARM).
**Impact:** `ret x0, x1` (and any extra Imm/Symbol/Mem) is encoded as `ret x0` instead of an assembler error, so malformed assembly silently produces a valid return.
**Function:** encode_ret
**Detected by:** Algebraic — Negative/Error Contract (4e)
**Minimal input:** operands = [Reg("x0"), Reg("x1")]  (n=0, which=0)
**Expected:** Err (llvm-mc: invalid operand)
**Actual:** Ok(Word(0xd65f0000)) — extra operand ignored; only operands[0] is used
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_ret_pbt::test_encode_ret_regression_extra_operand
**Serial reconfirmation:** reproduced with PBT_TEST_JOBS=1 / --test-threads=1
**Root cause:** encode_ret only inspects emptiness vs operands[0]; it never checks operands.len() <= 1.
