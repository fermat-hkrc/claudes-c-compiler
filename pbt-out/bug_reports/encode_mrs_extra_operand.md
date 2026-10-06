# Bug: encode_mrs ignores extra operands
**Law:** If llvm-mc / GNU as reject `mrs Xt, sysreg, extra`, then encode_mrs([Reg(Xt), Symbol(sysreg), extra]) must return Err
**Impact:** Typos such as `mrs x0, sp_el0, x0` assemble as a silent `mrs x0, sp_el0` instead of being rejected. An extra operand that should have been a parse/encode error is dropped.
**Function:** encode_mrs
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:54
**Detected by:** Negative/Error Contract (extra operand; llvm-mc rejects arity > 2)
**Minimal input:** encode_mrs(&[Operand::Reg("x0".into()), Operand::Symbol("sp_el0".into()), Operand::Reg("x0".into())])  (assembly: `mrs x0, sp_el0, x0`)
**Expected:** Err (llvm-mc: "invalid operand")
**Actual:** Ok(Word(0xd5384100))  // encodes as mrs x0, sp_el0
**Severity:** medium
**Root cause:** system.rs:56-57 `get_reg(operands, 0)` and `operands.get(1)` — only the first two operands are examined; `operands.len()` is never checked, so trailing operands are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:56`
```rust
    let (rt, _) = get_reg(operands, 0)?;
    let sysreg = match operands.get(1) {
```
**Suggested fix:** Reject a slice longer than two operands before encoding.
```rust
    if operands.len() != 2 {
        return Err("mrs: expected Xt, system_reg".to_string());
    }
    let (rt, _) = get_reg(operands, 0)?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_mrs_regression_extra_x0 -- --test-threads=1 --nocapture
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_mrs_pbt::encode_mrs_neg_extra' panicked at src/backend/arm/assembler/encoder/encode_mrs_pbt.rs:419:1:
Test failed: extra operand must Err (llvm-mc rejects mrs x0, sp_el0, x0) at src/backend/arm/assembler/encoder/encode_mrs_pbt.rs:588.
minimal failing input: name = "sp_el0", xt = "x0", extra = Reg("x0")
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_mrs_pbt.rs
