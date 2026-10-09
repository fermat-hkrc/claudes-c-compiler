# Bug: encode_imul accepts mismatched-width register pairs
**Law:** When llvm-mc rejects a width-mismatched IMUL pair, the SUT must Err (not silently alias via reg_num).
**Impact:** `imull %ax, %eax` encodes as if both were 32-bit.
**Function:** encode_imul
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:717
**Detected by:** Negative/error (encode_imul_neg_mismatched_or_non_gp)
**Minimal input:** imull %ax, %eax (kind=0)
**Expected:** Err
**Actual:** Ok with GP-aliased bytes
**Severity:** medium
**Root cause:** No reg_size check against mnemonic size before reg_num.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:717`
```rust
let src_num = reg_num(&src.name).ok_or("bad register")?;
let dst_num = reg_num(&dst.name).ok_or("bad register")?;
```
**Suggested fix:**
```rust
if reg_size(&src.name) != size || reg_size(&dst.name) != size {
    return Err("imul register width mismatch".into());
}
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_imul_neg_mismatched_or_non_gp -- --test-threads=1
```
**Raw output:**
```text
SUT accepted invalid `imull %ax, %eax` as Ok(...); llvm-mc rejected
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_imul_pbt.rs
