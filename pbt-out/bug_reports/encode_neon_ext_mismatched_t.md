# Bug: encode_neon_ext ignores mismatched source arrangements
**Law:** EXT requires matching T on Vd, Vn, and Vm; encode_neon_ext([Vd.Td, Vn.Tn, Vm.Tm, #i]) = Err when Td, Tn, Tm are not all equal
**Impact:** `ext v0.8b, v0.8b, v0.16b, #0` is assembled using only the dest arrangement, so a mixed 64-bit/128-bit permute silently becomes an 8B extract.
**Function:** encode_neon_ext
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:405
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_ext([v0.8b, v0.8b, v0.16b, #0])
**Expected:** Err (llvm-mc/gas reject mismatched T)
**Actual:** Ok(Word(0x2e000000)) — encoded as `ext v0.8b, v0.8b, v0.8b, #0`
**Severity:** medium
**Root cause:** neon.rs:410-411 bind source arrangements to `_` and never compare them to arr_d, so only dest T affects Q.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:410`
```rust
    let (rn, _) = get_neon_reg(operands, 1)?;
    let (rm, _) = get_neon_reg(operands, 2)?;
```
**Suggested fix:** Require matching arrangements on all three registers.
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    let (rm, arr_m) = get_neon_reg(operands, 2)?;
    if arr_n != arr_d || arr_m != arr_d {
        return Err(format!("EXT arrangement mismatch: {} vs {} vs {}", arr_d, arr_n, arr_m));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ext_regression_mismatched_t -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_ext_pbt::test_encode_neon_ext_regression_mismatched_t' panicked at src/backend/arm/assembler/encoder/encode_neon_ext_pbt.rs:582:5:
ext v0.16b, v1.8b, v2.16b, #3 must Err (gas/llvm-mc require matching T)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_ext_pbt.rs
