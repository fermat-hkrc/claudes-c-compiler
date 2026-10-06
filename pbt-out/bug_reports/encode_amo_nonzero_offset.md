# Bug: encode_amo silently drops a nonzero memory offset
**Law:** AMO memory operands may only use offset 0; llvm-mc reports `optional integer offset must be 0` for `amoswap.w a0, a1, 8(a2)`, and the RISC-V AMO encoding has no immediate field
**Impact:** Source that names a displaced address such as `8(a2)` is assembled as `(a2)`, so the atomic operates on a different location than the assembly text says
**Function:** encode_amo
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/atomics.rs:21
**Detected by:** Negative/Error Contract
**Minimal input:** encode_amo([Reg("x0"), Reg("x0"), Mem { base: "x0", offset: 1 }], funct3=0b010, funct5=0b00001)
**Expected:** Err
**Actual:** Ok(Word(0x0800202f)) — identical to `amoswap.w x0, x0, (x0)` / offset 0
**Severity:** high
**Root cause:** atomics.rs:24 — `let (rs1, _offset) = get_mem(operands, 2)?` binds the offset and discards it, then encodes rs1 only
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/atomics.rs:24`
```rust
    let (rs1, _offset) = get_mem(operands, 2)?;
```
**Suggested fix:** Reject any nonzero offset before packing the R-type word
```rust
    let (rs1, offset) = get_mem(operands, 2)?;
    if offset != 0 {
        return Err(format!("amo: memory offset must be 0, got {}", offset));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_amo_neg_nonzero_offset -- --test-threads=1
```
**Raw output:**
```text
Test failed: nonzero offset must Err for amoswap.w x0, x0, 1(x0) (llvm-mc: optional integer offset must be 0); got Ok(Word(134225967)) at src/backend/riscv/assembler/encoder/encode_amo_pbt.rs:397.
minimal failing input: (mn, f3, f5) = (
    "amoswap.w",
    2,
    1,
), rd = "x0", rs2 = "x0", rs1 = "x0", off = 1
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_amo_pbt.rs
