# Bug: encode_alu_imm accepts GOT/TLS/plain symbols as OP-IMM immediates
**Law:** An OP-IMM Symbol that is not %lo/%pcrel_lo/%tprel_lo must return Err. llvm-mc only allows those three modifiers (or a 12-bit integer) on addi/slti/sltiu/xori/ori/andi.
**Impact:** %got_pcrel_hi, TLS hi modifiers, %tprel_add, and a bare symbol are accepted and emitted as WordWithReloc with a non-lo reloc kind (or PcrelLo12I for a bare name). The linker then applies the wrong RISC-V relocation to an I-type immediate.
**Function:** encode_alu_imm
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:223
**Detected by:** Negative/Error Contract
**Minimal input:** encode_alu_imm([Reg("x0"), Reg("x0"), Symbol("%got_pcrel_hi(foo)")], funct3=0)  // addi x0, x0, %got_pcrel_hi(foo)
**Expected:** Err
**Actual:** Ok(WordWithReloc { word: 0x00000013, reloc_type: GotHi20, symbol: "foo", addend: 0 })
**Severity:** high
**Root cause:** base.rs:236 `other => other` keeps GotHi20/TlsGotHi20/TlsGdHi20/TprelAdd, and a plain symbol is classified as PcrelHi20 then remapped to PcrelLo12I.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:236`
```rust
                other => other,
```
**Suggested fix:** Accept only lo-12 I-type modifiers; reject every other reloc kind.
```rust
            let reloc_type = match reloc_type {
                RelocType::PcrelLo12I | RelocType::Lo12I | RelocType::TprelLo12I => reloc_type,
                _ => return Err("alu_imm: expected %lo/%pcrel_lo/%tprel_lo".to_string()),
            };
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_alu_imm_neg_other_modifier -- --test-threads=1
```
**Raw output:**
```text
Test failed: non-lo modifier %got_pcrel_hi(foo) must Err on OP-IMM; got Ok(WordWithReloc { word: 19, reloc: Relocation { reloc_type: GotHi20, symbol: "foo", addend: 0 } }) at src/backend/riscv/assembler/encoder/encode_alu_imm_pbt.rs:634.
minimal failing input: (mn, f3) = (
    "addi",
    0,
), rd = "x0", rs1 = "x0", form = "%got_pcrel_hi(foo)"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_alu_imm_pbt.rs
