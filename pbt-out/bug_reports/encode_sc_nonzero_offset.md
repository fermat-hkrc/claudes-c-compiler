# Bug: encode_sc silently drops a nonzero memory offset
**Law:** A nonzero memory offset must be rejected; llvm-mc reports `optional integer offset must be 0` for `sc.w a0, a1, 8(a2)`
**Impact:** Source that names a displaced address such as `8(a2)` is assembled as `(a2)`, so the store-conditional operates on a different location than the assembly text says
**Function:** encode_sc
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/atomics.rs:13
**Detected by:** Negative/Error Contract
**Minimal input:** encode_sc([Reg("x0"), Reg("x0"), Mem { base: "x0", offset: 1 }], funct3=0b010)
**Expected:** Err
**Actual:** Ok(Word(0x1800202f)) identical to offset 0
**Severity:** high
**Root cause:** atomics.rs:16 `let (rs1, _offset) = get_mem(operands, 2)?` binds the offset and discards it, then encodes rs1 only
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/atomics.rs:16`
```rust
    let (rs1, _offset) = get_mem(operands, 2)?;
```
**Suggested fix:** Reject any nonzero offset before packing the R-type word
```rust
    let (rs1, offset) = get_mem(operands, 2)?;
    if offset != 0 {
        return Err(format!("sc: memory offset must be 0, got {}", offset));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_sc_neg_nonzero_offset -- --test-threads=1
```
**Raw output:**
```text
Test failed: nonzero offset must Err for sc.w x0, x0, 1(x0) (llvm-mc: optional integer offset must be 0); got Ok(Word(402661423)) at src/backend/riscv/assembler/encoder/encode_sc_pbt.rs:380.
minimal failing input: (mn, f3) = (
    "sc.w",
    2,
), rd = "x0", rs2 = "x0", rs1 = "x0", off = 1
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_sc_pbt.rs
