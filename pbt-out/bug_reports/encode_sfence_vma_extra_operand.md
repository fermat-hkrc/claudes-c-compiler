# Bug: encode_sfence_vma ignores extra operands
**Law:** A third (or later) operand must be rejected; llvm-mc reports `invalid operand for instruction` for `sfence.vma x0, x0, 0`
**Impact:** Typos and extra tokens after a valid SFENCE.VMA are silently dropped, so the assembler accepts instructions other RISC-V assemblers reject
**Function:** encode_sfence_vma
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/system.rs:30
**Detected by:** Negative/Error Contract
**Minimal input:** encode_sfence_vma([Reg("x0"), Reg("x0"), Imm(0)])
**Expected:** Err
**Actual:** Ok(Word(0x12000073)) — the encoding of `sfence.vma x0, x0` with the extra Imm ignored
**Severity:** medium
**Root cause:** system.rs:31-32 — encode_sfence_vma reads at most operands[0] and operands[1] and never checks operands.len() > 2; encode_instruction (mod.rs:699) passes the operand slice through
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/system.rs:31`
```rust
    let rs1 = if operands.is_empty() { 0 } else { get_reg(operands, 0)? };
    let rs2 = if operands.len() < 2 { 0 } else { get_reg(operands, 1)? };
```
**Suggested fix:** Reject any operand list longer than 2
```rust
    if operands.len() > 2 {
        return Err(format!("sfence.vma: expected at most 2 operands, got {}", operands.len()));
    }
    let rs1 = if operands.is_empty() { 0 } else { get_reg(operands, 0)? };
    let rs2 = if operands.len() < 2 { 0 } else { get_reg(operands, 1)? };
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_sfence_vma_neg_extra -- --test-threads=1
```
**Raw output:**
```text
Test failed: extra operand must Err for sfence.vma x0, x0 (llvm-mc rejects extra operands); got Ok(Word(301990003)) at src/backend/riscv/assembler/encoder/encode_sfence_vma_pbt.rs:335.
minimal failing input: rs1 = "x0", rs2 = "x0", extra = Imm(
    0,
)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_sfence_vma_pbt.rs
