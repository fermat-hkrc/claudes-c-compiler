# Bug: encode_cmp ignores extra operands
**Law:** CMP takes two operands (Rn, Rm|#imm) plus an optional shift/extend; encode_cmp(ops ++ [extra]) must be Err for extra ∈ {Reg, Imm, Symbol, Mem}.
**Impact:** A third register (`cmp x0, x0, x2`) is silently dropped and a CMP word is still emitted. That hides parse/typo errors and diverges from gas/llvm-mc, which reject the instruction.
**Function:** encode_cmp
**Detected by:** Negative/Error Contract
**Minimal input:** [Reg("x0"), Reg("x0"), Reg("x2")]  (`cmp x0, x0, x2`)
**Expected:** Err
**Actual:** Ok(Word) encoding `cmp x0, x0` — encode_add_sub only treats operand 3 as Shift/Extend and otherwise ignores it.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_cmp_pbt::test_encode_cmp_regression_extra_operand
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib encode_cmp_neg_ -- --test-threads=1 reproduced encode_cmp_neg_extra_operand (shrunk CE pair=("x0","x0"), which=0).
