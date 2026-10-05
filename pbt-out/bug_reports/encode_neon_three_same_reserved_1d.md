# Bug: encode_neon_three_same encodes reserved .1d (size:Q = 11:0)
**Law:** ARM ARM Advanced SIMD three-same reserves size:Q = 11:0 (the `.1d` arrangement). CMEQ/UQSUB/SQSUB/CMHI and the other integer three-same mnemonics this encoder implements must reject `.1d`.
**Impact:** `cmeq v0.1d, v0.1d, v0.1d` encodes a reserved instruction word; llvm-mc rejects the same text. Callers of encode_neon_three_same (encoder/mod.rs cmeq/cmhi/…) pass operands through unchanged.
**Function:** encode_neon_three_same
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:65
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** encode_neon_three_same([v0.1d, v0.1d, v0.1d], u=1, opcode=0b10001)
**Expected:** Err (llvm-mc: "invalid operand for instruction"; ARM reserved)
**Actual:** Ok(Word) with Q=0, size=11
**Severity:** medium
**Root cause:** neon.rs:73 takes Q/size from neon_arr_to_q_size, which treats `"1d"` as valid (Q=0, size=11) with no three-same reserved-encoding check.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:73`
```rust
    let (q, size) = neon_arr_to_q_size(&arr_d)?;
```
**Suggested fix:** Reject Q=0 && size=11 (`.1d`) for three-same, which the ARM ARM marks Reserved.
```rust
    let (q, size) = neon_arr_to_q_size(&arr_d)?;
    if q == 0 && size == 0b11 {
        return Err(format!("NEON three-same reserved arrangement: {arr_d}"));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_three_same_regression_reserved_1d -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_three_same_pbt::test_encode_neon_three_same_regression_reserved_1d' panicked at src/backend/arm/assembler/encoder/encode_neon_three_same_pbt.rs:509:5:
cmeq v0.1d, v0.1d, v0.1d must Err (ARM size:Q=11:0 reserved; llvm-mc rejects)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_three_same_pbt.rs
