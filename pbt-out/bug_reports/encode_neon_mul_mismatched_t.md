# Bug: encode_neon_mul ignores source arrangements
**Law:** Vector MUL requires matching T on Vd, Vn, and Vm; a mismatched arrangement must be rejected
**Impact:** `mul v0.8b, v0.8b, v0.16b` is encoded as `mul v0.8b, v0.8b, v0.8b`, so a width mismatch is silently coerced to the destination arrangement
**Function:** encode_neon_mul
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:323
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_mul([v0.8b, v0.8b, v0.16b])
**Expected:** Err (ARM / llvm-mc require matching T)
**Actual:** Ok(Word(0x0e209c00)) — dest T drives Q/size; source T discarded
**Severity:** medium
**Root cause:** neon.rs:325-326 bind source arrangements to `_`, so only dest T is used
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:325`
```rust
    let (rn, _) = get_neon_reg(operands, 1)?;
    let (rm, _) = get_neon_reg(operands, 2)?;
```
**Suggested fix:** Require Vn.T and Vm.T to match Vd.T
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    let (rm, arr_m) = get_neon_reg(operands, 2)?;
    if arr_n != arr_d || arr_m != arr_d {
        return Err(format!("MUL arrangement mismatch: {arr_d} vs {arr_n} vs {arr_m}"));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_mul_regression_mismatched_t -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_mul_pbt::encode_neon_mul_neg_invalid_t' panicked at src/backend/arm/assembler/encoder/encode_neon_mul_pbt.rs:327:1:
Test failed: invalid/mismatched/reserved T must Err (ARM MUL T in {8B,16B,4H,8H,2S,4S} matching; llvm-mc rejects mul v0.8b, v0.8b, v0.16b) at src/backend/arm/assembler/encoder/encode_neon_mul_pbt.rs:347.
minimal failing input: rd = 0, rn = 0, rm = 0, td = "8b", tn = "8b", tm = "16b"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_mul_pbt.rs
