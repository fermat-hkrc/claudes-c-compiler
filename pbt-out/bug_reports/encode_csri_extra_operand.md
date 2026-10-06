# Bug: encode_csri ignores extra operands
**Law:** A SYSTEM CSR-immediate instruction (csrrwi/csrrsi/csrrci) with a fourth (or later) operand must return Err, matching llvm-mc which rejects extra operands.
**Impact:** A mistyped extra operand is silently dropped, so the assembler emits a well-formed SYSTEM CSR-immediate word instead of diagnosing the line.
**Function:** encode_csri
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/system.rs:56
**Detected by:** Negative/Error Contract
**Minimal input:** encode_csri([Reg("x0"), Csr("fflags"), Imm(0), Imm(0)], funct3=0b101)  // csrrwi x0, fflags, 0, 0
**Expected:** Err
**Actual:** Ok(Word(1069171))  // 0x00105073 = csrrwi x0, fflags, 0
**Severity:** high
**Root cause:** system.rs:56-62 reads only operands[0..2] via get_reg/get_csr_num/get_imm and never checks operands.len(), so any trailing operands are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/system.rs:61`
```rust
    Ok(EncodeResult::Word(encode_i(OP_SYSTEM, rd, funct3, rs1, csr as i32)))
```
**Suggested fix:** Reject anything other than exactly three operands before packing.
```rust
pub(crate) fn encode_csri(operands: &[Operand], funct3: u32) -> Result<EncodeResult, String> {
    if operands.len() != 3 {
        return Err("csri: expected rd, csr, zimm".to_string());
    }
    let rd = get_reg(operands, 0)?;
    let csr = get_csr_num(operands, 1)?;
    let zimm = get_imm(operands, 2)?;
    if !(0..=31).contains(&zimm) {
        return Err("csri: zimm out of range 0..=31".to_string());
    }
    if csr > 4095 {
        return Err("csri: csr out of range 0..=4095".to_string());
    }
    let rs1 = (zimm as u32) & 0x1F;
    Ok(EncodeResult::Word(encode_i(OP_SYSTEM, rd, funct3, rs1, csr as i32)))
}
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_csri_neg_extra -- --test-threads=1
```
**Raw output:**
```text
Test failed: extra operand must Err for csrrwi x0, fflags, 0 (llvm-mc rejects extra operands); got Ok(Word(1069171)) at src/backend/riscv/assembler/encoder/encode_csri_pbt.rs:484.
minimal failing input: (mn, f3) = (
    "csrrwi",
    5,
), rd = "x0", (csr_name, _num) = (
    "fflags",
    1,
), zimm = 0, extra = Imm(
    0,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_csri_pbt.rs
