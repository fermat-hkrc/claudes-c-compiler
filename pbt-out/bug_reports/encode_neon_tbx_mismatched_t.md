# Bug: encode_neon_tbx ignores Vm.Ta vs Vd.Ta mismatch
**Law:** ARM TBX requires Vm.Ta = Vd.Ta; llvm-mc rejects `tbx v0.8b, {v0.16b}, v0.16b`, so encode_neon_tbx with mismatched Ta = Err
**Impact:** `tbx v0.8b, {v0.16b}, v0.16b` is assembled using only Vd's Q bit, so a mismatched index arrangement is silently accepted.
**Function:** encode_neon_tbx
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:803
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_tbx([v0.8b, {v0.16b}, v0.16b])
**Expected:** Err (llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word(0x0e001000))
**Severity:** medium
**Root cause:** neon.rs:822 binds `(rm, _)` and discards Vm's arrangement.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:822`
```rust
    let (rm, _) = get_neon_reg(operands, 2)?;
```
**Suggested fix:** Compare Vm arrangement to Vd's Ta.
```rust
    let (rm, arr_m) = get_neon_reg(operands, 2)?;
    if arr_m != arr_d {
        return Err(format!("tbx: Vm.Ta ({}) must match Vd.Ta ({})", arr_m, arr_d));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_tbx_regression_mismatched_t -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_tbx_pbt::test_encode_neon_tbx_regression_mismatched_t' panicked at src/backend/arm/assembler/encoder/encode_neon_tbx_pbt.rs:695:5:
tbx v0.8b, {v0.16b}, v0.16b must Err (Vd.Ta must equal Vm.Ta)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_tbx_pbt.rs
