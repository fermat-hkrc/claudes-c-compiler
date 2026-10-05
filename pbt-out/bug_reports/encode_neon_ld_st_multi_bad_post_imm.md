# Bug: encode_neon_ld_st_multi encodes an illegal post-index immediate as the legal #imm form
**Law:** Immediate post-index #imm must equal n_regs*(Q?16:8); any other immediate must Err
**Impact:** `st1 {v0.8b}, [x0], #0` is encoded as `st1 {v0.8b}, [x0], #8` (Rm=11111). A wrong writeback amount is assembled as the legal one, so SP/Xn is updated by 8 at runtime instead of being rejected at assemble time.
**Function:** encode_neon_ld_st_multi
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1008
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_ld_st_multi([RegList({v0.8b}), MemPostIndex{x0, 0}], is_load=false, num_structs=1)
**Expected:** Err (llvm-mc rejects `#0`; legal is `#8` for .8b one-register)
**Actual:** Ok(Word(0x0c9f7000)) — encoded as immediate post-index (legal #8)
**Severity:** medium
**Root cause:** neon.rs:1064 binds the offset as `_imm` and always writes Rm=11111; the numeric immediate is never compared to the transferred size.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1064`
```rust
    if let Some(_imm) = post_index {
        // Post-index with immediate: use Rm=11111 (0x1F)
        let word = ((q << 30) | (0b001100 << 24) | (1 << 23) | (l_bit << 22)) | (0b11111 << 16) | (opcode << 12) | (size << 10) | (rn << 5) | rt;
        return Ok(EncodeResult::Word(word));
```
**Suggested fix:** Require the immediate to equal n_regs * (if q==1 {16} else {8}).
```rust
    if let Some(imm) = post_index {
        let legal = n_regs as i64 * if q == 1 { 16 } else { 8 };
        if imm != legal {
            return Err(format!("ld{}/st{}: post-index #{} != #{}", num_structs, num_structs, imm, legal));
        }
        let word = ((q << 30) | (0b001100 << 24) | (1 << 23) | (l_bit << 22)) | (0b11111 << 16) | (opcode << 12) | (size << 10) | (rn << 5) | rt;
        return Ok(EncodeResult::Word(word));
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ld_st_multi_regression_bad_post_imm -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_ld_st_multi_pbt::test_encode_neon_ld_st_multi_regression_bad_post_imm' panicked at src/backend/arm/assembler/encoder/encode_neon_ld_st_multi_pbt.rs:741:5:
st1 {v0.8b}, [x0], #0 must Err
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_ld_st_multi_pbt.rs
