# Bug: encode_neon_rev64 encodes reserved .1d/.2d arrangements
**Law:** NEON REV64 is valid only for T in {8B,16B,4H,8H,2S,4S}; size=11 (.1d/.2d) is reserved and must be rejected
**Impact:** The assembler emits a reserved Advanced SIMD encoding for `rev64 v0.1d, v0.1d` / `rev64 v0.2d, v0.2d` that llvm-mc and gas reject as invalid operands, so assembled objects disagree with the claimed gas-compatible assembler
**Function:** encode_neon_rev64
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:752
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_rev64([v0.1d, v0.1d])
**Expected:** Err (llvm-mc: invalid operand; gas: operand mismatch, valid variants {8b,16b,4h,8h,2s,4s})
**Actual:** Ok(Word(0x0ee00800)) — size field 0b11 (reserved)
**Severity:** medium
**Root cause:** neon.rs:759 calls neon_arr_to_q_size which maps "1d"/"2d" to size=11; encode_neon_rev64 never rejects the reserved size
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:759`
```rust
    let (q, size) = neon_arr_to_q_size(&arr_d)?;
```
**Suggested fix:** Reject size=11 (and any T not in {8b,16b,4h,8h,2s,4s})
```rust
    let (q, size) = neon_arr_to_q_size(&arr_d)?;
    if size == 0b11 {
        return Err(format!("rev64: unsupported arrangement .{arr_d}"));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_rev64_regression_invalid_t -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_rev64_pbt::encode_neon_rev64_neg_invalid_t' (2294846) panicked at src/backend/arm/assembler/encoder/encode_neon_rev64_pbt.rs:205:1:
Test failed: invalid T must Err (only .8b/.16b/.4h/.8h/.2s/.4s; llvm-mc rejects rev64 v0.1d, v0.1d) at src/backend/arm/assembler/encoder/encode_neon_rev64_pbt.rs:321.
minimal failing input: rd = 0, rn = 0, t = "1d"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_rev64_pbt.rs
