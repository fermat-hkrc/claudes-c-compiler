# Bug: encode_br ignores extra operands
**Law:** BR takes a single Xn operand; encode_br([Reg(xn), extra]) must be Err for extra ∈ {Reg, Imm, Symbol, Mem}.
**Impact:** A second operand (`br x0, x1`) is silently dropped and a BR word is still emitted. That hides parse/typo errors and diverges from gas/llvm-mc, which reject the instruction.
**Function:** encode_br
**Detected by:** Negative/Error Contract
**Minimal input:** [Reg("x0"), Reg("x1")]  (`br x0, x1`)
**Expected:** Err
**Actual:** Ok(Word(0xd61f0000)) — get_reg only inspects operand 0.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_br_pbt::test_encode_br_regression_extra_operand
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib encode_br_neg_extra_operand -- --test-threads=1 reproduced the failure.
