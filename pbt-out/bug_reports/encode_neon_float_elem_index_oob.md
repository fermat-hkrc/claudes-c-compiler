# Bug: encode_neon_float_elem accepts an out-of-range lane index
**Law:** Lane index must be in llvm-mc/ARM range: S lanes [0, 3], D lanes [0, 1]
**Impact:** `fmul v0.2s, v0.2s, v0.s[4]` encodes by wrapping H:L instead of failing, so an out-of-range lane is silently turned into a different in-range lane
**Function:** encode_neon_float_elem
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1613
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_float_elem([v0.2s, v0.2s, v0.s[4]], u_bit=0, opcode=0b1001)
**Expected:** Err (llvm-mc: vector lane must be an integer in range [0, 3])
**Actual:** Ok(Word) with H:L taken from the low bits of 4
**Severity:** medium
**Root cause:** neon.rs:1618-1628 extracts `index` and uses only H/L bits; there is no range check against the arrangement
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1618`
```rust
    let (rm, index) = match &operands[2] {
```
**Suggested fix:** Reject index above the ARM maximum for the dest size
```rust
    let imax = if sz == 0 { 3u32 } else { 1u32 };
    if index > imax {
        return Err(format!("float by-element: lane index {} out of range 0..{}", index, imax));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_float_elem_neg_index_oob -- --test-threads=1
```
**Raw output:**
```text
Test failed: index 4 out of range for .s must Err (llvm-mc rejects fmul v0.2s, v0.2s, v0.s[4])
minimal failing input: rd = 0, rn = 0, rm = 0, shape = ("2s", "s", 3), extra = 1
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_float_elem_pbt.rs::test_encode_neon_float_elem_regression_index_oob
