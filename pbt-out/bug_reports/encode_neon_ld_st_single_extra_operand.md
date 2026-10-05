# Bug: encode_neon_ld_st_single ignores a surplus operand
**Law:** A third operand that is not a legal post-index immediate must be rejected (llvm-mc/gas reject a surplus operand).
**Impact:** The assembler silently encodes `st1 {v0.b}[0], [x0]` when a trailing junk operand is present, so a malformed line produces a valid 32-bit word instead of an error.
**Function:** encode_neon_ld_st_single
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:904
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_ld_st_single([RegListIndexed({v0.b}[0]), Mem{x0,0}, Cond("eq")], is_load=false, num_structs=1)
**Expected:** Err
**Actual:** Ok(Word(0x0d000000))
**Severity:** medium
**Root cause:** neon.rs:905 only rejects operands.len() < 2; extra non-Imm operands at index 2 are ignored (neon.rs:933-936 treats only Imm as post-index).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:905`
```rust
    if operands.len() < 2 {
        return Err(format!("ld/st{} single element requires at least 2 operands", num_structs));
    }
```
**Suggested fix:** Reject operands.len() > 2 unless operands[2] is a legal post-index Imm or Rm.
```rust
    if operands.len() < 2 {
        return Err(format!("ld/st{} single element requires at least 2 operands", num_structs));
    }
    if operands.len() > 2 {
        match &operands[2] {
            Operand::Imm(_) | Operand::Reg(_) => {}
            _ => return Err("unexpected extra operand".to_string()),
        }
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ld_st_single_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::neon::encode_neon_ld_st_single_pbt::test_encode_neon_ld_st_single_regression_extra_operand' panicked at src/backend/arm/assembler/encoder/neon.rs:13416:9:
st1 {v0.b}[0], [x0], eq must Err
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_ld_st_single_pbt.rs
