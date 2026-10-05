# Bug: encode_neon_elem ignores a fourth operand
**Law:** A GNU-style by-element instruction must have exactly three operands; a fourth operand must be rejected
**Impact:** The assembler silently encodes `mul v0.4h, v0.4h, v0.h[0], v0.4h` (and MLA/MLS/SQDMULH/SQRDMULH siblings) as the three-operand form, so illegal assembly becomes a valid 32-bit word instead of an error
**Function:** encode_neon_elem
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1591
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_elem([v0.4h, v0.4h, v0.h[0], v0.4h], u=0, opcode=0b1000)
**Expected:** Err
**Actual:** Ok(Word) — fourth operand ignored
**Severity:** medium
**Root cause:** neon.rs:1592 checks only `operands.len() < 3` and then encodes, so any extra operand after the lane is dropped
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1592`
```rust
    if operands.len() < 3 { return Err("NEON by-element requires 3 operands".to_string()); }
```
**Suggested fix:** Reject when `operands.len() != 3` as well as when it is below 3
```rust
    if operands.len() != 3 {
        return Err("NEON by-element requires 3 operands".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_elem_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_elem_pbt::test_encode_neon_elem_regression_extra_operand' panicked at src/backend/arm/assembler/encoder/encode_neon_elem_pbt.rs:597:5:
mul v0.4h, v0.4h, v0.h[0], v0.4h must Err (llvm-mc/gas reject a fourth operand)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_elem_pbt.rs
