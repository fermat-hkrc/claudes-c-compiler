# Bug: encode_neon_umov ignores a surplus third operand
**Law:** A GNU-style assembler must reject a third UMOV operand; encode_neon_umov(ops ++ [extra]) = Err
**Impact:** `umov w0, v0.b[0], w0` is assembled as the 2-operand form, so a typo or extra token silently produces a valid instruction instead of an assembler error.
**Function:** encode_neon_umov
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:461
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_umov([w0, v0.b[0], w0])
**Expected:** Err (llvm-mc/gas reject `umov w0, v0.b[0], w0`)
**Actual:** Ok(Word(0x0e013c00)) — encoded as `umov w0, v0.b[0]`
**Severity:** medium
**Root cause:** neon.rs:462 `if operands.len() < 2` only rejects too few operands; extras past index 1 are never inspected.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:462`
```rust
    if operands.len() < 2 {
        return Err("umov requires 2 operands".to_string());
    }
```
**Suggested fix:** Require exactly two operands.
```rust
    if operands.len() != 2 {
        return Err("umov requires 2 operands".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_umov_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_umov_pbt::test_encode_neon_umov_regression_extra_operand' panicked at src/backend/arm/assembler/encoder/encode_neon_umov_pbt.rs:569:9:
umov w0, v0.b[0], w0 must Err (llvm-mc rejects a third operand)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_umov_pbt.rs
