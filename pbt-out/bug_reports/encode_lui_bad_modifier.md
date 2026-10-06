# Bug: encode_lui accepts plain symbols and non-%hi modifiers as R_RISCV_HI20
**Law:** ∀ rd, ∀ s ∉ {%hi(ident), %tprel_hi(ident)}. encode_lui([Reg(rd), Symbol(s)]) = Err
**Impact:** `lui x0, foo`, `lui x0, %pcrel_hi(foo)`, and `lui x0, %lo(foo)` emit R_RISCV_HI20 against a stripped or raw name. llvm-mc rejects those forms. The linker then applies the wrong reloc kind or looks up a non-existent symbol.
**Function:** encode_lui
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:5
**Detected by:** Negative/Error Contract
**Minimal input:** encode_lui([Reg("x0"), Symbol("foo")])
**Expected:** Err (llvm-mc: operand must be a symbol with %hi/%tprel_hi modifier or an integer in [0, 1048575])
**Actual:** Ok(WordWithReloc { word: 0x37, reloc_type: Hi20, symbol: "foo", addend: 0 })
**Severity:** high
**Root cause:** base.rs:16-20 special-cases only `%tprel_hi(`; every other Symbol, including a bare identifier and `%pcrel_hi`/`%lo`, is classified as Hi20.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:16`
```rust
                    reloc_type: if s.starts_with("%tprel_hi(") {
                        RelocType::TprelHi20
                    } else {
                        RelocType::Hi20
                    },
```
**Suggested fix:** Accept only `%hi(` and `%tprel_hi(`; reject every other symbol form.
```rust
                    reloc_type: if s.starts_with("%tprel_hi(") {
                        RelocType::TprelHi20
                    } else if s.starts_with("%hi(") {
                        RelocType::Hi20
                    } else {
                        return Err("lui: expected %hi/%tprel_hi or integer".to_string());
                    },
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_lui_regression_plain_symbol -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_lui_pbt::test_encode_lui_regression_plain_symbol' panicked at src/backend/riscv/assembler/encoder/encode_lui_pbt.rs:355:5:
lui x0, foo must Err (llvm-mc requires %hi/%tprel_hi); got Ok(WordWithReloc { word: 55, reloc: Relocation { reloc_type: Hi20, symbol: "foo", addend: 0 } })
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_lui_pbt.rs
