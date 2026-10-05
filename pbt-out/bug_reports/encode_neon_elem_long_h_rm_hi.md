# Bug: encode_neon_elem_long silently truncates H-lane Rm v16–v31 to v0–v15
**Law:** For size=01 (half-word) long by-element, Vm must be v0–v15; v16–v31 must be rejected, not encoded as Vm & 15
**Impact:** `smull v0.4s, v0.4h, v16.h[0]` is assembled as `smull v0.4s, v0.4h, v0.h[0]` — a different register, with no error — so the object file contains the wrong instruction
**Function:** encode_neon_elem_long
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:235
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_elem_long([v0.4s, v0.4h, v16.h[0]], u=0, opcode=0b1010, is_high=false)
**Expected:** Err (ARM size=01 Rm is 4 bits; llvm-mc rejects v16.h[0])
**Actual:** Ok(Word) with Rm field 0 (v16 & 0xF)
**Severity:** high
**Root cause:** neon.rs:287 masks `rm & 0xF` for half-word instead of rejecting rm > 15
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:287`
```rust
    let rm_enc = if size == 0b01 { rm & 0xF } else { rm & 0x1F };
```
**Suggested fix:** Return Err when size=01 and rm > 15, matching ARM and llvm-mc
```rust
    if size == 0b01 && rm > 15 {
        return Err(format!("H-lane Rm v{} out of range (v0-v15)", rm));
    }
    let rm_enc = if size == 0b01 { rm & 0xF } else { rm & 0x1F };
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_elem_long_regression_h_rm_hi -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_elem_long_pbt::test_encode_neon_elem_long_regression_h_rm_hi' panicked at src/backend/arm/assembler/encoder/encode_neon_elem_long_pbt.rs:538:5:
smull v0.4s, v0.4h, v16.h[0] must Err (ARM size=01 Rm v0-v15; llvm-mc rejects)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_elem_long_pbt.rs
