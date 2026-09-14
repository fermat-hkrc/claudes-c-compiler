# Bug: encode_cbz ignores extra operands
**Law:** CBZ/CBNZ take a register and a target; encode_cbz([Reg(rt), Symbol(s), extra], is_nz) must be Err for extra ∈ {Reg, Imm, Symbol, Mem}.
**Impact:** A third operand (`cbz x0, labl0, x1`) is silently dropped and a CondBr19 reloc is still emitted. That hides parse/typo errors and diverges from gas/llvm-mc, which reject the instruction.
**Function:** encode_cbz
**Detected by:** Negative/Error Contract
**Minimal input:** [Reg("x0"), Symbol("labl0"), Reg("x1")], is_nz=false  (`cbz x0, labl0, x1`)
**Expected:** Err
**Actual:** Ok(WordWithReloc { word: 0xb4000000, reloc: CondBr19, symbol: "labl0", addend: 0 }) — get_reg/get_symbol only inspect operands 0 and 1.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs `test_encode_cbz_regression_extra_operand`
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib encode_cbz -- --test-threads=1 reproduced the failure.
