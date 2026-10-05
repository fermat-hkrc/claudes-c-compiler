# Bug: encode_neon_pmul ignores source arrangement mismatch
**Law:** PMUL requires matching T on Vd, Vn, and Vm; mismatched arrangements must be rejected
**Impact:** `pmul v0.8b, v0.8b, v0.16b` is encoded as 8B PMUL, so a width typo assembles to the wrong (or an unintended) instruction instead of failing
**Function:** encode_neon_pmul
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:336
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_pmul([v0.8b, v0.8b, v0.16b])
**Expected:** Err (ARM/gas/llvm-mc require matching T in {8B,16B})
**Actual:** Ok(Word(0x2e209c00)) — dest-only Q=0 8B encoding
**Severity:** medium
**Root cause:** neon.rs:338-339 discard Vn/Vm arrangements (`_`); only dest `arr_d` is consulted, so a mismatched source T is never compared
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:338`
```rust
    let (rn, _) = get_neon_reg(operands, 1)?;
    let (rm, _) = get_neon_reg(operands, 2)?;
    let q: u32 = if arr_d == "16b" { 1 } else { 0 };
```
**Suggested fix:** Require all three arrangements to be equal and in {8b,16b}
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    let (rm, arr_m) = get_neon_reg(operands, 2)?;
    if arr_d != arr_n || arr_n != arr_m || (arr_d != "8b" && arr_d != "16b") {
        return Err(format!("pmul requires matching T in {{8b,16b}}, got {arr_d}/{arr_n}/{arr_m}"));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_pmul_regression_mismatched_t -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_pmul_pbt::encode_neon_pmul_neg_invalid_t' panicked at src/backend/arm/assembler/encoder/encode_neon_pmul_pbt.rs:309:1:
Test failed: invalid/mismatched/reserved T must Err (ARM PMUL T in {8B,16B} matching; llvm-mc rejects pmul v0.8b, v0.8b, v0.16b) at src/backend/arm/assembler/encoder/encode_neon_pmul_pbt.rs:329.
minimal failing input: rd = 0, rn = 0, rm = 0, td = "8b", tn = "8b", tm = "16b"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_pmul_pbt.rs
