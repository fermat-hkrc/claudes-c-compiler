# Bug: encode_neon_elem truncates H-lane Rm v16-v31 to v0-v15
**Law:** ARM size=01 (H) by-element restricts Rm to v0-v15; v16.h[idx] through v31.h[idx] must be rejected
**Impact:** `mul v0.4h, v0.4h, v16.h[0]` encodes as `v0.h[0]` (Rm masked to 4 bits), so the assembler emits the wrong register instead of an error
**Function:** encode_neon_elem
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1591
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_elem([v0.4h, v0.4h, v16.h[0]], u=0, opcode=0b1000)
**Expected:** Err
**Actual:** Ok(Word) encoding Rm=v0
**Severity:** medium
**Root cause:** neon.rs:1605 masks half-word Rm with `rm & 0xF` instead of rejecting rm >= 16
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1605`
```rust
    let rm_enc = if size == 0b01 { rm & 0xF } else { rm & 0x1F };
```
**Suggested fix:** Reject Rm v16-v31 when size is 01 (H)
```rust
    if size == 0b01 && rm > 15 {
        return Err(format!("H-lane Rm v{rm} out of range (v0-v15)"));
    }
    let rm_enc = if size == 0b01 { rm & 0xF } else { rm & 0x1F };
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_elem_regression_h_rm_hi -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_elem_pbt::test_encode_neon_elem_regression_h_rm_hi' panicked at src/backend/arm/assembler/encoder/encode_neon_elem_pbt.rs:615:5:
mul v0.4h, v0.4h, v16.h[0] must Err (ARM size=01 Rm v0-v15; llvm-mc rejects)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_elem_pbt.rs
