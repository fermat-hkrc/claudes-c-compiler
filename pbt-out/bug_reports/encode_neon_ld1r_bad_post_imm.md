# Bug: encode_neon_ld1r accepts illegal post-index immediates
**Law:** Immediate post-index #imm for LD1R must equal the element size; any other #imm must make encode_neon_ld1r return Err.
**Impact:** `ld1r {v0.8b}, [x0], #-1` (and any #imm ≠ 1 for .8b/.16b, ≠ 2 for .h, ≠ 4 for .s, ≠ 8 for .d) encodes as the legal post-index form. The assembler emits writeback by esize regardless of the written immediate, so the text and the machine code disagree on the increment.
**Function:** encode_neon_ld1r
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:832
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_ld1r([RegList({v0.8b}), MemPostIndex{base:"x0", offset:-1}])
**Expected:** Err
**Actual:** Ok(Word(0x0ddfc000)) — encoding of `ld1r {v0.8b}, [x0], #1`
**Severity:** low (documented by the author)
**Root cause:** neon.rs:876 discards offset (`let _ = offset`) and always encodes Rm=11111. The comment states the offset must match element size but the body never checks.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:876`
```rust
            let _ = offset; // offset must match element size, not encoded separately
```
**Suggested fix:** Compare offset to esize(T) and reject a mismatch.
```rust
            let esize = match size { 0b00 => 1i64, 0b01 => 2, 0b10 => 4, _ => 8 };
            if *offset != esize {
                return Err(format!("ld1r: post-index #{} must be #{}", offset, esize));
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ld1r_regression_bad_post_imm -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::neon::encode_neon_ld1r_pbt::encode_neon_ld1r_neg_bad_post_imm' panicked at src/backend/arm/assembler/encoder/neon.rs:12939:5:
Test failed: ld1r post-index #-1 (legal #1) must Err at src/backend/arm/assembler/encoder/neon.rs:13148.
minimal failing input: t = "8b", rt = 0, rn = 0, imm = -1
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_ld1r_pbt.rs
