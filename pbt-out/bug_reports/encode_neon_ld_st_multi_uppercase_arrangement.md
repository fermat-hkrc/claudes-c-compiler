# Bug: encode_neon_ld_st_multi rejects uppercase arrangement specifiers that gas/llvm-mc accept
**Law:** A GNU-style assembler accepts the same textual assembly as gas; encode_neon_ld_st_multi(RegList({Vrt.T_upper..}), Mem[XN], load, n) = llvm-mc(uppercase spelling)
**Impact:** Valid input such as `ld1 {V0.8B}, [X0]` fails with "unsupported NEON arrangement: 8B" while llvm-mc/gas assemble it to 0x0c407000. Uppercase assembly from GCC cannot be consumed.
**Function:** encode_neon_ld_st_multi
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1008
**Detected by:** Differential vs llvm-mc
**Minimal input:** encode_neon_ld_st_multi([RegList({V0.8B}), Mem{X0,0}], is_load=true, num_structs=1)
**Expected:** Ok(Word(0x0c407000)) matching llvm-mc `ld1 {V0.8B}, [X0]`
**Actual:** Err("unsupported NEON arrangement: 8B")
**Severity:** low
**Root cause:** neon.rs:1028 passes the arrangement string to neon_arr_to_q_size, whose match arms are lowercase-only (neon.rs:45-55). Register names are lowercased by parse_reg_num; arrangements are not.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1028`
```rust
    let (q, size) = neon_arr_to_q_size(&arr)?;
```
**Suggested fix:** Case-fold the arrangement before looking it up (or teach neon_arr_to_q_size to accept uppercase).
```rust
    let (q, size) = neon_arr_to_q_size(&arr.to_ascii_lowercase())?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ld_st_multi_regression_uppercase_arr -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_ld_st_multi_pbt::test_encode_neon_ld_st_multi_regression_uppercase_arr' panicked at src/backend/arm/assembler/encoder/encode_neon_ld_st_multi_pbt.rs:721:5:
assertion `left == right` failed: uppercase arrangement must match llvm-mc
  left: Err("unsupported NEON arrangement: 8B")
 right: Ok(205549568)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_ld_st_multi_pbt.rs
