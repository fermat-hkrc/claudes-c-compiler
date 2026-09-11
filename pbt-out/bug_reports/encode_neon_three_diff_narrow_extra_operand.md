# Bug: encode_neon_three_diff_narrow silently ignores a fourth operand
**Law:** ARM ADDHN/SUBHN have a three-register form. `encode_neon_three_diff_narrow` with four operands must return Err, matching llvm-mc / GNU as which reject a trailing fourth register.
**Impact:** Source text `addhn v0.8b, v0.8h, v0.8h, v0.8h` is assembled as the three-register `addhn v0.8b, v0.8h, v0.8h` instead of being rejected. The assembled object silently disagrees with the written instruction.
**Function:** encode_neon_three_diff_narrow
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** operands = [Vd.8b, Vn.8h, Vm.8h, Vextra.8h], u_bit=0, opcode=0b0100, is_high=false
**Expected:** Err (too many operands; llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word) — only `operands.len() < 3` is checked; extra operands are ignored.
**Severity:** medium
**Regression test:** `test_encode_neon_three_diff_narrow_regression_extra_operand` in src/backend/arm/assembler/encoder/neon.rs
**Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_neon_three_diff_narrow_extra_operand_err -- --test-threads=1`
