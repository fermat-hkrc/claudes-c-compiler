# Bug: encode_neon_mls ignores source arrangement mismatch
**Law:** Vector MLS requires matching T on Vd, Vn, and Vm; mismatched arrangements must be rejected
**Impact:** A width typo such as `mls v0.8b, v0.16b, v0.8b` assembles as dest-only 8B MLS instead of failing, so the assembled object silently uses the wrong source shape
**Function:** encode_neon_mls
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:361
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_mls([v0.8b, v0.16b, v0.8b])
**Expected:** Err (ARM/gas/llvm-mc require matching T)
**Actual:** Ok(Word(0x2e209400)) — encoded as `mls v0.8b, v0.8b, v0.8b`
**Severity:** medium
**Root cause:** neon.rs:363-364 discard Vn/Vm arrangements (`_`); only dest arr_d is passed to neon_arr_to_q_size
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:363`
```rust
    let (rn, _) = get_neon_reg(operands, 1)?;
```
**Suggested fix:** Require all three arrangements to be equal and in the MLS T set
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    let (rm, arr_m) = get_neon_reg(operands, 2)?;
    if arr_d != arr_n || arr_n != arr_m {
        return Err(format!("mls requires matching T, got {arr_d}/{arr_n}/{arr_m}"));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_mls_regression_mismatched_t -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_mls_pbt::encode_neon_mls_neg_invalid_t' panicked at src/backend/arm/assembler/encoder/encode_neon_mls_pbt.rs:330:1:
Test failed: invalid/mismatched/reserved T must Err (ARM MLS T in {8B,16B,4H,8H,2S,4S} matching; llvm-mc rejects mls v0.8b, v0.16b, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_mls_pbt.rs:350.
minimal failing input: rd = 0, rn = 0, rm = 0, td = "8b", tn = "16b", tm = "8b"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_mls_pbt.rs
