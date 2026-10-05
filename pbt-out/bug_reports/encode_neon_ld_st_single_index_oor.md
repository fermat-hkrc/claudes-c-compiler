# Bug: encode_neon_ld_st_single accepts an out-of-range lane index
**Law:** Lane index must be in .b[0,15] / .h[0,7] / .s[0,3] / .d[0,1]; out-of-range indices must be Err.
**Impact:** `st1 {v0.b}[16], [x0]` encodes as lane 0 (index bits masked), so a typo produces a silently wrong element load/store.
**Function:** encode_neon_ld_st_single
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:904
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_ld_st_single([RegListIndexed({v0.b}[16]), Mem{x0,0}], is_load=false, num_structs=1)
**Expected:** Err
**Actual:** Ok(Word) — index 16 is masked to Q:S:size bits as 0
**Severity:** medium
**Root cause:** neon.rs:964-967 extract Q/S/size from index with shifts and masks and never range-check against the element size.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:964`
```rust
            let q = (index >> 3) & 1;
            let s = (index >> 2) & 1;
            let sz = index & 3;
```
**Suggested fix:** Reject index above the documented lane maximum for the element size.
```rust
            if index > 15 {
                return Err(format!("vector lane must be in range [0, 15], got {}", index));
            }
            let q = (index >> 3) & 1;
            let s = (index >> 2) & 1;
            let sz = index & 3;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ld_st_single_regression_index_oor -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::neon::encode_neon_ld_st_single_pbt::test_encode_neon_ld_st_single_regression_index_oor' panicked at src/backend/arm/assembler/encoder/neon.rs:13463:9:
st1 {v0.b}[16], [x0] must Err
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_ld_st_single_pbt.rs
