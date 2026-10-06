# Bug: encode_jal ignores a third operand
**Law:** ∀ rd, ∀ off, ∀ extra. encode_jal([Reg(rd), Imm(off), extra]) = Err
**Impact:** `jal rd, off, extra` encodes as a two-operand JAL. llvm-mc rejects the extra operand. Typos and extra tokens in assembly are silently dropped, producing wrong programs without an error.
**Function:** encode_jal
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:51
**Detected by:** Negative/Error Contract
**Minimal input:** encode_jal([Reg("x0"), Imm(0), Imm(0)])
**Expected:** Err (llvm-mc: invalid operand for instruction)
**Actual:** Ok(Word(0x0000006f)) — encoding of `jal x0, 0`
**Severity:** high
**Root cause:** base.rs:71 uses `else` (any arity ≠ 1) and matches only operands[1], never checking operands.len() == 2, so a third operand is ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:71`
```rust
    } else {
```
**Suggested fix:** Accept only arity 2 in the two-operand arm.
```rust
    } else if operands.len() == 2 {
        let rd = get_reg(operands, 0)?;
        match &operands[1] {
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_jal_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_jal_pbt::test_encode_jal_regression_extra_operand' panicked at src/backend/riscv/assembler/encoder/encode_jal_pbt.rs:341:5:
jal x0, 0 with a third operand must Err; got Ok(Word(111))
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_jal_pbt.rs
