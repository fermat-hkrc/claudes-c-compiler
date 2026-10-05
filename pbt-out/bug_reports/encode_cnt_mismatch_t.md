# Bug: encode_cnt ignores a mismatched source arrangement
**Law:** CNT Vd.<T>, Vn.<T> requires the same T on both operands
**Impact:** `cnt v0.8b, v0.16b` is encoded as `cnt v0.8b, v0.8b` (dest T wins, source T dropped), so a mixed-width form that gas/llvm-mc reject becomes silent wrong code
**Function:** encode_cnt
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:23
**Detected by:** Negative/Error Contract
**Minimal input:** encode_cnt([v0.8b, v0.16b])
**Expected:** Err (llvm-mc/gas: operand mismatch)
**Actual:** Ok(Word(0x0e205800)) — Q taken from dest only
**Severity:** medium
**Root cause:** neon.rs:32 binds source arrangement as `_arr_n` and never compares it to dest
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:32`
```rust
    let (rn, _arr_n) = get_neon_reg(operands, 1)?;
```
**Suggested fix:** Require matching arrangements
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    if arr_d != arr_n {
        return Err(format!("cnt: arrangement mismatch .{arr_d} vs .{arr_n}"));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_cnt_regression_mismatch_t -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_cnt_pbt::encode_cnt_neg_mismatch_t' panicked at src/backend/arm/assembler/encoder/encode_cnt_pbt.rs:166:1:
Test failed: mismatched T must Err (llvm-mc rejects cnt v0.8b, v0.16b) at src/backend/arm/assembler/encoder/encode_cnt_pbt.rs:295.
minimal failing input: rd = 0, rn = 0, td = "8b"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_cnt_pbt.rs
