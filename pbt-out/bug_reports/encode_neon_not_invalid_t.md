# Bug: encode_neon_not encodes illegal arrangements as NOT .8b
**Law:** NOT is only valid for T in {8b, 16b}; any other arrangement must be rejected
**Impact:** `not v0.4h, v0.4h` (and 8h/2s/4s/2d/1d) is assembled as `not v0.8b, v0.8b` (0x2e205800), emitting the wrong instruction instead of an error
**Function:** encode_neon_not
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:608
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_not([v0.4h, v0.4h])
**Expected:** Err (llvm-mc: invalid operand; gas: operand mismatch; ARM: T in {8B,16B} only)
**Actual:** Ok(Word(0x2e205800))
**Severity:** high
**Root cause:** neon.rs:615 sets Q=1 only for "16b" and Q=0 for every other arrangement, with no check that T is 8b or 16b
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:615`
```rust
    let q: u32 = if arr_d == "16b" { 1 } else { 0 };
```
**Suggested fix:** Require T in {8b,16b} before encoding Q
```rust
    if arr_d != "8b" && arr_d != "16b" {
        return Err(format!("not: unsupported arrangement .{arr_d}, expected .8b or .16b"));
    }
    let q: u32 = if arr_d == "16b" { 1 } else { 0 };
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_not_regression_invalid_t -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_not_pbt::encode_neon_not_neg_invalid_t' panicked at src/backend/arm/assembler/encoder/encode_neon_not_pbt.rs:167:1:
Test failed: invalid T must Err (only .8b/.16b; llvm-mc rejects not v0.4h, v0.4h) at src/backend/arm/assembler/encoder/encode_neon_not_pbt.rs:278.
minimal failing input: rd = 0, rn = 0, t = "4h"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_not_pbt.rs
