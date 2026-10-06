# Bug: encode_alu_imm accepts %hi/%pcrel_hi/%tprel_hi on OP-IMM immediates
**Law:** An OP-IMM Symbol whose modifier is %hi, %pcrel_hi, or %tprel_hi must return Err. llvm-mc only allows %lo/%pcrel_lo/%tprel_lo on I-type OP-IMM immediates.
**Impact:** A hi-type modifier is remapped to an I-type lo reloc and an OP-IMM word with imm=0 is emitted. The linker then patches the low 12 I-type bits from a high-part symbol, producing a wrong immediate.
**Function:** encode_alu_imm
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:223
**Detected by:** Negative/Error Contract
**Minimal input:** encode_alu_imm([Reg("x0"), Reg("x0"), Symbol("%hi(foo)")], funct3=0)  // addi x0, x0, %hi(foo)
**Expected:** Err
**Actual:** Ok(WordWithReloc { word: 0x00000013, reloc_type: Lo12I, symbol: "foo", addend: 0 })
**Severity:** high
**Root cause:** base.rs:232-237 remaps PcrelHi20/Hi20/TprelHi20 onto the I-type lo reloc kinds instead of rejecting hi modifiers that llvm-mc does not accept on OP-IMM.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:232`
```rust
            let reloc_type = match reloc_type {
                RelocType::PcrelHi20 => RelocType::PcrelLo12I,
                RelocType::Hi20 => RelocType::Lo12I,
                RelocType::TprelHi20 => RelocType::TprelLo12I,
                other => other,
            };
```
**Suggested fix:** Accept only lo-12 I-type modifiers; reject hi/GOT/TLS/plain forms.
```rust
            let reloc_type = match reloc_type {
                RelocType::PcrelLo12I | RelocType::Lo12I | RelocType::TprelLo12I => reloc_type,
                _ => return Err("alu_imm: expected %lo/%pcrel_lo/%tprel_lo".to_string()),
            };
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_alu_imm_neg_hi_modifier -- --test-threads=1
```
**Raw output:**
```text
Test failed: hi-type modifier %hi(foo) must Err on OP-IMM (llvm-mc only allows %lo/%pcrel_lo/%tprel_lo); got Ok(WordWithReloc { word: 19, reloc: Relocation { reloc_type: Lo12I, symbol: "foo", addend: 0 } }) at src/backend/riscv/assembler/encoder/encode_alu_imm_pbt.rs:589.
minimal failing input: (mn, f3) = (
    "addi",
    0,
), rd = "x0", rs1 = "x0", s = "foo", hi = "%hi"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_alu_imm_pbt.rs
