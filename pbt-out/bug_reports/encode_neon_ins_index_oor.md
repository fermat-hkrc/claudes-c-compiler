# Bug: encode_neon_ins masks an out-of-range lane index instead of rejecting it
**Law:** ARM INS lane index must be in [0, imax(Ts)] (B:15 H:7 S:3 D:1); llvm-mc rejects anything larger, so encode_neon_ins([Vd.ts[i], R]) = Err when i > imax(ts)
**Impact:** `ins v0.b[16], w0` is assembled as `ins v0.b[0], w0` because the index is masked with `& 0xF`, so an out-of-range lane silently becomes a different (legal) insert.
**Function:** encode_neon_ins
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:549
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_ins([v0.b[16], w0])
**Expected:** Err (llvm-mc: vector lane must be an integer in range [0, 15])
**Actual:** Ok(Word(0x4e011c00)) — encoded as `ins v0.b[0], w0`
**Severity:** medium
**Root cause:** neon.rs:559-563 mask the index (`*index & 0xF` / `& 0x7` / `& 0x3` / `& 0x1`) instead of rejecting values outside the ARM range.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:559`
```rust
            let imm5 = match elem_size.as_str() {
                "b" => ((*index & 0xF) << 1) | 0b00001,
                "h" => ((*index & 0x7) << 2) | 0b00010,
                "s" => ((*index & 0x3) << 3) | 0b00100,
                "d" => ((*index & 0x1) << 4) | 0b01000,
```
**Suggested fix:** Reject out-of-range indices before packing imm5 (and the element-form imm4).
```rust
            let max = match elem_size.as_str() {
                "b" => 15u32,
                "h" => 7,
                "s" => 3,
                "d" => 1,
                _ => return Err(format!("unsupported ins element size: {}", elem_size)),
            };
            if *index > max {
                return Err(format!("ins: lane index {} out of range [0, {}]", index, max));
            }
            let imm5 = match elem_size.as_str() {
                "b" => (*index << 1) | 0b00001,
                "h" => (*index << 2) | 0b00010,
                "s" => (*index << 3) | 0b00100,
                "d" => (*index << 4) | 0b01000,
                _ => unreachable!(),
            };
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ins_regression_index_oor -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_ins_pbt::test_encode_neon_ins_regression_index_oor' panicked at src/backend/arm/assembler/encoder/encode_neon_ins_pbt.rs:585:5:
ins v0.b[16], w1 must Err (llvm-mc range for .b is [0, 15])
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_ins_pbt.rs
