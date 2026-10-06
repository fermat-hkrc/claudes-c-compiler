# Bug: encode_sc ignores extra operands
**Law:** A fourth (or later) operand must be rejected; llvm-mc reports `invalid operand for instruction` for `sc.w a0, a1, (a2), 0`
**Impact:** Typos and extra tokens after a valid SC are silently dropped, so the assembler accepts instructions other RISC-V assemblers reject
**Function:** encode_sc
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/atomics.rs:13
**Detected by:** Negative/Error Contract
**Minimal input:** encode_sc([Reg("x0"), Reg("x0"), Mem { base: "x0", offset: 0 }, Imm(0)], funct3=0b010)
**Expected:** Err
**Actual:** Ok(Word(0x1800202f)) — the encoding of `sc.w x0, x0, (x0)` with the extra Imm ignored
**Severity:** medium
**Root cause:** atomics.rs:18 — encode_sc returns Ok after reading only operands[0..2] via get_reg/get_mem and never checks operands.len(); encode_instruction (mod.rs:659-660) passes the operand slice through
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/atomics.rs:18`
```rust
    Ok(EncodeResult::Word(encode_r(OP_AMO, rd, funct3, rs1, rs2, funct7)))
```
**Suggested fix:** Reject any operand list whose length is not exactly 3
```rust
    if operands.len() != 3 {
        return Err(format!("sc: expected 3 operands, got {}", operands.len()));
    }
    Ok(EncodeResult::Word(encode_r(OP_AMO, rd, funct3, rs1, rs2, funct7)))
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_sc_neg_extra -- --test-threads=1
```
**Raw output:**
```text
Test failed: extra operand must Err for sc.w x0, x0, (x0) (llvm-mc rejects extra operands); got Ok(Word(402661423)) at src/backend/riscv/assembler/encoder/encode_sc_pbt.rs:362.
minimal failing input: (mn, f3) = (
    "sc.w",
    2,
), rd = "x0", rs2 = "x0", rs1 = "x0", extra = Imm(
    0,
)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_sc_pbt.rs
