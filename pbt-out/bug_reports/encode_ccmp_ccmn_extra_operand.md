# Bug: encode_ccmp_ccmn ignores extra operands
**Law:** CCMP/CCMN takes exactly four operands (Rn, Rm-or-imm5, nzcv, cond); encode_ccmp_ccmn(ops ++ [extra]) must be Err for extra ∈ {Reg, Imm, Symbol, Mem, Cond, Shift}.
**Impact:** A fifth operand (`ccmn x0, #0, #0, eq, x1`) is silently dropped and a CCMP/CCMN word is still emitted. That hides parse/typo errors and diverges from gas/llvm-mc, which reject the instruction.
**Function:** encode_ccmp_ccmn
**Detected by:** Negative/Error Contract
**Minimal input:** [Reg("x0"), Imm(0), Imm(0), Cond("eq"), Reg("x1")], is_ccmp=false  (`ccmn x0, #0, #0, eq, x1`)
**Expected:** Err
**Actual:** Ok(Word(0xba400800)) — the encoder only inspects operands 0..3.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_ccmp_ccmn_pbt::test_encode_ccmp_ccmn_regression_extra_operand
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib encode_ccmp_ccmn_neg_extra_operand -- --test-threads=1 reproduced the failure.
