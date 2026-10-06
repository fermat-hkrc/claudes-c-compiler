# Bug: encode_auipc accepts plain symbols and non-AUIPC reloc modifiers
**Law:** ∀ rd, ∀ s ∉ {%pcrel_hi(ident), %got_pcrel_hi(ident), %tls_ie_pcrel_hi(ident), %tls_gd_pcrel_hi(ident)}. encode_auipc([Reg(rd), Symbol(s)]) = Err
**Impact:** `auipc x0, foo` and `auipc x0, %hi(foo)` encode as a PC-relative (or absolute-hi) relocation instead of being rejected. llvm-mc requires `%pcrel_hi` / `%got_pcrel_hi` / `%tls_ie_pcrel_hi` / `%tls_gd_pcrel_hi`. A bare symbol becomes R_RISCV_PCREL_HI20 against `foo`; `%hi` becomes R_RISCV_HI20 on an AUIPC, which the linker will treat as the wrong reloc kind.
**Function:** encode_auipc
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:30
**Detected by:** Negative/Error Contract
**Minimal input:** encode_auipc([Reg("x0"), Symbol("foo")])
**Expected:** Err (llvm-mc: operand must be a symbol with a %pcrel_hi/%got_pcrel_hi/%tls_ie_pcrel_hi/%tls_gd_pcrel_hi modifier)
**Actual:** Ok(WordWithReloc { word: 0x00000017, reloc_type: PcrelHi20, symbol: "foo", addend: 0 })
**Severity:** high
**Root cause:** base.rs:36-37 forwards every Symbol through parse_reloc_modifier, whose else branch treats a plain name as PcrelHi20 and which also maps %hi/%lo/%pcrel_lo/%tprel_* — modifiers llvm-mc rejects on AUIPC.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:36`
```rust
        Some(Operand::Symbol(s)) => {
            let (reloc_type, symbol) = parse_reloc_modifier(s);
```
**Suggested fix:** Accept only the four AUIPC-valid modifiers; Err on anything else.
```rust
        Some(Operand::Symbol(s)) => {
            let (reloc_type, symbol) = parse_reloc_modifier(s);
            match reloc_type {
                RelocType::PcrelHi20 | RelocType::GotHi20
                | RelocType::TlsGotHi20 | RelocType::TlsGdHi20
                    if s.starts_with('%') => {}
                _ => return Err("auipc: invalid operands".to_string()),
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_auipc_regression_plain_symbol -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_auipc_pbt::test_encode_auipc_regression_plain_symbol' panicked at src/backend/riscv/assembler/encoder/encode_auipc_pbt.rs:370:5:
auipc x0, foo must Err (llvm-mc requires %pcrel_hi/%got_pcrel_hi/%tls_*_pcrel_hi); got Ok(WordWithReloc { word: 23, reloc: Relocation { reloc_type: PcrelHi20, symbol: "foo", addend: 0 } })
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_auipc_pbt.rs
