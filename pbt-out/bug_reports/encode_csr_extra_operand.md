# Bug: encode_csr ignores extra operands
**Law:** A SYSTEM CSR instruction (csrrw/csrrs/csrrc) with a fourth (or later) operand must return Err, matching llvm-mc which rejects extra operands.
**Impact:** A mistyped extra operand is silently dropped, so the assembler emits a well-formed SYSTEM CSR word instead of diagnosing the line.
**Function:** encode_csr
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/system.rs:40
**Detected by:** Negative/Error Contract
**Minimal input:** encode_csr([Reg("x0"), Csr("fflags"), Reg("x0"), Imm(0)], funct3=0b001)  // csrrw x0, fflags, x0, 0
**Expected:** Err
**Actual:** Ok(Word(1052787))  // 0x00101073 = csrrw x0, fflags, x0
**Severity:** high
**Root cause:** system.rs:40-53 reads only operands[0..2] via get_reg/get_csr_num and never checks operands.len(), so any trailing operands are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/system.rs:53`
```rust
    Ok(EncodeResult::Word(encode_i(OP_SYSTEM, rd, funct3, rs1, csr as i32)))
```
**Suggested fix:** Reject anything other than exactly three operands before packing.
```rust
pub(crate) fn encode_csr(operands: &[Operand], funct3: u32) -> Result<EncodeResult, String> {
    if operands.len() != 3 {
        return Err("csr: expected rd, csr, rs1/zimm".to_string());
    }
    let rd = get_reg(operands, 0)?;
    let csr = get_csr_num(operands, 1)?;
    if matches!(operands.get(2), Some(Operand::Imm(_))) {
        let zimm = get_imm(operands, 2)? as u32;
        if zimm > 31 {
            return Err("csr: zimm out of range 0..=31".to_string());
        }
        let rs1 = zimm & 0x1F;
        let imm_funct3 = funct3 | 0b100;
        return Ok(EncodeResult::Word(encode_i(OP_SYSTEM, rd, imm_funct3, rs1, csr as i32)));
    }
    let rs1 = get_reg(operands, 2)?;
    Ok(EncodeResult::Word(encode_i(OP_SYSTEM, rd, funct3, rs1, csr as i32)))
}
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_csr_neg_extra -- --test-threads=1
```
**Raw output:**
```text
Test failed: extra operand must Err for csrrw x0, fflags, x0 (llvm-mc rejects extra operands); got Ok(Word(1052787)) at src/backend/riscv/assembler/encoder/encode_csr_pbt.rs:485.
minimal failing input: (mn, f3) = (
    "csrrw",
    1,
), rd = "x0", (csr_name, _num) = (
    "fflags",
    1,
), rs1 = "x0", extra = Imm(
    0,
)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_csr_pbt.rs
