# Bug: encode_neon_tbx ignores a surplus fourth operand
**Law:** A GNU-style assembler must reject a fourth TBX operand; encode_neon_tbx(ops ++ [extra]) = Err
**Impact:** `tbx v0.8b, {v0.16b}, v0.8b, v0.8b` is assembled as the 3-operand form, so a typo or extra token silently produces a valid instruction instead of an assembler error.
**Function:** encode_neon_tbx
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:803
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_tbx([v0.8b, {v0.16b}, v0.8b, v0.8b])
**Expected:** Err (llvm-mc/gas reject `tbx v0.8b, {v0.16b}, v0.8b, v0.8b`)
**Actual:** Ok(Word(0x0e001000)) — encoded as `tbx v0.8b, {v0.16b}, v0.8b`
**Severity:** medium
**Root cause:** neon.rs:804 `if operands.len() < 3` only rejects too few operands; extras past index 2 are never inspected.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:804`
```rust
    if operands.len() < 3 {
        return Err("tbx requires 3 operands".to_string());
    }
```
**Suggested fix:** Require exactly three operands.
```rust
    if operands.len() != 3 {
        return Err("tbx requires 3 operands".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_tbx_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_tbx_pbt::test_encode_neon_tbx_regression_extra_operand' panicked at src/backend/arm/assembler/encoder/encode_neon_tbx_pbt.rs:574:5:
tbx v0.8b, {v0.16b}, v0.8b, v0.8b must Err (exactly 3 operands)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_tbx_pbt.rs
