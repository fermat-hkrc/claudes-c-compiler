# Bug: encode_neon_addv encodes reserved ADDV arrangements
**Law:** ADDV T is {8B,16B,4H,8H,4S}; size:Q=10:0 (2S) and size=11 (1D/2D) are reserved and must be rejected
**Impact:** Illegal `addv s0, v0.2s` (and 1d/2d) is assembled into a reserved encoding instead of being diagnosed, so the object file does not match gas/llvm-mc
**Function:** encode_neon_addv
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:424
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_addv([s0, v0.2s])
**Expected:** Err (llvm-mc: invalid operand; ARM size:Q=10:0 reserved)
**Actual:** Ok(Word(0x0eb0dc00))
**Severity:** medium
**Root cause:** neon.rs:431 calls neon_arr_to_q_size on the source arrangement with no ADDV T filter, so 2s/1d/2d become size/Q fields
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:431`
```rust
    let (q, size) = neon_arr_to_q_size(&arr_n)?;
```
**Suggested fix:** Reject arrangements other than 8b/16b/4h/8h/4s before encoding
```rust
    let (q, size) = neon_arr_to_q_size(&arr_n)?;
    if matches!((q, size), (0, 0b10) | (_, 0b11)) {
        return Err(format!("addv: unsupported arrangement: {}", arr_n));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_addv_regression_reserved_2s -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_addv_pbt::encode_neon_addv_neg_invalid_t' panicked at src/backend/arm/assembler/encoder/encode_neon_addv_pbt.rs:245:1:
Test failed: invalid T must Err (ARM reserved/unsupported; llvm-mc rejects addv s0, v0.2s) at src/backend/arm/assembler/encoder/encode_neon_addv_pbt.rs:359.
minimal failing input: rd = 0, rn = 0, t = "2s"
	successes: 2
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_addv_pbt.rs
