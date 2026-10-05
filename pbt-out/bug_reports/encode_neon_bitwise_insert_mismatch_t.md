# Bug: encode_neon_bitwise_insert ignores mismatched source arrangements
**Law:** BIT/BIF require matching T on Vd, Vn, and Vm; a mismatched arrangement must be Err
**Impact:** `bit v0.16b, v0.8b, v0.8b` encodes as 16B BIT using dest Q only, so a width mismatch is silently accepted
**Function:** encode_neon_bitwise_insert
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1667
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_bitwise_insert([v0.16b, v0.8b, v0.8b], size=0b10)
**Expected:** Err (llvm-mc/gas: operand mismatch)
**Actual:** Ok(Word) — Q taken from dest "16b" only
**Severity:** medium
**Root cause:** neon.rs:1672–1673 discard source arrangements (`let (rn, _)`, `let (rm, _)`); only dest T is read for Q
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1672`
```rust
    let (rn, _) = get_neon_reg(operands, 1)?;
    let (rm, _) = get_neon_reg(operands, 2)?;
```
**Suggested fix:** Require Vn.T and Vm.T to equal Vd.T
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    let (rm, arr_m) = get_neon_reg(operands, 2)?;
    if arr_n != arr_d || arr_m != arr_d {
        return Err("bit/bif: operand arrangement mismatch".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_bitwise_insert_regression_mismatch_t -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_bitwise_insert_pbt::encode_neon_bitwise_insert_neg_mismatch_t' panicked at src/backend/arm/assembler/encoder/encode_neon_bitwise_insert_pbt.rs:185:1:
Test failed: mismatched T must Err (llvm-mc rejects bit v0.16b, v0.8b, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_bitwise_insert_pbt.rs:352.
minimal failing input: rd = 0, rn = 0, rm = 0, td = "16b", tn = "8b", tm = "8b", size = 2
	successes: 0
	local rejects: 0
	global rejects: 1
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_bitwise_insert_pbt.rs
