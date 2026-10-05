# Bug: encode_neon_movi truncates out-of-range 8-bit immediates
**Law:** ∀ rd ∈ [0,31], T ∈ {8b,16b,4h,8h,2s,4s}, imm ∉ [0,255]. llvm-mc rejects ∧ encode_neon_movi([Vd.T, #imm]) = Err
**Impact:** `movi v0.8b, #256` is accepted and encoded as `movi v0.8b, #0` (`imm as u32 & 0xFF`). Negative immediates such as `#-1` become 255. A mistyped immediate silently wraps instead of failing assembly, so the object file contains a different constant than the source.
**Function:** encode_neon_movi
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:624
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_movi([v0.8b, #256])
**Expected:** Err (llvm-mc: immediate must be an integer in range [0, 255]; gas: out of range -128 to 255)
**Actual:** Ok(Word(0x0f00e400)) — same as `movi v0.8b, #0`
**Severity:** high
**Root cause:** neon.rs:637 (and the 2s/4s and 4h/8h copies) mask with `imm as u32 & 0xFF` instead of requiring the immediate to already be in 0..=255.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:637`
```rust
            let imm8 = imm as u32 & 0xFF;
```
**Suggested fix:** Reject immediates outside 0..=255 for 8-bit MOVI forms.
```rust
            if !(0..=255).contains(&imm) {
                return Err(format!("movi: immediate {} out of range [0, 255]", imm));
            }
            let imm8 = imm as u32;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_movi_regression_imm_oor -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_movi_pbt::test_encode_neon_movi_regression_imm_oor' (2277465) panicked at src/backend/arm/assembler/encoder/encode_neon_movi_pbt.rs:622:5:
movi v0.8b, #256 must Err (llvm-mc: immediate must be in range [0, 255])
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_movi_pbt.rs
