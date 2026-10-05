# Bug: encode_neon_across encodes reserved across-lanes arrangements
**Law:** UMAXV/UMINV/SMAXV/SMINV T is {8B,16B,4H,8H,4S}; size:Q=10:0 (2S) and size=11 (1D/2D) are reserved and must be rejected
**Impact:** Illegal `umaxv s0, v0.2s` (and 1d/2d) is assembled into a reserved encoding instead of being diagnosed, so the object file does not match gas/llvm-mc
**Function:** encode_neon_across
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:445
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_across([s0, v0.2s], 1, 0b01010)
**Expected:** Err (llvm-mc: invalid operand; ARM size:Q=10:0 reserved)
**Actual:** Ok(Word(0x2eb0a800))
**Severity:** medium
**Root cause:** neon.rs:452 calls neon_arr_to_q_size on the source arrangement with no across-lanes T filter, so 2s/1d/2d become size/Q fields
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:452`
```rust
    let (q, size) = neon_arr_to_q_size(&arr_n)?;
```
**Suggested fix:** Reject arrangements other than 8b/16b/4h/8h/4s before encoding
```rust
    let (q, size) = neon_arr_to_q_size(&arr_n)?;
    if matches!((q, size), (0, 0b10) | (_, 0b11)) {
        return Err(format!("NEON across-vector: unsupported arrangement: {}", arr_n));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_across_regression_reserved_2s -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_across_pbt::encode_neon_across_neg_invalid_t' panicked at src/backend/arm/assembler/encoder/encode_neon_across_pbt.rs:297:1:
Test failed: invalid T must Err (ARM reserved/unsupported; llvm-mc rejects umaxv s0, v0.2s) at src/backend/arm/assembler/encoder/encode_neon_across_pbt.rs:443.
minimal failing input: rd = 0, rn = 0, t = "2s", (mnem, u_bit, opcode) = (
    "umaxv",
    1,
    10,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_across_pbt.rs
