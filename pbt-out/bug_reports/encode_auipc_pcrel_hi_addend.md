# Bug: encode_auipc treats %pcrel_hi(foo+4) as symbol "foo+4" with addend 0
**Law:** ∀ rd, ∀ sym, ∀ a ≠ 0. encode_auipc([Reg(rd), Symbol("%pcrel_hi("+sym+±a+")")]) relocates against `sym` with addend `a`
**Impact:** `auipc rd, %pcrel_hi(foo+4)` emits R_RISCV_PCREL_HI20 against the literal name `foo+4` instead of `foo` with addend 4. llvm-mc object files use `R_RISCV_PCREL_HI20 foo + 4`. The linker cannot resolve `foo+4`, so PC-relative addressing of `symbol+offset` (including `call`/`la` expansions that the README documents) is wrong.
**Function:** encode_auipc
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:30
**Detected by:** Differential (llvm-mc R_RISCV_PCREL_HI20 foo 0x4)
**Minimal input:** encode_auipc([Reg("x0"), Symbol("%pcrel_hi(foo+4)")])
**Expected:** WordWithReloc { PcrelHi20, symbol: "foo", addend: 4 }
**Actual:** WordWithReloc { PcrelHi20, symbol: "foo+4", addend: 0 }
**Severity:** high
**Root cause:** base.rs:37-43 stores parse_reloc_modifier's symbol (the full inner text `foo+4` from extract_modifier_symbol) and hardcodes addend 0.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:42`
```rust
                    symbol,
                    addend: 0,
```
**Suggested fix:** Split a trailing `+N`/`-N` off the extracted inner text into symbol and addend.
```rust
            let (reloc_type, inner) = parse_reloc_modifier(s);
            let (symbol, addend) = split_symbol_addend(&inner);
            Ok(EncodeResult::WordWithReloc {
                word: encode_u(OP_AUIPC, rd, 0),
                reloc: Relocation { reloc_type, symbol, addend },
            })
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_auipc_regression_pcrel_hi_addend -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_auipc_pbt::test_encode_auipc_regression_pcrel_hi_addend' panicked at src/backend/riscv/assembler/encoder/encode_auipc_pbt.rs:383:13:
assertion `left == right` failed: %pcrel_hi(foo+4) symbol must be foo, not foo+4
  left: "foo+4"
 right: "foo"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_auipc_pbt.rs
