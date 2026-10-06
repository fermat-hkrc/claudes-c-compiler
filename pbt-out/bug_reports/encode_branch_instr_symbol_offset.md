# Bug: encode_branch_instr rejects SymbolOffset (symbol+addend) as a branch target
**Law:** ∀ rs1, rs2 ∈ GPR, ∀ s ∈ ident, ∀ addend ∈ ℤ\{0}. encode_branch_instr([Reg(rs1), Reg(rs2), SymbolOffset(s, addend)], 0) = WordWithReloc{word=encode(beq rs1, rs2, 0), reloc_type=Branch, symbol=s, addend=addend}
**Impact:** Valid textual assembly `beq rs1, rs2, foo+N` (llvm-mc emits a branch fixup with value foo+N) cannot be encoded. The parser's SymbolOffset operand is dropped on the floor.
**Function:** encode_branch_instr
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:125
**Detected by:** Differential
**Minimal input:** encode_branch_instr([Reg("x0"), Reg("x0"), SymbolOffset("foo", 1)], 0)
**Expected:** Ok(WordWithReloc { word: 0x00000063, reloc: Relocation { reloc_type: Branch, symbol: "foo", addend: 1 } })
**Actual:** Err("branch: expected offset or label as 3rd operand")
**Severity:** medium
**Root cause:** base.rs:133 matches only Symbol, Label, and Reg; SymbolOffset falls through to the error arm at base.rs:143. Reloc.addend is hardcoded 0 even on the arms that do relocate.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:133`
```rust
        Some(Operand::Symbol(s)) | Some(Operand::Label(s)) | Some(Operand::Reg(s)) => {
```
**Suggested fix:** Accept SymbolOffset as a reloc target and preserve the addend.
```rust
        Some(Operand::SymbolOffset(s, addend)) => {
            Ok(EncodeResult::WordWithReloc {
                word: encode_b(OP_BRANCH, funct3, rs1, rs2, 0),
                reloc: Relocation {
                    reloc_type: RelocType::Branch,
                    symbol: s.clone(),
                    addend: *addend,
                },
            })
        }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_branch_instr_regression_symbol_offset -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_branch_instr_pbt::test_encode_branch_instr_regression_symbol_offset' (2737316) panicked at src/backend/riscv/assembler/encoder/encode_branch_instr_pbt.rs:403:18:
expected WordWithReloc for foo+4, got Err("branch: expected offset or label as 3rd operand")
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_branch_instr_pbt.rs
