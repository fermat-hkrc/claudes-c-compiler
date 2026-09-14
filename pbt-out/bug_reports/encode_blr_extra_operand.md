# Bug: encode_blr ignores extra operands
**Law:** BLR takes a single Xn operand; encode_blr([Reg(xn), extra]) must be Err for extra ∈ {Reg, Imm, Symbol, Mem}.
**Impact:** A second operand (`blr x0, x1`) is silently dropped and a BLR word is still emitted. That hides parse/typo errors and diverges from gas/llvm-mc, which reject the instruction.
**Function:** encode_blr
**Detected by:** Negative/Error Contract
**Minimal input:** [Reg("x0"), Reg("x1")]  (`blr x0, x1`)
**Expected:** Err
**Actual:** Ok(Word(0xd63f0000)) — get_reg only inspects operand 0.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_blr_pbt::test_encode_blr_regression_extra_operand
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib encode_blr_neg_extra_operand -- --test-threads=1 reproduced the failure.
