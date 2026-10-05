# Bug: encode_neon_elem_long ignores a fourth operand
**Law:** A GNU-style long by-element instruction must have exactly three operands; a fourth operand must be rejected
**Impact:** The assembler silently encodes `smull v0.4s, v0.4h, v0.h[0], v0.4s` (and siblings) as the three-operand form, so illegal assembly becomes a valid 32-bit word instead of an error
**Function:** encode_neon_elem_long
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:235
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_elem_long([v0.4s, v0.4h, v0.h[0], v0.4s], u=0, opcode=0b1010, is_high=false)
**Expected:** Err
**Actual:** Ok(Word) — fourth operand ignored
**Severity:** medium
**Root cause:** neon.rs:236 checks only `operands.len() < 3` and then encodes, so any extra operand after the lane is dropped
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:236`
```rust
    if operands.len() < 3 {
        return Err("NEON elem-long requires 3 operands".to_string());
    }
```
**Suggested fix:** Reject when `operands.len() != 3` (or `> 3`) as well as when it is below 3
```rust
    if operands.len() != 3 {
        return Err("NEON elem-long requires 3 operands".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_elem_long_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_elem_long_pbt::test_encode_neon_elem_long_regression_extra_operand' panicked at src/backend/arm/assembler/encoder/encode_neon_elem_long_pbt.rs:520:5:
smull v0.4s, v0.4h, v0.h[0], v0.4s must Err (llvm-mc/gas reject a fourth operand)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_elem_long_pbt.rs
