# Bug: encode_neon_cmp_zero encodes reserved .1d (size:Q=11:0)
**Law:** ∀ rd,rn ∈ {0..31}, (U,opc) in cmp_zero_u_opc. llvm-mc(cmeq Vd.1d, Vn.1d, #0) = Err ∧ encode_neon_cmp_zero([Vd.1d, Vn.1d], U, opc) = Err
**Impact:** ARM ARM reserves size:Q=11:0 for integer compare-to-zero. The encoder emits that reserved encoding, which llvm-mc and gas reject as an invalid operand.
**Function:** encode_neon_cmp_zero
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:189
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_cmp_zero([v0.1d, v0.1d], u=0, opcode=0b01000)  // cmeq v0.1d, v0.1d, #0
**Expected:** Err
**Actual:** Ok(Word) with Q=0, size=11
**Severity:** medium
**Root cause:** neon.rs:195 calls neon_arr_to_q_size, which maps "1d" to (Q=0, size=0b11) at neon.rs:52, and encode_neon_cmp_zero does not reject that reserved combination.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:52`
```rust
        "1d" => Ok((0, 0b11)),
```
**Suggested fix:** Reject .1d (Q=0, size=11) inside encode_neon_cmp_zero; do not change neon_arr_to_q_size globally.
```rust
    let (q, size) = neon_arr_to_q_size(&arr_d)?;
    if q == 0 && size == 0b11 {
        return Err("NEON compare-zero: .1d is reserved".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_cmp_zero_neg_reserved_1d -- --test-threads=1
```
**Raw output:**
```text
Test failed: reserved .1d must Err (ARM size:Q=11:0 reserved; llvm-mc rejects cmeq v0.1d, v0.1d, #0) at src/backend/arm/assembler/encoder/encode_neon_cmp_zero_pbt.rs:345.
minimal failing input: rd = 0, rn = 0, u = 0, opcode = 8
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_cmp_zero_pbt.rs
