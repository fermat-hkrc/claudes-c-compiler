# Bug: encode_neon_ext ignores a surplus fifth operand
**Law:** A GNU-style assembler must reject a fifth EXT operand; encode_neon_ext(ops ++ [extra]) = Err
**Impact:** `ext v0.16b, v1.16b, v2.16b, #3, v3.16b` is assembled as the 4-operand form, so a typo or extra token silently produces a valid instruction instead of an assembler error.
**Function:** encode_neon_ext
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:405
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_ext([v0.8b, v0.8b, v0.8b, #0, v0.8b])
**Expected:** Err (llvm-mc/gas reject `ext v0.8b, v0.8b, v0.8b, #0, v0.8b`)
**Actual:** Ok(Word(0x2e000000)) — encoded as `ext v0.8b, v0.8b, v0.8b, #0`
**Severity:** medium
**Root cause:** neon.rs:406 `if operands.len() < 4` only rejects too few operands; extras past index 3 are never inspected.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:406`
```rust
    if operands.len() < 4 {
        return Err("ext requires 4 operands".to_string());
    }
```
**Suggested fix:** Require exactly four operands.
```rust
    if operands.len() != 4 {
        return Err("ext requires 4 operands".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ext_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_ext_pbt::test_encode_neon_ext_regression_extra_operand' panicked at src/backend/arm/assembler/encoder/encode_neon_ext_pbt.rs:533:5:
ext v0.16b, v1.16b, v2.16b, #3, v3.16b must Err (gas/llvm-mc reject a fifth operand)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_ext_pbt.rs
