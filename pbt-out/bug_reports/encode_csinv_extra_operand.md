# Bug: encode_csinv ignores extra operands
**Law:** CSINV takes exactly four operands (Rd, Rn, Rm, cond); encode_csinv(ops ++ [extra]) must be Err for extra ∈ {Reg, Imm, Symbol, Mem}.
**Impact:** A fifth operand (`csinv x0, x1, x2, eq, x3`) is silently dropped and a CSINV word is still emitted. That hides parse/typo errors and diverges from gas/llvm-mc, which reject the instruction.
**Function:** encode_csinv
**Detected by:** Negative/Error Contract
**Minimal input:** [Reg("x0"), Reg("x0"), Reg("x0"), Cond("eq"), Reg("x3")]  (`csinv x0, x0, x0, eq, x3`)
**Expected:** Err
**Actual:** Ok(Word) — the encoder only inspects operands 0..3.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_csinv_pbt::test_encode_csinv_regression_extra_operand
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib encode_csinv_neg -- --test-threads=1 reproduced encode_csinv_neg_extra_operand.
