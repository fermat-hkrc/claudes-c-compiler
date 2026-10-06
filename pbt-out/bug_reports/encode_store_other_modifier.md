# Bug: encode_store accepts GOT/TLS/plain MemSymbol modifiers
**Law:** A store MemSymbol whose modifier is not %lo/%pcrel_lo/%tprel_lo must return Err. llvm-mc only allows those three lo modifiers on S-type store offsets.
**Impact:** %got_pcrel_hi (and TLS / tprel_add / a plain symbol) is accepted with reloc_type GotHi20 (or TlsGotHi20 / TlsGdHi20 / TprelAdd / PcrelLo12S) on an S-type store. The linker then patches the wrong reloc class.
**Function:** encode_store
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:194
**Detected by:** Negative/Error Contract
**Minimal input:** encode_store([Reg("x0"), MemSymbol { base: "x0", symbol: "%got_pcrel_hi(foo)", modifier: "" }], funct3=0)
**Expected:** Err
**Actual:** Ok(WordWithReloc { word: 35, reloc_type: GotHi20, symbol: "foo", addend: 0 })
**Severity:** high
**Root cause:** base.rs:204-209 passes unmatched reloc kinds through `other => other`, so GotHi20 from parse_reloc_modifier is stored on the S-type instruction.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:208`
```rust
                other => other,
```
**Suggested fix:** Reject non-lo modifiers.
```rust
                _ => return Err("store: expected %lo/%pcrel_lo/%tprel_lo".to_string()),
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_store_neg_other_modifier -- --test-threads=1
```
**Raw output:**
```text
Test failed: non-lo modifier %got_pcrel_hi(foo) must Err on store; got Ok(WordWithReloc { word: 35, reloc: Relocation { reloc_type: GotHi20, symbol: "foo", addend: 0 } }) at src/backend/riscv/assembler/encoder/encode_store_pbt.rs:641.
minimal failing input: (mn, f3) = (
    "sb",
    0,
), rs2 = "x0", rs1 = "x0", form = "%got_pcrel_hi(foo)"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_store_pbt.rs
