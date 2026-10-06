# Bug: encode_ldr_str ignores extra operands
**Law:** LDR/STR/LDRB/STRB/LDRH/STRH take exactly two operands. llvm-mc/gas reject a third operand (`strb w0, [x0], x0` as a third parsed operand).
**Impact:** A trailing extra operand is dropped and the instruction is encoded as the two-operand form (0x39000000 for `strb w0, [x0]`). Malformed assembly is accepted.
**Function:** encode_ldr_str
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:33
**Detected by:** Negative/error contract — llvm-mc rejects a 3rd operand
**Minimal input:** `encode_ldr_str([Reg("w0"), Mem{base:"x0", offset:0}, Reg("x0")], is_load=false, size=0, is_signed=false, is_128bit=false)`
**Expected:** `Err(...)`
**Actual:** `Ok(Word(0x39000000))` — same as `strb w0, [x0]`
**Severity:** medium
**Root cause:** load_store.rs:34-36 only rejects `operands.len() < 2`. There is no upper bound; extra operands are never read.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:34`
```rust
    if operands.len() < 2 {
        return Err("ldr/str requires at least 2 operands".to_string());
    }
```
**Suggested fix:** Require exactly two operands (literal/register-offset forms included).
```rust
    if operands.len() != 2 {
        return Err("ldr/str requires exactly 2 operands".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_ldr_str_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
STRB W0, [X0], X0 must Err; extra operand is invalid (llvm-mc rejects it)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_ldr_str_pbt.rs
