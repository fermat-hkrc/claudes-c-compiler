# Bug: encode_auipc ignores a third operand
**Law:** ∀ rd, ∀ imm, ∀ extra. encode_auipc([Reg(rd), Imm(imm), extra]) = Err
**Impact:** `auipc rd, imm, extra` encodes as a two-operand AUIPC. llvm-mc rejects the extra operand. Typos and extra tokens in assembly are silently dropped, producing wrong programs without an error.
**Function:** encode_auipc
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:30
**Detected by:** Negative/Error Contract
**Minimal input:** encode_auipc([Reg("x0"), Imm(0), Imm(0)])
**Expected:** Err (llvm-mc: invalid operand for instruction)
**Actual:** Ok(Word(0x00000017)) — encoding of `auipc x0, 0`
**Severity:** high
**Root cause:** base.rs:32 matches only operands.get(1) and never checks operands.len(), so a third operand is ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:32`
```rust
    match &operands.get(1) {
```
**Suggested fix:** Reject arity other than 2 after reading rd.
```rust
    if operands.len() != 2 {
        return Err("auipc: invalid operands".to_string());
    }
    match &operands.get(1) {
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_auipc_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_auipc_pbt::test_encode_auipc_regression_extra_operand' panicked at src/backend/riscv/assembler/encoder/encode_auipc_pbt.rs:359:5:
auipc x0, 0 with a third operand must Err; got Ok(Word(23))
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_auipc_pbt.rs
