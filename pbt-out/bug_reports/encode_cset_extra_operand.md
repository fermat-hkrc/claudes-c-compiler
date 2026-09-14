# Bug: encode_cset ignores extra operands
**Law:** CSET takes exactly two operands (Rd, cond); encode_cset([Reg(rd), Cond(c), extra]) must be Err for extra ∈ {Reg, Imm, Symbol, Mem}.
**Impact:** A third operand (`cset x0, eq, x2`) is silently dropped and a CSET word is still emitted. That hides parse/typo errors and diverges from gas/llvm-mc, which reject the instruction.
**Function:** encode_cset
**Detected by:** Negative/Error Contract
**Minimal input:** [Reg("x0"), Cond("eq"), Reg("x2")]  (`cset x0, eq, x2`)
**Expected:** Err
**Actual:** Ok(Word(0x9a9f17e0)) — the encoder only inspects operands 0 and 1.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_cset_pbt::test_encode_cset_regression_extra_operand
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib encode_cset_pbt -- --test-threads=1 reproduced encode_cset_neg_extra_operand with rd="x0", cond="eq", which=0.
