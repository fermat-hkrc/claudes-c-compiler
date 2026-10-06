# Bug: encode_lui ignores extra operands
**Law:** ∀ rd, ∀ imm ∈ [0, 1048575], ∀ extra. encode_lui([Reg(rd), Imm(imm), extra]) = Err
**Impact:** `lui rd, imm, extra` is encoded as a two-operand LUI. Typos and extra tokens are silently dropped, so the assembler accepts invalid syntax that llvm-mc rejects.
**Function:** encode_lui
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:5
**Detected by:** Negative/Error Contract
**Minimal input:** encode_lui([Reg("x0"), Imm(0), Imm(0)])
**Expected:** Err (llvm-mc: invalid operand for instruction)
**Actual:** Ok(Word(0x37)) — encoding of `lui x0, 0`
**Severity:** medium
**Root cause:** base.rs:7 matches only operands[0] (via get_reg) and operands[1]; operands.len() is never checked, so a third operand is ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:7`
```rust
    match &operands.get(1) {
```
**Suggested fix:** Require exactly two operands before encoding.
```rust
    if operands.len() != 2 {
        return Err("lui: invalid operands".to_string());
    }
    match &operands.get(1) {
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_lui_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_lui_pbt::test_encode_lui_regression_extra_operand' panicked at src/backend/riscv/assembler/encoder/encode_lui_pbt.rs:344:5:
lui x0, 0 with a third operand must Err; got Ok(Word(55))
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_lui_pbt.rs
