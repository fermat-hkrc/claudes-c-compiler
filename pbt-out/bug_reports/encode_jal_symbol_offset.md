# Bug: encode_jal rejects SymbolOffset instead of emitting R_RISCV_JAL with addend
**Law:** ∀ rd ∈ GPR, ∀ s ∈ ident, ∀ a ∈ i64\{0}. encode_jal([Reg(rd), SymbolOffset(s, a)]) = WordWithReloc { word: jal rd,0; reloc: {Jal, s, addend=a} }
**Impact:** `jal rd, foo+4` (parser Operand::SymbolOffset) fails with "jal: invalid operand". llvm-mc accepts it with fixup value foo+4. Compiler output that uses a symbol plus addend cannot be assembled.
**Function:** encode_jal
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:51
**Detected by:** Differential — llvm-mc JAL reloc with addend
**Minimal input:** encode_jal([Reg("x0"), SymbolOffset("foo", 1)])
**Expected:** Ok(WordWithReloc { word: 0x0000006f, reloc: {Jal, "foo", addend: 1} })
**Actual:** Err("jal: invalid operand")
**Severity:** medium
**Root cause:** base.rs:73 matches Imm / Symbol / Label / Reg but not SymbolOffset, so the `_` arm returns Err. Relocation.addend is documented and the parser emits SymbolOffset for `sym+N`.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:87`
```rust
            _ => Err("jal: invalid operand".to_string()),
```
**Suggested fix:** Treat SymbolOffset like Symbol, preserving the addend.
```rust
            Operand::SymbolOffset(s, add) => {
                Ok(EncodeResult::WordWithReloc {
                    word: encode_j(OP_JAL, rd, 0),
                    reloc: Relocation {
                        reloc_type: RelocType::Jal,
                        symbol: s.clone(),
                        addend: *add,
                    },
                })
            }
            _ => Err("jal: invalid operand".to_string()),
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_jal_regression_symbol_offset -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_jal_pbt::test_encode_jal_regression_symbol_offset' panicked at src/backend/riscv/assembler/encoder/encode_jal_pbt.rs:365:18:
expected WordWithReloc for foo+4, got Err("jal: invalid operand")
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_jal_pbt.rs
