# Bug: encode_amo ignores extra operands
**Law:** A fourth (or later) operand must be rejected; llvm-mc reports `invalid operand for instruction` for `amoswap.w a0, a1, (a2), a3`
**Impact:** Typos and extra tokens after a valid AMO are silently dropped, so the assembler accepts instructions other RISC-V assemblers reject
**Function:** encode_amo
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/atomics.rs:21
**Detected by:** Negative/Error Contract
**Minimal input:** encode_amo([Reg("x0"), Reg("x0"), Mem { base: "x0", offset: 0 }, Imm(0)], funct3=0b010, funct5=0b00001)
**Expected:** Err
**Actual:** Ok(Word(0x0800202f)) — the encoding of `amoswap.w x0, x0, (x0)` with the extra Imm ignored
**Severity:** medium
**Root cause:** atomics.rs:26 — encode_amo returns Ok after reading only operands[0..2] via get_reg/get_mem and never checks operands.len(); encode_instruction (mod.rs:657-674) passes the operand slice through
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/atomics.rs:26`
```rust
    Ok(EncodeResult::Word(encode_r(OP_AMO, rd, funct3, rs1, rs2, funct7)))
```
**Suggested fix:** Reject any operand list whose length is not exactly 3
```rust
    if operands.len() != 3 {
        return Err(format!("amo: expected 3 operands, got {}", operands.len()));
    }
    Ok(EncodeResult::Word(encode_r(OP_AMO, rd, funct3, rs1, rs2, funct7)))
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_amo_neg_extra -- --test-threads=1
```
**Raw output:**
```text
Test failed: extra operand must Err for amoswap.w x0, x0, (x0) (llvm-mc rejects extra operands); got Ok(Word(134225967)) at src/backend/riscv/assembler/encoder/encode_amo_pbt.rs:379.
minimal failing input: (mn, f3, f5) = (
    "amoswap.w",
    2,
    1,
), rd = "x0", rs2 = "x0", rs1 = "x0", extra = Imm(
    0,
)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_amo_pbt.rs
