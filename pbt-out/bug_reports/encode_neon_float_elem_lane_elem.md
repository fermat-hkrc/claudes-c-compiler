# Bug: encode_neon_float_elem ignores a mismatched lane element size
**Law:** The lane element size must match the vector arrangement (s for 2s/4s, d for 2d); llvm-mc/gas reject `fmul v0.2d, v0.2d, v0.b[0]`
**Impact:** A mistyped lane size is encoded as if it matched T, so the assembler does not catch an invalid by-element operand
**Function:** encode_neon_float_elem
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1613
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_float_elem([v0.2d, v0.2d, v0.b[0]], u_bit=0, opcode=0b1001)
**Expected:** Err (llvm-mc: invalid operand)
**Actual:** Ok(Word) because elem_size is discarded
**Severity:** medium
**Root cause:** neon.rs:1618 matches RegLane with `elem_size` in `..`, so the lane size is never compared to dest T
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1619`
```rust
        Operand::RegLane { reg, index, .. } => (parse_reg_num(reg).ok_or("invalid reg")?, *index),
```
**Suggested fix:** Bind elem_size in the match and, after sz is known, require it to match dest T
```rust
    let expect = if sz == 0 { "s" } else { "d" };
    if elem_size != expect {
        return Err(format!("float by-element: lane size {} does not match {}", elem_size, expect));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_float_elem_neg_lane_elem_mismatch -- --test-threads=1
```
**Raw output:**
```text
Test failed: lane elem_size b must match arrangement d (llvm-mc rejects fmul v0.2d, v0.2d, v0.b[0])
minimal failing input: rd = 0, rn = 0, rm = 0, idx_raw = 0, shape = ("2d", "d", 1), wrong = "b"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_float_elem_pbt.rs::test_encode_neon_float_elem_regression_lane_elem_mismatch
