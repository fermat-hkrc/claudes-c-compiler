# Bug: encode_neon_mls encodes reserved .1d/.2d MLS
**Law:** ARM Advanced SIMD MLS (vector) T is {8B,16B,4H,8H,2S,4S}; size:Q=11:x is reserved, so .1d and .2d must be rejected
**Impact:** `mls v0.1d, ...` / `mls v0.2d, ...` assemble to unallocated encodings that llvm-mc/gas reject; executing them is UNDEFINED
**Function:** encode_neon_mls
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:361
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_mls([v0.1d, v0.1d, v0.1d])
**Expected:** Err (ARM size:Q=11:x reserved for MLS; llvm-mc rejects .1d and .2d)
**Actual:** Ok(Word(0x2ee09400)) for .1d; Ok(Word(0x6ee09400)) for .2d
**Severity:** medium
**Root cause:** neon.rs:365 calls neon_arr_to_q_size, which maps 1d/2d to size=11, and encode_neon_mls has no MLS-specific rejection of that reserved size
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:365`
```rust
    let (q, size) = neon_arr_to_q_size(&arr_d)?;
```
**Suggested fix:** Reject size=11 (1d/2d) for vector MLS
```rust
    let (q, size) = neon_arr_to_q_size(&arr_d)?;
    if size == 0b11 {
        return Err(format!("MLS does not support arrangement {arr_d}"));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_mls_regression_reserved_1d -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_mls_pbt::encode_neon_mls_neg_reserved_t' panicked at src/backend/arm/assembler/encoder/encode_neon_mls_pbt.rs:428:1:
Test failed: reserved MLS T=1d (ARM size:Q=11:x) must Err (llvm-mc rejects mls v0.1d, v0.1d, v0.1d) at src/backend/arm/assembler/encoder/encode_neon_mls_pbt.rs:513.
minimal failing input: rd = 0, rn = 0, rm = 0, t = "1d"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_mls_pbt.rs
