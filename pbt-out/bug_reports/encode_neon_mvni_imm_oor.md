# Bug: encode_neon_mvni truncates out-of-range immediates
**Law:** Immediates outside [0, 255] must be rejected, matching llvm-mc/gas
**Impact:** `mvni v0.4h, #256` encodes as `#0` and `mvni v0.4s, #-1` encodes as `#255`; the assembler silently produces the wrong immediate
**Function:** encode_neon_mvni
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1333
**Detected by:** Negative/error contract vs llvm-mc
**Minimal input:** encode_neon_mvni([v0.4h, #256])
**Expected:** Err (llvm-mc: immediate must be an integer in range [0, 255])
**Actual:** Ok(Word) encoding `mvni v0.4h, #0` (256 as u32 & 0xFF == 0)
**Severity:** high
**Root cause:** neon.rs:1338 masks the immediate with `imm as u32 & 0xFF` instead of range-checking
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1338`
```rust
    let imm8 = imm as u32 & 0xFF;
```
**Suggested fix:** Reject values outside the 8-bit unsigned range before encoding
```rust
    if imm < 0 || imm > 255 {
        return Err(format!("mvni: immediate {} out of range [0, 255]", imm));
    }
    let imm8 = imm as u32;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_mvni_neg_imm_oor_invalid_t -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_mvni_pbt::encode_neon_mvni_neg_imm_oor_invalid_t' panicked at src/backend/arm/assembler/encoder/encode_neon_mvni_pbt.rs:278:1:
Test failed: OOR imm / invalid T must Err (llvm-mc rejects mvni v0.4h, #256)
minimal failing input: rd = 0, kind = 0, t_ok = "4h", t_bad = "8b", over = 1, neg = 1
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_mvni_pbt.rs (test_encode_neon_mvni_regression_imm_oor)
