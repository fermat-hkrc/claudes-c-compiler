# Bug: encode_neon_elem ignores a mismatched source arrangement
**Law:** MUL/MLA/MLS by element requires matching Vd.T and Vn.T; a mismatched source arrangement must be rejected
**Impact:** `mul v0.4h, v0.8b, v0.h[0]` encodes as a valid .4h by-element word (Q/size taken only from dest), so illegal assembly is accepted
**Function:** encode_neon_elem
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1591
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_elem([v0.4h, v0.8b, v0.h[0]], u=0, opcode=0b1000)
**Expected:** Err
**Actual:** Ok(Word) — source arrangement discarded
**Severity:** medium
**Root cause:** neon.rs:1594 binds source as `let (rn, _) = get_neon_reg(operands, 1)?`, discarding the source arrangement, then Q/size come only from dest
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1594`
```rust
    let (rn, _) = get_neon_reg(operands, 1)?;
```
**Suggested fix:** Compare source arrangement against dest and return Err on mismatch
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    if arr_n != arr_d {
        return Err(format!("mismatched arrangement: dest {arr_d} src {arr_n}"));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_elem_regression_mismatch_t -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_elem_pbt::test_encode_neon_elem_regression_mismatch_t' panicked at src/backend/arm/assembler/encoder/encode_neon_elem_pbt.rs:606:5:
mul v0.4h, v0.8b, v0.h[0] must Err (llvm-mc/gas require matching T)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_elem_pbt.rs
