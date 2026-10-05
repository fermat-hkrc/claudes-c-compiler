# Bug: encode_neon_sshr ignores the source arrangement
**Law:** SSHR is `Vd.T, Vn.T, #shift` with matching T; mismatched or reserved arrangements must be rejected
**Impact:** Invalid assembly `sshr v0.2s, v0.8b, #1` is encoded as `sshr v0.2s, v0.2s, #1` (size taken only from the destination), so the assembler emits a different instruction than the source text
**Function:** encode_neon_sshr
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1205
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_sshr([v0.2s, v0.8b, #1])
**Expected:** Err (ARM/llvm-mc require matching T; llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word) — encoded as if both were .2s
**Severity:** medium
**Root cause:** neon.rs:1210 discards the source arrangement (`let (rn, _)`), so Td/Tn never compared and 1d/1q on the source never rejected
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1210`
```rust
    let (rn, _) = get_neon_reg(operands, 1)?;
```
**Suggested fix:** Require matching arrangements and a valid SSHR T
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    if arr_n != arr_d {
        return Err(format!("sshr arrangement mismatch: {} vs {}", arr_d, arr_n));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_sshr_regression_mismatched_t -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_sshr_pbt::encode_neon_sshr_neg_invalid_t' (2343207) panicked at src/backend/arm/assembler/encoder/encode_neon_sshr_pbt.rs:263:1:
Test failed: invalid/mismatched/reserved T must Err (ARM SSHR T in {8B,16B,4H,8H,2S,4S,2D} matching; llvm-mc rejects sshr v0.2s, v0.8b, #1) at src/backend/arm/assembler/encoder/encode_neon_sshr_pbt.rs:392.
minimal failing input: rd = 0, rn = 0, td = "2s", tn = "8b", shift = 1
	successes: 1
	local rejects: 0
	global rejects: 1
		1 times at src/backend/arm/assembler/encoder/encode_neon_sshr_pbt.rs:383:13: shift >= 1 && shift <= esize(td)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_sshr_pbt.rs
