# Bug: encode_csel ignores extra operands
**Law:** CSEL takes exactly four operands (Rd, Rn, Rm, cond); encode_csel(ops ++ [extra]) must be Err for extra ∈ {Reg, Imm, Symbol, Mem}.
**Impact:** A fifth operand (`csel x0, x1, x2, eq, x3`) is silently dropped and a CSEL word is still emitted. That hides parse/typo errors and diverges from gas/llvm-mc, which reject the instruction.
**Function:** encode_csel
**Detected by:** Negative/Error Contract
**Minimal input:** [Reg("x0"), Reg("x1"), Reg("x2"), Cond("eq"), Reg("x3")]  (`csel x0, x1, x2, eq, x3`)
**Expected:** Err
**Actual:** Ok(Word) — the encoder only inspects operands 0..3.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_csel_pbt::test_encode_csel_regression_extra_operand
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib encode_csel_neg -- --test-threads=1 reproduced encode_csel_neg_extra_operand.
