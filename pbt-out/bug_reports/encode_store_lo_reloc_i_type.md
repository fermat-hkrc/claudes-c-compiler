# Bug: encode_store emits I-type lo relocs on S-type stores
**Law:** A store of the form `mn rs2, %lo(s)(rs1)` / `%pcrel_lo` / `%tprel_lo` must carry RelocType Lo12S / PcrelLo12S / TprelLo12S. encoder/mod.rs documents those S-type reloc kinds as the SW/SD lo12 patches; llvm-mc emits fixup_riscv_lo12_s / pcrel_lo12_s / tprel_lo12_s.
**Impact:** The ELF writer applies I-type imm[31:20] patching to an S-type instruction, overwriting rs2 and the scattered imm[11:5]|imm[4:0] field. Every relocatable `sb/sh/sw/sd rs, %lo(sym)(base)` (and pcrel/tprel twins) links to a corrupted store.
**Function:** encode_store
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:194
**Detected by:** Algebraic — Invariant
**Minimal input:** encode_store([Reg("x0"), MemSymbol { base: "x0", symbol: "%pcrel_lo(foo)", modifier: "" }], funct3=0)  // sb x0, %pcrel_lo(foo)(x0)
**Expected:** WordWithReloc { reloc_type: PcrelLo12S, symbol: "foo", addend: 0 } (and Lo12S for %lo, TprelLo12S for %tprel_lo)
**Actual:** reloc_type: PcrelLo12I for %pcrel_lo; Lo12I for %lo; TprelLo12I for %tprel_lo
**Severity:** high
**Root cause:** parse_reloc_modifier returns the I-type lo variants (PcrelLo12I/Lo12I/TprelLo12I). base.rs:204-209 only remaps Hi20 kinds onto S-type lo relocs and passes `other => other`, so the valid lo modifiers keep the I-type reloc.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:204`
```rust
            let reloc_type = match reloc_type {
                RelocType::PcrelHi20 => RelocType::PcrelLo12S,
                RelocType::Hi20 => RelocType::Lo12S,
                RelocType::TprelHi20 => RelocType::TprelLo12S,
                other => other,
            };
```
**Suggested fix:** Remap the I-type lo variants that parse_reloc_modifier actually returns for %lo/%pcrel_lo/%tprel_lo.
```rust
            let reloc_type = match reloc_type {
                RelocType::PcrelLo12I | RelocType::PcrelHi20 => RelocType::PcrelLo12S,
                RelocType::Lo12I | RelocType::Hi20 => RelocType::Lo12S,
                RelocType::TprelLo12I | RelocType::TprelHi20 => RelocType::TprelLo12S,
                RelocType::PcrelLo12S | RelocType::Lo12S | RelocType::TprelLo12S => reloc_type,
                _ => return Err("store: expected %lo/%pcrel_lo/%tprel_lo".to_string()),
            };
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_store_reloc_lo -- --test-threads=1
```
**Raw output:**
```text
Test failed: assertion failed: `(left == right)`
  left: `"PcrelLo12I"`,
 right: `"PcrelLo12S"` at src/backend/riscv/assembler/encoder/encode_store_pbt.rs:473.
minimal failing input: (_mn, f3) = (
    "sb",
    0,
), rs2 = "x0", rs1 = "x0", s = "foo"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_store_pbt.rs
