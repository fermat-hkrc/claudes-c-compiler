# Bug: encode_bl ignores extra operands
**Law:** BL takes a single target; encode_bl([Symbol(s), extra]) must be Err for extra ∈ {Reg, Imm, Symbol, Mem}.
**Impact:** A second operand (`bl foo, x0`) is silently dropped and a Call26 reloc is still emitted. That hides parse/typo errors and diverges from gas/llvm-mc, which reject the instruction.
**Function:** encode_bl
**Detected by:** Negative/Error Contract
**Minimal input:** [Symbol("labl0"), Reg("x0")]  (`bl labl0, x0`)
**Expected:** Err
**Actual:** Ok(WordWithReloc { word: 0x94000000, reloc: Call26, symbol: "labl0", addend: 0 }) — get_symbol only inspects operand 0.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs `test_encode_bl_regression_extra_operand`
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib encode_bl_pbt -- --test-threads=1 reproduced the failure.
