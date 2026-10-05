# Bug: encode_neon_pmul encodes reserved non-byte T as 8B
**Law:** ARM PMUL T is 8B or 16B only; any other arrangement must be rejected
**Impact:** `pmul v0.4h, v0.4h, v0.4h` (and 8h/2s/4s/1d/2d/1q) is silently encoded as 8B PMUL (Q=0), so invalid SIMD widths assemble to a different instruction
**Function:** encode_neon_pmul
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:336
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_pmul([v0.4h, v0.4h, v0.4h])
**Expected:** Err (ARM PMUL T is 8B/16B only; llvm-mc rejects)
**Actual:** Ok(Word(0x2e209c00)) — same as `pmul v0.8b, v0.8b, v0.8b`
**Severity:** medium
**Root cause:** neon.rs:340 sets Q=1 only when `arr_d == "16b"`, else Q=0, with size hardcoded to 00. Non-byte T is therefore encoded as 8B instead of rejected. The doc comment says "bytes only" but does not declare other T invalid at the API; the assembler still accepts the operands.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:340`
```rust
    let q: u32 = if arr_d == "16b" { 1 } else { 0 };
```
**Suggested fix:** Accept only 8b/16b and error otherwise
```rust
    let q: u32 = match arr_d.as_str() {
        "8b" => 0,
        "16b" => 1,
        _ => return Err(format!("pmul T must be 8b or 16b, got {arr_d}")),
    };
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_pmul_regression_reserved_4h -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_pmul_pbt::encode_neon_pmul_neg_reserved_t' panicked at src/backend/arm/assembler/encoder/encode_neon_pmul_pbt.rs:441:1:
Test failed: reserved PMUL T=4h (ARM T in {8B,16B} only) must Err (llvm-mc rejects pmul v0.4h, v0.4h, v0.4h) at src/backend/arm/assembler/encoder/encode_neon_pmul_pbt.rs:457.
minimal failing input: rd = 0, rn = 0, rm = 0, t = "4h"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_pmul_pbt.rs
