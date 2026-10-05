# Bug: encode_neon_movi ignores surplus operands
**Law:** ∀ valid 2-operand movi plus an extra RegArrangement or illegal shift. llvm-mc rejects the asm ∧ encode_neon_movi(ops) = Err
**Impact:** `movi v0.8b, #0, v0.8b` is assembled as the 2-operand form. A typo or extra token silently produces a valid instruction instead of an assembler error. The same hole accepts LSL on 8B/16B/2D and LSR on any arrangement (llvm-mc/gas reject those).
**Function:** encode_neon_movi
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:624
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_movi([v0.8b, #0, v0.8b])
**Expected:** Err (llvm-mc/gas reject `movi v0.8b, #0, v0.8b`)
**Actual:** Ok(Word) — encoded as `movi v0.8b, #0`
**Severity:** medium
**Root cause:** neon.rs:625 `if operands.len() < 2` only rejects too few operands. The 8b/16b/2d/4h/8h arms never inspect operands past index 1; the 2s/4s arm peeks at a Shift at index 2 but still ignores a fourth operand and non-Shift extras.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:625`
```rust
    if operands.len() < 2 {
        return Err("movi requires 2 operands".to_string());
    }
```
**Suggested fix:** Require exactly 2 operands, or 3 when the third is a legal Shift for that arrangement.
```rust
    if operands.len() < 2 {
        return Err("movi requires 2 operands".to_string());
    }
    // After encoding the legal 2- or 3-operand form:
    // if operands.len() > expected { return Err("movi: unexpected extra operand".into()); }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_movi_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_movi_pbt::test_encode_neon_movi_regression_extra_operand' (2277462) panicked at src/backend/arm/assembler/encoder/encode_neon_movi_pbt.rs:613:5:
movi v0.8b, #0, v0.8b must Err (gas/llvm-mc reject a third non-shift operand)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_movi_pbt.rs
