# Bug: encode_neon_sshr accepts a shift of 0 (and other values outside [1, esize])
**Law:** SSHR immediate shift must be in [1, esize(T)]; out-of-range shifts must be rejected
**Impact:** `sshr v0.8b, v0.8b, #0` is encoded instead of diagnosed. llvm-mc/gas require [1, 8] for .8b. Out-of-range shifts are masked into immh:immb (sometimes a reserved immh=0000 encoding, sometimes wrapping to a valid shift); debug builds also panic with "attempt to subtract with overflow" when `16 - shift` underflows
**Function:** encode_neon_sshr
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1205
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_sshr([v0.8b, v0.8b, #0])
**Expected:** Err (ARM/llvm-mc require shift in [1, 8] for .8b)
**Actual:** Ok(Word) — reserved-looking encoding with immh:immb = 0
**Severity:** medium
**Root cause:** neon.rs:1218 computes `(16 - shift) & 0xF` with no range check, so shift=0 and shift>esize wrap into the immh:immb field
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1218`
```rust
        "8b" | "16b" => (16 - shift) & 0xF,
```
**Suggested fix:** Reject shift outside [1, esize] before encoding
```rust
        "8b" | "16b" => {
            if !(1..=8).contains(&shift) {
                return Err(format!("sshr shift {} out of range [1, 8]", shift));
            }
            16 - shift
        }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_sshr_regression_shift_oob -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_sshr_pbt::encode_neon_sshr_neg_shift_oob' (2343225) panicked at src/backend/arm/assembler/encoder/encode_neon_sshr_pbt.rs:263:1:
Test failed: shift 0 not in [1, 8] must Err (llvm-mc rejects sshr v0.8b, v0.8b, #0) at src/backend/arm/assembler/encoder/encode_neon_sshr_pbt.rs:416.
minimal failing input: rd = 0, rn = 0, t_shift = (
    "8b",
    0,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_sshr_pbt.rs
