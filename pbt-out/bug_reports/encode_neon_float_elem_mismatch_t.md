# Bug: encode_neon_float_elem ignores a mismatched source arrangement
**Law:** Dest and source arrangements must match (Vd.T, Vn.T); llvm-mc/gas reject a different Vn arrangement
**Impact:** `fmul v0.4s, v0.8b, v0.s[0]` encodes as if both were .4s, hiding a type error in the assembly
**Function:** encode_neon_float_elem
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1613
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_float_elem([v0.4s, v0.8b, v0.s[0]], u_bit=0, opcode=0b1001)
**Expected:** Err (llvm-mc: invalid operand)
**Actual:** Ok(Word) using dest T only
**Severity:** medium
**Root cause:** neon.rs:1617 discards the source arrangement (`let (rn, _) = get_neon_reg(operands, 1)?`) and never compares it to dest
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1617`
```rust
    let (rn, _) = get_neon_reg(operands, 1)?;
```
**Suggested fix:** Require the source arrangement to equal dest
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    if arr_n != arr_d {
        return Err(format!("float by-element: mismatched arrangement {} vs {}", arr_d, arr_n));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_float_elem_neg_mismatch_t -- --test-threads=1
```
**Raw output:**
```text
Test failed: mismatched source T must Err (llvm-mc rejects fmul v0.4s, v0.8b, v0.s[0])
minimal failing input: rd = 0, rn = 0, rm = 0, idx_raw = 0, shape = ("4s", "s", 3), t_wrong = "8b"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_float_elem_pbt.rs::test_encode_neon_float_elem_regression_mismatch_t
