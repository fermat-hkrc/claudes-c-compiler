# Bug: encode_alu_reg ignores extra operands
**Law:** An OP/R-type instruction with a fourth (or later) operand must return Err, matching llvm-mc which rejects extra operands on add/sub/sll/slt/sltu/xor/srl/sra/or/and and the M/Zbb OP mnemonics.
**Impact:** A mistyped extra operand is silently dropped, so the assembler emits a well-formed OP word instead of diagnosing the line.
**Function:** encode_alu_reg
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:260
**Detected by:** Negative/Error Contract
**Minimal input:** encode_alu_reg([Reg("x0"), Reg("x0"), Reg("x0"), Imm(0)], funct3=0, funct7=0)  // add x0, x0, x0, 0
**Expected:** Err
**Actual:** Ok(Word(51))  // 0x00000033 = add x0, x0, x0
**Severity:** high
**Root cause:** base.rs:261-264 reads only operands[0..2] via get_reg and never checks operands.len(), so any trailing operands are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:260`
```rust
pub(crate) fn encode_alu_reg(operands: &[Operand], funct3: u32, funct7: u32) -> Result<EncodeResult, String> {
    let rd = get_reg(operands, 0)?;
    let rs1 = get_reg(operands, 1)?;
    let rs2 = get_reg(operands, 2)?;
    Ok(EncodeResult::Word(encode_r(OP_OP, rd, funct3, rs1, rs2, funct7)))
}
```
**Suggested fix:** Reject anything other than exactly three operands before packing.
```rust
pub(crate) fn encode_alu_reg(operands: &[Operand], funct3: u32, funct7: u32) -> Result<EncodeResult, String> {
    if operands.len() != 3 {
        return Err("alu_reg: expected rd, rs1, rs2".to_string());
    }
    let rd = get_reg(operands, 0)?;
    let rs1 = get_reg(operands, 1)?;
    let rs2 = get_reg(operands, 2)?;
    Ok(EncodeResult::Word(encode_r(OP_OP, rd, funct3, rs1, rs2, funct7)))
}
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_alu_reg_neg_extra -- --test-threads=1
```
**Raw output:**
```text
Test failed: extra operand must Err for add x0, x0, x0 (llvm-mc rejects extra operands); got Ok(Word(51)) at src/backend/riscv/assembler/encoder/encode_alu_reg_pbt.rs:456.
minimal failing input: (mn, f3, f7) = (
    "add",
    0,
    0,
), rd = "x0", rs1 = "x0", rs2 = "x0", extra = Imm(
    0,
)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_alu_reg_pbt.rs
