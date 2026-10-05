# Bug: encode_neon_ld_st_single accepts an illegal post-index immediate
**Law:** Immediate post-index #imm must equal n*esize (1/2/4/8 × struct count); any other immediate must be Err.
**Impact:** `st1 {v0.b}[0], [x0], #0` encodes as a legal `#1` immediate post-index (Rm=11111), so a wrong writeback amount is assembled without error.
**Function:** encode_neon_ld_st_single
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:904
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_ld_st_single([RegListIndexed({v0.b}[0]), MemPostIndex{x0, 0}], is_load=false, num_structs=1)
**Expected:** Err
**Actual:** Ok(Word) with Rm=11111 (legal immediate post-index encoding)
**Severity:** medium
**Root cause:** neon.rs:994 binds Some(_offset) and ignores the value; the word always uses Rm=11111.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:994`
```rust
    if let Some(_offset) = post_index {
        // Post-index form: Q 0011011 L R 11111 opcode S size Rn Rt
        // (Rm=11111 means immediate post-index, the amount is implicit from element size)
        let word = (q_bit << 30) | (0b0011011 << 23) | (l_bit << 22) | (r_bit << 21)
            | (0b11111 << 16) | (opcode << 13) | (s_bit << 12) | (size_field << 10) | (rn << 5) | rt;
```
**Suggested fix:** Require offset == num_structs * esize before encoding Rm=11111.
```rust
    if let Some(offset) = post_index {
        let legal = (num_structs as i64) * esize_of(&elem_size);
        if offset != legal {
            return Err(format!("post-index immediate must be #{}, got #{}", legal, offset));
        }
        let word = (q_bit << 30) | (0b0011011 << 23) | (l_bit << 22) | (r_bit << 21)
            | (0b11111 << 16) | (opcode << 13) | (s_bit << 12) | (size_field << 10) | (rn << 5) | rt;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ld_st_single_regression_bad_post_imm -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::neon::encode_neon_ld_st_single_pbt::test_encode_neon_ld_st_single_regression_bad_post_imm' panicked at src/backend/arm/assembler/encoder/neon.rs:13505:9:
st1 {v0.b}[0], [x0], #0 must Err
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_ld_st_single_pbt.rs
