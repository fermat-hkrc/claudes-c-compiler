# Bug: encode_alu_imm ignores extra operands
**Law:** An OP-IMM instruction with a fourth (or later) operand must return Err, matching llvm-mc which rejects extra operands on addi/slti/sltiu/xori/ori/andi.
**Impact:** A mistyped extra operand is silently dropped, so the assembler emits a well-formed OP-IMM word instead of diagnosing the line.
**Function:** encode_alu_imm
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:223
**Detected by:** Negative/Error Contract
**Minimal input:** encode_alu_imm([Reg("x0"), Reg("x0"), Imm(0), Imm(0)], funct3=0)  // addi x0, x0, 0, 0
**Expected:** Err
**Actual:** Ok(Word(0x00000013))
**Severity:** high
**Root cause:** base.rs:226 matches only operands.get(2) and never checks operands.len(), so any trailing operands are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:226`
```rust
    match &operands.get(2) {
```
**Suggested fix:** Reject anything other than exactly three operands before packing.
```rust
    if operands.len() != 3 {
        return Err("alu_imm: expected rd, rs1, imm".to_string());
    }
    match &operands.get(2) {
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_alu_imm_neg_extra -- --test-threads=1
```
**Raw output:**
```text
Test failed: extra operand must Err for addi x0, x0, 0 (llvm-mc rejects extra operands); got Ok(Word(19)) at src/backend/riscv/assembler/encoder/encode_alu_imm_pbt.rs:519.
minimal failing input: (mn, f3) = (
    "addi",
    0,
), rd = "x0", rs1 = "x0", imm = 0, extra = Imm(
    0,
)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_alu_imm_pbt.rs
