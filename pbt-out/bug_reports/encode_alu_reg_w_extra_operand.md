# Bug: encode_alu_reg_w ignores extra operands
**Law:** An OP-32/R-type word instruction with a fourth (or later) operand must return Err, matching llvm-mc which rejects extra operands on addw/subw/sllw/srlw/sraw/mulw/divw/divuw/remw/remuw/rolw/rorw.
**Impact:** A mistyped extra operand is silently dropped, so the assembler emits a well-formed OP-32 word instead of diagnosing the line.
**Function:** encode_alu_reg_w
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:283
**Detected by:** Negative/Error Contract
**Minimal input:** encode_alu_reg_w([Reg("x0"), Reg("x0"), Reg("x0"), Imm(0)], funct3=0, funct7=0)  // addw x0, x0, x0, 0
**Expected:** Err
**Actual:** Ok(Word(59))  // 0x0000003b = addw x0, x0, x0
**Severity:** high
**Root cause:** base.rs:284-287 reads only operands[0..2] via get_reg and never checks operands.len(), so any trailing operands are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:287`
```rust
    Ok(EncodeResult::Word(encode_r(OP_OP_32, rd, funct3, rs1, rs2, funct7)))
```
**Suggested fix:** Reject anything other than exactly three operands before packing.
```rust
pub(crate) fn encode_alu_reg_w(operands: &[Operand], funct3: u32, funct7: u32) -> Result<EncodeResult, String> {
    if operands.len() != 3 {
        return Err("alu_reg_w: expected rd, rs1, rs2".to_string());
    }
    let rd = get_reg(operands, 0)?;
    let rs1 = get_reg(operands, 1)?;
    let rs2 = get_reg(operands, 2)?;
    Ok(EncodeResult::Word(encode_r(OP_OP_32, rd, funct3, rs1, rs2, funct7)))
}
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_alu_reg_w_neg_extra -- --test-threads=1
```
**Raw output:**
```text
Test failed: extra operand must Err for addw x0, x0, x0 (llvm-mc rejects extra operands); got Ok(Word(59)) at src/backend/riscv/assembler/encoder/encode_alu_reg_w_pbt.rs:446.
minimal failing input: (mn, f3, f7) = (
    "addw",
    0,
    0,
), rd = "x0", rs1 = "x0", rs2 = "x0", extra = Imm(
    0,
)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_alu_reg_w_pbt.rs
