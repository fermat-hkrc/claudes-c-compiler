# Bug: encode_neon_zip_uzp ignores mismatched source arrangements
**Law:** ZIP/UZP/TRN require matching arrangements on Vd, Vn, and Vm
**Impact:** The assembler encodes `zip1 v0.8b, v0.8b, v0.16b` as `zip1 v0.8b, v0.8b, v0.8b`, silently dropping the source arrangement mismatch that gas and llvm-mc diagnose
**Function:** encode_neon_zip_uzp
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1094
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_zip_uzp([v0.8b, v0.8b, v0.16b], 0b011, false)
**Expected:** Err (llvm-mc/gas require matching T)
**Actual:** Ok(Word(0x0e003800)) — same as `zip1 v0.8b, v0.8b, v0.8b`
**Severity:** medium
**Root cause:** neon.rs:1099-1100 discard source arrangements (`let (rn, _)`, `let (rm, _)`), so only dest T is checked
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1099`
```rust
    let (rn, _) = get_neon_reg(operands, 1)?;
    let (rm, _) = get_neon_reg(operands, 2)?;
```
**Suggested fix:** Require source arrangements to match dest T
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    let (rm, arr_m) = get_neon_reg(operands, 2)?;
    if arr_n != arr_d || arr_m != arr_d {
        return Err(format!("uzp/zip: mismatched arrangements {arr_d}/{arr_n}/{arr_m}"));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_zip_uzp_regression_mismatched_t -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_zip_uzp_pbt::encode_neon_zip_uzp_neg_mismatched_t' panicked at src/backend/arm/assembler/encoder/encode_neon_zip_uzp_pbt.rs:187:1:
Test failed: mismatched T must Err (llvm-mc/gas reject zip1 v0.8b, v0.8b, v0.16b) at src/backend/arm/assembler/encoder/encode_neon_zip_uzp_pbt.rs:362.
minimal failing input: rd = 0, rn = 0, rm = 0, td = "8b", tn = "8b", tm = "16b", m = "zip1"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_zip_uzp_pbt.rs
