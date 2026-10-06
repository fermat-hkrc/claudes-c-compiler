# Bug: encode_fence treats Imm(0) as iorw (full barrier)
**Law:** Numeric fence operand 0 must encode pred/succ bits 0000, matching llvm-mc `fence 0, 0` = 0x0000000f
**Impact:** Valid GNU/LLVM syntax `fence 0, 0` and mixed forms such as `fence 0, rw` assemble to a full `iorw` predecessor (or successor), silently strengthening the memory barrier
**Function:** encode_fence
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:5
**Detected by:** Differential — llvm-mc RISC-V assembler
**Minimal input:** encode_fence([Imm(0), Imm(0)])
**Expected:** Ok(Word(0x0000000f))
**Actual:** Ok(Word(0x0ff0000f))
**Severity:** high
**Root cause:** system.rs:11 and system.rs:15 — any non-FenceArg operand, including Imm(0) that the parser emits for numeric 0, is replaced with 0xF
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/system.rs:11`
```rust
            _ => 0xF,
```
**Suggested fix:** Decode Imm(0) as pred/succ 0; reject other non-FenceArg kinds
```rust
            Operand::Imm(0) => 0,
            Operand::FenceArg(s) => parse_fence_bits(s)?,
            other => return Err(format!("invalid fence operand: {:?}", other)),
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_fence_imm0_diff_llvm_mc -- --test-threads=1
```
**Raw output:**
```text
Test failed: assertion failed: `(left == right)`
  left: `267386895`,
 right: `15`: SUT 0ff0000f != llvm-mc 0000000f for fence 0, 0
minimal failing input: a = "0", b = "0"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_fence_pbt.rs
