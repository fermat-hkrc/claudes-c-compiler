# Bug: encode_alu_imm_w ignores extra operands
**Law:** An OP-IMM-32 (addiw) instruction with a fourth (or later) operand must return Err, matching llvm-mc which rejects extra operands on addiw.
**Impact:** A mistyped extra operand is silently dropped, so the assembler emits a well-formed OP-IMM-32 word instead of diagnosing the line.
**Function:** encode_alu_imm_w
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:267
**Detected by:** Negative/Error Contract
**Minimal input:** encode_alu_imm_w([Reg("x0"), Reg("x0"), Imm(0), Imm(0)], funct3=0)  // addiw x0, x0, 0, 0
**Expected:** Err
**Actual:** Ok(Word(0x0000001b))
**Severity:** high
**Root cause:** base.rs:267-271 never checks operands.len(), so any trailing operands after rd, rs1, imm are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:267`
```rust
pub(crate) fn encode_alu_imm_w(operands: &[Operand], funct3: u32) -> Result<EncodeResult, String> {
    let rd = get_reg(operands, 0)?;
    let rs1 = get_reg(operands, 1)?;
    let imm = get_imm(operands, 2)? as i32;
    Ok(EncodeResult::Word(encode_i(OP_OP_IMM_32, rd, funct3, rs1, imm)))
}
```
**Suggested fix:** Reject anything other than exactly three operands before packing.
```rust
    if operands.len() != 3 {
        return Err("alu_imm_w: expected rd, rs1, imm".to_string());
    }
    let rd = get_reg(operands, 0)?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_alu_imm_w_neg_extra -- --test-threads=1
```
**Raw output:**
```text
Test failed: extra operand must Err for addiw x0, x0, 0 (llvm-mc rejects extra operands); got Ok(Word(27)) at src/backend/riscv/assembler/encoder/encode_alu_imm_w_pbt.rs:476.
minimal failing input: rd = "x0", rs1 = "x0", imm = 0, extra = Imm(
    0,
)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_alu_imm_w_pbt.rs
