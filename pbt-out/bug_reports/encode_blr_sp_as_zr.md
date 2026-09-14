# Bug: encode_blr treats SP as XZR
**Law:** ARM BLR Rn is Xn; register 31 is XZR, not SP. llvm-mc rejects `blr sp`. encode_blr must Err when Rn is sp.
**Impact:** `blr sp` is assembled as `blr xzr` (encoding 0xd63f03e0). The object file writes an indirect call through XZR instead of rejecting the stack pointer.
**Function:** encode_blr
**Detected by:** Algebraic — Negative/Error Contract (4e)
**Minimal input:** operands = [Reg("sp")]
**Expected:** Err (invalid operand; SP is not a valid BLR register)
**Actual:** Ok(Word(0xd63f03e0)) — same encoding as `blr xzr`. parse_reg_num maps sp to 31.
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_blr_pbt::test_encode_blr_regression_sp
**Serial reconfirmation:** reproduced with PBT_TEST_JOBS=1 (cargo test --lib encode_blr_neg_wrong_reg_class -- --test-threads=1)
