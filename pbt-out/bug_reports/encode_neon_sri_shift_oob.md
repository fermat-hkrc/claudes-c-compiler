# Bug: encode_neon_sri encodes out-of-range shift immediates
**Law:** Vector SRI shift must be in [1, esize(T)]; values outside that range must be rejected
**Impact:** `sri v0.8b, v0.8b, #0` is accepted and encoded with reserved immh=0000. llvm-mc/gas reject it (`immediate must be an integer in range [1, 8]`). Negative shifts overflow-panic in debug at `16 - shift`.
**Function:** encode_neon_sri
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1285
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_sri([v0.8b, v0.8b, #0])
**Expected:** Err (ARM/llvm-mc require shift in [1, 8] for .8b)
**Actual:** Ok(Word) with immh:immb = 0 (unallocated)
**Severity:** medium
**Root cause:** neon.rs:1296-1301 computes `(2*esize - shift) & mask` with no range check, so shift=0 wraps to reserved immh=0000 instead of returning Err
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1297`
```rust
        "8b" | "16b" => (16 - shift) & 0xF,
        "4h" | "8h" => (32 - shift) & 0x1F,
        "2s" | "4s" => (64 - shift) & 0x3F,
        "2d" => (128 - shift) & 0x7F,
```
**Suggested fix:** Reject shift outside [1, esize] before encoding
```rust
        "8b" | "16b" if (1..=8).contains(&shift) => (16 - shift) & 0xF,
        "4h" | "8h" if (1..=16).contains(&shift) => (32 - shift) & 0x1F,
        "2s" | "4s" if (1..=32).contains(&shift) => (64 - shift) & 0x3F,
        "2d" if (1..=64).contains(&shift) => (128 - shift) & 0x7F,
        "8b" | "16b" | "4h" | "8h" | "2s" | "4s" | "2d" => {
            return Err(format!("sri shift {} out of range for {}", shift, arr_d));
        }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_sri_regression_shift_oob -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_sri_pbt::encode_neon_sri_neg_shift_oob' (2357992) panicked at src/backend/arm/assembler/encoder/encode_neon_sri_pbt.rs:267:1:
Test failed: shift 0 not in [1, 8] must Err (llvm-mc rejects sri v0.8b, v0.8b, #0) at src/backend/arm/assembler/encoder/encode_neon_sri_pbt.rs:420.
minimal failing input: rd = 0, rn = 0, t_shift = (
    "8b",
    0,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_sri_pbt.rs
