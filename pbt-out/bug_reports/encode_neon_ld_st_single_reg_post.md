# Bug: encode_neon_ld_st_single encodes register post-index as no-offset
**Law:** `ld1 {v0.s}[0], [x1], x2` must encode ARM register post-index (bit23=1, Rm=x2), matching llvm-mc 0x0dc28020.
**Impact:** The extra Xm is ignored, so the assembler emits a non-writeback load/store. Callers that requested post-index addressing get the wrong machine code with no error.
**Function:** encode_neon_ld_st_single
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:904
**Detected by:** Differential — llvm-mc AArch64 assembler
**Minimal input:** encode_neon_ld_st_single([RegListIndexed({v0.s}[0]), Mem{x1,0}, Reg("x2")], is_load=true, num_structs=1)
**Expected:** Ok(Word(0x0dc28020))
**Actual:** Ok(Word(0x0d408020)) — no-offset form
**Severity:** high
**Root cause:** neon.rs:933-936 only promotes operands[2] when it is Imm; a trailing Reg is dropped. The post-index encoder always writes Rm=11111 (immediate), never Xm.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:933`
```rust
            let pi = if operands.len() > 2 {
                match &operands[2] {
                    Operand::Imm(off) => Some(*off),
                    _ => None,
                }
            } else {
                None
            };
```
**Suggested fix:** Treat a trailing GPR as register post-index and encode Rm=Xm with bit23=1.
```rust
            let pi = if operands.len() > 2 {
                match &operands[2] {
                    Operand::Imm(off) => Some(PostIndex::Imm(*off)),
                    Operand::Reg(rm) => Some(PostIndex::Reg(parse_reg_num(rm).ok_or("invalid Rm")?)),
                    _ => None,
                }
            } else {
                None
            };
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ld_st_single_regression_reg_post -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::neon::encode_neon_ld_st_single_pbt::test_encode_neon_ld_st_single_regression_reg_post' panicked at src/backend/arm/assembler/encoder/neon.rs:13495:9:
assertion `left == right` failed: ld1 {v0.s}[0], [x1], x2 must be 0x0dc28020
  left: 222330912
  right: 230850592
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_ld_st_single_pbt.rs
