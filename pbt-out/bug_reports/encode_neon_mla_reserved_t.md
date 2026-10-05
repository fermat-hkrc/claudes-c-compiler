# Bug: encode_neon_mla encodes reserved .1d/.2d MLA
**Law:** ARM Advanced SIMD MLA (vector) T is {8B,16B,4H,8H,2S,4S}; size:Q=11:x is reserved, so .1d and .2d must be rejected
**Impact:** `mla v0.1d, ...` / `mla v0.2d, ...` assemble to unallocated encodings that llvm-mc/gas reject; executing them is UNDEFINED
**Function:** encode_neon_mla
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:349
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_mla([v0.1d, v0.1d, v0.1d])
**Expected:** Err (ARM size:Q=11:x reserved for MLA; llvm-mc rejects .1d and .2d)
**Actual:** Ok(Word(0x0ee09400)) for .1d; Ok(Word(0x4ee09400)) for .2d
**Severity:** medium
**Root cause:** neon.rs:353 calls neon_arr_to_q_size, which maps 1d/2d to size=11, and encode_neon_mla has no MLA-specific rejection of that reserved size
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:353`
```rust
    let (q, size) = neon_arr_to_q_size(&arr_d)?;
```
**Suggested fix:** Reject size=11 (1d/2d) for vector MLA
```rust
    let (q, size) = neon_arr_to_q_size(&arr_d)?;
    if size == 0b11 {
        return Err(format!("MLA does not support arrangement {arr_d}"));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_mla_regression_reserved_1d -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_mla_pbt::encode_neon_mla_neg_reserved_t' panicked at src/backend/arm/assembler/encoder/encode_neon_mla_pbt.rs:427:1:
Test failed: reserved MLA T=1d (ARM size:Q=11:x) must Err (llvm-mc rejects mla v0.1d, v0.1d, v0.1d) at src/backend/arm/assembler/encoder/encode_neon_mla_pbt.rs:512.
minimal failing input: rd = 0, rn = 0, rm = 0, t = "1d"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_mla_pbt.rs
