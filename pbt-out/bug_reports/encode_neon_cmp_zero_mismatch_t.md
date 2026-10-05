# Bug: encode_neon_cmp_zero ignores source arrangement mismatch
**Law:** ∀ rd,rn ∈ {0..31}, Td ≠ Tn both in {8b,16b,4h,8h,2s,4s,2d}. llvm-mc(cmeq Vd.Td, Vn.Tn, #0) = Err ∧ encode_neon_cmp_zero([Vd.Td, Vn.Tn], 0, 0b01001) = Err
**Impact:** Mismatched lane arrangements assemble using only dest T, producing a word gas/llvm-mc refuse. Callers that pass through parsed operands will emit the wrong size/Q for the source.
**Function:** encode_neon_cmp_zero
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:189
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_cmp_zero([v0.16b, v0.8b], u=0, opcode=0b01001)  // cmeq v0.16b, v0.8b, #0
**Expected:** Err
**Actual:** Ok(Word) encoded as if both were .16b
**Severity:** medium
**Root cause:** neon.rs:194 discards the source arrangement (`let (rn, _) = get_neon_reg(operands, 1)?`), so Q/size come only from dest T.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:194`
```rust
    let (rn, _) = get_neon_reg(operands, 1)?;
```
**Suggested fix:** Require the source arrangement to match dest T.
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    if arr_n != arr_d {
        return Err(format!("NEON compare-zero: arrangement mismatch {} vs {}", arr_d, arr_n));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_cmp_zero_neg_mismatch_t -- --test-threads=1
```
**Raw output:**
```text
Test failed: mismatched T must Err (llvm-mc rejects cmeq v0.16b, v0.8b, #0) at src/backend/arm/assembler/encoder/encode_neon_cmp_zero_pbt.rs:324.
minimal failing input: rd = 0, rn = 0, td = "16b", tn = "8b"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_cmp_zero_pbt.rs
