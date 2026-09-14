# Bug: encode_br treats SP as XZR
**Law:** ARM BR Rn is Xn; register 31 is XZR, not SP. llvm-mc rejects `br sp`. encode_br must Err when Rn is sp.
**Impact:** `br sp` is assembled as `br xzr` (encoding 0xd61f03e0). The object file writes an indirect jump through XZR instead of rejecting the stack pointer.
**Function:** encode_br
**Detected by:** Algebraic — Negative/Error Contract (4e)
**Minimal input:** operands = [Reg("sp")]
**Expected:** Err (invalid operand; SP is not a valid BR register)
**Actual:** Ok(Word(0xd61f03e0)) — same encoding as `br xzr`. parse_reg_num maps sp to 31.
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_br_pbt::test_encode_br_regression_sp
**Serial reconfirmation:** reproduced with PBT_TEST_JOBS=1 (cargo test --lib encode_br_neg_wrong_reg_class -- --test-threads=1)
