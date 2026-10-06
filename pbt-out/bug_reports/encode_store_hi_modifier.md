# Bug: encode_store accepts %hi/%pcrel_hi/%tprel_hi on store memory operands
**Law:** A store MemSymbol whose modifier is %hi, %pcrel_hi, or %tprel_hi must return Err. llvm-mc only allows %lo/%pcrel_lo/%tprel_lo on S-type store offsets.
**Impact:** A hi-type modifier is remapped to an S-type lo reloc (Lo12S / PcrelLo12S / TprelLo12S) and a store with imm=0 is emitted. The linker then patches the low 12 S-type bits from a high-part symbol, producing a wrong address.
**Function:** encode_store
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:194
**Detected by:** Negative/Error Contract
**Minimal input:** encode_store([Reg("x0"), MemSymbol { base: "x0", symbol: "%hi(foo)", modifier: "" }], funct3=0)  // sb x0, %hi(foo)(x0); also %got_pcrel_hi(foo) -> GotHi20
**Expected:** Err
**Actual:** Ok(WordWithReloc { word: 0x00000023, reloc_type: Lo12S, symbol: "foo", addend: 0 }) for %hi; Ok(WordWithReloc { reloc_type: GotHi20 }) for %got_pcrel_hi
**Severity:** high
**Root cause:** base.rs:204-209 remaps PcrelHi20/Hi20/TprelHi20 onto the S-type lo reloc kinds instead of rejecting hi modifiers that llvm-mc does not accept on stores.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:204`
```rust
            let reloc_type = match reloc_type {
                RelocType::PcrelHi20 => RelocType::PcrelLo12S,
                RelocType::Hi20 => RelocType::Lo12S,
                RelocType::TprelHi20 => RelocType::TprelLo12S,
                other => other,
            };
```
**Suggested fix:** Return Err for hi-type modifiers; keep only lo-type S relocs.
```rust
            let reloc_type = match reloc_type {
                RelocType::PcrelLo12I => RelocType::PcrelLo12S,
                RelocType::Lo12I => RelocType::Lo12S,
                RelocType::TprelLo12I => RelocType::TprelLo12S,
                RelocType::PcrelLo12S | RelocType::Lo12S | RelocType::TprelLo12S => reloc_type,
                _ => return Err("store: expected %lo/%pcrel_lo/%tprel_lo".to_string()),
            };
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_store_neg_hi_modifier -- --test-threads=1
```
**Raw output:**
```text
Test failed: hi-type modifier %hi(foo) must Err on store (llvm-mc only allows %lo/%pcrel_lo/%tprel_lo); got Ok(WordWithReloc { word: 35, reloc: Relocation { reloc_type: Lo12S, symbol: "foo", addend: 0 } }) at src/backend/riscv/assembler/encoder/encode_store_pbt.rs:604.
minimal failing input: (mn, f3) = (
    "sb",
    0,
), rs2 = "x0", rs1 = "x0", s = "foo", hi = "%hi"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_store_pbt.rs
