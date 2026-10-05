# Bug: encode_neon_ld_st_single rejects uppercase arrangement (V0.B)
**Law:** GNU as / llvm-mc accept uppercase Vn.T and Xn; encode_neon_ld_st_single must produce the same word as llvm-mc for `st1 {V0.B}[0], [X0]`.
**Impact:** Valid gas assembly with uppercase arrangement is rejected (`unsupported element size: B`), breaking the README gas-compatibility claim.
**Function:** encode_neon_ld_st_single
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:904
**Detected by:** Differential — llvm-mc AArch64 assembler
**Minimal input:** encode_neon_ld_st_single([RegListIndexed({V0.B}[0]), Mem{base:"X0", offset:0}], is_load=false, num_structs=1)
**Expected:** Ok(Word(0x0d000000)) matching llvm-mc
**Actual:** Err("unsupported element size for ld/st single: B")
**Severity:** medium
**Root cause:** neon.rs:960 matches elem_size only as lowercase "b"/"h"/"s"/"d"; arrangement is not lowercased (parse_reg_num lowercases the register name only).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:960`
```rust
    let (opcode, s_bit, q_bit, size_field) = match elem_size.as_str() {
        "b" => {
```
**Suggested fix:** Lowercase the arrangement before the match.
```rust
    let elem_size = elem_size.to_ascii_lowercase();
    let (opcode, s_bit, q_bit, size_field) = match elem_size.as_str() {
        "b" => {
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ld_st_single_regression_alt_spellings -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::neon::encode_neon_ld_st_single_pbt::test_encode_neon_ld_st_single_regression_alt_spellings' panicked at src/backend/arm/assembler/encoder/neon.rs:13455:44:
st1 {V0.B}[0], [X0] must encode: "unsupported element size for ld/st single: B"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_ld_st_single_pbt.rs
