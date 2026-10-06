# Bug: encode_branch_instr ignores a fourth operand
**Law:** ∀ rs1, rs2 ∈ GPR, ∀ off even in [-4096, 4094], ∀ extra. encode_branch_instr([Reg(rs1), Reg(rs2), Imm(off), extra], 0) = Err
**Impact:** `beq rs1, rs2, off, extra` encodes as a three-operand branch. llvm-mc rejects the extra operand. Typos and extra tokens in assembly are silently dropped, producing wrong programs without an error.
**Function:** encode_branch_instr
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:125
**Detected by:** Negative/Error Contract
**Minimal input:** encode_branch_instr([Reg("x0"), Reg("x0"), Imm(0), Imm(0)], 0)
**Expected:** Err (llvm-mc: invalid operand for instruction)
**Actual:** Ok(Word(0x00000063)) — encoding of `beq x0, x0, 0`
**Severity:** high
**Root cause:** base.rs:129 uses operands.get(2) and never checks operands.len() == 3, so a fourth operand is ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:129`
```rust
    match &operands.get(2) {
```
**Suggested fix:** Reject arity other than 3 before matching the offset operand.
```rust
    if operands.len() != 3 {
        return Err("branch: wrong number of operands".to_string());
    }
    match &operands[2] {
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_branch_instr_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_branch_instr_pbt::test_encode_branch_instr_regression_extra_operand' (2737314) panicked at src/backend/riscv/assembler/encoder/encode_branch_instr_pbt.rs:379:5:
beq x0, x1, 0 with a fourth operand must Err; got Ok(Word(1048675))
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_branch_instr_pbt.rs
