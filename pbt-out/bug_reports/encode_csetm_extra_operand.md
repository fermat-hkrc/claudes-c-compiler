# Bug: encode_csetm ignores extra operands
**Law:** CSETM takes exactly two operands (Rd, cond); encode_csetm([Reg(rd), Cond(c), extra]) must be Err for extra ∈ {Reg, Imm, Symbol, Mem}.
**Impact:** A third operand (`csetm x0, eq, x2`) is silently dropped and a CSETM word is still emitted. That hides parse/typo errors and diverges from gas/llvm-mc, which reject the instruction.
**Function:** encode_csetm
**Detected by:** Negative/Error Contract
**Minimal input:** [Reg("x0"), Cond("eq"), Reg("x2")]  (`csetm x0, eq, x2`)
**Expected:** Err
**Actual:** Ok(Word(0xda9f13e0)) — the encoder only inspects operands 0 and 1.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_csetm_pbt::test_encode_csetm_regression_extra_operand
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib encode_csetm_neg -- --test-threads=1 reproduced encode_csetm_neg_extra_operand with rd="x0", cond="eq", which=0.
