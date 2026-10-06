# Bug: encode_load rejects ld rd, symbol+addend
**Law:** `mn rd, symbol+addend` (Operand::SymbolOffset) must expand like the documented bare-symbol pseudo (`auipc rd, %pcrel_hi(symbol)` + `mn rd, 0(rd)`) with the addend preserved on the HI20 reloc. llvm-mc accepts `ld x1, foo+4` as that pair.
**Impact:** Valid `ld rd, foo+4` (and lb/lh/lw/lbu/lhu/lwu) from codegen or handwritten asm fails with "load: expected memory operand" instead of emitting the PC-relative load pair, so objects that llvm-mc would assemble are rejected.
**Function:** encode_load
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:147
**Detected by:** Differential (llvm-mc) — bare-symbol expansion with addend
**Minimal input:** encode_load([Reg("x0"), SymbolOffset("foo", 1)], funct3=0)  // lb x0, foo+1
**Expected:** Ok(WordsWithRelocs) of auipc x0 / lb x0, 0(x0) with PcrelHi20 addend=1 on "foo"
**Actual:** Err("load: expected memory operand")
**Severity:** medium
**Root cause:** base.rs:176 matches only Symbol and Label; SymbolOffset falls through to the default error at base.rs:190.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:190`
```rust
        _ => Err("load: expected memory operand".to_string()),
```
**Suggested fix:** Handle SymbolOffset like Symbol, copying the addend onto the AUIPC reloc.
```rust
        Some(Operand::SymbolOffset(s, addend)) => {
            Ok(EncodeResult::WordsWithRelocs(vec![
                (encode_u(OP_AUIPC, rd, 0), Some(Relocation {
                    reloc_type: RelocType::PcrelHi20,
                    symbol: s.clone(),
                    addend: *addend,
                })),
                (encode_i(OP_LOAD, rd, funct3, rd, 0), Some(Relocation {
                    reloc_type: RelocType::PcrelLo12I,
                    symbol: s.clone(),
                    addend: 0,
                })),
            ]))
        }
        _ => Err("load: expected memory operand".to_string()),
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_load_symbol_offset_addend -- --test-threads=1
```
**Raw output:**
```text
Test failed: expected WordsWithRelocs for lb x0, foo+1, got Err("load: expected memory operand").
minimal failing input: (mn, f3) = (
    "lb",
    0,
), rd = "x0", s = "foo", addend = 1
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_load_pbt.rs
