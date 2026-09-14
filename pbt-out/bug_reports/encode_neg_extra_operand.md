# Bug: encode_neg silently ignores extra operands
**Law:** RISC-V `neg` is a two-operand pseudoinstruction (`neg rd, rs`). `encode_neg([rd, rs, extra])` must return Err, matching llvm-mc which rejects `neg rd, rs, extra`.
**Impact:** Source text `neg zero, zero, zero` is assembled as `neg zero, zero` (`sub x0, x0, x0`) instead of being rejected. The assembled object silently disagrees with the written instruction.
**Function:** encode_neg
**Detected by:** Algebraic — Negative/Error Contract (4e)
**Minimal input:** operands = [Reg("zero"), Reg("zero"), Reg("zero")]
**Expected:** Err (invalid operand; documented form is `neg rd, rs`; llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word(0x40000033)) — extra operand ignored; encodes as `neg zero, zero`
**Severity:** medium
**Regression test:** src/backend/riscv/assembler/encoder/pseudo.rs::encode_neg_pbt::test_encode_neg_regression_extra_operand
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 (`cargo test --lib encode_neg_neg_extra -- --test-threads=1`)
