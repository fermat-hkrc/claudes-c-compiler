# Bug: encode_neon_elem wraps an out-of-range lane index instead of rejecting it
**Law:** ARM H-lane index is H:L:M in 0..7 and S-lane index is H:L in 0..3; an index outside that range must be rejected
**Impact:** `mul v0.4h, v0.4h, v0.h[8]` encodes as index 0 (low 3 bits), so the assembler emits the wrong lane instead of an error. llvm-mc reports "vector lane must be an integer in range [0, 7]"
**Function:** encode_neon_elem
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1591
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_elem([v0.4h, v0.4h, v0.h[8]], u=0, opcode=0b1000)
**Expected:** Err
**Actual:** Ok(Word) encoding index 0
**Severity:** medium
**Root cause:** neon.rs:1599-1603 takes only the bits that fit in H:L:M / H:L and never range-checks `index` (unlike encode_neon_elem_long)
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1599`
```rust
    let (h, l, m_bit) = match size {
        0b01 => ((index >> 2) & 1, (index >> 1) & 1, index & 1),
        0b10 => ((index >> 1) & 1, index & 1, (rm >> 4) & 1),
        _ => return Err("unsupported element size for by-element".to_string()),
    };
```
**Suggested fix:** Range-check index the same way encode_neon_elem_long does
```rust
    let (h, l, m_bit) = match size {
        0b01 => {
            if index > 7 {
                return Err(format!("element index {index} out of range for .h"));
            }
            ((index >> 2) & 1, (index >> 1) & 1, index & 1)
        }
        0b10 => {
            if index > 3 {
                return Err(format!("element index {index} out of range for .s"));
            }
            ((index >> 1) & 1, index & 1, (rm >> 4) & 1)
        }
        _ => return Err("unsupported element size for by-element".to_string()),
    };
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_elem_regression_index_oob -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_elem_pbt::encode_neon_elem_neg_index_oob' panicked at src/backend/arm/assembler/encoder/encode_neon_elem_pbt.rs:223:1:
Test failed: index 8 out of range for .h must Err (llvm-mc rejects mul v0.4h, v0.4h, v0.h[8])
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_elem_pbt.rs
