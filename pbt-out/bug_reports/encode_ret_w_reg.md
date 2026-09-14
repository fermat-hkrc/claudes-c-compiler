# Bug: encode_ret accepts W-form Rn
**Law:** RET Rn is a 64-bit GPR (Xn / XZR / LR); W-form registers must be rejected (ARM ARM Unconditional branch (register); llvm-mc rejects `ret w0`).
**Impact:** `ret w0` encodes as `ret x0` (0xd65f0000). Width is discarded by get_reg; the 32-bit name is silently treated as the matching X register.
**Function:** encode_ret
**Detected by:** Algebraic — Negative/Error Contract (4e)
**Minimal input:** operands = [Reg("w0")]  (n=0)
**Expected:** Err (llvm-mc: invalid operand for instruction)
**Actual:** Ok(Word(0xd65f0000)) — same encoding as `ret x0`
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_ret_pbt::test_encode_ret_regression_w_reg
**Serial reconfirmation:** reproduced with PBT_TEST_JOBS=1 / --test-threads=1
**Root cause:** get_reg returns (num, is_64) but encode_ret uses only .0; is_32bit_reg is never checked.
