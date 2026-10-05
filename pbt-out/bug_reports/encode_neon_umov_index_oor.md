# Bug: encode_neon_umov masks an out-of-range lane index instead of rejecting it
**Law:** A GNU-style assembler must reject a UMOV lane index outside ARM's range; encode_neon_umov([Rd, Vn.Ts[i]]) = Err when i > imax(Ts)
**Impact:** `umov w0, v0.b[16]` is encoded as `umov w0, v0.b[0]`, so an out-of-range lane silently wraps and produces a different valid instruction.
**Function:** encode_neon_umov
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:461
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_umov([w0, v0.b[16]])
**Expected:** Err (llvm-mc: vector lane must be an integer in range [0, 15])
**Actual:** Ok(Word(0x0e013c00)) — encoded as `umov w0, v0.b[0]`
**Severity:** medium
**Root cause:** neon.rs:474-477 mask the index (`index & 0xF` / `& 0x7` / `& 0x3` / `& 0x1`) instead of range-checking against imax(Ts).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:474`
```rust
            let imm5 = match elem_size.as_str() {
                "b" => ((*index & 0xF) << 1) | 0b00001,
                "h" => ((*index & 0x7) << 2) | 0b00010,
                "s" => ((*index & 0x3) << 3) | 0b00100,
                "d" => ((*index & 0x1) << 4) | 0b01000,
```
**Suggested fix:** Reject an index above the ARM maximum for that element size before encoding imm5.
```rust
            let (mask, max) = match elem_size.as_str() {
                "b" => (0xFu32, 15u32),
                "h" => (0x7, 7),
                "s" => (0x3, 3),
                "d" => (0x1, 1),
                _ => return Err(format!("unsupported umov element size: {}", elem_size)),
            };
            if *index > max {
                return Err(format!("umov lane index {} out of range for .{}", index, elem_size));
            }
            let imm5 = ((*index & mask) << (elem_size_shift)) | size_bit;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_umov_regression_index_oor -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_umov_pbt::test_encode_neon_umov_regression_index_oor' panicked at src/backend/arm/assembler/encoder/encode_neon_umov_pbt.rs:578:9:
umov w0, v0.b[16] must Err (llvm-mc range for .b is [0, 15])
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_umov_pbt.rs
