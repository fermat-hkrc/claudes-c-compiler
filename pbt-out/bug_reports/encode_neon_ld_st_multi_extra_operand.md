# Bug: encode_neon_ld_st_multi ignores a surplus non-post-index operand
**Law:** A GNU-style assembler must reject a surplus operand that is not a post-index Xm/#imm; encode_neon_ld_st_multi(ops ++ [extra], load, n) = Err
**Impact:** `st1 {v0.8b}, [x0], eq` (and any trailing Cond/Shift/Label/arrangement) is assembled as a valid no-offset store, so a typo or extra token silently produces the wrong instruction instead of an assembler error.
**Function:** encode_neon_ld_st_multi
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1008
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_ld_st_multi([RegList({v0.8b}), Mem{x0,0}, Cond("eq")], is_load=false, num_structs=1)
**Expected:** Err (llvm-mc/gas reject `st1 {v0.8b}, [x0], eq`)
**Actual:** Ok(Word(0x0c007000)) — encoded as `st1 {v0.8b}, [x0]`
**Severity:** medium
**Root cause:** neon.rs:1083 `_ => {}` ignores a third operand that is not Imm or Reg, then falls through to the no-offset encoding at neon.rs:1088-1090.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1083`
```rust
            _ => {}
```
**Suggested fix:** Return Err for any extra operand that is not a post-index Imm or Xm.
```rust
            other => return Err(format!("ld{}/st{}: unexpected extra operand {:?}", num_structs, num_structs, other)),
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ld_st_multi_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_ld_st_multi_pbt::test_encode_neon_ld_st_multi_regression_extra_operand' panicked at src/backend/arm/assembler/encoder/encode_neon_ld_st_multi_pbt.rs:674:5:
st1 {v0.8b}, [x0], eq must Err
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_ld_st_multi_pbt.rs
