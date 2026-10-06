# Bug: encode_load accepts %hi/%pcrel_hi/%tprel_hi as Lo12I
**Law:** A load memory operand with a hi-type modifier (%hi, %pcrel_hi, %tprel_hi) must return Err. llvm-mc only allows %lo/%pcrel_lo/%tprel_lo (or a 12-bit integer) on lb/lh/lw/ld/lbu/lhu/lwu.
**Impact:** `ld rd, %hi(sym)(rs1)` is encoded as a load with R_RISCV_LO12_I on `sym`, i.e. the hi modifier is silently treated as %lo. Link-time relocation then patches the low 12 bits of a symbol that the source asked to take the high 20 bits of.
**Function:** encode_load
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:147
**Detected by:** Negative/Error Contract
**Minimal input:** encode_load([Reg("x0"), MemSymbol { base: "x0", symbol: "%hi(foo)", modifier: "" }], funct3=0)
**Expected:** Err
**Actual:** Ok(WordWithReloc { word: 0x00000003, reloc_type: Lo12I, symbol: "foo", addend: 0 })
**Severity:** medium
**Root cause:** base.rs:160-164 remaps Hi20/PcrelHi20/TprelHi20 to the corresponding Lo12I variant instead of rejecting hi-type modifiers on I-type loads.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:161`
```rust
                RelocType::Hi20 => RelocType::Lo12I,
```
**Suggested fix:** Accept only lo-type modifiers on loads; return Err for hi-type ones.
```rust
            let reloc_type = match reloc_type {
                RelocType::PcrelLo12I | RelocType::Lo12I | RelocType::TprelLo12I => reloc_type,
                _ => return Err("load: expected %lo/%pcrel_lo/%tprel_lo".to_string()),
            };
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_load_neg_hi_modifier -- --test-threads=1
```
**Raw output:**
```text
Test failed: hi-type modifier %hi(foo) must Err on load (llvm-mc only allows %lo/%pcrel_lo/%tprel_lo); got Ok(WordWithReloc { word: 3, reloc: Relocation { reloc_type: Lo12I, symbol: "foo", addend: 0 } }) at src/backend/riscv/assembler/encoder/encode_load_pbt.rs:729.
minimal failing input: (mn, f3) = (
    "lb",
    0,
), rd = "x0", rs1 = "x0", s = "foo", hi = "%hi"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_load_pbt.rs
