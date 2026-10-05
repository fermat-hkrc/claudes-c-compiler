# Bug: encode_neon_ext masks an out-of-range EXT index instead of rejecting it
**Law:** EXT index must be in [0,7] for .8B and [0,15] for .16B; encode_neon_ext([Vd.T, Vn.T, Vm.T, #i]) = Err when i is outside that range (including negative)
**Impact:** `ext v0.8b, v1.8b, v2.8b, #8` encodes UNALLOCATED (Q=0 and imm4<3>=1). A typo or computed index silently produces a different or illegal instruction. gas rejects with "immediate value out of range 0 to 7".
**Function:** encode_neon_ext
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:405
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_ext([v0.8b, v0.8b, v0.8b, #8])
**Expected:** Err (gas: immediate value out of range 0 to 7)
**Actual:** Ok(Word(0x2e004000)) — imm4=8, UNALLOCATED for Q=0
**Severity:** medium
**Root cause:** neon.rs:412 casts the immediate as u32 with no range check; neon.rs:418 then does `index & 0xF`, so 8B index 8–15 and negative i64 values (wrap to high bits, then & 0xF) are encoded.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:418`
```rust
        | (rm << 16)) | ((index & 0xF) << 11)) | (rn << 5) | rd;
```
**Suggested fix:** Reject index outside the arrangement's byte count before encoding.
```rust
    let max = if arr_d == "16b" { 15 } else { 7 };
    if index > max {
        return Err(format!("EXT index {} out of range 0 to {}", index, max));
    }
    let word = ((((q << 30) | (0b101110 << 24))
        | (rm << 16)) | (index << 11)) | (rn << 5) | rd;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ext_regression_index_oor -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_ext_pbt::test_encode_neon_ext_regression_index_oor' panicked at src/backend/arm/assembler/encoder/encode_neon_ext_pbt.rs:559:5:
ext v0.8b, v1.8b, v2.8b, #8 must Err (gas range for .8b is [0, 7])
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_ext_pbt.rs
