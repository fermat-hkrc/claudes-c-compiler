# Bug: encode_lr silently drops a nonzero memory offset
**Law:** A nonzero memory offset must be rejected; llvm-mc reports `optional integer offset must be 0` for `lr.w a0, 8(a1)`
**Impact:** Source that names a displaced address such as `8(a1)` is assembled as `(a1)`, so the load-reserved operates on a different location than the assembly text says
**Function:** encode_lr
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/atomics.rs:5
**Detected by:** Negative/Error Contract
**Minimal input:** encode_lr([Reg("x0"), Mem { base: "x0", offset: 1 }], funct3=0b010)
**Expected:** Err
**Actual:** Ok(Word(0x1000202f)) identical to offset 0
**Severity:** high
**Root cause:** atomics.rs:7 `let (rs1, _offset) = get_mem(operands, 1)?` binds the offset and discards it, then encodes rs1 only
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/atomics.rs:7`
```rust
    let (rs1, _offset) = get_mem(operands, 1)?;
```
**Suggested fix:** Reject any nonzero offset before packing the R-type word
```rust
    let (rs1, offset) = get_mem(operands, 1)?;
    if offset != 0 {
        return Err(format!("lr: memory offset must be 0, got {}", offset));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_lr_neg_nonzero_offset -- --test-threads=1
```
**Raw output:**
```text
Test failed: nonzero offset must Err for lr.w x0, 1(x0) (llvm-mc: optional integer offset must be 0); got Ok(Word(268443695)) at src/backend/riscv/assembler/encoder/encode_lr_pbt.rs:368.
minimal failing input: (mn, f3) = (
    "lr.w",
    2,
), rd = "x0", rs1 = "x0", off = 1
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_lr_pbt.rs
