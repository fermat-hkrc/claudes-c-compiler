# Bug: encode_lr ignores extra operands
**Law:** A third (or later) operand must be rejected; llvm-mc reports `invalid operand for instruction` for `lr.w a0, (a1), a2`
**Impact:** Typos and extra tokens after a valid LR are silently dropped, so the assembler accepts instructions other RISC-V assemblers reject
**Function:** encode_lr
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/atomics.rs:5
**Detected by:** Negative/Error Contract
**Minimal input:** encode_lr([Reg("x0"), Mem { base: "x0", offset: 0 }, Imm(0)], funct3=0b010)
**Expected:** Err
**Actual:** Ok(Word(0x1000202f)) — the encoding of `lr.w x0, (x0)` with the extra Imm ignored
**Severity:** medium
**Root cause:** atomics.rs:10 — encode_lr returns Ok after reading only operands[0..1] via get_reg/get_mem and never checks operands.len(); encode_instruction (mod.rs:655-656) passes the operand slice through
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/atomics.rs:10`
```rust
    Ok(EncodeResult::Word(encode_r(OP_AMO, rd, funct3, rs1, 0, funct7)))
```
**Suggested fix:** Reject any operand list whose length is not exactly 2
```rust
    if operands.len() != 2 {
        return Err(format!("lr: expected 2 operands, got {}", operands.len()));
    }
    Ok(EncodeResult::Word(encode_r(OP_AMO, rd, funct3, rs1, 0, funct7)))
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_lr_neg_extra -- --test-threads=1
```
**Raw output:**
```text
Test failed: extra operand must Err for lr.w x0, (x0) (llvm-mc rejects extra operands); got Ok(Word(268443695)) at src/backend/riscv/assembler/encoder/encode_lr_pbt.rs:351.
minimal failing input: (mn, f3) = (
    "lr.w",
    2,
), rd = "x0", rs1 = "x0", extra = Imm(
    0,
)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_lr_pbt.rs
