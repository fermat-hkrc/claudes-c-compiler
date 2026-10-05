# Bug: encode_neon_ld_st_multi encodes LD2/ST2/LD3/ST3/LD4/ST4 with the wrong list length
**Law:** LD2/ST2 require exactly 2 consecutive registers, LD3/ST3 exactly 3, LD4/ST4 exactly 4; encode_neon_ld_st_multi(RegList of length ≠ n, load, n≥2) = Err
**Impact:** `ld2 {v0.16b}, [x0]` is accepted and encoded as LD2 (opcode 1000) from v0, which llvm-mc/gas reject. Callers get a 2-structure load from a 1-register list.
**Function:** encode_neon_ld_st_multi
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1008
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_ld_st_multi([RegList({v0.16b}), Mem{x0,0}], is_load=true, num_structs=2)
**Expected:** Err (llvm-mc: invalid operand / invalid number of vectors)
**Actual:** Ok(Word(0x4c408000)) — LD2 opcode with Rt=v0
**Severity:** high
**Root cause:** neon.rs:1056-1058 assigns a fixed opcode for num_structs∈{2,3,4} and never checks num_regs against n. The comment at neon.rs:1063 says "LD2/ST2: 2 reg=1000".
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1056`
```rust
        2 => 0b1000u32,
        3 => 0b0100,
        4 => 0b0000,
```
**Suggested fix:** Require num_regs == num_structs for n in {2,3,4}.
```rust
        2 if num_regs == 2 => 0b1000u32,
        3 if num_regs == 3 => 0b0100,
        4 if num_regs == 4 => 0b0000,
        2 | 3 | 4 => return Err(format!("ld{}/st{}: expected {} registers, got {}", num_structs, num_structs, num_structs, num_regs)),
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ld_st_multi_regression_ld2_one_reg -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_ld_st_multi_pbt::test_encode_neon_ld_st_multi_regression_ld2_one_reg' panicked at src/backend/arm/assembler/encoder/encode_neon_ld_st_multi_pbt.rs:700:5:
ld2 {v0.16b}, [x0] must Err
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_ld_st_multi_pbt.rs
