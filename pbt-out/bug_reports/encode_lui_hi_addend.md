# Bug: encode_lui treats %hi(foo+4) as symbol "foo+4" with addend 0
**Law:** ∀ rd, ∀ sym, ∀ a ≠ 0. encode_lui([Reg(rd), Symbol("%hi("+sym+±a+")")]) relocates against `sym` with addend `a`
**Impact:** `lui rd, %hi(foo+4)` emits R_RISCV_HI20 against the literal name `foo+4` instead of `foo` with addend 4. llvm-mc object files use `R_RISCV_HI20 foo + 4`. The linker cannot resolve `foo+4`, so TLS/absolute high-part addressing of `symbol+offset` is wrong.
**Function:** encode_lui
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:5
**Detected by:** Differential (llvm-mc R_RISCV_HI20 foo 0x4)
**Minimal input:** encode_lui([Reg("x0"), Symbol("%hi(foo+4)")])
**Expected:** WordWithReloc { Hi20, symbol: "foo", addend: 4 }
**Actual:** WordWithReloc { Hi20, symbol: "foo+4", addend: 0 }
**Severity:** high
**Root cause:** base.rs:22-23 stores extract_modifier_symbol(s) (the full inner text `foo+4`) and hardcodes addend 0. extract_modifier_symbol only slices between parentheses.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:22`
```rust
                    symbol: extract_modifier_symbol(s),
                    addend: 0,
```
**Suggested fix:** Split a trailing `+N`/`-N` off the extracted inner text into symbol and addend.
```rust
                    let inner = extract_modifier_symbol(s);
                    let (symbol, addend) = split_symbol_addend(&inner);
                    // ...
                    symbol,
                    addend,
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_lui_regression_hi_addend -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_lui_pbt::test_encode_lui_regression_hi_addend' panicked at src/backend/riscv/assembler/encoder/encode_lui_pbt.rs:368:13:
assertion `left == right` failed: %hi(foo+4) symbol must be foo, not foo+4
  left: "foo+4"
 right: "foo"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_lui_pbt.rs
