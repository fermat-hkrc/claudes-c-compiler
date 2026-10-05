# Bug: encode_neon_three_same ignores a fourth operand
**Law:** A NEON three-same instruction has exactly three register operands; a fourth operand must be rejected, matching GNU gas / llvm-mc.
**Impact:** The assembler silently drops trailing operands and emits a 32-bit word for the first three, so invalid assembly such as `cmeq v0.8b, v0.8b, v0.8b, v0.8b` becomes a real instruction instead of an error.
**Function:** encode_neon_three_same
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:65
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** encode_neon_three_same([v0.8b, v0.8b, v0.8b, v0.8b], u=1, opcode=0b10001)  (`cmeq v0.8b, v0.8b, v0.8b, v0.8b`)
**Expected:** Err (llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word) for the first three operands
**Severity:** medium
**Root cause:** neon.rs:66 checks only `operands.len() < 3` and never rejects extra slots, so operands beyond index 2 are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:66`
```rust
    if operands.len() < 3 {
        return Err("NEON three-same requires 3 operands".to_string());
    }
```
**Suggested fix:** Reject `operands.len() != 3` (or `> 3`) with an error naming the extra operand.
```rust
    if operands.len() != 3 {
        return Err("NEON three-same requires 3 operands".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_three_same_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_three_same_pbt::test_encode_neon_three_same_regression_extra_operand' panicked at src/backend/arm/assembler/encoder/encode_neon_three_same_pbt.rs:489:5:
cmeq v0.8b, v0.8b, v0.8b, v0.8b must Err (gas/llvm-mc reject a fourth operand)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_three_same_pbt.rs
