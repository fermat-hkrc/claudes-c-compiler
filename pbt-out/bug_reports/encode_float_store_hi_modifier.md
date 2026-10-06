# Bug: encode_float_store accepts %hi/%pcrel_hi/%tprel_hi on STORE-FP memory operands
**Law:** A STORE-FP MemSymbol whose modifier is %hi, %pcrel_hi, or %tprel_hi must return Err. llvm-mc only allows %lo/%pcrel_lo/%tprel_lo on S-type store offsets.
**Impact:** A hi-type modifier is remapped to an S-type lo reloc (Lo12S / PcrelLo12S) and a store with imm=0 is emitted. The linker then patches the low 12 S-type bits from a high-part symbol, producing a wrong address.
**Function:** encode_float_store
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:33
**Detected by:** Negative/Error Contract
**Minimal input:** encode_float_store([Reg("f0"), MemSymbol { base: "x0", symbol: "%hi(foo)", modifier: "" }], 0b010)
**Expected:** Err
**Actual:** Ok(WordWithReloc { word: 8231, reloc_type: Lo12S, symbol: "foo", addend: 0 })
**Severity:** high
**Root cause:** float.rs:43-47 remaps PcrelHi20/Hi20 onto the S-type lo reloc kinds instead of rejecting hi modifiers that llvm-mc does not accept on stores. TprelHi20 is left as TprelHi20 (`other => other`) and also accepted.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:43`
```rust
            let reloc_type = match reloc_type {
                RelocType::PcrelHi20 => RelocType::PcrelLo12S,
                RelocType::Hi20 => RelocType::Lo12S,
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
                _ => return Err("float store: expected %lo/%pcrel_lo/%tprel_lo".to_string()),
            };
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_float_store_neg_hi_modifier -- --test-threads=1
cargo test --lib test_encode_float_store_regression_hi_modifier -- --test-threads=1
```
**Raw output:**
```text
Test failed: hi-type modifier %hi(foo) must Err on float store (llvm-mc only allows %lo/%pcrel_lo/%tprel_lo); got Ok(WordWithReloc { word: 8231, reloc: Relocation { reloc_type: Lo12S, symbol: "foo", addend: 0 } }) at src/backend/riscv/assembler/encoder/encode_float_store_pbt.rs:602.
minimal failing input: (mn, f3) = (
    "fsw",
    2,
), rs2 = "f0", rs1 = "x0", s = "foo", hi = "%hi"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_float_store_pbt.rs
