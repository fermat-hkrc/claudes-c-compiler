# Bug: encode_neon_ld_st_multi encodes LD2/ST2/LD3/ST3/LD4/ST4 with .1d, which llvm-mc/gas reject
**Law:** Arrangement .1d is valid only for LD1/ST1; encode_neon_ld_st_multi(RegList({v.1d}×n), Mem[Xn], load, n∈{2,3,4}) = Err
**Impact:** `st2 {v0.1d, v1.1d}, [x0]` is encoded (Q=0 size=11 opcode=1000) although ARM/llvm-mc treat .1d as invalid for LD2/3/4. The assembler emits an unpredictable or undocumented encoding.
**Function:** encode_neon_ld_st_multi
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1008
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_ld_st_multi([RegList({v0.1d, v1.1d}), Mem{x0,0}], is_load=false, num_structs=2)
**Expected:** Err (llvm-mc: invalid operand for instruction)
**Actual:** Ok(Word(0x0c008c00))
**Severity:** medium
**Root cause:** neon.rs:1028 uses neon_arr_to_q_size, which accepts "1d" for every mnemonic; there is no subsequent check that Q=0 size=11 is illegal for num_structs≥2.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1028`
```rust
    let (q, size) = neon_arr_to_q_size(&arr)?;
```
**Suggested fix:** After translating T, reject .1d (Q=0, size=11) when num_structs is 2, 3, or 4.
```rust
    let (q, size) = neon_arr_to_q_size(&arr)?;
    if num_structs >= 2 && q == 0 && size == 0b11 {
        return Err(format!("ld{}/st{}: .1d is not a valid arrangement", num_structs, num_structs));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ld_st_multi_regression_1d_ld2 -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_ld_st_multi_pbt::test_encode_neon_ld_st_multi_regression_1d_ld2' panicked at src/backend/arm/assembler/encoder/encode_neon_ld_st_multi_pbt.rs:751:5:
st2 {v0.1d, v1.1d}, [x0] must Err
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_ld_st_multi_pbt.rs
