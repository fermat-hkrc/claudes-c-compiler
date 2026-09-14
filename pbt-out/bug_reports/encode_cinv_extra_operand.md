# Bug: encode_cinv ignores extra operands
**Law:** CINV takes exactly three operands (Rd, Rn, cond); encode_cinv(ops ++ [extra]) must be Err for extra ∈ {Reg, Imm, Symbol, Mem}.
**Impact:** A fourth operand (`cinv x0, x0, eq, x2`) is silently dropped and a CINV word is still emitted. That hides parse/typo errors and diverges from gas/llvm-mc, which reject the instruction.
**Function:** encode_cinv
**Detected by:** Negative/Error Contract
**Minimal input:** [Reg("x0"), Reg("x0"), Cond("eq"), Reg("x2")]  (`cinv x0, x0, eq, x2`)
**Expected:** Err
**Actual:** Ok(Word) — the encoder only inspects operands 0..2.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_cinv_pbt::test_encode_cinv_regression_extra_operand
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib encode_cinv_neg -- --test-threads=1 reproduced encode_cinv_neg_extra_operand.
