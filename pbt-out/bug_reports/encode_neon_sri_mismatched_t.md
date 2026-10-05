# Bug: encode_neon_sri encodes mismatched dest/src arrangements
**Law:** Vector SRI requires matching arrangements on Vd.T and Vn.T
**Impact:** The assembler accepts `sri Vd.Td, Vn.Tn, #shift` with Td ≠ Tn (e.g. `.2s` dest and `.8b` src) and emits a word as if both used Td, so invalid assembly is assembled instead of diagnosed
**Function:** encode_neon_sri
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1285
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_sri([v0.2s, v0.8b, #1])
**Expected:** Err (ARM/llvm-mc require matching T)
**Actual:** Ok(Word) encoded from dest arrangement `.2s` only
**Severity:** medium
**Root cause:** neon.rs:1290 discards the source arrangement (`let (rn, _) = get_neon_reg(operands, 1)?`), so Tn is never compared to Td
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1290`
```rust
    let (rn, _) = get_neon_reg(operands, 1)?;
```
**Suggested fix:** Require the source arrangement to match the dest arrangement
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    if arr_n != arr_d {
        return Err(format!("sri arrangement mismatch: .{} vs .{}", arr_d, arr_n));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_sri_regression_mismatched_t -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_sri_pbt::encode_neon_sri_neg_invalid_t' (2357977) panicked at src/backend/arm/assembler/encoder/encode_neon_sri_pbt.rs:267:1:
Test failed: invalid/mismatched/reserved T must Err (ARM SRI T in {8B,16B,4H,8H,2S,4S,2D} matching; llvm-mc rejects sri v0.2s, v0.8b, #1) at src/backend/arm/assembler/encoder/encode_neon_sri_pbt.rs:396.
minimal failing input: rd = 0, rn = 0, td = "2s", tn = "8b", shift = 1
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_sri_pbt.rs
