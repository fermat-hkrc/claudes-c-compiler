# Bug: encode_neon_three_same ignores source arrangements
**Law:** Advanced SIMD three-same requires matching arrangement T on Vd, Vn, and Vm; mismatched T must be rejected.
**Impact:** `cmeq v0.8b, v0.16b, v0.8b` encodes as if all three were `.8b`, producing a word that does not match the written assembly and that llvm-mc refuses.
**Function:** encode_neon_three_same
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:65
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** encode_neon_three_same([v0.8b, v0.16b, v0.8b], u=1, opcode=0b10001)
**Expected:** Err (llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word) using Q/size from the destination only
**Severity:** medium
**Root cause:** neon.rs:70-71 bind source arrangements as `_arr_n` / `_arr_m` and never compare them to `arr_d`.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:70`
```rust
    let (rn, _arr_n) = get_neon_reg(operands, 1)?;
    let (rm, _arr_m) = get_neon_reg(operands, 2)?;
```
**Suggested fix:** Require `arr_n == arr_d && arr_m == arr_d` after reading the three arrangements.
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    let (rm, arr_m) = get_neon_reg(operands, 2)?;
    if arr_n != arr_d || arr_m != arr_d {
        return Err(format!("NEON three-same arrangement mismatch: dest {arr_d}, src {arr_n}/{arr_m}"));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_three_same_regression_mismatched_t -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_three_same_pbt::test_encode_neon_three_same_regression_mismatched_t' panicked at src/backend/arm/assembler/encoder/encode_neon_three_same_pbt.rs:499:5:
cmeq v0.8b, v0.16b, v0.8b must Err (ARM/gas/llvm-mc require matching T)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_three_same_pbt.rs
