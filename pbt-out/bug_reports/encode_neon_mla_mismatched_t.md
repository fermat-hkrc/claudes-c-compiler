# Bug: encode_neon_mla ignores source arrangement mismatch
**Law:** Vector MLA requires matching T on Vd, Vn, and Vm; mismatched arrangements must be rejected
**Impact:** A width typo such as `mla v0.8b, v0.8b, v0.16b` assembles as dest-only 8B MLA instead of failing, so the assembled object silently uses the wrong source shape
**Function:** encode_neon_mla
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:349
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_mla([v0.8b, v0.8b, v0.16b])
**Expected:** Err (ARM/gas/llvm-mc require matching T)
**Actual:** Ok(Word(0x0e209400)) — encoded as `mla v0.8b, v0.8b, v0.8b`
**Severity:** medium
**Root cause:** neon.rs:351-352 discard Vn/Vm arrangements (`_`); only dest arr_d is passed to neon_arr_to_q_size
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:351`
```rust
    let (rn, _) = get_neon_reg(operands, 1)?;
```
**Suggested fix:** Require all three arrangements to be equal and in the MLA T set
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    let (rm, arr_m) = get_neon_reg(operands, 2)?;
    if arr_d != arr_n || arr_n != arr_m {
        return Err(format!("mla requires matching T, got {arr_d}/{arr_n}/{arr_m}"));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_mla_regression_mismatched_t -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_mla_pbt::encode_neon_mla_neg_invalid_t' panicked at src/backend/arm/assembler/encoder/encode_neon_mla_pbt.rs:329:1:
Test failed: invalid/mismatched/reserved T must Err (ARM MLA T in {8B,16B,4H,8H,2S,4S} matching; llvm-mc rejects mla v0.8b, v0.8b, v0.16b) at src/backend/arm/assembler/encoder/encode_neon_mla_pbt.rs:349.
minimal failing input: rd = 0, rn = 0, rm = 0, td = "8b", tn = "8b", tm = "16b"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_mla_pbt.rs
