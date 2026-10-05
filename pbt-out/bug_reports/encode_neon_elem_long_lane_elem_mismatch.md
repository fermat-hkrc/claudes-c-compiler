# Bug: encode_neon_elem_long ignores the lane element size
**Law:** The third operand of a long by-element instruction must be a lane whose element size matches the source (Vm.H[i] with .4H/.8H, Vm.S[i] with .2S/.4S); a mismatched lane size must be rejected
**Impact:** `smull v0.2d, v0.2s, v0.b[0]` is encoded as if the lane were `v0.s[0]`, so illegal assembly becomes a different (legal-looking) instruction
**Function:** encode_neon_elem_long
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:235
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_elem_long([v0.2d, v0.2s, v0.b[0]], u=0, opcode=0b1010, is_high=false)
**Expected:** Err
**Actual:** Ok(Word) — `elem_size` discarded; index encoded using the source arrangement's size
**Severity:** medium
**Root cause:** neon.rs:244 binds `elem_size: _` and never checks it against the source element size
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:244`
```rust
        Operand::RegLane { reg, elem_size: _, index } => {
```
**Suggested fix:** Require `elem_size` to be `"h"` when size=01 and `"s"` when size=10
```rust
        Operand::RegLane { reg, elem_size, index } => {
            let rm = parse_reg_num(reg).ok_or("invalid NEON register")?;
            (rm, elem_size.clone(), *index)
        }
    // after size is known:
    let want = if size == 0b01 { "h" } else { "s" };
    if elem_size != want {
        return Err(format!("elem-long lane size .{} does not match source", elem_size));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_elem_long_regression_lane_elem_mismatch -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_elem_long_pbt::encode_neon_elem_long_neg_lane_elem_mismatch' panicked at src/backend/arm/assembler/encoder/encode_neon_elem_long_pbt.rs:246:1:
Test failed: lane elem_size b must match source s (llvm-mc rejects smull v0.2d, v0.2s, v0.b[0]) at src/backend/arm/assembler/encoder/encode_neon_elem_long_pbt.rs:576.
minimal failing input: rd = 0, rn = 0, rm_raw = 0, idx_raw = 0, shape = ("2s", "2d", "s", 3, 31, false), wrong = "b"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_elem_long_pbt.rs
