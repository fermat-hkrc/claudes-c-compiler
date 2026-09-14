# Bug: encode_cinc ignores extra operands
**Law:** CINC takes exactly three operands (Rd, Rn, cond); encode_cinc(ops ++ [extra]) must be Err for extra ∈ {Reg, Imm, Symbol, Mem}.
**Impact:** A fourth operand (`cinc x0, x0, eq, x2`) is silently dropped and a CINC word is still emitted. That hides parse/typo errors and diverges from gas/llvm-mc, which reject the instruction.
**Function:** encode_cinc
**Detected by:** Negative/Error Contract
**Minimal input:** [Reg("x0"), Reg("x0"), Cond("eq"), Reg("x2")]  (`cinc x0, x0, eq, x2`)
**Expected:** Err
**Actual:** Ok(Word) — the encoder only inspects operands 0..2.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_cinc_pbt::test_encode_cinc_regression_extra_operand
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib encode_cinc_pbt -- --test-threads=1 reproduced encode_cinc_neg_extra_operand.
