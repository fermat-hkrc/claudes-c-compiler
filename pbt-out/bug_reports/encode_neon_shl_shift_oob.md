# Bug: encode_neon_shl accepts a shift outside [0, esize-1] (debug overflow / wrap)
**Law:** SHL immediate shift must be in [0, esize(T)-1]; out-of-range shifts must be rejected
**Impact:** `shl v0.8b, v0.8b, #-1` panics in debug with "attempt to add with overflow" instead of diagnosing invalid assembly. `shl v0.8b, v0.8b, #8` is encoded (immh:immb wrapped to 0, a reserved immh=0000 encoding) instead of rejected. llvm-mc/gas require [0, 7] for .8b
**Function:** encode_neon_shl
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1231
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_shl([v0.8b, v0.8b, #-1])
**Expected:** Err (ARM/llvm-mc require shift in [0, 7] for .8b)
**Actual:** debug panic "attempt to add with overflow"; for shift=8, Ok(Word) with reserved immh=0000
**Severity:** medium
**Root cause:** neon.rs:1237 casts the i64 immediate to u32 then neon.rs:1245 computes `(8 + shift) & 0xF` with no range check, so negative shifts overflow in debug and shift>=esize wrap into the immh:immb field
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1245`
```rust
        "8b" | "16b" => (8 + shift) & 0xF,
```
**Suggested fix:** Reject shift outside [0, esize-1] before encoding
```rust
        "8b" | "16b" => {
            if shift > 7 {
                return Err(format!("shl shift {} out of range [0, 7]", shift));
            }
            8 + shift
        }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_shl_regression_shift_oob -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_shl_pbt::encode_neon_shl_neg_shift_oob' (2353161) panicked at src/backend/arm/assembler/encoder/encode_neon_shl_pbt.rs:265:1:
Test failed: attempt to add with overflow.
minimal failing input: rd = 0, rn = 0, t_shift = (
    "8b",
    -1,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_shl_pbt.rs
