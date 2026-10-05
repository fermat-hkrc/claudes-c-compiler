# Bug: encode_neon_ushr ignores a mismatched source arrangement
**Law:** USHR requires matching T on Vd and Vn; a dest/src arrangement mismatch must be rejected
**Impact:** `ushr v0.8b, v0.16b, #1` is encoded as 8-bit USHR using only the destination arrangement, so the assembler accepts assembly gas and llvm-mc reject and emits the wrong element size if the programmer wrote a 16-bit source
**Function:** encode_neon_ushr
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1179
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_ushr([v0.8b, v0.16b, #1])
**Expected:** Err (ARM/gas/llvm-mc require matching T)
**Actual:** Ok(Word) — same as `ushr v0.8b, v0.8b, #1`
**Severity:** medium
**Root cause:** neon.rs:1184 discards the source arrangement (`let (rn, _)`), so Vn.T is never compared with Vd.T
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1184`
```rust
    let (rn, _) = get_neon_reg(operands, 1)?;
```
**Suggested fix:** Require matching arrangements
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    if arr_n != arr_d {
        return Err(format!("ushr arrangement mismatch: {} vs {}", arr_d, arr_n));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ushr_regression_mismatched_t -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_ushr_pbt::encode_neon_ushr_neg_invalid_t' panicked at src/backend/arm/assembler/encoder/encode_neon_ushr_pbt.rs:263:1:
Test failed: invalid/mismatched/reserved T must Err (ARM USHR T in {8B,16B,4H,8H,2S,4S,2D} matching; llvm-mc rejects ushr v0.8b, v0.16b, #1) at src/backend/arm/assembler/encoder/encode_neon_ushr_pbt.rs:392.
minimal failing input: rd = 0, rn = 0, td = "8b", tn = "16b", shift = 1
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_ushr_pbt.rs
