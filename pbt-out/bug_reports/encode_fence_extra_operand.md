# Bug: encode_fence ignores extra operands
**Law:** A third (or later) operand must be rejected; llvm-mc reports `invalid operand for instruction` for `fence iorw, iorw, x0`
**Impact:** Typos and extra tokens after a valid fence are silently dropped, so the assembler accepts instructions other RISC-V assemblers reject
**Function:** encode_fence
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:5
**Detected by:** Negative/Error Contract
**Minimal input:** encode_fence([FenceArg("i"), FenceArg("i"), Imm(0)])
**Expected:** Err
**Actual:** Ok(Word) using only the first two operands
**Severity:** medium
**Root cause:** system.rs:8 — `operands.len() >= 2` uses operands[0] and operands[1] and never checks for extras; encode_instruction (mod.rs:682) passes the operand slice through
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/system.rs:8`
```rust
    } else if operands.len() >= 2 {
```
**Suggested fix:** Require exactly two operands on the non-empty path
```rust
    } else if operands.len() == 2 {
```
and `return Err(...)` when `operands.len() > 2`.

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_fence_neg_extra -- --test-threads=1
```
**Raw output:**
```text
Test failed: extra operand must Err for fence i, i (llvm-mc rejects extra operands); got Ok(Word(142606351))
minimal failing input: pred = "i", succ = "i", extra = Imm(0)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_fence_pbt.rs
