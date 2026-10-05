# Bug: encode_neon_zip_uzp encodes reserved 1D arrangement
**Law:** ZIP/UZP/TRN arrangements are {8B,16B,4H,8H,2S,4S,2D}; size:Q=11:0 (1D) is reserved and must be rejected
**Impact:** The assembler emits a reserved encoding for `zip1 v0.1d, v0.1d, v0.1d` that gas and llvm-mc reject, so invalid SIMD permute assembly becomes a silent 32-bit word
**Function:** encode_neon_zip_uzp
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1094
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_zip_uzp([v0.1d, v0.1d, v0.1d], 0b011, false)
**Expected:** Err (ARM reserved; llvm-mc/gas reject `*.1d`)
**Actual:** Ok(Word(0x0ec03800))
**Severity:** medium
**Root cause:** neon.rs:1101 calls neon_arr_to_q_size which maps "1d" to (Q=0, size=11); encode_neon_zip_uzp does not reject that reserved pair
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1101`
```rust
    let (q, size) = neon_arr_to_q_size(&arr_d)?;
```
**Suggested fix:** Reject size:Q = 11:0 (1D) after decoding Q/size
```rust
    let (q, size) = neon_arr_to_q_size(&arr_d)?;
    if size == 0b11 && q == 0 {
        return Err("uzp/zip: 1d arrangement is reserved".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_zip_uzp_regression_reserved_1d -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_zip_uzp_pbt::encode_neon_zip_uzp_neg_reserved_1d' panicked at src/backend/arm/assembler/encoder/encode_neon_zip_uzp_pbt.rs:187:1:
Test failed: reserved 1d must Err (llvm-mc/gas reject zip1 v0.1d, v0.1d, v0.1d) at src/backend/arm/assembler/encoder/encode_neon_zip_uzp_pbt.rs:337.
minimal failing input: rd = 0, rn = 0, rm = 0, m = "zip1"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_zip_uzp_pbt.rs
