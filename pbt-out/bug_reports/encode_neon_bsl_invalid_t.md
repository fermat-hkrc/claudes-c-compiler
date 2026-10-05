# Bug: encode_neon_bsl encodes illegal BSL arrangements as .8b
**Law:** BSL T is only 8B or 16B; any other arrangement must be rejected
**Impact:** `bsl v0.4h, v0.4h, v0.4h` (and .8h/.2s/.4s/.2d/.1d) is silently encoded as BSL .8B (Q=0), so the object file contains a different instruction than the assembly text
**Function:** encode_neon_bsl
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:735
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_bsl([v0.4h, v0.4h, v0.4h])
**Expected:** Err (llvm-mc: invalid operand; ARM BSL T ∈ {8B,16B})
**Actual:** Ok(Word(0x2e601c00)) — identical to `bsl v0.8b, v0.8b, v0.8b`
**Severity:** medium
**Root cause:** neon.rs:743 sets Q=1 only when arr_d == "16b" and otherwise Q=0, with no check that T is 8b or 16b
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:743`
```rust
    let q: u32 = if arr_d == "16b" { 1 } else { 0 };
```
**Suggested fix:** Reject arrangements other than 8b/16b before encoding Q
```rust
    if arr_d != "8b" && arr_d != "16b" {
        return Err(format!("bsl: unsupported arrangement: {}", arr_d));
    }
    let q: u32 = if arr_d == "16b" { 1 } else { 0 };
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_bsl_regression_invalid_t -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_bsl_pbt::encode_neon_bsl_neg_invalid_t' panicked at src/backend/arm/assembler/encoder/encode_neon_bsl_pbt.rs:165:1:
Test failed: invalid T must Err (only .8b/.16b; llvm-mc rejects bsl v0.4h, v0.4h, v0.4h) at src/backend/arm/assembler/encoder/encode_neon_bsl_pbt.rs:292.
minimal failing input: rd = 0, rn = 0, rm = 0, t = "4h"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_bsl_pbt.rs
