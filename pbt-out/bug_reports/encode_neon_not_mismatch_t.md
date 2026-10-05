# Bug: encode_neon_not ignores a mismatched source arrangement
**Law:** NOT requires matching arrangements on Vd.T and Vn.T
**Impact:** `not v0.8b, v0.16b` is assembled as `not v0.8b, v0.8b` (0x2e205800), using only the destination arrangement and silently dropping the source T
**Function:** encode_neon_not
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:608
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_not([v0.8b, v0.16b])
**Expected:** Err (llvm-mc: invalid operand; gas: operand mismatch)
**Actual:** Ok(Word(0x2e205800))
**Severity:** high
**Root cause:** neon.rs:613 discards the source arrangement (`let (rn, _) = get_neon_reg(operands, 1)?`), so dest T alone selects Q
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:613`
```rust
    let (rn, _) = get_neon_reg(operands, 1)?;
```
**Suggested fix:** Require the source arrangement to match the destination
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    if arr_n != arr_d {
        return Err(format!("not: arrangement mismatch .{arr_d} vs .{arr_n}"));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_not_regression_mismatch_t -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_not_pbt::encode_neon_not_neg_mismatch_t' panicked at src/backend/arm/assembler/encoder/encode_neon_not_pbt.rs:167:1:
Test failed: mismatched T must Err (llvm-mc rejects not v0.8b, v0.16b) at src/backend/arm/assembler/encoder/encode_neon_not_pbt.rs:299.
minimal failing input: rd = 0, rn = 0, td = "8b"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_not_pbt.rs
